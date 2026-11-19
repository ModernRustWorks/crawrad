//! Makepad component implementation for `Breadcrumb`.

use super::codegen::{AppState, MakepadComponentCodegen};
use crate::identificador;
use crate::targets::makepad::{layout, state};
use crawrad_ir::{Document, Item, Node, Value};

pub struct BreadcrumbCodegen;

impl MakepadComponentCodegen for BreadcrumbCodegen {
    fn class(&self) -> &'static str {
        "Breadcrumb"
    }

    fn whole_look_widget(&self) -> Option<&'static str> {
        Some("crawrad_breadcrumb")
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Breadcrumb").expect("Breadcrumb desc");
        // the tab order is the form's, as for every control here
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Breadcrumb {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Breadcrumb").expect("Breadcrumb desc");
        let body = state::inits_of(node, desc)
            .into_iter()
            .map(|(n, v)| {
                if n == "items" {
                    let items = node
                        .props
                        .get("items")
                        .map(Item::list_from)
                        .unwrap_or_default();
                    let quoted: Vec<String> =
                        items.iter().map(|it| crate::literal(&it.text)).collect();
                    format!("items: Strings::from_slice(&[{}])", quoted.join(", "))
                } else if n == "separator" {
                    let sep = match node.props.get("separator") {
                        Some(Value::Str(s)) => s.as_str(),
                        _ => "›",
                    };
                    format!("separator: {}.to_string()", crate::literal(sep))
                } else {
                    format!("{n}: {v}")
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!("Breadcrumb {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        let forget = crate::targets::shared::forget_selection_past_end(
            &format!("self.form.{field}"),
            crate::targets::shared::Selection::Signed,
        );
        format!(
            "{forget}        {{\n\
             \x20           let breadcrumb = self.ui.crawrad_breadcrumb(id!({field}));\n\
             \x20           breadcrumb.set_items(cx, &self.form.{field}.items);\n\
             \x20           breadcrumb.set_separator(cx, &self.form.{field}.separator);\n\
             \x20           breadcrumb.set_item_index(cx, self.form.{field}.item_index);\n\
             \x20           breadcrumb.set_enabled(cx, self.form.{field}.enabled);\n\
             \x20           breadcrumb.set_style(cx, &self.form.{field}.style);\n\
             \x20           let sc = self.form.{field}.segment_color;\n\
             \x20           breadcrumb.set_segment_color(cx, vec4(sc.r as f32 / 255.0, sc.g as f32 / 255.0, sc.b as f32 / 255.0, sc.a as f32 / 255.0));\n\
             \x20           let cc = self.form.{field}.current_color;\n\
             \x20           breadcrumb.set_current_color(cx, vec4(cc.r as f32 / 255.0, cc.g as f32 / 255.0, cc.b as f32 / 255.0, cc.a as f32 / 255.0));\n\
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
        let mut on_click_call = String::new();
        if let Some(h) = layout::handler(node, "on_click") {
            on_click_call = format!("                self.form.{}();\n", identificador(&h)?);
        }
        let mut on_change_call = String::new();
        if let Some(h) = layout::handler(node, "on_change") {
            on_change_call = format!("                self.form.{}();\n", identificador(&h)?);
        }

        let change_check = if !on_change_call.is_empty() {
            format!(
                "            if changed {{\n{}\
                 \x20           }}\n",
                on_change_call
            )
        } else {
            String::new()
        };

        let decl_changed = if !on_change_call.is_empty() {
            format!("            let changed = self.form.{field}.item_index != i as i64;\n")
        } else {
            String::new()
        };

        let mut code = String::new();

        if !on_click_call.is_empty() || !on_change_call.is_empty() {
            code.push_str(&format!(
                "        if let Some(i) = self.ui.crawrad_breadcrumb(id!({field})).clicked(actions) {{\n\
                 {decl_changed}\
                 \x20           self.form.{field}.item_index = i as i64;\n\
                 {on_click_call}\
                 {change_check}\
                 \x20       }}\n"
            ));
        } else {
            code.push_str(&format!(
                "        if let Some(i) = self.ui.crawrad_breadcrumb(id!({field})).clicked(actions) {{\n\
                 \x20           self.form.{field}.item_index = i as i64;\n\
                 \x20       }}\n"
            ));
        }

        Ok(code)
    }

    fn app_state(&self, _node: &Node, _field: &str) -> Result<Option<AppState>, crate::Erro> {
        Ok(Some(AppState {
            uses: "use crawrad_runtime_makepad::breadcrumb::CrawradBreadcrumbWidgetRefExt;\n"
                .to_string(),
            ..AppState::default()
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
        let pos = layout::posicao(x, y);
        let name = identificador(node.name())?;
        let style = match node.props.get("style") {
            Some(Value::Enum(s)) | Some(Value::Str(s)) => s.as_str(),
            _ => "bsText",
        };
        let seg_color = match node.props.get("segment_color") {
            Some(Value::Color(c)) => crate::targets::shared::dsl_colour(&c.to_hex()),
            _ => "#45464B".to_string(),
        };
        let cur_color = match node.props.get("current_color") {
            Some(Value::Color(c)) => crate::targets::shared::dsl_colour(&c.to_hex()),
            _ => "#E0612B".to_string(),
        };
        Ok(format!(
            "            {name} = <CrawradBreadcrumb> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20               style: \"{style}\"\n\
             \x20               segment_color: {seg_color}\n\
             \x20               current_color: {cur_color}\n\
             \x20           }}\n"
        ))
    }
}
