use crawrad_note::note;
#[note("session-shelltype")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellType {
    Cmd,
    Pwsh,
    Wsl,
    GitBash,
}

impl ShellType {
    pub const ALL: &'static [Self] = &[Self::Cmd, Self::Pwsh, Self::Wsl, Self::GitBash];

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Cmd => "cmd",
            Self::Pwsh => "pwsh",
            Self::Wsl => "wsl",
            Self::GitBash => "git-bash",
        }
    }

    pub fn executable(self) -> &'static str {
        match self {
            Self::Cmd => "cmd.exe",
            Self::Pwsh => "powershell.exe",
            Self::Wsl => "wsl.exe",
            Self::GitBash => "bash.exe",
        }
    }

    pub fn default_args(self) -> &'static [&'static str] {
        match self {
            Self::Cmd => &[],
            Self::Pwsh => &["-NoExit"],
            Self::Wsl => &[],
            Self::GitBash => &["-l", "-i"],
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Cmd => "⌘",
            Self::Pwsh => "⚡",
            Self::Wsl => "🐧",
            Self::GitBash => "🌿",
        }
    }

    pub fn welcome_banner(self, cwd: &str) -> String {
        match self {
            Self::Pwsh => format!(
                "Aefos Terminal — PowerShell 7 active\nWorking Directory: {cwd}\n\nPS {cwd}> "
            ),
            Self::Cmd => format!(
                "Aefos Terminal — Command Prompt (cmd.exe) active\nWorking Directory: {cwd}\n\n{cwd}> "
            ),
            Self::Wsl => format!(
                "Aefos Terminal — WSL (Ubuntu/Linux) active\nWorking Directory: {cwd}\n\nuser@aefos:{cwd}$ "
            ),
            Self::GitBash => format!(
                "Aefos Terminal — Git Bash active\nWorking Directory: {cwd}\n\nuser@aefos MINGW64 {cwd} (main)\n$ "
            ),
        }
    }
}

#[note("session-terminalsession")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalSession {
    pub id: String,
    pub title: String,
    pub shell: Option<ShellType>,
    pub cwd: String,
    pub timestamp: String,
    pub is_welcome: bool,
    pub buffer: Vec<String>,
}

impl TerminalSession {
    pub fn new_welcome() -> Self {
        Self {
            id: "welcome".to_string(),
            title: "Welcome".to_string(),
            shell: None,
            cwd: String::new(),
            timestamp: String::new(),
            is_welcome: true,
            buffer: Vec::new(),
        }
    }

    pub fn new_shell(shell: ShellType, cwd: impl Into<String>) -> Self {
        let cwd_str = cwd.into();
        let title = shell.display_name().to_string();
        let id = format!("{}-{}", title, fastrand_id());
        let timestamp = chrono_like_timestamp();
        let mut buffer = Vec::new();
        buffer.push(format!("Aefos Terminal — {} active", title));
        buffer.push(format!("Working Directory: {}", cwd_str));
        buffer.push(String::new());

        Self {
            id,
            title,
            shell: Some(shell),
            cwd: cwd_str,
            timestamp,
            is_welcome: false,
            buffer,
        }
    }

    pub fn append_output(&mut self, text: &str) {
        for line in text.lines() {
            self.buffer.push(line.to_string());
        }
    }
}

fn fastrand_id() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let d = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    (d.subsec_nanos() % 9000) + 1000
}

fn chrono_like_timestamp() -> String {
    // Generates a readable timestamp string without requiring chrono crate
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let hours = (secs / 3600) % 24;
    let minutes = (secs / 60) % 60;
    format!("{hours:02}:{minutes:02}")
}
