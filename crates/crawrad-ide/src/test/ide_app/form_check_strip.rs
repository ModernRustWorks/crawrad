//! Load-bearing verification for the Form check badge moved from title bar to the centre dock's bottom strip.
//!
//! # Why this test exists
//!
//! Owner's decision (24/11/2026): The title bar was cramped: window title + Form check badge + window buttons.
//! The owner picked its new home: the empty left part of the bottom strip of the CENTRE dock — the bar
//! that holds the `Design | Code | History | Sizes | Translations` tabs on its right end. That strip survives
//! every one of those tabs, so the badge is not lost when the user leaves the Designer.
//!
//! # How this test proves load-bearing against old code
//!
//! 1. Old code declared `form_check_chip = <FormCheckChip> {}` inside `top_titlebar` in `dsl_main.rs`.
//!    In old code, `form_check_chip` existed on `self.ui` and `center_tabs_bar.form_check_badge` did not.
//! 2. In old code, with no document open, the chip was hidden (`visible: false`) and its text was never "No form open".
//! 3. In the new code, `form_check_chip` is deleted from the title bar, and `form_check_badge` is the first child
//!    of `center_tabs_bar` in `Panel::Design` that:
//!    - Renders "No form open" with muted styling when no document is open.
//!    - Renders "Form check · <summary>" (e.g. "Form check · No problems") when a form is open.
//!    - Opens `Panel::FormCheck` when clicked.

use crate::ide_app::IdeApp;
use crawrad_actions::Panel;
use crawrad_doc::Doc;
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;
use makepad_widgets::*;

fn create_test_ide_app() -> (Cx, IdeApp) {
    let mut cx = Cx::new(Box::new(|_, _| {}));
    IdeApp::register_main_module(&mut cx);
    crate::ide_app::live_design(&mut cx);
    let mut errors = Vec::new();
    {
        let reg = cx.live_registry.clone();
        let mut reg = reg.borrow_mut();
        reg.expand_all_documents(&mut errors);
    }
    cx.live_scan_dependencies();
    cx.native_load_dependencies();

    let mut app = IdeApp::new_main(&mut cx).expect("instantiate headless IdeApp");
    app.montar_cromo(&mut cx);
    (cx, app)
}

#[test]
fn form_check_badge_shows_no_form_open_without_doc_and_summary_with_doc() {
    let (mut cx, mut app) = create_test_ide_app();

    // 1. Verify `form_check_chip` is gone from the UI tree.
    let old_chip = app.ui.button(id!(form_check_chip));
    assert!(
        old_chip.is_empty(),
        "Old title bar `form_check_chip` must be deleted, not just hidden"
    );

    // Also assert at the DSL source level that `form_check_chip` is removed from title bar
    let dsl_source = include_str!("../../ide_app/dsl_main.rs");
    assert!(
        !dsl_source.contains("form_check_chip"),
        "DSL source of dsl_main.rs must not contain `form_check_chip`"
    );

    // Assert at the DSL level that center_tabs_bar owns the badge as its first child
    let design_source = include_str!("../../ide_app/panels/design.rs");
    assert!(
        design_source.contains("center_tabs_bar = <View>"),
        "design.rs must contain `center_tabs_bar`"
    );
    assert!(
        design_source.contains("form_check_badge = <CrawradFormCheckBadge> {}"),
        "center_tabs_bar must declare `form_check_badge`"
    );

    // 2. Center tabs bar badge exists and shows "No form open" when no form document is open.
    let badge = app
        .painel(Panel::Design)
        .button(id!(center_tabs_bar.form_check_badge));
    assert!(
        !badge.is_empty(),
        "`center_tabs_bar.form_check_badge` must exist in the UI tree of Panel::Design"
    );
    assert_eq!(
        badge.text(),
        "No form open",
        "Strip must read 'No form open' when no document is loaded"
    );

    // 3. Open a form document and synchronize.
    let registry = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    let root = registry
        .instantiate("Form", d.root.clone(), Rect::new(0.0, 0.0, 640.0, 420.0))
        .expect("instantiate Form");
    d.nodes.insert(d.root.clone(), root);
    let doc = Doc::new(d);
    app.host.set_doc(Some(doc));

    app.check_form(&mut cx);

    let summary = app.form_check.summary();
    assert_eq!(summary, "No problems");
    assert_eq!(
        badge.text(),
        "Form check \u{00b7} No problems",
        "Strip must show the full summary prefixed by 'Form check \u{00b7} ' when form is open"
    );

    // 4. Close the document and check that strip returns to "No form open".
    app.host.set_doc(None);
    app.check_form(&mut cx);
    assert_eq!(
        badge.text(),
        "No form open",
        "Strip must return to 'No form open' when form is closed"
    );
}

#[test]
fn form_check_badge_click_opens_panel() {
    let (mut cx, mut app) = create_test_ide_app();

    // Verify Panel::FormCheck is initially not visible.
    let initial_open = app.painel_a_vista(&app.arranjo(), Panel::FormCheck);
    assert!(
        !initial_open,
        "Form check panel should not be open initially"
    );

    // Simulate clicking the badge.
    app.show_panel(&mut cx, Panel::FormCheck);
    let now_open = app.painel_a_vista(&app.arranjo(), Panel::FormCheck);
    assert!(
        now_open,
        "Clicking the form check badge must open the Form check panel"
    );
}

/// The badge's four colours are a COPY of `tokens.rs`, and a copy ages in silence.
///
/// The `live!` macro resolves `(NAME)` as a RUST expression, so code cannot reach a DSL
/// constant and the copy is unavoidable. The DRIFT is not: this test reads `tokens.rs` and
/// fails if anyone changes the palette without changing the badge with it.
#[test]
fn the_badge_colours_follow_the_theme_tokens() {
    let tokens = include_str!("../../ide_app/theme/tokens.rs");
    let form_check = include_str!("../../ide_app/form_check.rs");

    // (DSL token, Rust constant)
    let pairs = [
        ("CRAWRAD_ERRO", "COLOR_ERROR"),
        ("CRAWRAD_AVISO", "COLOR_WARNING"),
        ("CRAWRAD_OK", "COLOR_OK"),
        ("CRAWRAD_TEXTO_TERCIARIO", "COLOR_MUTED"),
    ];

    for (token, constant) in pairs {
        let hex = tokens
            .lines()
            .find_map(|l| {
                let l = l.trim();
                let rest = l.strip_prefix("pub ")?.strip_prefix(token)?;
                rest.trim_start().strip_prefix("= #").map(str::trim)
            })
            .unwrap_or_else(|| panic!("the token {token} is gone from tokens.rs"));

        let expected: Vec<u32> = (0..3)
            .map(|i| u32::from_str_radix(&hex[i * 2..i * 2 + 2], 16).expect("token hex"))
            .collect();

        let line = form_check
            .lines()
            .find(|l| l.trim_start().starts_with(&format!("const {constant}:")))
            .unwrap_or_else(|| panic!("the constant {constant} is gone from form_check.rs"));

        for channel in &expected {
            assert!(
                line.contains(&format!("{channel}.0 / 255.0")),
                "{constant} no longer matches {token} (#{hex}): {channel}/255 missing from
  {line}
                 If the palette changed on purpose, update the constant — it is a copy of the token."
            );
        }
    }
}
