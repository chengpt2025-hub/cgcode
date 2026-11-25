//! cgcode 界面主题：iOS 风格的强调色和对话气泡底色。
//!
//! 选择结果保存在 `$CODEX_HOME/cgcode-theme.txt`（默认 `~/.cgcode`），启动时读取。

use std::path::PathBuf;
use std::sync::Once;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub(crate) struct UiTheme {
    pub id: &'static str,
    pub name: &'static str,
    /// 强调色（标题、选中项等）。
    pub accent: (u8, u8, u8),
    /// 对话气泡底色：在终端背景上按比例叠加的颜色。
    pub tint: (u8, u8, u8),
    pub tint_alpha: f32,
    /// 搭配的代码高亮主题。
    pub syntax: &'static str,
}

pub(crate) const THEMES: &[UiTheme] = &[
    UiTheme { id: "ios-light", name: "iOS 浅色", accent: (0, 122, 255), tint: (0, 122, 255), tint_alpha: 0.07, syntax: "github" },
    UiTheme { id: "ios-dark", name: "iOS 深色", accent: (10, 132, 255), tint: (255, 255, 255), tint_alpha: 0.12, syntax: "one-half-dark" },
    UiTheme { id: "midnight", name: "午夜蓝", accent: (100, 210, 255), tint: (40, 70, 160), tint_alpha: 0.35, syntax: "base16-ocean-dark" },
    UiTheme { id: "mint", name: "薄荷绿", accent: (0, 199, 190), tint: (0, 199, 150), tint_alpha: 0.14, syntax: "catppuccin-frappe" },
    UiTheme { id: "sunset", name: "日落橙", accent: (255, 149, 0), tint: (255, 120, 60), tint_alpha: 0.14, syntax: "gruvbox-dark" },
    UiTheme { id: "graphite", name: "石墨灰", accent: (174, 174, 178), tint: (142, 142, 147), tint_alpha: 0.18, syntax: "two-dark" },
    UiTheme { id: "contrast", name: "高对比", accent: (255, 214, 10), tint: (255, 255, 255), tint_alpha: 0.22, syntax: "monokai-extended-bright" },
];

const NONE: usize = usize::MAX;
static CURRENT: AtomicUsize = AtomicUsize::new(NONE);
static INIT: Once = Once::new();

fn theme_file() -> Option<PathBuf> {
    std::env::var_os("CODEX_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .map(|home| home.join("cgcode-theme.txt"))
}

fn init() {
    INIT.call_once(|| {
        let saved = theme_file()
            .and_then(|path| std::fs::read_to_string(path).ok())
            .map(|s| s.trim().to_owned());
        let id = saved.as_deref().unwrap_or("ios-dark");
        if let Some(idx) = THEMES.iter().position(|t| t.id == id) {
            CURRENT.store(idx, Ordering::Relaxed);
        }
    });
}

/// 当前界面主题。
pub(crate) fn current() -> Option<&'static UiTheme> {
    init();
    THEMES.get(CURRENT.load(Ordering::Relaxed))
}

pub(crate) fn current_id() -> Option<&'static str> {
    current().map(|t| t.id)
}

/// 切换（预览）主题，不保存。
pub(crate) fn set_by_id(id: &str) -> Option<&'static UiTheme> {
    init();
    let idx = THEMES.iter().position(|t| t.id == id)?;
    CURRENT.store(idx, Ordering::Relaxed);
    THEMES.get(idx)
}

pub(crate) fn clear() {
    init();
    CURRENT.store(NONE, Ordering::Relaxed);
}

/// 保存选择，下次启动沿用。
pub(crate) fn save(id: &str) {
    if let Some(path) = theme_file() {
        let _ = std::fs::write(path, id);
    }
}
