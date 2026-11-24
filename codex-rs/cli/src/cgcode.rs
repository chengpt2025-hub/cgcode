//! cgcode 启动准备：配置目录、中文首次启动向导、本地模型接口适配层。
//!
//! 用户的模型接口配置保存在 `~/.cgcode/cgcode.toml`：
//!
//! ```toml
//! [api]
//! base_url = "https://api.deepseek.com/v1"
//! api_key = "sk-..."
//! model = "deepseek-chat"
//! wire = "chat"          # chat 或 responses，由向导自动探测
//!
//! # 可选：主接口失败时依次尝试的备用接口
//! [[fallback]]
//! base_url = "https://example.com/v1"
//! api_key = "sk-..."
//! model = "some-model"
//! wire = "chat"
//! ```

use std::collections::HashMap;
use std::io::BufRead;
use std::io::IsTerminal;
use std::io::Write;
use std::path::PathBuf;

use cgcode_chat_bridge::BridgeConfig;
use cgcode_chat_bridge::Upstream;
use cgcode_chat_bridge::UpstreamMode;
use codex_utils_cli::CliConfigOverrides;
use serde::Deserialize;
use serde::Serialize;

const CONFIG_FILE: &str = "cgcode.toml";
const PROVIDER_ID: &str = "cgcode";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiConfig {
    pub base_url: String,
    #[serde(default)]
    pub api_key: String,
    pub model: String,
    #[serde(default = "default_wire")]
    pub wire: String,
}

