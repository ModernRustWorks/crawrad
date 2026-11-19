//! **THE PROPERTY BELONGS TO THE CLASS** — and the catalog is what knows this.
//!
//! # The question this module answers, and who answered it before
//!
//! *"Can the key `url` exist in this document?"*
//!
//! Until 17/09/2026 the one answering was a list of forty names written by hand
//! in `crawrad-ir` — `contract::COMMON`. [ADR 0031 Amendment 1] stripped that
//! role from it, in the owner's words: *"if we are going to focus on the component, why
//! do we need this common one? That is already duplication"*.
//!
//! The component **already declares** its properties. The list was a second
//! copy, maintained by hand — and forgetting it already cost dearly:
//! `Panel.border_width` was born in the palette with a property that `validate()`
//! rejected, and **every form with a panel failed**.
//!
//! # And the ruler became STRONGER, not weaker
//!
//! The list was global: it knew that `interval` was a legal key, but not whose.
//! Measured in `validate()` before:
//!
//! ```text
//! a `Button` with the property `interval` — which belongs to `Timer` — VALIDATED.
//! ```
//!
//! Asking the catalog by CLASS, it is rejected with the right sentence. It is
//! what ADR 0031 had already predicted: *"any property guard needs
//! the key to become (class, key), or the loophole remains"*.
//!
//! # Why in the CATALOG, and not in `crawrad-ir` nor `crawrad-doc`
//!
//! `crawrad-ir` is the lower layer and **does not see the catalog** — nor should it. What
//! stays there is what is judged without any catalog: malformed keys and unknown
//! targets.
//!
//! `crawrad-doc` was the first attempt, and it was wrong: it depends on the
//! catalog only as a `dev-dependency`, **for gates**, and its job is
//! change — invertible commands, transactions, undo, auditing —, not *what a
//! component is*.
//!
//! Here is the right place and costs no dependencies: **who knows whether a
//! class declares a property is the catalog**, and it already sees
//! `Document`.
//!
//! # The prefix rule, and it is what preserves legacy
//!
//! | key | who authorizes it |
//! |---|---|
//! | **without prefix** — `url` | the CLASS must declare it |
//! | **with prefix** — `win32.ShowRoot` | it is enough for the target to be known |
//!
//! The second row is what the contract itself promises in writing: *"runtimes
//! that do not know it must preserve it in the file and ignore it at runtime"*.
//! It is what keeps standing the eight `win32.*` keys that the example
//! `form_main.crawform` still carries — the legacy runtime died in ADR 0013 and
//! no class declares them.
//!

use crate::Registry;
use crawrad_ir::Document;
use crawrad_ir::contract::PropKey;
use crawrad_note::note;

#[note("class-property-classerror")]
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ClassError {
    #[error(
        "the node '{node}' is of class '{class}', which is not installed: \
         install the component that provides it, or remove the node"
    )]
    UnknownClass { node: String, class: String },

    #[error("the class '{class}' has no property '{key}' (node '{node}')")]
    UndeclaredProperty {
        node: String,
        class: String,
        key: String,
    },

    /// An event key the class does not declare. The IDE's own door
    /// (`event.bind`) already refuses it; this is what stops a document that
    /// came by another path — edited by hand, written by an agent — from
    /// getting a stub for an event that does not exist.
    #[error("the class '{class}' has no event '{event}' (node '{node}')")]
    UndeclaredEvent {
        node: String,
        class: String,
        event: String,
    },
}

#[note("class-property-check")]
pub fn check(doc: &Document, reg: &Registry) -> Result<(), ClassError> {
    for (id, node) in doc.nodes.iter() {
        let Some(desc) = reg.get(&node.class) else {
            // THE CLASS THAT IS NOT INSTALLED has its own sentence, and it is what
            // the user needs: an uninstalled component leaves forms
            // that used it without an owner, and *"invalid property key"* sends
            // them looking in the wrong place.
            return Err(ClassError::UnknownClass {
                node: id.to_string(),
                class: node.class.clone(),
            });
        };
        for key in node.props.keys() {
            // A key with a target prefix passes without the class declaring it — see
            // the header. The target was already checked by `Document::validate`.
            if !matches!(PropKey::parse(key), Ok(PropKey::Common(_))) {
                continue;
            }
            if desc.prop(key).is_none() {
                return Err(ClassError::UndeclaredProperty {
                    node: id.to_string(),
                    class: node.class.clone(),
                    key: key.clone(),
                });
            }
        }
        // The event belongs to the class too, by the same rule.
        for event in node.events.keys() {
            if !desc.events.iter().any(|e| e.name == event) {
                return Err(ClassError::UndeclaredEvent {
                    node: id.to_string(),
                    class: node.class.clone(),
                    event: event.clone(),
                });
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/class_property/test.rs"]
mod test;
