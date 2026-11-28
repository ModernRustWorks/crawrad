//! The framework catalogue.

use super::{Availability, CATALOGUE, ROWS, badge, of, resolve_framework};
use crawrad_codegen::AlvoGerador;

/// Every generator target has its entry, and the dialog has a row for each.
#[test]
fn every_target_is_listed_and_fits_the_dialog() {
    for &t in AlvoGerador::TODOS {
        assert_eq!(of(t).target, Some(t));
    }
    assert!(
        CATALOGUE.len() <= ROWS,
        "add rows to the dialog (np_fw_N) and to ROWS"
    );
}

/// A ready entry has a generator; a planned one has none and says so.
#[test]
fn availability_matches_the_generator() {
    for f in CATALOGUE {
        match f.availability {
            Availability::Ready => {
                assert!(f.target.is_some(), "{}", f.name);
                assert_eq!(badge(f), "");
            }
            Availability::Planned => {
                assert!(f.target.is_none(), "{}", f.name);
                assert_eq!(badge(f), "planned");
            }
        }
    }
    let names: std::collections::BTreeSet<_> = CATALOGUE.iter().map(|f| f.name).collect();
    assert_eq!(names.len(), CATALOGUE.len(), "names are unique");
}

#[test]
fn resolve_framework_maps_defaults_and_known_names() {
    assert_eq!(resolve_framework(None), Ok(AlvoGerador::Makepad));
    assert_eq!(resolve_framework(Some("")), Ok(AlvoGerador::Makepad));
    assert_eq!(resolve_framework(Some("makepad")), Ok(AlvoGerador::Makepad));
    assert_eq!(resolve_framework(Some("Makepad")), Ok(AlvoGerador::Makepad));
    assert_eq!(resolve_framework(Some("egui")), Ok(AlvoGerador::Egui));
    assert_eq!(resolve_framework(Some("Egui")), Ok(AlvoGerador::Egui));
    assert!(resolve_framework(Some("unknown_fw")).is_err());
}

/// Slint is on the list, selectable, and does not claim a platform it cannot reach.
///
/// The dialog draws `CATALOGUE` and nothing else, so a framework the generator
/// supports but the catalogue omits is a framework nobody can pick: `of()` would
/// panic on it. That was the single red on the Slint branch.
#[test]
fn slint_is_offered_by_the_dialog_and_claims_only_what_it_reaches() {
    let slint = of(AlvoGerador::Slint);
    assert_eq!(slint.name, "Slint");
    assert_eq!(slint.availability, Availability::Ready);
    assert_eq!(slint.crate_dep, "slint");
    assert_eq!(resolve_framework(Some("slint")), Ok(AlvoGerador::Slint));
    assert_eq!(resolve_framework(Some("Slint")), Ok(AlvoGerador::Slint));

    // The platforms LINE is what a person reads and the two flags are what the
    // code decides with: they must agree, or the dialog offers a template that
    // refuses itself.
    assert!(!slint.web, "the scaffold still emits Makepad for wasm_app");
    assert!(
        !slint.mobile,
        "the scaffold still emits Makepad for mobile_app"
    );
    assert!(!slint.platforms.contains("Web"), "{}", slint.platforms);
    assert!(!slint.platforms.contains("Mobile"), "{}", slint.platforms);
}
