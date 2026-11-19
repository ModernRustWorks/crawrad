//! Declarative data binding code emission for generated forms (ADR 0041, ADR 0036).

use crate::{Erro, literal};
use crawrad_ir::binding::BindingMode;
use crawrad_ir::{Document, Node};
use crawrad_note::note;

#[note("targets-data-binding-boundcontrol")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundControl {
    /// Node id in the document.
    pub node_id: String,
    /// Identifier in generated code (e.g. `edit_name`).
    pub node_name: String,
    /// Component class name (`"Edit"` or `"CheckBox"`).
    pub class: String,
    /// Bound property name (`"text"` or `"checked"`).
    pub prop: String,
    /// Target column name in the dataset schema.
    pub column: String,
    /// Binding synchronization mode.
    pub mode: BindingMode,
    /// Identifier of the bound `Dataset` component.
    pub dataset_name: String,
    /// Identifier of the `Connection` supplying the connection.
    pub engine_name: String,
}

#[note("targets-data-binding-datasetaction")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetAction {
    /// Action node id in the document.
    pub node_id: String,
    /// Identifier in generated code (e.g. `act_next`).
    pub action_name: String,
    /// Operation name (e.g. `"First"`, `"Next"`, `"Post"`, etc.).
    pub operation: String,
    /// Identifier of the bound `Dataset` component.
    pub dataset_name: String,
    /// Identifier of the `Connection` supplying the connection.
    pub engine_name: String,
    /// Whether the action has an `on_execute` handler.
    pub has_on_execute: bool,
    /// Whether the action has an `on_update` handler.
    pub has_on_update: bool,
}

#[note("targets-data-binding-dataset-action-of")]
pub fn dataset_action_of(doc: &Document, act_node: &Node) -> Result<Option<DatasetAction>, Erro> {
    if act_node.class != "Action" {
        return Ok(None);
    }
    let op = match act_node.props.get("dataset_operation") {
        Some(crawrad_ir::Value::Enum(s)) | Some(crawrad_ir::Value::Str(s))
            if s != "None" && !s.is_empty() =>
        {
            s.as_str()
        }
        _ => return Ok(None),
    };
    let dataset_ref = match act_node.props.get("dataset") {
        Some(crawrad_ir::Value::Ref(id)) | Some(crawrad_ir::Value::Str(id))
            if !id.trim().is_empty() =>
        {
            id.as_str()
        }
        _ => return Ok(None),
    };
    let Some(conn) = crate::targets::shared::bound_dataset_connection(doc, dataset_ref)? else {
        return Ok(None);
    };
    let action_name = crate::identificador(act_node.name())?;
    let has_on_execute = act_node
        .events
        .get("on_execute")
        .is_some_and(|h| !h.trim().is_empty());
    let has_on_update = act_node
        .events
        .get("on_update")
        .is_some_and(|h| !h.trim().is_empty());
    Ok(Some(DatasetAction {
        node_id: act_node.id.clone(),
        action_name,
        operation: op.to_string(),
        dataset_name: conn.dataset_name,
        engine_name: conn.engine_name,
        has_on_execute,
        has_on_update,
    }))
}

#[note("targets-data-binding-collect-dataset-actions")]
pub fn collect_dataset_actions(
    doc: &Document,
    emitted: &[&Node],
) -> Result<Vec<DatasetAction>, Erro> {
    let mut actions = Vec::new();
    for node in emitted {
        if let Some(da) = dataset_action_of(doc, node)? {
            actions.push(da);
        }
    }
    Ok(actions)
}

#[note("targets-data-binding-dataset-action-enabled-expr")]
pub fn dataset_action_enabled_expr(operation: &str, adapter: &str) -> String {
    match operation {
        "First" | "Prior" => format!(
            "{adapter}.state().get().is_open() && {adapter}.collection().cursor().is_some() && {adapter}.collection().cursor() != Some(0)"
        ),
        "Next" | "Last" => format!(
            "{adapter}.state().get().is_open() && {adapter}.collection().cursor().is_some_and(|c| c + 1 < {adapter}.collection().len())"
        ),
        "Insert" => format!(
            "matches!({adapter}.state().get(), crawrad_binding::DatasetState::Browse | crawrad_binding::DatasetState::Inactive)"
        ),
        "Edit" | "Delete" => format!(
            "{adapter}.state().get() == crawrad_binding::DatasetState::Browse && {adapter}.collection().cursor().is_some()"
        ),
        "Post" | "Cancel" => format!("{adapter}.state().get().can_modify()"),
        "Refresh" => format!("{adapter}.state().get().is_open()"),
        _ => "false".to_string(),
    }
}

