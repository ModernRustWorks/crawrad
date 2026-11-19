//! The gate over the six top menus — the sibling of the Toolbox gate.
//!
//! The defect it locks down is the one the Toolbox already paid for: the name
//! of a command written **twice**, once in the DSL and once in a Rust table.
//! In a menu it is worse, because the row keeps being drawn after the action
//! leaves the catalogue, and a row that does nothing never complains.
//!
//! Two of the tests read the file as TEXT, and that is deliberate: what has to
//! stay locked is the *shape* of the source, not only the result in memory. A
//! correct menu written by hand would pass every behaviour test and bring the
//! defect back on the first new action.

use crawrad_actions::{Args, Author, Context, Project};
use crawrad_designer::clipboard::Clipboard;
use crawrad_registry::Registry;

use super::menu::{self, Menu};

/// The Makepad shell, as text.
use super::casca::CASCA as SHELL;

/// A context with **no document** — the IDE just opened, before anything is
/// loaded.
///
/// It is the state where the most items are greyed out, and therefore the best
/// one to prove that the grey exists at all.
fn with_nothing<R>(f: impl FnOnce(&Context) -> R) -> R {
    let registry = Registry::builtin();
    let area = Clipboard::default();
    let ctx = Context {
        doc: None,
        registry: &registry,
        clipboard: &area,
        args: Args::none(),
        project: Project::default(),
        author: Author::User,
        editor_dirty: false,
        text: Default::default(),
    };
    f(&ctx)
}

/// The menus of the bar, in order, and not one more.
///
/// The order is the egui shell's (`main.rs:4819-4995`), and it is the owner's.
/// A menu added here without a reason would be the Makepad bar drifting from
/// the bar he knows — `Component` has one: it is the classic RAD menu, and it
/// arrived with package installation on 18/09/2026.
#[test]
fn the_bar_has_the_menus_in_the_classic_order() {
    let titles: Vec<&str> = Menu::ALL.iter().map(|m| m.title()).collect();
    // `Component` sits between `Run` and `Help`, which is where a classic RAD IDE puts it.
    assert_eq!(
        titles,
        vec![
            "File",
            "Edit",
            "View",
            "Go",
            "Refactor",
            "Project",
            "Run",
            "Component",
            "Tools",
            "Help"
        ]
    );
}

/// **The `Component` menu shows the TWO installation actions, and nothing else.**
///
/// The catalogue's `Component` group has eight actions: the two suite actions and six
/// DESIGNER ones (`component.add`, `component.set_property`, …), which the MCP calls with
/// arguments. A bar menu has nowhere to type arguments, which is why
/// [`menu::rows`] rejects any action that requires one.
///
/// This test ensures that this rejection continues to separate the two
/// families — without it, `Add Component` would appear in the user's menu as
/// a row that does nothing.
#[test]
fn o_menu_component_so_desenha_o_que_nao_pede_argumento() {
    let linhas = with_nothing(|ctx| menu::rows(Menu::Component, ctx));
    let ids: Vec<&str> = linhas.iter().filter_map(|r| r.action.as_deref()).collect();
    assert_eq!(ids, vec!["component.install", "component.installed"]);
}

#[test]
fn o_menu_tools_tem_a_acao_options() {
    let linhas = with_nothing(|ctx| menu::rows(Menu::Tools, ctx));
    let ids: Vec<&str> = linhas.iter().filter_map(|r| r.action.as_deref()).collect();
    assert_eq!(
        ids,
        vec![
            "editor.manage_templates",
            "tools.options",
            "tools.mcp",
            "tools.ai_memory",
            "tools.sidecar_note"
        ]
    );
}

/// The menu title **is** the catalogue group.
///
/// Without this a menu would open empty and nothing would say why: the title
/// and the group would be two free strings, able to drift apart.
#[test]
fn no_menu_opens_empty() {
    for m in Menu::ALL {
        let rows = with_nothing(|ctx| menu::rows(*m, ctx));
        assert!(
            rows.iter().any(|r| r.action.is_some()),
            "the `{}` menu has no catalogue command at all. The menu title IS \
             the `Action::grupo` it draws — if the group was renamed, the menu \
             opens empty and nothing warns.",
            m.title()
        );
    }
}

/// Each menu shows exactly the actions of its group that a menu can draw.
///
/// The oracle is the catalogue itself, filtered by the SAME rule — not a list
/// written here, which would be rule 8 of the study: two hand-written lists
/// compared against each other.
#[test]
fn each_menu_is_its_group_in_the_catalogue() {
    for m in Menu::ALL {
        let group = crawrad_actions::by_group(m.title());
        // With the designer in front, the text twin of Cut/Copy/Paste/Select
        // all (same label, the code view's) is not listed.
        let twin_of_design = |a: &crawrad_actions::Action| {
            a.view == crawrad_actions::ViewAffinity::Code
                && group
                    .iter()
                    .any(|b| b.label == a.label && b.view == crawrad_actions::ViewAffinity::Design)
        };
        let mut top_actions: Vec<_> = group
            .iter()
            .filter(|a| !twin_of_design(a) && a.place.submenu.is_none())
            .collect();
        top_actions.sort_by_key(|a| a.place.section);
        let expected: Vec<&str> = top_actions
            .iter()
            .flat_map(|a| match a.row_per_value() {
                // One row per value of the argument: the id repeats, once per
                // choice — see `Domain::row_per_value`.
                Some(p) => vec![a.id; p.choices().len()],
                None if a.requires_argument() => Vec::new(),
                None => vec![a.id],
            })
            .collect();
        let rows = with_nothing(|ctx| menu::rows(*m, ctx));
        let drawn: Vec<&str> = rows.iter().filter_map(|r| r.action.as_deref()).collect();
        assert_eq!(
            drawn,
            expected,
            "the `{}` menu drifted from its group",
            m.title()
        );
    }
}

