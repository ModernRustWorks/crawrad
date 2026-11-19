//! CrawRAD Studio IDE in Native Makepad (`makepad-widgets` & `makepad-code-editor`).
//!
//! Native GPU-accelerated IDE shell featuring:
//! - Titlebar & Top Menu Bar (File, Edit, View, Project, Run, Help, Undo, Redo, Save, Run).
//! - Centered Project Title and Top Status Badges (Preview, + AI, saved).
//! - Leftmost VS Code-style Activity Rail (Files, Search, Source Control, Debug, Settings).
//! - Left Dock: Collapsible Toolbox (with Filter and Categories: Additional, Standard, System),
//!   Structure Object Tree (Form Hierarchy) and the Object Inspector.
//! - Center Area: the `DesignCanvas` of the open form (which draws its own rulers) and
//!   bottom Design / Code tabs.
//! - Right Dock: Aefos AI panel — a **mockup**, marked as such on its own header;
//!   it calls no model and the chat backend arrives in Phase 4.
//! - Bottom Status Bar: Open file path, node count, selection status, revision indicator, and GUI framework.
//!
//! # Presentation shell
//!
//! Connects `.crawform` on disk to `DesignCanvas` and `crawrad-designer` (ADR 0027).
//! Dispatches UI events, translates gestures, and hosts presentation components.
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

use crate::ide_app::mcp_host::{McpRequest, McpServer};
use crawrad_actions::capability::CapabilityRegistry;
use crawrad_actions::capability::PaletteItem as QuickOpenCommandItem;
use crawrad_extension_host::ShellExtensionHost;
use crawrad_insight::lsp_cell::LspCell;
use makepad_code_editor::*;
use crawrad_actions::KeymapScheme;
use crawrad_designer::clipboard::Clipboard;
use crawrad_designer::{Designer, TabAction};
use crawrad_doc::Doc;
use crawrad_host::HostModel;
use crawrad_host::disk_watch::DiskWatch;
use crawrad_ir::{NodeId, Value};
use makepad_widgets::splitter::SplitterAlign;
use makepad_widgets::*;
// The trait must be in scope: it provides `open`, `set_selection`,
// `set_marquee`, `set_guides`, `set_grid`, and `drain_events` for the canvas.
use crawrad_runtime::{Runtime, RuntimeEvent, Scale, ViewportPx};
use crawrad_runtime_makepad::CrawradColumnSplitterWidgetRefExt;
use crawrad_runtime_makepad::{CrawradSegmentListWidgetRefExt, DesignCanvas};

/// Shell theme DSL bindings. Explicit `#[path]` ensures resolution across binaries.
#[path = "ide_app/theme.rs"]
pub(crate) mod theme;
pub(crate) use theme as tema;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::crawrad_theme::*;
    use link::crawrad_dsl_main::*;

    IdeApp = {{IdeApp}} {
        ui: <CrawradMainUi> {}
        measure: { text_style: <CRAWRAD_FONT_CODE_ICONS> { font_size: (CRAWRAD_FONTE_PEQUENO) } }
    }
}

use crate::support::i18n::Language;
// SINGLE-SEGMENT ALIAS, due to the `#[derive(Live)]` pitfall noted in
// `CacheDeOpcoes`: it does not read multi-segment paths as field types.
use crate::view_models::editor_tabs::EditorTabs;
// FOR THE SAME REASON as `EditorTabs` above: `#[derive(Live)]` does not read
// multi-segment paths as field types.
use crate::support::encoding::Encoding;

/// Where the form comes from, and what the status bar says about it.
///
/// The `#[path]` is for the same reason as `theme` above: this file is loaded
/// by two binaries, and a module opened via `#[path]` does not open a
/// directory scope.
pub(crate) use crawrad_host::document;
pub(crate) use document as documento;

/// The Toolbox rows, derived from the `crawrad-registry` catalog — pure, in crawrad-designer.
pub(crate) use crawrad_designer::palette;

/// The keyboard bridge: from `KeyCode` to catalog action id, and to the designer's
/// arrow keys. The `#[path]` is for the same reason as `documento` above.
#[path = "ide_app/keyboard.rs"]
pub(crate) mod keyboard;
pub(crate) use keyboard as teclado;

/// The Object Inspector's rows and property editors (`crawrad_actions::inspector_model`).
use crawrad_actions::inspector_model::{self as inspector, NameSplitter};

use crawrad_project::notes_panel::NotesPanel;
/// The activity rail targets — the list that the DSL does **not** have (`crawrad_project::rail`).
use crawrad_project::rail;

/// The Explorer: who owns each file (`crawrad_project::explorer`).
use crawrad_project::explorer;

/// The dock's PACKAGE MANAGER: the panel rows — pure, in crawrad-project.
use crawrad_project::package_rows as packages;

/// THE DOCK: which panel and which tab, and how the layout survives window
/// close. Introduced with Amendment 9 of ADR 0014 — the region ceased to have an owner. The
/// `#[path]` is for the same reason as `documento` above.
#[path = "ide_app/layout.rs"]
mod layout;

/// What the CENTRE shows when it is not the drawing (`crawrad_project::source`).
#[path = "ide_app/source.rs"]
pub mod source;
pub use source::Center;

/// The rows of the six top menus, derived from the action catalog. The
/// `#[path]` is for the same reason as `documento` above.
#[path = "ide_app/menu.rs"]
mod menu;

/// The bar's strip of commands — Undo, Redo, Save and Run (`crawrad_project::toolbar`).
use crawrad_project::toolbar;

/// The icons: each chrome role's Codicons glyph, each catalog class's drawing
/// (`crawrad_project::icons`).
pub use crawrad_project::icons as icon;

#[path = "ide_app/arrangement.rs"]
pub(crate) mod arrangement;

#[path = "ide_app/components.rs"]
pub(crate) mod components;

#[path = "ide_app/chat.rs"]
pub(crate) mod chat;
#[path = "ide_app/chat_attachments.rs"]
mod chat_attachments;
#[path = "ide_app/chat_state.rs"]
pub(crate) mod chat_state;
#[path = "ide_app/chat_teacher.rs"]
mod chat_teacher;
pub use chat_state::ChatState;

