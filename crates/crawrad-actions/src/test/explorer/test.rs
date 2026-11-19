//! The tests of the Explorer fold family.
//!
//! They answer the owner's ruler of 15/09/2026, the same one the layout family
//! answers: *can an agent do this by tool, with no mouse?* Every assertion
//! below runs with no window open — which is the whole point of the command
//! existing before the click.

use std::path::PathBuf;

use crate::{Args, Author, Context, Doc, ExplorerCommand, Project, by_group, by_id, catalogue};
use crawrad_designer::clipboard::Clipboard;
use crawrad_ir::Document;
use crawrad_registry::Registry;

/// Any document — the predicate of the two actions only asks whether one
/// exists, not what is inside it.
fn scenario() -> (Doc, Registry, Clipboard) {
    (
        Doc::new(Document::new("form_main", "Form")),
        Registry::builtin(),
        Clipboard::default(),
    )
}

fn ctx<'a>(d: Option<&'a Doc>, r: &'a Registry, a: &'a Clipboard, args: Args) -> Context<'a> {
    Context {
        doc: d,
        registry: r,
        clipboard: a,
        args,
        project: Project::default(),
        author: Author::User,
        editor_dirty: false,
        text: Default::default(),
    }
}

/// Runs the action by id and returns the Explorer command it requested.
fn command(id: &str, args: Args) -> Option<ExplorerCommand> {
    let (d, r, a) = scenario();
    let c = ctx(Some(&d), &r, &a, args);
    let action = by_id(id).unwrap_or_else(|| panic!("`{id}` is not in the catalogue"));
    (action.execute)(&c).explorer
}

#[test]
fn the_two_folds_are_in_the_catalogue() {
    // Hand-written list on PURPOSE: it is the published contract, and one
    // derived from the catalogue would pass green on the day an action vanished.
    for id in ["explorer.toggle_folder", "explorer.toggle_legend"] {
        assert!(by_id(id).is_some(), "`{id}` vanished from the catalogue");
    }
}

#[test]
fn folding_a_folder_carries_the_path_it_received() {
    assert_eq!(
        command("explorer.toggle_folder", Args::text("path", "/g/app")),
        Some(ExplorerCommand::ToggleFolder(PathBuf::from("/g/app"))),
    );
}

#[test]
fn the_legend_folds_without_any_argument() {
    assert_eq!(
        command("explorer.toggle_legend", Args::none()),
        Some(ExplorerCommand::ToggleLegend),
    );
}

#[test]
fn the_cards_fold_without_arguments() {
    for (id, cmd) in [
        (
            "explorer.toggle_open_editors",
            ExplorerCommand::ToggleOpenEditors,
        ),
        ("explorer.toggle_outline", ExplorerCommand::ToggleOutline),
        ("explorer.toggle_timeline", ExplorerCommand::ToggleTimeline),
        (
            "explorer.toggle_dependencies",
            ExplorerCommand::ToggleDependencies,
        ),
    ] {
        assert_eq!(command(id, Args::none()), Some(cmd));
    }
}

/// Without `path` it **refuses and explains**, instead of folding an empty path.
///
/// An empty `PathBuf` is not any node: the fold would enter the set, would not
/// match any member, and the tree would remain exactly as it was —
/// the gesture that appears to have worked and did nothing.
#[test]
fn without_a_path_the_fold_refuses_and_explains() {
    for args in [Args::none(), Args::text("path", "")] {
        let (d, r, a) = scenario();
        let c = ctx(Some(&d), &r, &a, args);
        let e = (by_id("explorer.toggle_folder").unwrap().execute)(&c);
        assert!(e.explorer.is_none(), "folded without knowing what");
        let sentence = e.status.unwrap_or_default();
        assert!(
            sentence.contains("path"),
            "the refusal does not name the missing argument: {sentence:?}"
        );
    }
}

/// **Without a project there is no tree; without a form there still is.**
///
/// The Explorer root is the project's folder (ADR 0017): a console project has
/// a tree and no form, so the Explorer's commands read `Project::open`, not the
/// document. With no project open the panel only knows how to say `No folder
/// open.`, and nothing may fold a tree that does not exist.
#[test]
fn without_a_project_there_is_nothing_to_fold() {
    let (_, r, a) = scenario();
    let mut c = ctx(None, &r, &a, Args::none());
    let explorer: Vec<_> = (catalogue().into_iter())
        .filter(|x| x.id.starts_with("explorer."))
        .collect();
    for action in &explorer {
        assert!(
            action.enabled.allows(&c),
            "`{}` greyed in a project without a form",
            action.id
        );
    }
    c.project.open = false;
    for action in &explorer {
        assert!(
            !action.enabled.allows(&c),
            "`{}` lit up with no project open — the panel has no root to \
             hang any tree on in this state",
            action.id
        );
    }
}

/// Neither of the two writes to the document.
///
/// This is what separates the family: an agent reading `Effect` to find out what
/// will happen must be able to conclude *"this does not touch the form"* without
/// executing anything.
#[test]
fn folding_is_not_editing() {
    for (id, args) in [
        ("explorer.toggle_folder", Args::text("path", "/g/app")),
        ("explorer.toggle_legend", Args::none()),
        ("explorer.toggle_open_editors", Args::none()),
        ("explorer.toggle_outline", Args::none()),
        ("explorer.toggle_timeline", Args::none()),
        (
            "explorer.open_timeline_entry",
            Args::one("index", crawrad_ir::Value::Int(0)),
        ),
        ("explorer.toggle_dependencies", Args::none()),
    ] {
        let (d, r, a) = scenario();
        let c = ctx(Some(&d), &r, &a, args);
        let e = (by_id(id).unwrap().execute)(&c);
        assert!(e.transaction.is_none(), "`{id}` returned a transaction");
        assert!(e.selection.is_none(), "`{id}` modified the selection");
        assert!(e.file.is_none(), "`{id}` modified disk");
        assert!(e.layout.is_none(), "`{id}` modified panel arrangement");
    }
}

/// The group is `Explorer`, and **no menu bar item draws it**.
///
/// What is open inside a panel is not a window menu item:
/// `View` already lists the six panels and `Reset Layout`, and a third family
/// inside there would be the menu explaining the panel instead of the panel explaining itself.
#[test]
fn the_group_belongs_to_the_panel_and_not_the_bar() {
    for action in catalogue()
        .into_iter()
        .filter(|x| x.id.starts_with("explorer."))
    {
        assert_eq!(action.group, "Explorer", "{} changed group", action.id);
    }
    // And the contrast: the groups drawn by the bar remain without them.
    for group in ["File", "Edit", "View", "Project", "Run", "Help"] {
        assert!(
            !by_group(group)
                .iter()
                .any(|a| a.id.starts_with("explorer.")),
            "an Explorer fold leaked into menu `{group}`"
        );
    }
}

#[test]
fn opening_timeline_entry_parses_its_index() {
    assert_eq!(
        command(
            "explorer.open_timeline_entry",
            Args::one("index", crawrad_ir::Value::Int(2)),
        ),
        Some(ExplorerCommand::OpenTimelineEntry(2)),
    );
    assert_eq!(command("explorer.open_timeline_entry", Args::none()), None,);
}
