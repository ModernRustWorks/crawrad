//! The top MENU BAR of the Makepad shell — where its rows come from.
//!
//! # Not one action id is written here, and that is the point
//!
//! A menu is a view over the action catalogue, never a second list of
//! commands. Every row below comes from `crawrad_actions::by_group(group)`
//! (`crates/crawrad-actions/src/lib.rs:2111`), which filters the catalogue by
//! `Action::group`: the label is `Action::label`, the shortcut is
//! `Action::shortcut` and the greyed-out state is `Action::enabled(&ctx)`.
//!
//! The reason is the one this shell keeps paying for elsewhere: the Toolbox
//! used to be seventeen rows hand-written in the DSL plus a fourteen-entry
//! table in Rust, and three of the rows armed nothing — silently. A hand
//! written menu is the same defect with a bigger surface, because a command
//! that stops existing leaves a row that still looks clickable.
//!
//! The menu TITLE is the group name, and there is no translation table between
//! them. `Menu::File` draws the group `"File"`; a group renamed in the
//! catalogue renames the menu.
//!
//! # a classic RAD IDE shows and disables — it does not hide
//!
//! The owner proved it with a screenshot: in a classic RAD IDE `Deploy`, `Dependencies`
//! and nearly the whole `Run` menu while debugging are **visible and greyed**.
//! That is exactly what `Action::enabled` is for, so a row that cannot run
//! now is drawn muted and never removed. A menu that changes length as the
//! selection changes is a menu nobody can learn.
//!
//! # What a menu cannot draw
//!
//! An action that REQUIRES an argument (`Action::requires_argument`,
//! `crates/crawrad-actions/src/lib.rs:354`). A menu has nowhere to type one, so
//! drawing it produces a lit row that does nothing — measured in the egui
//! shell, where the `Edit` menu showed two `Standard actions...`, the
//! catalogue one inert and the real one eleven rows below. Whoever needs an
//! argument shows up through a door that can collect it: a dialog, the
//! Explorer, the MCP.
//!
//! # This module does not draw
//!
//! It is a pure function over the catalogue, which is why the content and the
//! order of every menu can be measured without opening a window.

use crate::{Args, Context, DockSide, Panel};
use crawrad_ir::Value;
use crawrad_note::note;
use std::borrow::Cow;

#[note("menu-menu")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    File,
    Edit,
    View,
    Go,
    Refactor,
    Project,
    Run,
    Component,
    Tools,
    Help,
}

impl Menu {
    #[note("menu-menu-all")]
    pub const ALL: &'static [Self] = &[
        Self::File,
        Self::Edit,
        Self::View,
        Self::Go,
        Self::Refactor,
        Self::Project,
        Self::Run,
        Self::Component,
        Self::Tools,
        Self::Help,
    ];

    #[note("menu-menu-title")]
    pub fn title(self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Edit => "Edit",
            Self::View => "View",
            Self::Go => "Go",
            Self::Refactor => "Refactor",
            Self::Project => "Project",
            Self::Run => "Run",
            Self::Component => "Component",
            Self::Tools => "Tools",
            Self::Help => "Help",
        }
    }
}

#[note("menu-nothing-here")]
pub const NOTHING_HERE: &str = "no command here yet";

#[note("menu-menu-path")]
pub fn menu_path(a: &crate::Action) -> String {
    if let Some(sub) = a.place.submenu {
        format!("{} › {} › {}", a.group, sub, a.label)
    } else {
        format!("{} › {}", a.group, a.label)
    }
}