#[path = "ide_app/auto_save.rs"]
mod auto_save;
#[path = "ide_app/build.rs"]
pub(crate) mod build;
#[path = "ide_app/chat_bar.rs"]
mod chat_bar;
#[path = "ide_app/constants.rs"]
mod constants;
#[path = "ide_app/desktop.rs"]
mod desktop;
#[path = "ide_app/diff_review.rs"]
mod diff_review;
#[path = "ide_app/disk_watch.rs"]
mod disk_watch;
#[path = "ide_app/document_flow.rs"]
mod document_flow;
#[path = "ide_app/effects.rs"]
mod effects;
#[path = "ide_app/events.rs"]
mod events;
#[path = "ide_app/explorer_actions.rs"]
mod explorer_actions;
#[path = "ide_app/history.rs"]
mod history;
#[path = "ide_app/host_api.rs"]
mod host_api;
#[path = "ide_app/key_dispatch.rs"]
mod key_dispatch;
#[path = "ide_app/live.rs"]
mod live;
#[path = "ide_app/recover.rs"]
mod recover;
#[path = "ide_app/sync.rs"]
mod sync;
#[path = "ide_app/view_state.rs"]
mod view_state;
#[path = "ide_app/welcome.rs"]
mod welcome;
use constants::*;
pub use crawrad_project::build_settings::BuildState;

#[path = "ide_app/actions.rs"]
pub(crate) mod actions;

#[path = "ide_app/chrome.rs"]
pub(crate) mod chrome;

#[path = "ide_app/class_icon_strip.rs"]
pub(crate) mod class_icon_strip;
pub(crate) use class_icon_strip::ClassIconStrip;

#[path = "ide_app/dialogs.rs"]
pub(crate) mod dialogs;
pub(crate) use dialogs as dialogos;
pub(crate) use dialogs::AfterSaving;
#[path = "ide_app/component_rename.rs"]
mod component_rename;
#[path = "ide_app/debug.rs"]
mod debug;
#[path = "ide_app/draw.rs"]
pub(crate) mod draw;
#[path = "ide_app/dsl_main.rs"]
pub(crate) mod dsl_main;
#[path = "ide_app/editor/breakpoints.rs"]
mod editor_breakpoints;
#[path = "ide_app/editor/buffer.rs"]
mod editor_buffer;
#[path = "ide_app/editor/change_bars.rs"]
mod editor_change_bars;
#[path = "ide_app/editor/code_actions.rs"]
mod editor_code_actions;
#[path = "ide_app/editor/find.rs"]
pub(crate) mod editor_find;
#[path = "ide_app/editor/footer.rs"]
mod editor_footer;
#[path = "ide_app/editor/item_doc.rs"]
mod editor_item_doc;
#[path = "ide_app/editor/keys.rs"]
mod editor_keys;
#[path = "ide_app/editor/line_marks.rs"]
mod editor_line_marks;
#[path = "ide_app/editor/semantic.rs"]
mod editor_semantic;
#[path = "ide_app/editor/signature.rs"]
mod editor_signature;
#[path = "ide_app/extension_views.rs"]
mod extension_views;
#[path = "ide_app/form_check.rs"]
mod form_check;
#[path = "ide_app/notes_panel.rs"]
mod notes_panel;
#[path = "ide_app/problems.rs"]
mod problems;
#[path = "ide_app/references.rs"]
mod references;
#[path = "ide_app/rename.rs"]
mod rename;
#[path = "ide_app/size_preview.rs"]
mod size_preview;
#[path = "ide_app/tests_panel.rs"]
mod tests_panel;
#[path = "ide_app/translations.rs"]
mod translations;
use editor_find::EditorFind;
#[path = "ide_app/editor/complete.rs"]
pub(crate) mod editor_complete;
use editor_complete::EditorComplete;

#[path = "ide_app/link.rs"]
pub(crate) mod link;

#[path = "ide_app/lsp_service.rs"]
pub(crate) mod lsp_service;

#[path = "ide_app/maximize.rs"]
mod maximize;
#[path = "ide_app/mcp_host.rs"]
pub mod mcp_host;

#[path = "ide_app/packages_app.rs"]
pub(crate) mod packages_app;
use makepad_code_editor::minimap::CrawradMinimap;

#[path = "ide_app/panels.rs"]
pub(crate) mod panels;
pub(crate) use panels as paineis;

#[path = "ide_app/property.rs"]
pub(crate) mod property;

#[path = "ide_app/status_bar.rs"]
mod status_bar;
#[path = "ide_app/ui_zoom.rs"]
mod ui_zoom;

use explorer::Folds;
use crate::options;
// SINGLE-SEGMENT alias, due to the `#[derive(Live)]` pitfall noted in
// `CacheDeOpcoes`: it does not read multi-segment paths as field types.
use crate::action_list;
use crate::image_list_editor;
use crate::image_list_editor::model::ImageListEditorState;
use crate::items_editor;
use crate::items_editor::model::ItemsEditorState;
use crate::lines_editor;
use crate::lines_editor::model::LinesEditorState;
use crate::options::Category as OptionsCategory;
use crawrad_actions::Panel;
use crawrad_agent::AcpConfig;
use crawrad_terminal::{ShellType, TerminalModel};
use inspector::Tab as InspectorTab;
use dialogos::build::BuildMode;
use packages::model::PackageManagerState as PackageStore;
#[note("ide-app-audiosessioninstance")]
pub type AudioSessionInstance = crawrad_audio::AudioSession<crawrad_audio::CpalAudioRecorder>;
pub type ShrinkDialog = crawrad_project::release_size::ShrinkDialog;
/// (code file, desktop root) and what they measure: (member, group root).
type CodePlace = (
    Option<PathBuf>,
    Option<PathBuf>,
    Option<PathBuf>,
    Option<PathBuf>,
);
type CodePlaceCell = std::cell::RefCell<CodePlace>;
type ExplorerTarget = Option<(std::path::PathBuf, bool)>;
type ExplorerRects = Vec<(usize, Rect)>;
type ExplorerDrawn = serde_json::Value;
type ExplorerRowTargets = Vec<Option<(std::path::PathBuf, bool)>>;
type UnsavedTabFiles = std::collections::BTreeSet<PathBuf>;
type SizePreview = crawrad_designer::size_preview::SizePreview;

#[note("ide-app-cachedeopcoes")]
type CacheDeOpcoes = std::collections::HashMap<usize, Vec<String>>;

#[note("ide-app-gruposabertos")]
type GruposAbertos = std::collections::BTreeSet<String>;

#[note("ide-app-edicao")]
struct Edicao {
    /// Node being edited when field received focus.
    no: NodeId,
    /// Property key from catalog.
    key: &'static str,
    /// Row index in the portal list.
    linha: usize,
    /// Line widget slot.
    slot: inspector::Slot,
    /// Current text in the input.
    texto: String,
}

#[note("ide-app-eventedit")]
struct EventEdit {
    /// Component owning the event.
    no: NodeId,
    /// Event name from catalog.
    evento: &'static str,
    /// Current input text. Empty unbinds the event.
    texto: String,
}

#[note("ide-app-padaberto")]
struct PadAberto {
    /// Component owning the property.
    no: NodeId,
    /// Catalog property key.
    key: String,
}

pub(crate) use crawrad_host::navigation::PousoCodigo;

