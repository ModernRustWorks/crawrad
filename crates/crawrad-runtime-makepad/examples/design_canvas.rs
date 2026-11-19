//! The designer canvas, end-to-end, in a real Makepad window.
//!
//! Opens a real `.crawform` and lets you **select by clicking, move by dragging, and
//! resize by pulling a handle** — with `crawrad-designer` plugged in, which is what
//! decides all of this. This file plays the role of the shell: it is the only place in the
//! crate that talks to the designer (ADR 0027).
//!
//! ```text
//! cargo run -p crawrad-runtime-makepad --example design_canvas
//! cargo run -p crawrad-runtime-makepad --example design_canvas -- caminho/do/form.crawform
//! CRAWRAD_FORM=caminho/do/form.crawform cargo run -p crawrad-runtime-makepad --example design_canvas
//! ```
//!
//! The loop is this, and it is the same as the egui IDE (`crawrad-ide/src/main.rs:5067`):
//!
//! ```text
//! Hit  ->  DesignCanvas  ->  RuntimeEvent  ->  Designer  ->  Transaction
//!                                                              |
//!            redesenha  <-  Runtime::update/set_*  <-  Doc::apply
//! ```

use makepad_widgets::*;

use crawrad_designer::{Designer, TabAction};
use crawrad_doc::Doc;
use crawrad_ir::{Document, NodeId, Value};
use crawrad_registry::Registry;
use crawrad_runtime::{Runtime, RuntimeEvent, Scale, ViewportPx};
use crawrad_runtime_makepad::{DesignCanvas, MakepadRuntime};

