//! Makepad component implementation for `ScrollBar`.

use super::codegen::MakepadComponentCodegen;
use super::util::{self, FIELD_ENABLED, FIELD_VISIBLE};
use crate::identificador;
use crate::targets::makepad::layout;
use crate::targets::makepad::shared_geometry::{
    BAR_BUTTON_SIDE, BAR_THUMB_LEN, bar_available_travel, bar_track_len,
};
use crawrad_ir::{Document, Node, Value};

pub struct ScrollBarCodegen;

impl MakepadComponentCodegen for ScrollBarCodegen {
    fn class(&self) -> &'static str {
        "ScrollBar"
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
    let desc = crawrad_registry::components::scroll_bar();
    let mut fields = vec![
        FIELD_VISIBLE.to_string(),
        FIELD_ENABLED.to_string(),
        "    pub position: f32,".to_string(),
        "    pub min: f32,".to_string(),
        "    pub max: f32,".to_string(),
        "    pub small_change: f32,".to_string(),
        "    pub large_change: f32,".to_string(),
    ];
    fields.extend(util::catalogue_fields(
        &desc,
        &[
            "kind",
            "visible",
            "enabled",
            "position",
            "min",
            "max",
            "small_change",
            "large_change",
        ],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct ScrollBar {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let vis = layout::is_visible(node);
    let en = layout::is_enabled(node);
    let pos = match node.props.get("position") {
        Some(Value::Int(i)) => *i as f32,
        Some(Value::Float(f)) => *f as f32,
        _ => 0.0,
    };
    let min = match node.props.get("min") {
        Some(Value::Int(i)) => *i as f32,
        Some(Value::Float(f)) => *f as f32,
        _ => 0.0,
    };
    let max = match node.props.get("max") {
        Some(Value::Int(i)) => *i as f32,
        Some(Value::Float(f)) => *f as f32,
        _ => 100.0,
    };
    let sc = match node.props.get("small_change") {
        Some(Value::Int(i)) => *i as f32,
        _ => 1.0,
    };
    let lc = match node.props.get("large_change") {
        Some(Value::Int(i)) => *i as f32,
        _ => 10.0,
    };
    let mut inits = vec![
        format!("visible: {vis}"),
        format!("enabled: {en}"),
        format!("position: {pos:.1}"),
        format!("min: {min:.1}"),
        format!("max: {max:.1}"),
        format!("small_change: {sc:.1}"),
        format!("large_change: {lc:.1}"),
    ];
    let desc = crawrad_registry::components::scroll_bar();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &[
            "kind",
            "visible",
            "enabled",
            "position",
            "min",
            "max",
            "small_change",
            "large_change",
        ],
    ));
    let body = inits.join(", ");
    format!("ScrollBar {{ {body} }}")
}

pub fn emit_sync(_doc: &Document, node: &Node, field: &str) -> String {
    let r = node
        .rect()
        .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 120.0, 16.0));
    let is_horizontal = r.w >= r.h;
    if is_horizontal {
        let track_w = bar_track_len(r.w);
        let available_travel = bar_available_travel(track_w);
        format!(
            "        {{\n\
             \x20           let min = self.form.{field}.min;\n\
             \x20           let max = self.form.{field}.max;\n\
             \x20           let pos = self.form.{field}.position.clamp(min, max);\n\
             \x20           let ratio = if max > min {{ (pos - min) / (max - min) }} else {{ 0.0 }};\n\
             \x20           let offset = ratio * {available_travel:.1};\n\
             \x20           self.ui.view(id!({field}.track.thumb)).apply_over(cx, live!{{\n\
             \x20               margin: {{ left: (offset) }}\n\
             \x20           }});\n\
             \x20       }}\n"
        )
    } else {
        let track_h = bar_track_len(r.h);
        let available_travel = bar_available_travel(track_h);
        format!(
            "        {{\n\
             \x20           let min = self.form.{field}.min;\n\
             \x20           let max = self.form.{field}.max;\n\
             \x20           let pos = self.form.{field}.position.clamp(min, max);\n\
             \x20           let ratio = if max > min {{ (pos - min) / (max - min) }} else {{ 0.0 }};\n\
             \x20           let offset = ratio * {available_travel:.1};\n\
             \x20           self.ui.view(id!({field}.track.thumb)).apply_over(cx, live!{{\n\
             \x20               margin: {{ top: (offset) }}\n\
             \x20           }});\n\
             \x20       }}\n"
        )
    }
}

