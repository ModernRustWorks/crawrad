//! Pure state model for the Project Options dialog.

use std::path::PathBuf;

use crate::dialog_fields::{DialogFields, FieldKind, FieldValue};
use crate::project_log::{
    LogFolder, LogLevel, LogOptions, MarkerValues, NewFile, env_variable, expand_file_name,
    override_file, user_data_folder,
};

/// Default caption displayed for the built-in icon.
pub const DEFAULT_CAPTION: &str = "CrawRAD crab (default)";

/// The source of an application icon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IconSource {
    /// Use the built-in default CrawRAD crab icon.
    Default,
    /// Use a custom icon file on disk.
    File(PathBuf),
}

/// The pages available in the Project Options dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    Application,
    Log,
}

impl Page {
    /// Every page in display order.
    pub const ALL: &'static [Self] = &[Self::Application, Self::Log];

    /// User-visible label for the page.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Application => "Application",
            Self::Log => "Log",
        }
    }
}

/// Pure state of the Project Options dialog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectOptionsState {
    /// Available pages in the left list.
    pub pages: Vec<Page>,
    /// Currently active page.
    pub current_page: Page,
    /// Editable application title.
    pub title: String,
    /// Pending icon choice before OK is pressed.
    pub pending_icon: Option<IconSource>,
    /// Caption text under icon controls.
    pub caption: String,
    /// Optional error message to display in the dialog.
    pub error: Option<String>,
    /// The dialog's title bar: `Project Options — {package}`.
    pub heading: String,
    /// The Log page's settings, as edited.
    pub log: LogOptions,
    /// The custom folder's text, kept while another folder is picked.
    pub custom_folder: String,
    /// The "Keep the last" box's text, as typed.
    pub keep_text: String,
    /// The package's name: the `{app}` marker and the support names.
    pub package: String,
}

/// The Log page's derived lines, for the labels that only show them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogPageText {
    /// `Today: <name>`.
    pub today: String,
    /// `%APPDATA%\<app>\logs`.
    pub user_folder: String,
    /// The precedence box's three overrides, first wins.
    pub command_line: String,
    pub env_variable: String,
    pub override_file: String,
    /// The sample button's caption.
    pub sample: String,
}

/// The Log page's folder choices, top to bottom.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FolderChoice {
    Executable,
    UserData,
    Custom,
}

impl FolderChoice {
    /// The radios' order.
    pub const ALL: [Self; 3] = [Self::Executable, Self::UserData, Self::Custom];
}

/// The level drop-downs' items, in [`LogLevel::ALL`]'s order.
const LEVELS: &[&str] = &["Debug", "Info", "Warning", "Error", "Off"];

/// The dialog's controls for [`DialogFields`]; the marker chips are `plog_m0..4`.
const FIELDS: &[(&str, FieldKind)] = &[
    ("popt_title", FieldKind::Label),
    ("popt_title_input", FieldKind::Text),
    ("popt_caption_lbl", FieldKind::Label),
    ("popt_error_lbl", FieldKind::Label),
    ("plog_enabled", FieldKind::Check),
    ("plog_name", FieldKind::Text),
    ("plog_m0", FieldKind::Button),
    ("plog_m1", FieldKind::Button),
    ("plog_m2", FieldKind::Button),
    ("plog_m3", FieldKind::Button),
    ("plog_m4", FieldKind::Button),
    ("plog_today", FieldKind::Label),
    ("plog_user_dir", FieldKind::Label),
    (
        "plog_folder",
        FieldKind::Radio(&["plog_f0", "plog_f1", "plog_f2"]),
    ),
    ("plog_custom", FieldKind::Text),
    ("plog_debug", FieldKind::Drop(LEVELS)),
    ("plog_release", FieldKind::Drop(LEVELS)),
    ("plog_new", FieldKind::Drop(&["Every day", "Every run"])),
    ("plog_keep", FieldKind::Text),
    ("plog_p1.what", FieldKind::Label),
    ("plog_p2.what", FieldKind::Label),
    ("plog_p3.what", FieldKind::Label),
    ("plog_sample", FieldKind::Button),
];

