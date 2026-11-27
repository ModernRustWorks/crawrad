//! The **Options** dialog model: the tree on the left and its names.
//!
//! # Why it is a separate file
//!
//! The same reason as the other models: the Makepad shell is another `[[bin]]`. Here
//! what must not diverge is the TREE — groups and destinations —, because
//! it is the map of what the IDE allows configuring. Two trees would result in two IDEs
//! with different options, and the second would only be discovered by someone looking for an
//! option that exists on the other side.
//!
//! # What is NOT here
//!
//! The VALUES. Which checkboxes are checked is shell state — the designer `snap`,
//! the generation target of preferences —, and each keeps them where they were already
//! kept. This file states **what exists to be configured**, not what is configured.

use crawrad_note::note;
#[note("options-opt-grade-min")]
pub const OPT_GRADE_MIN: f32 = 2.0;
pub const OPT_GRADE_MAX: f32 = 64.0;

#[note("options-opt-ver-grade")]
pub const OPT_VER_GRADE: &str = "Display grid";
pub const OPT_SNAP: &str = "Snap to grid";
pub const OPT_GRADE_NOTA: &str = "Minimum: 2 (high precision).";
pub const OPT_ZOOM_NOTA: &str = "Not available in the Makepad shell: the framework has no live lever for a \
     window-wide scale (pass.rs:114). Ctrl + and Ctrl - are unwired for the same reason.";
pub const OPT_ICONES: &str = "Icon sheet...";
pub const OPT_TEMA_NOTA: &str = "One theme for now. The form you draw keeps the colours of the target platform, \
     whatever the IDE wears.";

#[note("options-font-sizes")]
pub const FONT_SIZES: &[u32] = &[9, 10, 11, 12, 14, 16, 18, 20];

#[note("options-tab-sizes")]
pub const TAB_SIZES: &[u8] = &[2, 4, 8];

#[note("options-category")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Category {
    #[default]
    CommonlyUsed,
    IdeDefaultFolders,
    IdeEnvironmentVariables,
    IdeFileHistory,
    IdeSavingRecovering,
    UiFormDesigner,
    UiAppearance,
    Editor,
    EditorColorScheme,
    Keyboard,
    AiAcp,
    AiRegistry,
}

#[note("options-page-of-menu-row")]
pub fn page_of_menu_row(label: &str) -> Option<Category> {
    Some(match label {
        "Keyboard Shortcuts" => Category::Keyboard,
        "Themes" => Category::UiAppearance,
        "Configure Editors" => Category::Editor,
        _ => return None,
    })
}

impl Category {
    #[note("options-category-pages")]
    pub const PAGES: [&'static str; 10] = [
        "opt_commonly_used",
        "opt_folders",
        "opt_history",
        "opt_saving",
        "opt_designer",
        "opt_appearance",
        "opt_editor",
        "opt_color_scheme",
        "opt_keyboard",
        "opt_ai",
    ];