pub fn emit_actions(
    _doc: &Document,
    node: &Node,
    field: &str,
    _emitted: &[&Node],
) -> Result<String, crate::Erro> {
    let r = node
        .rect()
        .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 120.0, 16.0));
    let is_horizontal = r.w >= r.h;
    // The `max(1.0)` lives here, not in `bar_track_len`: the division happens in
    // the EMITTED code, and what cannot happen is zero in its denominator. The
    // canvas rule remains `max(0.0)`, which is the geometric truth.
    let track_len = bar_track_len(if is_horizontal { r.w } else { r.h }).max(1.0);
    let on_change_call = match layout::handler(node, "on_change") {
        Some(h) => {
            let h_name = identificador(&h)?;
            format!("            self.form.{h_name}();\n")
        }
        None => String::new(),
    };
    let on_scroll_call = match layout::handler(node, "on_scroll") {
        Some(h) => {
            let h_name = identificador(&h)?;
            format!("            self.form.{h_name}();\n")
        }
        None => String::new(),
    };
    let mut code = String::new();
    code.push_str(&format!(
        "        if self.ui.button(id!({field}.btn_prev)).clicked(actions) {{\n\
         \x20           self.form.{field}.position = (self.form.{field}.position - self.form.{field}.small_change).clamp(self.form.{field}.min, self.form.{field}.max);\n\
         {on_change_call}\
         {on_scroll_call}\
         \x20       }}\n\
         \x20       if self.ui.button(id!({field}.btn_next)).clicked(actions) {{\n\
         \x20           self.form.{field}.position = (self.form.{field}.position + self.form.{field}.small_change).clamp(self.form.{field}.min, self.form.{field}.max);\n\
         {on_change_call}\
         {on_scroll_call}\
         \x20       }}\n\
         \x20       if let Some(fd) = self.ui.view(id!({field}.track)).finger_down(actions) {{\n\
         \x20           let rect = self.ui.view(id!({field}.track)).area().rect(cx);\n\
         \x20           let rel = if {is_horizontal} {{ fd.abs.x - rect.pos.x }} else {{ fd.abs.y - rect.pos.y }};\n\
         \x20           let ratio = (rel / {track_len:.1}).clamp(0.0, 1.0) as f32;\n\
         \x20           self.form.{field}.position = self.form.{field}.min + ratio * (self.form.{field}.max - self.form.{field}.min);\n\
         {on_change_call}\
         {on_scroll_call}\
         \x20       }}\n\
         \x20       if let Some(fm) = self.ui.view(id!({field}.track)).finger_move(actions) {{\n\
         \x20           let rect = self.ui.view(id!({field}.track)).area().rect(cx);\n\
         \x20           let rel = if {is_horizontal} {{ fm.abs.x - rect.pos.x }} else {{ fm.abs.y - rect.pos.y }};\n\
         \x20           let ratio = (rel / {track_len:.1}).clamp(0.0, 1.0) as f32;\n\
         \x20           self.form.{field}.position = self.form.{field}.min + ratio * (self.form.{field}.max - self.form.{field}.min);\n\
         {on_change_call}\
         {on_scroll_call}\
         \x20       }}\n"
    ));
    Ok(code)
}

