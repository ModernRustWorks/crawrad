//! Rust code sections clipped from `desenho_comum.rs`.
//!
//! A single source shared between the preview runtime and generated code.

pub const DESENHO_COMUM: &str =
    include_str!("../../../../crawrad-runtime-egui/src/desenho_comum.rs");

#[note("targets-egui-desenho-trecho")]
pub fn trecho(nome: &str) -> String {
    recorte(DESENHO_COMUM, nome)
}

/// Clipping, and shared arithmetic emitted by BOTH targets.
///
/// The three lived here and moved to [`crate::targets::compartilhado`] when
/// the Makepad target began clipping the same `fraction`: one target reaching into
/// the other's module to find a text function is the kind of dependency that only
/// shows up once spread around.
pub use crate::targets::compartilhado::{
    GEOMETRIA as GEOMETRIA_COMPARTILHADA, recorte, trecho_da_geometria,
};
use crawrad_note::note;