    #[note("options-category-page")]
    pub fn page(self) -> Option<&'static str> {
        let at = |i: usize| Some(Self::PAGES[i]);
        match self {
            Self::CommonlyUsed => at(0),
            Self::IdeDefaultFolders => at(1),
            Self::IdeEnvironmentVariables => None,
            Self::IdeFileHistory => at(2),
            Self::IdeSavingRecovering => at(3),
            Self::UiFormDesigner => at(4),
            Self::UiAppearance => at(5),
            Self::Editor => at(6),
            Self::EditorColorScheme => at(7),
            Self::Keyboard => at(8),
            Self::AiAcp | Self::AiRegistry => at(9),
        }
    }

    #[note("options-category-note")]
    pub fn note(self) -> Option<&'static str> {
        match self {
            Self::IdeEnvironmentVariables => {
                Some("System environment variables are inherited from the parent process.")
            }
            Self::CommonlyUsed
            | Self::IdeDefaultFolders
            | Self::IdeFileHistory
            | Self::IdeSavingRecovering
            | Self::Editor
            | Self::EditorColorScheme
            | Self::Keyboard
            | Self::UiFormDesigner
            | Self::UiAppearance
            | Self::AiAcp
            | Self::AiRegistry => None,
        }
    }

    #[note("options-category-label")]
    pub fn label(self) -> &'static str {
        match self {
            Self::CommonlyUsed => "Commonly Used",
            Self::IdeDefaultFolders => "Default Folders",
            Self::IdeEnvironmentVariables => "Environment Variables",
            Self::IdeFileHistory => "File History & Backups",
            Self::IdeSavingRecovering => "Saving and Recovering",
            Self::UiFormDesigner => "Form Designer",
            Self::UiAppearance => "Appearance",
            Self::Editor => "General",
            Self::EditorColorScheme => "Color Scheme",
            Self::Keyboard => "Key Mappings",
            Self::AiAcp => "Agent Client Protocol (ACP)",
            Self::AiRegistry => "ACP Registry (Portal)",
        }
    }

    #[note("options-category-title")]
    pub fn title(self) -> &'static str {
        match self {
            Self::CommonlyUsed => "Commonly Used",
            Self::IdeDefaultFolders => "Default Folders",
            Self::IdeEnvironmentVariables => "Environment Variables",
            Self::IdeFileHistory => "IDE / File History & Backups",
            Self::IdeSavingRecovering => "IDE / Saving and Recovering",
            Self::UiFormDesigner => "Form Designer",
            Self::UiAppearance => "Appearance",
            Self::Editor => "Editor",
            Self::EditorColorScheme => "Editor / Color Scheme",
            Self::Keyboard => "Keyboard",
            Self::AiAcp => "AI Assistant / Agent Client Protocol (ACP)",
            Self::AiRegistry => "ACP Registry / Portal (agentclientprotocol.com)",
        }
    }
}

#[note("options-group")]
pub struct Group {
    pub name: &'static str,
    pub items: &'static [Category],
    pub open_by_default: bool,
}

#[note("options-tree")]
pub const TREE: &[Group] = &[
    Group {
        name: "Preferences",
        items: &[Category::CommonlyUsed],
        open_by_default: true,
    },
    Group {
        name: "IDE",
        items: &[
            Category::IdeDefaultFolders,
            Category::IdeEnvironmentVariables,
            Category::IdeFileHistory,
            Category::IdeSavingRecovering,
        ],
        open_by_default: true,
    },
    Group {
        name: "User Interface",
        items: &[Category::UiFormDesigner, Category::UiAppearance],
        open_by_default: true,
    },
    Group {
        name: "AI Assistant",
        items: &[Category::AiAcp],
        open_by_default: true,
    },
    Group {
        name: "Editor",
        items: &[Category::Editor, Category::EditorColorScheme],
        open_by_default: true,
    },
    Group {
        name: "Keyboard",
        items: &[Category::Keyboard],
        open_by_default: false,
    },
];

#[note("options-numberchoice")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberChoice {
    /// File backup limit: copies kept per file.
    Limit,
    /// Saves closer than this many seconds merge into one copy (0 = off).
    MergeSeconds,
    /// Files larger than this many KB get no copy.
    MaxFileKb,
    /// Minutes between recovery copies.
    RecoverMinutes,
    /// Maximum recent projects retained.
    RecentProjects,
}

impl NumberChoice {
    #[note("options-numberchoice-all")]
    pub const ALL: [Self; 5] = [
        Self::Limit,
        Self::MergeSeconds,
        Self::MaxFileKb,
        Self::RecoverMinutes,
        Self::RecentProjects,
    ];

