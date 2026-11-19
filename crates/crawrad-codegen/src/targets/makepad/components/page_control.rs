//! Makepad component implementation for `PageControl` and `TabSheet`.

use super::codegen::{AppState, MakepadComponentCodegen};
use super::util::{self, FIELD_ENABLED, FIELD_TEXT, FIELD_VISIBLE};
use crate::identificador;
use crate::targets::makepad::layout;
use crawrad_ir::{Document, Node, Value};

pub struct PageControlCodegen;

impl MakepadComponentCodegen for PageControlCodegen {
    fn class(&self) -> &'static str {
        "PageControl"
    }

    fn struct_definition(&self) -> String {
        page_control_struct()
    }

    fn emit_default(&self, node: &Node) -> String {
        emit_page_control_default(node)
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

    fn app_state_doc(
        &self,
        doc: &Document,
        node: &Node,
        field: &str,
    ) -> Result<Option<AppState>, crate::Erro> {
        Ok(Some(app_state(doc, node, field)?))
    }

    fn focus_path(&self, doc: &Document, node: &Node, field: &str) -> Option<String> {
        focus_path(doc, node, field)
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

pub struct TabSheetCodegen;

impl MakepadComponentCodegen for TabSheetCodegen {
    fn class(&self) -> &'static str {
        "TabSheet"
    }

    fn struct_definition(&self) -> String {
        tab_sheet_struct()
    }

    fn emit_default(&self, node: &Node) -> String {
        emit_tab_sheet_default(node)
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, _field: &str) -> String {
        String::new()
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        _node: &Node,
        _field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
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
        emit_tabsheet_dsl(node, x, y, w, h)
    }
}

pub fn page_control_struct() -> String {
    let desc_pgc = crawrad_registry::components::page_control();
    let mut fields_pgc = vec![
        FIELD_VISIBLE.to_string(),
        FIELD_ENABLED.to_string(),
        "    pub active_page: usize,".to_string(),
        "    pub active_tab: usize,".to_string(),
    ];
    fields_pgc.extend(util::catalogue_fields(
        &desc_pgc,
        &["visible", "enabled", "active_page"],
    ));
    let body_pgc = fields_pgc.join("\n");
    format!(
        "#[derive(Debug, Clone, Default)]\n\
         pub struct PageControl {{\n\
         {body_pgc}\n\
         }}\n"
    )
}

pub fn tab_sheet_struct() -> String {
    let desc_tab = crawrad_registry::components::tab_sheet();
    let mut fields_tab = vec![
        FIELD_VISIBLE.to_string(),
        FIELD_ENABLED.to_string(),
        FIELD_TEXT.to_string(),
    ];
    fields_tab.extend(util::catalogue_fields(
        &desc_tab,
        &["visible", "enabled", "text"],
    ));
    let body_tab = fields_tab.join("\n");
    format!(
        "#[derive(Debug, Clone, Default)]\n\
         pub struct TabSheet {{\n\
         {body_tab}\n\
         }}\n"
    )
}

pub fn struct_definition() -> String {
    format!("{}\n{}", page_control_struct(), tab_sheet_struct())
}

pub fn emit_tab_sheet_default(node: &Node) -> String {
    let vis = layout::is_visible(node);
    let en = layout::is_enabled(node);
    let txt = layout::text(node);
    let txt_lit = crate::literal(&txt);
    let mut inits = vec![
        format!("visible: {vis}"),
        format!("enabled: {en}"),
        format!("text: {txt_lit}.to_string()"),
    ];
    let desc = crawrad_registry::components::tab_sheet();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "text"],
    ));
    let body = inits.join(", ");
    format!("TabSheet {{ {body} }}")
}

pub fn emit_page_control_default(node: &Node) -> String {
    let vis = layout::is_visible(node);
    let en = layout::is_enabled(node);
    let page = layout::active_page(node);
    let mut inits = vec![
        format!("visible: {vis}"),
        format!("enabled: {en}"),
        format!("active_page: {page}"),
        format!("active_tab: {page}"),
    ];
    let desc = crawrad_registry::components::page_control();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "active_page"],
    ));
    let body = inits.join(", ");
    format!("PageControl {{ {body} }}")
}

pub fn emit_default(node: &Node) -> String {
    if node.class == "TabSheet" {
        emit_tab_sheet_default(node)
    } else {
        emit_page_control_default(node)
    }
}

