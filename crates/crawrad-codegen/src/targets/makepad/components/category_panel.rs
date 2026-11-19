//! Makepad component implementation for `CategoryPanel`: a `View`
//! with a `header` button over a `body` that holds the children. Folded, the
//! body goes out of sight and the panel shrinks to the header.

use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::{layout, state};
use crawrad_ir::{Document, Node};

pub struct CategoryPanelCodegen;

#[crawrad_note::note("targets-makepad-components-category-panel-header")]
const HEADER: f32 = 24.0;

impl MakepadComponentCodegen for CategoryPanelCodegen {
    fn class(&self) -> &'static str {
        "CategoryPanel"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("CategoryPanel").expect("CategoryPanel desc");
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct CategoryPanel {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("CategoryPanel").expect("CategoryPanel desc");
        let body = state::inits_of(node, desc)
            .into_iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("CategoryPanel {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, node: &Node, field: &str) -> String {
        let full = node.rect().map(|r| r.h).unwrap_or(160.0);
        // the header's text carries the fold mark, so a change in it is the
        // change of state: only then the body and the height follow
        format!(
            "        // folded to the header, the body out of sight\n\
             \x20       let header = format!(\"{{}} {{}}\", if self.form.{field}.collapsed {{ \"+\" }} else {{ \"-\" }}, self.form.{field}.text);\n\
             \x20       if self.ui.button(id!({field}.header)).text() != header {{\n\
             \x20           self.ui.button(id!({field}.header)).set_text(cx, &header);\n\
             \x20           self.ui.view(id!({field}.body)).set_visible(cx, !self.form.{field}.collapsed);\n\
             \x20           let height = if self.form.{field}.collapsed {{ {HEADER:.1} }} else {{ {full:.1} }};\n\
             \x20           self.ui.widget(id!({field})).apply_over(cx, live! {{ height: (height) }});\n\
             \x20       }}\n"
        )
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let call = |event: &str| -> Result<String, crate::Erro> {
            Ok(match layout::handler(node, event) {
                Some(h) => format!("                self.form.{}();\n", identificador(&h)?),
                None => String::new(),
            })
        };
        let on_collapsed = call("on_collapsed")?;
        let on_expanded = call("on_expanded")?;
        let dispatch = crate::targets::shared::toggle_branches(
            &format!("self.form.{field}.collapsed"),
            &on_collapsed,
            &on_expanded,
            "            ",
        );
        Ok(format!(
            "        if self.ui.button(id!({field}.header)).clicked(actions) {{\n\
             \x20           self.form.{field}.collapsed = !self.form.{field}.collapsed;\n\
             {dispatch}\
             \x20       }}\n"
        ))
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
        let mut dsl = format!(
            "            {name} = <View> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20               flow: Overlay\n\
             \x20               show_bg: true\n\
             \x20               draw_bg: {{ color: #1e1e2e }}\n\
             \x20               body = <View> {{\n\
             \x20                   width: Fill, height: Fill\n\
             \x20                   flow: Overlay\n"
        );
        dsl.push_str(&super::emit_children(doc, reg, node, "        ")?);
        dsl.push_str(&format!(
            "                }}\n\
             \x20               header = <Button> {{\n\
             \x20                   width: Fill, height: {HEADER:.1}\n\
             \x20                   align: {{x: 0.0, y: 0.5}}\n\
             \x20                   text: \"- {caption}\"\n\
             \x20               }}\n\
             \x20           }}\n"
        ));
        Ok(dsl)
    }
}
