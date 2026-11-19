use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::{handler, pai_de};
use crate::{Erro, identificador};

pub struct RadioButtonCodegen;

impl EguiComponentCodegen for RadioButtonCodegen {
    fn class(&self) -> &'static str {
        "RadioButton"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_RADIOBUTTON)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("RadioButton").expect("RadioButton descriptor");
    let doc = ctx.doc;
    let radios = ctx.radios;

    // A legenda sai do ESTADO — mesmo motivo escrito no `checkbox.rs`.
    let widget = format!("egui::RadioButton::new(self.{nome}.checked, self.{nome}.text.as_str())");
    let pai = pai_de(doc, n);
    // a radio whose `checked` really moved calls its own OnChange —
    // the one the click marks, and the sibling that loses the mark
    let on_change_of = |field: &str| -> Result<String, Erro> {
        let node = doc.nodes.values().find(|o| {
            o.class == "RadioButton" && identificador(o.name()).ok().as_deref() == Some(field)
        });
        Ok(match node.and_then(|o| handler(o, "on_change")) {
            Some(h) => format!("                    self.{}();\n", identificador(&h)?),
            None => String::new(),
        })
    };
    let mut exclusao = String::new();
    for (_, outro) in radios.iter().filter(|(p, _)| *p == pai) {
        let (was, now) = if *outro == nome {
            ("!", "true")
        } else {
            ("", "false")
        };
        let call = on_change_of(outro)?;
        // with no handler there is nothing to guard: a plain assignment
        if call.is_empty() {
            exclusao.push_str(&format!("                self.{outro}.checked = {now};\n"));
        } else {
            exclusao.push_str(&format!(
                "                if {was}self.{outro}.checked {{\n\
                 \x20                   self.{outro}.checked = {now};\n\
                 {call}\
                 \x20               }}\n"
            ));
        }
    }
    let chamada = match handler(n, "on_click") {
        Some(h) => format!("                self.{}();\n", identificador(&h)?),
        None => String::new(),
    };
    let depois = if exclusao.is_empty() && chamada.is_empty() {
        "            let _ = clicked;\n".to_string()
    } else {
        format!("            if clicked {{\n{exclusao}{chamada}            }}\n")
    };
    let corpo = format!(
        "        if self.{nome}.visible {{\n\
         \x20           let mut builder = egui::UiBuilder::new().max_rect(r);\n\
         \x20           if !self.{nome}.enabled {{\n\
         \x20               builder = builder.disabled();\n\
         \x20           }}\n\
         \x20           let clicked = ui\n\
         \x20               .new_child(builder)\n\
         \x20               .scope(|ui| {{\n\
         \x20                   ui.scope_builder(egui::UiBuilder::new().max_rect(r).layout(egui::Layout::left_to_right(egui::Align::Center)), |ui| ui.add({widget})).inner\n\
         \x20                       .clicked()\n\
         \x20               }})\n\
         \x20               .inner;\n\
         {depois}\
         \x20       }}\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: RadioButton,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo,
        tipo_usado: "RadioButton",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_RADIOBUTTON: StateText = StateText {
    head: r#"
/// A `RadioButton` on the form.
///
/// Picking one clears its siblings: the exclusion is in the generated code,
/// not up to you. It is the same rule the designer shows, and the same rule
/// classic RAD has — the group is the **parent**, so two panels side by side are
/// two independent groups.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