pub fn emit_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let pos = layout::posicao(x, y);
    let name = identificador(node.name())?;
    let is_horizontal = w >= h;
    let mut dsl = String::new();
    // The button and thumb sizes come from `shared_geometry`, the same
    // file the designer canvas reads. The `{:.1}` preserves `16.0` instead
    // of `16`, because the emitted text is compared by a gate.
    let lado = format!("{BAR_BUTTON_SIDE:.1}");
    let polegar = format!("{BAR_THUMB_LEN:.1}");
    if is_horizontal {
        dsl.push_str(&format!(
            "            {name} = <View> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20               flow: Right\n\
             \x20               show_bg: true\n\
             \x20               draw_bg: {{ color: #181825 }}\n\
             \x20               btn_prev = <Button> {{\n\
             \x20                   width: {lado}, height: Fill\n\
             \x20                   margin: 0.0, padding: 0.0\n\
             \x20                   align: {{x: 0.5, y: 0.5}}\n\
             \x20                   draw_bg: {{ border_radius: 0.0, border_size: 0.0, color: #242436, color_hover: #313244 }}\n\
             \x20                   text: \"<\"\n\
             \x20               }}\n\
             \x20               track = <View> {{\n\
             \x20                   width: Fill, height: Fill\n\
             \x20                   flow: Overlay\n\
             \x20                   cursor: Hand\n\
             \x20                   show_bg: true\n\
             \x20                   draw_bg: {{ color: #1e1e2e }}\n\
             \x20                   thumb = <View> {{\n\
             \x20                       width: {polegar}, height: Fill\n\
             \x20                       show_bg: true\n\
             \x20                       draw_bg: {{ color: #45475a }}\n\
             \x20                   }}\n\
             \x20               }}\n\
             \x20               btn_next = <Button> {{\n\
             \x20                   width: {lado}, height: Fill\n\
             \x20                   margin: 0.0, padding: 0.0\n\
             \x20                   align: {{x: 0.5, y: 0.5}}\n\
             \x20                   draw_bg: {{ border_radius: 0.0, border_size: 0.0, color: #242436, color_hover: #313244 }}\n\
             \x20                   text: \">\"\n\
             \x20               }}\n\
             \x20           }}\n"
        ));
    } else {
        dsl.push_str(&format!(
            "            {name} = <View> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20               flow: Down\n\
             \x20               show_bg: true\n\
             \x20               draw_bg: {{ color: #181825 }}\n\
             \x20               btn_prev = <Button> {{\n\
             \x20                   width: Fill, height: {lado}\n\
             \x20                   margin: 0.0, padding: 0.0\n\
             \x20                   align: {{x: 0.5, y: 0.5}}\n\
             \x20                   draw_bg: {{ border_radius: 0.0, border_size: 0.0, color: #242436, color_hover: #313244 }}\n\
             \x20                   text: \"^\"\n\
             \x20               }}\n\
             \x20               track = <View> {{\n\
             \x20                   width: Fill, height: Fill\n\
             \x20                   flow: Overlay\n\
             \x20                   cursor: Hand\n\
             \x20                   show_bg: true\n\
             \x20                   draw_bg: {{ color: #1e1e2e }}\n\
             \x20                   thumb = <View> {{\n\
             \x20                       width: Fill, height: {polegar}\n\
             \x20                       show_bg: true\n\
             \x20                       draw_bg: {{ color: #45475a }}\n\
             \x20                   }}\n\
             \x20               }}\n\
             \x20               btn_next = <Button> {{\n\
             \x20                   width: Fill, height: {lado}\n\
             \x20                   margin: 0.0, padding: 0.0\n\
             \x20                   align: {{x: 0.5, y: 0.5}}\n\
             \x20                   draw_bg: {{ border_radius: 0.0, border_size: 0.0, color: #242436, color_hover: #313244 }}\n\
             \x20                   text: \"v\"\n\
             \x20               }}\n\
             \x20           }}\n"
        ));
    }
    Ok(dsl)
}

pub fn emitir_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    emit_dsl(node, x, y, w, h)
}
