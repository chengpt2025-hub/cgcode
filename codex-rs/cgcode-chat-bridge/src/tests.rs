use super::*;

fn chat_upstream() -> Upstream {
    Upstream {
        base_url: "http://example.invalid/v1".into(),
        api_key: Some("k".into()),
        mode: UpstreamMode::ChatCompletions,
        model: None,
        extra_body: Map::new(),
        headers: HashMap::new(),
    }
}

#[test]
fn translates_messages_tools_and_tool_results() {
    let req = json!({
        "model": "deepseek-chat",
        "instructions": "你是 cgcode",
        "input": [
            {"type": "message", "role": "developer", "content": [{"type": "input_text", "text": "环境信息"}]},
            {"type": "message", "role": "user", "content": [{"type": "input_text", "text": "列出文件"}]},
            {"type": "reasoning", "summary": [], "content": [{"type": "reasoning_text", "text": "想一想"}]},
            {"type": "function_call", "name": "shell", "arguments": "{\"command\":[\"ls\"]}", "call_id": "c1"},
            {"type": "custom_tool_call", "name": "apply_patch", "input": "*** Begin Patch", "call_id": "c2"},
            {"type": "function_call_output", "call_id": "c1", "output": "a.txt"},
            {"type": "message", "role": "assistant", "content": [{"type": "output_text", "text": "好了"}]}
        ],
        "tools": [
            {"type": "function", "name": "shell", "description": "运行命令", "parameters": {"type": "object", "properties": {}}},
            {"type": "custom", "name": "apply_patch", "description": "改文件", "format": {"type": "grammar", "definition": "start: x"}},
            {"type": "namespace", "name": "mcp", "description": "", "tools": [{"type": "function", "name": "search", "parameters": {"type": "object"}}]},
            {"type": "web_search"}
        ],
        "parallel_tool_calls": true
    });
    let (chat, ctx) = translate_request(&req, &chat_upstream(), true);
    let msgs = chat["messages"].as_array().unwrap();
    assert_eq!(msgs[0]["role"], "system");
    assert!(msgs[0]["content"].as_str().unwrap().contains("环境信息"));
    assert_eq!(msgs[1]["content"], "列出文件");
    assert_eq!(msgs[2]["role"], "assistant");
    assert_eq!(msgs[2]["tool_calls"].as_array().unwrap().len(), 2);
    assert_eq!(msgs[2]["reasoning_content"], "想一想");
    assert_eq!(msgs[3]["role"], "tool");
    assert_eq!(msgs[3]["tool_call_id"], "c1");
    // c2 没有结果，应自动补上占位结果
    assert_eq!(msgs[4]["tool_call_id"], "c2");
    assert_eq!(msgs[5]["content"], "好了");
    let tools = chat["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 3);
    assert!(ctx.custom_tools.contains("apply_patch"));
    assert_eq!(ctx.namespaced.get("mcp__search").unwrap().1, "search");
    assert_eq!(chat["stream"], true);
}

async fn collect(rx: mpsc::Receiver<Result<Bytes, std::io::Error>>) -> Vec<Value> {
    let mut stream = ReceiverStream::new(rx);
    let mut out = Vec::new();
    while let Some(Ok(b)) = stream.next().await {
        let s = String::from_utf8_lossy(&b).to_string();
        for line in s.lines() {
            if let Some(d) = line.strip_prefix("data: ") {
                out.push(serde_json::from_str(d).unwrap());
            }
        }
    }
    out
}

#[tokio::test]
async fn converts_stream_with_think_tags_and_tool_calls() {
    let mut ctx = RequestContext::default();
    ctx.custom_tools.insert("apply_patch".into());
    let (tx, rx) = mpsc::channel(256);
    let handle = tokio::spawn(collect(rx));
    let mut conv = Converter::new(ctx, tx);
    conv.start().await;
    for c in [
        json!({"choices": [{"delta": {"content": "<thi"}}]}),
        json!({"choices": [{"delta": {"content": "nk>先分析</th"}}]}),
        json!({"choices": [{"delta": {"content": "ink>结论"}}]}),
        json!({"choices": [{"delta": {"tool_calls": [{"index": 0, "id": "t1", "function": {"name": "apply_patch", "arguments": "{\"inp"}}]}}]}),
        json!({"choices": [{"delta": {"tool_calls": [{"index": 0, "function": {"arguments": "ut\":\"PATCH\"}"}}]}}]}),
        json!({"choices": [], "usage": {"prompt_tokens": 10, "completion_tokens": 5, "total_tokens": 15}}),
    ] {
        conv.on_chunk(&c).await;
    }
    conv.finish().await;
    drop(conv);
    let events = handle.await.unwrap();
    let kinds: Vec<&str> = events.iter().map(|e| e["type"].as_str().unwrap()).collect();
    assert_eq!(kinds.first(), Some(&"response.created"));
    assert_eq!(kinds.last(), Some(&"response.completed"));
    let reasoning: String = events
        .iter()
        .filter(|e| e["type"] == "response.reasoning_text.delta")
        .map(|e| e["delta"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(reasoning, "先分析");
    let text: String = events
        .iter()
        .filter(|e| e["type"] == "response.output_text.delta")
        .map(|e| e["delta"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(text, "结论");
    let call = events
        .iter()
        .find(|e| e["type"] == "response.output_item.done" && e["item"]["type"] == "custom_tool_call")
        .unwrap();
    assert_eq!(call["item"]["input"], "PATCH");
    assert_eq!(call["item"]["call_id"], "t1");
    assert_eq!(events.last().unwrap()["response"]["usage"]["total_tokens"], 15);
}
