//! The gate of the ACTIVITY RAIL: what it offers, and what a click on it runs.
//!
//! # What changed under it, and why this file was rewritten
//!
//! The rail used to switch a nailed-down region: one destination at a time
//! filled `left_dock`, and `mostrar_doca` flipped the visibility of three
//! views. [ADR 0014, Emenda 9] took the owner away from the region — every
//! panel is a dockable tab now — so the assertions about `dock_components`,
//! `dock_inspector` and `visible: false` stopped describing anything. They
//! were not relaxed: they were **replaced** by the ones in
//! `test/ide_app/layout.rs`, which charge the dock itself.
//!
//! # The two levels it charges
//!
//! 1. **The list of panels** is a pure function over an enum of the
//!    catalogue, and it is measured item by item — order, label, icon, and
//!    which one has no screen yet.
//! 2. **The wiring**, by the technique of `fios.rs`: the shell is read as
//!    TEXT, so that deleting the call costs a red. A perfect [`RAIL`] nobody
//!    draws passes every unit test.
//!
//! What it does **not** prove: that the column comes out right on screen.
//! That is screen, and screen is checked by looking.

use crawrad_actions::Panel;
use crawrad_registry::Registry;

use super::rail::{self, RAIL};

/// The Makepad shell — the whole file, as text.
use super::casca::CASCA;

/// The theme of the shell, also as text: the dock measurements come from there.
const TEMA: &str = super::tests::TEMA_COMPLETO;

/// `CASCA` with no whitespace at all — immune to whatever `rustfmt` picks.
fn compacto() -> String {
    CASCA.chars().filter(|c| !c.is_whitespace()).collect()
}

/// The value of a numeric token of the theme.
fn theme_number(token: &str) -> f64 {
    let agulha = format!("pub {token} = ");
    let i = TEMA
        .find(&agulha)
        .unwrap_or_else(|| panic!("token `{token}` disappeared from src/ide_app/tema.rs"));
    let resto = &TEMA[i + agulha.len()..];
    let fim = resto
        .find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-'))
        .unwrap_or(resto.len());
    resto[..fim]
        .parse()
        .unwrap_or_else(|_| panic!("`{token}` is not a number"))
}

#[test]
fn rail_opens_in_owner_numbered_order() {
    assert_eq!(
        RAIL,
        &[
            Panel::Explorer,
            Panel::Components,
            Panel::Inspector,
            Panel::Packages,
            // The approved mockup puts the flask right before Run and Debug.
            Panel::Tests,
            Panel::Debug,
            Panel::Extensions
        ],
        "rail order changed. It was specified by the owner looking at the screen in \
         08/09/2026 — Explorer, Components, Inspector (`main.rs:313`) —, and the \
         package store comes after the panels, which is where its button is \
         on the egui rail (`main.rs:5734`). Run and Debug (issue #276) enters \
         last, without touching the four numbered ones. The extensions' views          (#398) come after it, leaving every earlier place where it was."
    );
}

#[test]
fn rail_is_catalog_subset_not_second_list() {
    // `RAIL` is a SUBSET of `Panel::ALL`, and whatever is not in it must
    // state so via its own `role`. Divergence would mean the panel list exists in
    // two places — the exact defect Amendment 9 exists to prevent repeating.
    for p in Panel::ALL {
        assert_eq!(
            RAIL.contains(p),
            crawrad_project::icons::rail_role(*p).is_some(),
            "`{}` is on the rail and has no icon role, or vice versa",
            p.id()
        );
    }
    assert!(
        !RAIL.contains(&Panel::Design) && !RAIL.contains(&Panel::Chat),
        "`Design` or `Chat` got a button on the rail. `Design` is what the \
         rail exists to UNCOVER, and `Chat` starts on the right (Amendment 1) \
         and goes wherever the user drops it (Amendment 9)."
    );
}