#[note("menu-row")]
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    /// The catalogue or extension id this row runs, or `None` for the notice row.
    pub action: Option<Cow<'static, str>>,
    /// What the row shows — `Action::label`, verbatim.
    pub label: Cow<'static, str>,
    /// The shortcut, as the catalogue declares it for display.
    pub shortcut: Option<Cow<'static, str>>,
    /// Can it run right now? A disabled row is drawn muted and stays.
    pub enabled: bool,
    /// What this row passes to the action. Empty for every row of the top bar.
    pub args: Args,
    /// The panel whose visibility this row MIRRORS — the tick of `View`.
    pub panel_state: Option<Panel>,
    /// Is this row the RULE that separates two blocks, instead of a command?
    pub separator: bool,
    /// Submenu that this row opens, if any.
    pub submenu: Option<Cow<'static, str>>,
    /// How the row is drawn: a command, a title, or a notice.
    pub tone: Tone,
}

/// How a menu row is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tone {
    /// A row that runs something, or would.
    #[default]
    Command,
    /// The small title on top of a context menu: what it speaks of.
    Title,
    /// Two lines of explanation under the rows it explains.
    Notice,
}

impl Row {
    #[note("menu-row-command")]
    pub fn command(a: &crate::Action, ctx: &Context) -> Self {
        Self {
            action: Some(Cow::Borrowed(a.id)),
            label: Cow::Borrowed(a.label),
            shortcut: crate::preferred_shortcut(a.id)
                .or(a.shortcut)
                .map(Cow::Borrowed),
            enabled: a.enabled.allows(ctx),
            args: Args::none(),
            panel_state: None,
            separator: false,
            submenu: None,
            tone: Tone::Command,
        }
    }

    #[note("menu-row-rule")]
    pub fn rule() -> Self {
        Self {
            enabled: false,
            separator: true,
            ..Self::custom("", None)
        }
    }

    #[note("menu-row-submenu")]
    pub fn submenu(name: &'static str) -> Self {
        Self {
            submenu: Some(Cow::Borrowed(name)),
            ..Self::custom(name, None)
        }
    }

    #[note("menu-row-custom")]
    pub fn custom(label: impl Into<Cow<'static, str>>, shortcut: Option<&'static str>) -> Self {
        Self {
            action: None,
            label: label.into(),
            shortcut: shortcut.map(Cow::Borrowed),
            enabled: true,
            args: Args::none(),
            panel_state: None,
            separator: false,
            submenu: None,
            tone: Tone::Command,
        }
    }
}

#[note("menu-rows-with-scheme")]
pub fn rows_with_scheme(menu: Menu, ctx: &Context, scheme: crate::KeymapScheme) -> Vec<Row> {
    rows_with_scheme_and_contributions(menu, ctx, scheme, None, &[])
}

#[note("menu-hidden-by-twin")]
fn hidden_by_twin(a: &crate::Action, group: &[crate::Action], ctx: &Context) -> bool {
    use crate::ViewAffinity::{Code, Design};
    let code = ctx.text.code_in_front;
    let mine = |v| match v {
        Code => code,
        Design => !code,
        _ => true,
    };
    let other = matches!(a.view, Code | Design) && !mine(a.view);
    other
        && group
            .iter()
            .any(|b| b.id != a.id && b.label == a.label && mine(b.view))
}

#[note("menu-drawn-actions")]
fn drawn_actions(menu: Menu, ctx: &Context, scheme: crate::KeymapScheme) -> Vec<crate::Action> {
    let group = crate::by_group_with_scheme(menu.title(), scheme);
    let keep: Vec<bool> = group
        .iter()
        .map(|a| !hidden_by_twin(a, &group, ctx))
        .collect();
    group
        .into_iter()
        .zip(keep)
        .filter_map(|(a, k)| k.then_some(a))
        .collect()
}