#[note("ide-app-aiturnresult")]
pub type AiTurnResult = crawrad_agent::acp_live::TurnResult;
#[note("ide-app-aisessionperms")]
pub type AiSessionPerms = std::sync::Arc<std::sync::Mutex<crawrad_agent::SessionPermissions>>;
#[note("ide-app-aipermqueue")]
pub type AiPermQueue = crawrad_agent::PermissionQueue;
#[note("ide-app-creationtasks")]
// The Live derive cannot read generic arguments such as `<()>` on a field.
type CreationTasks = crawrad_os::tasks::TaskManager<()>;
type BuildTasks = crawrad_os::tasks::TaskManager<crawrad_os::process::Outcome>;
type WebRun = crawrad_project::web_run::WebRun;
type AppRun = crawrad_project::app_run::AppRun;
type TestPanel = crawrad_project::test_run::TestPanel;
type CheckPanel = crawrad_designer::form_check_panel::CheckPanel;
use crawrad_note::note;
use crawrad_project::note_preview::DoctestRunner;
/// Alias: `#[derive(Live)]` rejects a field whose type is a long path.
type EnvironmentSession = crawrad_project::environment_session::EnvironmentSession;
type EnvironmentSessionCell = std::cell::RefCell<EnvironmentSession>;
pub type AiPermRx = std::sync::mpsc::Receiver<std::sync::Arc<crawrad_agent::PermissionRequest>>;
#[note("ide-app-aidiffreviews")]
pub type AiDiffReviews = crawrad_agent::diff_review::DiffReviewManager;
#[note("ide-app-lsphints")]
pub type LspHints = crawrad_lsp::EditorAssist;
pub type NavHistory = crawrad_host::navigation::Aids;
pub type CodeOffer = crawrad_lsp::code_action::Offer;
pub type MarksSeen = (u64, Option<std::path::PathBuf>);
pub type ExtViews = crawrad_extension_host::views::ViewPanel;
pub type EditorUndoStacks = std::collections::HashMap<std::path::PathBuf, Vec<String>>;
type HistoryTab = crawrad_host::history_view::HistorySession;
type AutoSaveMode = crawrad_project::workspace::AutoSave;
type RecoveryMirror = crawrad_project::recover::Mirror;
type RecoveryOffer = crawrad_project::recover::Offer;
type DesktopWindow = crawrad_project::desktop::WindowPlace;
type KeptDesktop = (PathBuf, crawrad_project::desktop::Desktop);
type ChangeTracker = makepad_code_editor::line_change::ChangeTracker;
// The derive reads a bare type name only: a plain path is not a field form it knows.
type Problems = crawrad_host::Problems;
type ProjectOptionsState = crawrad_actions::project_options_dialog::ProjectOptionsState;
type TemplateManager = crawrad_insight::templates::TemplateManager;
type TemplateSelected = crawrad_insight::templates::Selected;