#[test]
fn each_rail_panel_has_own_label_and_icon() {
    let rotulos: std::collections::BTreeSet<&str> = RAIL.iter().map(|p| p.label()).collect();
    assert_eq!(
        rotulos.len(),
        RAIL.len(),
        "two rail panels have the same label"
    );
    let icones: std::collections::BTreeSet<&str> = RAIL.iter().map(|p| rail::glyph(*p)).collect();
    assert_eq!(
        icones.len(),
        RAIL.len(),
        "two rail panels have the same icon. They need to be \
         distinguishable at 24 px — and a rail with two identical drawings is a \
         rail that cannot be read."
    );
    for p in RAIL {
        let icone = rail::glyph(*p);
        assert_eq!(
            icone.chars().count(),
            1,
            "icon for `{}` has more than one character. The label of an \
             icon button has ONE glyph: the icon font lacks space, and what \
             is left is an empty box.",
            p.label()
        );
        let cp = icone.chars().next().unwrap() as u32;
        // The CODICONS range is MEASURED, not assumed: reading the `cmap` of
        // `assets/fontes/codicon.ttf` gives 639 codepoints, the smallest `U+EA60` and the
        // largest `U+ECE7`. The sibling test (`src/test/icones/test.rs`) reads the same
        // table and checks glyph by glyph; here what is enforced is that nobody
        // writes a codepoint from another family, which would render as tofu.
        assert!(
            (0xea60..=0xece7).contains(&cp),
            "icon for `{}` (U+{cp:04X}) is outside the Codicons range. Outside \
             it, the text font draws the glyph — and what comes out is tofu.",
            p.label()
        );
    }
}

/// **Every rail destination opens a real panel.**
///
/// This test used to do the opposite: it required that EXACTLY TWO destinations —
/// `Explorer` and `Package Manager` — had no screen, and that both
/// explained why in a shared placeholder panel. Both gained
/// screens in this round, and with that went `pending_reason`,
/// `PanelPending`, and the dimmed rail template.
///
/// What replaced it is the rule that mechanism served, and it is
/// stronger than the count previously here: **no rail button leads to a
/// blank panel**. `template_name` and reading the DSL text are what
/// enforce this — a new destination without its own template does not compile, and a
/// template not declared by the DSL fails here.
///
/// The regression it catches: pointing `Panel::Explorer` back to a placeholder
/// template, or declaring a template with an incorrect name in the DSL.
#[test]
fn every_rail_destination_opens_real_panel() {
    let c = compacto();
    let mut moldes = std::collections::BTreeSet::new();
    for p in RAIL {
        let nome = p.template_name();
        assert!(
            c.contains(&format!("{nome}=<")),
            "rail destination `{}` points to template `{nome}`, which the DSL \
             does not declare. `item_or_create` would return `None` and the tab would start \
             EMPTY without anything crashing (`dock.rs:1076-1085`) — a button leading to a \
             blank panel looks broken.",
            p.label()
        );
        assert!(
            moldes.insert(nome.clone()),
            "two rail destinations share template `{nome}`. One \
             would draw the other, and whoever clicked the first would see the second."
        );
    }
    // And the placeholder mechanism MUST NOT have been left behind: a pending
    // panel template requested by no tab is dead DSL, and dead DSL reads like a promise.
    assert!(
        !c.contains("PanelPending=<"),
        "`PanelPending` remains in the DSL and no panel requests it. When \
         a new destination is added without a screen, rewrite the twenty \
         lines — do not keep an empty template waiting."
    );
}

/// The list of destinations exists in EXACTLY ONE place — the catalog enum.
///
/// The regression this test catches is returning to the previous setup: five buttons
/// hardcoded in the DSL (`rail_files`, `rail_toolbox`, `rail_search`,
/// `rail_debug`), which were the destination list in a second place
/// **and were never read by a single line of Rust**.
#[test]
fn destination_list_is_not_hardcoded_in_dsl() {
    let c = compacto();
    for morto in ["rail_files", "rail_toolbox", "rail_search", "rail_debug"] {
        // The needle includes `=<`, and not out of fussiness: the four names are CITED in
        // the comment explaining why they were removed, and comments do not declare
        // any widget.
        assert!(
            !c.contains(&format!("{morto}=<")),
            "`{morto}` returned to the DSL. The panel list lives in \
             `crawrad_actions::Panel`, and a row of buttons in the DSL is the same \
             list in two places — with the second born outdated."
        );
    }
    assert!(
        c.contains("rail_list=<PortalList>"),
        "the rail is no longer a code-fed list"
    );
    assert!(
        c.contains("Destino=<CrawradRailButton>"),
        "rail button template disappeared from the DSL"
    );
    // And the DIMMED template did not return on its own: it only makes sense while
    // there are destinations without a screen, and today there are none — see the top of
    // `ide_app/rail.rs`.
    assert!(
        !c.contains("DestinoSemFio=<"),
        "dimmed rail template returned to the DSL with no destination to use it"
    );
}

