use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador, literal};

pub struct PageControlCodegen;

impl EguiComponentCodegen for PageControlCodegen {
    fn class(&self) -> &'static str {
        "PageControl"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_PAGECONTROL)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }
}

pub struct TabSheetCodegen;

impl EguiComponentCodegen for TabSheetCodegen {
    fn class(&self) -> &'static str {
        "TabSheet"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_TABSHEET)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir_tabsheet(ctx)
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("PageControl").expect("PageControl descriptor");
    let doc = ctx.doc;

    // Captions come from page STATE, not from a constant with
    // frozen text: `self.tab_dados.caption = "Outra".into()` must
    // appear on the strip. While the strip read a constant, the form's `TabSheet`
    // fields were not read by anyone — it was `dead_code`, and was
    // silent.
    let paginas: Vec<String> = n
        .children
        .iter()
        .filter_map(|c| doc.node(c))
        .filter(|c| c.class == "TabSheet")
        .map(|c| identificador(c.name()))
        .collect::<Result<_, Erro>>()?;
    let n_paginas = paginas.len();
    // Each tab is its caption and the sheet's hint
    let legendas: String = paginas
        .iter()
        .map(|p| {
            format!(
                "(self.{p}.text.as_str(), if CRAWRAD_FORM_SHOW_HINT && self.{p}.show_hint {{ self.{p}.hint.as_str() }} else {{ \"\" }})"
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    // A disabled page does not open — this is `TTabSheet.Enabled` in a classic RAD IDE. The
    // strip is drawn by `page_control`, which is shared text with the
    // preview and does not receive this; what can be done on this side is refusing the
    // switch, and that is what is done.
    let shown: String = paginas
        .iter()
        .map(|p| format!("self.{p}.tab_visible"))
        .collect::<Vec<_>>()
        .join(", ");
    let habilitadas: String = paginas
        .iter()
        .map(|p| format!("self.{p}.enabled"))
        .collect::<Vec<_>>()
        .join(", ");
    let depois = match handler(n, "on_change") {
        Some(h) => format!("                self.{}();\n", identificador(&h)?),
        None => String::new(),
    };
    // `OnChanging` can say no, and the page stays
    let veto = match handler(n, "on_changing") {
        Some(h) => format!(
            "            if changed && allowed {{\n\
             \x20               let mut allow_change = true;\n\
             \x20               self.{}(&mut allow_change);\n\
             \x20               allowed = allow_change;\n\
             \x20           }}\n",
            identificador(&h)?
        ),
        None => String::new(),
    };
    let mut_allowed = if veto.is_empty() { "" } else { "mut " };
    let corpo = format!(
        "        if self.{nome}.visible {{\n\
         \x20           let captions: [(&str, &str); {n_paginas}] = [{legendas}];\n\
         \x20           let selectable: [bool; {n_paginas}] = [{habilitadas}];\n\
         \x20           let shown: [bool; {n_paginas}] = [{shown}];\n\
         \x20           let bottom = self.{nome}.tab_position == \"tpBottom\";\n\
         \x20           let is_enabled = self.{nome}.enabled;\n\
         \x20           let mut page = self.{nome}.active_page;\n\
         \x20           let changed = ui\n\
         \x20               .add_enabled_ui(is_enabled, |ui| {{\n\
         \x20                   page_control(ui, {}, r, &captions, &shown, bottom, &mut page)\n\
         \x20               }})\n\
         \x20               .inner;\n\
         \x20           let {mut_allowed}allowed = selectable.get(page).copied().unwrap_or(true);\n\
         {veto}\
         \x20           if allowed {{\n\
         \x20               // Also when nothing changed: `page_control` pulls an index out of\n\
         \x20               // range back to the first page, and that correction has to\n\
         \x20               // reach the state.\n\
         \x20               self.{nome}.active_page = page;\n\
         \x20           }}\n\
         \x20           if changed && allowed {{\n\
         {depois}\
         \x20           }}\n\
         \x20       }}\n",
        literal(nome),
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: PageControl,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo,
        tipo_usado: "PageControl",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

pub fn emitir_tabsheet(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("TabSheet").expect("TabSheet descriptor");

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: TabSheet,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo: String::new(),
        tipo_usado: "TabSheet",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_PAGECONTROL: StateText = StateText {
    head: r#"
/// A `PageControl` on the form — tabbed pages.
///
/// `active_page` is which page is open, counting from zero. It is classic RAD's
/// `ActivePageIndex`, and like there it is saved with the form: the page that
/// was open when you designed it is the page the app starts on.
#[derive(Clone, Debug)]
"#,
    tail: "",
};

const ESTADO_TABSHEET: StateText = StateText {
    head: r#"
/// A `TabSheet` on the form — one page of a `PageControl`.
///
/// It draws nothing of its own: the page control draws the strip and the body.
/// What lives on the page are fields of the form, emitted right after, each one
/// guarded by the active page.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
