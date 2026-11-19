//! **Every name the catalogue hands out is a Rust identifier in snake_case.**
//!
//! A component is born with the name [`ComponentDesc::base_name`] gives it plus
//! the first free number — `button1`, `page_control2`. That name becomes a
//! field (`pub button1: Button`) and a method in the generated project, so a
//! catalogue that hands out `Button1` hands out a `non_snake_case` warning in a
//! file the user did not write and cannot fix: the `.designer.rs` is rewritten
//! whole on the next generation.
//!
//! # Why this gate exists at all, as of 16/09/2026
//!
//! Until that date nothing declared the base name — it was **guessed** from
//! `display` by a case conversion at run time, `para_snake_case("PageControl")`.
//! The guess was right, and that is exactly what made it dangerous: it was the
//! single piece keeping every component's name valid, it lived twice (character
//! for character, in `crawrad-codegen` and here), and it was invisible from the
//! catalogue — the place anyone would look.
//!
//! The owner ordered the translation out of the IDE: *"here it is Rust, not
//! a classic RAD IDE (…) it has to be written the way Rust allows."* Removing it naively
//! would have made components be born `Button1` — precisely the classic RAD name he
//! does not want. So the catalogue declares instead of guessing, and this gate
//! charges the declaration.
//!
//! # Why it reads the catalogue instead of listing classes
//!
//! Because a list written here goes stale the day someone adds a component, and
//! the one that goes stale is the one nobody tested. Reading
//! [`Registry::builtin`] whole means **a new class is charged on its own**, and
//! this file never finds out. Same design as the class dispatch gate in
//! `crawrad-doc`.

use crate::Registry;

/// The same ruler `rustc` applies to a field: ASCII lowercase, digits and
/// underscore, and it does not start with a digit.
///
/// It is asked of `crawrad_ir`, and not written again here, because it is the
/// very rule `Doc::apply` uses to refuse a rename. Two copies of it would be
/// two ways for the catalogue and the entrance to disagree about the same
/// word — with the catalogue handing out a name its own IDE would refuse.
fn is_valid(name: &str) -> bool {
    crawrad_ir::is_snake_case_name(name)
}

#[test]
fn every_base_name_in_the_catalogue_is_snake_case() {
    let registry = Registry::builtin();
    let mut checked = 0usize;
    let mut complaints = Vec::new();

    for desc in registry.all() {
        checked += 1;
        if !is_valid(desc.base_name) {
            complaints.push(format!(
                "`{}` declares base_name `{}`, which is not a snake_case identifier",
                desc.class, desc.base_name
            ));
        }
    }

    assert!(
        checked > 15,
        "only {checked} classes read: the catalogue shrank, or the gate is measuring nothing"
    );
    assert!(
        complaints.is_empty(),
        "the catalogue hands out a name `rustc` would warn about ({} complaint(s)):\n  {}",
        complaints.len(),
        complaints.join("\n  ")
    );
}

/// The name is charged where it is actually **used**, and not only where it is
/// declared: `suggest_id` is what the palette, the canvas and the agent call.
///
/// Without this half, a `suggest_id` that stopped reading `base_name` — going
/// back to deriving it, or falling back to the class — would leave the test
/// above green while every component was born wrong again.
#[test]
fn every_name_suggest_id_hands_out_is_snake_case() {
    let registry = Registry::builtin();
    let mut checked = 0usize;
    let mut complaints = Vec::new();

    for desc in registry.all() {
        // Three in a row: the suffix is part of the name, and `button10` has to
        // be as valid as `button1`.
        let mut taken: Vec<String> = Vec::new();
        for _ in 0..3 {
            let id = registry.suggest_id(desc.class, |i| taken.iter().any(|t| t == i));
            checked += 1;
            if !is_valid(&id) {
                complaints.push(format!("`{}` suggested `{id}`", desc.class));
            }
            taken.push(id);
        }
    }

    assert!(checked > 45, "only {checked} names measured");
    assert!(
        complaints.is_empty(),
        "`suggest_id` hands out a name `rustc` would warn about ({} complaint(s)):\n  {}",
        complaints.len(),
        complaints.join("\n  ")
    );
}

/// The class outside the catalogue does not get a guessed name either.
///
/// It used to fall back to `para_snake_case(class)`, which made the conversion
/// load-bearing for a path that is already a caller's bug. The answer is not a
/// prettier guess: it is a name that is honest about what it knows, and that
/// `rustc` will not warn about.
#[test]
fn an_unknown_class_still_gets_a_valid_name() {
    let registry = Registry::builtin();
    let id = registry.suggest_id("NoSuchClass", |_| false);
    assert!(
        is_valid(&id),
        "an unknown class produced `{id}`, which the compiler would warn about"
    );
    assert_eq!(id, "component1");
}

#[test]
fn every_standard_action_has_snake_case_base_name() {
    let actions = crate::standard_actions();
    let mut complaints = Vec::new();

    for act in &actions {
        if !is_valid(act.base_name) {
            complaints.push(format!(
                "action `{}` declares base_name `{}`, which is not snake_case",
                act.name, act.base_name
            ));
        }
    }

    assert!(!actions.is_empty(), "standard_actions returned empty list");
    assert!(
        complaints.is_empty(),
        "standard actions have invalid base_name ({} complaint(s)):\n  {}",
        complaints.len(),
        complaints.join("\n  ")
    );
}