#[note("targets-data-binding-dataset-operation-call")]
pub fn dataset_operation_call(operation: &str, adapter_expr: &str, indent: &str) -> String {
    let call = match operation {
        "First" => "adapter.collection().first()",
        "Prior" => "adapter.collection().previous()",
        "Next" => "adapter.collection().next()",
        "Last" => "adapter.collection().last()",
        "Insert" => "adapter.insert()",
        "Edit" => "adapter.edit()",
        "Delete" => "adapter.delete()",
        "Post" => "adapter.post()",
        "Cancel" => "adapter.cancel()",
        "Refresh" => "adapter.refresh()",
        _ => return String::new(),
    };
    // one statement, so the caller's `if` around it never nests a second one
    format!("{indent}let _ = {adapter_expr}.as_ref().map(|adapter| {call});\n")
}

#[note("targets-data-binding-collect-bound-controls")]
pub fn collect_bound_controls(
    doc: &Document,
    emitted: &[&Node],
) -> Result<Vec<BoundControl>, Erro> {
    let mut controls = Vec::new();
    for node in emitted {
        let (prop, expected_class) = match node.class.as_str() {
            "Edit" => ("text", "Edit"),
            "CheckBox" => ("checked", "CheckBox"),
            _ => continue,
        };
        let Some(binding) = node.bindings.get(prop) else {
            continue;
        };
        let Some(ref col) = binding.field else {
            continue;
        };
        if col.trim().is_empty() {
            continue;
        }
        let Some(conn) = crate::targets::shared::bound_dataset_connection(doc, &binding.source)?
        else {
            continue;
        };
        let node_name = crate::identificador(node.name())?;
        controls.push(BoundControl {
            node_id: node.id.clone(),
            node_name,
            class: expected_class.to_string(),
            prop: prop.to_string(),
            column: col.clone(),
            mode: binding.mode,
            dataset_name: conn.dataset_name,
            engine_name: conn.engine_name,
        });
    }
    Ok(controls)
}

#[note("targets-data-binding-active-datasets")]
pub fn active_datasets(
    controls: &[BoundControl],
    actions: &[DatasetAction],
) -> Vec<(String, String)> {
    let mut list: Vec<(String, String)> = controls
        .iter()
        .map(|c| (c.dataset_name.clone(), c.engine_name.clone()))
        .chain(
            actions
                .iter()
                .map(|a| (a.dataset_name.clone(), a.engine_name.clone())),
        )
        .collect();
    list.sort();
    list.dedup();
    list
}

#[note("targets-data-binding-distinct-datasets")]
pub fn distinct_datasets(controls: &[BoundControl]) -> Vec<(String, String)> {
    active_datasets(controls, &[])
}

#[note("targets-data-binding-egui-fields-all")]
pub fn egui_fields_all(controls: &[BoundControl], actions: &[DatasetAction]) -> (String, String) {
    let datasets = active_datasets(controls, actions);
    if datasets.is_empty() && controls.is_empty() {
        return (String::new(), String::new());
    }
    let mut fields = String::new();
    let mut defaults = String::new();

    for (dataset, _) in datasets {
        fields.push_str(&format!(
            "    pub {dataset}_adapter: Option<crawrad_binding::DatasetAdapter>,\n\
             \x20   pub {dataset}_opened: bool,\n\
             \x20   pub {dataset}_error: Option<String>,\n"
        ));
        defaults.push_str(&format!(
            "            {dataset}_adapter: None,\n\
             \x20           {dataset}_opened: false,\n\
             \x20           {dataset}_error: None,\n"
        ));
    }

    for c in controls {
        let name = &c.node_name;
        let prop = &c.prop;
        fields.push_str(&format!("    pub {name}_{prop}_bound: String,\n"));
        defaults.push_str(&format!(
            "            {name}_{prop}_bound: String::new(),\n"
        ));
    }

    (fields, defaults)
}

#[note("targets-data-binding-egui-fields")]
pub fn egui_fields(controls: &[BoundControl]) -> (String, String) {
    egui_fields_all(controls, &[])
}

