//! Slint component implementation for `Timer`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crate::identificador;
use crate::targets::slint::layout;
use crawrad_ir::{Document, Node, Value};

pub struct TimerCodegen;

impl SlintComponentCodegen for TimerCodegen {
    fn class(&self) -> &'static str {
        "Timer"
    }

    fn struct_definition(&self) -> String {
        struct_definition()
    }

    fn emit_default(&self, node: &Node) -> String {
        emit_default(node)
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
        emit_actions(node, field)
    }

    fn emit_slint(
        &self,
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        _node: &Node,
        _x: f32,
        _y: f32,
        _w: f32,
        _h: f32,
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::timer();
    let mut fields = vec![
        util::FIELD_ENABLED.to_string(),
        "    pub interval_ms: u64,".to_string(),
    ];
    fields.extend(util::catalogue_fields(&desc, &["enabled", "interval"]));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct Timer {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let en = layout::is_enabled(node);
    let interval = match node.props.get("interval") {
        Some(Value::Int(i)) => (*i).max(0) as u64,
        _ => 1000,
    };
    let mut inits = vec![format!("enabled: {en}"), format!("interval_ms: {interval}")];
    let desc = crawrad_registry::components::timer();
    inits.extend(util::catalogue_inits(node, &desc, &["enabled", "interval"]));
    let body = inits.join(", ");
    format!("Timer {{ {body} }}")
}

pub fn emit_actions(node: &Node, field: &str) -> Result<String, crate::Erro> {
    let mut code = String::new();
    if let Some(h) = layout::event_handler(node, "on_timer") {
        let h_name = identificador(&h)?;
        code.push_str(&format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           let timer = slint::Timer::default();\n\
             \x20           timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(1000), move || {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             \x20                   f.form.{h_name}();\n\
             \x20               }}\n\
             \x20           }});\n\
             \x20           let _ = timer;\n\
             \x20           let _ = {field:?};\n\
             \x20       }}\n"
        ));
    }
    Ok(code)
}