/// The label and the shortcut come from the catalogue, character by character.
///
/// Two sources, because there are two kinds of row: a plain command reads
/// `Action::label`, and a row expanded from an argument reads the VALUE's
/// label — `Action::label` there is *"Toggle panel"*, the name of the
/// mechanism, which is not a row anyone clicks on purpose.
#[test]
fn the_label_and_the_shortcut_are_the_catalogue_ones() {
    for m in Menu::ALL {
        for row in with_nothing(|ctx| menu::rows(*m, ctx)) {
            let Some(id) = row.action.as_deref() else {
                continue;
            };
            let a = crawrad_actions::by_id(id).expect("the row came from the catalogue");
            match a.row_per_value() {
                Some(p) => {
                    let labels: Vec<&str> = p.choices().iter().map(|c| c.label).collect();
                    assert!(
                        labels.contains(&row.label.as_ref()),
                        "`{id}` drew the row `{}`, which is no value of `{}`",
                        row.label,
                        p.name
                    );
                    assert_eq!(
                        row.shortcut, None,
                        "`{id}` is six rows: a shortcut on each promises six keys                          that do not exist"
                    );
                }
                None => {
                    assert_eq!(
                        row.label.as_ref(),
                        a.label,
                        "`{id}` changed label on the way"
                    );
                    // THE SHORTCUT IS THE CATALOGUE'S — and the catalogue has
                    // two for some actions.
                    //
                    // `ATALHOS_PREFERIDOS` carries the chord everyone tries
                    // first for zoom (`Ctrl +`, `Ctrl -`, `Ctrl 0`), which the
                    // egui host owns and this one does not. This shell binds it
                    // and therefore this shell shows it — showing the other
                    // would be the menu teaching the worse chord.
                    //
                    // What the assertion still refuses is the row inventing a
                    // key: it has to be one of the two the catalogue declared.
                    let esperado = crawrad_actions::preferred_shortcut(id).or(a.shortcut);
                    assert_eq!(
                        row.shortcut.as_deref(),
                        esperado,
                        "`{id}` changed shortcut on the way"
                    );
                }
            }
        }
    }
}

/// An action that REQUIRES an argument the menu cannot supply never becomes a
/// row.
///
/// A menu has nowhere to type. Without this rule the `Edit` menu showed two
/// `Standard actions...` — the catalogue one, lit and inert, and the real one.
///
/// The exception is not a hole, it is the mechanism: an argument that publishes
/// a [`Domain`](crawrad_actions::Domain) with `row_per_value` is supplied by the
/// SHOWING — one row per value, each carrying its own. Every such row is
/// charged below for actually carrying it, which is what separates a row that
/// runs from the lit-and-inert one this test was born for.
#[test]
fn an_action_that_needs_an_argument_stays_out_of_the_menu() {
    let rows: Vec<menu::Row> = Menu::ALL
        .iter()
        .flat_map(|m| with_nothing(|ctx| menu::rows(*m, ctx)))
        .collect();
    let drawn: Vec<&str> = rows.iter().filter_map(|r| r.action.as_deref()).collect();
    for row in &rows {
        let Some(id) = row.action.as_deref() else {
            continue;
        };
        let a = crawrad_actions::by_id(id).expect("the row came from the catalogue");
        match a.row_per_value() {
            Some(p) => assert!(
                row.args.as_text(p.name).is_some_and(|v| !v.is_empty()),
                "`{id}` was expanded into rows and one of them carries no `{}`:                  that is the lit-and-inert row, back again",
                p.name
            ),
            None => assert!(
                !a.requires_argument(),
                "`{id}` requires an argument and a menu draws it: a lit row that does nothing"
            ),
        }
    }
    // The contrast — without it a filter returning an empty list would pass
    // this test with flying colours.
    assert!(
        drawn.contains(&"edit.select_all"),
        "the filter ate an action that takes no argument"
    );
    assert!(
        !drawn.contains(&"action.standard"),
        "`action.standard` requires classes and is still in the menu"
    );
}

