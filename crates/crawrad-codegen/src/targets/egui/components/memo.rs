use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};
use crawrad_ir::Value;

pub struct MemoCodegen;

impl EguiComponentCodegen for MemoCodegen {
    fn class(&self) -> &'static str {
        "Memo"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_MEMO)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }

    #[crawrad_note::note("targets-egui-components-memo-memocodegen-rola")]
    fn rola(&self) -> bool {
        true
    }
}

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("Memo").expect("Memo descriptor");

    // read at run time, so the form's code can change it (as on Makepad)
    // Tab, Enter and the limit are read at run time too
    let leitura = format!(
        ".interactive(!self.{nome}.readonly)\n\
         \x20                                 .lock_focus(self.{nome}.want_tabs)\n\
         \x20                                 .return_key(self.{nome}.want_returns.then_some(egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::Enter)))\n\
         \x20                                 .char_limit(if self.{nome}.max_length > 0 {{ self.{nome}.max_length as usize }} else {{ usize::MAX }})"
    );
    let scroll_bars = match n.props.get("scroll_bars") {
        Some(Value::Enum(s)) => s.as_str(),
        Some(Value::Str(s)) => s.as_str(),
        _ => "ssNone",
    };
    let (h_scroll, v_scroll) = match scroll_bars {
        "ssHorizontal" => (true, false),
        "ssVertical" => (false, true),
        "ssBoth" => (true, true),
        _ => (false, true),
    };
    let visibility = if scroll_bars == "ssNone" {
        ".scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)"
    } else {
        ".scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)"
    };
    // `WordWrap` is read at run time; a horizontal bar never wraps
    let wraps = if h_scroll {
        "false".to_string()
    } else {
        format!("self.{nome}.word_wrap")
    };
    let edit_body = format!(
        "                           let wraps = {wraps};\n\
         \x20                          let mut memo_layouter = |ui: &egui::Ui, text: &dyn egui::TextBuffer, wrap_width: f32| {{\n\
         \x20                              let font_id = egui::TextStyle::Body.resolve(ui.style());\n\
         \x20                              let mut job = egui::text::LayoutJob::simple(\n\
         \x20                                  text.as_str().to_string(),\n\
         \x20                                  font_id,\n\
         \x20                                  ui.visuals().widgets.inactive.text_color(),\n\
         \x20                                  if wraps {{ wrap_width }} else {{ f32::INFINITY }},\n\
         \x20                              );\n\
         \x20                              job.keep_trailing_whitespace = true;\n\
         \x20                              ui.fonts_mut(|f| f.layout_job(job))\n\
         \x20                          }};\n\
         \x20                          if ui.add(\n\
         \x20                              egui::TextEdit::multiline(&mut self.{nome}.text)\n\
         \x20                                  .id(edit_id)\n\
         \x20                                  .frame(egui::Frame::NONE)\n\
         \x20                                  .desired_width(f32::INFINITY)\n\
         \x20                                  .layouter(&mut memo_layouter){leitura}\n\
         \x20                          ).changed() {{\n\
         \x20                              changed = true;\n\
         \x20                          }}"
    );
    let depois = match handler(n, "on_change") {
        Some(h) => format!(
            "            if changed {{\n                self.{}();\n            }}\n",
            identificador(&h)?
        ),
        None => "            let _ = changed;\n".to_string(),
    };
    let corpo = format!(
        "        if self.{nome}.visible {{\n\
         \x20           if self.{nome}.lines.text() != self.{nome}.text {{\n\
         \x20               self.{nome}.text = self.{nome}.lines.text();\n\
         \x20           }}\n\
         \x20           let is_enabled = self.{nome}.enabled;\n\
         \x20           let mut builder = egui::UiBuilder::new().max_rect(r);\n\
         \x20           if !is_enabled {{\n\
         \x20               builder = builder.disabled();\n\
         \x20           }}\n\
         \x20           let mut w = ui.new_child(builder);\n\
         \x20           let mut changed = false;\n\
         \x20           let edit_id = egui::Id::new((\"rxp-memo-txt\", \"{nome}\"));\n\
         \x20           let is_focado = w.ctx().memory(|m| m.has_focus(edit_id));\n\
         \x20           let borda = if is_focado {{\n\
         \x20               w.visuals().selection.stroke\n\
         \x20           }} else {{\n\
         \x20               w.visuals().widgets.inactive.bg_stroke\n\
         \x20           }};\n\
         \x20           // its own Color, or the theme's box colour\n\
         \x20           let c = self.{nome}.color;\n\
         \x20           let fill = if !self.{nome}.parent_color && c.a > 0 {{\n\
         \x20               egui::Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)\n\
         \x20           }} else {{\n\
         \x20               w.visuals().extreme_bg_color\n\
         \x20           }};\n\
         \x20           egui::Frame::group(w.style())\n\
         \x20               .fill(fill)\n\
         \x20               .stroke(borda)\n\
         \x20               .corner_radius(w.visuals().widgets.inactive.corner_radius)\n\
         \x20               .inner_margin(egui::Margin::ZERO)\n\
         \x20               .show(&mut w, |ui| {{\n\
         \x20                   ui.set_clip_rect(r);\n\
         \x20                   ui.set_min_size(r.size());\n\
         \x20                   ui.set_max_size(r.size());\n\
         \x20                   egui::ScrollArea::new([{h_scroll}, {v_scroll}])\n\
         \x20                       .id_salt((\"rxp-memo\", \"{nome}\"))\n\
         \x20                       .max_height(r.height().max(16.0))\n\
         \x20                       .max_width(r.width())\n\
         \x20                       .auto_shrink([false, false])\n\
         \x20                       {visibility}\n\
         \x20                       .show(ui, |ui| {{\n\
{edit_body}\n\
         \x20                       }});\n\
         \x20               }});\n\
         \x20           if changed {{\n\
         \x20               self.{nome}.lines.set_text(&self.{nome}.text);\n\
         \x20           }}\n\
{depois}\
         \x20       }}\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: Memo,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo,
        tipo_usado: "Memo",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_MEMO: StateText = StateText {
    head: r#"
/// A `Memo` on the form. `lines` holds the lines of text — classic RAD `TMemo.Lines`.
/// `text` is the raw multiline string buffer synchronized with `lines`.
/// `readonly`: the text can be read but not edited — classic RAD `ReadOnly`.
/// `word_wrap`: a long line breaks at the edge.
/// `want_returns` / `want_tabs`: Enter starts a line / Tab types a tab.
/// `max_length`: the most characters it takes, `0` for no limit.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