    #[note("options-numberchoice-widget")]
    pub fn widget(self) -> &'static str {
        match self {
            Self::Limit => "opt_history_limit",
            Self::MergeSeconds => "opt_history_merge",
            Self::MaxFileKb => "opt_history_max",
            Self::RecoverMinutes => "opt_recover_minutes",
            Self::RecentProjects => "opt_recent_limit",
        }
    }

    #[note("options-numberchoice-is-history")]
    pub fn is_history(self) -> bool {
        match self {
            Self::Limit | Self::MergeSeconds | Self::MaxFileKb => true,
            Self::RecoverMinutes | Self::RecentProjects => false,
        }
    }

    #[note("options-numberchoice-values")]
    pub fn values(self) -> &'static [u32] {
        match self {
            Self::Limit => &[1, 2, 3, 5, 10, 15, 20, 30, 50, 100],
            Self::MergeSeconds => &[0, 5, 10, 30, 60],
            Self::MaxFileKb => &[256, 1024, 5120, 10240],
            Self::RecoverMinutes => &crate::recover::MINUTES,
            Self::RecentProjects => &[3, 5, 10, 15, 20, 30, 50],
        }
    }

    #[note("options-numberchoice-labels")]
    pub fn labels(self) -> Vec<String> {
        let label = |v: u32| match self {
            Self::Limit => format!("{v} copies"),
            Self::MergeSeconds if v == 0 => "Off".to_string(),
            Self::MergeSeconds => format!("{v} s"),
            Self::MaxFileKb if v.is_multiple_of(1024) => format!("{} MB", v / 1024),
            Self::MaxFileKb => format!("{v} KB"),
            Self::RecoverMinutes => format!("{v} min"),
            Self::RecentProjects => format!("{v} projects"),
        };
        self.values().iter().map(|&v| label(v)).collect()
    }

    #[note("options-numberchoice-position")]
    pub fn position(self, p: &crate::workspace::Preferences) -> Option<usize> {
        let current = match self {
            Self::Limit => p.history_limit,
            Self::MergeSeconds => p.history_merge_seconds,
            Self::MaxFileKb => p.history_max_file_kb,
            Self::RecoverMinutes => p.recover_minutes,
            Self::RecentProjects => p.recent_limit,
        };
        self.values().iter().position(|&v| v == current)
    }

    #[note("options-numberchoice-pick")]
    pub fn pick(self, p: &mut crate::workspace::Preferences, pos: usize) {
        let Some(&v) = self.values().get(pos) else {
            return;
        };
        match self {
            Self::Limit => p.history_limit = v,
            Self::MergeSeconds => p.history_merge_seconds = v,
            Self::MaxFileKb => p.history_max_file_kb = v,
            Self::RecoverMinutes => p.recover_minutes = v,
            Self::RecentProjects => p.recent_limit = v,
        }
    }
}

#[note("options-editorcheck")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorCheck {
    LineNumbers,
    InlayHints,
    CurrentLine,
    BreadcrumbChevron,
}

impl EditorCheck {
    #[note("options-editorcheck-all")]
    pub const ALL: [Self; 4] = [
        Self::LineNumbers,
        Self::InlayHints,
        Self::CurrentLine,
        Self::BreadcrumbChevron,
    ];

    #[note("options-editorcheck-label")]
    pub fn label(self) -> &'static str {
        match self {
            Self::LineNumbers => "Show line numbers in editor margin",
            Self::InlayHints => "Show inferred types in grey inside the lines (inlay hints)",
            Self::CurrentLine => "Highlight the line the caret is on",
            Self::BreadcrumbChevron => "Breadcrumb in Chevron style (off: plain text)",
        }
    }

    #[note("options-editorcheck-widgets")]
    pub fn widgets(self) -> &'static [&'static str] {
        match self {
            Self::LineNumbers => &[
                "opt_editor_line_numbers",
                "opt_cu_line_numbers",
                "opt_sr_line_numbers",
            ],
            Self::InlayHints => &["opt_editor_inlay_hints"],
            Self::CurrentLine => &["opt_editor_current_line"],
            Self::BreadcrumbChevron => &["opt_editor_breadcrumb_chevron"],
        }
    }

    #[note("options-editorcheck-get")]
    pub fn get(self, p: &crate::workspace::Preferences) -> bool {
        match self {
            Self::LineNumbers => p.editor_line_numbers,
            Self::InlayHints => p.editor_inlay_hints,
            Self::CurrentLine => p.editor_current_line,
            Self::BreadcrumbChevron => p.editor_breadcrumb_chevron,
        }
    }

    #[note("options-editorcheck-set")]
    pub fn set(self, p: &mut crate::workspace::Preferences, on: bool) {
        match self {
            Self::LineNumbers => p.editor_line_numbers = on,
            Self::InlayHints => p.editor_inlay_hints = on,
            Self::CurrentLine => p.editor_current_line = on,
            Self::BreadcrumbChevron => p.editor_breadcrumb_chevron = on,
        }
    }
}