/// What cannot run right now **stays**, and stays greyed.
///
/// It is the law the owner proved with the classic RAD screenshot: there `Deploy`,
/// `Dependencies` and nearly the whole `Run` menu while debugging are visible
/// and disabled. With no document open, `edit.copy` is exactly that case.
#[test]
fn what_cannot_run_is_drawn_muted_and_never_removed() {
    let rows = with_nothing(|ctx| menu::rows(Menu::Edit, ctx));
    let copy = rows
        .iter()
        .find(|r| r.action.as_deref() == Some("edit.copy"))
        .expect("`edit.copy` left the Edit menu instead of being greyed out");
    assert!(
        !copy.enabled,
        "`edit.copy` is lit with no document open — the catalogue predicate is \
         the one that decides, and it says no"
    );
    // With no document the whole Edit menu is grey, and that is CORRECT: there
    // is nothing to copy, paste or undo. The contrast is the next test, which
    // lights the SAME action as soon as there is a selection; that is what
    // proves the grey is not a hard-coded `false`.
    assert!(
        rows.iter().all(|r| !r.enabled),
        "some Edit action is lit with no document open"
    );
}

/// Every command on the top strip exists in the catalogue.
///
/// The strip is a CHOICE of four commands, and the choice lives in
/// `toolbar::Command`. What this test locks is the other half: renaming the
/// action in the catalogue used to leave the button lit and answering
/// `unknown action`, and the only way to find out was to click it.
#[test]
fn every_toolbar_command_exists_in_the_catalogue() {
    use super::toolbar::Command;
    for c in Command::ALL {
        let id = c.action();
        assert!(
            crawrad_actions::by_id(id).is_some(),
            "the `{c:?}` button points at `{id}`, which is not in the catalogue"
        );
    }
    let ids: std::collections::HashSet<&str> = Command::ALL.iter().map(|c| c.action()).collect();
    assert_eq!(
        ids.len(),
        Command::ALL.len(),
        "two toolbar buttons fire the same action"
    );
    let glyphs: std::collections::HashSet<&str> =
        Command::ALL.iter().map(|c| c.icon().glyph()).collect();
    assert_eq!(
        glyphs.len(),
        Command::ALL.len(),
        "two toolbar buttons draw the same icon"
    );
}

/// The state of a row is `Action::habilitada` applied to the context of NOW.
///
/// This is the contrast to the test above, with a document open: the SAME
/// action changes state once there is a selection, without a single `if` in
/// the shell.
#[test]
fn the_grey_is_the_catalogue_predicate_and_it_follows_the_selection() {
    let registry = Registry::builtin();
    let area = Clipboard::default();
    let mut ir = crawrad_ir::Document::new("form_main", "Form");
    ir.add_child("form_main", crawrad_ir::Node::new("btn", "Button"))
        .expect("the button goes into the form");
    let mut doc = crawrad_doc::Doc::new(ir);

    let state = |doc: &crawrad_doc::Doc| {
        let ctx = Context {
            doc: Some(doc),
            registry: &registry,
            clipboard: &area,
            args: Args::none(),
            project: Project::default(),
            author: Author::User,
            editor_dirty: false,
            text: Default::default(),
        };
        menu::rows(Menu::Edit, &ctx)
            .into_iter()
            .find(|r| r.action.as_deref() == Some("edit.copy"))
            .map(|r| r.enabled)
    };

    assert_eq!(
        state(&doc),
        Some(false),
        "with no selection, `Copy` has to be grey"
    );
    doc.select(vec!["btn".to_string()]);
    assert_eq!(
        state(&doc),
        Some(true),
        "with one component selected, `Copy` has to light up — and the one who \
         decides is the catalogue predicate, not the shell"
    );
}

/// The shell **writes the id of no COMMAND**.
///
/// It is the sibling of `a_dsl_da_toolbox_nao_escreve_nome_de_componente`, for
/// the same reason: an id written here is a second list of commands, and it
/// ages in silence.
///
/// # The boundary, and why it is exactly this one
///
/// A **command** is an action with no argument: it exists whole in the
/// catalogue, and whoever shows it — menu or strip — DERIVES it. None of those
/// ids may appear in this shell; the four on the top strip live in
/// `toolbar.rs`, a declared choice of four out of thirty-one with a test of
/// its own.
///
/// A **gesture bridge** is an action WITH arguments: `component.add` takes the
/// class and the drop coordinates, `component.set_property` takes node, key
/// and value from the Object Inspector. They cannot be derived from a group —
/// no menu draws them (`Action::pede_argumento`) — and whoever calls them is
/// precisely whoever knows how to collect the argument. Banning them here
/// would be banning the shell from talking to the catalogue.
/// ## What a REQUIRED parameter is, and is not
///
/// This used to exempt an action by `Action::pede_argumento` — whether the
/// catalogue declares some param as **required**. That coincided with the rule
/// above only by luck, and `page.new` is where the luck ran out: none of its
/// params is required (so the menu draws it, and with no argument it falls
/// back to the selection), but the `+` on the tab strip belongs to a
/// `PageControl` that **may not be the selected one** — the strip has an
/// argument to hand over, and is precisely the one who knows it.
///
/// So the exemption is measured where the rule actually lives: an id may be
/// written **only inside an `execute_with` that carries arguments**. The bare
/// `execute_action` is `execute_with` with `Args::nenhum()`, and THAT is the
/// command path — a literal id there is still a second list of commands.
#[test]
fn the_shell_writes_no_command_id() {
    // Whitespace out: `rustfmt` wraps the same call in one line or four
    // depending on how long the id is, and the shape must not decide a verdict.
    let compacto: String = SHELL.chars().filter(|c| !c.is_whitespace()).collect();
    let mut written = Vec::new();
    for a in crawrad_actions::catalogue() {
        if a.requires_argument() {
            continue;
        }
        let citado = format!("\"{}\"", a.id);
        let vezes = compacto.matches(&citado).count();
        if vezes == 0 {
            continue;
        }
        let ponte = format!("self.execute_with(cx,\"{}\",", a.id);
        let vazias = compacto
            .matches(&format!("{ponte}crawrad_actions::Args::none()"))
            .count();
        let pontes = compacto.matches(&ponte).count() - vazias;
        // EVERY mention of the id must be a gesture bridge. If even one is not,
        // the id becomes loose text in the shell again.
        if pontes < vezes {
            written.push(a.id);
        }
    }
    assert!(
        written.is_empty(),
        "the shell went back to writing command ids. A command comes from the \
         catalogue, through `menu::rows` and `toolbar::Command`:\n  {}",
        written.join("\n  ")
    );
    // The contrast: without it, a test that never looks at anything would pass
    // forever.
    assert!(
        SHELL.contains("\"component.add\""),
        "the gesture bridge left `ide_app.rs`; this test stopped reading the \
         right file"
    );
}

