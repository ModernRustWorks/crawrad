//! The UI frameworks a GUI project can be made with — **one list**.
//!
//! The New Project dialog draws one row per entry and the detail of the
//! selected one; adding a framework is an entry here, not a layout change.
//! Two side-by-side cards had room for two; Slint and others would each have
//! meant redrawing the dialog.

use crawrad_codegen::AlvoGerador;
use crawrad_note::note;

#[note("frameworks-availability")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Availability {
    /// The generator writes it.
    Ready,
    /// Listed so the user sees where the product is going; not selectable.
    Planned,
}

#[note("frameworks-framework")]
#[derive(Debug, Clone, Copy)]
pub struct Framework {
    pub name: &'static str,
    /// The generator that writes it; `None` while it is only planned.
    pub target: Option<AlvoGerador>,
    pub summary: &'static str,
    pub platforms: &'static str,
    /// The crate the project's manifest gets.
    pub crate_dep: &'static str,
    pub availability: Availability,
}

#[note("frameworks-rows")]
pub const ROWS: usize = 6;

#[note("frameworks-catalogue")]
pub const CATALOGUE: &[Framework] = &[
    Framework {
        name: "Makepad",
        target: Some(AlvoGerador::Makepad),
        summary: "GPU-drawn, live-reload of the design, shaders and effects.",
        platforms: "⊞ Windows   🐧 Linux   🍎 macOS   🌐 Web   📱 Mobile",
        crate_dep: "makepad-widgets",
        availability: Availability::Ready,
    },
    Framework {
        name: "egui",
        target: Some(AlvoGerador::Egui),
        summary: "Immediate mode, small and quick to build.",
        platforms: "⊞ Windows   🐧 Linux   🍎 macOS   🌐 Web",
        crate_dep: "eframe",
        availability: Availability::Ready,
    },
];

#[note("frameworks-of")]
pub fn of(target: AlvoGerador) -> &'static Framework {
    CATALOGUE
        .iter()
        .find(|f| f.target == Some(target))
        .expect("every generator target is in the catalogue")
}

#[note("frameworks-badge")]
pub fn badge(f: &Framework) -> &'static str {
    match f.availability {
        Availability::Ready => "",
        Availability::Planned => "planned",
    }
}

#[note("frameworks-resolve-framework")]
pub fn resolve_framework(name: Option<&str>) -> Result<AlvoGerador, String> {
    let Some(name) = name.map(str::trim).filter(|s| !s.is_empty()) else {
        return Ok(AlvoGerador::Makepad);
    };
    for f in CATALOGUE {
        if f.name.eq_ignore_ascii_case(name) {
            return f.target.ok_or_else(|| {
                format!("Framework '{name}' is planned and not yet available for generation")
            });
        }
    }
    let valid: Vec<_> = CATALOGUE
        .iter()
        .filter_map(|f| f.target.map(|_| f.name.to_ascii_lowercase()))
        .collect();
    Err(format!(
        "Unknown framework '{name}'. Valid frameworks: {}",
        valid.join(", ")
    ))
}

#[cfg(test)]
#[path = "test/frameworks/test.rs"]
mod tests;
