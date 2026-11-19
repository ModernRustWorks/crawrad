//! **The Glyph of a button** on Makepad, shared by `BitBtn` and
//! `SpeedButton` — and a `ToolBar`'s `Flat` and `ShowCaptions` for the
//! buttons on it.
//!
//! The picture is a sibling view `<field>_glyph` over the button's left edge
//! (`crawrad_runtime_makepad::glyph`); the button keeps every click.

use super::components::codegen::AppState;
use crate::{Erro, identificador};
use crawrad_ir::{Document, Node};
use crawrad_note::note;

#[note("targets-makepad-glyph-side")]
const SIDE: f32 = 16.0;

#[note("targets-makepad-glyph-tool-bar-of")]
pub fn tool_bar_of(doc: &Document, node: &Node) -> Result<Option<String>, Erro> {
    doc.parent_of(&node.id)
        .and_then(|p| doc.node(p))
        .filter(|p| p.class == "ToolBar")
        .map(|p| identificador(p.name()))
        .transpose()
}

#[note("targets-makepad-glyph-dsl")]
pub fn dsl(field: &str, x: f32, y: f32, h: f32) -> String {
    let left = x + 5.0;
    let top = y + ((h - SIDE) / 2.0).max(0.0);
    format!(
        "            {field}_glyph = <View> {{\n\
         \x20               visible: false\n\
         \x20               margin: {{ left: {left:.1}, top: {top:.1} }}\n\
         \x20               width: {SIDE:.1}, height: {SIDE:.1}\n\
         \x20               clip_x: true, clip_y: true\n\
         \x20               img = <Image> {{ width: {SIDE:.1}, height: {SIDE:.1}, fit: Stretch }}\n\
         \x20           }}\n"
    )
}

#[note("targets-makepad-glyph-sync")]
pub fn sync(field: &str) -> String {
    format!(
        "        crawrad_runtime_makepad::glyph::show(cx, &self.ui, id!({field}_glyph), &self.ui.widget(id!({field})), self.form.{field}.glyph.bytes(), self.form.{field}.num_glyphs, self.form.{field}.visible, &mut self.{field}_glyph_shown);\n"
    )
}

#[note("targets-makepad-glyph-app-state")]
pub fn app_state(field: &str) -> AppState {
    AppState {
        uses: "use crawrad_runtime_makepad::glyph::Shown as CrawradGlyphShown;\n".to_string(),
        field: format!("    #[rust]\n    pub {field}_glyph_shown: CrawradGlyphShown,\n"),
        handle: String::new(),
    }
}
