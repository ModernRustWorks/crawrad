//! The tests of the layout command family.
//!
//! They answer one question, and it is the owner's ruler of 15/09/2026: *can
//! an agent do this by tool, with no mouse?* Every assertion below runs with
//! no window open, which is the whole point of the command coming before the
//! drag (ADR 0014, Amendment 10).

use crate::{
    Args, Author, Context, DockSide, LayoutCommand, Panel, ProfileCommand, Project, by_group,
    by_id, catalogue,
};
use crawrad_designer::clipboard::Clipboard;
use crawrad_registry::Registry;

/// The emptiest context there is: no form, no selection, no clipboard.
///
/// Deliberately empty. Rearranging panels is not about the document, and a
/// test that set one up would be hiding a dependency that must not exist —
/// the day a layout action starts reading `doc`, this file goes red.
fn scenario() -> (Registry, Clipboard) {
    (Registry::builtin(), Clipboard::default())
}

fn ctx<'a>(r: &'a Registry, a: &'a Clipboard, args: Args) -> Context<'a> {
    Context {
        doc: None,
        registry: r,
        clipboard: a,
        args,
        project: Project::default(),
        author: Author::User,
        editor_dirty: false,
        text: Default::default(),
    }
}

/// Runs a catalogue action by id and returns the layout command it asked for.
fn command(id: &str, args: Args) -> Option<LayoutCommand> {
    let (r, a) = scenario();
    let c = ctx(&r, &a, args);
    let action = by_id(id).unwrap_or_else(|| panic!("`{id}` is not in the catalogue"));
    (action.execute)(&c).layout
}

/// The refusal sentence, for the calls that must not act.
fn refusal(id: &str, args: Args) -> String {
    let (r, a) = scenario();
    let c = ctx(&r, &a, args);
    let action = by_id(id).unwrap();
    let e = (action.execute)(&c);
    assert!(
        e.layout.is_none(),
        "`{id}` acted when it should refuse: {:?}",
        e.layout
    );
    e.status.unwrap_or_default()
}

#[test]
fn every_panel_in_the_catalogue_returns_by_its_own_id() {
    for p in Panel::ALL {
        assert_eq!(
            Panel::from_id(p.id()),
            Some(*p),
            "`{}` does not return via `from_id` — the id is what MCP passes and what the \
             saved arrangement stores",
            p.id()
        );
    }
    assert_eq!(Panel::from_id("does.not.exist"), None);
}

#[test]
fn no_panel_id_is_repeated() {
    let mut seen: Vec<&str> = Vec::new();
    for p in Panel::ALL {
        assert!(!seen.contains(&p.id()), "duplicate id: {}", p.id());
        seen.push(p.id());
    }
}

#[test]
fn no_panel_label_equals_its_id() {
    // The id is stable and the label is translatable. Equal, nobody notices the
    // difference until the day the label changes and all saved arrangements break.
    for p in Panel::ALL {
        assert_ne!(
            p.id(),
            p.label(),
            "`{}` uses the same text as id and as label",
            p.id()
        );
    }
}

#[test]
fn every_dock_side_returns_by_its_own_id() {
    for s in DockSide::ALL {
        assert_eq!(DockSide::from_id(s.id()), Some(*s));
    }
    assert_eq!(DockSide::from_id("top"), None);
}

#[test]
fn the_seven_layout_actions_are_in_the_catalogue() {
    // Hand-written list on PURPOSE: it is the published contract, and one
    // derived from the catalogue would pass green on the day an action vanished.
    for id in [
        "layout.reset",
        "layout.show_panel",
        "layout.hide_panel",
        "layout.toggle_panel",
        "layout.float_panel",
        "layout.move_panel",
        "layout.stack_panel",
    ] {
        assert!(by_id(id).is_some(), "`{id}` vanished from the catalogue");
    }
}

