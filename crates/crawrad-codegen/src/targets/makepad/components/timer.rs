//! Makepad component implementation for `Timer`.
//!
//! `Timer` is a non-visual component and does not emit visual elements into Makepad DSL.
//!
//! It ticks: the generated `App` keeps one
//! `crawrad_runtime_makepad::timer::Ticker` per `Timer`, the sync keeps it in step
//! with `enabled` / `interval_ms`, and `handle_event` calls `OnTimer` on its tick.

use super::codegen::{AppState, MakepadComponentCodegen};
use super::util::{self, FIELD_ENABLED};
use crate::identificador;
use crate::targets::makepad::layout;
use crawrad_ir::{Document, Node, Value};

pub struct TimerCodegen;

impl MakepadComponentCodegen for TimerCodegen {
    fn class(&self) -> &'static str {
        "Timer"
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

    fn app_state(&self, node: &Node, field: &str) -> Result<Option<AppState>, crate::Erro> {
        app_state(node, field).map(Some)
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
    let desc = crawrad_registry::components::timer();
    let mut fields = vec![
        FIELD_ENABLED.to_string(),
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

pub fn emit_sync(_doc: &Document, _node: &Node, field: &str) -> String {
    format!(
        "        self.{field}_ticker.sync(cx, self.form.{field}.enabled, self.form.{field}.interval_ms);\n"
    )
}

#[crawrad_note::note("targets-makepad-components-timer-app-state")]
pub fn app_state(node: &Node, field: &str) -> Result<AppState, crate::Erro> {
    let handle = match layout::handler(node, "on_timer") {
        Some(h) => {
            let h = identificador(&h)?;
            format!(
                "        if self.{field}_ticker.fired(event) {{\n\
                 \x20           self.form.{h}();\n\
                 \x20           self.sync_ui(cx);\n\
                 \x20       }}\n"
            )
        }
        None => String::new(),
    };
    Ok(AppState {
        uses: "use crawrad_runtime_makepad::timer::Ticker as CrawradTicker;\n".to_string(),
        field: format!("    #[rust]\n    pub {field}_ticker: CrawradTicker,\n"),
        handle,
    })
}

pub fn emit_actions(
    _doc: &Document,
    _node: &Node,
    _field: &str,
    _emitted: &[&Node],
) -> Result<String, crate::Erro> {
    Ok(String::new())
}

pub fn emit_dsl(_node: &Node, _x: f32, _y: f32, _w: f32, _h: f32) -> Result<String, crate::Erro> {
    Ok(String::new())
}

pub fn emitir_dsl(_node: &Node, _x: f32, _y: f32, _w: f32, _h: f32) -> Result<String, crate::Erro> {
    Ok(String::new())
}