/// And it writes no action LABEL either.
///
/// The id is the address; the label is what is read on screen, and it is what
/// the user compares against the egui shell. Written in the DSL, it outlives
/// the action that produced it.
#[test]
fn the_shell_writes_no_action_label() {
    let mut written = Vec::new();
    for a in crawrad_actions::catalogue() {
        // Short one-word labels ("Run", "Close") show up in widget names and in
        // comments; what this test looks for is SCREEN text, which in the DSL
        // always arrives as `text: "..."`.
        if SHELL.contains(&format!("text: \"{}\"", a.label)) {
            written.push(a.label);
        }
    }
    assert!(
        written.is_empty(),
        "the shell wrote an action label into the DSL; it comes from \
         `Action::label`:\n  {}",
        written.join("\n  ")
    );
}

/// The menu TITLES are not written in the DSL either.
///
/// The six are written at runtime by `montar_cromo`, from `Menu::title()` —
/// which is also the `Action::grupo` each of them draws.
#[test]
fn the_dsl_does_not_write_the_menu_titles() {
    for m in Menu::ALL {
        assert!(
            !SHELL.contains(&format!(
                "<CrawradMenuButton> {{ text: \"{}\" }}",
                m.title()
            )),
            "the `{}` menu got its title written back into the DSL",
            m.title()
        );
    }
    assert_eq!(
        SHELL.matches("<CrawradMenuButton> { text: \"\" }").count(),
        Menu::ALL.len(),
        "the number of menu buttons in the DSL is not the number of `Menu::ALL`"
    );
}

/// An empty menu SAYS it is empty instead of opening a box with no height.
#[test]
fn a_menu_with_no_command_shows_the_notice() {
    // There is no empty group today — `no_menu_opens_empty` charges for that.
    // What is measured here is the SHAPE of the answer when there is one, and
    // it is a row that executes nothing.
    let notice = menu::Row {
        action: None,
        label: std::borrow::Cow::Borrowed(menu::NOTHING_HERE),
        shortcut: None,
        enabled: false,
        args: crawrad_actions::Args::none(),
        panel_state: None,
        separator: false,
        submenu: None,
        tone: menu::Tone::Command,
    };
    assert!(notice.action.is_none() && !notice.enabled);
    assert!(
        !menu::NOTHING_HERE.is_empty(),
        "the empty-menu sentence cannot itself be empty — the box would be blank"
    );
}

/// The widest menu in the catalogue fits the fixed width of the popup.
///
/// **The arithmetic, and what it is NOT:** text cannot be measured without a
/// font in hand, and opening a `Cx` in a unit test is not an option. What this
/// uses is a CEILING per character — 8 px for a 13 px body — which is generous
/// for IBM Plex Sans. If the catalogue gains a label that does not fit next to
/// its shortcut, this test fails before the screen clips the word.
#[test]
fn the_widest_menu_fits_the_popup() {
    /// The ceiling for one character, in logical pixels.
    const CEILING_PER_CHAR: f64 = 8.0;
    /// `CRAWRAD_MENU_LARGURA` minus the two `CRAWRAD_ESPACO_L` insets.
    const USABLE: f64 = 260.0 - 2.0 * 10.0;

    for m in Menu::ALL {
        for row in with_nothing(|ctx| menu::rows(*m, ctx)) {
            let chars = row.label.chars().count()
                + row.shortcut.as_ref().map_or(0, |s| s.chars().count() + 2);
            let width = chars as f64 * CEILING_PER_CHAR;
            assert!(
                width <= USABLE,
                "`{}` + `{}` asks for {width:.0} px and the menu has {USABLE:.0} px usable",
                row.label,
                row.shortcut.as_deref().unwrap_or("")
            );
        }
    }
}