#[note("menu-sectioned-rows")]
fn sectioned_rows(
    mut actions: Vec<crate::Action>,
    section: impl Fn(&crate::Action) -> u8,
    collapse: bool,
    ctx: &Context,
) -> Vec<Row> {
    actions.sort_by_key(|a| section(a));
    let mut out: Vec<Row> = Vec::new();
    let mut opened: Vec<&'static str> = Vec::new();
    let mut last = None;
    for a in &actions {
        let rows = match (collapse, a.place.submenu, a.row_per_value()) {
            (true, Some(name), _) if opened.contains(&name) => continue,
            (true, Some(name), _) => {
                opened.push(name);
                vec![Row::submenu(name)]
            }
            (_, _, Some(p)) => {
                let enabled = a.enabled.allows(ctx);
                let row = |c: &crate::Choice| Row {
                    action: Some(Cow::Borrowed(a.id)),
                    label: Cow::Borrowed(c.label),
                    enabled,
                    args: Args::text(p.name, c.id),
                    panel_state: Panel::from_id(c.id),
                    ..Row::custom("", None)
                };
                p.choices().iter().map(row).collect()
            }
            _ if a.requires_argument() => continue,
            _ => vec![Row::command(a, ctx)],
        };
        if rows.is_empty() {
            continue;
        }
        if last.is_some_and(|l| l != section(a)) {
            out.push(Row::rule());
        }
        last = Some(section(a));
        out.extend(rows);
    }
    out
}

#[note("menu-or-nothing-here")]
fn or_nothing_here(mut out: Vec<Row>) -> Vec<Row> {
    while out.first().is_some_and(|r| r.separator) {
        out.remove(0);
    }
    while out.last().is_some_and(|r| r.separator) {
        out.pop();
    }
    if out.is_empty() {
        out.push(Row {
            label: Cow::Borrowed(NOTHING_HERE),
            enabled: false,
            ..Row::custom("", None)
        });
    }
    out
}

#[note("menu-rows-with-scheme-and-contributions")]
pub fn rows_with_scheme_and_contributions(
    menu: Menu,
    ctx: &Context,
    scheme: crate::KeymapScheme,
    registry: Option<&crate::capability::CapabilityRegistry>,
    contributions: &[crawrad_extension_api::contribution::MenuContribution],
) -> Vec<Row> {
    let actions = drawn_actions(menu, ctx, scheme);
    let mut out = sectioned_rows(actions, |a| a.place.section, true, ctx);
    inject_menu_contributions(menu, &mut out, contributions, registry, ctx);
    or_nothing_here(out)
}

#[note("menu-row-json")]
pub fn row_json(r: &Row) -> serde_json::Value {
    serde_json::json!({ "action": r.action.as_deref(), "label": r.label.as_ref(), "shortcut": r.shortcut.as_deref(), "submenu": r.submenu.as_deref(), "enabled": r.enabled, "rule": r.separator })
}

#[note("menu-open-menu-json")]
pub fn open_menu_json(
    menu: Option<String>,
    highlight: Option<usize>,
    rows: &[Row],
    submenu: Option<(&str, &[Row])>,
    rects: serde_json::Value,
) -> serde_json::Value {
    let list = |rows: &[Row]| rows.iter().map(row_json).collect::<Vec<_>>();
    let sub = submenu.map(|(name, rows)| serde_json::json!({ "name": name, "rows": list(rows) }));
    serde_json::json!({ "open": menu.is_some(), "menu": menu, "highlight": highlight, "rows": list(rows), "open_submenu": sub, "rects": rects })
}

#[note("menu-rows-height")]
pub fn rows_height(rows: &[Row], row: f64, rule: f64) -> f64 {
    rows.iter()
        .map(|r| match (r.separator, r.tone) {
            (true, _) => rule,
            (false, Tone::Notice) => 2.0 * row,
            (false, _) => row,
        })
        .sum()
}

#[note("menu-row-at")]
pub fn row_at(rows: &[Row], y: f64, row: f64, rule: f64) -> Option<usize> {
    let mut bottom = 0.0;
    rows.iter().position(|r| {
        bottom += rows_height(std::slice::from_ref(r), row, rule);
        (0.0..bottom).contains(&y)
    })
}

#[note("menu-panel-height")]
pub fn panel_height(rows: &[Row], row: f64, rule: f64, padding: f64) -> f64 {
    rows_height(rows, row, rule).max(row) + 2.0 * padding
}