fn default_wire() -> String {
    "chat".to_owned()
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CgcodeConfig {
    pub api: ApiConfig,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fallback: Vec<ApiConfig>,
    /// 模型上下文窗口（token 数），不填时使用 128000。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_window: Option<i64>,
}

/// 在 tokio 运行时启动之前调用：未设置 CODEX_HOME 时默认使用 `~/.cgcode`。
pub fn init_home() {
    if std::env::var_os("CODEX_HOME").is_some_and(|v| !v.is_empty()) {
        return;
    }
    let Some(home) = std::env::home_dir() else {
        return;
    };
    let dir = home.join(".cgcode");
    if std::fs::create_dir_all(&dir).is_ok() {
        // SAFETY: 在程序入口、尚未创建任何其他线程时调用。
        unsafe { std::env::set_var("CODEX_HOME", &dir) };
    }
}

fn home_dir() -> PathBuf {
    std::env::var_os("CODEX_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::home_dir().map(|h| h.join(".cgcode")))
        .unwrap_or_else(|| PathBuf::from(".cgcode"))
}

fn config_path() -> PathBuf {
    home_dir().join(CONFIG_FILE)
}

fn load_config() -> anyhow::Result<Option<CgcodeConfig>> {
    let path = config_path();
    if !path.exists() {
        return Ok(None);
    }
    let text = std::fs::read_to_string(&path)?;
    let cfg: CgcodeConfig = toml::from_str(&text)
        .map_err(|e| anyhow::anyhow!("配置文件 {} 格式有误：{e}\n可以运行 `cgcode setup` 重新配置。", path.display()))?;
    if cfg.api.base_url.trim().is_empty() || cfg.api.model.trim().is_empty() {
        return Ok(None);
    }
    Ok(Some(cfg))
}

fn save_config(cfg: &CgcodeConfig) -> anyhow::Result<()> {
    let dir = home_dir();
    std::fs::create_dir_all(&dir)?;
    let body = toml::to_string_pretty(cfg)?;
    let text = format!(
        "# cgcode 模型接口配置，可直接编辑，或运行 `cgcode setup` 重新配置。\n\
         # 可以添加 [[fallback]] 段配置备用接口，主接口出错时会自动切换。\n\n{body}"
    );
    std::fs::write(dir.join(CONFIG_FILE), text)?;
    ensure_agents_md(&dir);
    Ok(())
}

/// 首次配置时写入全局 AGENTS.md，要求 cgcode 默认用简体中文交流。
fn ensure_agents_md(dir: &std::path::Path) {
    let path = dir.join("AGENTS.md");
    if path.exists() {
        return;
    }
    let text = "# cgcode 全局约定\n\n\
        - 始终使用简体中文与用户交流，包括计划、说明、总结和提问。\n\
        - 代码、命令、文件名和报错原文保持原样，不要翻译。\n\
        - 代码注释和提交说明默认使用简体中文，除非项目已有其他约定。\n\
        - 动手修改前先说明要做什么；改完简要说明改了哪些文件、为什么改。\n";
    let _ = std::fs::write(path, text);
}

fn prompt(label: &str, default: Option<&str>) -> anyhow::Result<String> {
    let mut stdout = std::io::stdout();
    match default.filter(|d| !d.is_empty()) {
        Some(d) => write!(stdout, "{label}（直接回车使用 {d}）：")?,
        None => write!(stdout, "{label}：")?,
    }
    stdout.flush()?;
    let mut line = String::new();
    let read = std::io::stdin().lock().read_line(&mut line)?;
    if read == 0 {
        anyhow::bail!("输入已结束，配置未完成");
    }
    let value = line.trim().to_owned();
    Ok(if value.is_empty() {
        default.unwrap_or_default().to_owned()
    } else {
        value
    })
}

fn mask_key(key: &str) -> String {
    let chars: Vec<char> = key.chars().collect();
    if chars.len() <= 8 {
        return "已设置".to_owned();
    }
    let head: String = chars[..4].iter().collect();
    let tail: String = chars[chars.len() - 4..].iter().collect();
    format!("{head}…{tail}")
}

/// 中文首次启动向导。
pub async fn run_setup() -> anyhow::Result<CgcodeConfig> {
    let existing = load_config().ok().flatten();
    println!();
    println!("  ╭──────────────────────────────────────────╮");
    println!("  │   欢迎使用 cgcode                          │");
    println!("  │   先连接你的模型接口，只需要填三项。       │");
    println!("  ╰──────────────────────────────────────────╯");
    println!();
    println!("  支持兼容 OpenAI 格式的接口，例如 DeepSeek、通义千问、各类中转站。");
    println!("  配置保存在 {}，Key 只存在你自己的电脑上。", config_path().display());
    println!();
    loop {
        let prev = existing.as_ref().map(|c| c.api.clone()).unwrap_or_default();
        let base_url = prompt("① 接口地址（例如 https://api.deepseek.com/v1）", Some(&prev.base_url))?;
        if base_url.is_empty() {
            println!("  接口地址不能为空。\n");
            continue;
        }
        let key_hint = if prev.api_key.is_empty() { None } else { Some(mask_key(&prev.api_key)) };
        let key_input = prompt("② API Key", key_hint.as_deref())?;
        let api_key = match key_hint {
            Some(ref masked) if key_input == *masked => prev.api_key.clone(),
            _ => key_input,
        };
        let model = prompt("③ 模型名（例如 deepseek-chat）", Some(&prev.model))?;
        if model.is_empty() {
            println!("  模型名不能为空。\n");
            continue;
        }
        println!("\n  正在测试连接，请稍候……");
        match cgcode_chat_bridge::probe(&base_url, &api_key, &model).await {
            Ok((normalized, mode)) => {
                let wire = match mode {
                    UpstreamMode::ChatCompletions => "chat",
                    UpstreamMode::Responses => "responses",
                };
                println!("  ✓ 连接成功（接口类型：{}）\n", if wire == "chat" { "Chat Completions" } else { "Responses" });
                let cfg = CgcodeConfig {
                    api: ApiConfig { base_url: normalized, api_key, model, wire: wire.to_owned() },
                    fallback: existing.as_ref().map(|c| c.fallback.clone()).unwrap_or_default(),
                    context_window: existing.as_ref().and_then(|c| c.context_window),
                };
                save_config(&cfg)?;
                println!("  配置已保存。之后直接运行 cgcode 即可开始；想修改时运行 cgcode setup。\n");
                return Ok(cfg);
            }
            Err(err) => {
                println!("  ✗ 连接失败：{err}\n");
                let again = prompt("  重新填写？(Y/n)", Some("Y"))?;
                if again.eq_ignore_ascii_case("n") {
                    anyhow::bail!("配置未完成，可以稍后运行 cgcode setup 重新配置");
                }
                println!();
            }
        }
    }
}

fn to_upstream(api: &ApiConfig) -> Upstream {
    Upstream {
        base_url: api.base_url.trim().to_owned(),
        api_key: Some(api.api_key.trim().to_owned()).filter(|k| !k.is_empty()),
        mode: UpstreamMode::parse(&api.wire).unwrap_or(UpstreamMode::ChatCompletions),
        model: Some(api.model.trim().to_owned()).filter(|m| !m.is_empty()),
        extra_body: serde_json::Map::new(),
        headers: HashMap::new(),
    }
}

fn toml_str(value: &str) -> String {
    toml::Value::String(value.to_owned()).to_string()
}

/// 启动前准备：读取（或通过向导创建）模型接口配置，启动本地适配层，
/// 并把 Codex 的模型提供方指向它。
pub async fn prepare(is_interactive: bool, overrides: &mut CliConfigOverrides) -> anyhow::Result<()> {
    let cfg = match load_config()? {
        Some(cfg) => cfg,
        None => {
            if is_interactive && std::io::stdin().is_terminal() && std::io::stdout().is_terminal() {
                run_setup().await?
            } else {
                eprintln!("cgcode 还没有配置模型接口，请先运行 `cgcode setup`。");
                return Ok(());
            }
        }
    };
    let mut upstreams = vec![to_upstream(&cfg.api)];
    upstreams.extend(cfg.fallback.iter().filter(|f| !f.base_url.trim().is_empty()).map(to_upstream));
    let addr = cgcode_chat_bridge::spawn(BridgeConfig {
        upstreams,
        echo_reasoning: true,
        connect_timeout: std::time::Duration::from_secs(30),
    })
    .await
    .map_err(|e| anyhow::anyhow!("无法启动本地模型适配服务：{e}"))?;

    let base_url = format!("http://{addr}/v1");
    let entries = [
        ("model_provider".to_owned(), toml_str(PROVIDER_ID)),
        ("model".to_owned(), toml_str(cfg.api.model.trim())),
        (format!("model_providers.{PROVIDER_ID}.name"), toml_str("cgcode")),
        (format!("model_providers.{PROVIDER_ID}.base_url"), toml_str(&base_url)),
        (format!("model_providers.{PROVIDER_ID}.wire_api"), toml_str("responses")),
        (format!("model_providers.{PROVIDER_ID}.stream_idle_timeout_ms"), "600000".to_owned()),
        ("model_context_window".to_owned(), cfg.context_window.unwrap_or(128_000).to_string()),
    ];
    // 放在最前面，用户在命令行里用 -c 指定的同名项可以覆盖这些默认值。
    let mut merged: Vec<String> = entries.into_iter().map(|(k, v)| format!("{k}={v}")).collect();
    merged.append(&mut overrides.raw_overrides);
    overrides.raw_overrides = merged;
    Ok(())
}