#[derive(Live)]
pub struct IdeApp {
    #[live]
    ui: WidgetRef,
    /// Measures text in the completion row's font.
    #[live]
    measure: DrawText,
    /// The state model and non-visual API of CrawRAD (ADR 0044).
    #[rust]
    pub host: HostModel,
    /// The gesture engine: 155 headless tests, and not a single line of it lives
    /// here. The shell only TRANSLATES (ADR 0027).
    // **The IDE catalog, not the stock one.** This is what makes an installed
    // component appear in the palette and tree — ADR 0033 §9.
    #[rust(crawrad_designer::Designer::with_registry(crawrad_project::installed::ide_registry()))]
    designer: Designer,
    /// The revision at which the document was last saved — determines the
    /// `saved` / `modified` badge.
    #[rust]
    rev_salva: u64,
    /// The (Save, Save All) lit state last applied to the toolbar.
    #[rust]
    save_buttons_drawn: Option<(bool, bool)>,
    /// The project member whose framework the generator target follows.
    #[rust]
    framework_of: Option<std::path::PathBuf>,
    /// The latest gesture or action message, if any. It takes precedence over
    /// the file path on the bar, as in the egui shell.
    #[rust]
    status: Option<String>,
    /// Whether Preview mode is active.
    #[rust]
    vivo: bool,
    /// Whether the window is currently maximized.
    #[rust]
    maximizada: bool,
    /// Window bounds before maximizing, for restore.
    #[rust]
    restore_to: Option<crawrad_os::ScreenRect>,
    /// Last build diagnostics, or None prior to first build.
    #[rust]
    diagnosticos: Option<Vec<crate::view_models::diagnostics_model::Diagnostic>>,
    /// The Problems panel: errors and warnings of the last build, by file.
    #[rust]
    problems: Problems,
    /// The rows a dialog's list drew (the Options page tree): text and rect.
    #[rust]
    dialog_rows: Vec<(String, Area)>,
    /// Pending one-time fit to screen geometry on startup.
    #[rust(true)]
    fit_on_screen_pending: bool,
    /// Failure reason if opening failed.
    #[rust]
    falha: Option<String>,
    #[rust(true)]
    initial_sync: bool,
    /// Open editor file tabs.
    #[rust]
    abas: EditorTabs,
    #[rust]
    language: Language,
    /// Active language preview in visual designer (None for source English).
    #[rust]
    pub design_language: Option<String>,
    /// The previewed language's entries, read once when it is chosen (never while painting).
    #[rust]
    pub design_locale: Option<crawrad_designer::translations::Locale>,
    /// The project's locale codes, read when a language menu opens or a language is chosen.
    #[rust]
    pub language_codes: Vec<String>,
    /// Language code for project execution (None for system language).
    #[rust]
    pub run_language: Option<String>,
    #[rust]
    tool_armed: Option<String>,
    /// Filter text for the toolbox palette.
    #[rust]
    filtro: String,
    /// Status bar text cache to avoid redundant redraws.
    #[rust]
    barra_na_tela: Vec<String>,
    /// Editor footer text cache.
    #[rust]
    rodape_do_editor_na_tela: Vec<String>,
    /// Current text shown in the code view.
    #[rust]
    texto_na_vista: String,
    /// Unmodified disk text baseline for dirty tracking.
    #[rust]
    base_na_vista: String,
    /// Remembers each file's initial session text for change bars.
    #[rust]
    change_tracker: ChangeTracker,
    /// Selection cached for structure tree redraw checks.
    #[rust]
    selecao_na_arvore: Vec<NodeId>,
    /// File editable right now, or None for read-only view.
    #[rust]
    editavel: Option<PathBuf>,
    #[rust]
    find: EditorFind,
    #[rust]
    complete: EditorComplete,
    #[rust]
    pub lsp: Option<std::sync::Arc<crawrad_lsp::LspClient>>,
    #[rust]
    pub lsp_versao: i32,
    #[rust]
    pub lsp_hints: LspHints,
    #[rust]
    nav: NavHistory,
    #[rust]
    notes: NotesPanel,
    #[rust]
    inlay_hints_off: bool,
    /// rust-analyzer's pill while there is no client: offline, starting,
    /// stopped, or why it failed to start.
    #[rust]
    pub lsp_cell: LspCell,
    /// The pill as drawn, and its progress line as fitted in its cell.
    #[rust]
    lsp_shown: (LspCell, String),
    /// The frame that turns the busy pill's arc.
    #[rust]
    lsp_spin: NextFrame,
    #[rust]
    goto_dialog_aberto: bool,
    #[rust]
    goto_input_texto: String,
    #[rust]
    goto_status_msg: Option<String>,
    #[rust]
    quick_open_aberto: bool,
    /// The New Inherited Form dialog, while it is open.
    #[rust]
    inherit_form: Option<crawrad_project::inherited_form::NewInheritedForm>,
    /// The New Component wizard dialog, while it is open.
    #[rust]
    new_component: Option<crawrad_project::new_component::NewComponentWizard>,
    #[rust]
    new_component_fresh: bool,
    /// The Code Templates manager dialog.
    #[rust]
    templates_dialog: Option<TemplateManager>,
    #[rust]
    templates_selected_seen: Option<TemplateSelected>,
    /// The "Shrink the Release build" dialog.
    #[rust]
    pub shrink_dialog: Option<ShrinkDialog>,
    #[rust]
    component_rename: Option<crawrad_host::rename::ComponentRename>,
    #[rust]
    component_rename_ticket: Option<u64>,
    /// The status bar's target cell for a project without forms (console or library).
    #[rust]
    project_kind: Option<String>,
    /// Whether the open project has no form files.
    #[rust]
    project_without_form: bool,
    /// The project root and active file it was measured for: it reads the
    /// disk, so `sincronizar` measures again only when they change.
    #[rust]
    project_kind_key: (Option<PathBuf>, Option<PathBuf>),
    /// Where the code file in front lives, see `code_file_place`.
    #[rust]
    code_file_place: CodePlaceCell,
    /// The dialog was just opened or its base changed: refill the name and preview.
    #[rust]
    inherit_form_fresh: bool,
    /// The dialog's name field takes the keyboard once it has been drawn.
    #[rust]
    inherit_name_focus: bool,
    #[rust]
    quick_open_filtro: String,
    #[rust]
    quick_open_arquivos: Vec<PathBuf>,
    #[rust]
    quick_open_filtrados: Vec<PathBuf>,
    #[rust]
    quick_open_comandos: Vec<QuickOpenCommandItem>,
    #[rust]
    quick_open_focar: bool,
    #[rust(true)]
    pub minimap_visivel: bool,
    /// File encoding (BOM and line endings).
    #[rust]
    encoding_na_vista: Encoding,
    /// Designer clipboard storage for cut/copy/paste actions.
    #[rust]
    transferencia: Clipboard,
    /// Canvas zoom level (screen points per design unit).
    #[rust(crawrad_runtime::ZOOM_ONE)]
    zoom: f32,
    /// UI scale multiplier.
    #[rust(ui_zoom::UM)]
    zoom_da_ide: f64,
    /// Desktop DPI scaling factor.
    #[rust]
    dpi_base: f64,
    /// The text field that currently holds keyboard focus, if any.
    #[rust]
    campo_focado: Option<WidgetRef>,
    /// The Object Inspector draft — see [`Edicao`].
    #[rust]
    edicao: Option<Edicao>,
    /// The options that each Inspector combo row ALREADY has on screen.
    #[rust]
    enums_na_tela: CacheDeOpcoes,
    /// What the Object Inspector header ALREADY shows, for the same reason as
    /// [`IdeApp::barra_na_tela`].
    #[rust]
    alvo_na_tela: String,
    /// The Object Inspector tab — `Properties` or `Events`.
    #[rust]
    aba: InspectorTab,
    /// The draft of the `Events` tab — see [`EventEdit`].
    #[rust]
    edicao_de_evento: Option<EventEdit>,
    /// Properties with sub-rows expanded — see [`GruposAbertos`].
    ///
    /// Only the effect of the `inspector.toggle_group` action modifies it, never a
    /// click reading this field directly: folding is a capability, and a capability that
    /// the shell resolves internally never reaches the protocol.
    #[rust]
    grupos_abertos: GruposAbertos,
    /// The Object Inspector categories that are CLOSED.
    ///
    /// CLOSED, rather than open, because the default is open: four of the five
    /// start visible, and an open set would have to be seeded with
    /// four names that are already in `ORDER`. The only closed category is seeded by
    /// `inspector::categories_closed_on_open()`, derived directly from
    /// `ORDER` — this shell writes `Advanced` nowhere.
    ///
    /// Only the effect of the `inspector.toggle_category` action modifies it, never a
    /// click reading this field directly — the same rule as `grupos_abertos`.
    #[rust(inspector::categories_closed_on_open())]
    categorias_fechadas: GruposAbertos,
    /// The name/value splitter, moved only by `inspector.set_name_width`.
    #[rust]
    name_splitter: NameSplitter,
    /// The property whose window editor is OPEN, if any.
    ///
    /// See [`PadAberto`]. Opened and closed by the actions
    /// `inspector.open_editor` and `inspector.close_editor`.
    #[rust]
    pad: Option<PadAberto>,
    /// The screen position of the row requesting the pad, used to anchor the picker.
    #[rust]
    pad_pos: Option<DVec2>,
    /// The previous form rectangle, for the anchors and alignment pass.
    #[rust]
    form_anterior: Option<crawrad_ir::Rect>,
    /// The open **Items Editor**, if any.
    #[rust]
    items: Option<ItemsEditorState>,
    /// What the Items Editor ALREADY has on screen, to avoid rewriting per frame.
    #[rust]
    items_na_tela: Option<items_editor::model::ItemPath>,
    /// The open **Lines Editor**, if any.
    #[rust]
    lines: Option<LinesEditorState>,
    /// Does the Lines Editor ALREADY have the text on screen?
    ///
    /// Same reason as [`IdeApp::items_na_tela`] right above: `set_text` clears the
    /// field history, so rewriting per frame would kill `Ctrl+Z` inside
    /// it with each character. A `bool` suffices here because the dialog edits a
    /// single property — changing properties means closing and reopening.
    #[rust]
    lines_na_tela: bool,
    /// The Color dialog and where its OK writes; `None` when closed.
    #[rust]
    color: Option<(
        crawrad_actions::color_dialog::ColorDialog,
        crawrad_actions::color_dialog::ColorTarget,
    )>,
    /// Cursor landing waiting for the next frame.
    ///
    /// See [`IdeApp::pousar_cursor`]: the editor area only exists after it
    /// draws, and focus — which TURNS ON the caret — requires it.
    #[rust]
    pouso_pendente: Option<PousoCodigo>,
    /// Frames left to re-reveal the caret after a landing (fresh tab layout).
    #[rust]
    reveal_after_landing: u8,
    /// The open **ImageList Editor**, if any.
    ///
    /// The state is [`crate::image_list_editor::model::ImageListEditorState`] —
    /// the SAME as the egui shell, and with it comes the inner window (`Add Icons from
    /// Catalog`), which is a field of it rather than a separate state: it is modal
    /// OVER this editor, and does not exist without it.
    #[rust]
    imagelist: Option<ImageListEditorState>,
    /// The icon GALLERY: stock icons plus any added by the user.
    ///
    /// Read from disk when the editor OPENS, not on every frame: it is a
    /// file, and reading a file every frame prevents the window from sleeping. Storing it
    /// here also makes newly added icons appear in the list within the same
    /// gesture, without reopening anything.
    #[rust]
    galeria: Vec<image_list_editor::GalleryIcon>,
    /// Is the icon sheet currently on screen?
    #[rust]
    icones: bool,
    /// Open `New Project…`: the selected type and entered name.
    ///
    /// A tuple rather than a struct because it is two fields and no rules: `workspace`
    /// knows what a valid name is, and the catalog knows what a type is.
    #[rust]
    novo_projeto: Option<crate::ide_app::dialogos::new_project::NovoProjetoDados>,
    /// The rejection of the LAST creation attempt, if any.
    ///
    /// It REMAINS on screen until the next attempt: without it, a rejected `Choose folder…`
    /// would close the dialog and report only via the status bar — a gesture without a visible
    /// result, which is the class of defect this shell eliminated from the screen.
    #[rust]
    np_recusa: Option<String>,
    /// In-flight creation, if any.
    #[rust]
    criando: Option<crawrad_project::creation::Creation>,
    /// Background project creation tasks.
    #[rust]
    creation_tasks: CreationTasks,
    /// Is the **Unsaved changes** dialog on screen?
    ///
    /// `Some` while it is: what to do once the answer is Save or Discard.
    #[rust]
    confirmando_saida: Option<AfterSaving>,
    /// Is the **Options** dialog on screen?
    #[rust]
    options: bool,
    /// Project › Options… open: the project folder and the dialog's state.
    #[rust]
    project_options: Option<(PathBuf, ProjectOptionsState)>,
    /// The chosen page in its tree.
    #[rust]
    secao: OptionsCategory,
    /// The COLLAPSED groups in the Options tree.
    ///
    /// Seeded with [`options::closed_on_open`] when the dialog opens: daily
    /// configuration remains visible, and the rest expands upon search.
    #[rust]
    grupos_options_fechados: GruposAbertos,
    /// Is the designer grid VISIBLE?
    ///
    /// Separate from its size (`designer.grid`), allowing toggling the
    /// checkbox to restore the user's previous grid — instead of a
    /// zero-sized grid.
    #[rust(true)]
    ver_grade: bool,
    /// The Tab-order badges on the canvas (the Designer bar's `Tab order` button).
    #[rust(true)]
    show_tab_order: bool,
    /// What Options found when opened, for `Cancel` to roll back.
    ///
    /// **A snapshot, not a `Ctrl+Z`**: options are not the document, do not
    /// enter history, and what `Cancel` on a preferences dialog promises is
    /// *leave it as it was* — not *undo the last gesture*.
    #[rust]
    options_antes: Option<(f32, bool, bool, crawrad_project::workspace::Preferences)>,
    /// Snapshot of the ACP configuration when the Options dialog was opened (for rollback on Cancel).
    #[rust]
    acp_antes: Option<crawrad_agent::AcpConfig>,
    /// The search term in Settings/Options.
    #[rust]
    options_busca: String,
    /// The scope tab in Settings: 0 = User, 1 = Workspace.
    #[rust]
    options_scope: usize,
    /// The active keyboard shortcut scheme in the IDE (VS Code by default, or the classic one).
    #[rust(KeymapScheme::VsCode)]
    esquema_teclado: KeymapScheme,
    /// Is the **Aefos Memory** dialog on screen?
    #[rust]
    memoria_aberta: bool,
    /// The text being edited in the memory window.
    #[rust]
    memoria_texto: String,
    /// Environment Check dialog session.
    #[rust]
    environment: EnvironmentSessionCell,
    #[rust]
    environment_open: bool,
    /// Is the **MCP Servers** dialog on screen?
    #[rust]
    mcp_aberto: bool,
    /// ACP (Agent Client Protocol) configuration.
    #[rust(AcpConfig::load())]
    pub acp_config: AcpConfig,
    /// AI chat state, prompt queue, audio, and ACP session (ADR 0043, PR 7).
    #[rust]
    pub chat: ChatState,
    /// Integrated terminal state and sessions (Aefos Terminal).
    #[rust]
    terminal: TerminalModel,
    /// Pending AI code diff reviews.
    #[rust]
    pub diff_reviews: AiDiffReviews,
    /// Active change index during diff review navigation.
    #[rust]
    pub review_change_index: usize,
    /// Editor undo stack per file (each AI accept becomes 1 undo transaction).
    #[rust]
    pub editor_undo_stacks: EditorUndoStacks,
    /// The History tab: its file, the copy compared and the block in focus.
    #[rust]
    history: HistoryTab,
    /// The folders of the Welcome page's rows, in the order drawn.
    #[rust]
    welcome: Vec<PathBuf>,
    /// The auto save chosen in Options, read at startup and on change.
    #[rust]
    auto_save: AutoSaveMode,
    /// `AutoSave::AfterDelay`'s clock, restarted by each keystroke.
    #[rust]
    auto_save_clock: Timer,
    /// The recovery copies of the open project and their clock.
    #[rust]
    recover: RecoveryMirror,
    #[rust]
    recover_clock: Timer,
    /// What the "Recover unsaved work" dialog offers; empty, it is closed.
    #[rust]
    recover_offer: RecoveryOffer,
    /// The Snap button's tooltip is on screen.
    #[rust]
    snap_hint: bool,
    /// The inherited component whose badge has the tooltip.
    #[rust]
    badge_hint: Option<NodeId>,
    /// The greyed Toolbox row whose reason is in the tooltip.
    #[rust]
    toolbox_hint: Option<usize>,
    /// The form check finding whose pin has the hover tip.
    #[rust]
    form_check_tip: Option<usize>,
    /// The greyed menu row (submenu?, row) whose reason is in the tooltip.
    #[rust]
    menu_hint: Option<((bool, usize), String)>,
    /// `Rename`: put the cursor in the Name field when it is drawn.
    #[rust]
    name_focus_pending: bool,
    /// Whether the editor had the keyboard on the last event.
    #[rust]
    editor_had_focus: bool,
    /// The project whose desktop is kept on close.
    #[rust]
    desktop_root: Option<PathBuf>,
    /// The window its desktop kept, placed once at startup.
    #[rust]
    startup_window: Option<DesktopWindow>,
    /// Its tabs, waiting for the code view they open into.
    #[rust]
    desktop_tabs: Option<KeptDesktop>,
    /// See [`IdeApp::remake_editor_view`].
    #[rust]
    remake_editor: bool,
    /// Reception channel for MCP requests from the background listener.
    #[rust]
    mcp_rx: Option<Receiver<McpRequest>>,
    /// Embedded MCP server translating requests into catalog actions.
    #[rust]
    mcp_server: Option<McpServer>,
    /// Is the decoupled notes dialog (Sidecar Code Notes) open?
    #[rust]
    nota_dialog_aberta: bool,
    /// Headless model of the note dialog.
    #[rust]
    nota: Option<crawrad_project::note_dialog::NoteDialogModel>,
    /// Background doctest runs of the note dialog.
    #[rust]
    doctest: DoctestRunner,