#[note("menu-next-live-row")]
pub fn next_live_row(rows: &[Row], current: Option<usize>, down: bool) -> Option<usize> {
    let live: Vec<usize> = (0..rows.len())
        .filter(|i| rows[*i].enabled && !rows[*i].separator)
        .collect();
    let n = live.len().max(1);
    let at = current.and_then(|h| live.iter().position(|i| *i == h));
    let next = match (at, down) {
        (None, true) => 0,
        (None, false) => live.len().saturating_sub(1),
        (Some(p), true) => (p + 1) % n,
        (Some(p), false) => (p + n - 1) % n,
    };
    live.get(next).copied()
}

#[note("menu-submenu-rows")]
pub fn submenu_rows(
    menu: Menu,
    name: &str,
    ctx: &Context,
    scheme: crate::KeymapScheme,
) -> Vec<Row> {
    let mut actions = drawn_actions(menu, ctx, scheme);
    actions.retain(|a| a.place.submenu == Some(name));
    or_nothing_here(sectioned_rows(actions, |a| a.place.sub_section, false, ctx))
}

#[note("menu-inject-menu-contributions")]
pub fn inject_menu_contributions(
    target_menu: Menu,
    rows: &mut Vec<Row>,
    contributions: &[crawrad_extension_api::contribution::MenuContribution],
    registry: Option<&crate::capability::CapabilityRegistry>,
    ctx: &Context,
) {
    use crawrad_extension_api::contribution::MenuPosition;

    for contrib in contributions {
        if !contrib.target.eq_ignore_ascii_case(target_menu.title()) {
            continue;
        }

        let (label, shortcut, enabled) = if let Some(reg) = registry {
            if let Some(cap) = reg.get(&contrib.action_id) {
                (
                    Cow::Owned(cap.label.clone()),
                    cap.shortcut.clone().map(Cow::Owned),
                    cap.is_enabled(ctx),
                )
            } else if let Some(act) = crate::by_id(&contrib.action_id) {
                (
                    Cow::Borrowed(act.label),
                    crate::preferred_shortcut(act.id)
                        .or(act.shortcut)
                        .map(Cow::Borrowed),
                    act.enabled.allows(ctx),
                )
            } else {
                (Cow::Owned(contrib.action_id.clone()), None, true)
            }
        } else if let Some(act) = crate::by_id(&contrib.action_id) {
            (
                Cow::Borrowed(act.label),
                crate::preferred_shortcut(act.id)
                    .or(act.shortcut)
                    .map(Cow::Borrowed),
                act.enabled.allows(ctx),
            )
        } else {
            (Cow::Owned(contrib.action_id.clone()), None, true)
        };

        let row = Row {
            action: Some(Cow::Owned(contrib.action_id.clone())),
            label,
            shortcut,
            enabled,
            args: Args::none(),
            panel_state: None,
            separator: false,
            submenu: None,
            tone: Tone::Command,
        };

        match &contrib.position {
            MenuPosition::Top => {
                rows.insert(0, row);
            }
            MenuPosition::Bottom => {
                if contrib.group.is_some()
                    && !rows.is_empty()
                    && !rows.last().map(|r| r.separator).unwrap_or(false)
                {
                    rows.push(Row::rule());
                }
                rows.push(row);
            }
            MenuPosition::Before(target_id) => {
                if let Some(idx) = rows
                    .iter()
                    .position(|r| r.action.as_deref() == Some(target_id.as_str()))
                {
                    rows.insert(idx, row);
                } else {
                    rows.push(row);
                }
            }
            MenuPosition::After(target_id) => {
                if let Some(idx) = rows
                    .iter()
                    .position(|r| r.action.as_deref() == Some(target_id.as_str()))
                {
                    rows.insert(idx + 1, row);
                } else {
                    rows.push(row);
                }
            }
        }
    }
}

