use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct CategoryPanelCodegen;

impl EguiComponentCodegen for CategoryPanelCodegen {
    fn class(&self) -> &'static str {
        "CategoryPanel"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_CATEGORYPANEL)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emit(ctx)
    }

    #[crawrad_note::note(
        "targets-egui-components-category-panel-categorypanelcodegen-envolve-filhos"
    )]
    fn envolve_filhos(&self) -> bool {
        true
    }

    fn corpo_fecha(&self, _ctx: &ContextoEmissao) -> Result<String, Erro> {
        Ok(CLOSE.to_string())
    }
}

#[crawrad_note::note("targets-egui-components-category-panel-close")]
const CLOSE: &str = "\
\x20               });\n\
\x20           });\n\
\x20           }\n\
\x20       }\n";

#[crawrad_note::note("targets-egui-components-category-panel-emit")]
pub fn emit(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let name = ctx.nome;
    let n = ctx.node;
    let desc = ctx
        .reg
        .get("CategoryPanel")
        .expect("CategoryPanel descriptor");
    let call = |event: &str| -> Result<String, Erro> {
        Ok(match handler(n, event) {
            Some(h) => format!("                    self.{}();\n", identificador(&h)?),
            None => String::new(),
        })
    };
    let on_collapsed = call("on_collapsed")?;
    let on_expanded = call("on_expanded")?;
    let dispatch = crate::targets::shared::toggle_branches(
        &format!("self.{name}.collapsed"),
        &on_collapsed,
        &on_expanded,
        "                ",
    );
    let body = format!(
        "        if self.{name}.visible {{\n\
         \x20           let font = ui.style().override_font_id.clone().unwrap_or_else(|| egui::TextStyle::Button.resolve(ui.style()));\n\
         \x20           let header = ui\n\
         \x20               .add_enabled_ui(self.{name}.enabled, |ui| {{\n\
         \x20                   paint_category(ui.painter(), r, self.{name}.text.as_str(), self.{name}.collapsed, font, ui.visuals(), 24.0)\n\
         \x20               }})\n\
         \x20               .inner;\n\
         \x20           if self.{name}.enabled && ui.interact(header, ui.id().with((\"crawrad-category\", \"{name}\")), egui::Sense::click()).clicked() {{\n\
         \x20               self.{name}.collapsed = !self.{name}.collapsed;\n\
         {dispatch}\
         \x20           }}\n\
         \x20           if !self.{name}.collapsed {{\n\
         \x20           ui.add_enabled_ui(self.{name}.enabled, |ui| {{\n\
         \x20               ui.scope_builder(egui::UiBuilder::new().max_rect(r), |ui| {{\n"
    );
    Ok(ResultadoComponente {
        campo: format!("    pub {name}: CategoryPanel,\n"),
        padrao: format!(
            "            {name}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo: body,
        tipo_usado: "CategoryPanel",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_CATEGORYPANEL: StateText = StateText {
    head: r#"
/// A `CategoryPanel` on the form: a container under a header that folds it.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
