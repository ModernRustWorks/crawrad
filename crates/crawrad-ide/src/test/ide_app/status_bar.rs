//! The gate over the foot of the window.
//!
//! # The failure this file exists to catch
//!
//! A status bar cell has the worst failure mode in this shell: it draws
//! nowhere and nothing complains. `WidgetRef::label` on a path the DSL does not
//! have hands back an empty widget, `set_text` on it is a no-op, and the
//! compiler never sees a problem. The bar simply has one fewer number than the
//! code believes it has.
//!
//! So the two lists that make a cell — [`SLOTS`], which says where each value
//! goes, and [`ICONS`], which says which glyph sits beside it — are charged
//! against the DSL by name here.

use super::{ICONS, NAO_MEDIDO, SLOTS};
use crate::ide_app::icon;

/// The shell's DSL, as text.
///
/// The `live_design!` block of `ide_app.rs` — the only place a widget name can
/// be declared.
const DSL: &str = concat!(
    include_str!("../../ide_app.rs"),
    include_str!("../../ide_app/dsl_main.rs"),
);

/// The status bar module, as text.
const FONTE: &str = include_str!("../../ide_app/status_bar.rs");

/// Every `id!(bottom_status_bar.….nome)` written in the module, as the dotted
/// path it is.
///
/// # Why the names are read from the SOURCE and not from the `LiveId`
///
/// Because a `LiveId` cannot say its own name here. It is a hash, and the
/// spelling only comes back through the interned table the `live_design!`
/// registration fills — which happens when the app starts, not in a unit test.
/// Measured: `to_string()` on these answers `a2379b009181f38f`.
///
/// So the gate reads the text, which is the same oracle every other DSL gate
/// in this shell uses, and for the same reason.
fn caminhos_escritos() -> Vec<String> {
    let mut out = Vec::new();
    let mut resto = FONTE;
    while let Some(i) = resto.find("id!(bottom_status_bar.") {
        let dentro = &resto[i + "id!(".len()..];
        let f = dentro
            .find(')')
            .expect("an unclosed `id!(` would not compile");
        out.push(dentro[..f].to_string());
        resto = &dentro[f..];
    }
    out
}

/// The last segment of a dotted path — the widget's own name in the DSL.
fn nome(caminho: &str) -> &str {
    caminho.rsplit('.').next().unwrap_or(caminho)
}

/// **Every cell the bar fills has a widget with that name in the DSL.**
///
/// The mutation this refuses: renaming `lbl_nodes` in the DSL, or writing a
/// slot here and forgetting the widget. Either way the value is computed every
/// frame and painted nowhere.
#[test]
fn every_status_slot_is_declared_in_the_dsl() {
    let caminhos = caminhos_escritos();
    assert_eq!(
        caminhos.len(),
        SLOTS.len() + ICONS.len(),
        "the number of `id!(bottom_status_bar.…)` written in the module no longer matches the two lists. A path outside them is written by no one, and one fewer is a cell that does not draw: {caminhos:?}"
    );
    for caminho in &caminhos {
        let n = nome(caminho);
        assert!(
            DSL.contains(&format!("{n} = <")),
            "the status bar writes `{caminho}` and the DSL does not declare any `{n}`. The value is computed every frame and painted nowhere — `set_text` on an empty `WidgetRef` is a no-op that reports nothing"
        );
        // And the PARENT also exists: a wrong `cell_nodes` with a correct `lbl_nodes`
        // would still not draw, and the leaf name would not reveal it.
        let pai = caminho
            .rsplit_once('.')
            .map(|(p, _)| p)
            .and_then(|p| p.rsplit('.').next())
            .unwrap_or("bottom_status_bar");
        assert!(
            DSL.contains(&format!("{pai} = <")),
            "cell `{pai}` of `{caminho}` does not exist in the DSL"
        );
    }
}

/// **No two cells write to the same place.**
///
/// The mistake this catches is the one made by copying the line above and
/// forgetting to change the name: two values into one label means the second
/// wins and the first is simply gone from the screen — and the count in
/// [`every_status_slot_is_declared_in_the_dsl`] would still add up.
#[test]
fn no_two_cells_share_a_slot() {
    let caminhos = caminhos_escritos();
    for (i, a) in caminhos.iter().enumerate() {
        for b in &caminhos[i + 1..] {
            assert_ne!(a, b, "two status bar cells write to the same label");
        }
    }
}

/// **The icons are the ones the catalogue names, and no glyph is written here.**
///
/// ADR 0026: the code point lives in one place. A `"\u{ea87}"` written in this
/// module would be the second place, and the two would be free to drift.
#[test]
fn the_icons_come_from_the_catalogue() {
    let papeis: Vec<icon::Chrome> = ICONS.iter().map(|(_, p)| *p).collect();
    assert!(
        papeis.contains(&icon::Chrome::Error) && papeis.contains(&icon::Chrome::Warning),
        "the problems cell lost its error or warning drawing: {papeis:?}"
    );
    let fonte = include_str!("../../ide_app/status_bar.rs");
    assert!(
        !fonte.contains("\\u{"),
        "a glyph was written into the status bar module. The code point lives \
         in `icon::Chrome` and nowhere else (ADR 0026)"
    );
}

/// **The target cell is short, and it never says the dialog's words.**
///
/// `AlvoGerador::rotulo` answers `Makepad (Nativo)` and `Egui / Eframe` — the
/// right words for the dialog that asks, and far too many for a cell 26 pixels
/// tall. The mutation this refuses is reaching for `rotulo()` here, which
/// would look correct and blow the cell across half the bar.
#[test]
fn the_target_cell_is_one_word() {
    for alvo in crawrad_codegen::AlvoGerador::TODOS {
        let curto = alvo.alvo_curto();
        assert!(
            !curto.contains(' ') && curto.len() <= 8,
            "`{curto}` is not a one-word name for a status bar cell"
        );
        assert_ne!(
            curto,
            alvo.rotulo(),
            "the target cell went back to the dialog's label"
        );
    }
    // And the two targets do not say the same thing.
    assert_ne!(
        crawrad_codegen::AlvoGerador::Makepad.alvo_curto(),
        crawrad_codegen::AlvoGerador::Egui.alvo_curto()
    );
}

/// **Nothing was measured yet says `—`, never `0`.**
///
/// Zero errors is a **verdict**, and claiming one before the compiler has
/// spoken is exactly the plausible lie this bar had to be cleaned of once
/// already — it used to read `no selection    19 nodes    rev 0` over a file
/// the shell had never opened.
#[test]
fn nothing_measured_is_a_dash_and_not_a_zero() {
    assert_eq!(NAO_MEDIDO, "—");
    assert_ne!(NAO_MEDIDO, "0");
}

/// **The framework cell names Makepad, and it sits at the right end of the bar.**
#[test]
fn the_framework_cell_names_makepad_at_the_end_of_the_bar() {
    assert_eq!(super::IDE_FRAMEWORK, "Makepad");
    let caminhos = caminhos_escritos();
    assert_eq!(
        caminhos.get(super::SLOTS.len() - 1).map(|s| s.as_str()),
        Some("bottom_status_bar.cell_framework.pill.lbl_framework")
    );
}
