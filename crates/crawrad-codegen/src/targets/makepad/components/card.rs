use super::codegen::AppState;
use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::layout;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node};

pub struct CardCodegen;

impl MakepadComponentCodegen for CardCodegen {
    fn class(&self) -> &'static str {
        "Card"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Card").expect("Card desc");
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Card {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Card").expect("Card desc");
        let body = state::inits_of(node, desc)
            .into_iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("Card {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        format!(
            "        // the Card is the palette's `CrawradFormCard` widget, as in the IDE\n\
             \x20       let card = self.ui.crawrad_form_card(id!({field}));\n\
             \x20       card.set_header(cx, &self.form.{field}.text, &self.form.{field}.subtitle, self.form.{field}.show_header);\n\
             \x20       card.set_look(cx, self.form.{field}.corner_radius, self.form.{field}.elevation);\n"
        )
    }

    #[crawrad_note::note("targets-makepad-components-card-cardcodegen-app-state")]
    fn app_state(&self, _node: &Node, _field: &str) -> Result<Option<AppState>, crate::Erro> {
        Ok(Some(AppState {
            uses: "use crawrad_runtime_makepad::card::CrawradFormCardWidgetRefExt;\n".to_string(),
            ..AppState::default()
        }))
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let mut code = String::new();
        if let Some(h) = layout::handler(node, "on_click") {
            let h_name = identificador(&h)?;
            code.push_str(&format!(
                "        if self.ui.crawrad_form_card(id!({field})).clicked(actions) {{\n\
                 \x20           self.form.{h_name}();\n\
                 \x20       }}\n"
            ));
        }
        Ok(code)
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
        let pos = layout::posicao(x, y);
        let name = identificador(node.name())?;
        let caption = layout::text(node)
            .replace('\\', "\\\\")
            .replace('"', "\\\"");
        let subtitle = match node.props.get("subtitle") {
            Some(crawrad_ir::Value::Str(s)) => s.as_str(),
            _ => "",
        }
        .replace('\\', "\\\\")
        .replace('"', "\\\"");
        let corner_radius = match node.props.get("corner_radius") {
            Some(crawrad_ir::Value::Int(i)) => *i,
            _ => 8,
        };
        let show_header = !matches!(
            node.props.get("show_header"),
            Some(crawrad_ir::Value::Bool(false))
        );
        // The first frame shows the design's header and radius; the shadow
        // (elevation) comes with the first sync — a `vec2(..)` in the static DSL
        // is a silent no-op.
        let header_h = crawrad_runtime_makepad_header_height(&subtitle);
        let mut dsl = format!(
            "            {name} = <CrawradFormCard> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20               draw_bg: {{ border_radius: {corner_radius}.0 }}\n\
             \x20               header = {{\n\
             \x20                   visible: {show_header}, height: {header_h:.1}\n\
             \x20                   labels = {{ title = {{ text: \"{caption}\" }} subtitle = {{ text: \"{subtitle}\" }} }}\n\
             \x20                   divider = {{ margin: {{ top: {divider:.1} }} }}\n\
             \x20               }}\n",
            divider = header_h - 1.0
        );
        if layout::handler(node, "on_click").is_some() {
            dsl.push_str("                cursor: Default\n");
        }
        dsl.push_str(&super::emit_children(doc, reg, node, "    ")?);
        dsl.push_str("            }\n");
        Ok(dsl)
    }
}

#[crawrad_note::note("targets-makepad-components-card-crawrad-runtime-makepad-header-height")]
fn crawrad_runtime_makepad_header_height(subtitle: &str) -> f32 {
    if subtitle.is_empty() { 28.0 } else { 42.0 }
}
