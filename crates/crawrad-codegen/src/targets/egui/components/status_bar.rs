use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::{Erro, identificador};

pub struct StatusBarCodegen;

impl EguiComponentCodegen for StatusBarCodegen {
    fn class(&self) -> &'static str {
        "StatusBar"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_STATUSBAR)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        let nome = ctx.nome;
        let n = ctx.node;
        let desc = ctx.reg.get("StatusBar").expect("StatusBar descriptor");

        // Off SimplePanel the bar shows its panels, side by side, each
        // its own width, with a line between them
        let panels: Vec<String> = n
            .children
            .iter()
            .filter_map(|c| ctx.doc.node(c))
            .filter(|c| c.class == "StatusPanel")
            .map(|c| identificador(c.name()))
            .collect::<Result<_, _>>()?;
        let mut panel_code = String::new();
        for p in &panels {
            panel_code.push_str(&format!(
                "                    {{\n\
                 \x20                       let w = (self.{p}.width.max(0) as f32).min(right - x);\n\
                 \x20                       let cell = egui::Rect::from_min_size(egui::pos2(x, r.top()), egui::vec2(w, r.height()));\n\
                 \x20                       let align = match self.{p}.alignment.as_str() {{\n\
                 \x20                           \"taCenter\" => egui::Align::Center,\n\
                 \x20                           \"taRightJustify\" => egui::Align::Max,\n\
                 \x20                           _ => egui::Align::Min,\n\
                 \x20                       }};\n\
                 \x20                       ui.scope_builder(\n\
                 \x20                           egui::UiBuilder::new()\n\
                 \x20                               .max_rect(cell.shrink2(egui::vec2(4.0, 0.0)))\n\
                 \x20                               .layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight).with_cross_align(egui::Align::Center).with_main_align(align)),\n\
                 \x20                           |ui| {{\n\
                 \x20                               ui.label(self.{p}.text.as_str());\n\
                 \x20                           }},\n\
                 \x20                       );\n\
                 \x20                       x += w;\n\
                 \x20                       ui.painter().vline(x, r.y_range(), ui.visuals().widgets.noninteractive.bg_stroke);\n\
                 \x20                   }}\n"
            ));
        }
        let shown = if panels.is_empty() {
            "                let _ = self.{nome}.simple_panel;\n".replace("{nome}", nome)
        } else {
            format!(
                "                if !self.{nome}.simple_panel {{\n\
                 \x20                   let right = r.right();\n\
                 \x20                   let mut x = r.left();\n\
                 {panel_code}\
                 \x20                   return;\n\
                 \x20               }}\n"
            )
        };

        let body = format!(
            "        if self.{nome}.visible {{\n\
             \x20           let is_enabled = self.{nome}.enabled;\n\
             \x20           // a bar along the bottom, the text at its left, as in the\n\
             \x20           // classic RAD IDE (a Color of its own was painted above)\n\
             \x20           if self.{nome}.parent_color || self.{nome}.color.a == 0 {{\n\
             \x20               ui.painter().rect_filled(r, 0.0, ui.visuals().panel_fill);\n\
             \x20           }}\n\
             \x20           ui.add_enabled_ui(is_enabled, |ui| {{\n\
             {shown}\
             \x20               ui.scope_builder(\n\
             \x20                   egui::UiBuilder::new()\n\
             \x20                       .max_rect(r.shrink2(egui::vec2(6.0, 0.0)))\n\
             \x20                       .layout(egui::Layout::left_to_right(egui::Align::Center)),\n\
             \x20                   |ui| {{\n\
             \x20                       ui.label(self.{nome}.simple_text.as_str());\n\
             \x20                   }},\n\
             \x20               );\n\
             \x20           }});\n\
             \x20       }}\n"
        );

        Ok(ResultadoComponente {
            campo: format!("    pub {nome}: StatusBar,\n"),
            padrao: format!(
                "            {nome}: {},\n",
                crate::targets::egui::state::init_of(n, desc, ctx.doc)
            ),
            constante: String::new(),
            corpo: body,
            tipo_usado: "StatusBar",
            precisa_timer: false,
            precisa_item: false,
            compartilhado: Compartilhado::default(),
        })
    }
}

#[crawrad_note::note("targets-egui-components-status-bar-statuspanelcodegen")]
pub struct StatusPanelCodegen;

impl EguiComponentCodegen for StatusPanelCodegen {
    fn class(&self) -> &'static str {
        "StatusPanel"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_STATUSPANEL)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        let nome = ctx.nome;
        let n = ctx.node;
        let desc = ctx.reg.get("StatusPanel").expect("StatusPanel descriptor");
        Ok(ResultadoComponente {
            campo: format!("    pub {nome}: StatusPanel,\n"),
            padrao: format!(
                "            {nome}: {},\n",
                crate::targets::egui::state::init_of(n, desc, ctx.doc)
            ),
            constante: String::new(),
            corpo: String::new(),
            tipo_usado: "StatusPanel",
            precisa_timer: false,
            precisa_item: false,
            compartilhado: Compartilhado::default(),
        })
    }
}

const ESTADO_STATUSBAR: StateText = StateText {
    head: r#"
/// A `StatusBar` on the form.
#[derive(Clone, Debug)]
"#,
    tail: "",
};

const ESTADO_STATUSPANEL: StateText = StateText {
    head: r#"
/// One panel of a `StatusBar`: its text, its width in points, and where the
/// text sits (`taLeftJustify`, `taCenter`, `taRightJustify`).
#[derive(Clone, Debug)]
"#,
    tail: "",
};