#[test]
fn layout_actions_never_disable() {
    let (r, a) = scenario();
    let c = ctx(&r, &a, Args::none());
    for action in catalogue()
        .into_iter()
        .filter(|x| x.id.starts_with("layout."))
    {
        assert!(
            action.enabled.allows(&c),
            "`{}` disabled without a document — changing layout does not depend on \
             an open form, and an agent has no window to inspect",
            action.id
        );
    }
}

#[test]
fn reset_is_the_only_one_the_menu_can_draw() {
    // A menu has nowhere to type an argument. The six requesting `panel`
    // appear through ports that know how to collect: the rail, tab menu, MCP.
    let drawable: Vec<&str> = by_group("View")
        .into_iter()
        .filter(|a| a.id.starts_with("layout.") && !a.requires_argument())
        .map(|a| a.id)
        .collect();
    assert_eq!(drawable, vec!["layout.reset"]);
}

#[test]
fn reset_requests_default_arrangement_back() {
    assert_eq!(
        command("layout.reset", Args::none()),
        Some(LayoutCommand::Reset)
    );
}

#[test]
fn open_close_toggle_and_float_carry_the_requested_panel() {
    for (id, expected) in [
        ("layout.show_panel", LayoutCommand::Show(Panel::Inspector)),
        ("layout.hide_panel", LayoutCommand::Hide(Panel::Inspector)),
        (
            "layout.toggle_panel",
            LayoutCommand::Toggle(Panel::Inspector),
        ),
        ("layout.float_panel", LayoutCommand::Float(Panel::Inspector)),
    ] {
        assert_eq!(
            command(id, Args::text("panel", "inspector")),
            Some(expected),
            "`{id}` did not carry the requested panel"
        );
    }
}

#[test]
fn move_carries_the_panel_and_the_side() {
    assert_eq!(
        command(
            "layout.move_panel",
            Args::text("panel", "chat").and("side", crawrad_ir::Value::Str("bottom".into()))
        ),
        Some(LayoutCommand::Move {
            panel: Panel::Chat,
            side: DockSide::Bottom
        })
    );
}

#[test]
fn stack_carries_both_panels() {
    assert_eq!(
        command(
            "layout.stack_panel",
            Args::text("panel", "inspector")
                .and("onto", crawrad_ir::Value::Str("components".into()))
        ),
        Some(LayoutCommand::Stack {
            panel: Panel::Inspector,
            onto: Panel::Components
        })
    );
}

#[test]
fn nonexistent_panel_hears_refusal_and_the_list() {
    // On the other side of the protocol there is no window: whoever typed the wrong name needs
    // to hear this, and receive the list of what exists.
    let sentence = refusal("layout.show_panel", Args::text("panel", "inspetor"));
    assert!(
        sentence.contains("no such panel"),
        "poor sentence: {sentence}"
    );
    for p in Panel::ALL {
        assert!(
            sentence.contains(p.id()),
            "the refusal does not mention `{}` — the list is derived from Panel::ALL on \
             purpose, so new panels enter it automatically",
            p.id()
        );
    }
}

#[test]
fn without_any_argument_the_command_states_what_is_missing() {
    let sentence = refusal("layout.move_panel", Args::none());
    assert!(sentence.contains("required"), "poor sentence: {sentence}");
}

#[test]
fn nonexistent_side_hears_refusal() {
    let sentence = refusal(
        "layout.move_panel",
        Args::text("panel", "chat").and("side", crawrad_ir::Value::Str("top".into())),
    );
    assert!(
        sentence.contains("no such side"),
        "poor sentence: {sentence}"
    );
}

#[test]
fn stacking_a_panel_onto_itself_is_refused() {
    let sentence = refusal(
        "layout.stack_panel",
        Args::text("panel", "chat").and("onto", crawrad_ir::Value::Str("chat".into())),
    );
    assert!(sentence.contains("itself"), "poor sentence: {sentence}");
}