live_design! {
    use link::widgets::*;
    use link::theme::*;
    // The canvas comes from the crate, via the `crawrad` link declared in `src/canvas.rs`.
    use link::crawrad::*;

    App = {{App}} {
        ui: <Root> {
            janela = <Window> {
                window: { title: "CrawRAD — Makepad design canvas" },
                body = <View> {
                    flow: Down,
                    barra = <View> {
                        height: 26, flow: Right, align: { y: 0.5 },
                        padding: { left: 8, right: 8 },
                        show_bg: true,
                        draw_bg: { color: #181825 }
                        estado = <Label> {
                            draw_text: { color: #a6adc8 }
                            text: "loading..."
                        }
                    }
                    canvas = <DesignCanvas> {}
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    /// The document with history, selection, and auditing — the same type the IDE
    /// uses.
    #[rust(Doc::new(Document::new("form", "Form")))]
    doc: Doc,
    /// The gesture engine. 155 headless tests, and not a single line lives in the
    /// adapter.
    #[rust(designer_com_catalogo())]
    designer: Designer,
    #[rust]
    carregado: bool,
    #[rust]
    estado: String,
    /// What the bar already displays. Without this, rewriting the label on every frame
    /// would request a redraw on every frame (`label.rs:247-249`) and the window
    /// would never sleep.
    #[rust]
    estado_na_tela: String,
}

fn designer_com_catalogo() -> Designer {
    Designer::with_registry(std::rc::Rc::new(Registry::builtin()))
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crawrad_runtime_makepad::live_design(cx);
    }
}

/// The environment variable pointing to a form OUTSIDE the repository.
///
/// Exists because the author's local path is not a repository
/// path — and the example must open on anyone's machine.
const VAR_FORM: &str = "CRAWRAD_FORM";

/// The initial zoom, to verify the ruler and font size at two scales.
const VAR_ZOOM: &str = "CRAWRAD_ZOOM";

/// The node selected as soon as the form opens, by id.
///
/// It exists for **proof**: the clipping law says the handles are
/// cut together with the component, and a handle only exists on a selected
/// component. Reaching that state by hand means clicking the right pixel before
/// every print, and a print that depends on aim is a print nobody can repeat.
const VAR_SELECT: &str = "CRAWRAD_SELECT";

/// The REPOSITORY forms, in order: the default when no path is provided.
///
/// No substitute is silently invented: the status bar reports which one
/// loaded, and if none loads it explains why.
const CANDIDATOS: &[&str] = &[
    "examples/calculator/calculator/src/form_main.crawform",
    "examples/contacts/contacts/src/form_main.crawform",
    "examples/contacts-edit/contacts_edit/src/form_main.crawform",
];

/// Search order: the command-line argument, then [`VAR_FORM`],
/// then the repository [`CANDIDATOS`].
fn carregar_documento(arg: Option<String>) -> (Document, String) {
    let mut tentados = Vec::new();
    let lista: Vec<String> = match arg.or_else(|| std::env::var(VAR_FORM).ok()) {
        Some(a) => vec![a],
        None => CANDIDATOS.iter().map(|s| s.to_string()).collect(),
    };
    for caminho in lista {
        match std::fs::read_to_string(&caminho) {
            Ok(texto) => match Document::from_json(&texto) {
                Ok(doc) => {
                    let n = doc.in_paint_order().len();
                    return (doc, format!("{caminho} — {n} components"));
                }
                Err(e) => tentados.push(format!("{caminho}: {e}")),
            },
            Err(e) => tentados.push(format!("{caminho}: {e}")),
        }
    }
    // Nothing loaded. The minimal form is used, but ANNOUNCED — a silent
    // fallback would invalidate the test.
    (
        formulario_de_emergencia(),
        format!("FALLBACK (nothing loaded): {}", tentados.join(" | ")),
    )
}

/// A hand-crafted form, so the example still opens when no
/// `.crawform` exists on disk.
fn formulario_de_emergencia() -> Document {
    let mut doc = Document::new("form", "Form");
    if let Some(raiz) = doc.node_mut("form") {
        raiz.props.insert(
            "rect".into(),
            Value::Rect(crawrad_ir::Rect::new(0.0, 0.0, 480.0, 320.0)),
        );
        raiz.props.insert("text".into(), Value::Str("Form1".into()));
    }
    let pos = |i: f32| crawrad_ir::Rect::new(24.0, 24.0 + i * 40.0, 120.0, 24.0);
    for (i, (id, class, texto)) in [
        ("btn1", "Button", "Button1"),
        ("edt1", "Edit", "Edit1"),
        ("chk1", "CheckBox", "CheckBox1"),
        ("lbl1", "Label", "Label1"),
        ("pnl1", "Panel", "Panel1"),
    ]
    .into_iter()
    .enumerate()
    {
        let no = crawrad_ir::Node::new(id, class)
            .with_prop("rect", Value::Rect(pos(i as f32)))
            .with_prop("name", Value::Str(id.into()))
            .with_prop("text", Value::Str(texto.into()));
        let _ = doc.add_child("form", no);
    }
    doc
}

impl App {
    fn canvas(&self) -> WidgetRef {
        self.ui.widget(id!(canvas))
    }

    fn carregar(&mut self, cx: &mut Cx) {
        let arg = std::env::args().nth(1).filter(|a| !a.starts_with("--"));
        let (documento, de_onde) = carregar_documento(arg);
        self.doc = Doc::new(documento);
        self.estado = de_onde;

        let canvas = self.canvas();
        if let Some(mut c) = canvas.borrow_mut::<DesignCanvas>() {
            let rt = c.runtime_mut();
            if let Err(e) = rt.open(self.doc.ir()) {
                self.estado = format!("runtime: {e}");
            }
            rt.set_viewport(
                ViewportPx {
                    x: 0,
                    y: 0,
                    w: 1200,
                    h: 800,
                },
                Scale(1.0),
            );
            rt.set_grid(self.designer.grid);
            // Zoom is set via environment variable, purely for VERIFICATION: the
            // example has no keyboard shortcuts, and the ruler and font size
            // can only be checked by viewing the same form at two scales.
            if let Some(z) = std::env::var(VAR_ZOOM)
                .ok()
                .and_then(|v| v.parse::<f32>().ok())
            {
                rt.set_zoom(z);
                self.estado = format!("{} — zoom {z}", self.estado);
            }
        }
        if let Ok(id) = std::env::var(VAR_SELECT)
            && self.doc.ir().node(&id).is_some()
        {
            self.doc.select(vec![id.clone()]);
            if let Some(mut c) = canvas.borrow_mut::<DesignCanvas>() {
                c.runtime_mut().set_selection(&[id]);
            }
        }
        self.carregado = true;
        self.mostrar_estado(cx);
        canvas.redraw(cx);
    }

    fn mostrar_estado(&mut self, cx: &mut Cx) {
        let sel = self.doc.selection().join(", ");
        let texto = if sel.is_empty() {
            format!(
                "{}  |  click to select, drag to move, pull a handle to resize",
                self.estado
            )
        } else {
            format!("{}  |  selected: {sel}", self.estado)
        };
        if texto == self.estado_na_tela {
            return;
        }
        // During `Event::Startup` the `<Root>` tree does not exist yet:
        // `id!(estado)` returns an empty reference and the text would be lost.
        // Therefore the label is reapplied until it takes effect.
        if self.ui.widget(id!(estado)).is_empty() {
            return;
        }
        self.ui.label(id!(estado)).set_text(cx, &texto);
        self.estado_na_tela = texto;
    }

    /// The gesture loop: collects from the canvas, delivers to the designer, applies to the
    /// document, and returns the result to the canvas.
    ///
    /// This is EXACTLY the shell's role, and the reason it lives in an example rather than
    /// the crate: the adapter does not depend on `crawrad-designer`.
    fn bombear(&mut self, cx: &mut Cx) {
        let canvas = self.canvas();
        let eventos = {
            let Some(mut c) = canvas.borrow_mut::<DesignCanvas>() else {
                return;
            };
            c.runtime_mut().drain_events()
        };
        if eventos.is_empty() {
            return;
        }

        let mut marquee = None;
        let mut guias = Vec::new();
        let mut status = None;

        for ev in eventos {
            let sel: Vec<NodeId> = self.doc.selection().to_vec();
            let out = match ev {
                RuntimeEvent::PointerDown { x, y, ctrl } => {
                    // Tab strip comes BEFORE selection: clicking a tab
                    // changes the page, it does not select the `PageControl`.
                    if let Some(acao) = self.designer.hit_tab(self.doc.ir(), x, y) {
                        self.acao_de_aba(acao);
                        continue;
                    }
                    self.designer.pointer_down(self.doc.ir(), &sel, x, y, ctrl)
                }
                RuntimeEvent::PointerMove { x, y, .. } => {
                    self.designer.pointer_move(self.doc.ir(), x, y)
                }
                RuntimeEvent::PointerUp { .. } => self.designer.pointer_up(self.doc.ir()),
                RuntimeEvent::PointerDblClick { .. } => continue,
                RuntimeEvent::Cancel => self.designer.cancel(),
            };

            if let Some(nova) = out.select {
                self.doc.select(nova);
            }
            if let Some(tx) = out.tx
                && let Err(e) = self.doc.apply(tx)
            {
                status = Some(format!("doc: {e}"));
            }
            marquee = out.marquee;
            guias = out.guides;
            if out.status.is_some() {
                status = out.status;
            }
        }

        if let Some(s) = status {
            self.estado = s;
        }

        let sel: Vec<NodeId> = self.doc.selection().to_vec();
        if let Some(mut c) = canvas.borrow_mut::<DesignCanvas>() {
            let rt: &mut MakepadRuntime = c.runtime_mut();
            // Reopening is the honest approach here: `Applied`'s `touched` is
            // lost in the loop above, and a form has dozens of nodes.
            let _ = rt.open(self.doc.ir());
            rt.set_selection(&sel);
            rt.set_marquee(marquee);
            rt.set_guides(&guias);
        }
        self.mostrar_estado(cx);
        canvas.redraw(cx);
    }

    fn acao_de_aba(&mut self, acao: TabAction) {
        if let TabAction::Open { control, page } = acao {
            let tx =
                crawrad_doc::Transaction::user("Change page").with(crawrad_doc::Command::SetProp {
                    node: control,
                    key: "active_page".into(),
                    value: Some(Value::Int(page as i64)),
                });
            let _ = self.doc.apply(tx);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Startup = event {
            self.carregar(cx);
        }
        self.ui.handle_event(cx, event, &mut Scope::empty());
        if self.carregado {
            self.bombear(cx);
            self.mostrar_estado(cx);
        }
    }
}

app_main!(App);

/// `app_main!` generates `pub fn app_main()`; a Cargo example needs
/// `main`.
fn main() {
    app_main()
}
