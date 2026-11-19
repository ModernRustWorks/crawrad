use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::Erro;

pub struct ScrollBoxCodegen;

impl EguiComponentCodegen for ScrollBoxCodegen {
    fn class(&self) -> &'static str {
        "ScrollBox"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_SCROLLBOX)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        emitir(ctx)
    }

    #[crawrad_note::note("targets-egui-components-scroll-box-scrollboxcodegen-rola")]
    fn rola(&self) -> bool {
        true
    }

    #[crawrad_note::note("targets-egui-components-scroll-box-scrollboxcodegen-envolve-filhos")]
    fn envolve_filhos(&self) -> bool {
        true
    }

    fn corpo_fecha(&self, _ctx: &ContextoEmissao) -> Result<String, Erro> {
        Ok(FECHA.to_string())
    }
}

// **NEITHER CONTENT RESERVATION NOR `max_height` — both were tried and both
// were removed.**
//
// After nesting the children the bar still did not appear, and I tried two
// things: reserving the content extent with `allocate_rect`, and limiting the
// `ScrollArea` with `max_height`/`max_width`. Neither changed anything, and the
// second also misaligned a neighbor.
//
// The measurement that settled it was a standalone egui, with four variants side by
// side displaying `min_rect` and `inner_rect` on screen: the content ALREADY measured
// `104x232` in a `194x144` viewport — `ui.put` counts, and always did. What
// was missing was **one frame**: egui's `ScrollArea` decides the scrollbar from
// the PREVIOUS frame's `state.show_scroll` (`scroll_area.rs:747`), and writes the
// current frame's at the end (`:1552`). Without repainting, the first frame measures and the
// second draws.
//
// This is documented because it is counterintuitive and because the two lines I would
// have left here would have seemed necessary forever.

#[crawrad_note::note("targets-egui-components-scroll-box-fecha")]
const FECHA: &str = "\
\x20                           });\n\
\x20                   });\n\
\x20               });\n\
\x20           });\n\
\x20       }\n";

pub fn emitir(ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
    let nome = ctx.nome;
    let n = ctx.node;
    let desc = ctx.reg.get("ScrollBox").expect("ScrollBox descriptor");

    let corpo = format!(
        "        if self.{nome}.visible {{\n\
         \x20           let is_enabled = self.{nome}.enabled;\n\
         \x20           // `auto_scroll` is the classic `AutoScroll`: the bar\n\
         \x20           // shows up when the content runs past the box and goes away\n\
         \x20           // when it does not. Turned off, it never shows up.\n\
         \x20           let bars = if self.{nome}.auto_scroll {{\n\
         \x20               egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded\n\
         \x20           }} else {{\n\
         \x20               egui::scroll_area::ScrollBarVisibility::AlwaysHidden\n\
         \x20           }};\n\
         \x20           ui.add_enabled_ui(is_enabled, |ui| {{\n\
         \x20               // `scope_builder` with a `max_rect` IS what the removed\n\
         \x20               // `allocate_ui_at_rect` did — same body, same allocation\n\
         \x20               // of the space actually used. `new_child` would draw in\n\
         \x20               // the rectangle without allocating it.\n\
         \x20               ui.scope_builder(egui::UiBuilder::new().max_rect(r), |ui| {{\n\
         \x20                   // the border is `border_width` points (0: none)\n\
         \x20                   // and the background its own Color, or the theme's\n\
         \x20                   let mut frame = egui::Frame::canvas(ui.style());\n\
         \x20                   let c = self.{nome}.color;\n\
         \x20                   if !self.{nome}.parent_color && c.a > 0 {{\n\
         \x20                       frame = frame.fill(egui::Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a));\n\
         \x20                   }}\n\
         \x20                   frame = frame.stroke(egui::Stroke::new(self.{nome}.border_width.max(0) as f32, frame.stroke.color));\n\
         \x20                   frame.show(ui, |ui| {{\n\
         \x20                       // `HorzScrollBar`/`VertScrollBar` off: that axis neither shows a bar nor scrolls, as in the classic.\n\
         \x20                       egui::ScrollArea::new([self.{nome}.horz_scroll_bar, self.{nome}.vert_scroll_bar])\n\
         \x20                           .auto_shrink([false, false])\n\
         \x20                           .scroll_bar_visibility(bars)\n\
         \x20                           .show(ui, |ui| {{\n"
    );

    Ok(ResultadoComponente {
        campo: format!("    pub {nome}: ScrollBox,\n"),
        padrao: format!(
            "            {nome}: {},\n",
            crate::targets::egui::state::init_of(n, desc, ctx.doc)
        ),
        constante: String::new(),
        corpo,
        tipo_usado: "ScrollBox",
        precisa_timer: false,
        precisa_item: false,
        compartilhado: Compartilhado::default(),
    })
}

const ESTADO_SCROLLBOX: StateText = StateText {
    head: r#"
/// A `ScrollBox` container on the form.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