/// The two numbers the popup arithmetic uses are the theme's.
///
/// They live twice — in the DSL, where the row and the inset are declared, and
/// in Rust, where the popup height is computed from the number of rows. The
/// DSL has no arithmetic over a count it does not know, so sharing is
/// impossible by construction and what is left is duplication a test holds
/// together — the same arrangement `tema.rs` and `ide_app/tema.rs` already
/// live under.
#[test]
fn the_menu_arithmetic_uses_the_theme_numbers() {
    const THEME: &str = super::tests::TEMA_COMPLETO;
    for (token, value) in [
        ("CRAWRAD_MENU_LINHA", "22.0"),
        ("CRAWRAD_ESPACO_S", "4.0"),
        ("CRAWRAD_MENU_LARGURA", "260.0"),
        ("CRAWRAD_MENU_SEPARADOR", "7.0"),
    ] {
        assert!(
            THEME.contains(&format!("pub {token} = {value}")),
            "`{token}` is no longer {value} in the theme, and the Rust side of \
             the menu arithmetic still assumes it is"
        );
    }
    assert!(
        SHELL.contains("const MENU_ROW_HEIGHT: f64 = 22.0;"),
        "MENU_ROW_HEIGHT drifted from CRAWRAD_MENU_LINHA"
    );
    assert!(
        SHELL.contains("const MENU_PADDING: f64 = 4.0;"),
        "MENU_PADDING drifted from the CRAWRAD_ESPACO_S the popup declares as padding"
    );
    assert!(
        SHELL.contains("const MENU_SEPARATOR_HEIGHT: f64 = 7.0;"),
        "MENU_SEPARATOR_HEIGHT drifted from CRAWRAD_MENU_SEPARADOR"
    );
}

// ---------------------------------------------------------------------------
// THE `View` MENU LISTS PANELS — defect C of 16/09/2026.
// ---------------------------------------------------------------------------

/// The `View` menu lists EVERY panel, in the order the catalogue declares them.
///
/// # The defect this locks down
///
/// The owner opened `View` and found `Reset Layout` alone. The cause was
/// measured and is one line: `rows` filtered out every action that requires an
/// argument, and the six panel commands all take `panel`. So the only door to
/// showing a panel that has no rail icon — `Design` and `Chat` — was the
/// protocol.
#[test]
fn the_view_menu_lists_every_panel() {
    let rows = with_nothing(|ctx| {
        menu::submenu_rows(
            Menu::View,
            "Panels",
            ctx,
            crawrad_actions::KeymapScheme::Modern,
        )
    });
    let listed: Vec<&str> = rows
        .iter()
        .filter_map(|r| r.panel_state)
        .map(|p| p.label())
        .collect();
    let expected: Vec<&str> = crawrad_actions::Panel::ALL
        .iter()
        .map(|p| p.label())
        .collect();
    assert_eq!(
        listed, expected,
        "the `Panels` submenu is no longer the list of panels. It is derived from \
         `Panel::ALL` through the domain of the `panel` argument — a panel \
         missing here means the domain stopped being derived."
    );
    // And the label on the row is the PANEL's, not the action's.
    for row in &rows {
        let Some(p) = row.panel_state else { continue };
        assert_eq!(row.label, p.label());
    }
}

/// Every panel row carries the argument that makes it run.
///
/// It is the whole difference between this and the lit-and-inert row: the row
/// was drawn knowing which panel it is about, and it carries it — instead of
/// the shell reading some state again at click time, which is the window where
/// the drawing and the running disagree.
#[test]
fn every_panel_row_carries_its_own_argument() {
    let rows = with_nothing(|ctx| {
        menu::submenu_rows(
            Menu::View,
            "Panels",
            ctx,
            crawrad_actions::KeymapScheme::Modern,
        )
    });
    for row in rows {
        let Some(p) = row.panel_state else { continue };
        assert_eq!(
            row.args.as_text("panel"),
            Some(p.id()),
            "the `{}` row carries no panel id",
            p.label()
        );
        assert_eq!(
            row.action.as_deref(),
            Some("layout.toggle_panel"),
            "a panel row stopped being a toggle"
        );
    }
}

/// A rule separates the sections of the View menu.
///
/// `Reset Layout` is preceded by a rule, and View's 4 sections are separated
/// by exactly 3 rules.
#[test]
fn a_rule_separates_the_sections_of_the_view_menu() {
    let rows = with_nothing(|ctx| menu::rows(Menu::View, ctx));
    let reset_idx = rows
        .iter()
        .position(|r| r.action.as_deref() == Some("layout.reset"))
        .expect("the View menu lists reset layout");
    assert!(
        rows[reset_idx - 1].separator,
        "nothing separates reset layout from what precedes it"
    );
    // A rule is not a command: it must never be clickable, and it must never
    // be the whole menu.
    for row in &rows {
        if row.separator {
            assert!(row.action.is_none() && !row.enabled);
        }
    }
    assert_eq!(
        rows.iter().filter(|r| r.separator).count(),
        3,
        "the View menu has 4 sections and must have exactly 3 separating rules"
    );
}

