use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::Erro;

pub struct ImageCodegen;

impl EguiComponentCodegen for ImageCodegen {
    fn class(&self) -> &'static str {
        "Image"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_IMAGE)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("Image").expect("Image descriptor");

    // the picture is decoded with what `eframe` already brings (a
    // generated egui project compiles `image` with `png` through it), once,
    // and kept as a texture in egui's memory under a key that changes with the
    // picture, so a picture the form's code swaps at run time is decoded
    // again. No loader to install, no new dependency in the user's project.
    // A format eframe's `image` does not read (JPEG today) draws the empty box.
    let body = format!(
        "        if self.{nome}.visible {{\n\
         \x20           let is_enabled = self.{nome}.enabled;\n\
         \x20           let bytes = self.{nome}.picture.bytes();\n\
         \x20           let key = egui::Id::new((\"crawrad-picture\", \"{nome}\", bytes.len(), bytes.iter().take(64).fold(0u64, |h, b| h.wrapping_mul(31).wrapping_add(*b as u64))));\n\
         \x20           let texture = if bytes.is_empty() {{\n\
         \x20               None\n\
         \x20           }} else {{\n\
         \x20               ui.ctx().data_mut(|d| d.get_temp::<Option<egui::TextureHandle>>(key)).unwrap_or_else(|| {{\n\
         \x20                   let loaded = eframe::icon_data::from_png_bytes(bytes).ok().map(|icon| {{\n\
         \x20                       let size = [icon.width as usize, icon.height as usize];\n\
         \x20                       ui.ctx().load_texture(\"{nome}\", egui::ColorImage::from_rgba_unmultiplied(size, &icon.rgba), Default::default())\n\
         \x20                   }});\n\
         \x20                   ui.ctx().data_mut(|d| d.insert_temp(key, loaded.clone()));\n\
         \x20                   loaded\n\
         \x20               }})\n\
         \x20           }};\n\
         \x20           ui.add_enabled_ui(is_enabled, |ui| {{\n\
         \x20               match texture {{\n\
         \x20                   Some(tex) => {{\n\
         \x20                       let mut size = tex.size_vec2();\n\
         \x20                       if self.{nome}.stretch || self.{nome}.auto_size {{\n\
         \x20                           let fit = r.size() / size;\n\
         \x20                           size = if self.{nome}.proportional {{ size * fit.x.min(fit.y) }} else {{ r.size() }};\n\
         \x20                       }}\n\
         \x20                       let at = if self.{nome}.center {{ r.center() - size / 2.0 }} else {{ r.min }};\n\
         \x20                       let mut image = egui::Image::new((tex.id(), size));\n\
         \x20                       if !self.{nome}.transparent {{\n\
         \x20                           image = image.bg_fill(ui.visuals().extreme_bg_color);\n\
         \x20                       }}\n\
         \x20                       ui.put(egui::Rect::from_min_size(at, size).intersect(r), image);\n\
         \x20                   }}\n\
         \x20                   None => {{\n\
         \x20                       ui.painter().rect_stroke(\n\
         \x20                           r,\n\
         \x20                           1.0,\n\
         \x20                           egui::Stroke::new(1.0, egui::Color32::from_rgb(0xB0, 0xB0, 0xB8)),\n\
         \x20                           egui::StrokeKind::Inside,\n\
         \x20                       );\n\
         \x20                   }}\n\
         \x20               }}\n\
         \x20           }});\n\
         \x20       }}\n"
    );
    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: Image,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo: body,
        tipo_usado: "Image",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_IMAGE: StateText = StateText {
    head: r#"
/// An `Image` on the form.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