pub fn emit_sync(doc: &Document, node: &Node, field: &str) -> String {
    if node.class == "TabSheet" {
        return String::new();
    }
    let tab_count = node
        .children
        .iter()
        .filter_map(|c_id| doc.node(c_id))
        .filter(|c| c.class == "TabSheet")
        .count();
    let tab_count = if tab_count == 0 { 1 } else { tab_count };
    let sheets: Vec<String> = node
        .children
        .iter()
        .filter_map(|c_id| doc.node(c_id))
        .filter(|c| c.class == "TabSheet")
        .filter_map(|c| identificador(c.name()).ok())
        .collect();
    let width = node.rect().map_or(320.0, |r| r.w);
    let height = node.rect().map_or(200.0, |r| r.h);
    let shown_slice = sheets
        .iter()
        .map(|s| format!("self.form.{s}.tab_visible"))
        .collect::<Vec<_>>()
        .join(", ");
    // the strip above the pages, or under them
    let mut sync = format!(
        "        crawrad_runtime_makepad::tabs::place(cx, &self.ui.widget(id!({field}.tabs_header)), &self.ui.widget(id!({field}.pages_body)), {height:.1}, self.form.{field}.tab_position == \"tpBottom\");\n\
         \x20       let window = crawrad_runtime_makepad::tabs::window({width:.1}, &[{shown_slice}], self.form.{field}.active_page);\n\
         \x20       if self.ui.widget(id!({field}.tabs_header.tab_scroll_left)).visible() != window.arrows {{\n\
         \x20           self.ui.widget(id!({field}.tabs_header.tab_scroll_left)).set_visible(cx, window.arrows);\n\
         \x20       }}\n\
         \x20       if self.ui.widget(id!({field}.tabs_header.tab_scroll_right)).visible() != window.arrows {{\n\
         \x20           self.ui.widget(id!({field}.tabs_header.tab_scroll_right)).set_visible(cx, window.arrows);\n\
         \x20       }}\n"
    );
    for i in 0..tab_count {
        sync.push_str(&format!(
            "        {{\n\
             \x20           let is_active = self.form.{field}.active_page == {i};\n\
             \x20           self.ui.view(id!({field}.pages_body.tabsheet_{i})).set_visible(cx, is_active);\n\
             \x20           let is_focused = is_active && crawrad_runtime_makepad::tabs::focused(cx, &self.ui.widget(id!({field}.tabs_header.tab_{i})));\n\
             \x20           crawrad_runtime_makepad::tabs::look(cx, &self.ui.widget(id!({field}.tabs_header.tab_{i}_mark)), &self.ui.widget(id!({field}.tabs_header.tab_{i}_ring)), is_active, is_focused);\n\
             \x20       }}\n"
        ));
        // the page's caption and whether its tab shows, read at run time
        if let Some(sheet) = sheets.get(i) {
            sync.push_str(&format!(
                "        {{\n\
                 \x20           let tab = self.ui.button(id!({field}.tabs_header.tab_{i}));\n\
                 \x20           if tab.text() != self.form.{sheet}.text {{\n\
                 \x20               tab.set_text(cx, &self.form.{sheet}.text);\n\
                 \x20           }}\n\
                 \x20           let shown = self.form.{sheet}.tab_visible && window.contains({i});\n\
                 \x20           if self.ui.widget(id!({field}.tabs_header.tab_{i}_cell)).visible() != shown {{\n\
                 \x20               self.ui.widget(id!({field}.tabs_header.tab_{i}_cell)).set_visible(cx, shown);\n\
                 \x20           }}\n\
                 \x20       }}\n"
            ));
        }
    }
    sync
}

