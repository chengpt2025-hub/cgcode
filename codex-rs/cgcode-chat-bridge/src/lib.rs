//! cgcode 模型接口适配层。
//!
//! 上游 Codex 只支持 OpenAI 的 Responses 接口，而国内大多数模型服务和中转站只提供
//! Chat Completions 接口。本模块在本机 127.0.0.1 的随机端口上运行一个转换服务：
//! Codex 把请求发到这里的 `/v1/responses`，本服务转换成 `/chat/completions` 请求发往
//! 用户配置的模型接口，再把流式结果转换回 Responses 的事件格式。
//!
//! 如果上游本身支持 Responses 接口，则以透传模式运行，只负责附加鉴权信息。
//! API Key 只保存在本服务内存中，不写入 Codex 的配置。

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::body::Body;
use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::response::Response;
use axum::routing::get;
use axum::routing::post;
use futures::StreamExt;
use serde_json::Map;
use serde_json::Value;
use serde_json::json;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

/// 上游接口类型。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpstreamMode {
    /// `/chat/completions`（DeepSeek、通义千问、大多数中转站）。
    ChatCompletions,
    /// `/responses`（OpenAI 官方及兼容 Responses 的服务），透传。
    Responses,
}

impl UpstreamMode {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "chat" | "chat_completions" | "chat-completions" => Some(Self::ChatCompletions),
            "responses" | "response" => Some(Self::Responses),
            _ => None,
        }
    }
}

/// 一个上游模型接口。
#[derive(Clone, Debug)]
pub struct Upstream {
    /// 例如 `https://api.deepseek.com/v1`。
    pub base_url: String,
    pub api_key: Option<String>,
    pub mode: UpstreamMode,
    /// 发往此上游时替换的模型名；为空时沿用 Codex 请求里的模型名。
    pub model: Option<String>,
    /// 合并进请求体的额外字段（例如 `{"temperature": 0.2}`）。
    pub extra_body: Map<String, Value>,
    /// 额外请求头。
    pub headers: HashMap<String, String>,
}

/// 适配层配置。`upstreams[0]` 为主接口，其余为失败时依次切换的备用接口。
#[derive(Clone, Debug)]
pub struct BridgeConfig {
    pub upstreams: Vec<Upstream>,
    /// 把模型返回的思考内容回传给上游（DeepSeek 思考模式调用工具时需要）。
    pub echo_reasoning: bool,
    pub connect_timeout: Duration,
}

impl BridgeConfig {
    pub fn single(upstream: Upstream) -> Self {
        Self {
            upstreams: vec![upstream],
            echo_reasoning: true,
            connect_timeout: Duration::from_secs(30),
        }
    }
}

struct AppState {
    cfg: BridgeConfig,
    client: reqwest::Client,
}

/// 在 127.0.0.1 的随机端口上启动适配服务，返回监听地址。
/// Codex 的模型提供方 base_url 应设为 `http://{addr}/v1`。
pub async fn spawn(cfg: BridgeConfig) -> std::io::Result<SocketAddr> {
    if cfg.upstreams.is_empty() {
        return Err(std::io::Error::other("没有配置任何模型接口"));
    }
    let client = reqwest::Client::builder()
        .connect_timeout(cfg.connect_timeout)
        .build()
        .map_err(std::io::Error::other)?;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    let state = Arc::new(AppState { cfg, client });
    let app = Router::new()
        .route("/v1/responses", post(handle_responses))
        .route("/responses", post(handle_responses))
        .route("/v1/models", get(handle_models))
        .route("/models", get(handle_models))
        .route("/health", get(|| async { "ok" }))
        .with_state(state);
    tokio::spawn(async move {
        if let Err(err) = axum::serve(listener, app).await {
            tracing::error!("cgcode 适配服务异常退出: {err}");
        }
    });
    Ok(addr)
}

fn error_response(status: StatusCode, message: &str) -> Response {
    let body = json!({"error": {"message": message, "type": "cgcode_bridge_error"}});
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}

fn join_url(base: &str, path: &str) -> String {
    format!("{}/{}", base.trim_end_matches('/'), path.trim_start_matches('/'))
}

fn apply_auth(mut rb: reqwest::RequestBuilder, up: &Upstream) -> reqwest::RequestBuilder {
    if let Some(key) = up.api_key.as_deref().filter(|k| !k.is_empty()) {
        rb = rb.bearer_auth(key);
    }
    for (k, v) in &up.headers {
        rb = rb.header(k, v);
    }
    rb
}

/// 从上游错误响应里提取可读的错误信息。
fn upstream_error_message(status: StatusCode, text: &str) -> String {
    let detail = serde_json::from_str::<Value>(text)
        .ok()
        .and_then(|v| {
            v.pointer("/error/message")
                .or_else(|| v.get("message"))
                .or_else(|| v.get("error"))
                .and_then(|m| m.as_str().map(str::to_owned))
        })
        .unwrap_or_else(|| text.chars().take(500).collect());
    let hint = match status.as_u16() {
        401 | 403 => "（API Key 无效或没有权限，可运行 cgcode setup 重新配置）",
        404 => "（接口地址或模型名可能填错了，可运行 cgcode setup 重新配置）",
        429 => "（请求太频繁或额度不足）",
        500..=599 => "（模型服务暂时不可用）",
        _ => "",
    };
    format!("模型接口返回错误 {}：{}{}", status.as_u16(), detail, hint)
}

