//! **THE DOOR A THIRD PARTY COMES IN BY** — and the first place it can be wrong.
//!
//! # Why the door refuses instead of accepting
//!
//! `Registry::register` takes a `ComponentDesc` and inserts it, no questions
//! asked. That is right for the built-ins: they are literals the compiler
//! checked, and a wrong one is a wrong one in this repository.
//!
//! A component from outside is different. It is written by someone who never
//! saw this code, and it arrives while the IDE is running. The worst thing the
//! catalogue can do with it is take it — because a class whose property keys
//! break the contract of [ADR 0002] **poisons every document that uses it**:
//! the form saves, and `Document::validate` refuses it on the way back in.
//!
//! The refusal is the feature. `Component ▸ Install…` has to be able to say
//! *"this component declares `Foo.bar`, and `Foo` is not a runtime this IDE
//! knows"* — with the component's name on it — instead of installing something
//! that breaks a form three gestures later.
//!
//! # Validate BEFORE leaking
//!
//! [`ExternalComponent::into_catalogue`] leaks every string it carries (see
//! [`crate::external`]). So the checks here run on the **owned** value, and only
//! a component that passed all of them is converted. A refusal costs nothing.
//!

use crawrad_ir::contract::{PropKey, TARGETS};

use crate::external::ExternalComponent;
use crate::{Registry, UNIVERSAL_TARGET};
use crawrad_note::note;

#[note("registration-registrationerror")]
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RegistrationError {
    #[error(
        "'{0}' is not a usable class name: it has to start with an upper-case \
         ASCII letter and continue in letters and digits, because it becomes a \
         Rust type in the generated project"
    )]
    BadClass(String),

    #[error(
        "'{0}' is not a usable name prefix: it has to start with a lower-case \
         ASCII letter and continue in lower-case letters, digits or '_', \
         because it becomes a field name in the generated project"
    )]
    BadPrefix(String),

    #[error("the class '{0}' is already in the catalogue")]
    ClassTaken(String),

    #[error("the class '{class}' declares the property '{key}' twice")]
    DuplicateProp { class: String, key: String },

    #[error("the class '{class}' declares the event '{name}' twice")]
    DuplicateEvent { class: String, name: String },

    #[error("the class '{class}' declares the property '{key}': {why}")]
    BadPropKey {
        class: String,
        key: String,
        why: String,
    },

    #[error(
        "the class '{class}' claims the runtime '{target}', which this IDE does not know (known: {known})"
    )]
    UnknownTarget {
        class: String,
        target: String,
        known: String,
    },

    #[error("the class '{0}' declares no property at all — not even a name")]
    NoProps(String),

    #[error("the class '{class}' inherits from '{base}', which is not in the catalogue")]
    UnknownBase { class: String, base: String },

    #[error(
        "the class '{class}' inherits from '{base}', which is external, and only native components can be inherited"
    )]
    ExternalBase { class: String, base: String },

    #[error("the class '{class}' declares '{key}', which clashes with the base class")]
    ClashesWithBase { class: String, key: String },
}

#[note("registration-usable-class")]
fn usable_class(s: &str) -> bool {
    let mut cs = s.chars();
    match cs.next() {
        Some(c) if c.is_ascii_uppercase() => {}
        _ => return false,
    }
    cs.all(|c| c.is_ascii_alphanumeric())
}

#[note("registration-usable-prefix")]
fn usable_prefix(s: &str) -> bool {
    let mut cs = s.chars();
    match cs.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => return false,
    }
    cs.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// Validates base component inheritance and checks for member clashes.
fn validate_base(
    registry: &Registry,
    c: &ExternalComponent,
) -> Result<Option<crate::ComponentDesc>, RegistrationError> {
    let Some(base_class) = &c.inherits else {
        return Ok(None);
    };
    let Some(base) = registry.get(base_class) else {
        return Err(RegistrationError::UnknownBase {
            class: c.class.clone(),
            base: base_class.clone(),
        });
    };
    // Native means "shipped with the IDE": a manifest-only external package also has an
    // empty `crate_module`, so that field cannot tell the two apart.
    if Registry::builtin().get(base_class).is_none() {
        return Err(RegistrationError::ExternalBase {
            class: c.class.clone(),
            base: base_class.clone(),
        });
    }
    for p in &c.props {
        if base.props.iter().any(|bp| bp.key == p.key) {
            return Err(RegistrationError::ClashesWithBase {
                class: c.class.clone(),
                key: p.key.clone(),
            });
        }
    }
    for e in &c.events {
        if base.events.iter().any(|be| be.name == e.name) {
            return Err(RegistrationError::ClashesWithBase {
                class: c.class.clone(),
                key: e.name.clone(),
            });
        }
    }
    Ok(Some(base.clone()))
}

impl Registry {
    #[note("registration-registry-register-external")]
    pub fn register_external(&mut self, c: ExternalComponent) -> Result<(), RegistrationError> {
        if !usable_class(&c.class) {
            return Err(RegistrationError::BadClass(c.class));
        }
        if !usable_prefix(&c.prefix) {
            return Err(RegistrationError::BadPrefix(c.prefix));
        }
        // AN ALREADY TAKEN CLASS is rejected instead of overwritten, and the difference
        // matters: `register` overwrites, which is right for
        // `builtin()` reloading itself. A third-party component that
        // overwrote `Button` would swap the IDE's entire palette without a
        // single word on screen.
        if self.get(&c.class).is_some() {
            return Err(RegistrationError::ClassTaken(c.class));
        }

        let base_desc = validate_base(self, &c)?;
        if base_desc.is_none() && c.props.is_empty() {
            return Err(RegistrationError::NoProps(c.class));
        }

        let mut seen: Vec<&str> = Vec::new();
        for p in &c.props {
            if seen.contains(&p.key.as_str()) {
                return Err(RegistrationError::DuplicateProp {
                    class: c.class.clone(),
                    key: p.key.clone(),
                });
            }
            seen.push(&p.key);
            // **THE ADR 0002 CONTRACT APPLIES TO WHAT COMES FROM OUTSIDE AS WELL.**
            // Without this, a component with key `Foo.bar` installs, appears in the
            // palette, and the first form using it is rejected by
            // `Document::validate` upon reopening — far from here, and without
            // the culprit's name.
            if let Err(e) = PropKey::parse(&p.key) {
                return Err(RegistrationError::BadPropKey {
                    class: c.class.clone(),
                    key: p.key.clone(),
                    why: e.to_string(),
                });
            }
        }

        let mut events: Vec<&str> = Vec::new();
        for e in &c.events {
            if events.contains(&e.name.as_str()) {
                return Err(RegistrationError::DuplicateEvent {
                    class: c.class.clone(),
                    name: e.name.clone(),
                });
            }
            events.push(&e.name);
        }

        for t in &c.targets {
            if t != UNIVERSAL_TARGET && !TARGETS.contains(&t.as_str()) {
                return Err(RegistrationError::UnknownTarget {
                    class: c.class.clone(),
                    target: t.clone(),
                    known: TARGETS.join(", "),
                });
            }
        }

        // ONLY NOW does the leak happen — see the header.
        let mut desc = c.into_catalogue();
        if let Some(base) = base_desc {
            desc.default_size = base.default_size;
            desc.visual = base.visual;
            desc.derived_geometry = base.derived_geometry;
            desc.text_line = base.text_line;
            let mut props = base.props;
            props.extend(desc.props);
            desc.props = props;

            let mut events = base.events;
            events.extend(desc.events);
            desc.events = events;
        }
        self.register(desc);
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/registration/test.rs"]
mod test;
