use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};
use crawrad_ir::Value;

pub struct BadgeCodegen;

impl EguiComponentCodegen for BadgeCodegen {
    fn class(&self) -> &'static str {
        "Badge"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_BADGE)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit(ctx)
    }
}

#[crawrad_note::note("targets-egui-components-badge-emit")]
pub fn emit(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("Badge").expect("Badge descriptor");
    let click_code = match handler(n, "on_click") {
        Some(h) => {
            let func = identificador(&h)?;
            format!(
                "            if ui.interact(r, egui::Id::new(\"{name}\"), egui::Sense::click()).clicked() {{\n\
                 \x20               self.{func}();\n\
                 \x20           }}\n"
            )
        }
        None => String::new(),
    };
    let assoc_raw = match n.props.get("associate") {
        Some(Value::Ref(s)) | Some(Value::Str(s)) => s.as_str(),
        _ => "",
    };
    let assoc_target = if !assoc_raw.is_empty() {
        ctx.doc
            .node(assoc_raw)
            .or_else(|| ctx.doc.nodes.values().find(|nd| nd.name() == assoc_raw))
    } else {
        None
    };

    let assoc_code = if let Some(target_node) = assoc_target {
        let assoc_name = identificador(target_node.name())?;
        let target_id = &target_node.id;
        format!(
            "                let mut r = egui::Rect::from_min_size(r.min, egui::vec2(w, h));\n\
             \x20               if !self.{name}.associate.is_empty() && (self.{name}.associate == \"{assoc_name}\" || self.{name}.associate == \"{target_id}\") {{\n\
             \x20                   r = egui::Rect::from_center_size(r_{assoc_name}.right_top(), egui::vec2(w, h));\n\
             \x20               }}\n"
        )
    } else {
        format!(
            "                let mut r = egui::Rect::from_min_size(r.min, egui::vec2(w, h));\n\
             \x20               if !self.{name}.associate.is_empty() {{\n\
             \x20                   r = egui::Rect::from_center_size(r.right_top(), egui::vec2(w, h));\n\
             \x20               }}\n"
        )
    };

    let body = format!(
        "        if self.{name}.visible\n\
         \x20           && let Some(badge_str) = badge_text(&self.{name}.text, self.{name}.count, self.{name}.max_count)\n\
         \x20       {{\n\
         \x20           {{\n\
         \x20               let h = r.height();\n\
         \x20               let w = if badge_str.chars().count() == 1 {{ h }} else {{ r.width().max(h) }};\n\
         {assoc_code}\
         \x20               let effective_kind = if self.{name}.count > 0 && (self.{name}.kind == \"bkPrimary\" || self.{name}.kind.is_empty()) {{\n\
         \x20                   \"bkDanger\"\n\
         \x20               }} else {{\n\
         \x20                   &self.{name}.kind\n\
         \x20               }};\n\
         \x20               let font = egui::FontId::proportional(11.0);\n\
         \x20               let mut painter = ui.painter().clone();\n\
         \x20               if !self.{name}.enabled {{\n\
         \x20                   painter.multiply_opacity(0.5);\n\
         \x20               }}\n\
         \x20               paint_badge(&painter, r, &badge_str, effective_kind, self.{name}.outline, ui.visuals(), font);\n\
         {click_code}\
         \x20           }}\n\
         \x20       }}\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {name}: Badge,\n"),
        padrao: format!(
            "            {name}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo: body,
        tipo_usado: "Badge",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_BADGE: StateText = StateText {
    head: r#"
/// A `Badge` on the form: a pill label (#500).
#[derive(Clone, Debug)]
"#,
    tail: "",
};
