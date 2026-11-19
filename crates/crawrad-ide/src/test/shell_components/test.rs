//! **What the shell SAYS about the component suite.**
//!
//! The message is the product here: the entire installation happens off-screen,
//! and what reaches the user is a single line. A line that does not say what happened
//! is the same as having no door.

use super::*;

/// **The summary counts BOTH sides.**
///
/// This repository has components in `components/`, and the test runs with the
/// working directory in the crate — so the number may be zero. What it
/// enforces is the FORM: that the phrase mentions packages and components, and that
/// it does not invent a refusal that did not happen.
#[test]
fn o_resumo_fala_de_pacotes_e_de_componentes() {
    let f = crawrad_project::installed::summary();
    assert!(
        f.contains("package(s) installed"),
        "the phrase does not count packages: {f}"
    );
    assert!(
        f.contains("component(s)"),
        "the phrase does not count components: {f}"
    );
    let i = crawrad_project::installed::ide_catalog();
    assert_eq!(
        f.contains("REFUSED"),
        !i.refusals.is_empty(),
        "the phrase invented — or hid — a refusal: {f}"
    );
}

/// **Without a refusal, opening says nothing.**
///
/// A bar that speaks when there is nothing to say is a bar no one reads
/// when there is.
#[test]
fn o_aviso_de_abertura_so_existe_quando_ha_recusa() {
    let i = crawrad_project::installed::ide_catalog();
    let aviso = crawrad_project::installed::refusal_notice();
    assert_eq!(
        aviso.is_some(),
        !i.refusals.is_empty(),
        "the notice and refusals disagree: {aviso:?} vs {:?}",
        i.refusals
    );
    if let Some(a) = aviso {
        for r in &i.refusals {
            assert!(a.contains(r), "the notice swallowed refusal `{r}`: {a}");
        }
    }
}

/// **Both catalogue actions reach the shell.**
///
/// The `Efeito` coverage gate already enforces the FIELD; this enforces the tip:
/// that `component.install` and `component.installed` exist with the id that the menu
/// and agent use, and that the `Component` menu actually renders them.
#[test]
fn o_menu_component_desenha_as_duas_acoes() {
    let ids: Vec<&str> = crawrad_actions::by_group("Component")
        .iter()
        .map(|a| a.id)
        .collect();
    for esperado in ["component.install", "component.installed"] {
        assert!(
            ids.contains(&esperado),
            "`{esperado}` left the group: {ids:?}"
        );
    }

    // And the bar menu points to the same group. A title that does not match
    // the group would open an empty menu, without saying why.
    assert_eq!(super::super::menu::Menu::Component.title(), "Component");
    assert!(
        super::super::menu::Menu::ALL.contains(&super::super::menu::Menu::Component),
        "the menu exists in the enum and is not in the bar"
    );
}

/// **The picker title states what it wants.**
///
/// *"Pick the component package folder"*, not *"Open"*: what is selected is the
/// package folder — the one containing `manifest.json` —, and a generic picker
/// would cause the user to select the parent folder on the first attempt.
#[test]
fn selector_requests_package_folder() {
    assert!(
        ESCOLHER_PACOTE.contains("package folder"),
        "{ESCOLHER_PACOTE}"
    );
}
