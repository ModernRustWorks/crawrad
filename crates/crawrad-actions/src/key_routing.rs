//! Which keymap is active, and who owns a key: the text editor, the view in
//! front, or the IDE. Pure rules over the catalog, moved out of the shell's
//! keyboard bridge, which keeps only the `KeyCode` translation.
//!
//! # The gap this module closes
//!
//! `grep -c KeyDown src/ide_app.rs` returned **0**. The Makepad IDE added
//! components via Toolbox and **deleted none**: the action `edit.delete`
//! exists fully in the catalog (`crates/crawrad-actions/src/lib.rs:860`), with
//! `atalho: Some("Del")`, and no one was listening for keystrokes. Being able to add
//! but not remove is not a gap, it is a trap.
//!
//! # Shortcut source is the CATALOG, never a local table
//!
//! `Action` carries `atalho: Option<&'static str>` as TEXT (`"Ctrl+Shift+B"`).
//! This module only writes the **chord → `KeyCode`** translation, and dispatch
//! queries the catalog for matches. A shortcut list written here would be the
//! second maintenance point that [ADR 0011] exists to prevent: tooltip
//! text and `match` keys can diverge without anyone noticing, and that
//! is exactly how the egui shell started.
//!
//! It follows the same design as `main.rs:5199` (`atalho_bate`), translated from egui to
//! Makepad — and the shell keyboard test verifies that **every** catalog shortcut
//! is reachable here, deriving the list from
//! `crawrad_actions::catalogo()` instead of hardcoding it.
//!
//! # Two destinations, not one
//!
//! Not every key is a catalog action. Arrow keys and `Esc` are **designer
//! gestures**: they have no place in a menu bar or in an agent call, and
//! therefore are not in the catalog. They go straight to `crawrad-designer`
//! (`Designer::nudge`, `Designer::nudge_size`, and `Designer::cancel`), just as
//! they do in the egui shell. See [`Comando`].
//!
//! And they do not carry DISTANCE: the bridge translates the chord to a
//! `crawrad_designer::Seta` and the step comes from `Designer::passo_da_seta`, which
//! knows the grid. A number written here would mean the options' `Grid size`
//! applied to the mouse but not the keyboard (ADR 0027).
//!
//! # What this module does NOT decide
//!
//! Whether the action can run now — that is answered by the catalog's own `habilitada`
//! in dispatch. Here translation is intentionally blind: `Del` returns
//! `edit.delete` even without a selection, and `habilitada` refuses with the
//! appropriate message in the status bar.
//!

use crate::KeymapScheme;
use crawrad_designer::ArrowNudge;
use crawrad_note::note;
use std::sync::atomic::{AtomicU8, Ordering};

static ESQUEMA_ATIVO: AtomicU8 = AtomicU8::new(0);

#[note("key-routing-active-scheme")]
pub fn active_scheme() -> KeymapScheme {
    match ESQUEMA_ATIVO.load(Ordering::Relaxed) {
        1 => KeymapScheme::Classic,
        _ => KeymapScheme::VsCode,
    }
}

#[note("key-routing-set-active-scheme")]
pub fn set_active_scheme(esquema: KeymapScheme) {
    let val = match esquema {
        KeymapScheme::VsCode => 0,
        KeymapScheme::Classic => 1,
    };
    ESQUEMA_ATIVO.store(val, Ordering::Relaxed);
}

#[note("key-routing-comando")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Comando {
    /// A catalog action, by id. It will still pass through `enabled` and
    /// `execute` during dispatch — this module decides none of that.
    Acao(&'static str),
    /// A designer arrow: the DIRECTION in signs and the MODE requested by the chord.
    ///
    /// This is not a catalog action — it is a designer gesture —, and does not carry
    /// distance: what knows the size of a grid cell is
    /// `Designer::passo_da_seta`, and the shell does not recalculate any of this
    /// (ADR 0027).
    ///
    /// [`Designer::arrow_step`]: crawrad_designer::Designer::arrow_step
    Seta { dx: f32, dy: f32, modo: ArrowNudge },
    /// Cancel the gesture in progress (`Designer::cancel`).
    Cancelar,
}

