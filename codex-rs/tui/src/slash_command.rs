use strum::IntoEnumIterator;
use strum_macros::AsRefStr;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

/// Commands that can be invoked by starting a message with a leading slash.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, EnumIter, AsRefStr, IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum SlashCommand {
    // DO NOT ALPHA-SORT! Enum order is presentation order in the popup, so
    // more frequently used commands should be listed first.
    Model,
    Presets,
    Daybreak,
    Ide,
    Permissions,
    Keymap,
    Vim,
    #[strum(serialize = "setup-default-sandbox")]
    ElevateSandbox,
    Experimental,
    #[strum(to_string = "approve")]
    AutoReview,
    Memories,
    Skills,
    Import,
    Hooks,
    Review,
    Rename,
    New,
    Archive,
    Delete,
    Resume,
    Fork,
    Worktree,
    App,
    Init,
    Compact,
    Config,
    Recap,
    Plan,
    Voice,
    Goal,
    Agents,
    Side,
    Btw,
    Copy,
    Export,
    Raw,
    Tui,
    Diff,
    Mention,
    Status,
    Daemon,
    #[strum(to_string = "rc", serialize = "remote")]
    Rc,
    Warnings,
    Cd,
    #[strum(to_string = "pwd", serialize = "cwd")]
    Pwd,
    Usage,
    DebugConfig,
    Title,
    Statusline,
    Theme,
    #[strum(to_string = "pets", serialize = "pet")]
    Pets,
    Mcp,
    Apps,
    Plugins,
    Logout,
    Quit,
    Exit,
    Feedback,
    Rollout,
    Ps,
    #[strum(to_string = "stop", serialize = "clean")]
    Stop,
    Clear,
    TestApproval,
    #[strum(serialize = "subagents")]
    MultiAgents,
    // Debugging commands.
    #[strum(serialize = "debug-m-drop")]
    MemoryDrop,
    #[strum(serialize = "debug-m-update")]
    MemoryUpdate,
}

