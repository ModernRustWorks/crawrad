//! The menus' model (bar, rows, shortcuts per scheme, contributions) lives in
//! `crawrad_actions::menu`, beside the catalog it reads; the shell draws it.

pub use crawrad_actions::menu::*;

/// The rows of a menu, using the active keymap scheme.
#[cfg(test)]
pub fn rows(menu: Menu, ctx: &crawrad_actions::Context) -> Vec<Row> {
    rows_with_scheme(menu, ctx, super::teclado::active_scheme())
}