/// No menu starts or ends with a rule, and no two rules are adjacent.
#[test]
fn no_menu_starts_or_ends_with_a_rule_and_no_rules_are_adjacent() {
    for m in Menu::ALL {
        let rows = with_nothing(|ctx| menu::rows(*m, ctx));
        if rows.is_empty() {
            continue;
        }
        assert!(
            !rows.first().unwrap().separator,
            "the `{}` menu starts with a rule",
            m.title()
        );
        assert!(
            !rows.last().unwrap().separator,
            "the `{}` menu ends with a rule",
            m.title()
        );
        for pair in rows.windows(2) {
            assert!(
                !(pair[0].separator && pair[1].separator),
                "the `{}` menu has two adjacent rules",
                m.title()
            );
        }
    }
}

/// The tick comes from the DOCK, and the shell is the one that asks.
///
/// This test reads the shell as TEXT, which is the only oracle available
/// without a window — and what it charges is the SHAPE that makes the tick
/// honest: `painel_a_vista`, the same judgement that lights the rail, and not
/// a second notion of "open" kept beside the dock.
#[test]
fn the_tick_is_the_same_judgement_that_lights_the_rail() {
    assert!(
        SHELL.contains("match linha.panel_state {"),
        "the menu row stopped reading `panel_state`"
    );
    assert!(
        SHELL.contains("Some(p) if self.painel_a_vista(arranjo, p) =>"),
        "the tick of the `View` menu is no longer `painel_a_vista` — a second \
         notion of `open` beside the dock is the field that drifts the day the \
         user drags a tab"
    );
    assert!(
        SHELL.contains("icon::Chrome::Check.glyph()"),
        "the tick stopped coming from the one chrome table"
    );
}

#[test]
fn menu_contributions_injection_and_positioning() {
    use crawrad_extension_api::contribution::ActionContribution;
    use crawrad_extension_api::contribution::{MenuContribution, MenuPosition};
    use crawrad_extension_api::manifest::{ExtensionContributions, ExtensionManifest};

    let mut registry = crawrad_actions::capability::CapabilityRegistry::default();
    let manifest = ExtensionManifest {
        id: "vendor.tools".into(),
        name: "Custom Tools".into(),
        version: "1.0.0".into(),
        publisher: "Vendor".into(),
        description: "Custom menu actions".into(),
        contributions: ExtensionContributions {
            actions: vec![
                ActionContribution {
                    id: "tool.top".into(),
                    label: "Top Action".into(),
                    description: Some("Action at the top".into()),
                    shortcut: Some("Ctrl+1".into()),
                    icon: None,
                },
                ActionContribution {
                    id: "tool.bottom".into(),
                    label: "Bottom Action".into(),
                    description: Some("Action at the bottom".into()),
                    shortcut: Some("Ctrl+2".into()),
                    icon: None,
                },
                ActionContribution {
                    id: "tool.before_copy".into(),
                    label: "Before Copy Action".into(),
                    description: Some("Action before copy".into()),
                    shortcut: None,
                    icon: None,
                },
                ActionContribution {
                    id: "tool.after_copy".into(),
                    label: "After Copy Action".into(),
                    description: Some("Action after copy".into()),
                    shortcut: None,
                    icon: None,
                },
            ],
            menus: vec![
                MenuContribution {
                    target: "Edit".into(),
                    action_id: "tool.top".into(),
                    position: MenuPosition::Top,
                    group: None,
                },
                MenuContribution {
                    target: "Edit".into(),
                    action_id: "tool.bottom".into(),
                    position: MenuPosition::Bottom,
                    group: Some("CustomGroup".into()),
                },
                MenuContribution {
                    target: "Edit".into(),
                    action_id: "tool.before_copy".into(),
                    position: MenuPosition::Before("edit.copy".into()),
                    group: None,
                },
                MenuContribution {
                    target: "Edit".into(),
                    action_id: "tool.after_copy".into(),
                    position: MenuPosition::After("edit.copy".into()),
                    group: None,
                },
            ],
            cards: Vec::new(),
            tools: Vec::new(),
            templates: Vec::new(),
            views: Vec::new(),
        },
        entrypoint: None,
        activation_events: Vec::new(),
    };

    registry
        .register_extension_manifest(&manifest)
        .expect("manifest must be registered");

    let rows = with_nothing(|ctx| {
        menu::rows_with_scheme_and_contributions(
            Menu::Edit,
            ctx,
            crawrad_actions::KeymapScheme::Modern,
            Some(&registry),
            &manifest.contributions.menus,
        )
    });

    // 1. MenuPosition::Top must be in the first position
    assert_eq!(rows.first().unwrap().action.as_deref(), Some("tool.top"));
    assert_eq!(rows.first().unwrap().label.as_ref(), "Top Action");
    assert_eq!(rows.first().unwrap().shortcut.as_deref(), Some("Ctrl+1"));

    // 2. MenuPosition::Before("edit.copy") and After("edit.copy")
    let copy_idx = rows
        .iter()
        .position(|r| r.action.as_deref() == Some("edit.copy"))
        .expect("edit.copy must be in the Edit menu");
    assert!(copy_idx > 0);
    assert_eq!(
        rows[copy_idx - 1].action.as_deref(),
        Some("tool.before_copy"),
        "tool.before_copy must come immediately before edit.copy"
    );
    assert_eq!(
        rows[copy_idx + 1].action.as_deref(),
        Some("tool.after_copy"),
        "tool.after_copy must come immediately after edit.copy"
    );

    // 3. MenuPosition::Bottom must be at the end of the menu
    assert_eq!(rows.last().unwrap().action.as_deref(), Some("tool.bottom"));
    assert_eq!(rows.last().unwrap().label.as_ref(), "Bottom Action");
    assert_eq!(rows.last().unwrap().shortcut.as_deref(), Some("Ctrl+2"));

    // 4. Group adds a separator before the bottom item
    let bottom_idx = rows.len() - 1;
    assert!(
        rows[bottom_idx - 1].separator,
        "an item with group must be preceded by a separator if it is not the first"
    );
}