impl SlashCommand {
    /// User-visible description shown in the popup.
    pub fn description(self) -> &'static str {
        match self {
            SlashCommand::Config => "设置 cgcode 偏好",
            SlashCommand::Feedback => "把日志保存为 ZIP 压缩包",
            SlashCommand::New => "开始一段新对话",
            SlashCommand::Init => "为当前项目生成 AGENTS.md 说明文件",
            SlashCommand::Compact => "压缩对话，避免超出上下文长度",
            SlashCommand::Recap => "立即总结当前对话",
            SlashCommand::Review => "审查当前改动并找出问题",
            SlashCommand::Rename => "重命名当前对话",
            SlashCommand::Resume => "继续之前保存的对话",
            SlashCommand::Archive => "归档当前会话",
            SlashCommand::Delete => "永久删除当前会话",
            SlashCommand::Clear => "清屏并开始新对话",
            SlashCommand::Fork => "从当前对话分出一个副本",
            SlashCommand::Worktree => "在新的 git 工作树中开始或继续对话",
            SlashCommand::App => "在桌面版中继续当前会话",
            SlashCommand::Quit | SlashCommand::Exit => "退出 cgcode",
            SlashCommand::Copy => "复制最后一条回复（或其中一部分）",
            SlashCommand::Export => "把对话导出为 Markdown",
            SlashCommand::Raw => "切换原始滚动模式，方便在终端里选中复制",
            SlashCommand::Tui => "选择下次启动的界面模式",
            SlashCommand::Diff => "查看 git 改动（含未跟踪文件）",
            SlashCommand::Mention => "引用一个文件",
            SlashCommand::Skills => "使用技能，提升特定任务的完成质量",
            SlashCommand::Import => "从 Claude Code 导入设置、项目和最近对话",
            SlashCommand::Hooks => "查看和管理生命周期钩子",
            SlashCommand::Daemon => "管理本地后台服务",
            SlashCommand::Rc => "管理本地后台服务的远程控制",
            SlashCommand::Warnings => "查看警告和诊断详情",
            SlashCommand::Status => "查看当前会话配置和 token 用量",
            SlashCommand::Cd => "切换当前工作目录",
            SlashCommand::Pwd => "显示当前工作目录",
            SlashCommand::Usage => "查看账户用量",
            SlashCommand::DebugConfig => "显示配置层级（调试用）",
            SlashCommand::Title => "设置终端标题显示的内容",
            SlashCommand::Statusline => "设置状态栏显示的内容",
            SlashCommand::Theme => "切换界面主题和代码高亮（实时预览）",
            SlashCommand::Pets => "选择或隐藏终端宠物",
            SlashCommand::Ps => "列出后台终端",
            SlashCommand::Stop => "停止所有后台终端",
            SlashCommand::MemoryDrop => "DO NOT USE",
            SlashCommand::MemoryUpdate => "DO NOT USE",
            SlashCommand::Model => "选择模型和思考强度",
            SlashCommand::Presets => "运行或保存常用提示词和命令",
            SlashCommand::Daybreak => "turn Daybreak on or off",
            SlashCommand::Ide => {
                "带上编辑器中的选中内容、打开的文件等上下文"
            }
            SlashCommand::Plan => "切换到计划模式",
            SlashCommand::Voice => "开启或关闭语音；/voice settings 选择声音",
            SlashCommand::Goal => "设置或查看长任务的目标",
            SlashCommand::Agents => "打开智能体控制中心",
            SlashCommand::MultiAgents => "在本会话的子智能体之间切换",
            SlashCommand::Side | SlashCommand::Btw => "开一个旁支对话",
            SlashCommand::Permissions => "设置 cgcode 可以执行哪些操作",
            SlashCommand::Keymap => "自定义快捷键",
            SlashCommand::Vim => "切换输入框的 Vim 模式",
            SlashCommand::ElevateSandbox => "设置高权限沙箱",
            SlashCommand::Experimental => "开关实验功能",
            SlashCommand::AutoReview => "允许重试一次被自动审查拒绝的操作",
            SlashCommand::Memories => "设置记忆的使用和生成",
            SlashCommand::Mcp => {
                "列出 MCP 工具；/mcp verbose 查看详情，/mcp login <名称> 登录"
            }
            SlashCommand::Apps => "管理应用",
            SlashCommand::Plugins => "浏览插件",
            SlashCommand::Logout => "退出登录",
            SlashCommand::Rollout => "显示会话记录文件路径",
            SlashCommand::TestApproval => "test approval request",
        }
    }

    /// Command string without the leading '/'. Provided for compatibility with
    /// existing code that expects a method named `command()`.
    pub fn command(self) -> &'static str {
        self.into()
    }

    /// Whether this command supports inline args (for example `/review ...`).
    pub fn supports_inline_args(self) -> bool {
        matches!(
            self,
            SlashCommand::Review
                | SlashCommand::Model
                | SlashCommand::Permissions
                | SlashCommand::Title
                | SlashCommand::Statusline
                | SlashCommand::Rename
                | SlashCommand::New
                | SlashCommand::Clear
                | SlashCommand::Fork
                | SlashCommand::Plan
                | SlashCommand::Goal
                | SlashCommand::Voice
                | SlashCommand::Ide
                | SlashCommand::Keymap
                | SlashCommand::Mcp
                | SlashCommand::Export
                | SlashCommand::Raw
                | SlashCommand::Cd
                | SlashCommand::Pwd
                | SlashCommand::Usage
                | SlashCommand::Pets
                | SlashCommand::Side
                | SlashCommand::Btw
                | SlashCommand::Resume
                | SlashCommand::Rc
        )
    }

    /// Whether this command remains available inside an active side conversation.
    pub fn available_in_side_conversation(self) -> bool {
        matches!(
            self,
            SlashCommand::Copy
                | SlashCommand::Agents
                | SlashCommand::Export
                | SlashCommand::Raw
                | SlashCommand::Diff
                | SlashCommand::Mention
                | SlashCommand::Status
                | SlashCommand::Daemon
                | SlashCommand::Rc
                | SlashCommand::Warnings
                | SlashCommand::Pwd
                | SlashCommand::Usage
                | SlashCommand::Ide
                | SlashCommand::Rename
        )
    }

    /// Whether dispatch needs thread state to validate this command before consuming its draft.
    /// The composer must defer busy-state rejection and draft clearing for these commands.
    pub(crate) fn requires_dispatch_validation(self) -> bool {
        matches!(self, SlashCommand::Fork | SlashCommand::Review)
    }

    /// Commands the TUI can handle locally while disconnected from its app server.
    pub(crate) fn available_when_disconnected(self) -> bool {
        matches!(
            self,
            Self::Warnings | Self::Config | Self::Rc | Self::Feedback | Self::Quit | Self::Exit
        )
    }

    /// Commands that do not require a writable current thread. The server must still be connected.
    pub(crate) fn available_when_thread_unavailable(self) -> bool {
        matches!(
            self,
            SlashCommand::New
                | SlashCommand::Clear
                | SlashCommand::Resume
                | SlashCommand::Agents
                | SlashCommand::MultiAgents
                | SlashCommand::Quit
                | SlashCommand::Exit
                | SlashCommand::Status
                | SlashCommand::Rc
                | SlashCommand::Warnings
                | SlashCommand::DebugConfig
                | SlashCommand::Pwd
                | SlashCommand::Rollout
                | SlashCommand::Copy
                | SlashCommand::Raw
        )
    }

    /// Whether this command can be run while a task is in progress.
    pub fn available_during_task(self) -> bool {
        match self {
            SlashCommand::New
            | SlashCommand::Delete
            | SlashCommand::Fork
            | SlashCommand::Worktree
            | SlashCommand::Init
            | SlashCommand::Compact
            | SlashCommand::Recap
            | SlashCommand::Export
            | SlashCommand::Keymap
            | SlashCommand::Tui
            | SlashCommand::Vim
            | SlashCommand::ElevateSandbox
            | SlashCommand::Experimental
            | SlashCommand::Memories
            | SlashCommand::Import
            | SlashCommand::Review
            | SlashCommand::Plan
            | SlashCommand::Cd
            | SlashCommand::Clear
            | SlashCommand::Logout
            | SlashCommand::MemoryDrop
            | SlashCommand::MemoryUpdate => false,
            SlashCommand::Diff
            | SlashCommand::Archive
            | SlashCommand::Resume
            | SlashCommand::Model
            | SlashCommand::Presets
            | SlashCommand::Daybreak
            | SlashCommand::Permissions
            | SlashCommand::Copy
            | SlashCommand::Raw
            | SlashCommand::Rename
            | SlashCommand::Mention
            | SlashCommand::Skills
            | SlashCommand::Hooks
            | SlashCommand::Status
            | SlashCommand::Daemon
            | SlashCommand::Rc
            | SlashCommand::Warnings
            | SlashCommand::Pwd
            | SlashCommand::Usage
            | SlashCommand::DebugConfig
            | SlashCommand::Ps
            | SlashCommand::Stop
            | SlashCommand::App
            | SlashCommand::Goal
            | SlashCommand::Voice
            | SlashCommand::Mcp
            | SlashCommand::Apps
            | SlashCommand::Plugins
            | SlashCommand::Title
            | SlashCommand::Statusline
            | SlashCommand::AutoReview
            | SlashCommand::Feedback
            | SlashCommand::Ide
            | SlashCommand::Quit
            | SlashCommand::Exit
            | SlashCommand::Side
            | SlashCommand::Btw => true,
            SlashCommand::Rollout => true,
            SlashCommand::TestApproval => true,
            SlashCommand::Agents | SlashCommand::MultiAgents => true,
            SlashCommand::Config | SlashCommand::Theme | SlashCommand::Pets => false,
        }
    }

    fn is_visible(self) -> bool {
        match self {
            SlashCommand::Copy => !cfg!(target_os = "android"),
            SlashCommand::App => cfg!(any(target_os = "macos", target_os = "windows")),
            SlashCommand::Voice => true,
            SlashCommand::Rollout | SlashCommand::TestApproval => cfg!(debug_assertions),
            _ => true,
        }
    }
}

