//! Project Options › Log: what the project's log writes, where, and how much.
//! Pure: the manifest I/O lives in `crawrad_project::project_options`.

/// How much a build writes, least to most quiet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Off,
}

impl LogLevel {
    /// Every level in the order the drop-down lists them.
    pub const ALL: [Self; 5] = [
        Self::Debug,
        Self::Info,
        Self::Warning,
        Self::Error,
        Self::Off,
    ];

    /// The word in the manifest, on the command line and in the env variable.
    pub fn key(self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warning => "warn",
            Self::Error => "error",
            Self::Off => "off",
        }
    }

    /// The drop-down's label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Debug => "Debug",
            Self::Info => "Info",
            Self::Warning => "Warning",
            Self::Error => "Error",
            Self::Off => "Off",
        }
    }

    /// The level a manifest word names; `warning` is taken for `warn`.
    pub fn from_key(key: &str) -> Option<Self> {
        match key.trim().to_ascii_lowercase().as_str() {
            "warning" => Some(Self::Warning),
            k => Self::ALL.into_iter().find(|l| l.key() == k),
        }
    }
}

/// Where the log files go.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogFolder {
    /// Beside the executable (often not writable under Program Files).
    Executable,
    /// `%APPDATA%\<app>\logs` — the default.
    UserData,
    /// A folder the developer names.
    Custom(String),
}

/// When a new log file starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewFile {
    EveryDay,
    EveryRun,
}

impl NewFile {
    /// Both choices in the drop-down's order.
    pub const ALL: [Self; 2] = [Self::EveryDay, Self::EveryRun];

    /// The word in the manifest.
    pub fn key(self) -> &'static str {
        match self {
            Self::EveryDay => "day",
            Self::EveryRun => "run",
        }
    }

    /// The drop-down's label.
    pub fn label(self) -> &'static str {
        match self {
            Self::EveryDay => "Every day",
            Self::EveryRun => "Every run",
        }
    }
}

/// The markers a file name can carry, in the order the page offers them.
pub const MARKERS: [&str; 5] = ["{app}", "{date}", "{time}", "{pid}", "{user}"];

/// The most files a project keeps.
pub const MAX_KEEP: u32 = 999;

/// A project's log settings (`[package.metadata.crawrad.log]`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogOptions {
    pub enabled: bool,
    pub file_name: String,
    pub folder: LogFolder,
    pub debug_level: LogLevel,
    pub release_level: LogLevel,
    pub new_file: NewFile,
    pub keep: u32,
}

impl Default for LogOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            file_name: "{app}_{date}.log".to_string(),
            folder: LogFolder::UserData,
            debug_level: LogLevel::Info,
            release_level: LogLevel::Warning,
            new_file: NewFile::EveryDay,
            keep: 30,
        }
    }
}

/// What the markers stand for when a name is shown or written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkerValues<'a> {
    pub app: &'a str,
    /// `YYYY-MM-DD`.
    pub date: &'a str,
    /// `HHMMSS`.
    pub time: &'a str,
    pub pid: u32,
    pub user: &'a str,
}

/// The file name with its markers filled in.
pub fn expand_file_name(pattern: &str, v: &MarkerValues<'_>) -> String {
    pattern
        .replace("{app}", v.app)
        .replace("{date}", v.date)
        .replace("{time}", v.time)
        .replace("{pid}", &v.pid.to_string())
        .replace("{user}", v.user)
}

/// The user's data folder for an app, as the page shows it.
pub fn user_data_folder(app: &str) -> String {
    format!("%APPDATA%\\{app}\\logs")
}

/// The environment variable the support team sets at the customer.
pub fn env_variable(app: &str) -> String {
    let name: String = app
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect();
    format!("{name}_LOG")
}

/// The override file beside the executable.
pub fn override_file(app: &str) -> String {
    format!("{app}.log.toml")
}

/// Why the settings cannot be saved, if they cannot.
pub fn check(opts: &LogOptions) -> Result<(), String> {
    let name = opts.file_name.trim();
    if name.is_empty() {
        return Err("Log: the file name is empty".to_string());
    }
    if name.contains(['/', '\\', ':']) {
        return Err("Log: the file name cannot hold a folder — pick the folder below".to_string());
    }
    if opts.new_file == NewFile::EveryDay && !name.contains("{date}") {
        return Err("Log: a new file every day needs {date} in the name".to_string());
    }
    if opts.new_file == NewFile::EveryRun && !name.contains("{time}") && !name.contains("{pid}") {
        return Err("Log: a new file every run needs {time} or {pid} in the name".to_string());
    }
    if let LogFolder::Custom(f) = &opts.folder
        && f.trim().is_empty()
    {
        return Err("Log: name the custom folder or pick another".to_string());
    }
    if !(1..=MAX_KEEP).contains(&opts.keep) {
        return Err(format!("Log: keep between 1 and {MAX_KEEP} files"));
    }
    Ok(())
}

/// The sample override file: the same settings, ready to edit at the customer.
pub fn sample_override(app: &str, opts: &LogOptions) -> String {
    let folder = match &opts.folder {
        LogFolder::Executable => ".".to_string(),
        LogFolder::UserData => user_data_folder(app),
        LogFolder::Custom(f) => f.clone(),
    };
    let quote = |s: &str| s.replace('\\', "\\\\").replace('"', "\\\"");
    format!(
        "# {app} log override: beside {app}.exe, read at start-up.\n\
         # --log=<level> and {env} come first; this file comes next.\n\
         # level: debug, info, warn, error or off\n\
         level = \"{level}\"\n\
         folder = \"{folder}\"\n\
         file_name = \"{name}\"\n",
        env = env_variable(app),
        level = opts.release_level.key(),
        folder = quote(&folder),
        name = quote(&opts.file_name),
    )
}

#[cfg(test)]
#[path = "test/project_log/test.rs"]
mod project_log_test;
