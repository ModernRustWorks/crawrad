use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct ColorBoxCodegen;

impl EguiComponentCodegen for ColorBoxCodegen {
    fn class(&self) -> &'static str {
        "ColorBox"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_COLORBOX)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit(ctx)
    }
}

#[crawrad_note::note("targets-egui-components-color-box-emit")]
pub fn emit(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("ColorBox").expect("ColorBox descriptor");
    let change = match handler(n, "on_change") {
        Some(h) => format!(
            "            if self.{name}.selected != before {{\n\
             \x20               self.{}();\n\
             \x20           }}\n",
            identificador(&h)?
        ),
        None => "            let _ = before;\n".to_string(),
    };
    let body = format!(
        "        if self.{name}.visible {{\n\
         \x20           let before = self.{name}.selected;\n\
         \x20           let swatch = |c: Color| egui::Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a);\n\
         \x20           let label = CRAWRAD_STANDARD_COLORS.iter().find(|(_, c)| *c == self.{name}.selected).map_or(\"Custom\", |(n, _)| *n);\n\
         \x20           let chip = egui::Rect::from_min_size(egui::pos2(r.min.x + 2.0, r.center().y - 7.0), egui::vec2(14.0, 14.0));\n\
         \x20           ui.painter().rect_filled(chip, 2.0, swatch(self.{name}.selected));\n\
         \x20           let list = egui::Rect::from_min_max(egui::pos2(r.min.x + 20.0, r.min.y), r.max);\n\
         \x20           ui.add_enabled_ui(self.{name}.enabled, |ui| {{\n\
         \x20               let mut child = ui.new_child(egui::UiBuilder::new().max_rect(list));\n\
         \x20               egui::ComboBox::from_id_salt((\"crawrad-colorbox\", \"{name}\"))\n\
         \x20                   .width(list.width())\n\
         \x20                   .selected_text(label)\n\
         \x20                   .show_ui(&mut child, |ui| {{\n\
         \x20                       for (n, c) in CRAWRAD_STANDARD_COLORS.iter() {{\n\
         \x20                           let row = ui.selectable_label(self.{name}.selected == *c, format!(\"      {{n}}\"));\n\
         \x20                           let at = egui::Rect::from_min_size(row.rect.min + egui::vec2(4.0, 3.0), egui::vec2(14.0, (row.rect.height() - 6.0).max(4.0)));\n\
         \x20                           ui.painter().rect_filled(at, 2.0, swatch(*c));\n\
         \x20                           if row.clicked() {{\n\
         \x20                               self.{name}.selected = *c;\n\
         \x20                           }}\n\
         \x20                       }}\n\
         \x20                   }});\n\
         \x20           }});\n\
         {change}\
         \x20       }}\n"
    );
    Ok(ResultadoComponente {
        campo: format!("    pub {name}: ColorBox,\n"),
        padrao: format!(
            "            {name}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: crate::targets::shared::standard_colors_const(),
        corpo: body,
        tipo_usado: "ColorBox",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_COLORBOX: StateText = StateText {
    head: r#"
/// A `ColorBox` on the form: the colour chosen from the standard ones.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