#[note("targets-data-binding-open-datasets-code")]
pub fn open_datasets_code(datasets: &[(String, String)], form: &str) -> String {
    let mut code = String::new();
    for (dataset, engine) in datasets {
        let open_expr = crate::targets::shared::bound_dataset_open(
            &format!("&{form}{engine}.url"),
            &format!("&{form}{dataset}.table"),
        );
        code.push_str(&format!(
            "        let {dataset}_just_opened = !self.{dataset}_opened;\n\
             \x20       if {dataset}_just_opened {{\n\
             \x20           self.{dataset}_opened = true;\n\
             \x20           match {open_expr} {{\n\
             \x20               Ok(adapter) => self.{dataset}_adapter = Some(adapter),\n\
             \x20               Err(err) => self.{dataset}_error = Some(err),\n\
             \x20           }}\n\
             \x20       }}\n"
        ));
    }
    code
}

#[note("targets-data-binding-model-to-widget")]
fn model_to_widget(
    controls: &[BoundControl],
    form: &str,
    push: impl Fn(&BoundControl) -> String,
) -> String {
    let mut code = String::new();
    for c in controls {
        let name = &c.node_name;
        let prop = &c.prop;
        let col = literal(&c.column);
        let ds = &c.dataset_name;
        // a check box compares in ONE spelling, "true"/"false", so a "1" from the
        // database is not written back as "true" the moment it is shown
        let (normalize, assign) = if c.class == "Edit" {
            (
                String::new(),
                format!("                {form}{name}.text = val.clone();\n"),
            )
        } else {
            (
                "            let val = (val.eq_ignore_ascii_case(\"true\") || val == \"1\").to_string();\n"
                    .to_string(),
                format!(
                    "                let is_checked = val == \"true\";\n\
                     \x20               {form}{name}.checked = is_checked;\n"
                ),
            )
        };
        let extra = push(c);
        // OneTime reads only on the open: the condition rides in the same `if`
        // (a nested one is what Clippy's `collapsible_if` refuses in the app)
        let adapter = if c.mode == BindingMode::OneTime {
            format!("self.{ds}_adapter.as_ref().filter(|_| {ds}_just_opened)")
        } else {
            format!("self.{ds}_adapter.as_ref()")
        };
        code.push_str(&format!(
            "        if let Some(adapter) = {adapter} {{\n\
             \x20           let val = adapter.field_or_create({col}).get();\n\
             {normalize}\
             \x20           if val != self.{name}_{prop}_bound {{\n\
             {assign}\
             {extra}\
             \x20               self.{name}_{prop}_bound = val;\n\
             \x20           }}\n\
             \x20       }}\n"
        ));
    }
    code
}

#[note("targets-data-binding-widget-to-model")]
fn widget_to_model(c: &BoundControl, form: &str, indent: &str) -> String {
    if !matches!(c.mode, BindingMode::TwoWay | BindingMode::OneWayToSource) {
        return String::new();
    }
    let name = &c.node_name;
    let prop = &c.prop;
    let col = literal(&c.column);
    let ds = &c.dataset_name;
    let current = if c.class == "Edit" {
        format!("{form}{name}.text.clone()")
    } else {
        format!("{form}{name}.checked.to_string()")
    };
    format!(
        "{indent}let current = {current};\n\
         {indent}if current != self.{name}_{prop}_bound {{\n\
         {indent}    if let Some(ref adapter) = self.{ds}_adapter {{\n\
         {indent}        adapter.field_or_create({col}).set(current.clone());\n\
         {indent}    }}\n\
         {indent}    self.{name}_{prop}_bound = current;\n\
         {indent}}}\n"
    )
}

#[note("targets-data-binding-egui-before-components-all")]
pub fn egui_before_components_all(controls: &[BoundControl], actions: &[DatasetAction]) -> String {
    let datasets = active_datasets(controls, actions);
    if datasets.is_empty() && controls.is_empty() && actions.is_empty() {
        return String::new();
    }
    let mut code = open_datasets_code(&datasets, "self.");
    for a in actions {
        if !a.has_on_update {
            let expr = dataset_action_enabled_expr(&a.operation, "adapter");
            code.push_str(&format!(
                "        self.{}.enabled = self.{}_adapter.as_ref().is_some_and(|adapter| {expr});\n",
                a.action_name, a.dataset_name
            ));
        }
    }
    code.push_str(&model_to_widget(controls, "self.", |_| String::new()));
    code
}

#[note("targets-data-binding-egui-before-components")]
pub fn egui_before_components(controls: &[BoundControl]) -> String {
    egui_before_components_all(controls, &[])
}

