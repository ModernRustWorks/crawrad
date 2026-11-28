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
        fields.push("    pub cells: crawrad_runtime::string_grid::Cells,".to_string());
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
        let text_rows = match node.props.get("cell_text") {
            Some(Value::Str(s)) if !s.is_empty() => s.split('\n').count() as i64,
            _ => 0,
        };
        let text_cols = match node.props.get("cell_text") {
            Some(Value::Str(s)) if !s.is_empty() => s
                .lines()
                .map(|l| l.split('\t').count() as i64)
                .max()
                .unwrap_or(0),
            _ => 0,
        };
        let col_count = match node.props.get("col_count") {
            Some(Value::Int(i)) => (*i).max(text_cols),
            _ => 5.max(text_cols),
        };
        let row_count = match node.props.get("row_count") {
            Some(Value::Int(i)) => (*i).max(text_rows),
            _ => 5.max(text_rows),
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
            // grid inits
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

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        format!(
            "        {{\n\
             \x20           if let Ok(app_guard) = app.lock() {{\n\
             \x20               let mut row_models = Vec::with_capacity(app_guard.form.{field}.row_count as usize);\n\
             \x20               for r in 0..app_guard.form.{field}.row_count as usize {{\n\
             \x20                   let mut col_strings = Vec::with_capacity(app_guard.form.{field}.col_count as usize);\n\
             \x20                   for c in 0..app_guard.form.{field}.col_count as usize {{\n\
             \x20                       col_strings.push(slint::SharedString::from(app_guard.form.{field}.cells.get(c, r)));\n\
             \x20                   }}\n\
             \x20                   row_models.push(slint::ModelRc::from(std::rc::Rc::new(slint::VecModel::from(col_strings))));\n\
             \x20               }}\n\
             \x20               window.set_{field}_cells(slint::ModelRc::from(std::rc::Rc::new(slint::VecModel::from(row_models))));\n\
             \x20           }}\n\
             \x20       }}\n"
        )
    }

    fn emit_actions(
        &self,
        // grid action target
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let on_click_call = util::format_handler_call(node, "on_click")?;
        Ok(format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           window.on_{field}_cell_selected(move |col, row| {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             \x20                   f.form.{field}.selected = Some((col as usize, row as usize));\n\
             {on_click_call}\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        ))
    }

    fn emit_slint(
        &self,
        // grid document reference
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        // grid layout bounds
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        // grid component name
        let name = identificador(node.name())?;
        let fixed_rows = match node.props.get("fixed_rows") {
            Some(Value::Int(i)) => *i,
            _ => 1,
        };
        let default_col_w = match node.props.get("default_col_width") {
            Some(Value::Int(i)) => *i,
            _ => 64,
        };
        let default_row_h = match node.props.get("default_row_height") {
            Some(Value::Int(i)) => *i,
            _ => 24,
        };
        let en = layout::is_enabled(node);

        let mut out = util::emit_container_header(node, x, y, w, h, 4.0, true)?;
        out.push_str("            VerticalLayout {\n");
        out.push_str("                alignment: start;\n");
        out.push_str("                spacing: 0px;\n");
        out.push_str("                padding: 0px;\n");
        out.push_str(&format!(
            "                for row[r] in root.{name}_cells: Rectangle {{\n\
             \x20                   height: {default_row_h}px;\n\
             \x20                   background: r < {fixed_rows} ? #f0f0f0 : #ffffff;\n\
             \x20                   HorizontalLayout {{\n\
             \x20                       alignment: start;\n\
             \x20                       spacing: 0px;\n\
             \x20                       padding: 0px;\n\
             \x20                       for cell[c] in row: Rectangle {{\n\
             \x20                           width: {default_col_w}px;\n\
             \x20                           border-width: 1px;\n\
             \x20                           border-color: #e0e0e0;\n\
             \x20                           Text {{\n\
             \x20                               text: cell;\n\
             \x20                               color: #000000;\n\
             \x20                               font-weight: r < {fixed_rows} ? 700 : 400;\n\
             \x20                               vertical-alignment: center;\n\
             \x20                               horizontal-alignment: center;\n\
             \x20                               overflow: elide;\n\
             \x20                           }}\n\
             \x20                           TouchArea {{\n"
        ));
        if !en {
            out.push_str("                                enabled: false;\n");
        }
        out.push_str(&format!(
            "                                clicked => {{ root.{name}_cell_selected(c, r); }}\n\
             \x20                           }}\n\
             \x20                       }}\n\
             \x20                   }}\n\
             \x20               }}\n\
             \x20           }}\n\
             \x20       }}\n"
        ));
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_cell_selected(int, int);")]
    }

    fn slint_properties(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("in property <[[string]]> {name}_cells;")]
    }
}
