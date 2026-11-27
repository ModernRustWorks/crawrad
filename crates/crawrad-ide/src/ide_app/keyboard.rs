//! A PONTE DO TECLADO da casca Makepad — de `KeyCode` a comando.
//!
//! What a key DOES — and why the catalog is its only source — is told in
//! `crawrad_actions::key_routing`; this keeps the `KeyCode` side.

use crawrad_actions::key_routing::{self, Press};
use makepad_widgets::{KeyCode, KeyModifiers};
// The active scheme, who owns a key and what a key does live beside the
// catalog (ratchet); this keeps only the `KeyCode` translation.
pub use crawrad_actions::key_routing::set_active_scheme;
pub use crawrad_actions::key_routing::{Comando, Front, active_scheme, blocked};
use crawrad_note::note;
pub use crawrad_runtime_makepad::keys::{arrow_direction, matches_shortcut};

#[note("ide-app-keyboard-comandos")]
pub fn comandos(tecla: KeyCode, m: &KeyModifiers, do_campo: bool) -> Vec<Comando> {
    let matches = |c: &str| matches_shortcut(c, tecla, m);
    key_routing::commands(&press(tecla, m, &matches), do_campo)
}

/// Whether the key is the paste chord of the active scheme.
pub fn is_paste(tecla: KeyCode, m: &KeyModifiers) -> bool {
    key_routing::is_paste(&press(tecla, m, &|c: &str| matches_shortcut(c, tecla, m)))
}

/// The commands a key fires in `esquema`: see [`key_routing::commands_with_scheme`].
#[cfg(test)]
pub fn commands_with_scheme(
    tecla: KeyCode,
    m: &KeyModifiers,
    do_campo: bool,
    esquema: crawrad_actions::KeymapScheme,
) -> Vec<Comando> {
    let matches = |c: &str| matches_shortcut(c, tecla, m);
    key_routing::commands_with_scheme(&press(tecla, m, &matches), do_campo, esquema)
}

fn press<'a>(tecla: KeyCode, m: &KeyModifiers, matches: &'a dyn Fn(&str) -> bool) -> Press<'a> {
    let (shift, control, alt, logo) = (m.shift, m.control, m.alt, m.logo);
    let escape = tecla == KeyCode::Escape;
    let arrow = arrow_direction(tecla);
    Press {
        matches,
        arrow,
        escape,
        shift,
        control,
        alt,
        logo,
    }
}