impl DialogFields for ProjectOptionsState {
    fn fields(&self) -> &'static [(&'static str, FieldKind)] {
        FIELDS
    }

    fn value(&self, field: &str) -> Option<FieldValue> {
        let level = |l| LogLevel::ALL.iter().position(|x| *x == l);
        let text = |t: &str| Some(FieldValue::Text(t.to_string()));
        let log = || self.log_text_now();
        match field {
            "popt_title" => text(&self.heading),
            "popt_title_input" => text(&self.title),
            "popt_caption_lbl" => text(&self.caption),
            "popt_error_lbl" => text(self.error.as_deref().unwrap_or("")),
            "plog_enabled" => Some(FieldValue::On(self.log.enabled)),
            "plog_name" => text(&self.log.file_name),
            "plog_today" => Some(FieldValue::Text(log().today)),
            "plog_user_dir" => Some(FieldValue::Text(log().user_folder)),
            "plog_folder" => FolderChoice::ALL
                .iter()
                .position(|c| *c == self.folder_choice())
                .map(FieldValue::Index),
            "plog_custom" => text(&self.custom_folder),
            "plog_debug" => level(self.log.debug_level).map(FieldValue::Index),
            "plog_release" => level(self.log.release_level).map(FieldValue::Index),
            "plog_new" => Some(FieldValue::Index(usize::from(
                self.log.new_file == NewFile::EveryRun,
            ))),
            "plog_keep" => text(&self.keep_text),
            "plog_p1.what" => Some(FieldValue::Text(log().command_line)),
            "plog_p2.what" => Some(FieldValue::Text(log().env_variable)),
            "plog_p3.what" => Some(FieldValue::Text(log().override_file)),
            "plog_sample" => Some(FieldValue::Text(log().sample)),
            _ => None,
        }
    }

    fn input(&mut self, field: &str, value: FieldValue) {
        match (field, value) {
            ("popt_title_input", FieldValue::Text(t)) => self.set_title(t),
            ("plog_enabled", FieldValue::On(on)) => self.set_log_enabled(on),
            ("plog_name", FieldValue::Text(t)) => self.set_file_name(t),
            ("plog_custom", FieldValue::Text(t)) => self.set_custom_folder(t),
            ("plog_keep", FieldValue::Text(t)) => self.set_keep(t),
            ("plog_folder", FieldValue::Index(i)) => {
                if let Some(c) = FolderChoice::ALL.get(i) {
                    self.choose_folder(*c);
                }
            }
            ("plog_debug" | "plog_release", FieldValue::Index(i)) => {
                if let Some(l) = LogLevel::ALL.get(i) {
                    self.set_level(field == "plog_debug", *l);
                }
            }
            ("plog_new", FieldValue::Index(i)) => {
                if let Some(n) = NewFile::ALL.get(i) {
                    self.set_new_file(*n);
                }
            }
            (chip, FieldValue::On(true)) if chip.starts_with("plog_m") => {
                let n = chip["plog_m".len()..].parse::<usize>().ok();
                if let Some(m) = n.and_then(|n| crate::project_log::MARKERS.get(n)) {
                    self.insert_marker(m);
                }
            }
            _ => {}
        }
    }
}

impl Default for ProjectOptionsState {
    fn default() -> Self {
        Self::new("", false, "")
    }
}

impl ProjectOptionsState {
    /// Creates a new state with initial title and icon status.
    pub fn new(title: impl Into<String>, is_custom_icon: bool, package: &str) -> Self {
        let caption = if is_custom_icon {
            "icon.ico".to_string()
        } else {
            DEFAULT_CAPTION.to_string()
        };
        Self {
            pages: Page::ALL.to_vec(),
            current_page: Page::Application,
            title: title.into(),
            pending_icon: None,
            caption,
            error: None,
            heading: format!("Project Options \u{2014} {package}"),
            log: LogOptions::default(),
            custom_folder: String::new(),
            keep_text: LogOptions::default().keep.to_string(),
            package: package.to_string(),
        }
    }

    /// [`Self::log_text`] at this moment, as this user.
    pub fn log_text_now(&self) -> LogPageText {
        let stamp = crawrad_os::clock::local_stamp();
        let (date, hm) = stamp.split_once(' ').unwrap_or((stamp.as_str(), "00:00"));
        let user = std::env::var("USERNAME")
            .or_else(|_| std::env::var("USER"))
            .unwrap_or_default();
        self.log_text(&MarkerValues {
            app: "",
            date,
            time: &format!("{}00", hm.replace(':', "")),
            pid: std::process::id(),
            user: &user,
        })
    }

    /// The Log page's derived lines; `now` fills the date and time markers.
    pub fn log_text(&self, now: &MarkerValues<'_>) -> LogPageText {
        let app = self.package.as_str();
        let today = expand_file_name(&self.log.file_name, &MarkerValues { app, ..now.clone() });
        LogPageText {
            today: format!("Today: {today}"),
            user_folder: user_data_folder(app),
            command_line: format!("{app}.exe --log=info"),
            env_variable: format!("{}=warn", env_variable(app)),
            override_file: override_file(app),
            sample: format!("Write a sample {}", override_file(app)),
        }
    }