/// 失败时是否值得切换到下一个备用接口。
fn should_fail_over(status: Option<StatusCode>) -> bool {
    match status {
        None => true,
        Some(s) => s.is_server_error() || s == StatusCode::TOO_MANY_REQUESTS || s.as_u16() == 401 || s.as_u16() == 403,
    }
}

/// Codex 期望的模型目录格式与 OpenAI 的 /models 不同，这里明确返回 404，
/// 让 Codex 使用内置的模型信息，而不是解析失败。
async fn handle_models(State(_st): State<Arc<AppState>>) -> Response {
    error_response(StatusCode::NOT_FOUND, "cgcode 适配层不提供模型目录")
}

/// 首次启动向导用：探测用户填写的接口是 Chat Completions 还是 Responses，
/// 并确认 Key 和模型名可用。返回规范化后的 base_url 和接口类型。
pub async fn probe(base_url: &str, api_key: &str, model: &str) -> Result<(String, UpstreamMode), String> {
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(90))
        .build()
        .map_err(|e| format!("无法创建网络客户端：{e}"))?;
    let base = base_url.trim().trim_end_matches('/').to_owned();
    if !(base.starts_with("http://") || base.starts_with("https://")) {
        return Err("接口地址需要以 http:// 或 https:// 开头".into());
    }
    let mut candidates = vec![base.clone()];
    if !base.ends_with("/v1") {
        candidates.push(format!("{base}/v1"));
    }
    let mut last_error = String::from("接口没有响应");
    for candidate in candidates {
        let auth = |rb: reqwest::RequestBuilder| if api_key.is_empty() { rb } else { rb.bearer_auth(api_key) };
        let chat = auth(client.post(join_url(&candidate, "chat/completions")))
            .json(&json!({
                "model": model,
                "messages": [{"role": "user", "content": "回复“好”"}],
                "max_tokens": 16,
                "stream": false
            }))
            .send()
            .await;
        match chat {
            Ok(resp) if resp.status().is_success() => return Ok((candidate, UpstreamMode::ChatCompletions)),
            Ok(resp) => {
                let status = StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
                let text = resp.text().await.unwrap_or_default();
                if matches!(status.as_u16(), 401 | 403) {
                    return Err(upstream_error_message(status, &text));
                }
                last_error = upstream_error_message(status, &text);
            }
            Err(err) => last_error = format!("无法连接到 {candidate}：{err}"),
        }
        let responses = auth(client.post(join_url(&candidate, "responses")))
            .json(&json!({"model": model, "input": "回复“好”", "max_output_tokens": 16}))
            .send()
            .await;
        match responses {
            Ok(resp) if resp.status().is_success() => return Ok((candidate, UpstreamMode::Responses)),
            Ok(resp) => {
                let status = StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
                if !matches!(status.as_u16(), 404 | 405) {
                    let text = resp.text().await.unwrap_or_default();
                    last_error = upstream_error_message(status, &text);
                }
            }
            Err(_) => {}
        }
    }
    Err(last_error)
}

async fn handle_responses(State(st): State<Arc<AppState>>, body: Bytes) -> Response {
    let request: Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(err) => return error_response(StatusCode::BAD_REQUEST, &format!("请求解析失败：{err}")),
    };
    let mut last_error = (StatusCode::BAD_GATEWAY, String::from("没有可用的模型接口"));
    for (idx, up) in st.cfg.upstreams.iter().enumerate() {
        let attempt = match up.mode {
            UpstreamMode::Responses => send_responses_passthrough(&st, up, &request).await,
            UpstreamMode::ChatCompletions => send_chat(&st, up, &request).await,
        };
        match attempt {
            Ok(resp) => return resp,
            Err((status, message)) => {
                let has_next = idx + 1 < st.cfg.upstreams.len();
                if has_next && should_fail_over(status) {
                    tracing::warn!("模型接口 {} 失败，切换到备用接口：{message}", up.base_url);
                    last_error = (status.unwrap_or(StatusCode::BAD_GATEWAY), message);
                    continue;
                }
                return error_response(status.unwrap_or(StatusCode::BAD_GATEWAY), &message);
            }
        }
    }
    error_response(last_error.0, &last_error.1)
}

type AttemptResult = Result<Response, (Option<StatusCode>, String)>;