#[note("targets-data-binding-egui-after-components")]
pub fn egui_after_components(controls: &[BoundControl]) -> String {
    controls
        .iter()
        .map(|c| {
            let block = widget_to_model(c, "self.", "            ");
            if block.is_empty() {
                block
            } else {
                format!("        {{\n{block}        }}\n")
            }
        })
        .collect()
}

#[note("targets-data-binding-makepad-app-state-all")]
pub fn makepad_app_state_all(
    controls: &[BoundControl],
    actions: &[DatasetAction],
) -> Option<crate::targets::makepad::components::codegen::AppState> {
    let datasets = active_datasets(controls, actions);
    if datasets.is_empty() && controls.is_empty() {
        return None;
    }
    let mut field = String::new();

    for (dataset, _) in datasets {
        field.push_str(&format!(
            "    #[rust]\n    pub {dataset}_adapter: Option<DatasetAdapter>,\n\
             \x20   #[rust]\n    pub {dataset}_opened: bool,\n\
             \x20   #[rust]\n    pub {dataset}_error: Option<String>,\n"
        ));
    }

    for c in controls {
        let name = &c.node_name;
        let prop = &c.prop;
        field.push_str(&format!(
            "    #[rust]\n    pub {name}_{prop}_bound: String,\n"
        ));
    }

    Some(crate::targets::makepad::components::codegen::AppState {
        uses: "use crawrad_binding::DatasetAdapter;\n".to_string(),
        field,
        handle: String::new(),
    })
}

#[note("targets-data-binding-makepad-app-state")]
pub fn makepad_app_state(
    controls: &[BoundControl],
) -> Option<crate::targets::makepad::components::codegen::AppState> {
    makepad_app_state_all(controls, &[])
}

#[note("targets-data-binding-makepad-model-to-widget")]
pub fn makepad_model_to_widget(controls: &[BoundControl]) -> String {
    model_to_widget(controls, "self.form.", |c| {
        let name = &c.node_name;
        if c.class == "Edit" {
            format!("                self.ui.text_input(id!({name})).set_text(cx, &val);\n")
        } else {
            format!(
                "                if let Some(mut inner) = self.ui.check_box(id!({name}_box)).borrow_mut() {{\n\
                 \x20                   let state = if is_checked {{ id!(active.on) }} else {{ id!(active.off) }};\n\
                 \x20                   inner.animator_play(cx, state);\n\
                 \x20               }}\n"
            )
        }
    })
}

#[note("targets-data-binding-makepad-sync-all")]
pub fn makepad_sync_all(controls: &[BoundControl], actions: &[DatasetAction]) -> String {
    let datasets = active_datasets(controls, actions);
    if datasets.is_empty() && controls.is_empty() && actions.is_empty() {
        return String::new();
    }
    let mut code = open_datasets_code(&datasets, "self.form.");
    for a in actions {
        if !a.has_on_update {
            let expr = dataset_action_enabled_expr(&a.operation, "adapter");
            code.push_str(&format!(
                "        self.form.{}.enabled = self.{}_adapter.as_ref().is_some_and(|adapter| {expr});\n",
                a.action_name, a.dataset_name
            ));
        }
    }
    code.push_str(&makepad_model_to_widget(controls));
    code
}

#[note("targets-data-binding-makepad-sync")]
pub fn makepad_sync(controls: &[BoundControl]) -> String {
    makepad_sync_all(controls, &[])
}

#[note("targets-data-binding-makepad-writeback")]
pub fn makepad_writeback(doc: &Document, node: &Node) -> Result<String, Erro> {
    Ok(collect_bound_controls(doc, &[node])?
        .first()
        .map(|c| widget_to_model(c, "self.form.", "            "))
        .unwrap_or_default())
}

/// A Grid bound to a table-mode Dataset whose adapter the form holds: the grid
/// and the dataset's controls show the same record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetGrid {
    /// Identifier of the Grid in generated code.
    pub grid_name: String,
    /// Identifier of the Dataset it shows.
    pub dataset_name: String,
}

/// The emitted grids that follow (and drive) an active Dataset adapter.
///
/// # Errors
///
/// A grid name the generator cannot write.
pub fn dataset_grids(
    doc: &Document,
    emitted: &[&Node],
    datasets: &[(String, String)],
) -> Result<Vec<DatasetGrid>, Erro> {
    let mut grids = Vec::new();
    for node in emitted.iter().filter(|n| n.class == "Grid") {
        let Some(dataset) =
            crawrad_ir::design_rows::grid_binding(doc, node).and_then(|b| b.dataset)
        else {
            continue;
        };
        if !crate::targets::shared::dataset_table_mode(dataset) {
            continue;
        }
        let Ok(dataset_name) = crate::identificador(dataset.name()) else {
            continue;
        };
        if datasets.iter().any(|(d, _)| *d == dataset_name) {
            grids.push(DatasetGrid {
                grid_name: crate::identificador(node.name())?,
                dataset_name,
            });
        }
    }
    Ok(grids)
}

