//! **The Glyph of a button**: the picture left of the caption, the
//! first of `NumGlyphs` side by side in it — and, for a button on a
//! `ToolBar`, the bar's `ShowCaptions`.
//!
//! The picture is decoded the way the `Image` decodes its own (with
//! the `image` crate `eframe` already brings), once per picture, and kept as
//! a texture in egui's memory under a key that changes with the bytes.

use crate::{Erro, identificador};
use crawrad_ir::{Document, Node};
use crawrad_note::note;

#[note("targets-egui-glyph-side")]
pub const SIDE: f32 = 16.0;

#[note("targets-egui-glyph-tool-bar-of")]
pub fn tool_bar_of(doc: &Document, node: &Node) -> Result<Option<String>, Erro> {
    doc.parent_of(&node.id)
        .and_then(|p| doc.node(p))
        .filter(|p| p.class == "ToolBar")
        .map(|p| identificador(p.name()))
        .transpose()
}

#[note("targets-egui-glyph-atoms-code")]
pub fn atoms_code(doc: &Document, node: &Node, nome: &str, pad: &str) -> Result<String, Erro> {
    let caption = match tool_bar_of(doc, node)? {
        Some(bar) => {
            format!("if self.{bar}.show_captions {{ self.{nome}.text.as_str() }} else {{ \"\" }}")
        }
        None => format!("self.{nome}.text.as_str()"),
    };
    Ok(format!(
        "{pad}// the Glyph, left of the caption (the first of NumGlyphs)\n\
         {pad}let glyph_bytes = self.{nome}.glyph.bytes();\n\
         {pad}let glyph = if glyph_bytes.is_empty() {{\n\
         {pad}    None\n\
         {pad}}} else {{\n\
         {pad}    let key = egui::Id::new((\"crawrad-glyph\", \"{nome}\", glyph_bytes.len(), glyph_bytes.iter().take(64).fold(0u64, |h, b| h.wrapping_mul(31).wrapping_add(*b as u64))));\n\
         {pad}    ui.ctx().data_mut(|d| d.get_temp::<Option<egui::TextureHandle>>(key)).unwrap_or_else(|| {{\n\
         {pad}        let loaded = eframe::icon_data::from_png_bytes(glyph_bytes).ok().map(|icon| {{\n\
         {pad}            let size = [icon.width as usize, icon.height as usize];\n\
         {pad}            ui.ctx().load_texture(\"{nome}_glyph\", egui::ColorImage::from_rgba_unmultiplied(size, &icon.rgba), Default::default())\n\
         {pad}        }});\n\
         {pad}        ui.ctx().data_mut(|d| d.insert_temp(key, loaded.clone()));\n\
         {pad}        loaded\n\
         {pad}    }})\n\
         {pad}}};\n\
         {pad}let mut atoms = egui::Atoms::new({caption});\n\
         {pad}if let Some(tex) = &glyph {{\n\
         {pad}    let n = self.{nome}.num_glyphs.max(1) as f32;\n\
         {pad}    atoms.push_left(egui::Image::new((tex.id(), egui::vec2({SIDE:.1}, {SIDE:.1}))).uv(egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0 / n, 1.0))));\n\
         {pad}}}\n"
    ))
}