pub fn emit_actions(
    doc: &Document,
    node: &Node,
    field: &str,
    _emitted: &[&Node],
) -> Result<String, crate::Erro> {
    if node.class == "TabSheet" {
        return Ok(String::new());
    }
    let tab_count = node
        .children
        .iter()
        .filter_map(|c_id| doc.node(c_id))
        .filter(|c| c.class == "TabSheet")
        .count();
    let tab_count = if tab_count == 0 { 1 } else { tab_count };
    let on_change_call = match layout::handler(node, "on_change") {
        Some(h) => {
            let h_name = identificador(&h)?;
            format!("            self.form.{h_name}();\n")
        }
        None => String::new(),
    };
    let veto = match layout::handler(node, "on_changing") {
        Some(h) => format!(
            "            let mut allow_change = true;\n\
             \x20           self.form.{}(&mut allow_change);\n",
            identificador(&h)?
        ),
        None => "            let allow_change = true;\n".to_string(),
    };
    let mut code = String::new();
    let sheets: Vec<String> = node
        .children
        .iter()
        .filter_map(|c_id| doc.node(c_id))
        .filter(|c| c.class == "TabSheet")
        .filter_map(|c| identificador(c.name()).ok())
        .collect();
    let shown_slice = sheets
        .iter()
        .map(|s| format!("self.form.{s}.tab_visible"))
        .collect::<Vec<_>>()
        .join(", ");
    for i in 0..tab_count {
        // the tab already open changes nothing, and OnChanging can
        // say no
        code.push_str(&format!(
            "        if self.ui.button(id!({field}.tabs_header.tab_{i})).clicked(actions) && self.form.{field}.active_page != {i} {{\n\
             {veto}\
             \x20           if allow_change {{\n\
             \x20               self.form.{field}.active_page = {i};\n\
             \x20               self.form.{field}.active_tab = {i};\n\
             {on_change_call}\
             \x20           }}\n\
             \x20       }}\n",
            on_change_call = on_change_call.replace("            ", "                ")
        ));
    }
    let veto_scroll = veto.replace("            ", "                ");
    let on_change_scroll = on_change_call.replace("            ", "                    ");
    code.push_str(&format!(
        "        if self.ui.button(id!({field}.tabs_header.tab_scroll_left)).clicked(actions) {{\n\
         \x20           let vis = &[{shown_slice}];\n\
         \x20           if let Some(target) = crawrad_runtime_makepad::tabs::prev_page(self.form.{field}.active_page, vis) {{\n\
         {veto_scroll}\
         \x20               if allow_change {{\n\
         \x20                   self.form.{field}.active_page = target;\n\
         \x20                   self.form.{field}.active_tab = target;\n\
         {on_change_scroll}\
         \x20               }}\n\
         \x20           }}\n\
         \x20       }}\n\
         \x20       if self.ui.button(id!({field}.tabs_header.tab_scroll_right)).clicked(actions) {{\n\
         \x20           let vis = &[{shown_slice}];\n\
         \x20           if let Some(target) = crawrad_runtime_makepad::tabs::next_page(self.form.{field}.active_page, vis) {{\n\
         {veto_scroll}\
         \x20               if allow_change {{\n\
         \x20                   self.form.{field}.active_page = target;\n\
         \x20                   self.form.{field}.active_tab = target;\n\
         {on_change_scroll}\
         \x20               }}\n\
         \x20           }}\n\
         \x20       }}\n"
    ));
    Ok(code)
}