    /// The open **ActionList Editor**: the list id, if any.
    ///
    /// Only the id; the rest is re-read from the document on every draw: the window shows
    /// the list's actions, and they change underneath it — `New Action` and
    /// `Standard…` create, `Delete` removes. A snapshot stored here would result in the
    /// window showing what the list was when it opened.
    #[rust]
    actions: Option<NodeId>,
    /// The stack of owner nodes when navigating into nested collections (e.g. Grid -> GridColumn).
    #[rust]
    actions_stack: Vec<NodeId>,
    /// The chosen category in the left column.
    ///
    /// `(All)` is the initial value, and is one of the two pseudo-categories in
    /// [`action_list`] — written there, not here.
    #[rust(action_list::CAT_ALL.to_string())]
    categoria_de_acao: String,
    /// Is the **standard classes** window open?
    ///
    /// `bool` rather than `Option<State>`: it has no state of its own beyond what
    /// is checked and what is collapsed, both living in the two fields
    /// below.
    #[rust]
    standard: bool,
    /// Classes selected by the user in it.
    #[rust]
    classes_marcadas: GruposAbertos,
    /// COLLAPSED categories in the standard classes window.
    ///
    /// Stores what is CLOSED, and is populated when the window opens — see
    /// [`IdeApp::open_standard`]. Storing what is open would make an empty
    /// set mean *"all closed"*, and the window would open with nothing
    /// in view.
    #[rust]
    grupos_padrao_fechados: GruposAbertos,
    /// Property key whose text the pad ALREADY has on screen.
    ///
    /// Without it, drawing would rewrite the pad field every frame — and
    /// `TextInput::set_text` clears the field's internal history
    /// (`text_input.rs:1341`), so `Ctrl+Z` inside the pad would die on
    /// every keystroke. This is the same reason as the line draft ([`Edicao`]) and
    /// [`IdeApp::barra_na_tela`].
    #[rust]
    pad_na_tela: Option<String>,
    /// The top menu that is OPEN, if any.
    ///
    /// `Option` because having no menu open is the normal state, and clicking
    /// the title of an open menu closes it — as in any menu bar.
    /// Its content does not live here: it is reassembled from the catalog on every
    /// draw (`menu::rows`), because `Action::habilitada` depends on
    /// selection and document revision, which change underneath an
    /// open menu.
    #[rust]
    open_menu: Option<Aberto>,
    #[rust]
    menu_keyboard: Option<(Aberto, usize)>,
    /// The submenu open beside the bar menu: its name, the menu row it
    /// opens from, and the row the arrow keys moved to inside it.
    #[rust]
    pub open_submenu: Option<(std::borrow::Cow<'static, str>, usize, Option<usize>)>,
    /// Where each rail button REMAINED on the last frame.
    ///
    /// Exists because the context menu needs to know **which panel** the
    /// right click hit, and only drawing knows: `PortalList` only delivers
    /// items inside the draw loop (`portal_list.rs:1064`) and inside the
    /// action loop — never in raw `MouseDown`, which is where right-clicks
    /// arrive.
    ///
    /// Storing the MEASURED RECTANGLE, rather than computing an item height via a
    /// constant, follows the same rule governing the top menu position:
    /// *ask the screen for screen coordinates*. The top bar of this shell does not
    /// start at `y = 0`, and hardcoding 34 there led to bugs.
    #[rust]
    rail_rects: Vec<(Panel, Rect)>,
    #[rust]
    explorer_context_target: ExplorerTarget,
    /// The last project line clicked, and when: a second click soon after on
    /// the same line activates it instead of folding it again (owner, 13/11).
    #[rust]
    last_member_click: Option<(usize, std::time::Instant)>,
    #[rust]
    explorer_rects: ExplorerRects,
    #[rust]
    explorer_drawn: ExplorerDrawn,
    #[rust]
    explorer_row_targets: ExplorerRowTargets,
    #[rust]
    unsaved_tabs: UnsavedTabFiles,
    /// Where each row of the `Events` tab WAS on the last frame.
    #[rust]
    rects_dos_eventos: Vec<(usize, Rect)>,
    /// The last tap of the primary button — where and when.
    ///
    /// The memory required by Makepad's tap counter, which it does
    /// not expose: `Cx::fingers::tap_count` is `pub(crate)`
    /// (`makepad-platform-1.0.0/src/event/finger.rs:457`). The CONSTANTS belong
    /// to it, preventing this shell from having a divergent notion of what a
    /// double-click is — see [`crawrad_runtime_makepad::canvas::is_double_tap`].
    #[rust]
    ultimo_toque: Option<(DVec2, f64)>,
    /// What is expanded and collapsed in the Explorer tree.
    ///
    /// # Why this is a field HERE, and why it tracks the document with it
    ///
    /// The egui shell lacks this field, and its comment explains why: *"a
    /// field there would be yet another state to clear on document switch, and
    /// forgetting to clear has cost dearly here"*. There, the set lives in the
    /// framework's own memory, keyed by the member directory.
    ///
    /// Makepad lacks this implicit memory, so ownership must be
    /// explicit — bringing back the exact risk egui avoided for
    /// free. The answer is [`Folds`], providing GUARD rather than discipline: the
    /// set remembers **which document it belongs to**, and each read supplies
    /// the current document. A different document rebuilds the set
    /// immediately, leaving no calls that could be forgotten.
    #[rust]
    folds: Folds,
    /// The package manager: search, what is installed, and the two
    /// channels for `cargo` threads.
    ///
    /// It is the SAME type used by the egui shell — `crawrad_project::package_manager`, which
    /// both binaries compile via `#[path]`. The only difference is the painter: there a
    /// 720 px window, here a dock tab (ADR 0014, Amendment 8).
    #[rust]
    packages: PackageStore,
    /// Package Manager search field text.
    ///
    /// Lives here rather than inside `TextInput` for the same reason as
    /// [`IdeApp::filtro`] in Toolbox: the list is assembled by a pure function
    /// over state, and reading the widget during drawing would tie the rule to
    /// window existence.
    #[rust]
    packages_query: String,
    /// What the Package Manager header and status bar ALREADY show.
    ///
    /// For the same reason as [`IdeApp::barra_na_tela`]: rewriting a label on
    /// every frame triggers a redraw on every frame (`label.rs:247-249`), and the
    /// window would never sleep — except here the text updates automatically when the
    /// `cargo` thread responds.
    #[rust]
    packages_na_tela: (String, String),
    /// Background build tasks.
    #[rust]
    build_tasks: BuildTasks,
    #[rust]
    web_run: WebRun,
    #[rust]
    app_run: AppRun,
    /// The Tests panel.
    #[rust]
    tests: TestPanel,
    /// The Form check panel.
    #[rust]
    form_check: CheckPanel,
    /// The Sizes preview tab.
    #[rust(SizePreview::new(crawrad_designer::size_preview::SizePlatform::Desktop))]
    size_preview: SizePreview,
    #[rust]
    size_preview_rev: u64,
    /// The Translations tab.
    #[rust]
    translations_rev: u64,
    #[rust]
    translations_missing_only: bool,
    /// CrawRAD extension host: manifests, IDE services, and active contributions.
    #[rust]
    extensions: ShellExtensionHost,
    /// Unified registry of IDE capabilities (native and extension actions).
    #[rust]
    capabilities: CapabilityRegistry,
    /// Is the user dragging a panel splitter?
    #[rust]
    arrastando_divisor: bool,
    /// The resize cursor is ours to take back when the mouse leaves the divisor.
    #[rust]
    cursor_on_divisor: bool,
    /// Build manager and compilation settings for the IDE (ADR 0043, PR 7).
    #[rust]
    pub build: BuildState,
    /// Clock ticking while a build command is running.
    #[rust]
    pub build_clock: Timer,
    // The code actions at the caret's place, and the request on the way.
    #[rust]
    code_offer: CodeOffer,
    // The extensions' marks last given to the view, and for which file.
    #[rust]
    marks_seen: MarksSeen,
    // The Extensions panel: the views as last seen, filter, closed rows.
    #[rust]
    ext_views: ExtViews,
    #[rust]
    disk: DiskWatch,
    #[rust]
    class_icon_strip: ClassIconStrip,
}

impl IdeApp {
    /// A dialog is open: its veil is up. Every dialog sits in a
    /// `CrawradDialogVeil`, and the MCP's dialog table names them all.
    pub fn modal_open(&self) -> bool {
        self.open_veil().is_some()
    }

