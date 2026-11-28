//! Code generator for the **SLINT** target.
//!
//! Emits native Rust applications with declarative Slint UI markup.

pub mod components;
pub mod form;
pub mod layout;
pub mod main_app;
pub mod stubs;

#[cfg(test)]
#[path = "../../test/slint/test.rs"]
mod tests;

use std::path::PathBuf;
use std::sync::LazyLock;

use crate::{Arquivo, Dono, Erro, Gerador, SUFIXO_GERADO, identificador};
use crawrad_ir::{Document, Node};

pub static CLASSES: LazyLock<Vec<&'static str>> = LazyLock::new(components::classes);

macro_rules! declare_target {
    ($method:ident) => {
        fn $method(&self) -> &'static str {
            "slint"
        }
    };
}

pub struct SlintGenerator;
pub use SlintGenerator as GeradorSlint;

impl Gerador for SlintGenerator {
    declare_target!(alvo);

    fn classes(&self) -> &'static [&'static str] {
        CLASSES.as_slice()
    }

    fn generate_with_bases(
        &self,
        // CrawRAD UI document
        doc: &Document,
        // Crate name
        _nome_do_crate: &str,
        // Component registry
        reg: &crawrad_registry::Registry,
        // Base documents
        bases: &[Document],
        // Base indicator
        is_base: bool,
    ) -> Result<Vec<Arquivo>, Erro> {
        // Resolve parent base document if visual inheritance is present.
        let base_doc = crate::resolve_bases(bases);
        let base_ref = base_doc.as_ref();

        // Extract the root form node from document tree.
        let root = doc.node(&doc.root).ok_or(Erro::SemRaiz)?;
        crawrad_registry::class_property::check(doc, reg)?;

        // Slint target sanitized form and type names.
        let form_name = identificador(root.name())?;
        let form_type = crate::tipo_do_form(&form_name);

        // Filter and collect component nodes supported in the Slint catalog.
        let emitted: Vec<&Node> = doc
            .in_paint_order()
            .iter()
            .filter_map(|id| doc.node(id))
            .filter(|n| {
                let effective = reg.effective_class(&n.class);
                CLASSES.contains(&effective)
            })
            .filter(|n| {
                if let Some(base_doc) = base_ref {
                    !doc.is_inherited(&n.id, base_doc)
                } else {
                    true
                }
            })
            .collect();

        // Build the output files required for Slint forms.
        // Single source of truth: markup is compiled via inlined `slint::slint! { ... }` in designer.rs.
        let main_file = Arquivo {
            caminho: PathBuf::from("src/main.rs"),
            conteudo: main_app::main_rs(&form_name, &form_type),
            dono: Dono::Gerador,
            adicoes: Vec::new(),
        };

        let designer_rs_file = Arquivo {
            caminho: PathBuf::from(format!("src/{form_name}{SUFIXO_GERADO}")),
            conteudo: form::form_designer_rs_with_base(
                doc, reg, root, &form_type, &emitted, bases, is_base,
            )?,
            dono: Dono::Gerador,
            adicoes: Vec::new(),
        };

        let user_stubs_file = stubs::stubs_with_base(
            reg,
            &form_name,
            &form_type,
            root,
            &crate::events::handler_nodes(doc, &emitted, base_ref),
            base_ref.map(|b| (b, doc)),
        )?;

        Ok(vec![main_file, designer_rs_file, user_stubs_file])
    }
}
