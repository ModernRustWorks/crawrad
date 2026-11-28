//! Slint component implementation for `Grid`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct GridCodegen;

impl SlintComponentCodegen for GridCodegen {
    fn class(&self) -> &'static str {
        "Grid"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::grid::grid();
        let mut fields = util::common_control_fields();
        fields.push("    pub col_count: i64,".to_string());
        fields.push("    pub row_count: i64,".to_string());
        fields.push("    pub fixed_cols: i64,".to_string());
        fields.push("    pub fixed_rows: i64,".to_string());
        fields.push("    pub default_col_width: i64,".to_string());
        fields.push("    pub default_row_height: i64,".to_string());
        fields.push("    pub grid_lines: bool,".to_string());
        fields.push("    pub editing: bool,".to_string());
        fields.push("    pub cells: crawrad_runtime::string_grid::GridCells,".to_string());
        fields.push("    pub selected: Option<(usize, usize)>,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &[
                "visible",
                "enabled",
                "col_count",
                "row_count",
                "fixed_cols",
                "fixed_rows",
                "default_col_width",
                "default_row_height",
                "grid_lines",
                "editing",
            ],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Grid {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let mut inits = util::common_control_inits(node);
        let col_count = match node.props.get("col_count") {
            Some(Value::Int(i)) => *i,
            _ => 5,
        };
        let row_count = match node.props.get("row_count") {
            Some(Value::Int(i)) => *i,
            _ => 5,
        };
        let fixed_cols = match node.props.get("fixed_cols") {
            Some(Value::Int(i)) => *i,
            _ => 1,
        };
        let fixed_rows = match node.props.get("fixed_rows") {
            Some(Value::Int(i)) => *i,
            _ => 1,
        };
        let default_col_width = match node.props.get("default_col_width") {
            Some(Value::Int(i)) => *i,
            _ => 64,
        };
        let default_row_height = match node.props.get("default_row_height") {
            Some(Value::Int(i)) => *i,
            _ => 24,
        };
        let grid_lines = matches!(node.props.get("grid_lines"), None | Some(Value::Bool(true)));
        let editing = matches!(node.props.get("editing"), Some(Value::Bool(true)));

        inits.push(format!("col_count: {col_count}"));
        inits.push(format!("row_count: {row_count}"));
        inits.push(format!("fixed_cols: {fixed_cols}"));
        inits.push(format!("fixed_rows: {fixed_rows}"));
        inits.push(format!("default_col_width: {default_col_width}"));
        inits.push(format!("default_row_height: {default_row_height}"));
        inits.push(format!("grid_lines: {grid_lines}"));
        inits.push(format!("editing: {editing}"));
        inits.push(format!("cells: {}", crate::grid_cells_init(node)));
        inits.push("selected: None".to_string());

        let desc = crawrad_registry::components::grid::grid();
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &[
                "visible",
                "enabled",
                "col_count",
                "row_count",
                "fixed_cols",
                "fixed_rows",
                "default_col_width",
                "default_row_height",
                "grid_lines",
                "editing",
            ],
        ));
        let body = inits.join(", ");
        format!("Grid {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, _field: &str) -> String {
        String::new()
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let mut code = String::new();
        let on_click_call = match layout::event_handler(node, "on_click") {
            Some(h) => {
                let h_name = identificador(&h)?;
                format!("                    f.form.{h_name}();\n")
            }
            None => String::new(),
        };

        code.push_str(&format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           window.on_{field}_cell_selected(move |col, row| {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             \x20                   f.form.{field}.selected = Some((col as usize, row as usize));\n\
             {on_click_call}\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        ));

        Ok(code)
    }

    fn emit_slint(
        &self,
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        let name = identificador(node.name())?;
        let col_count = match node.props.get("col_count") {
            Some(Value::Int(i)) => *i,
            _ => 5,
        };
        let row_count = match node.props.get("row_count") {
            Some(Value::Int(i)) => *i,
            _ => 5,
        };
        let default_col_w = match node.props.get("default_col_width") {
            Some(Value::Int(i)) => *i,
            _ => 64,
        };
        let default_row_h = match node.props.get("default_row_height") {
            Some(Value::Int(i)) => *i,
            _ => 24,
        };
        let pos = layout::position(x, y);
        let vis = layout::is_visible(node);
        let en = layout::is_enabled(node);

        let mut out = String::new();
        out.push_str(&format!("        {name} := Rectangle {{\n"));
        out.push_str(&format!("            {pos}\n"));
        out.push_str(&format!("            width: {w:.1}px;\n"));
        out.push_str(&format!("            height: {h:.1}px;\n"));
        out.push_str("            border-width: 1px;\n");
        out.push_str("            border-color: #cccccc;\n");
        out.push_str("            border-radius: 4px;\n");
        out.push_str("            background: #ffffff;\n");
        out.push_str("            clip: true;\n");
        if !vis {
            out.push_str("            visible: false;\n");
        }
        out.push_str("            VerticalLayout {\n");

        for r in 0..row_count.min(8) {
            let bg = if r == 0 { "#f0f0f0" } else { "#ffffff" };
            out.push_str(&format!("                {name}_row_{r} := Rectangle {{\n"));
            out.push_str(&format!("                    height: {default_row_h}px;\n"));
            out.push_str(&format!("                    background: {bg};\n"));
            out.push_str("                    HorizontalLayout {\n");
            for c in 0..col_count.min(8) {
                out.push_str(&format!(
                    "                        {name}_cell_{r}_{c} := Rectangle {{\n"
                ));
                out.push_str(&format!(
                    "                            width: {default_col_w}px;\n"
                ));
                out.push_str("                            border-width: 1px;\n");
                out.push_str("                            border-color: #e0e0e0;\n");
                out.push_str("                            TouchArea {\n");
                if !en {
                    out.push_str("                                enabled: false;\n");
                }
                out.push_str(&format!(
                    "                                clicked => {{ root.{name}_cell_selected({c}, {r}); }}\n"
                ));
                out.push_str("                            }\n");
                out.push_str("                        }\n");
            }
            out.push_str("                    }\n");
            out.push_str("                }\n");
        }

        out.push_str("            }\n");
        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_cell_selected(int, int);")]
    }
}