/// The state a dataset grid keeps: what it last saw of the dataset and its row.
pub fn dataset_grid_fields(grids: &[DatasetGrid], attr: &str) -> String {
    grids
        .iter()
        .map(|g| {
            let n = &g.grid_name;
            format!(
                "{attr}    pub {n}_ds_seen: Option<(crawrad_binding::DatasetState, usize)>,\n\
                 {attr}    pub {n}_rec_seen: Option<usize>,\n"
            )
        })
        .collect()
}

/// Each frame: a dataset back in Browse after a change (Post, Cancel, Delete)
/// reloads the grid; a row the user picks in the grid moves the dataset cursor,
/// so the controls bound to the dataset follow.
///
/// A grid that (re)opens, at start or after a reload, first stands on the
/// dataset's record (clamped to the last row after a Delete) instead of row 0;
/// that row is recorded as seen, so it never moves the cursor back.
///
/// `reload` empties the grid so it reopens; `opened` says it is open;
/// `rec_no` is the grid's current row; `set_row` makes `row` the grid's row.
fn dataset_grid_follow(
    g: &DatasetGrid,
    reload: &str,
    opened: &str,
    rec_no: &str,
    set_row: &str,
) -> String {
    let n = &g.grid_name;
    let ds = &g.dataset_name;
    format!(
        "        if let Some(adapter) = self.{ds}_adapter.as_ref() {{\n\
         \x20           let seen = (adapter.state().get(), adapter.collection().len());\n\
         \x20           let browse = seen.0 == crawrad_binding::DatasetState::Browse;\n\
         \x20           if browse && self.{n}_ds_seen.is_some_and(|s| s != seen) {{\n\
         \x20               {reload}\n\
         \x20               self.{n}_rec_seen = None;\n\
         \x20           }} else if browse && {opened} && self.{n}_rec_seen.is_none() {{\n\
         \x20               let rows = adapter.collection();\n\
         \x20               let row = rows.cursor().unwrap_or(0).min(rows.len().saturating_sub(1));\n\
         \x20               if !rows.is_empty() && rows.cursor() != Some(row) {{\n\
         \x20                   rows.set_cursor(Some(row));\n\
         \x20               }}\n\
         \x20               {set_row}\n\
         \x20               self.{n}_rec_seen = Some(row);\n\
         \x20           }} else if browse && {opened} {{\n\
         \x20               let row = {rec_no};\n\
         \x20               if self.{n}_rec_seen != Some(row) {{\n\
         \x20                   self.{n}_rec_seen = Some(row);\n\
         \x20                   adapter.collection().set_cursor(Some(row));\n\
         \x20               }}\n\
         \x20           }}\n\
         \x20           self.{n}_ds_seen = Some(seen);\n\
         \x20       }}\n"
    )
}

/// The makepad sync lines of [`dataset_grid_follow`], before the grids open.
pub fn makepad_dataset_grid_follow(grids: &[DatasetGrid]) -> String {
    grids
        .iter()
        .map(|g| {
            let n = &g.grid_name;
            dataset_grid_follow(
                g,
                &format!("self.{n}_opened = false;"),
                &format!("self.{n}_opened"),
                &format!(
                    "crawrad_runtime_makepad::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!({n})).rec_no()"
                ),
                &format!(
                    "crawrad_runtime_makepad::CrawradDataGridWidgetRefExt::crawrad_data_grid(&self.ui, id!({n})).set_rec_no(cx, row);"
                ),
            )
        })
        .collect()
}

/// The egui lines of [`dataset_grid_follow`], before the components draw.
pub fn egui_dataset_grid_follow(grids: &[DatasetGrid]) -> String {
    grids
        .iter()
        .map(|g| {
            let n = &g.grid_name;
            dataset_grid_follow(
                g,
                &format!("self.{n}_rows = None;"),
                &format!("self.{n}_rows.is_some()"),
                &format!("self.{n}_rows.as_ref().map_or(0, |rows| rows.rec_no())"),
                &format!("if let Some(grid) = self.{n}_rows.as_mut() {{ grid.set_rec_no(row); }}"),
            )
        })
        .collect()
}
