//! # CrawRAD Code Generation Targets
//!
//! Each submodule represents an autonomous generation engine:
//! - `makepad`: Generates native Rust projects using the Makepad framework (`makepad-widgets`).
//! - `egui`: Generates Rust projects using the `eframe` / `egui` framework.
//! - `compartilhado`: The layout cuts and arithmetic emitted by both.

/// What BOTH targets carve from the same compiled file.
pub mod data_binding;
pub mod egui;
pub mod makepad;
pub mod shared;
pub mod slint;
pub mod speed_groups;
pub mod splitters;
pub mod tab_places;

use crawrad_note::note;
pub use shared as compartilhado;

#[note("targets-alvogerador")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlvoGerador {
    #[default]
    Makepad,
    Egui,
}

impl AlvoGerador {
    pub const TODOS: &[Self] = &[Self::Makepad, Self::Egui];

    pub fn rotulos() -> Vec<String> {
        Self::TODOS.iter().map(|a| a.rotulo().to_string()).collect()
    }

    pub fn indice(alvo: Self) -> Option<usize> {
        Self::TODOS.iter().position(|a| *a == alvo)
    }

    pub fn rotulo(&self) -> &'static str {
        match self {
            Self::Makepad => "Makepad (Nativo)",
            Self::Egui => "Egui / Eframe",
        }
    }

    pub fn alvo_curto(&self) -> &'static str {
        match self {
            Self::Makepad => "makepad",
            Self::Egui => "egui",
        }
    }

    pub fn sufixo_gerado(&self) -> &'static str {
        crate::SUFIXO_GERADO
    }

    pub fn extensao_codigo(&self) -> &'static str {
        "rs"
    }

    pub fn pasta_fontes(&self) -> &'static str {
        "src"
    }

    pub fn manifesto(&self) -> &'static str {
        "Cargo.toml"
    }

    pub fn create_generator(&self) -> Box<dyn crate::Gerador> {
        match self {
            Self::Makepad => Box::new(makepad::GeradorMakepad),
            Self::Egui => Box::new(egui::GeradorRxp),
        }
    }
}
