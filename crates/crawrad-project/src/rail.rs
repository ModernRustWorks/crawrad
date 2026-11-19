//! The ACTIVITY RAIL of the Makepad shell — which panels it offers, and what
//! a click on one of them does.
//!
//! # The rail stopped owning the left column
//!
//! It used to be the switch of a nailed-down region: one destination at a
//! time filled `left_dock`, and `IdeApp::mostrar_doca` flipped the visibility
//! of three views. [ADR 0014, Emenda 9] took the region's owner away — every
//! panel is a dockable tab now, and the rail is what **brings a panel back
//! into view**, wherever the user has left it.
//!
//! So a rail click is no longer shell state: it runs `layout.toggle_panel`
//! from the catalogue, with this panel's id as the argument. That is the
//! owner's law of 15/09/2026 applied to the smallest button on screen — *can
//! an agent do this by tool, with no mouse?* — and it is why the rail and the
//! MCP cannot drift: they are the same command.
//!
//! # Why the list lives HERE, and only here
//!
//! The rail is drawn by a `PortalList` fed by [`RAIL`], not by a row of
//! buttons written in the DSL. A row in the DSL would be the list of
//! destinations written in two places, and the second would be born out of
//! date the day the first grew one. What the DSL holds is the TEMPLATE of a
//! button, never which ones they are.
//!
//! [`RAIL`] is a subset of [`Panel::ALL`] and not a second list: `Design` and
//! `Chat` are dockable panels with no rail icon — `Design` because it is the
//! thing the rail exists to uncover, and `Chat` because Emenda 1 gave it the
//! right column and Emenda 9 let the user move it from there.
//!
//! # Every destination on the rail has a screen now
//!
//! `Explorer` and `Package Manager` used to sit here muted, with the reason
//! written inside a shared notice panel — which is the shape the owner's law
//! *"what you remove, you later forget"* asks for while a destination exists
//! and its screen does not. The two got their screens in this round,
//! and the muted rail template, the notice panel and the
//! `pending_reason` that fed them went out with them.
//!
//! Keeping them would not have been caution, it would have been a promise: a
//! code path no panel can reach reads as a mechanism that is still doing
//! something. What the day of the next screenless destination needs is written
//! down instead — a `pending_reason` here, one `PanelPending` in the DSL, and
//! the muted rail template. Twenty lines, rebuilt on purpose.
//!
//! `Settings` is **not** on the rail, and the reason is the one that took it
//! off the egui rail (`main.rs:294-298`): *"every rail item is a panel, and
//! they all talk about what is open. `Settings` was here and left: it is about
//! the IDE, not about the form, and it became a window."*
//!

use crawrad_actions::Panel;

#[note("rail-rail")]
pub const RAIL: &[Panel] = &[
    Panel::Explorer,
    Panel::Components,
    Panel::Inspector,
    Panel::Packages,
    Panel::Tests,
    Panel::Debug,
    Panel::Extensions,
];

#[note("rail-visible")]
pub fn visible(has_form: bool) -> Vec<Panel> {
    if has_form {
        RAIL.to_vec()
    } else {
        RAIL.iter()
            .copied()
            .filter(|p| *p != Panel::Inspector)
            .collect()
    }
}

/// The rail's icons live beside the chrome table they come from (`crawrad_project::icons`).
pub use crate::icons::rail_glyph as glyph;
use crawrad_note::note;

#[cfg(test)]
#[path = "test/rail/test.rs"]
mod test;

// HERE WAS `SETTINGS_NOT_WIRED` — the sentence the bar gave when the
// gear was a button that only spoke. It was removed on 17/09/2026:
// `Options` exists in this shell, and the gear opens it.
//
// The sentence also explained WHY `Settings` is not a rail panel, and that
// remains true — a rail item is a panel about the open FORM, and
// Settings is a window about the IDE. The test that now preserves this fact is
// `a_engrenagem_nao_e_painel_e_abre_as_opcoes`, which verifies both halves.
