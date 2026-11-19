//! Makepad component implementation for `Image`.
//!
//! It draws its picture: the control holds a makepad `<Image>` named
//! `img`, the sync hands it the bytes (`crawrad_runtime_makepad::picture`), and
//! the generated `App` remembers what is shown so the bytes are decoded only
//! when the picture changes. `stretch` / `proportional` / `center` choose the
//! fit and the alignment from the design values.

use super::codegen::{AppState, MakepadComponentCodegen};
use super::util;
use crate::identificador;
use crate::targets::makepad::layout;
use crawrad_ir::{Document, Node};

pub struct ImageCodegen;

impl MakepadComponentCodegen for ImageCodegen {
    fn class(&self) -> &'static str {
        "Image"
    }

    fn struct_definition(&self) -> String {
        struct_definition()
    }

    fn emit_default(&self, node: &Node) -> String {
        emit_default(node)
    }

    fn emit_sync(&self, doc: &Document, node: &Node, field: &str) -> String {
        emit_sync(doc, node, field)
    }

    fn emit_actions(
        &self,
        doc: &Document,
        node: &Node,
        field: &str,
        emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        emit_actions(doc, node, field, emitted)
    }

    fn app_state(&self, _node: &Node, field: &str) -> Result<Option<AppState>, crate::Erro> {
        Ok(Some(AppState {
            uses: "use crawrad_runtime_makepad::picture::Shown as CrawradPictureShown;\n"
                .to_string(),
            field: format!("    #[rust]\n    pub {field}_shown: CrawradPictureShown,\n"),
            handle: String::new(),
        }))
    }

    fn emit_dsl(
        &self,
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        emit_dsl(node, x, y, w, h)
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::image();
    let mut fields = util::common_control_fields();
    fields.extend(util::catalogue_fields(&desc, &["visible", "enabled"]));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct Image {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let mut inits = util::common_control_inits(node);

    let desc = crawrad_registry::components::image();
    inits.extend(util::catalogue_inits(node, &desc, &["visible", "enabled"]));
    let body = inits.join(", ");
    format!("Image {{ {body} }}")
}

pub fn emit_sync(_doc: &Document, _node: &Node, field: &str) -> String {
    format!(
        "        crawrad_runtime_makepad::picture::show(cx, &self.ui, id!({field}), self.form.{field}.picture.bytes(), &mut self.{field}_shown);\n\
         \x20       let _ = (\n\
         \x20           self.form.{field}.visible,\n\
         \x20           self.form.{field}.enabled,\n\
         \x20           self.form.{field}.stretch,\n\
         \x20           self.form.{field}.proportional,\n\
         \x20           self.form.{field}.center,\n\
         \x20           self.form.{field}.auto_size,\n\
         \x20           self.form.{field}.transparent,\n\
         \x20       );\n"
    )
}

pub fn emit_actions(
    _doc: &Document,
    _node: &Node,
    _field: &str,
    _emitted: &[&Node],
) -> Result<String, crate::Erro> {
    Ok(String::new())
}

pub fn emit_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let pos = layout::posicao(x, y);
    let name = identificador(node.name())?;
    let (fit, size, align) = fit_of(node);
    let mut dsl = String::new();
    dsl.push_str(&format!(
        "            {name} = <View> {{\n\
         \x20               {pos}\n\
         \x20               width: {w}, height: {h}\n\
         \x20               align: {align}\n\
         \x20               img = <Image> {{ {size}, fit: {fit} }}\n\
         \x20           }}\n"
    ));
    Ok(dsl)
}

#[crawrad_note::note("targets-makepad-components-image-fit-of")]
pub fn fit_of(node: &Node) -> (&'static str, &'static str, &'static str) {
    let on = |k: &str| matches!(node.props.get(k), Some(crawrad_ir::Value::Bool(true)));
    let (fit, size) = match (on("stretch"), on("proportional")) {
        (_, true) => ("Smallest", "width: Fill, height: Fill"),
        (true, false) => ("Stretch", "width: Fill, height: Fill"),
        (false, false) => ("Size", "width: Fit, height: Fit"),
    };
    let align = if on("center") {
        "{ x: 0.5, y: 0.5 }"
    } else {
        "{ x: 0.0, y: 0.0 }"
    };
    (fit, size, align)
}

pub fn emitir_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    emit_dsl(node, x, y, w, h)
}