/// **The Edit menu lists ONE of two twins: the view's in front**.
/// With the designer in front, Copy is the components'; with the code in
/// front, the text's — never both, and never neither.
#[test]
fn the_edit_menu_lists_the_twin_of_the_view_in_front() {
    let registry = Registry::builtin();
    let area = Clipboard::default();
    let ids = |code_in_front: bool| {
        let ctx = Context {
            doc: None,
            registry: &registry,
            clipboard: &area,
            args: Args::none(),
            project: Project::default(),
            author: Author::User,
            editor_dirty: false,
            text: crawrad_actions::EditorText {
                code_in_front,
                ..Default::default()
            },
        };
        menu::rows(Menu::Edit, &ctx)
            .iter()
            .filter_map(|r| r.action.as_deref().map(str::to_owned))
            .collect::<Vec<_>>()
    };
    let design = ids(false);
    let code = ids(true);
    for (text, components) in [
        ("editor.cut", "edit.cut"),
        ("editor.copy", "edit.copy"),
        ("editor.paste", "edit.paste"),
        ("editor.select_all", "edit.select_all"),
    ] {
        assert!(
            design.iter().any(|i| i == components),
            "{components} missing in the designer"
        );
        assert!(
            !design.iter().any(|i| i == text),
            "{text} listed in the designer"
        );
        assert!(
            code.iter().any(|i| i == text),
            "{text} missing with the code in front"
        );
        assert!(
            !code.iter().any(|i| i == components),
            "{components} listed with the code in front"
        );
    }
    // an action without a twin is listed in both views
    assert!(design.iter().any(|i| i == "edit.undo") && code.iter().any(|i| i == "edit.undo"));
}

/// **Cut and Copy are muted with no text selected, Paste with no text on the
/// clipboard** — drawn, never removed.
#[test]
fn text_cut_copy_and_paste_are_muted_without_their_input() {
    let registry = Registry::builtin();
    let area = Clipboard::default();
    let lit = |selection: bool, clipboard: bool| {
        let ctx = Context {
            doc: None,
            registry: &registry,
            clipboard: &area,
            args: Args::none(),
            project: Project::default(),
            author: Author::User,
            editor_dirty: false,
            text: crawrad_actions::EditorText {
                selection,
                clipboard,
                code_in_front: true,
                unit_has_form: false,
            },
        };
        menu::rows(Menu::Edit, &ctx)
            .iter()
            .filter_map(|r| Some((r.action.as_deref()?.to_owned(), r.enabled)))
            .collect::<std::collections::BTreeMap<_, _>>()
    };
    let none = lit(false, false);
    let both = lit(true, true);
    for id in ["editor.cut", "editor.copy", "editor.paste"] {
        assert_eq!(none.get(id), Some(&false), "{id} lit with nothing to take");
        assert_eq!(both.get(id), Some(&true), "{id} muted with its input there");
    }
    assert_eq!(lit(true, false).get("editor.paste"), Some(&false));
    assert_eq!(lit(false, true).get("editor.copy"), Some(&false));
    assert_eq!(none.get("editor.select_all"), Some(&true));
}

fn editor_menu_rows(
    unit_has_form: bool,
    selection: bool,
    scheme: crawrad_actions::KeymapScheme,
) -> Vec<menu::Row> {
    let registry = Registry::builtin();
    let area = Clipboard::default();
    let ctx = Context {
        doc: None,
        registry: &registry,
        clipboard: &area,
        args: Args::none(),
        project: Project::default(),
        author: Author::User,
        editor_dirty: false,
        text: crawrad_actions::EditorText {
            code_in_front: true,
            unit_has_form,
            selection,
            ..Default::default()
        },
    };
    menu::editor_context_rows(&ctx, scheme)
}

fn editor_menu(
    unit_has_form: bool,
    scheme: crawrad_actions::KeymapScheme,
) -> Vec<(String, String)> {
    editor_menu_rows(unit_has_form, false, scheme)
        .iter()
        .map(|r| {
            let id = r.action.as_deref().unwrap_or("-").to_owned();
            (id, r.shortcut.as_deref().unwrap_or("").to_owned())
        })
        .collect()
}