#[note("options-savingcheck")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavingCheck {
    /// Open the most recent project on start instead of the welcome.
    ReopenLastProject,
    /// Save every open editor file before Build or Run.
    SaveBeforeBuild,
    /// Keep the project desktop (docks, window, tabs and carets) on close.
    SaveDesktop,
    /// Keep recovery copies of the unsaved editor files.
    RecoverCopies,
}

impl SavingCheck {
    #[note("options-savingcheck-all")]
    pub const ALL: [Self; 4] = [
        Self::ReopenLastProject,
        Self::SaveBeforeBuild,
        Self::SaveDesktop,
        Self::RecoverCopies,
    ];

    #[note("options-savingcheck-label")]
    pub fn label(self) -> &'static str {
        match self {
            Self::ReopenLastProject => "Reopen the last project on start",
            Self::SaveBeforeBuild => "Save editor files before Build or Run",
            Self::SaveDesktop => "Save the project desktop on close",
            Self::RecoverCopies => "Keep recovery copies of unsaved files",
        }
    }

    #[note("options-savingcheck-widgets")]
    pub fn widgets(self) -> &'static [&'static str] {
        match self {
            Self::ReopenLastProject => &["opt_reopen_last"],
            Self::SaveBeforeBuild => &["opt_save_build", "opt_cu_save_build", "opt_sr_save_build"],
            Self::SaveDesktop => &["opt_save_desktop"],
            Self::RecoverCopies => &["opt_recover"],
        }
    }

    #[note("options-savingcheck-get")]
    pub fn get(self, p: &crate::workspace::Preferences) -> bool {
        match self {
            // Stored as its opposite since before this page: the welcome.
            Self::ReopenLastProject => !p.welcome_on_startup,
            Self::SaveBeforeBuild => p.save_before_build,
            Self::SaveDesktop => p.save_desktop,
            Self::RecoverCopies => p.recover_copies,
        }
    }

    #[note("options-savingcheck-set")]
    pub fn set(self, p: &mut crate::workspace::Preferences, on: bool) {
        match self {
            Self::ReopenLastProject => p.welcome_on_startup = !on,
            Self::SaveBeforeBuild => p.save_before_build = on,
            Self::SaveDesktop => p.save_desktop = on,
            Self::RecoverCopies => p.recover_copies = on,
        }
    }
}

#[note("options-optionrow")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionRow {
    Group { name: &'static str, open: bool },
    Item(Category),
}

#[note("options-option-rows")]
pub fn option_rows(is_closed: &dyn Fn(&str) -> bool) -> Vec<OptionRow> {
    let mut out = Vec::new();
    for g in TREE {
        let open = !is_closed(g.name);
        out.push(OptionRow::Group { name: g.name, open });
        if open {
            out.extend(g.items.iter().copied().map(OptionRow::Item));
        }
    }
    out
}

#[note("options-closed-on-open")]
pub fn closed_on_open() -> Vec<String> {
    TREE.iter()
        .filter(|g| !g.open_by_default)
        .map(|g| g.name.to_string())
        .collect()
}

#[note("options-optionssearchmatches")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OptionsSearchMatches {
    pub font: bool,
    pub line: bool,
    pub save: bool,
    pub grid: bool,
    pub ai: bool,
    pub key: bool,
}