#[note("menu-panel-rows")]
pub fn panel_rows(panel: Panel, ctx: &Context) -> Vec<Row> {
    let mut out = Vec::new();
    for side in DockSide::ALL {
        out.push(action_row(
            "layout.move_panel",
            move_label(*side),
            Args::text("panel", panel.id()).and("side", Value::Str(side.id().into())),
            ctx,
        ));
    }
    for id in ["layout.hide_panel", "layout.float_panel"] {
        out.push(action_row(id, None, Args::text("panel", panel.id()), ctx));
    }
    out
}

#[note("menu-placeholder-note")]
pub fn placeholder_note(label: &str) -> Option<&'static str> {
    Some(match label {
        "Check for Updates..." => {
            "CrawRAD v0.1.0-alpha: All tools and dependencies are up to date."
        }
        "Profiles" => "Active profile: Default (Developer / Full Workspace)",
        "\u{2713} Settings Sync is On" => "Settings Sync: on, stored locally in the project",
        "Enable Preview Editors" => "Preview Editors: enabled",
        "Lock Group" => "Editor Group: locked",
        _ => return None,
    })
}

#[note("menu-runs")]
fn runs(
    label: &'static str,
    keys: Option<&'static str>,
    id: &'static str,
    args: Args,
    ctx: &Context,
) -> Row {
    Row {
        shortcut: keys.map(Cow::Borrowed),
        ..action_row(id, Some(label), args, ctx)
    }
}

fn action_row(id: &'static str, label: Option<&'static str>, args: Args, ctx: &Context) -> Row {
    match crate::by_id(id) {
        Some(a) => Row {
            action: Some(Cow::Borrowed(a.id)),
            label: label.map(Cow::Borrowed).unwrap_or(Cow::Borrowed(a.label)),
            // THE PREFERRED FIRST, and that is what this shell delivers: the
            // `Ctrl +` of zoom works here and does not work in the egui shell,
            // whose host owns the key. Showing the other would be the menu
            // teaching the worse chord — see `crawrad_actions::PREFERRED_SHORTCUTS`.
            shortcut: crate::preferred_shortcut(a.id)
                .or(a.shortcut)
                .map(Cow::Borrowed),
            enabled: a.enabled.allows(ctx),
            args,
            panel_state: None,
            separator: false,
            submenu: None,
            tone: Tone::Command,
        },
        // The id vanished from the catalogue. The row STAYS, greyed out and saying what happened —
        // vanishing would be a menu that shrinks without explanation, and this whole
        // file exists so that the menu is never a second list.
        None => Row {
            action: None,
            label: Cow::Borrowed(NOTHING_HERE),
            shortcut: None,
            enabled: false,
            args: Args::none(),
            panel_state: None,
            separator: false,
            submenu: None,
            tone: Tone::Command,
        },
    }
}

#[note("menu-move-label")]
fn move_label(side: DockSide) -> Option<&'static str> {
    Some(match side {
        DockSide::Left => "Move to Left",
        DockSide::Right => "Move to Right",
        DockSide::Bottom => "Move to Bottom",
    })
}

#[note("menu-settings-rail-rows")]
pub fn settings_rail_rows(ctx: &Context) -> Vec<Row> {
    let panel = |p| Args::text("panel", p);
    vec![
        runs(
            "Command Palette...",
            Some("Ctrl+Shift+P"),
            "view.command_palette",
            Args::none(),
            ctx,
        ),
        Row::rule(),
        Row::custom("Profiles", None),
        runs(
            "Settings",
            Some("Ctrl+,"),
            "tools.options",
            Args::none(),
            ctx,
        ),
        runs(
            "Extensions",
            Some("Ctrl+Shift+X"),
            "layout.toggle_panel",
            panel("packages"),
            ctx,
        ),
        Row::custom("Keyboard Shortcuts", Some("Ctrl+K Ctrl+S")),
        runs("Tasks", None, "layout.toggle_panel", panel("terminal"), ctx),
        Row::custom("Themes", Some(">")),
        Row::rule(),
        Row::custom("✓ Settings Sync is On", None),
        Row::rule(),
        Row::custom("Check for Updates...", None),
    ]
}