/// **The editor's context menu is exactly its groups, in their fixed order**
/// Navigation, View, Refactor, AI, Clipboard, Selection, Command
/// Palette — no row for an action that does not exist, F12 only in a form's
/// unit.
#[test]
fn the_editor_menu_is_its_groups_in_order() {
    let vs = crawrad_actions::KeymapScheme::Modern;
    let ids: Vec<String> = editor_menu(true, vs).into_iter().map(|(i, _)| i).collect();
    assert_eq!(
        ids,
        [
            "editor.goto_definition",
            "editor.find_references",
            "editor.show_hover",
            "-",
            "view.toggle_code",
            "editor.toggle_inlay_hints",
            "-",
            "editor.rename_symbol",
            "editor.format_document",
            "-",
            "ai.add_file_to_chat",
            "ai.add_selection_to_chat",
            "ai.explain_selection",
            "ai.review_selection",
            "-",
            "tools.sidecar_note",
            "-",
            "editor.cut",
            "editor.copy",
            "editor.paste",
            "-",
            "editor.select_all",
            "-",
            "view.command_palette",
        ]
    );
    let without_form: Vec<String> = editor_menu(false, vs).into_iter().map(|(i, _)| i).collect();
    assert!(!without_form.iter().any(|i| i == "view.toggle_code"));
    for id in &without_form {
        if id != "-" {
            assert!(
                crawrad_actions::by_id(id).is_some(),
                "{id} is not in the catalog"
            );
        }
    }
}

#[test]
fn the_editor_menu_ai_rows_follow_the_selection() {
    let vs = crawrad_actions::KeymapScheme::Modern;
    let row_enabled = |rows: &[menu::Row], id: &str| {
        rows.iter()
            .find(|r| r.action.as_deref() == Some(id))
            .map(|r| r.enabled)
    };

    let without = editor_menu_rows(false, false, vs);
    assert_eq!(row_enabled(&without, "ai.add_file_to_chat"), Some(true));
    assert_eq!(
        row_enabled(&without, "ai.add_selection_to_chat"),
        Some(false)
    );
    assert_eq!(row_enabled(&without, "ai.explain_selection"), Some(false));
    assert_eq!(row_enabled(&without, "ai.review_selection"), Some(false));

    let with = editor_menu_rows(false, true, vs);
    assert_eq!(row_enabled(&with, "ai.add_file_to_chat"), Some(true));
    assert_eq!(row_enabled(&with, "ai.add_selection_to_chat"), Some(true));
    assert_eq!(row_enabled(&with, "ai.explain_selection"), Some(true));
    assert_eq!(row_enabled(&with, "ai.review_selection"), Some(true));
}

/// **The shortcut column is the active scheme's**: switching to the classic
/// scheme switches the keys; a modern editor's F2 (an editor key) shows for Rename.
#[test]
fn the_editor_menu_shows_the_active_schemes_keys() {
    let key = |scheme, id: &str| {
        editor_menu(true, scheme)
            .into_iter()
            .find(|(i, _)| i == id)
            .map(|(_, k)| k)
            .unwrap()
    };
    let vs = crawrad_actions::KeymapScheme::Modern;
    let classic = *crawrad_actions::KeymapScheme::ALL
        .iter()
        .find(|s| **s != vs)
        .unwrap();
    assert_eq!(key(vs, "editor.rename_symbol"), "F2");
    assert_eq!(key(classic, "editor.rename_symbol"), "Ctrl+Shift+E");
    assert_eq!(key(vs, "editor.find_references"), "Shift+Alt+F12");
    assert_eq!(key(classic, "editor.find_references"), "Ctrl+Shift+Enter");
    assert_eq!(key(vs, "editor.copy"), "Ctrl+C");
}

/// **The code action list:** one row per action, in the server's
/// order; each runs `editor.apply_code_action` with its index; the
/// preferred one is starred.
#[test]
fn the_code_action_rows_apply_by_index() {
    let rows = crawrad_actions::menu::code_action_rows(
        &[
            ("Import `std::collections::HashMap`".to_string(), true),
            ("Qualify as `std::collections::HashMap`".to_string(), false),
        ],
        false,
    );
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].label, "\u{2605} Import `std::collections::HashMap`");
    assert_eq!(rows[1].label, "Qualify as `std::collections::HashMap`");
    for (i, r) in rows.iter().enumerate() {
        assert_eq!(r.action.as_deref(), Some("editor.apply_code_action"));
        assert_eq!(r.args.as_number("index"), Some(i as f64));
        assert!(r.enabled && !r.separator);
    }
    assert!(crawrad_actions::by_id("editor.apply_code_action").is_some());
}

/// **The teacher in the quick fixes:** a line with a compiler error
/// ends the list with "Fix and Explain" after a rule, and alone when the
/// server offers nothing; without an error the list is the server's only.
#[test]
fn a_compiler_error_adds_fix_and_explain_to_the_quick_fixes() {
    let one = [("Borrow here".to_string(), true)];
    let rows = crawrad_actions::menu::code_action_rows(&one, true);
    assert_eq!(rows.len(), 3);
    assert!(rows[1].separator);
    assert_eq!(rows[2].action.as_deref(), Some("chat.fix_explain"));
    let alone = crawrad_actions::menu::code_action_rows(&[], true);
    assert_eq!(alone.len(), 1);
    assert_eq!(alone[0].action.as_deref(), Some("chat.fix_explain"));
    assert!(crawrad_actions::menu::code_action_rows(&one, false).len() == 1);
    assert!(crawrad_actions::by_id("chat.fix_explain").is_some());
}
