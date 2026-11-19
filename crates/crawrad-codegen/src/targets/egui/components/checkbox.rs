use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};

pub struct CheckBoxCodegen;

impl EguiComponentCodegen for CheckBoxCodegen {
    fn class(&self) -> &'static str {
        "CheckBox"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_CHECKBOX)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("CheckBox").expect("CheckBox descriptor");

    // A legenda sai do ESTADO, e nao de um literal cravado aqui: e o que o
    // ADR 0010 pede, e e o que faz `self.chk.text = "Outra".into()` mudar
    // a tela. Enquanto era literal, o campo `text` do designer nao era
    // lido por ninguem — o `dead_code` sabia disso e estava calado.
    // The grayed state draws the indeterminate mark
    let widget = format!(
        "egui::Checkbox::new(&mut self.{nome}.checked, self.{nome}.text.as_str()).indeterminate(self.{nome}.state == \"cbGrayed\")"
    );
    // The box alone, its caption drawn beside it (`Alignment` left)
    let widget_bare = format!(
        "egui::Checkbox::new(&mut self.{nome}.checked, \"\").indeterminate(self.{nome}.state == \"cbGrayed\")"
    );
    // the click first, then the change, as the classic RAD IDE orders them
    let mut chamada = String::new();
    for event in ["on_click", "on_change"] {
        if let Some(h) = handler(n, event) {
            chamada.push_str(&format!("                self.{}();\n", identificador(&h)?));
        }
    }
    // a click moves `state` one step (Unchecked -> Checked -> Grayed when
    // `allow_grayed`, a toggle otherwise), and `checked` follows it
    let depois = format!(
        "            if clicked {{\n\
         \x20               let next = if self.{nome}.allow_grayed {{\n\
         \x20                   match before.as_str() {{\n\
         \x20                       \"cbUnchecked\" => \"cbChecked\",\n\
         \x20                       \"cbChecked\" => \"cbGrayed\",\n\
         \x20                       _ => \"cbUnchecked\",\n\
         \x20                   }}\n\
         \x20               }} else if self.{nome}.checked {{\n\
         \x20                   \"cbChecked\"\n\
         \x20               }} else {{\n\
         \x20                   \"cbUnchecked\"\n\
         \x20               }};\n\
         \x20               self.{nome}.state = next.to_string();\n\
         \x20               self.{nome}.checked = next == \"cbChecked\";\n\
         {chamada}\
         \x20           }}\n"
    );
    let corpo = format!(
        "        if self.{nome}.visible {{\n\
                    // `checked` is the truth for on/off; `state` follows it,\n\
                    // except for cbGrayed, which only `state` can say\n\
                    if self.{nome}.state != \"cbGrayed\" {{\n\
                        let want = if self.{nome}.checked {{ \"cbChecked\" }} else {{ \"cbUnchecked\" }};\n\
                        if self.{nome}.state != want {{\n\
                            self.{nome}.state = want.to_string();\n\
                        }}\n\
                    }}\n\
                    let before = self.{nome}.state.clone();\n\
                    let mut builder = egui::UiBuilder::new().max_rect(r);\n\
                    if !self.{nome}.enabled {{\n\
                        builder = builder.disabled();\n\
                    }}\n\
                    let clicked = ui\n\
                        .new_child(builder)\n\
                        .scope(|ui| {{\n\
                            ui.scope_builder(egui::UiBuilder::new().max_rect(r).layout(egui::Layout::left_to_right(egui::Align::Center)), |ui| {{\n\
                                // #445: `Alignment` — the caption at the left edge and the\n\
                                // box at the right one; a click on either toggles\n\
                                if self.{nome}.alignment == \"taLeftJustify\" {{\n\
                                    let caption = ui.add(egui::Label::new(self.{nome}.text.as_str()).sense(egui::Sense::click()));\n\
                                    let mark = ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| ui.add({widget_bare})).inner;\n\
                                    if caption.clicked() {{\n\
                                        self.{nome}.checked = !self.{nome}.checked;\n\
                                    }}\n\
                                    caption.clicked() || mark.clicked()\n\
                                }} else {{\n\
                                    ui.add({widget}).clicked()\n\
                                }}\n\
                            }}).inner\n\
                        }})\n\
                        .inner;\n\
{depois}\
                }}\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: CheckBox,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo,
        tipo_usado: "CheckBox",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_CHECKBOX: StateText = StateText {
    head: r#"
/// A `CheckBox` on the form. `checked` is what the user ticked.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
