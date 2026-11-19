//! Code generator for the **EGUI / EFRAME** target (formerly RXP).
//!
//! Organized into single-responsibility modules:
//! - `declarado`: A third party's declared drawing turned into egui painting.
//! - `desenho`: Shared code snippets (`desenho_comum.rs`).
//! - `layout`: Geometry, anchors, and visibility.
//! - `main_app`: Executable entry point (`src/main.rs`).
//! - `form`: Form designer (`<form>.designer.rs`).
//! - `stubs`: User file (`<form>.rs`), stubs, and pruning routines.
//! - `componentes`: Emitting each component class in a dedicated file.

pub mod actions;
pub mod components;
pub mod control_events;
pub mod control_keys;
pub use components as componentes;
pub mod declarado;
pub mod desenho;
pub mod form;
pub mod form_events;
pub mod glyph;
pub mod layout;
pub mod main_app;
pub mod state;
pub mod stubs;
pub mod tab_order;

#[cfg(test)]
#[path = "../../test/targets/egui/test.rs"]
mod tests;

use std::path::PathBuf;

pub use desenho::{DESENHO_COMUM, recorte, trecho};
pub use form::ui_rs;
pub use layout::{desenha_por_si, e_visual, emitido, emitido_no};
pub use main_app::{main_rs, with_window_icon};
pub use stubs::{
    MARCAS_ACEITAS, eh_evento_conhecido, marca_em, podar, podaveis, todos_os_eventos,
    unidade_do_usuario, vinculos,
};

std::thread_local! {
    static LOCALES_ACTIVE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Sets whether the generated form should emit translation support.
pub fn set_has_locales(active: bool) {
    LOCALES_ACTIVE.with(|c| c.set(active));
}

/// Returns whether translation support is active for this generation thread.
pub fn has_locales() -> bool {
    LOCALES_ACTIVE.with(|c| c.get())
}

/// RAII scope for temporarily enabling or disabling translation support.
pub struct LocalesScope(bool);

impl LocalesScope {
    pub fn new(active: bool) -> Self {
        let prev = has_locales();
        set_has_locales(active);
        Self(prev)
    }
}

impl Drop for LocalesScope {
    fn drop(&mut self) {
        set_has_locales(self.0);
    }
}

use crate::{Arquivo, Dono, Erro, Gerador, SUFIXO_GERADO, identificador};
use crawrad_ir::{Document, Node};
use crawrad_note::note;

#[note("targets-egui-classes")]
pub static CLASSES: std::sync::LazyLock<Vec<&'static str>> =
    std::sync::LazyLock::new(components::classes);

pub struct GeradorRxp;

impl Gerador for GeradorRxp {
    fn alvo(&self) -> &'static str {
        "egui"
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
            .filter(|n| emitido_no(doc, n))
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
                conteudo: main_rs(&form_nome, &tipo, raiz),
                dono: Dono::Gerador,
                adicoes: Vec::new(),
            },
            Arquivo {
                caminho: PathBuf::from(format!("src/{form_nome}{SUFIXO_GERADO}")),
                conteudo: form::ui_rs_with_base(doc, reg, raiz, &tipo, &emitidos, bases, is_base)?,
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