/// **Wire** — the rail is drawn from the enum, and clicking runs the ACTION.
#[test]
fn rail_is_drawn_and_listened() {
    let c = compacto();
    assert!(
        c.contains("draw_rows!{cx,proximo,trilho,visible_rail,live_id!(Destino),"),
        "the rail stopped being drawn from `rail::RAIL`: buttons \
         would disappear from the screen and the IDE would have no gateway to palette, tree, and \
         properties"
    );
    assert!(
        c.contains("item.set_text(cx,rail::glyph(painel))"),
        "panel icon stopped going to the button"
    );
    assert!(
        c.contains("visible_rail.get(i).copied()"),
        "rail clicks stopped resolving to panels: buttons became silent \
         decorations"
    );
}

/// **Wire, and the most important one here** — clicking the rail runs an ACTION.
///
/// The owner's rule, 2026-09-15: *"everything in the IDE must have actions, so that the list
/// can later be used as tools for MCP"*. A rail directly manipulating
/// the dock would mean the mouse pathway works while the agent pathway does not exist
/// — and the two would diverge on the first bug fix.
#[test]
fn click_on_rail_executes_catalog_action() {
    let c = compacto();
    assert!(
        c.contains(r#""layout.toggle_panel","#),
        "the rail stopped executing `layout.toggle_panel`. Without this, opening \
         Inspector can only be done with the mouse — and the action list \
         ceases to be the MCP surface (ADR 0005)."
    );
    assert!(
        c.contains(r#"Args::text("panel",painel.id())"#),
        "panel id stopped being passed as action argument: command would run \
         without knowing which panel"
    );
    assert!(
        crawrad_actions::by_id("layout.toggle_panel").is_some(),
        "action `layout.toggle_panel` disappeared from catalog, and rail calls \
         it by id — a nonexistent id makes a mute button"
    );
}

/// **Wire** — the gear is not a panel, and it OPENS options.
///
/// It was once a button that only printed (*"Settings is not wired…"*) and that message
/// was removed along with the reason for it. What this test preserves are
/// the two halves: `Settings` remains not a rail panel — a rail item
/// is a panel over the FORM, whereas Settings is a window over the IDE —, and the
/// gear remains wired.
#[test]
fn gear_is_not_panel_and_opens_options() {
    let c = compacto();
    assert!(
        c.contains("(id!(rail_settings),icon::Chrome::Settings)"),
        "the gear lost its icon: it starts with blank text in the DSL"
    );
    assert!(
        c.contains("self.open_options(cx);"),
        "the gear became a mute button again: it stopped opening \
         `Options`, and a button accepting gesture without doing anything is the \
         defect family this shell removed from the screen"
    );
    assert!(
        !RAIL.iter().any(|p| p.label().contains("Settings")),
        "`Settings` became a rail panel. It was there before and removed: \
         ruler item is a panel over the open form, whereas Settings is a window."
    );
}

/// The dock has enough width for the widest label the catalog can request.
///
/// **The calculation, and what it is NOT:** text cannot be measured without a loaded font, and
/// opening a `Cx` in a unit test is not an option. What it uses is an UPPER BOUND
/// per character — 8 px for a 13 px body —, which is generous for IBM
/// Plex Sans. If the catalog acquires a label that does not fit, this test fails
/// before the screen truncates the word.
#[test]
fn dock_fits_widest_catalog_label() {
    /// Upper bound of character width, in logical pixels.
    const TETO_POR_CARACTERE: f64 = 8.0;

    let r = Registry::builtin();
    let mais_largo = r
        .all()
        .flat_map(|d| {
            d.props
                .iter()
                .map(|p| p.display)
                .chain(d.events.iter().map(|e| e.display))
        })
        .max_by_key(|s| s.chars().count())
        .expect("catalog has no properties and no events");

    // Since the splitter the name column is its own width, and the one
    // it is born with is the one that has to hold the label; the DSL token
    // and the Rust rest of the splitter are one number, checked here too.
    let sobra = theme_number("CRAWRAD_NAME_COLUMN");
    assert_eq!(
        sobra,
        crawrad_actions::inspector_model::NAME_WIDTH_DEFAULT,
        "(CRAWRAD_NAME_COLUMN) and NAME_WIDTH_DEFAULT drifted apart — the first \
         frame would draw one width and the splitter rest at another"
    );
    let pedido = mais_largo.chars().count() as f64 * TETO_POR_CARACTERE;

    assert!(
        pedido <= sobra,
        "the widest label in the catalog (`{mais_largo}`, {} characters) requires \
         up to {pedido} px and the name column has {sobra} px. Either shorten \
         the label or increase (CRAWRAD_DOCA_LARGURA) — the screen must not \
         truncate the property name in half.",
        mais_largo.chars().count()
    );
}