async fn send_responses_passthrough(st: &AppState, up: &Upstream, request: &Value) -> AttemptResult {
    let mut request = request.clone();
    if let (Some(model), Some(obj)) = (up.model.as_ref(), request.as_object_mut()) {
        obj.insert("model".into(), Value::String(model.clone()));
    }
    if let Some(obj) = request.as_object_mut() {
        for (k, v) in &up.extra_body {
            obj.insert(k.clone(), v.clone());
        }
    }
    let rb = apply_auth(st.client.post(join_url(&up.base_url, "responses")), up).json(&request);
    let resp = rb.send().await.map_err(|e| (None, format!("无法连接到模型接口：{e}")))?;
    let status = StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        return Err((Some(status), upstream_error_message(status, &text)));
    }
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("text/event-stream")
        .to_owned();
    let stream = resp.bytes_stream().map(|r| r.map_err(std::io::Error::other));
    Ok(Response::builder()
        .status(status)
        .header(CONTENT_TYPE, HeaderValue::from_str(&content_type).unwrap_or(HeaderValue::from_static("text/event-stream")))
        .body(Body::from_stream(stream))
        .unwrap_or_else(|_| Response::new(Body::empty())))
}

async fn send_chat(st: &AppState, up: &Upstream, request: &Value) -> AttemptResult {
    let (chat_request, ctx) = translate_request(request, up, st.cfg.echo_reasoning);
    let rb = apply_auth(st.client.post(join_url(&up.base_url, "chat/completions")), up)
        .header(reqwest::header::ACCEPT, "text/event-stream")
        .json(&chat_request);
    let resp = rb.send().await.map_err(|e| (None, format!("无法连接到模型接口：{e}")))?;
    let status = StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        return Err((Some(status), upstream_error_message(status, &text)));
    }
    let is_sse = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("event-stream"))
        .unwrap_or(true);
    let (tx, rx) = mpsc::channel::<Result<Bytes, std::io::Error>>(256);
    tokio::spawn(async move {
        let mut conv = Converter::new(ctx, tx);
        if is_sse {
            conv.run_sse(resp).await;
        } else {
            conv.run_json(resp).await;
        }
    });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "text/event-stream")
        .body(Body::from_stream(ReceiverStream::new(rx)))
        .unwrap_or_else(|_| Response::new(Body::empty())))
}

// ---------------------------------------------------------------------------
// 请求转换：Responses -> Chat Completions
// ---------------------------------------------------------------------------

/// 转换过程中需要带到响应阶段的信息。
#[derive(Debug, Default, Clone)]
pub struct RequestContext {
    /// 以自由文本输入的工具（如 apply_patch），在 Chat 里包装成带 `input` 参数的函数。
    pub custom_tools: HashSet<String>,
    /// 命名空间工具：扁平化后的名字 -> (命名空间, 原名)。
    pub namespaced: HashMap<String, (String, String)>,
}

const NS_SEP: &str = "__";

