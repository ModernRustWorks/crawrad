//! Makepad component implementation for `Pagination`.

use super::codegen::{AppState, MakepadComponentCodegen};
use crate::identificador;
use crate::targets::makepad::{layout, state};
use crawrad_ir::{Document, Node};

pub struct PaginationCodegen;

impl MakepadComponentCodegen for PaginationCodegen {
    fn class(&self) -> &'static str {
        "Pagination"
    }

    fn whole_look_widget(&self) -> Option<&'static str> {
        Some("crawrad_pagination")
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Pagination").expect("Pagination desc");
        // the tab order is the form's, as for every control here
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Pagination {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Pagination").expect("Pagination desc");
        let body = state::inits_of(node, desc)
            .into_iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("Pagination {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        format!(
            "        {{\n\
             \x20           let pagination = self.ui.crawrad_pagination(id!({field}));\n\
             \x20           pagination.set_page_count(cx, self.form.{field}.page_count);\n\
             \x20           pagination.set_page_index(cx, self.form.{field}.page_index);\n\
             \x20           pagination.set_max_buttons(cx, self.form.{field}.max_buttons);\n\
             \x20           pagination.set_show_arrows(cx, self.form.{field}.show_arrows);\n\
             \x20           pagination.set_enabled(cx, self.form.{field}.enabled);\n\
             \x20           pagination.set_visible(cx, self.form.{field}.visible);\n\
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
            format!("            let changed = self.form.{field}.page_index != i as i64;\n")
        } else {
            String::new()
        };

        let mut code = String::new();
        code.push_str(&format!(
            "        if let Some(i) = self.ui.crawrad_pagination(id!({field})).changed(actions) {{\n\
             {decl_changed}\
             \x20           self.form.{field}.page_index = i as i64;\n\
             {change_check}\
             \x20       }}\n"
        ));

        Ok(code)
    }

    fn app_state(&self, _node: &Node, _field: &str) -> Result<Option<AppState>, crate::Erro> {
        Ok(Some(AppState {
            uses: "use crawrad_runtime_makepad::pagination::CrawradPaginationWidgetRefExt;\n"
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
        Ok(format!(
            "            {name} = <CrawradPagination> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20           }}\n"
        ))
    }
}