    /// The state opened on the project's log settings.
    pub fn with_log(mut self, log: LogOptions) -> Self {
        if let LogFolder::Custom(f) = &log.folder {
            self.custom_folder = f.clone();
        }
        self.keep_text = log.keep.to_string();
        self.log = log;
        self
    }

    /// Turns writing the log points to a file on or off.
    pub fn set_log_enabled(&mut self, on: bool) {
        self.log.enabled = on;
    }

    /// The file name as typed.
    pub fn set_file_name(&mut self, name: impl Into<String>) {
        self.log.file_name = name.into();
        self.error = None;
    }

    /// Puts a marker in the name, before its extension, `_`-separated.
    pub fn insert_marker(&mut self, marker: &str) {
        let name = &self.log.file_name;
        let at = name.rfind('.').unwrap_or(name.len());
        let sep = if at == 0 || name[..at].ends_with(['_', '-']) {
            ""
        } else {
            "_"
        };
        self.log.file_name = format!("{}{sep}{marker}{}", &name[..at], &name[at..]);
        self.error = None;
    }

    /// The folder radio picked.
    pub fn choose_folder(&mut self, choice: FolderChoice) {
        self.log.folder = match choice {
            FolderChoice::Executable => LogFolder::Executable,
            FolderChoice::UserData => LogFolder::UserData,
            FolderChoice::Custom => LogFolder::Custom(self.custom_folder.clone()),
        };
        self.error = None;
    }

    /// Which folder radio is on.
    pub fn folder_choice(&self) -> FolderChoice {
        match self.log.folder {
            LogFolder::Executable => FolderChoice::Executable,
            LogFolder::UserData => FolderChoice::UserData,
            LogFolder::Custom(_) => FolderChoice::Custom,
        }
    }

    /// The custom folder typed or browsed to; it picks the Custom radio.
    pub fn set_custom_folder(&mut self, folder: impl Into<String>) {
        self.custom_folder = folder.into();
        self.choose_folder(FolderChoice::Custom);
    }

    /// The level a debug build (`debug`) or a release build writes.
    pub fn set_level(&mut self, debug: bool, level: LogLevel) {
        if debug {
            self.log.debug_level = level;
        } else {
            self.log.release_level = level;
        }
    }

    /// When a new file starts.
    pub fn set_new_file(&mut self, new_file: NewFile) {
        self.log.new_file = new_file;
        self.error = None;
    }

    /// "Keep the last N files": a number from 1 up, or the reason it is not.
    pub fn set_keep(&mut self, text: impl Into<String>) {
        self.keep_text = text.into();
        match self.keep_text.trim().parse::<u32>() {
            Ok(n) => {
                self.log.keep = n;
                self.error = None;
            }
            Err(_) => {
                self.log.keep = 0;
                self.error = Some("Log: \"Keep the last\" takes a number of files".into());
            }
        }
    }

    /// Selects the active page.
    pub fn select_page(&mut self, page: Page) {
        self.current_page = page;
    }

    /// Updates the edited title.
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    /// Sets the icon from an external file path.
    pub fn loaded(&mut self, path: PathBuf) {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "icon".to_string());
        self.caption = name;
        self.pending_icon = Some(IconSource::File(path));
        self.error = None;
    }

    /// The outcome of reading a picked file: its icon bytes are kept as pending
    /// and handed back for the previews, or the reason is shown.
    pub fn load_result(
        &mut self,
        path: PathBuf,
        result: Result<Vec<u8>, String>,
    ) -> Option<Vec<u8>> {
        match result {
            Ok(ico) => {
                self.loaded(path);
                Some(ico)
            }
            Err(e) => {
                self.set_error(e);
                None
            }
        }
    }

    /// Restores the built-in default crab icon.
    pub fn default_icon(&mut self) {
        self.caption = DEFAULT_CAPTION.to_string();
        self.pending_icon = Some(IconSource::Default);
        self.error = None;
    }

    /// Cancels dialog editing, clearing pending icon and errors.
    pub fn cancel(&mut self) {
        self.pending_icon = None;
        self.error = None;
    }

    /// Finalizes OK action, returning the applied title and pending icon choice.
    pub fn ok(&mut self) -> (String, Option<IconSource>) {
        (self.title.clone(), self.pending_icon.take())
    }

    /// Sets an error message to display in the dialog.
    pub fn set_error(&mut self, err: impl Into<String>) {
        self.error = Some(err.into());
    }

    /// Clears any displayed error message.
    pub fn clear_error(&mut self) {
        self.error = None;
    }
}

#[cfg(test)]
#[path = "test/project_options_dialog/test.rs"]
mod project_options_dialog_test;