#[note("key-routing-press")]
pub struct Press<'a> {
    pub matches: &'a dyn Fn(&str) -> bool,
    /// The arrow's direction as signs, `None` when it is not an arrow.
    pub arrow: Option<(f32, f32)>,
    pub escape: bool,
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub logo: bool,
}

#[note("key-routing-commands")]
pub fn commands(p: &Press, from_field: bool) -> Vec<Comando> {
    let active = active_scheme();
    let mut cmds = commands_with_scheme(p, from_field, active);
    if cmds.is_empty() {
        let other = match active {
            KeymapScheme::VsCode => KeymapScheme::Classic,
            KeymapScheme::Classic => KeymapScheme::VsCode,
        };
        cmds = commands_with_scheme(p, from_field, other);
    }
    cmds
}

#[note("key-routing-is-paste")]
pub fn is_paste(p: &Press) -> bool {
    commands(p, true).contains(&Comando::Acao("edit.paste"))
}

#[note("key-routing-commands-with-scheme")]
pub fn commands_with_scheme(p: &Press, from_field: bool, scheme: KeymapScheme) -> Vec<Comando> {
    let mut out = Vec::new();
    for a in crate::catalogue_with_scheme(scheme) {
        // BOTH CHORDS, and the second is what this shell can deliver.
        //
        // `crawrad_actions::ATALHOS_PREFERIDOS` carries the pair that everyone
        // tries first for zoom — `Ctrl +`, `Ctrl -`, `Ctrl 0` — and explains
        // why it could not be the `shortcut`: in the **egui** host those
        // keys have an owner, and there the chord would trigger both canvas AND
        // chrome zoom. Here there is no owner, so the expected chord is available.
        //
        // Both come from the CATALOG, which is what ADR 0011 mandates. What it
        // forbids is the shell inventing keys — and not a single line here chooses one.
        let chords = a
            .shortcut
            .into_iter()
            .chain(crate::preferred_shortcuts(a.id).iter().copied());
        if chords.into_iter().any(|c| (p.matches)(c)) {
            out.push(Comando::Acao(a.id));
        }
    }
    if from_field {
        return out;
    }
    // THE FOUR ARROW KEYS OF A CLASSIC RAD IDE, and they have been four since 17/09/2026.
    //
    // It was once written here that `Shift+arrow` did nothing, stating: *"this
    // function DOES NOT EXIST in the core — `Designer::nudge` only moves"*. The owner
    // asked for the key and described the entire keymap, `crawrad-designer` gained
    // `nudge_size` and `passo_da_seta`, and both IDEs gained it together — which is
    // what that comment was asking for.
    //
    // `Alt` and `Logo` remain excluded: none of the four chords uses them, and an arrow
    // with `Alt` held down is almost always someone on their way to something else.
    if p.alt || p.logo {
        return out;
    }
    if p.escape && !p.shift && !p.control {
        out.push(Comando::Cancelar);
        return out;
    }
    let Some((dx, dy)) = p.arrow else {
        return out;
    };
    // THE CHORD decides the MODE; the STEP belongs to the designer. This shell does not know
    // the size of a grid cell, and must not know (ADR 0027).
    let mode = match (p.control, p.shift) {
        (false, false) => ArrowNudge::Move,
        (true, false) => ArrowNudge::MoveFine,
        (true, true) => ArrowNudge::MoveFast,
        (false, true) => ArrowNudge::Resize,
    };
    out.push(Comando::Seta { dx, dy, modo: mode });
    out
}

#[note("key-routing-not-here")]
pub fn not_here(id: &str, editor_focused: bool, code_in_front: bool) -> bool {
    (editor_focused && e_do_editor_de_texto(id)) || !serves_view(id, code_in_front)
}

#[note("key-routing-serves-view")]
pub fn serves_view(id: &str, code_in_front: bool) -> bool {
    use crate::ViewAffinity::{Code, Design};
    match crate::by_id(id).map(|a| a.view) {
        Some(Code) => code_in_front,
        Some(Design) => !code_in_front,
        _ => true,
    }
}

pub fn e_do_editor_de_texto(id: &str) -> bool {
    crate::by_id(id).is_some_and(|a| a.group == GRUPO_DO_TEXTO)
}

#[note("key-routing-grupo-do-texto")]
pub const GRUPO_DO_TEXTO: &str = "Edit";