impl OptionsSearchMatches {
    pub fn from_query(query: &str) -> Self {
        let q = query.trim().to_lowercase();
        let font =
            q.contains("font") || q.contains("size") || q.contains("pixel") || q.contains("editor");
        let line = q.contains("line")
            || q.contains("number")
            || q.contains("margin")
            || q.contains("editor");
        let save =
            q.contains("save") || q.contains("auto") || q.contains("file") || q.contains("recent");
        let grid = q.contains("grid")
            || q.contains("snap")
            || q.contains("grade")
            || q.contains("designer");
        let ai = q.contains("ai")
            || q.contains("assistant")
            || q.contains("acp")
            || q.contains("agent")
            || q.contains("gemini")
            || q.contains("claude")
            || q.contains("model");
        let scheme_id = crawrad_actions::KeymapScheme::ALL[1].id();
        let key = q.contains("key")
            || q.contains("shortcut")
            || q.contains("scheme")
            || (!scheme_id.is_empty() && q.contains(scheme_id))
            || q.contains("modern")
            || q.contains("teclado");
        Self {
            font,
            line,
            save,
            grid,
            ai,
            key,
        }
    }

    pub fn has_any(&self) -> bool {
        self.font || self.line || self.save || self.grid || self.ai || self.key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saving_and_recovering_sits_under_ide() {
        let ide = TREE.iter().find(|g| g.name == "IDE").expect("IDE group");
        assert!(ide.items.contains(&Category::IdeSavingRecovering));
        assert_eq!(
            Category::IdeSavingRecovering.title(),
            "IDE / Saving and Recovering"
        );
    }

    /// Each box writes the preference it reads, and the reopen box is the
    /// welcome stored upside down.
    #[test]
    fn a_saving_check_reads_what_it_writes() {
        for check in SavingCheck::ALL {
            for on in [false, true] {
                let mut p = crate::workspace::Preferences::default();
                check.set(&mut p, on);
                assert_eq!(check.get(&p), on, "{check:?}");
            }
            assert!(!check.widgets().is_empty());
        }
        let mut p = crate::workspace::Preferences::default();
        SavingCheck::ReopenLastProject.set(&mut p, false);
        assert!(p.welcome_on_startup);
    }

    /// The editor boxes too, and line numbers sits in three places.
    #[test]
    fn an_editor_check_reads_what_it_writes() {
        for check in EditorCheck::ALL {
            for on in [false, true] {
                let mut p = crate::workspace::Preferences::default();
                check.set(&mut p, on);
                assert_eq!(check.get(&p), on, "{check:?}");
            }
        }
        assert_eq!(EditorCheck::LineNumbers.widgets().len(), 3);
    }

    /// Only the page that informs has no page of controls, and every page is
    /// some entry's — the AI one, two entries'.
    #[test]
    fn every_entry_shows_one_of_the_pages() {
        let all = TREE.iter().flat_map(|g| g.items.iter().copied());
        let shown: Vec<_> = all.filter_map(|c| c.page().map(|p| (c, p))).collect();
        for (c, p) in &shown {
            assert!(Category::PAGES.contains(p), "{c:?}");
        }
        for page in Category::PAGES {
            assert!(shown.iter().any(|(_, p)| *p == page), "{page} is nobody's");
        }
        assert_eq!(Category::IdeEnvironmentVariables.page(), None);
        assert_eq!(Category::AiRegistry.page(), Category::AiAcp.page());
    }

    #[test]
    fn the_recovery_interval_is_offered_in_minutes() {
        let mut p = crate::workspace::Preferences::default();
        let every = NumberChoice::RecoverMinutes;
        assert_eq!(every.labels(), ["1 min", "5 min", "10 min", "30 min"]);
        assert_eq!(every.position(&p), Some(2), "10 minutes by default");
        every.pick(&mut p, 0);
        assert_eq!(p.recover_minutes, 1);
        assert!(!every.is_history() && NumberChoice::Limit.is_history());
    }

    #[test]
    fn the_recent_projects_limit_is_offered() {
        let mut p = crate::workspace::Preferences::default();
        let choice = NumberChoice::RecentProjects;
        assert_eq!(choice.widget(), "opt_recent_limit");
        assert_eq!(
            choice.labels(),
            [
                "3 projects",
                "5 projects",
                "10 projects",
                "15 projects",
                "20 projects",
                "30 projects",
                "50 projects"
            ]
        );
        assert_eq!(choice.position(&p), Some(2), "10 projects by default");
        choice.pick(&mut p, 1);
        assert_eq!(p.recent_limit, 5);
        assert!(!choice.is_history());
    }

    #[test]
    fn the_color_preset_drop_follows_the_scheme_order() {
        use crate::workspace::EditorColorScheme as S;
        assert_eq!(S::preset_labels(S::ModernDark).len(), S::ALL.len());
        for (i, s) in S::ALL.iter().enumerate() {
            assert_eq!(s.position(), i);
        }
    }

    #[test]
    fn ai_assistant_group_has_acp() {
        let ai_group = TREE
            .iter()
            .find(|g| g.name == "AI Assistant")
            .expect("AI Assistant group must exist in options tree");
        assert!(ai_group.items.contains(&Category::AiAcp));
    }

    #[test]
    fn ai_registry_labels_and_titles_are_present() {
        assert_eq!(Category::AiRegistry.label(), "ACP Registry (Portal)");
        assert!(
            Category::AiRegistry
                .title()
                .contains("agentclientprotocol.com")
        );
    }

    #[test]
    fn option_rows_flattening_includes_ai_acp_when_open() {
        let rows = option_rows(&|_| false);
        assert!(rows.contains(&OptionRow::Item(Category::AiAcp)));
    }

    #[test]
    fn editor_group_has_color_scheme() {
        let ed_group = TREE
            .iter()
            .find(|g| g.name == "Editor")
            .expect("Editor group must exist in options tree");
        assert!(ed_group.items.contains(&Category::Editor));
        assert!(ed_group.items.contains(&Category::EditorColorScheme));
        assert_eq!(Category::EditorColorScheme.label(), "Color Scheme");
        assert_eq!(Category::EditorColorScheme.title(), "Editor / Color Scheme");
    }

    #[test]
    fn ide_group_has_file_history() {
        let ide_group = TREE
            .iter()
            .find(|g| g.name == "IDE")
            .expect("IDE group must exist in options tree");
        assert!(ide_group.items.contains(&Category::IdeFileHistory));
        assert_eq!(Category::IdeFileHistory.label(), "File History & Backups");
        assert_eq!(
            Category::IdeFileHistory.title(),
            "IDE / File History & Backups"
        );
        assert_eq!(Category::IdeFileHistory.note(), None);
    }

    #[test]
    fn history_choices_label_pick_and_find_their_values() {
        let mut prefs = crate::workspace::Preferences::default();
        assert_eq!(NumberChoice::Limit.position(&prefs), Some(4));
        assert_eq!(NumberChoice::MergeSeconds.labels()[0], "Off");
        assert_eq!(
            NumberChoice::MaxFileKb.labels(),
            ["256 KB", "1 MB", "5 MB", "10 MB"]
        );
        NumberChoice::Limit.pick(&mut prefs, 1);
        NumberChoice::MergeSeconds.pick(&mut prefs, 0);
        NumberChoice::MaxFileKb.pick(&mut prefs, 99);
        assert_eq!(
            (
                prefs.history_limit,
                prefs.history_merge_seconds,
                prefs.history_max_file_kb
            ),
            (2, 0, 5120)
        );
        prefs.history_limit = 7;
        assert_eq!(NumberChoice::Limit.position(&prefs), None);
    }

    #[test]
    fn presets_fill_the_custom_colors_and_custom_keeps_them() {
        use crate::workspace::{EditorColorScheme, EditorCustomColors};
        assert_eq!(
            EditorColorScheme::ModernDark.preset(),
            Some(EditorCustomColors::default())
        );
        let monokai = EditorColorScheme::Monokai
            .preset()
            .map(|c| (c.keyword, c.bg));
        assert_eq!(monokai, Some(("#f92672".into(), "#272822".into())));
        assert_eq!(EditorColorScheme::Custom.preset(), None);
    }

    #[test]
    fn only_the_environment_page_carries_a_note() {
        let noted: Vec<Category> = TREE
            .iter()
            .flat_map(|g| g.items.iter().copied())
            .filter(|c| c.note().is_some())
            .collect();
        assert_eq!(noted, vec![Category::IdeEnvironmentVariables]);
    }
}