/// Return all built-in commands in a Vec paired with their command string.
pub fn built_in_slash_commands() -> Vec<(&'static str, SlashCommand)> {
    SlashCommand::iter()
        .filter(|command| command.is_visible())
        .map(|c| (c.command(), c))
        .collect()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    use super::SlashCommand;

    #[test]
    fn stop_command_is_canonical_name() {
        assert_eq!(SlashCommand::Stop.command(), "stop");
    }

    #[test]
    fn clean_alias_parses_to_stop_command() {
        assert_eq!(SlashCommand::from_str("clean"), Ok(SlashCommand::Stop));
    }

    #[test]
    fn pet_alias_parses_to_pets_command() {
        assert_eq!(SlashCommand::Pets.command(), "pets");
        assert_eq!(SlashCommand::from_str("pet"), Ok(SlashCommand::Pets));
    }

    #[test]
    fn certain_commands_are_available_during_task() {
        assert!(SlashCommand::Goal.available_during_task());
        assert!(SlashCommand::Ide.available_during_task());
        assert!(SlashCommand::Title.available_during_task());
        assert!(SlashCommand::Statusline.available_during_task());
        assert!(SlashCommand::Raw.available_during_task());
        assert!(SlashCommand::Raw.available_in_side_conversation());
        assert!(SlashCommand::Raw.supports_inline_args());
        assert!(SlashCommand::App.available_during_task());
    }

    #[test]
    fn auto_review_command_is_approve() {
        assert_eq!(SlashCommand::AutoReview.command(), "approve");
        assert_eq!(
            SlashCommand::from_str("approve"),
            Ok(SlashCommand::AutoReview)
        );
    }
}