fn text_of_parts(content: &Value) -> String {
    match content {
        Value::String(s) => s.clone(),
        Value::Array(parts) => parts
            .iter()
            .filter_map(|p| p.get("text").and_then(Value::as_str))
            .collect::<Vec<_>>()
            .join(""),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// 把用户消息的内容转换成 Chat 格式；只有文字时压缩成字符串，兼容性最好。
fn convert_user_content(content: &Value) -> Value {
    let Value::Array(parts) = content else {
        return Value::String(text_of_parts(content));
    };
    let mut out = Vec::new();
    let mut has_image = false;
    for part in parts {
        match part.get("type").and_then(Value::as_str) {
            Some("input_text") | Some("output_text") | Some("text") => {
                if let Some(text) = part.get("text").and_then(Value::as_str) {
                    out.push(json!({"type": "text", "text": text}));
                }
            }
            Some("input_image") => {
                let url = part
                    .get("image_url")
                    .and_then(|u| u.as_str().map(str::to_owned).or_else(|| u.get("url").and_then(Value::as_str).map(str::to_owned)));
                if let Some(url) = url {
                    has_image = true;
                    out.push(json!({"type": "image_url", "image_url": {"url": url}}));
                }
            }
            _ => {}
        }
    }
    if has_image {
        Value::Array(out)
    } else {
        Value::String(
            out.iter()
                .filter_map(|p| p.get("text").and_then(Value::as_str))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

fn tool_output_text(output: &Value) -> String {
    match output {
        Value::String(s) => s.clone(),
        Value::Array(_) => text_of_parts(output),
        Value::Object(obj) => obj
            .get("content")
            .map(text_of_parts)
            .or_else(|| obj.get("output").map(text_of_parts))
            .unwrap_or_else(|| output.to_string()),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn reasoning_text(item: &Value) -> String {
    let from_content = item
        .get("content")
        .and_then(Value::as_array)
        .map(|parts| parts.iter().filter_map(|p| p.get("text").and_then(Value::as_str)).collect::<Vec<_>>().join(""))
        .unwrap_or_default();
    if !from_content.is_empty() {
        return from_content;
    }
    item.get("summary")
        .and_then(Value::as_array)
        .map(|parts| parts.iter().filter_map(|p| p.get("text").and_then(Value::as_str)).collect::<Vec<_>>().join("\n"))
        .unwrap_or_default()
}

fn push_tool_call(messages: &mut Vec<Value>, call_id: String, name: String, arguments: String, reasoning: Option<String>, echo_reasoning: bool) {
    let call = json!({"id": call_id, "type": "function", "function": {"name": name, "arguments": arguments}});
    // 连续的工具调用合并到同一条助手消息里。
    if let Some(last) = messages.last_mut() {
        if last.get("role").and_then(Value::as_str) == Some("assistant") {
            let obj = last.as_object_mut().expect("message is object");
            let calls = obj.entry("tool_calls").or_insert_with(|| Value::Array(Vec::new()));
            if let Value::Array(arr) = calls {
                arr.push(call);
                if echo_reasoning {
                    if let Some(r) = reasoning.filter(|r| !r.is_empty()) {
                        obj.entry("reasoning_content").or_insert(Value::String(r));
                    }
                }
                return;
            }
        }
    }
    let mut msg = json!({"role": "assistant", "content": Value::Null, "tool_calls": [call]});
    if echo_reasoning {
        if let Some(r) = reasoning.filter(|r| !r.is_empty()) {
            msg["reasoning_content"] = Value::String(r);
        }
    }
    messages.push(msg);
}

/// 保证每个工具调用后面都紧跟对应的工具结果，否则大多数 Chat 接口会直接报错。
fn repair_tool_pairs(messages: Vec<Value>) -> Vec<Value> {
    let mut out: Vec<Value> = Vec::with_capacity(messages.len());
    let mut open: Vec<String> = Vec::new();
    let close_open = |out: &mut Vec<Value>, open: &mut Vec<String>| {
        for id in open.drain(..) {
            out.push(json!({"role": "tool", "tool_call_id": id, "content": "（该工具调用已中断，没有结果）"}));
        }
    };
    for msg in messages {
        let role = msg.get("role").and_then(Value::as_str).unwrap_or("").to_owned();
        if role == "tool" {
            let id = msg.get("tool_call_id").and_then(Value::as_str).unwrap_or("").to_owned();
            if let Some(pos) = open.iter().position(|o| *o == id) {
                open.remove(pos);
                out.push(msg);
            } else {
                // 找不到对应调用的结果，改成普通文字，避免接口报错。
                let text = msg.get("content").map(text_of_parts).unwrap_or_default();
                out.push(json!({"role": "user", "content": format!("[工具结果]\n{text}")}));
            }
            continue;
        }
        close_open(&mut out, &mut open);
        if role == "assistant" {
            if let Some(calls) = msg.get("tool_calls").and_then(Value::as_array) {
                open = calls
                    .iter()
                    .filter_map(|c| c.get("id").and_then(Value::as_str).map(str::to_owned))
                    .collect();
            }
        }
        out.push(msg);
    }
    close_open(&mut out, &mut open);
    out
}

fn convert_tools(tools: &Value, ctx: &mut RequestContext) -> Vec<Value> {
    let mut out = Vec::new();
    let Some(list) = tools.as_array() else { return out };
    for tool in list {
        convert_tool(tool, None, ctx, &mut out);
    }
    out
}

fn convert_tool(tool: &Value, namespace: Option<&str>, ctx: &mut RequestContext, out: &mut Vec<Value>) {
    let kind = tool.get("type").and_then(Value::as_str).unwrap_or("");
    let name = tool.get("name").and_then(Value::as_str).unwrap_or("").to_owned();
    let flat_name = |name: &str, ctx: &mut RequestContext| -> String {
        match namespace {
            Some(ns) if !ns.is_empty() => {
                let flat: String = format!("{ns}{NS_SEP}{name}")
                    .chars()
                    .map(|c| if c.is_ascii_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
                    .take(64)
                    .collect();
                ctx.namespaced.insert(flat.clone(), (ns.to_owned(), name.to_owned()));
                flat
            }
            _ => name.to_owned(),
        }
    };
    match kind {
        "function" => {
            if name.is_empty() {
                return;
            }
            let fname = flat_name(&name, ctx);
            let mut parameters = tool.get("parameters").cloned().unwrap_or_else(|| json!({"type": "object", "properties": {}}));
            if parameters.get("type").is_none() {
                parameters["type"] = json!("object");
            }
            out.push(json!({
                "type": "function",
                "function": {
                    "name": fname,
                    "description": tool.get("description").and_then(Value::as_str).unwrap_or(""),
                    "parameters": parameters,
                }
            }));
        }
        "custom" => {
            if name.is_empty() {
                return;
            }
            let fname = flat_name(&name, ctx);
            ctx.custom_tools.insert(fname.clone());
            let mut description = tool.get("description").and_then(Value::as_str).unwrap_or("").to_owned();
            if let Some(def) = tool.pointer("/format/definition").and_then(Value::as_str) {
                description.push_str("\n\n把完整的原始输入文本放在参数 input 里，格式必须符合以下语法：\n");
                description.push_str(def);
            }
            out.push(json!({
                "type": "function",
                "function": {
                    "name": fname,
                    "description": description,
                    "parameters": {
                        "type": "object",
                        "properties": {"input": {"type": "string", "description": "工具的原始输入文本"}},
                        "required": ["input"],
                    }
                }
            }));
        }
        "namespace" => {
            if let Some(children) = tool.get("tools").and_then(Value::as_array) {
                for child in children {
                    convert_tool(child, Some(&name), ctx, out);
                }
            }
        }
        // web_search、image_generation、local_shell 等是 OpenAI 托管工具，Chat 接口不支持，跳过。
        _ => {}
    }
}

/// 把 Responses 请求转换成 Chat Completions 请求。
pub fn translate_request(request: &Value, up: &Upstream, echo_reasoning: bool) -> (Value, RequestContext) {
    let mut ctx = RequestContext::default();
    let tools = request.get("tools").map(|t| convert_tools(t, &mut ctx)).unwrap_or_default();

    let mut system_parts: Vec<String> = Vec::new();
    if let Some(ins) = request.get("instructions").and_then(Value::as_str).filter(|s| !s.trim().is_empty()) {
        system_parts.push(ins.to_owned());
    }
    let mut messages: Vec<Value> = Vec::new();
    let mut pending_reasoning: Option<String> = None;
    let flat = |name: &str, ns: Option<&str>| -> String {
        match ns {
            Some(ns) if !ns.is_empty() => format!("{ns}{NS_SEP}{name}"),
            _ => name.to_owned(),
        }
    };

    for item in request.get("input").and_then(Value::as_array).into_iter().flatten() {
        let kind = item.get("type").and_then(Value::as_str).unwrap_or("message");
        match kind {
            "message" => {
                let role = item.get("role").and_then(Value::as_str).unwrap_or("user");
                let content = item.get("content").cloned().unwrap_or(Value::Null);
                match role {
                    "system" | "developer" => {
                        let text = text_of_parts(&content);
                        if text.trim().is_empty() {
                            continue;
                        }
                        if messages.is_empty() {
                            system_parts.push(text);
                        } else {
                            // 很多接口不允许对话中途出现 system 消息，改成带标记的用户消息。
                            messages.push(json!({"role": "user", "content": format!("[系统提示]\n{text}")}));
                        }
                    }
                    "assistant" => {
                        let text = text_of_parts(&content);
                        let mut msg = json!({"role": "assistant", "content": text});
                        if echo_reasoning {
                            if let Some(r) = pending_reasoning.take().filter(|r| !r.is_empty()) {
                                msg["reasoning_content"] = Value::String(r);
                            }
                        }
                        messages.push(msg);
                    }
                    _ => {
                        pending_reasoning = None;
                        messages.push(json!({"role": "user", "content": convert_user_content(&content)}));
                    }
                }
            }
            "reasoning" => {
                let text = reasoning_text(item);
                if !text.is_empty() {
                    pending_reasoning = Some(text);
                }
            }
            "function_call" => {
                let name = item.get("name").and_then(Value::as_str).unwrap_or("");
                let ns = item.get("namespace").and_then(Value::as_str);
                let args = item.get("arguments").and_then(Value::as_str).unwrap_or("{}");
                let call_id = item.get("call_id").and_then(Value::as_str).unwrap_or("").to_owned();
                push_tool_call(&mut messages, call_id, flat(name, ns), args.to_owned(), pending_reasoning.take(), echo_reasoning);
            }
            "custom_tool_call" => {
                let name = item.get("name").and_then(Value::as_str).unwrap_or("");
                let ns = item.get("namespace").and_then(Value::as_str);
                let input = item.get("input").and_then(Value::as_str).unwrap_or("");
                let call_id = item.get("call_id").and_then(Value::as_str).unwrap_or("").to_owned();
                push_tool_call(&mut messages, call_id, flat(name, ns), json!({"input": input}).to_string(), pending_reasoning.take(), echo_reasoning);
            }
            "local_shell_call" => {
                let call_id = item
                    .get("call_id")
                    .and_then(Value::as_str)
                    .or_else(|| item.get("id").and_then(Value::as_str))
                    .unwrap_or("")
                    .to_owned();
                let args = item.get("action").cloned().unwrap_or(Value::Null).to_string();
                push_tool_call(&mut messages, call_id, "local_shell".into(), args, pending_reasoning.take(), echo_reasoning);
            }
            "function_call_output" | "custom_tool_call_output" | "local_shell_call_output" => {
                let call_id = item.get("call_id").and_then(Value::as_str).unwrap_or("").to_owned();
                let output = tool_output_text(item.get("output").unwrap_or(&Value::Null));
                messages.push(json!({"role": "tool", "tool_call_id": call_id, "content": output}));
            }
            _ => {}
        }
    }

    let mut final_messages = Vec::new();
    if !system_parts.is_empty() {
        final_messages.push(json!({"role": "system", "content": system_parts.join("\n\n")}));
    }
    final_messages.extend(repair_tool_pairs(messages));

    let model = up
        .model
        .clone()
        .filter(|m| !m.is_empty())
        .or_else(|| request.get("model").and_then(Value::as_str).map(str::to_owned))
        .unwrap_or_default();
    let mut chat = json!({
        "model": model,
        "messages": final_messages,
        "stream": true,
        "stream_options": {"include_usage": true},
    });
    if !tools.is_empty() {
        chat["tools"] = Value::Array(tools);
        chat["tool_choice"] = json!("auto");
        if let Some(p) = request.get("parallel_tool_calls").and_then(Value::as_bool) {
            chat["parallel_tool_calls"] = json!(p);
        }
    }
    if let Some(obj) = chat.as_object_mut() {
        for (k, v) in &up.extra_body {
            obj.insert(k.clone(), v.clone());
        }
    }
    (chat, ctx)
}

// ---------------------------------------------------------------------------
// 响应转换：Chat Completions 流 -> Responses 事件
// ---------------------------------------------------------------------------

#[derive(Default, Debug)]
struct PendingToolCall {
    id: String,
    name: String,
    arguments: String,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum ThinkState {
    /// 还没收到正文，需要判断是否以 <think> 开头。
    Undecided,
    InThink,
    Normal,
}

/// 把 Chat Completions 的流式块转换成 Responses 事件。
pub struct Converter {
    ctx: RequestContext,
    tx: mpsc::Sender<Result<Bytes, std::io::Error>>,
    response_id: String,
    message_id: String,
    reasoning_id: String,
    text: String,
    reasoning: String,
    message_started: bool,
    reasoning_started: bool,
    tool_calls: BTreeMap<u64, PendingToolCall>,
    usage: Option<Value>,
    think: ThinkState,
    think_buf: String,
    failed: bool,
    finished: bool,
}

fn new_id(prefix: &str) -> String {
    format!("{prefix}_{}", uuid::Uuid::new_v4().simple())
}

impl Converter {
    pub fn new(ctx: RequestContext, tx: mpsc::Sender<Result<Bytes, std::io::Error>>) -> Self {
        Self {
            ctx,
            tx,
            response_id: new_id("resp"),
            message_id: new_id("msg"),
            reasoning_id: new_id("rs"),
            text: String::new(),
            reasoning: String::new(),
            message_started: false,
            reasoning_started: false,
            tool_calls: BTreeMap::new(),
            usage: None,
            think: ThinkState::Undecided,
            think_buf: String::new(),
            failed: false,
            finished: false,
        }
    }

    async fn emit(&self, event: Value) {
        let kind = event.get("type").and_then(Value::as_str).unwrap_or("message").to_owned();
        let payload = format!("event: {kind}\ndata: {event}\n\n");
        let _ = self.tx.send(Ok(Bytes::from(payload))).await;
    }

    async fn start(&self) {
        self.emit(json!({"type": "response.created", "response": {"id": self.response_id}})).await;
    }

    async fn on_reasoning(&mut self, delta: &str) {
        if delta.is_empty() {
            return;
        }
        if !self.reasoning_started {
            self.reasoning_started = true;
            self.emit(json!({
                "type": "response.output_item.added",
                "output_index": 0,
                "item": {"type": "reasoning", "id": self.reasoning_id, "summary": [], "content": []}
            }))
            .await;
        }
        self.reasoning.push_str(delta);
        self.emit(json!({
            "type": "response.reasoning_text.delta",
            "item_id": self.reasoning_id,
            "output_index": 0,
            "content_index": 0,
            "delta": delta
        }))
        .await;
    }

    async fn on_text(&mut self, delta: &str) {
        if delta.is_empty() {
            return;
        }
        if !self.message_started {
            self.message_started = true;
            self.emit(json!({
                "type": "response.output_item.added",
                "output_index": 1,
                "item": {"type": "message", "role": "assistant", "id": self.message_id, "content": []}
            }))
            .await;
        }
        self.text.push_str(delta);
        self.emit(json!({
            "type": "response.output_text.delta",
            "item_id": self.message_id,
            "output_index": 1,
            "content_index": 0,
            "delta": delta
        }))
        .await;
    }

    /// 处理正文，兼容把思考过程写在 <think>...</think> 里的模型。
    async fn on_content(&mut self, delta: &str) {
        match self.think {
            ThinkState::Normal => self.on_text(delta).await,
            ThinkState::Undecided => {
                self.think_buf.push_str(delta);
                let trimmed = self.think_buf.trim_start();
                if trimmed.is_empty() {
                    return;
                }
                if trimmed.starts_with("<think>") {
                    let rest = trimmed["<think>".len()..].to_owned();
                    self.think_buf.clear();
                    self.think = ThinkState::InThink;
                    Box::pin(self.on_content(&rest)).await;
                } else if "<think>".starts_with(trimmed) {
                    // 还不能确定，继续等待。
                } else {
                    let buf = std::mem::take(&mut self.think_buf);
                    self.think = ThinkState::Normal;
                    self.on_text(&buf).await;
                }
            }
            ThinkState::InThink => {
                self.think_buf.push_str(delta);
                if let Some(pos) = self.think_buf.find("</think>") {
                    let thought = self.think_buf[..pos].to_owned();
                    let rest = self.think_buf[pos + "</think>".len()..].trim_start().to_owned();
                    self.think_buf.clear();
                    self.on_reasoning(&thought).await;
                    self.think = ThinkState::Normal;
                    self.on_text(&rest).await;
                } else {
                    // 保留可能被截断的结束标记，其余作为思考内容输出。
                    let keep = "</think>".len();
                    if self.think_buf.len() > keep {
                        let mut cut = self.think_buf.len() - keep;
                        while !self.think_buf.is_char_boundary(cut) {
                            cut -= 1;
                        }
                        let out: String = self.think_buf.drain(..cut).collect();
                        self.on_reasoning(&out).await;
                    }
                }
            }
        }
    }

    fn on_tool_call_delta(&mut self, position: usize, tc: &Value) {
        let index = tc.get("index").and_then(Value::as_u64).unwrap_or(position as u64);
        let entry = self.tool_calls.entry(index).or_default();
        if let Some(id) = tc.get("id").and_then(Value::as_str).filter(|s| !s.is_empty()) {
            entry.id = id.to_owned();
        }
        if let Some(f) = tc.get("function") {
            if let Some(name) = f.get("name").and_then(Value::as_str).filter(|s| !s.is_empty()) {
                if entry.name.is_empty() {
                    entry.name = name.to_owned();
                } else if !entry.name.ends_with(name) && entry.arguments.is_empty() {
                    entry.name.push_str(name);
                }
            }
            match f.get("arguments") {
                Some(Value::String(a)) => entry.arguments.push_str(a),
                Some(v @ Value::Object(_)) => entry.arguments.push_str(&v.to_string()),
                _ => {}
            }
        }
    }

    async fn on_chunk(&mut self, chunk: &Value) {
        if let Some(err) = chunk.get("error") {
            let message = err
                .get("message")
                .and_then(Value::as_str)
                .map(str::to_owned)
                .unwrap_or_else(|| err.to_string());
            self.fail(&format!("模型接口在输出过程中报错：{message}")).await;
            return;
        }
        if let Some(usage) = chunk.get("usage").filter(|u| u.is_object()) {
            self.usage = Some(usage.clone());
        }
        let Some(choice) = chunk.get("choices").and_then(Value::as_array).and_then(|c| c.first()) else {
            return;
        };
        let delta = choice.get("delta").or_else(|| choice.get("message")).cloned().unwrap_or(Value::Null);
        for key in ["reasoning_content", "reasoning", "thinking"] {
            if let Some(r) = delta.get(key).and_then(Value::as_str) {
                self.on_reasoning(r).await;
                break;
            }
        }
        if let Some(text) = delta.get("content").and_then(Value::as_str) {
            self.on_content(text).await;
        }
        if let Some(calls) = delta.get("tool_calls").and_then(Value::as_array) {
            for (pos, tc) in calls.iter().enumerate() {
                self.on_tool_call_delta(pos, tc);
            }
        }
    }

    async fn fail(&mut self, message: &str) {
        if self.failed || self.finished {
            return;
        }
        self.failed = true;
        self.emit(json!({
            "type": "response.failed",
            "response": {
                "id": self.response_id,
                "status": "failed",
                "error": {"code": "server_error", "message": message}
            }
        }))
        .await;
    }

    fn usage_json(&self) -> Option<Value> {
        let u = self.usage.as_ref()?;
        let input = u.get("prompt_tokens").and_then(Value::as_i64).unwrap_or(0);
        let output = u.get("completion_tokens").and_then(Value::as_i64).unwrap_or(0);
        let total = u.get("total_tokens").and_then(Value::as_i64).unwrap_or(input + output);
        let cached = u
            .pointer("/prompt_tokens_details/cached_tokens")
            .and_then(Value::as_i64)
            .or_else(|| u.get("prompt_cache_hit_tokens").and_then(Value::as_i64))
            .unwrap_or(0);
        let reasoning = u
            .pointer("/completion_tokens_details/reasoning_tokens")
            .and_then(Value::as_i64)
            .unwrap_or(0);
        Some(json!({
            "input_tokens": input,
            "input_tokens_details": {"cached_tokens": cached},
            "output_tokens": output,
            "output_tokens_details": {"reasoning_tokens": reasoning},
            "total_tokens": total
        }))
    }

    async fn finish(&mut self) {
        if self.failed || self.finished {
            return;
        }
        // 流结束时清空未处理完的缓冲。
        match self.think {
            ThinkState::Undecided if !self.think_buf.is_empty() => {
                let buf = std::mem::take(&mut self.think_buf);
                self.on_text(&buf).await;
            }
            ThinkState::InThink if !self.think_buf.is_empty() => {
                let buf = std::mem::take(&mut self.think_buf);
                self.on_reasoning(&buf).await;
            }
            _ => {}
        }
        self.finished = true;

        if self.reasoning_started {
            self.emit(json!({
                "type": "response.output_item.done",
                "output_index": 0,
                "item": {
                    "type": "reasoning",
                    "id": self.reasoning_id,
                    "summary": [],
                    "content": [{"type": "reasoning_text", "text": self.reasoning}]
                }
            }))
            .await;
        }
        let has_tools = !self.tool_calls.is_empty();
        if self.message_started || (!has_tools && !self.reasoning_started) {
            self.emit(json!({
                "type": "response.output_item.done",
                "output_index": 1,
                "item": {
                    "type": "message",
                    "role": "assistant",
                    "id": self.message_id,
                    "content": [{"type": "output_text", "text": self.text}]
                }
            }))
            .await;
        }
        let calls = std::mem::take(&mut self.tool_calls);
        for (n, (_, call)) in calls.into_iter().enumerate() {
            if call.name.is_empty() {
                continue;
            }
            let call_id = if call.id.is_empty() { new_id("call") } else { call.id };
            let arguments = if call.arguments.trim().is_empty() { "{}".to_owned() } else { call.arguments };
            let (namespace, name) = match self.ctx.namespaced.get(&call.name) {
                Some((ns, orig)) => (Some(ns.clone()), orig.clone()),
                None => (None, call.name.clone()),
            };
            let item = if self.ctx.custom_tools.contains(&call.name) {
                let input = serde_json::from_str::<Value>(&arguments)
                    .ok()
                    .and_then(|v| v.get("input").and_then(Value::as_str).map(str::to_owned))
                    .unwrap_or(arguments);
                let mut item = json!({"type": "custom_tool_call", "id": new_id("ctc"), "call_id": call_id, "name": name, "input": input, "status": "completed"});
                if let Some(ns) = namespace {
                    item["namespace"] = Value::String(ns);
                }
                item
            } else {
                let mut item = json!({"type": "function_call", "id": new_id("fc"), "call_id": call_id, "name": name, "arguments": arguments, "status": "completed"});
                if let Some(ns) = namespace {
                    item["namespace"] = Value::String(ns);
                }
                item
            };
            self.emit(json!({"type": "response.output_item.done", "output_index": 2 + n, "item": item})).await;
        }
        let mut response = json!({"id": self.response_id, "status": "completed"});
        if let Some(usage) = self.usage_json() {
            response["usage"] = usage;
        }
        self.emit(json!({"type": "response.completed", "response": response})).await;
    }

    /// 处理流式（SSE）响应。
    pub async fn run_sse(&mut self, resp: reqwest::Response) {
        self.start().await;
        let mut stream = resp.bytes_stream();
        let mut buf: Vec<u8> = Vec::new();
        let mut done = false;
        while let Some(next) = stream.next().await {
            let bytes = match next {
                Ok(b) => b,
                Err(err) => {
                    self.fail(&format!("与模型接口的连接中断：{err}")).await;
                    return;
                }
            };
            buf.extend_from_slice(&bytes);
            while let Some(pos) = buf.iter().position(|b| *b == b'\n') {
                let line_bytes: Vec<u8> = buf.drain(..=pos).collect();
                let line = String::from_utf8_lossy(&line_bytes);
                let line = line.trim();
                let Some(data) = line.strip_prefix("data:") else { continue };
                let data = data.trim();
                if data == "[DONE]" {
                    done = true;
                    break;
                }
                if data.is_empty() {
                    continue;
                }
                match serde_json::from_str::<Value>(data) {
                    Ok(chunk) => self.on_chunk(&chunk).await,
                    Err(err) => tracing::debug!("忽略无法解析的数据块：{err}"),
                }
                if self.failed {
                    return;
                }
            }
            if done {
                break;
            }
        }
        // 处理最后一行没有换行符的情况。
        if !done && !buf.is_empty() {
            let line = String::from_utf8_lossy(&buf).trim().to_owned();
            if let Some(data) = line.strip_prefix("data:").map(str::trim) {
                if data != "[DONE]" {
                    if let Ok(chunk) = serde_json::from_str::<Value>(data) {
                        self.on_chunk(&chunk).await;
                    }
                }
            }
        }
        self.finish().await;
    }

    /// 处理不支持流式、直接返回完整 JSON 的接口。
    pub async fn run_json(&mut self, resp: reqwest::Response) {
        self.start().await;
        match resp.json::<Value>().await {
            Ok(body) => {
                self.on_chunk(&body).await;
                if let Some(calls) = body.pointer("/choices/0/message/tool_calls").and_then(Value::as_array) {
                    // on_chunk 已经通过 message 字段处理过工具调用，这里无需重复。
                    let _ = calls;
                }
                self.finish().await;
            }
            Err(err) => self.fail(&format!("无法解析模型接口的返回内容：{err}")).await,
        }
    }
}

#[cfg(test)]
mod tests;