#[crawrad_note::note("targets-makepad-components-page-control-emit-dsl")]
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
    let mut dsl = String::new();
    // where the strip and the pages start, as designed
    let bottom = crawrad_registry::tabs_at_bottom(node);
    let strip_top = crawrad_registry::tab_strip_top(h, bottom);
    let pages_top = crawrad_registry::pages_top(bottom);

    let tabs: Vec<(String, String)> = node
        .children
        .iter()
        .filter_map(|c_id| doc.node(c_id))
        .filter(|c| c.class == "TabSheet")
        .map(|c| {
            let title = c.text().unwrap_or(c.name()).to_string();
            let tab_name = identificador(c.name()).unwrap_or_else(|_| c.name().to_string());
            (tab_name, title)
        })
        .collect();

    let tabs_list = if tabs.is_empty() {
        vec![("tab_0".to_string(), "TabSheet1".to_string())]
    } else {
        tabs
    };

    dsl.push_str(&format!(
        "            {name} = <View> {{\n\
         \x20               {pos}\n\
         \x20               width: {w}, height: {h}\n\
         \x20               flow: Overlay\n\
         \x20               show_bg: true\n\
         \x20               draw_bg: {{ color: #242424 }}\n\
         \x20               tabs_header = <View> {{\n\
         \x20                   margin: {{ top: {strip_top:.1} }}\n\
         \x20                   width: Fill, height: 24.0\n\
         \x20                   flow: Right\n\
         \x20                   show_bg: true\n\
         \x20                   draw_bg: {{ color: #1e1e2e }}\n"
    ));

    let active = match node.props.get("active_page") {
        Some(Value::Int(i)) if *i >= 0 => *i as usize,
        _ => 0,
    };

    let tab_w = crawrad_registry::TAB_WIDTH;
    let btn_w = crawrad_registry::TAB_SCROLL_BUTTON_WIDTH;

    for (i, (_tab_name, title)) in tabs_list.iter().enumerate() {
        let is_active = i == active;
        let bg = if is_active { "#313244" } else { "#181825" };
        let fg = if is_active { "#89b4fa" } else { "#7f849c" };
        let title_esc = title.replace('\\', "\\\\").replace('"', "\\\"");
        dsl.push_str(&format!(
            "                    tab_{i}_cell = <View> {{\n\
             \x20                       width: {tab_w:.1}, height: 24.0\n\
             \x20                       flow: Overlay\n\
             \x20                       tab_{i} = <Button> {{\n\
             \x20                           width: {tab_w:.1}, height: 24.0\n\
             \x20                           padding: {{left: 12.0, top: 4.0, right: 12.0, bottom: 4.0}}\n\
             \x20                           draw_bg: {{\n\
             \x20                               border_radius: 0.0\n\
             \x20                               color: {bg}\n\
             \x20                           }}\n\
             \x20                           draw_text: {{ color: {fg} }}\n\
             \x20                           text: \"{title_esc}\"\n\
             \x20                       }}\n\
             \x20                       tab_{i}_mark = <View> {{\n\
             \x20                           visible: {is_active}\n\
             \x20                           width: Fill, height: 2.0\n\
             \x20                           margin: {{ top: 22.0 }}\n\
             \x20                           show_bg: true\n\
             \x20                           draw_bg: {{ color: #89b4fa }}\n\
             \x20                       }}\n\
             \x20                       tab_{i}_ring = <RoundedView> {{\n\
             \x20                           visible: false\n\
             \x20                           width: Fill, height: Fill\n\
             \x20                           show_bg: true\n\
             \x20                           draw_bg: {{ color: #0000, border_radius: 0.0, border_size: 1.0, border_color: #89b4fa }}\n\
             \x20                       }}\n\
             \x20                   }}\n"
        ));
    }
    dsl.push_str(&format!(
        "                    tabs_fill = <View> {{ width: Fill, height: 24.0 }}\n\
         \x20                   tab_scroll_left = <Button> {{ visible: false, width: {btn_w:.1}, height: 24.0, padding: {{left: 0.0, top: 4.0, right: 0.0, bottom: 4.0}}, draw_bg: {{ border_radius: 0.0, color: #181825 }}, draw_text: {{ color: #7f849c }}, text: \"<\" }}\n\
         \x20                   tab_scroll_right = <Button> {{ visible: false, width: {btn_w:.1}, height: 24.0, padding: {{left: 0.0, top: 4.0, right: 0.0, bottom: 4.0}}, draw_bg: {{ border_radius: 0.0, color: #181825 }}, draw_text: {{ color: #7f849c }}, text: \">\" }}\n"
    ));
    dsl.push_str("                }\n");

    let client_h = (h - 24.0).max(0.0);
    dsl.push_str(&format!(
        "                pages_body = <View> {{\n\
         \x20                   margin: {{ top: {pages_top:.1} }}\n\
         \x20                   width: Fill, height: {client_h}\n\
         \x20                   flow: Overlay\n\
         \x20                   show_bg: true\n\
         \x20                   draw_bg: {{ color: #242424 }}\n"
    ));

    let tab_nodes: Vec<&Node> = node
        .children
        .iter()
        .filter_map(|c_id| doc.node(c_id))
        .filter(|c| c.class == "TabSheet")
        .collect();

    for (i, tab_node) in tab_nodes.iter().enumerate() {
        let is_visible = i == active;
        dsl.push_str(&format!(
            "                    tabsheet_{i} = <View> {{\n\
             \x20                       visible: {is_visible}\n\
             \x20                       width: Fill, height: Fill\n\
             \x20                       flow: Overlay\n"
        ));
        dsl.push_str(&super::emit_children(doc, reg, tab_node, "        ")?);
        dsl.push_str("                    }\n");
    }

    dsl.push_str("                }\n");
    dsl.push_str("            }\n");

    Ok(dsl)
}

pub fn emit_tabsheet_dsl(
    node: &Node,
    _x: f32,
    _y: f32,
    _w: f32,
    _h: f32,
) -> Result<String, crate::Erro> {
    let _name = identificador(node.name())?;
    Ok(String::new())
}

pub fn emitir_dsl(
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

pub fn emitir_tabsheet_dsl(
    node: &Node,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Result<String, crate::Erro> {
    emit_tabsheet_dsl(node, x, y, w, h)
}

#[crawrad_note::note("targets-makepad-components-page-control-sheets")]
fn sheets<'a>(doc: &'a Document, node: &Node) -> Vec<&'a Node> {
    node.children
        .iter()
        .filter_map(|c_id| doc.node(c_id))
        .filter(|c| c.class == "TabSheet")
        .collect()
}

#[crawrad_note::note("targets-makepad-components-page-control-tab-count")]
fn tab_count(doc: &Document, node: &Node) -> usize {
    sheets(doc, node).len().max(1)
}

