//! Makepad component implementation for `Memo`.

use super::codegen::MakepadComponentCodegen;
use super::util::{self, FIELD_ENABLED, FIELD_LINES, FIELD_TEXT, FIELD_VISIBLE};
use crate::identificador;
use crate::targets::makepad::layout;
use crawrad_ir::{Document, Node, Value};

pub struct MemoCodegen;

impl MakepadComponentCodegen for MemoCodegen {
    fn class(&self) -> &'static str {
        "Memo"
    }

    fn default_cursor(&self) -> &'static str {
        "text"
    }

    fn enter_key(&self) -> super::EnterKey {
        super::EnterKey::BreaksLine
    }

    fn struct_definition(&self) -> String {
        struct_definition()
    }

    fn emit_default(&self, node: &Node) -> String {
        emit_default(node)
    }

    fn text_path(&self, node: &Node, _field: &str) -> String {
        text_input_path(node)
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
    let desc = crawrad_registry::components::memo();
    let mut fields = vec![
        FIELD_VISIBLE.to_string(),
        FIELD_ENABLED.to_string(),
        FIELD_LINES.to_string(),
        FIELD_TEXT.to_string(),
    ];
    fields.extend(util::catalogue_fields(
        &desc,
        &["visible", "enabled", "text", "lines", "scroll_bars"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct Memo {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let vis = layout::is_visible(node);
    let en = layout::is_enabled(node);
    let txt = layout::text(node);
    let lines_str = match node.props.get("lines") {
        Some(Value::Str(s)) => s.as_str(),
        _ => &txt,
    };
    let lines_lit = crate::literal(lines_str);
    let mut inits = vec![
        format!("visible: {vis}"),
        format!("enabled: {en}"),
        format!("lines: Strings::from_text({lines_lit})"),
        format!("text: {lines_lit}.to_string()"),
    ];
    let desc = crawrad_registry::components::memo();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "text", "lines", "scroll_bars"],
    ));
    let body = inits.join(", ");
    format!("Memo {{ {body} }}")
}

pub fn emit_sync(_doc: &Document, node: &Node, field: &str) -> String {
    let ti_path = text_input_path(node);
    format!(
        "        let ti = self.ui.text_input(id!({ti_path}));\n\
         \x20       let memo_text = if !self.form.{field}.text.is_empty() {{\n\
         \x20           self.form.{field}.text.clone()\n\
         \x20       }} else {{\n\
         \x20           self.form.{field}.lines.text()\n\
         \x20       }};\n\
         \x20       if ti.text() != memo_text {{\n\
         \x20           ti.set_text(cx, &memo_text);\n\
         \x20       }}\n{}",
        util::text_input_flags_sync(field, false)
    ) + &format!(
        "        // `WordWrap`, read at run time like the other flags\n\
         \x20       crawrad_runtime_makepad::memo::set_word_wrap(cx, &ti, self.form.{field}.word_wrap);\n"
    )
}

pub fn emit_actions(
    _doc: &Document,
    node: &Node,
    field: &str,
    _emitted: &[&Node],
) -> Result<String, crate::Erro> {
    let ti_path = text_input_path(node);
    let on_change_call = match layout::handler(node, "on_change") {
        Some(h) => {
            let h_name = identificador(&h)?;
            format!("            self.form.{h_name}();\n")
        }
        None => String::new(),
    };
    // the text went in: the form's copy follows, and `OnChange` fires
    let took = |indent: &str| {
        format!(
            "{indent}self.form.{field}.text = _val.clone();\n\
             {indent}self.form.{field}.lines = Strings::from_text(&_val);\n\
             {}",
            on_change_call.replace("            ", indent)
        )
    };
    let mut code = format!(
        "        let ti = self.ui.text_input(id!({ti_path}));\n\
         \x20       let max_length = self.form.{field}.max_length;\n\
         \x20       if let Some(_val) = self.ui.text_input(id!({ti_path})).changed(actions) {{\n\
         \x20           // `MaxLength` — the widget has no limit of its own\n\
         \x20           let _val = crawrad_runtime_makepad::memo::hold_to(cx, &ti, &_val, max_length).unwrap_or(_val);\n\
         {}\
         \x20       }}\n",
        took("            ")
    );
    // a plain Enter only reports `Returned` in makepad 1.0.0; with
    // `WantReturns` it starts a new line, as in a classic RAD `Memo`
    code.push_str(&format!(
        "        if self.form.{field}.want_returns\n\
         \x20           && !self.form.{field}.readonly\n\
         \x20           && ti.returned(actions).is_some()\n\
         \x20           && let Some(_val) = crawrad_runtime_makepad::memo::type_text(cx, &ti, \"\\n\", max_length)\n\
         \x20       {{\n\
         {}\
         \x20       }}\n",
        took("            ")
    ));
    // the widget ignores Tab; with `WantTabs` it types one (one let
    // chain: nested `if`s are `clippy::collapsible_if`)
    code.push_str(&format!(
        "        if self.form.{field}.want_tabs\n\
         \x20           && !self.form.{field}.readonly\n\
         \x20           && let Some(k) = ti.key_down_unhandled(actions)\n\
         \x20           && k.key_code == KeyCode::Tab\n\
         \x20           && !k.modifiers.shift\n\
         \x20           && let Some(_val) = crawrad_runtime_makepad::memo::type_text(cx, &ti, \"\\t\", max_length)\n\
         \x20       {{\n\
         {}\
         \x20       }}\n",
        took("            ")
    ));
    Ok(code)
}

#[crawrad_note::note("targets-makepad-components-memo-has-scroll-bars")]
pub fn has_scroll_bars(node: &Node) -> bool {
    match node.props.get("scroll_bars") {
        Some(Value::Enum(s)) | Some(Value::Str(s)) => s != "ssNone",
        _ => false,
    }
}

#[crawrad_note::note("targets-makepad-components-memo-scroll-bars-axes")]
pub fn scroll_bars_axes(node: &Node) -> (bool, bool) {
    match node.props.get("scroll_bars") {
        Some(Value::Enum(s)) | Some(Value::Str(s)) => match s.as_str() {
            "ssHorizontal" => (true, false),
            "ssVertical" => (false, true),
            "ssBoth" => (true, true),
            _ => (false, false),
        },
        _ => (false, false),
    }
}

#[crawrad_note::note("targets-makepad-components-memo-text-input-path")]
pub fn text_input_path(node: &Node) -> String {
    let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
    if has_scroll_bars(node) {
        format!("{name}.{name}_text")
    } else {
        name
    }
}

pub fn emit_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let pos = layout::posicao(x, y);
    let name = identificador(node.name())?;
    let lines = match node.props.get("lines") {
        Some(Value::Str(s)) => s.clone(),
        _ => match node.props.get("text") {
            Some(Value::Str(s)) => s.clone(),
            _ => node.text().unwrap_or("").to_string(),
        },
    };
    let lines_esc = lines
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\r', "")
        .replace('\n', "\\n");
    let flags = util::text_input_flags(node);
    let word_wrap = !matches!(node.props.get("word_wrap"), Some(Value::Bool(false)));

    if !has_scroll_bars(node) {
        // `WordWrap` off — the line runs on instead of breaking at the edge
        let wrap = if !word_wrap {
            "                flow: Right\n"
        } else {
            ""
        };
        let mut dsl = String::new();
        dsl.push_str(&format!(
            "            {name} = <TextInput> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20               empty_text: \"\"\n\
             \x20               text: \"{lines_esc}\"\n\
             {flags}\
             {wrap}\
             \x20           }}\n"
        ));
        return Ok(dsl);
    }

    let (show_x, show_y) = scroll_bars_axes(node);
    let inner_w = if !word_wrap && show_x { "Fit" } else { "Fill" };
    let wrap_inner = if !word_wrap {
        "                    flow: Right\n"
    } else {
        ""
    };
    let flags_inner = flags.replace("                ", "                    ");

    // `TextInput` does not scroll, so the box is a scrolling view that
    // wears the text box's own inset, radius and bevel, and the text input
    // inside is see-through: the two memos look alike, with and without bars
    let mut dsl = String::new();
    dsl.push_str(&format!(
        "            {name} = <RoundedView> {{\n\
         \x20               {pos}\n\
         \x20               width: {w}, height: {h}\n\
         \x20               show_bg: true\n\
         \x20               draw_bg: {{ color: (THEME_COLOR_INSET), border_radius: (THEME_CORNER_RADIUS), border_size: (THEME_BEVELING), border_color: (THEME_COLOR_BEVEL_INSET_2) }}\n\
         \x20               scroll_bars: <ScrollBars> {{\n\
         \x20                   show_scroll_x: {show_x}\n\
         \x20                   show_scroll_y: {show_y}\n\
         \x20               }}\n\
         \x20               {name}_text = <TextInput> {{\n\
         \x20                   width: {inner_w}, height: Fit\n\
         \x20                   draw_bg: {{ color: #0000, color_hover: #0000, color_focus: #0000, color_down: #0000, color_empty: #0000, color_disabled: #0000, border_size: 0.0 }}\n\
         \x20                   empty_text: \"\"\n\
         \x20                   text: \"{lines_esc}\"\n\
         {flags_inner}\
         {wrap_inner}\
         \x20               }}\n\
         \x20           }}\n"
    ));
    Ok(dsl)
}

pub fn emitir_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    emit_dsl(node, x, y, w, h)
}
