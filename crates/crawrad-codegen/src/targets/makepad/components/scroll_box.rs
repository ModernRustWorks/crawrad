//! Makepad component implementation for `ScrollBox`.

use super::codegen::MakepadComponentCodegen;
use super::util::{self, FIELD_ENABLED, FIELD_VISIBLE};
use crate::identificador;
use crate::targets::makepad::layout;
use crawrad_ir::{Document, Node, Value};

pub struct ScrollBoxCodegen;

impl MakepadComponentCodegen for ScrollBoxCodegen {
    fn class(&self) -> &'static str {
        "ScrollBox"
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

    fn emit_dsl(
        &self,
        doc: &Document,
        reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        emit_dsl(doc, reg, node, x, y, w, h)
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::scroll_box();
    let mut fields = vec![
        FIELD_VISIBLE.to_string(),
        FIELD_ENABLED.to_string(),
        "    pub auto_scroll: bool,".to_string(),
        "    pub horz_scroll_bar: bool,".to_string(),
        "    pub vert_scroll_bar: bool,".to_string(),
        "    pub border_width: i64,".to_string(),
    ];
    fields.extend(util::catalogue_fields(
        &desc,
        &[
            "visible",
            "enabled",
            "auto_scroll",
            "horz_scroll_bar",
            "vert_scroll_bar",
            "border_width",
        ],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct ScrollBox {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let vis = layout::is_visible(node);
    let en = layout::is_enabled(node);
    let auto = match node.props.get("auto_scroll") {
        Some(Value::Bool(b)) => *b,
        _ => true,
    };
    let horz = match node.props.get("horz_scroll_bar") {
        Some(Value::Bool(b)) => *b,
        _ => true,
    };
    let vert = match node.props.get("vert_scroll_bar") {
        Some(Value::Bool(b)) => *b,
        _ => true,
    };
    let bw = match node.props.get("border_width") {
        Some(Value::Int(i)) => *i,
        _ => 1,
    };
    let mut inits = vec![
        format!("visible: {vis}"),
        format!("enabled: {en}"),
        format!("auto_scroll: {auto}"),
        format!("horz_scroll_bar: {horz}"),
        format!("vert_scroll_bar: {vert}"),
        format!("border_width: {bw}"),
    ];
    let desc = crawrad_registry::components::scroll_box();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &[
            "visible",
            "enabled",
            "auto_scroll",
            "horz_scroll_bar",
            "vert_scroll_bar",
            "border_width",
        ],
    ));
    let body = inits.join(", ");
    format!("ScrollBox {{ {body} }}")
}

pub fn emit_sync(_doc: &Document, _node: &Node, field: &str) -> String {
    // `border_size` is a shader uniform in Makepad 1.0 — only the DSL value
    // (written from the design in `emit_dsl`) draws; setting it at run time was a
    // silent no-op that logged `no matching field` on every redraw.
    format!("        let _ = self.form.{field}.border_width;\n")
}

pub fn emit_actions(
    _doc: &Document,
    _node: &Node,
    _field: &str,
    _emitted: &[&Node],
) -> Result<String, crate::Erro> {
    Ok(String::new())
}

pub fn emit_dsl(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    node: &Node,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Result<String, crate::Erro> {
    let pos = layout::posicao(x, y);
    let name = identificador(node.name())?;
    // `AutoScroll` off closes both axes, the rule the designer and the
    // egui app follow too
    let flag = |k: &str| !matches!(node.props.get(k), Some(Value::Bool(false)));
    let axes = crawrad_ir::scrolling::scroll_box_axes(
        flag("auto_scroll"),
        flag("horz_scroll_bar"),
        flag("vert_scroll_bar"),
    );
    let (horz, vert) = (axes.horizontal, axes.vertical);
    let bw = match node.props.get("border_width") {
        Some(Value::Int(i)) => (*i).max(0),
        _ => 1,
    };
    let mut dsl = String::new();
    // a `RectView`, the View that draws a border (`border_size`,
    // makepad-widgets 1.0.0 `view_ui.rs:112`); the plain View has none
    dsl.push_str(&format!(
        "            {name} = <RectView> {{\n\
         \x20               {pos}\n\
         \x20               width: {w}, height: {h}\n\
         \x20               flow: Overlay\n\
         \x20               show_bg: true\n\
         \x20               draw_bg: {{ color: #1e1e2e, border_size: {bw}.0, border_color: #45475a }}\n\
         \x20               scroll_bars: <ScrollBars> {{\n\
         \x20                   show_scroll_x: {horz}\n\
         \x20                   show_scroll_y: {vert}\n\
         \x20               }}\n"
    ));

    dsl.push_str(&super::emit_children(doc, reg, node, "    ")?);
    dsl.push_str(&medida_do_conteudo(doc, node, w, h));

    dsl.push_str("            }\n");
    Ok(dsl)
}

#[crawrad_note::note("targets-makepad-components-scroll-box-medida-do-conteudo")]
fn medida_do_conteudo(doc: &Document, node: &Node, w: f32, h: f32) -> String {
    let (cw, ch) = crawrad_ir::scrolling::content_extent(
        (0.0, 0.0),
        node.children
            .iter()
            .filter_map(|c| doc.node(c))
            .filter_map(|c| c.rect())
            .map(|r| (r.x, r.y, r.w, r.h)),
    );
    let precisa = crawrad_ir::scrolling::bars_needed((w, h), (cw, ch));
    if !precisa.vertical && !precisa.horizontal {
        return String::new();
    }
    format!(
        "                // The content SIZE, so the container knows there is more\n\
         \x20               // beyond the window. In `flow: Overlay` Makepad measures a\n\
         \x20               // child by its size and ignores its margin, so without this\n\
         \x20               // the box never scrolls.\n\
         \x20               {name}_content = <View> {{ width: {cw}, height: {ch} }}\n",
        name = crate::identificador(node.name()).unwrap_or_default(),
    )
}

pub fn emitir_dsl(
    reg: &crawrad_registry::Registry,
    node: &Node,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Result<String, crate::Erro> {
    let dummy_doc = Document::new("Form", "Form");
    emit_dsl(&dummy_doc, reg, node, x, y, w, h)
}