#[crawrad_note::note("targets-makepad-components-page-control-focus-path")]
pub fn focus_path(doc: &Document, node: &Node, field: &str) -> Option<String> {
    let tab_count = tab_count(doc, node);
    let mut arms = Vec::new();
    for i in 0..tab_count {
        arms.push(format!(
            "{i} => self.ui.widget(id!({field}.tabs_header.tab_{i}))"
        ));
    }
    arms.push(format!(
        "_ => self.ui.widget(id!({field}.tabs_header.tab_0))"
    ));
    Some(format!(
        "match self.form.{field}.active_page {{\n                {},\n            }}",
        arms.join(",\n                ")
    ))
}

#[crawrad_note::note("targets-makepad-components-page-control-app-state")]
pub fn app_state(doc: &Document, node: &Node, field: &str) -> Result<AppState, crate::Erro> {
    let tab_count = tab_count(doc, node);
    let sheets: Vec<String> = sheets(doc, node)
        .into_iter()
        .filter_map(|c| identificador(c.name()).ok())
        .collect();
    let eligible_slice = sheets
        .iter()
        .map(|s| format!("self.form.{s}.tab_visible"))
        .collect::<Vec<_>>()
        .join(", ");
    let on_change_call = match layout::handler(node, "on_change") {
        Some(h) => {
            let h_name = identificador(&h)?;
            format!("                        self.form.{h_name}();\n")
        }
        None => String::new(),
    };
    let veto = match layout::handler(node, "on_changing") {
        Some(h) => format!(
            "                    let mut allow_change = true;\n\
             \x20                   self.form.{}(&mut allow_change);\n",
            identificador(&h)?
        ),
        None => "                    let allow_change = true;\n".to_string(),
    };

    let mut tab_focus_arms = Vec::new();
    let mut focus_target_arms = Vec::new();
    for i in 0..tab_count {
        tab_focus_arms.push(format!(
            "{i} => crawrad_runtime_makepad::tabs::focused(cx, &self.ui.widget(id!({field}.tabs_header.tab_{i})))"
        ));
        focus_target_arms.push(format!(
            "{i} => self.ui.widget(id!({field}.tabs_header.tab_{i})).set_key_focus(cx)"
        ));
    }
    tab_focus_arms.push(format!(
        "_ => crawrad_runtime_makepad::tabs::focused(cx, &self.ui.widget(id!({field}.tabs_header.tab_0)))"
    ));
    focus_target_arms.push(format!(
        "_ => self.ui.widget(id!({field}.tabs_header.tab_0)).set_key_focus(cx)"
    ));

    let tab_focus_match = format!(
        "match self.form.{field}.active_page {{\n\
         \x20                   {},\n\
         \x20               }}",
        tab_focus_arms.join(",\n                    ")
    );
    let focus_target_match = format!(
        "match target {{\n\
         \x20                           {},\n\
         \x20                       }};",
        focus_target_arms.join(",\n                            ")
    );

    let handle = format!(
        "        // the focus moves a pass after it is asked for: the ring\n\
         \x20       // round the open tab follows it then\n\
         \x20       if let Event::KeyFocus(_) | Event::KeyFocusLost(_) = event {{\n\
         \x20           self.sync_ui(cx);\n\
         \x20       }}\n\
         \x20       // Left/Right arrow keys navigate tabs when PageControl is focused\n\
         \x20       // (one let chain: nested `if`s are `clippy::collapsible_if`)\n\
         \x20       if let Event::KeyDown(k) = event\n\
         \x20           && !k.modifiers.control && !k.modifiers.alt && !k.modifiers.logo\n\
         \x20           && {tab_focus_match}\n\
         \x20           && let Some(target) = match k.key_code {{\n\
         \x20               KeyCode::ArrowLeft => crawrad_runtime_makepad::tabs::prev_page(self.form.{field}.active_page, &[{eligible_slice}]),\n\
         \x20               KeyCode::ArrowRight => crawrad_runtime_makepad::tabs::next_page(self.form.{field}.active_page, &[{eligible_slice}]),\n\
         \x20               _ => None,\n\
         \x20           }}\n\
         \x20       {{\n\
         {veto}\
         \x20           if allow_change {{\n\
         \x20               self.form.{field}.active_page = target;\n\
         \x20               self.form.{field}.active_tab = target;\n\
         {on_change_call}\
         \x20               {focus_target_match}\n\
         \x20               self.sync_ui(cx);\n\
         \x20           }}\n\
         \x20       }}\n",
        veto = veto.replace("                    ", "            "),
        on_change_call = on_change_call.replace("                        ", "                "),
    );

    Ok(AppState {
        uses: String::new(),
        field: String::new(),
        handle,
    })
}
