//! Code generator for the **NATIVE MAKEPAD** target (100% Makepad).
//!
//! Emits Rust applications that depend exclusively on `makepad-widgets` and compile
//! directly with native shaders without any dependency on `eframe` or `egui`.

pub mod action_link;
pub mod actions;
pub mod anchors;
pub mod binding;
pub mod components;
pub mod control_events;
pub mod cursor;
pub mod declarado;
pub mod drawing;
pub mod form_hint;
/// **Bar geometry, read from the SAME file the designer canvas reads.**
///
/// Not a crate dependency — it is the path, just like the
/// `include_str!` through which the egui target reaches `desenho_comum.rs`. The
/// difference is `include!` instead of `include_str!`: here the generator needs the VALUES
/// to calculate the number hardcoded into the application, not the text.
///
/// Before this, `16.0` and `32.0` were raw literals scattered eight times
/// across `emit_dsl`, `emit_sync`, and `emit_actions` in `ScrollBar`, and the canvas
/// had its own copy in a `const` in another crate. Changing one side caused the
/// designer and the application to silently disagree.
pub mod shared_geometry {
    include!("../../../../crawrad-runtime-makepad/src/shared_geometry.rs");
}
pub mod form;
pub use form::form_designer_rs;
pub mod form_events;
pub mod glyph;
pub mod layout;
pub mod look;
pub mod main_app;
pub mod menus;
pub mod state;
pub mod stubs;
pub mod tab_order;

#[cfg(test)]
#[path = "../../test/targets/makepad/test.rs"]
mod tests;

use std::path::PathBuf;

use crate::{Arquivo, Dono, Erro, Gerador, SUFIXO_GERADO, identificador};
use crawrad_ir::{Document, Node};
use crawrad_note::note;

#[note("targets-makepad-classes")]
pub static CLASSES: std::sync::LazyLock<Vec<&'static str>> =
    std::sync::LazyLock::new(components::classes);

pub struct GeradorMakepad;

impl Gerador for GeradorMakepad {
    fn alvo(&self) -> &'static str {
        "makepad"
    }

    fn classes(&self) -> &'static [&'static str] {
        CLASSES.as_slice()
    }

    fn generate_with_bases(
        &self,
        doc: &Document,
        _nome_do_crate: &str,
        reg: &crawrad_registry::Registry,
        bases: &[Document],
        is_base: bool,
    ) -> Result<Vec<Arquivo>, Erro> {
        let base = crate::resolve_bases(bases);
        let base = base.as_ref();
        let raiz = doc.node(&doc.root).ok_or(Erro::SemRaiz)?;
        // Every property and event against its class's catalog entry,
        // before planning — whatever path the document came by.
        crawrad_registry::class_property::check(doc, reg)?;
        let form_nome = identificador(raiz.name())?;
        let tipo = crate::tipo_do_form(&form_nome);

        let emitidos: Vec<&Node> = doc
            .in_paint_order()
            .iter()
            .filter_map(|id| doc.node(id))
            // **THIRD-PARTY COMPONENTS ENTER HERE**, and this line was the
            // last place where an external class disappeared without a word.
            //
            // The filter was only the list of classes registered by THIS target, and
            // a `Gauge` installed via `manifest.json` fell outside it: the
            // form came out with `pub struct FormMain {}` — empty. No
            // field, no property, no drawing, and no warning.
            .filter(|n| {
                let effective = reg.effective_class(&n.class);
                CLASSES.contains(&effective) || components::terceiro(reg, &n.class).is_some()
            })
            .filter(|n| {
                if let Some(base_doc) = base {
                    !doc.is_inherited(&n.id, base_doc)
                } else {
                    true
                }
            })
            .collect();

        Ok(vec![
            Arquivo {
                caminho: PathBuf::from("src/main.rs"),
                conteudo: main_app::main_rs(&form_nome, &tipo),
                dono: Dono::Gerador,
                adicoes: Vec::new(),
            },
            Arquivo {
                caminho: PathBuf::from(format!("src/{form_nome}{SUFIXO_GERADO}")),
                conteudo: form::form_designer_rs_with_base(
                    doc, reg, raiz, &tipo, &emitidos, bases, is_base,
                )?,
                dono: Dono::Gerador,
                adicoes: Vec::new(),
            },
            stubs::stubs_with_base(
                reg,
                &form_nome,
                &tipo,
                raiz,
                &crate::events::handler_nodes(doc, &emitidos, base),
                base.map(|b| (b, doc)),
            )?,
        ])
    }
}