#[test]
fn no_layout_action_touches_the_document() {
    // The entire family is about the window. If one of them returned a transaction,
    // the `.crawform` would depend on where the user left the panels —
    // which is exactly what ADR 0019 forbids for zoom, for the same reason.
    let (r, a) = scenario();
    for action in catalogue()
        .into_iter()
        .filter(|x| x.id.starts_with("layout."))
    {
        let c = ctx(
            &r,
            &a,
            Args::text("panel", "inspector").and("onto", crawrad_ir::Value::Str("chat".into())),
        );
        let e = (action.execute)(&c);
        assert!(
            e.transaction.is_none() && e.selection.is_none() && e.file.is_none(),
            "`{}` touched the document or disk",
            action.id
        );
    }
}

#[test]
fn the_layout_parameter_is_declared_for_mcp() {
    // The signature is what MCP publishes. Without it, the tool appears without
    // stating what it receives — and an agent guesses.
    let move_cmd = by_id("layout.move_panel").unwrap();
    let names: Vec<&str> = move_cmd.params.iter().map(|p| p.name).collect();
    assert_eq!(names, vec!["panel", "side"]);
    assert!(move_cmd.params.iter().all(|p| p.required));
    assert!(move_cmd.requires_argument());

    let reset = by_id("layout.reset").unwrap();
    assert!(reset.params.is_empty());
    assert!(!reset.requires_argument());
}

#[test]
fn the_command_can_tell_which_panel_it_refers_to() {
    assert_eq!(
        LayoutCommand::Show(Panel::Explorer).panel(),
        Some(Panel::Explorer)
    );
    assert_eq!(
        LayoutCommand::Move {
            panel: Panel::Chat,
            side: DockSide::Left
        }
        .panel(),
        Some(Panel::Chat)
    );
    assert_eq!(LayoutCommand::Reset.panel(), None);
}

/// The saved-arrangement and DSL names are a compatibility surface: these are
/// the exact strings every build so far has written and declared.
#[test]
fn tab_and_template_names_never_change() {
    let names: Vec<(String, String)> = Panel::ALL
        .iter()
        .map(|p| (p.tab_name(), p.template_name()))
        .collect();
    let expected = [
        ("tab_explorer", "PanelExplorer"),
        ("tab_components", "PanelComponents"),
        ("tab_inspector", "PanelInspector"),
        ("tab_packages", "PanelPackages"),
        ("tab_design", "PanelDesign"),
        ("tab_chat", "PanelChat"),
        ("tab_terminal", "PanelTerminal"),
        ("tab_debug", "PanelDebug"),
        ("tab_references", "PanelReferences"),
        ("tab_problems", "PanelProblems"),
        ("tab_extensions", "PanelExtensions"),
        ("tab_notes", "PanelNotes"),
        ("tab_tests", "PanelTests"),
        ("tab_testoutput", "PanelTestoutput"),
        ("tab_formcheck", "PanelFormcheck"),
    ];
    for (tab, template) in expected {
        assert!(
            names.iter().any(|(t, d)| t == tab && d == template),
            "{tab} / {template} missing: {names:?}"
        );
    }
    assert_eq!(
        names.len(),
        expected.len(),
        "a panel without a fixed name here"
    );
}

/// The three desktop-profile actions carry the trimmed name, and refuse
/// without one rather than acting on an empty name.
#[test]
fn profile_actions_carry_the_name_or_refuse() {
    let (r, a) = scenario();
    let run = |id: &str, args: Args| (by_id(id).unwrap().execute)(&ctx(&r, &a, args));
    let cases = [
        (
            "layout.apply_profile",
            ProfileCommand::Apply as fn(String) -> ProfileCommand,
        ),
        ("layout.save_profile", ProfileCommand::Save),
        ("layout.delete_profile", ProfileCommand::Delete),
    ];
    for (id, make) in cases {
        let e = run(id, Args::text("name", "  Big screen "));
        assert_eq!(e.profile, Some(make("Big screen".into())), "`{id}`");
        for args in [Args::none(), Args::text("name", "   ")] {
            let e = run(id, args);
            assert_eq!(e.profile, None, "`{id}` acted with no name");
            assert!(
                e.status.unwrap_or_default().contains("name"),
                "`{id}` refused silently"
            );
        }
    }
}