    /// Shows or hides a dialog's veil (a `ModalVeil`, which `view()` cannot reach).
    pub(crate) fn show_veil(&self, cx: &mut Cx, veil: &[LiveId], on: bool) {
        self.ui.widget(veil).set_visible(cx, on);
    }

    /// The veil of the open dialog.
    pub(crate) fn open_veil(&self) -> Option<WidgetRef> {
        let mut veils = crawrad_mcp::dialogs::veil_roots();
        veils.find_map(|id| Some(self.ui.widget(&[LiveId::from_str(id)])).filter(|v| v.visible()))
    }

    /// A press or a wheel outside the open dialog's frame: it must not reach
    /// the window behind the dialog.
    pub(crate) fn pointer_outside_modal(&self, cx: &Cx, event: &Event) -> bool {
        let at = match event {
            Event::MouseDown(e) => e.abs,
            Event::Scroll(e) => e.abs,
            _ => return false,
        };
        let frame = self.open_veil().map(|v| v.widget(id!(dlg)).area().rect(cx));
        frame.is_some_and(|r| r.size.x > 0.0 && !r.contains(at))
    }

    #[note("ide-app-ideapp-no-form-in-project")]
    pub(crate) fn no_form_in_project(&self) -> bool {
        self.project_without_form && self.host.doc().is_none()
    }

