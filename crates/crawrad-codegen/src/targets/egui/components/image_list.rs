use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::Erro;
use crawrad_ir::Value;

pub struct ImageListCodegen;

impl EguiComponentCodegen for ImageListCodegen {
    fn class(&self) -> &'static str {
        "ImageList"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_IMAGELIST)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir_image_list(ctx)
    }
}

pub fn extrair_icones(n: &crawrad_ir::Node) -> Vec<(String, String)> {
    match n.props.get("images_data") {
        Some(Value::List(itens)) => itens
            .iter()
            .filter_map(|item| {
                if let Value::Map(m) = item {
                    let nome = match m.get("name") {
                        Some(Value::Str(s)) => s.clone(),
                        _ => String::new(),
                    };
                    let symbol = match m.get("symbol") {
                        Some(Value::Str(s)) => s.clone(),
                        _ => String::new(),
                    };
                    Some((nome, symbol))
                } else {
                    None
                }
            })
            .collect(),
        _ => Vec::new(),
    }
}

pub fn emitir_image_list(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("ImageList").expect("ImageList descriptor");

    let corpo = format!(
        "        let _ = (self.{nome}.image_width, self.{nome}.image_height, &self.{nome}.images);\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: ImageList,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo,
        tipo_usado: "ImageList",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_IMAGELIST: StateText = StateText {
    head: r#"
/// An item in an `ImageList`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ImageItem {
    pub name: String,
    pub symbol: String,
}

/// An `ImageList` on the form — classic RAD `TImageList`.
#[derive(Clone, Debug, Default)]
"#,
    tail: "",
};
