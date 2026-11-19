//! **Every `Enum` default is one of its own options**, for every class
//! of the catalogue.
//!
//! The BitBtn's `Kind` was born `"Custom"` among `bkCustom … bkNo`: the Object
//! Inspector opened on a value its own list does not have, and the generator
//! read a value no target knows. One sweep keeps the whole catalogue honest.

use crate::{PropKind, Registry};
use crawrad_ir::Value;

#[test]
fn every_enum_default_is_one_of_its_options() {
    let r = Registry::builtin();
    let mut wrong = Vec::new();
    for desc in r.all() {
        for p in &desc.props {
            let options: &[&str] = match p.kind {
                PropKind::Enum(o) if !o.is_empty() => o,
                _ if !p.options.is_empty() => p.options,
                _ => continue,
            };
            let default = match &p.default {
                Value::Enum(s) | Value::Str(s) => s.as_str(),
                _ => continue,
            };
            if !options.contains(&default) {
                wrong.push(format!(
                    "{}.{} = {default:?}, options {options:?}",
                    desc.class, p.key
                ));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "an Enum default outside its own options:\n  {}",
        wrong.join("\n  ")
    );
}