    /// Reads the project's locale codes from disk (`en` is the source, never a file).
    pub(crate) fn valid_languages(&self) -> Vec<String> {
        let root = self.desktop_root.clone().or_else(|| self.raiz());
        let mut codes = root
            .as_deref()
            .map(crawrad_project::locales::list_locales)
            .unwrap_or_default();
        codes.retain(|c| c != "en");
        codes.sort();
        codes
    }

    /// Chooses the Designer's preview language (`None`/`en` = the English source): reads the codes
    /// and that locale's file once, so painting never touches the disk. Unknown code → the list.
    pub(crate) fn set_design_language(&mut self, code: Option<&str>) -> Result<(), Vec<String>> {
        self.language_codes = self.valid_languages();
        let code = code.filter(|c| !c.is_empty() && *c != "en");
        let Some(code) = code else {
            self.design_language = None;
            self.design_locale = None;
            return Ok(());
        };
        if !self.language_codes.iter().any(|c| c == code) {
            return Err(self.language_codes.clone());
        }
        let root = self.desktop_root.clone().or_else(|| self.raiz());
        let entries = root
            .as_deref()
            .map(|r| crawrad_project::locales::read_locale(r, code))
            .unwrap_or_default();
        self.design_locale = Some(crawrad_designer::translations::Locale::from_tuples(
            code, entries,
        ));
        self.design_language = Some(code.to_string());
        Ok(())
    }
}

#[note("ide-app-aberto")]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Aberto {
    /// One of the six top bar titles.
    Barra(menu::Menu),
    /// Panel context menu — `Move To`, `Hide`, `Float`.
    ///
    /// It is the THIRD entry point for layout, and the only one working across all
    /// three platforms today: the command is the first (ADR 0014, Amendment 10),
    /// dragging is the second and **does not exist on Linux** —
    /// `CxOsOp::StartDragging` has no branch in `linux_x11.rs`.
    Painel {
        painel: Panel,
        /// WHERE the right click landed, in window logical pixels.
        ///
        /// The menu opens at the pointer, rather than anchored to a button, which is
        /// what enabled support for `Design` and `Chat`: neither has an item on the
        /// rail (`rail::RAIL` is a subset of `Panel::ALL`), so anchoring to the rail
        /// button would leave both **with no mouse entry point at all** — `Move To`
        /// would only exist where an icon was already present.
        ///
        /// And this is what every context menu does: it appears where the pointer
        /// is.
        em: DVec2,
    },
    /// Settings/Manage popup menu on the Activity Rail gear icon.
    SettingsRail,
    /// More Actions (...) popup menu on the editor tabs.
    EditorMore,
    /// The Run split button popup menu on the top toolbar.
    RunMenu,
    /// The Designer language popup menu on the Designer bar.
    LanguageMenu,
    /// The Object Inspector's component picker.
    InspectorPicker,
    /// Explorer context menu (folders and files).
    ExplorerContext { is_dir: bool, em: DVec2 },
    /// The menu of an inherited event's row in the Inspector.
    EventContext { row: usize, em: DVec2 },
    /// The menu of the component under the pointer on the canvas.
    ComponentContext { em: DVec2 },
    /// The code editor's context menu, at the pointer or the caret.
    EditorContext { em: DVec2 },
    /// The code actions at the caret, under its line.
    /// `teach`: the caret's line has a compiler error the teacher can explain.
    CodeActions { em: DVec2, teach: bool },
    /// rust-analyzer's menu, above its status bar cell (as VS Code's).
    LspStatus,
}