#[note("menu-lsp-status-rows")]
pub fn lsp_status_rows(version: Option<String>, ctx: &Context) -> Vec<Row> {
    // `rust-analyzer 1.95.0 (59807616 2026-…)`: the build stamp does not fit.
    let version = version.map(|v| v.split(" (").next().unwrap_or_default().to_string());
    let mut header = Row::custom(
        version.unwrap_or_else(|| "rust-analyzer: not running".into()),
        None,
    );
    header.enabled = false;
    vec![
        header,
        Row::rule(),
        action_row("lsp.reload_workspace", None, Args::none(), ctx),
        action_row("lsp.restart", None, Args::none(), ctx),
        action_row("lsp.stop", None, Args::none(), ctx),
    ]
}

#[note("menu-editor-more-rows")]
pub fn editor_more_rows(ctx: &Context) -> Vec<Row> {
    vec![
        runs(
            "Show Opened Editors",
            None,
            "file.quick_open",
            Args::none(),
            ctx,
        ),
        Row::rule(),
        Row::custom("Close All", Some("Ctrl+K W")),
        Row::custom("Close Saved", Some("Ctrl+K U")),
        Row::rule(),
        Row::custom("Enable Preview Editors", None),
        Row::rule(),
        Row::custom("Lock Group", None),
        Row::rule(),
        Row::custom("Configure Editors", None),
    ]
}

/// The Designer bar's language popup: `English (source)` first, then each project language,
/// a check on the previewed one. Each row runs `designer.language` (empty code = English).
pub fn designer_language_rows(codes: &[String], current: Option<&str>) -> Vec<Row> {
    crawrad_designer::translations::language_rows(codes, current)
        .into_iter()
        .map(|(code, name, checked)| {
            let check_prefix = if checked { "✓ " } else { "   " };
            let code_arg = if code == "en" { String::new() } else { code };
            let label = format!("{check_prefix}{name}");
            pick_row(
                "designer.language",
                label,
                Args::none().and("code", Value::Str(code_arg)),
            )
        })
        .collect()
}

/// The Object Inspector's component picker: one row per component (`name  Class`), a check
/// on the inspected one. Each row runs `component.select`.
pub fn inspector_target_rows(
    choices: &[(String, String, String)],
    current: Option<&str>,
) -> Vec<Row> {
    (choices.iter())
        .map(|(id, name, class)| {
            let check_prefix = if current == Some(id.as_str()) {
                "✓ "
            } else {
                "   "
            };
            let label = format!("{check_prefix}{name}   {class}");
            pick_row(
                "component.select",
                label,
                Args::none().and("id", Value::Str(id.clone())),
            )
        })
        .collect()
}

/// One row of a pick list: it runs `action` with `args`, no shortcut.
fn pick_row(action: &'static str, label: String, args: Args) -> Row {
    Row {
        action: Some(Cow::Borrowed(action)),
        label: Cow::Owned(label),
        shortcut: None,
        enabled: true,
        args,
        panel_state: None,
        separator: false,
        submenu: None,
        tone: Tone::Command,
    }
}

/// Whether the View button's arrow opens the language menu; with only the source it just views.
pub fn shows_preview_language(codes: &[String]) -> bool {
    !codes.is_empty()
}