#[note("ide-app-classe-arrastada")]
pub fn classe_arrastada(itens: &[DragItem]) -> Option<String> {
    itens.iter().find_map(|i| match i {
        DragItem::FilePath { path, internal_id }
            if *internal_id == Some(live_id!(crawrad_paleta)) =>
        {
            Some(path.clone())
        }
        _ => None,
    })
}

/// The row of `list` whose `alvo` button `actions` clicked, if any.
pub(crate) fn clicked_row(list: &PortalListRef, actions: &Actions) -> Option<usize> {
    let mut rows = list.items_with_actions(actions).into_iter();
    rows.find_map(|(i, item)| item.button(id!(alvo)).clicked(actions).then_some(i))
}

#[note("ide-app-slot-path")]
pub(crate) fn slot_path(slot: inspector::Slot) -> &'static [LiveId] {
    match slot {
        inspector::Slot::Value => id!(value),
        inspector::Slot::Note => id!(note),
        inspector::Slot::Button => id!(button),
        inspector::Slot::Swatch => id!(swatch),
        inspector::Slot::Fold => id!(fold),
        inspector::Slot::Clear => id!(clear),
    }
}

pub(crate) use crawrad_runtime_makepad::segment_list::rect_of as xywh;

app_main!(IdeApp);

#[cfg(test)]
#[path = "test/ide_app/test.rs"]
mod tests;

#[cfg(test)]
#[path = "test/ide_app/shell.rs"]
mod casca;

// For the same reason as above, and in the same place: the gate for the three fictions
// told by the status bar.
#[cfg(test)]
#[path = "test/ide_app/document.rs"]
mod testes_do_documento;

// The gate preventing Toolbox from regressing to two lists — the DSL one and the
// `handle_actions` one — with the class name written in both.
#[cfg(test)]
#[path = "test/ide_app/palette.rs"]
mod testes_da_paleta;

#[cfg(test)]
#[path = "test/third_party_in_palette/test.rs"]
mod testes_do_terceiro_na_paleta;

#[cfg(test)]
#[path = "test/capability/test.rs"]
mod testes_da_capacidade;

// And the headless gate for this shell's DSL — the only test here addressing
// Makepad's central trap: mistyped names in DSL do not break the build.
#[cfg(test)]
#[path = "test/ide_app/dsl_gate.rs"]
mod dsl_gate;

// The MAXIMIZE gate: the calculation stopping the window at the taskbar
// instead of overflowing the footer.
#[cfg(test)]
#[path = "test/ide_app/maximize.rs"]
mod maximize_tests;

// The WIRES gate: tests above cover pure functions, and a perfect pure
// function never called passes all of them. This one verifies the CALL.
#[cfg(test)]
#[path = "test/ide_app/wires.rs"]
mod testes_dos_fios;

// The keyboard bridge gate: the catalog's 18 shortcuts, arrow keys, and `Esc`.
#[cfg(test)]
#[path = "test/ide_app/keyboard.rs"]
mod testes_do_teclado;

// The gate of the startup warm-up of the Components lists.
#[cfg(test)]
#[path = "test/ide_app/warm_up.rs"]
mod warm_up_tests;

// The Object Inspector gate: composition from the catalog, census of the five
// active editors, and prohibition of dead fields.
#[cfg(test)]
#[path = "test/ide_app/inspector.rs"]
mod testes_do_inspector;

// And the AI panel mockup gate — the one that gave verdicts on the
// user's form without ever measuring anything.
#[cfg(test)]
#[path = "test/ide_app/ai_panel.rs"]
mod testes_do_painel_de_ia;

#[cfg(test)]
#[path = "test/ide_app/terminal_panel.rs"]
mod testes_do_terminal;

#[cfg(test)]
#[path = "test/ide_app/editor_complete.rs"]
mod testes_do_editor_complete;

// The activity RAIL gate: the list of panels, and clicking that triggers
// the catalog action rather than touching the dock from inside.
#[cfg(test)]
#[path = "test/ide_app/rail.rs"]
mod testes_do_trilho;

// And the DOCK gate: panel -> tab mapping, default arrangement, and the file
// in which the arrangement survives window close.
#[cfg(test)]
#[path = "test/ide_app/layout.rs"]
mod testes_da_doca;

// The CENTRE's gate: the view in front, its text derived at that instant.
#[cfg(test)]
#[path = "test/ide_app/source.rs"]
mod testes_do_centro;

// The six menus gate, sibling of the Toolbox gate: no action id or label
// is hardcoded in this shell.
#[cfg(test)]
#[path = "test/ide_app/menu.rs"]
mod menu_tests;

// The icons gate: font, catalog, and shader branches.
#[cfg(test)]
#[path = "test/ide_app/icon.rs"]
mod icon_tests;

// The dock's EXPLORER gate: row ordering, template of each, and —
// the reason for the file's existence — preservation of folds across document switches.
#[cfg(test)]
#[path = "test/ide_app/explorer.rs"]
mod explorer_tests;

// And the PACKAGE MANAGER gate: which row each state becomes, and what the panel says
// when there is nothing to list.
#[cfg(test)]
#[path = "test/ide_app/packages.rs"]
mod packages_tests;

#[cfg(test)]
#[path = "test/ide_app/minimap_tests.rs"]
mod minimap_tests;

#[cfg(test)]
#[path = "test/ide_app/mcp_server_tests.rs"]
mod mcp_server_tests;

#[cfg(test)]
#[path = "test/ide_app/new_project_e2e.rs"]
mod novo_projeto_e2e;

#[cfg(test)]
#[path = "test/ide_app/project_matrix_tests.rs"]
mod matriz_compilacao_projeto;

#[cfg(test)]
#[path = "test/ide_app/extension_reference_e2e.rs"]
mod extension_reference_e2e;

#[cfg(test)]
#[path = "test/ide_app/diff_review_test.rs"]
mod diff_review_test;