/// The rows of the Run split button menu on the title bar: Run commands followed by
/// the "Run in a language" section.
pub fn run_menu_rows(
    ctx: &Context,
    scheme: crate::KeymapScheme,
    codes: &[String],
    current: Option<&str>,
) -> Vec<Row> {
    let make_row = |id: &'static str| {
        let mut row = action_row(id, None, Args::none(), ctx);
        row.shortcut = crate::shortcut_for_scheme(id, scheme).map(Cow::Borrowed);
        row
    };
    let mut rows = vec![
        make_row("project.run"),
        make_row("project.run_without_debugging"),
        Row::rule(),
    ];
    let mut title_row = Row::custom("Run in a language", None);
    title_row.enabled = false;
    title_row.tone = Tone::Title;
    rows.push(title_row);

    for (code_opt, name, checked) in
        crawrad_designer::translations::run_language_rows(codes, current)
    {
        let check_prefix = if checked { "✓ " } else { "   " };
        let label = format!("{check_prefix}{name}");
        let code_val = code_opt.unwrap_or_default();
        rows.push(Row {
            action: Some(Cow::Borrowed("project.run_language")),
            label: Cow::Owned(label),
            shortcut: None,
            enabled: true,
            args: Args::none().and("code", Value::Str(code_val)),
            panel_state: None,
            separator: false,
            submenu: None,
            tone: Tone::Command,
        });
    }
    rows
}

#[note("menu-editor-keys")]
pub const EDITOR_KEYS: &[(&str, crate::KeymapScheme, &str)] =
    &[("editor.rename_symbol", crate::KeymapScheme::Modern, "F2")];

#[note("menu-code-action-rows")]
pub fn code_action_rows(actions: &[(String, bool)], teach: bool) -> Vec<Row> {
    let mut rows: Vec<Row> = actions
        .iter()
        .enumerate()
        .map(|(i, (title, preferred))| Row {
            action: Some(Cow::Borrowed("editor.apply_code_action")),
            label: Cow::Owned(if *preferred {
                format!("\u{2605} {title}")
            } else {
                title.clone()
            }),
            shortcut: None,
            enabled: true,
            args: Args::none().and("index", Value::Int(i as i64)),
            panel_state: None,
            separator: false,
            submenu: None,
            tone: Tone::Command,
        })
        .collect();
    if teach {
        if !rows.is_empty() {
            rows.push(Row::rule());
        }
        rows.push(teacher_row());
    }
    rows
}

#[note("menu-teacher-row")]
fn teacher_row() -> Row {
    Row {
        action: Some(Cow::Borrowed("chat.fix_explain")),
        ..Row::custom("Fix and Explain", None)
    }
}

pub fn editor_context_rows(ctx: &Context, scheme: crate::KeymapScheme) -> Vec<Row> {
    let row = |id: &'static str| {
        let mut r = action_row(id, None, Args::none(), ctx);
        let editor_key = EDITOR_KEYS
            .iter()
            .find(|(i, s, _)| *i == id && *s == scheme);
        r.shortcut = editor_key
            .map(|(_, _, k)| *k)
            .or_else(|| crate::shortcut_for_scheme(id, scheme))
            .map(Cow::Borrowed);
        r
    };
    let mut groups: Vec<Vec<&'static str>> = vec![
        vec![
            "editor.goto_definition",
            "editor.find_references",
            "editor.show_hover",
        ],
        vec!["editor.toggle_inlay_hints"],
        vec!["editor.rename_symbol", "editor.format_document"],
        vec![
            "ai.add_file_to_chat",
            "ai.add_selection_to_chat",
            "ai.explain_selection",
            "ai.review_selection",
        ],
        vec!["tools.sidecar_note"],
        vec!["editor.cut", "editor.copy", "editor.paste"],
        vec!["editor.select_all"],
        vec!["view.command_palette"],
    ];
    if ctx.text.unit_has_form {
        groups[1].insert(0, "view.toggle_code");
    }
    let mut out = Vec::new();
    for group in groups {
        if !out.is_empty() {
            out.push(Row::rule());
        }
        out.extend(group.into_iter().map(row));
    }
    out
}

#[note("menu-event-context-rows")]
pub fn event_context_rows(node: &str, event: &str, overridden: bool, ctx: &Context) -> Vec<Row> {
    let args = || Args::text("node", node).and("event", Value::Str(event.to_string()));
    let mut go = action_row("event.go_to_handler", None, args(), ctx);
    let mut revert = action_row("event.revert_inherited", None, args(), ctx);
    go.enabled &= overridden;
    revert.enabled &= overridden;
    let inherited = action_row("event.go_to_inherited", None, args(), ctx);
    vec![go, inherited, Row::rule(), revert]
}

#[note("menu-component-context-rows")]
pub fn component_context_rows(name: &str, base_form: Option<&str>, ctx: &Context) -> Vec<Row> {
    let inherited = base_form.is_some();
    let mut out = Vec::new();
    if let Some(form) = base_form {
        out.push(Row {
            enabled: false,
            tone: Tone::Title,
            ..Row::custom(format!("{name} \u{b7} from {form}").to_uppercase(), None)
        });
        let mut go = action_row("form.open_base", None, Args::none(), ctx);
        go.shortcut = Some(Cow::Owned(form.to_string()));
        out.push(go);
    }
    out.push(action_row("inspector.show_events", None, Args::none(), ctx));
    out.push(Row::rule());
    for id in ["edit.delete", "inspector.rename"] {
        let mut row = action_row(id, None, Args::none(), ctx);
        row.enabled &= !inherited;
        out.push(row);
    }
    if inherited {
        out.push(Row {
            enabled: false,
            ..Row::custom("Move to another parent", None)
        });
        out.push(Row {
            enabled: false,
            tone: Tone::Notice,
            ..Row::custom(INHERITED_LOCK_NOTICE, None)
        });
    }
    out
}

/// Why an inherited component cannot be removed, renamed or moved.
pub const INHERITED_LOCK_NOTICE: &str =
    "Comes from the base form: remove, rename or move it there.";

#[note("menu-explorer-context-rows")]
pub fn explorer_context_rows(
    is_dir: bool,
    path: &std::path::Path,
    role: crate::WorkspaceRole,
    ctx: &Context,
) -> Vec<Row> {
    let p = path.to_string_lossy().to_string();
    let mut out = Vec::new();
    // #471: the workspace's root adds projects; a member that is not the active
    // one can become it
    match role {
        crate::WorkspaceRole::Root => {
            out.push(action_row("project.build_all", None, Args::default(), ctx));
            out.push(action_row("project.clean_all", None, Args::default(), ctx));
            out.push(Row::rule());
            out.push(action_row("project.add", None, Args::default(), ctx));
            out.push(action_row(
                "project.add_existing",
                None,
                Args::default(),
                ctx,
            ));
            out.push(Row::rule());
        }
        crate::WorkspaceRole::Member { active: false } => {
            out.push(action_row(
                "project.set_active",
                None,
                Args::text("path", p.clone()),
                ctx,
            ));
            out.push(Row::rule());
        }
        _ => {}
    }
    if is_dir {
        out.push(action_row(
            "explorer.new_file",
            None,
            Args::text("path", p.clone()),
            ctx,
        ));
        out.push(action_row(
            "explorer.new_form",
            None,
            Args::text("path", p.clone()),
            ctx,
        ));
        out.push(action_row(
            "explorer.new_folder",
            None,
            Args::text("path", p.clone()),
            ctx,
        ));
        out.push(Row::rule());
    }
    out.push(action_row(
        "explorer.reveal",
        None,
        Args::text("path", p.clone()),
        ctx,
    ));
    out.push(Row::rule());
    out.push(action_row(
        "explorer.copy_path",
        None,
        Args::text("path", p.clone()),
        ctx,
    ));
    out.push(action_row(
        "explorer.copy_rel_path",
        None,
        Args::text("path", p.clone()),
        ctx,
    ));
    out.push(Row::rule());
    out.push(action_row(
        "explorer.rename",
        None,
        Args::text("path", p.clone()),
        ctx,
    ));
    out.push(action_row(
        "explorer.delete",
        None,
        Args::text("path", p),
        ctx,
    ));
    out
}
