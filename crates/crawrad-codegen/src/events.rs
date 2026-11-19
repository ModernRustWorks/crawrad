//! The event side of code generation: **what a handler looks like in the
//! generated project**, the payload types its signature is allowed to name,
//! and which handlers the user's file ends up carrying.
//!
//! # Why this is not inside a target
//!
//! The handler signature is not a drawing decision — it is the **contract the
//! user writes against**, and the catalogue is the one that declares it
//! (`crawrad_registry::EventDesc::signature`). Both generators ask the same
//! question, so they ask it in the same place: a second copy of
//! [`method_params`] would be a second point of maintenance, and two copies
//! diverge — the very disease the house already catalogued.
//!
//! The same goes for [`wired_handlers`]: the two targets used to carry a
//! byte-for-byte copy of the loop that decides **which** handlers the user's
//! file gets. That set is now also what decides which payload types the
//! designer emits, so a third copy of it would have been a way for the
//! designer and the user's file to disagree about a type — and that does not
//! fail a unit test, it fails the user's `cargo build`.
//!
//! # Why the payload types carry no framework type
//!
//! [`PAYLOAD_TABLE`] used to live in the egui target, and `KeyEvent` had a
//! `key: egui::Key` in it. That was measured before it was moved, and the
//! measurement is worth writing down:
//!
//! - **every other type the user's file touches is ours** — `Button`, `Edit`,
//!   `Strings`, all plain `String`/`bool`/`i64` inside. `egui::Key` was the
//!   **single** place where a framework type crossed into the user's
//!   contract;
//! - the two targets emit a type named `KeyEvent` into a file that also does
//!   `use makepad_widgets::*;`, where `KeyEvent`, `MouseButton` and `KeyCode`
//!   **already exist**. Ours shadows theirs (a local item beats a glob
//!   import). A `KeyEvent` of ours holding a `makepad_widgets::KeyCode`
//!   inside, next to the `makepad_widgets::KeyEvent` it shadows, would be
//!   confusing twice over;
//! - nothing in either generated project **constructs** one of these values
//!   today: no generated code calls `on_key_down`. They are pure contract, so
//!   picking a neutral vocabulary costs nothing now and keeps the user's
//!   handler bodies framework-free when the wiring lands.
//!
//! The [`Key`] vocabulary is not invented here: it is egui's logical-key list,
//! adopted whole, so the egui side maps by identity when the wiring comes and
//! the makepad side maps its `KeyCode` onto a list it did not have to argue
//! about. `Unknown` is the one addition — a mapping without a fallback is a
//! `panic!` waiting for a keyboard we did not think of.
//!
//! [`Key`]: PAYLOAD_TABLE
//!
//! # Why only the types this form names are emitted
//!
//! Here it used to say that the whole block went out **always**, wired or
//! not, so that wiring an event tomorrow would not have to rewrite the user's
//! file. That was paid for with a `#![allow(dead_code)]` at the top of every
//! generated designer, and the price is not payable: the designer is rewritten
//! whole on the next generation, so an attribute in it silences a file the
//! user cannot fix — and the house has no `allow` anywhere.
//!
//! What was measured, and is what makes the narrow rule work:
//!
//! - a payload type that **no** handler names is `dead_code` in the generated
//!   binary. `pub` does not save it — a `bin` crate has no outside for a `pub`
//!   item to be reachable from;
//! - a payload type **named in the signature** of a handler is live, even with
//!   an empty body and even though nothing calls the handler, and so are the
//!   types inside it (`KeyEvent` alive keeps `Key` and `ShiftState` alive).
//!   That is why [`needs`](PayloadType::needs) exists and why the closure is
//!   taken.
//!
//! So the rule is: **emit exactly what this form's handlers name, and its
//! transitive parts.** No form gets a type it does not use, and no form that
//! uses one is missing it.
//!
//! **What this costs, said plainly.** Wire `OnKeyDown`, write a body, then
//! unwire it: the handler stays (it is the user's the moment the body is not
//! empty) and its `KeyEvent` parameter with it, while the designer stops
//! emitting `KeyEvent` — and the user's file stops compiling until the
//! signature is fixed. That is the same breakage the user already gets by
//! deleting a component a handler body touches, and it is a trade made with
//! eyes open: a broken build that names the line beats a lint silenced for a
//! whole file.

use crate::{Erro, identificador};
use crawrad_ir::{Document, Node};
use crawrad_note::note;
use std::collections::BTreeSet;

#[note("events-method-params")]
pub fn method_params(reg: &crawrad_registry::Registry, class: &str, event: &str) -> String {
    if let Some(desc) = reg.get(class)
        && let Some(ev) = desc.events.iter().find(|e| e.name == event)
    {
        return ev.method_params();
    }
    for desc in reg.all() {
        if let Some(ev) = desc.events.iter().find(|e| e.name == event) {
            return ev.method_params();
        }
    }
    "&mut self".to_string()
}

#[note("events-handler")]
pub struct Handler<'a> {
    /// The component the event was wired on — the root node for a form event.
    pub node: &'a Node,
    /// The event in IR spelling (`on_click`).
    pub event: &'a str,
    /// The name of the `fn` in the user's file.
    pub name: String,
}

#[note("events-wired-handlers")]
pub fn wired_handlers<'a>(root: &'a Node, emitted: &[&'a Node]) -> Result<Vec<Handler<'a>>, Erro> {
    let mut pointing: Vec<(&Node, &str, String)> = Vec::new();
    for n in std::iter::once(root).chain(emitted.iter().copied()) {
        for (event, target) in &n.events {
            if target.trim().is_empty() {
                continue;
            }
            pointing.push((n, event.as_str(), identificador(target)?));
        }
    }

    let mut written = BTreeSet::new();
    let mut out = Vec::new();
    for (seen, event_seen, f) in &pointing {
        if !written.insert(f.clone()) {
            continue;
        }
        let (node, event) = pointing
            .iter()
            .find(|(m, ev, g)| g == f && crate::e_nome_derivado(m.name(), ev, f))
            .map_or((*seen, *event_seen), |(m, ev, _)| (*m, *ev));
        out.push(Handler {
            node,
            event,
            name: f.clone(),
        });
    }
    Ok(out)
}

/// The nodes whose wired handlers the user's file must carry: every node the
/// target emits, plus every node it does **not** emit that still has a wired
/// event — a `GridColumn` is drawn by its grid, but its `OnGetText` handler is
/// called from the designer all the same, so a missing stub is a `cargo build`
/// error in the user's project. Inherited nodes stay out: the base writes theirs.
pub(crate) fn handler_nodes<'a>(
    doc: &'a Document,
    emitted: &[&'a Node],
    base: Option<&Document>,
) -> Vec<&'a Node> {
    let mut out: Vec<&Node> = emitted.to_vec();
    for id in doc.in_paint_order() {
        let Some(n) = doc.node(&id) else { continue };
        if out.iter().any(|e| e.id == n.id) {
            continue;
        }
        if !n.events.iter().any(|(_, target)| !target.trim().is_empty()) {
            continue;
        }
        if base.is_some_and(|b| doc.is_inherited(&n.id, b)) {
            continue;
        }
        out.push(n);
    }
    out
}

#[note("events-payloadtype")]
pub struct PayloadType {
    /// The name the handler signature uses, with no path.
    pub name: &'static str,
    /// The payload types named by this one's fields. Emitting this type
    /// without them would not compile.
    pub needs: &'static [&'static str],
    /// The definition, exactly as it goes into the designer of both targets.
    pub text: &'static str,
}

#[note("events-payload-table")]
pub const PAYLOAD_TABLE: &[PayloadType] = &[
    PayloadType {
        name: "ShiftState",
        needs: &[],
        text: SHIFT_STATE,
    },
    PayloadType {
        name: "MouseButton",
        needs: &[],
        text: MOUSE_BUTTON,
    },
    PayloadType {
        name: "Key",
        needs: &[],
        text: KEY,
    },
    PayloadType {
        name: "MouseEvent",
        needs: &["MouseButton", "ShiftState"],
        text: MOUSE_EVENT,
    },
    PayloadType {
        name: "MouseMoveEvent",
        needs: &["ShiftState"],
        text: MOUSE_MOVE_EVENT,
    },
    PayloadType {
        name: "KeyEvent",
        needs: &["Key", "ShiftState"],
        text: KEY_EVENT,
    },
    PayloadType {
        name: "MouseActivateEvent",
        needs: &["MouseButton", "ShiftState"],
        text: MOUSE_ACTIVATE_EVENT,
    },
    // What a PaintBox's OnPaint draws on
    PayloadType {
        name: "Canvas",
        needs: &[],
        text: CANVAS,
    },
];

#[note("events-canvas")]
const CANVAS: &str = r#"
/// One thing a PaintBox's `OnPaint` drew, in the order it drew it.
#[derive(Clone, Debug, PartialEq)]
pub enum PaintCommand {
    /// A rectangle (`radius` 0) or a rounded one; each part optional.
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        fill: Option<[u8; 4]>,
        stroke: Option<([u8; 4], f32)>,
    },
    /// A circle around `(x, y)`.
    Circle {
        x: f32,
        y: f32,
        r: f32,
        fill: Option<[u8; 4]>,
        stroke: Option<([u8; 4], f32)>,
    },
    /// A straight line.
    Line {
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        color: [u8; 4],
        width: f32,
    },
    /// Text with its top-left corner at `(x, y)`.
    Text {
        text: String,
        x: f32,
        y: f32,
        color: [u8; 4],
        size: f32,
    },
}

/// **What a PaintBox's `OnPaint` draws on**: the box's own coordinates,
/// `(0, 0)` at its top-left corner. The names are the Canvas 2D ones —
/// `fill_rect`, `stroke_rect`, `move_to`, `line_to`, `fill_text` — with
/// colours as red, green, blue and alpha bytes (a `Color` converts).
#[derive(Clone, Debug, PartialEq)]
pub struct Canvas {
    width: f32,
    height: f32,
    fill: [u8; 4],
    stroke: [u8; 4],
    line_width: f32,
    font_size: f32,
    pen: (f32, f32),
    commands: Vec<PaintCommand>,
}

impl Canvas {
    /// A blank canvas of the box's size: a white brush, a black pen one
    /// point wide, text at 12 points.
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            fill: [255, 255, 255, 255],
            stroke: [0, 0, 0, 255],
            line_width: 1.0,
            font_size: 12.0,
            pen: (0.0, 0.0),
            commands: Vec::new(),
        }
    }

    /// The box's width.
    pub fn width(&self) -> f32 {
        self.width
    }

    /// The box's height.
    pub fn height(&self) -> f32 {
        self.height
    }

    /// The colour fills and text use.
    pub fn set_fill_style(&mut self, color: impl Into<[u8; 4]>) {
        self.fill = color.into();
    }

    /// The colour outlines and lines use.
    pub fn set_stroke_style(&mut self, color: impl Into<[u8; 4]>) {
        self.stroke = color.into();
    }

    /// How wide outlines and lines are, in points.
    pub fn set_line_width(&mut self, width: f32) {
        self.line_width = width.max(0.0);
    }

    /// The size of the text `fill_text` writes, in points.
    pub fn set_font_size(&mut self, size: f32) {
        self.font_size = size.max(1.0);
    }

    /// A filled rectangle.
    pub fn fill_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.fill_round_rect(x, y, w, h, 0.0);
    }

    /// A rectangle's outline.
    pub fn stroke_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.stroke_round_rect(x, y, w, h, 0.0);
    }

    /// A filled rectangle with rounded corners.
    pub fn fill_round_rect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32) {
        let fill = Some(self.fill);
        self.commands.push(PaintCommand::Rect { x, y, w, h, radius, fill, stroke: None });
    }

    /// A rounded rectangle's outline.
    pub fn stroke_round_rect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32) {
        let stroke = Some((self.stroke, self.line_width));
        self.commands.push(PaintCommand::Rect { x, y, w, h, radius, fill: None, stroke });
    }

    /// A filled circle.
    pub fn fill_circle(&mut self, x: f32, y: f32, r: f32) {
        let fill = Some(self.fill);
        self.commands.push(PaintCommand::Circle { x, y, r, fill, stroke: None });
    }

    /// A circle's outline.
    pub fn stroke_circle(&mut self, x: f32, y: f32, r: f32) {
        let stroke = Some((self.stroke, self.line_width));
        self.commands.push(PaintCommand::Circle { x, y, r, fill: None, stroke });
    }

    /// Moves the pen without drawing.
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.pen = (x, y);
    }

    /// A line from the pen to `(x, y)`, where the pen then stays.
    pub fn line_to(&mut self, x: f32, y: f32) {
        let (x0, y0) = self.pen;
        let (color, width) = (self.stroke, self.line_width);
        self.commands.push(PaintCommand::Line { x0, y0, x1: x, y1: y, color, width });
        self.pen = (x, y);
    }

    /// Text in the fill colour, its top-left corner at `(x, y)`.
    pub fn fill_text(&mut self, text: &str, x: f32, y: f32) {
        let (color, size) = (self.fill, self.font_size);
        self.commands.push(PaintCommand::Text { text: text.to_string(), x, y, color, size });
    }

    /// What was drawn, in order.
    pub fn commands(&self) -> &[PaintCommand] {
        &self.commands
    }
}
"#;

#[note("events-payload-types")]
pub fn payload_types(reg: &crawrad_registry::Registry, handlers: &[Handler]) -> String {
    payload_types_excluding(reg, handlers, &[])
}

#[note("events-payload-types-excluding")]
pub fn payload_types_excluding(
    reg: &crawrad_registry::Registry,
    handlers: &[Handler],
    excluding: &[&'static str],
) -> String {
    let mut needed: BTreeSet<&'static str> = BTreeSet::new();
    for h in handlers {
        let params = method_params(reg, &h.node.class, h.event);
        for t in PAYLOAD_TABLE {
            if names_type(&params, t.name) {
                close_over(t.name, &mut needed);
            }
        }
    }
    for excl in excluding {
        needed.remove(excl);
    }
    PAYLOAD_TABLE
        .iter()
        .filter(|t| needed.contains(t.name))
        .map(|t| t.text)
        .collect()
}

#[note("events-key-variants")]
pub fn key_variants() -> Vec<&'static str> {
    let corpo = KEY
        .split_once("pub enum Key {")
        .map_or("", |(_, t)| t.split_once('}').map_or(t, |(a, _)| a));
    corpo
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.starts_with("///") && !l.is_empty())
        .flat_map(|l| l.split(','))
        .map(str::trim)
        .filter(|v| {
            !v.is_empty()
                && v.starts_with(|c: char| c.is_ascii_uppercase())
                && v.chars().all(|c| c.is_ascii_alphanumeric())
        })
        .collect()
}

#[note("events-close-over")]
fn close_over(name: &'static str, needed: &mut BTreeSet<&'static str>) {
    if !needed.insert(name) {
        return;
    }
    let Some(t) = PAYLOAD_TABLE.iter().find(|t| t.name == name) else {
        return;
    };
    for d in t.needs {
        close_over(d, needed);
    }
}

#[note("events-names-type")]
fn names_type(params: &str, tipo: &str) -> bool {
    let bytes = params.as_bytes();
    let mut from = 0;
    while let Some(i) = params[from..].find(tipo) {
        let start = from + i;
        let end = start + tipo.len();
        let before_ok = start == 0 || !e_parte_de_nome(bytes[start - 1]);
        let after_ok = end == bytes.len() || !e_parte_de_nome(bytes[end]);
        if before_ok && after_ok {
            return true;
        }
        from = start + 1;
    }
    false
}

fn e_parte_de_nome(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

const SHIFT_STATE: &str = r#"
/// Modifiers for keyboard and mouse (Shift, Ctrl, Alt).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ShiftState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}
"#;

const MOUSE_BUTTON: &str = r#"
/// Mouse buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}
"#;

const KEY: &str = r#"
/// A key on the keyboard, as `OnKeyDown` and `OnKeyUp` report it.
///
/// These are LOGICAL keys — what the keymap in force produces, not the
/// position on the plastic. `OnKeyPress` is the other half: it hands you the
/// `char` that was typed, accents and all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Key {
    ArrowDown, ArrowLeft, ArrowRight, ArrowUp,
    Escape, Tab, Backspace, Enter, Space,
    Insert, Delete, Home, End, PageUp, PageDown,
    Copy, Cut, Paste,
    Colon, Comma, Backslash, Slash, Pipe, Questionmark, Exclamationmark,
    OpenBracket, CloseBracket, OpenCurlyBracket, CloseCurlyBracket,
    Backtick, Minus, Period, Plus, Equals, Semicolon, Quote,
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24,
    F25, F26, F27, F28, F29, F30, F31, F32, F33, F34, F35,
    BrowserBack, IntlBackslash,
    ShiftLeft, ShiftRight, ControlLeft, ControlRight,
    AltLeft, AltRight, SuperLeft, SuperRight,
    /// A key this vocabulary has no name for. It exists so that reading a
    /// keyboard is never a `panic!`, and it is the `Default` for the same
    /// reason: a `KeyEvent` that no one filled in reports "I do not know"
    /// rather than an `A` nobody pressed.
    #[default]
    Unknown,
}
"#;

const MOUSE_EVENT: &str = r#"
/// Payload for mouse button events (`OnMouseDown`, `OnMouseUp`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseEvent {
    pub button: MouseButton,
    pub x: f32,
    pub y: f32,
    pub shift: ShiftState,
}
"#;

const MOUSE_MOVE_EVENT: &str = r#"
/// Payload for mouse movement (`OnMouseMove`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseMoveEvent {
    pub x: f32,
    pub y: f32,
    pub shift: ShiftState,
}
"#;

const KEY_EVENT: &str = r#"
/// Payload for physical key events (`OnKeyDown`, `OnKeyUp`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: Key,
    pub shift: ShiftState,
}
"#;

const MOUSE_ACTIVATE_EVENT: &str = r#"
/// Payload for mouse activation events (`OnMouseActivate`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseActivateEvent {
    pub button: MouseButton,
    pub shift: ShiftState,
    pub x: f32,
    pub y: f32,
    pub hit_test: i32,
}
"#;

#[note("events-fired-variant-name")]
pub(crate) fn fired_variant_name(component: &str, event: &str) -> String {
    let comp_part = crate::tipo_do_form(component);
    let ev = event
        .strip_prefix("on_")
        .or_else(|| event.strip_prefix("On"))
        .or_else(|| event.strip_prefix("on"))
        .unwrap_or(event);
    let ev_part = crate::tipo_do_form(ev);
    format!("{comp_part}{ev_part}")
}

#[note("events-event-payload-type")]
pub(crate) fn event_payload_type(
    reg: &crawrad_registry::Registry,
    class: &str,
    event: &str,
) -> Option<String> {
    let params = method_params(reg, class, event);
    let args = handler_args(&params);
    let [only] = args.as_slice() else {
        return None;
    };
    let typ = only.split_once(':')?.1.trim();
    (!typ.starts_with('&') && !typ.is_empty()).then(|| typ.to_string())
}

#[note("events-handler-args")]
pub(crate) fn handler_args(params: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let (mut depth, mut start, mut prev) = (0i32, 0, ' ');
    for (i, c) in params.char_indices() {
        match c {
            '(' | '<' | '[' => depth += 1,
            '>' if prev == '-' => {}
            ')' | '>' | ']' => depth -= 1,
            ',' if depth == 0 => {
                out.push(params[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
        prev = c;
    }
    out.push(params[start..].trim());
    out.retain(|p| !p.is_empty() && *p != "&mut self" && *p != "&self");
    out
}

#[note("events-is-reportable")]
pub(crate) fn is_reportable(reg: &crawrad_registry::Registry, class: &str, event: &str) -> bool {
    let params = method_params(reg, class, event);
    match handler_args(&params).as_slice() {
        [] => true,
        [_] => event_payload_type(reg, class, event).is_some(),
        _ => false,
    }
}

#[note("events-has-balanced-parens")]
pub(crate) fn has_balanced_parens(s: &str) -> bool {
    let mut depth: i32 = 0;
    for c in s.chars() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    depth == 0
}

#[note("events-has-mut-arg")]
pub(crate) fn has_mut_arg(reg: &crawrad_registry::Registry, class: &str, event: &str) -> bool {
    let params = method_params(reg, class, event);
    for part in params.split(',').skip(1) {
        if part.trim().contains("&mut ") {
            return true;
        }
    }
    false
}

#[note("events-wiredevent")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WiredEvent {
    /// Identifier of the component owning the event.
    pub component: String,
    /// Component class name in the registry.
    pub class: String,
    /// Name of the event as declared in the form.
    pub event: String,
    /// Identifier of the handler function.
    pub handler: String,
    /// Variant name in the generated Fired enum.
    pub variant: String,
    /// Payload type name if the event takes parameters.
    pub payload: Option<String>,
    /// The base forms an inherited event is wrapped through, outermost first
    /// (`[FormA]` for an event FormB reports from its base FormA): its
    /// pattern is `FormBFired::Base(FormAFired::Variant)`. Empty for the
    /// form's own events.
    pub through: Vec<String>,
}

#[note("events-collect-wired-events")]
pub(crate) fn collect_wired_events(
    reg: &crawrad_registry::Registry,
    root: &Node,
    emitted: &[&Node],
) -> Result<Vec<WiredEvent>, Erro> {
    let mut out = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for n in std::iter::once(root).chain(emitted.iter().copied()) {
        let comp_name = n.name();
        for (event, target) in &n.events {
            if target.trim().is_empty() {
                continue;
            }
            if (event == "on_create" || event == "OnCreate") && n.id == root.id {
                continue;
            }
            let handler = identificador(target)?;
            if !is_reportable(reg, &n.class, event) {
                continue;
            }
            let variant = fired_variant_name(comp_name, event);
            if !seen.insert((comp_name.to_string(), event.clone())) {
                continue;
            }
            let payload = event_payload_type(reg, &n.class, event);
            out.push(WiredEvent {
                component: comp_name.to_string(),
                class: n.class.clone(),
                event: event.clone(),
                handler,
                variant,
                payload,
                through: Vec::new(),
            });
        }
    }
    Ok(out)
}

#[note("events-emit-fired-enum")]
pub(crate) fn emit_fired_enum(
    form_type: &str,
    events: &[WiredEvent],
    base: Option<&str>,
) -> String {
    if events.is_empty() && base.is_none() {
        return String::new();
    }
    let mut variants = String::new();
    let mut seen = std::collections::BTreeSet::new();
    for ev in events {
        if !seen.insert(ev.variant.clone()) {
            continue;
        }
        match &ev.payload {
            Some(p) => variants.push_str(&format!("    {}({p}),\n", ev.variant)),
            None => variants.push_str(&format!("    {},\n", ev.variant)),
        }
    }
    if let Some(base) = base {
        variants.push_str(&format!(
            "    /// An event of the base form, reported up.\n    Base({base}Fired),\n"
        ));
    }
    format!(
        "\n/// Events fired by the form and its components.\n\
         #[derive(Clone, Debug, PartialEq)]\n\
         pub enum {form_type}Fired {{\n\
         {variants}\
         }}\n"
    )
}

#[note("events-emit-dispatch")]
pub(crate) fn emit_dispatch(form_type: &str, events: &[WiredEvent], base_arm: &str) -> String {
    if events.is_empty() && base_arm.is_empty() {
        return String::new();
    }
    let mut arms = String::new();
    let mut seen = std::collections::BTreeSet::new();
    for ev in events {
        if !seen.insert(ev.variant.clone()) {
            continue;
        }
        let h = &ev.handler;
        match &ev.payload {
            Some(_) => arms.push_str(&format!(
                "            {form_type}Fired::{}(payload) => self.{h}(payload),\n",
                ev.variant
            )),
            None => arms.push_str(&format!(
                "            {form_type}Fired::{} => self.{h}(),\n",
                ev.variant
            )),
        }
    }
    format!(
        "\n    /// Dispatches a fired event to its corresponding handler.\n\
         \x20   pub fn dispatch(&mut self, fired: {form_type}Fired) {{\n\
         \x20       match fired {{\n\
         {arms}\
         {base_arm}\
         \x20       }}\n\
         \x20   }}\n"
    )
}

#[note("events-call-args-from-params")]
pub(crate) fn call_args_from_params(params: &str) -> String {
    let mut args = Vec::new();
    for part in params.split(',') {
        let part = part.trim();
        if part == "&mut self" || part == "&self" || part.is_empty() {
            continue;
        }
        if let Some((name, _)) = part.split_once(':') {
            args.push(name.trim());
        }
    }
    args.join(", ")
}

#[note("events-rewrite-statement-calls")]
pub(crate) fn rewrite_statement_calls(
    code: &str,
    tipo: &str,
    wired: &[WiredEvent],
    prefix: &str,
    reg: &crawrad_registry::Registry,
    used_indices: &mut std::collections::BTreeSet<usize>,
    rewritten: &mut Vec<WiredEvent>,
) -> String {
    let mut out_lines = Vec::new();

    for line in code.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(prefix)
            && rest.ends_with(");")
            && let Some(open_idx) = rest.find('(')
        {
            let h_name = rest[..open_idx].trim();
            let args = &rest[open_idx + 1..rest.len() - 2];
            if !h_name.is_empty()
                && has_balanced_parens(args)
                && let Some((idx, ev)) = wired.iter().enumerate().find(|(i, e)| {
                    !used_indices.contains(i)
                        && e.handler == h_name
                        && !has_mut_arg(reg, &e.class, &e.event)
                })
            {
                used_indices.insert(idx);
                rewritten.push(ev.clone());
                let indent = &line[..line.len() - line.trim_start().len()];
                let v = &ev.variant;
                let trimmed_args = args.trim();
                let call = if trimmed_args.is_empty() {
                    format!("{indent}crawrad_fired.push({tipo}Fired::{v});")
                } else {
                    format!("{indent}crawrad_fired.push({tipo}Fired::{v}({trimmed_args}));")
                };
                out_lines.push(call);
                continue;
            }
        }
        out_lines.push(line.to_string());
    }

    let mut result = out_lines.join("\n");
    if code.ends_with('\n') {
        result.push('\n');
    }
    result
}

/// The derived form's handler for an inherited event, when it overrides it.
fn derived_handler<'a>(doc: &'a Document, ev: &WiredEvent) -> Option<&'a str> {
    let node = doc.nodes.get(&ev.component)?;
    let (_, handler) = node
        .events
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(&ev.event))?;
    Some(handler.as_str()).filter(|h| !h.trim().is_empty())
}

/// The `match` pattern of a base's event, as the derived form sees it:
/// `FormBFired::Variant`, or `FormBFired::Base(FormAFired::Variant(payload))`
/// for one FormB reports from its own base.
fn fired_pattern(base_type: &str, ev: &WiredEvent) -> String {
    let leaf = match ev.payload {
        Some(_) => format!("{}(payload)", ev.variant),
        None => ev.variant.clone(),
    };
    let types: Vec<&str> = std::iter::once(base_type)
        .chain(ev.through.iter().map(String::as_str))
        .collect();
    let (inner, outer) = types.split_last().expect("one type at least");
    let mut pattern = format!("{inner}Fired::{leaf}");
    for t in outer.iter().rev() {
        pattern = format!("{t}Fired::Base({pattern})");
    }
    pattern
}

/// The match arms of the base's events this form overrides, and how many.
fn override_arms(
    doc: &Document,
    base_type: &str,
    base_events: &[WiredEvent],
    receiver: &str,
) -> Result<(String, usize), Erro> {
    let mut arms = String::new();
    let mut overridden = 0;
    for ev in base_events {
        let Some(dh) = derived_handler(doc, ev) else {
            continue;
        };
        overridden += 1;
        let dh_ident = identificador(dh)?;
        let args = if ev.payload.is_some() { "payload" } else { "" };
        arms.push_str(&format!(
            "                {} => {{\n\
             \x20                   {receiver}.{dh_ident}({args});\n\
             \x20               }}\n",
            fired_pattern(base_type, ev)
        ));
    }
    Ok((arms, overridden))
}

#[note("events-nest-base-events")]
pub(crate) fn nest_base_events(
    doc: &Document,
    base_type: &str,
    base_events: &[WiredEvent],
) -> Vec<WiredEvent> {
    base_events
        .iter()
        .map(|ev| {
            let mut up = ev.clone();
            up.through.insert(0, base_type.to_string());
            if let Some(dh) = derived_handler(doc, ev) {
                up.handler = dh.to_string();
            }
            up
        })
        .collect()
}

#[note("events-base-arm")]
pub(crate) fn base_arm(
    doc: &Document,
    form_type: &str,
    base_type: &str,
    base_events: &[WiredEvent],
) -> Result<String, Erro> {
    let (arms, overridden) = override_arms(doc, base_type, base_events, "self")?;
    if overridden == 0 {
        return Ok(format!(
            "            {form_type}Fired::Base(fired) => self.base.dispatch(fired),\n"
        ));
    }
    let rest = match overridden < base_events.len() {
        true => "                fired => self.base.dispatch(fired),\n",
        false => "",
    };
    Ok(format!(
        "            {form_type}Fired::Base(fired) => match fired {{\n\
         {arms}\
         {rest}\
         \x20           }},\n"
    ))
}

#[note("events-overrides-base")]
pub(crate) fn overrides_base(doc: &Document, base_events: &[WiredEvent]) -> bool {
    base_events
        .iter()
        .any(|ev| derived_handler(doc, ev).is_some())
}

#[note("events-base-dispatch")]
pub(crate) fn base_dispatch(
    doc: &Document,
    base_type: &str,
    base_events: &[WiredEvent],
    receiver: &str,
    base_path: &str,
    reporting_call: &str,
) -> Result<String, Erro> {
    let (mut match_arms, overridden) = override_arms(doc, base_type, base_events, receiver)?;
    let (had_fired_decl, repaint) = if receiver == "self" {
        (
            "        let crawrad_had_fired = !crawrad_base_fired.is_empty();\n",
            "        if crawrad_had_fired {\n\
             \x20           ui.ctx().request_repaint();\n\
             \x20       }\n",
        )
    } else {
        ("", "")
    };
    if overridden > 0 {
        // with every variant overridden a catch-all arm is unreachable: a
        // warning in the user's project that only the generator could fix
        if overridden < base_events.len() {
            match_arms.push_str(&format!(
                "                fired => {{\n\
                 \x20                   {base_path}.dispatch(fired);\n\
                 \x20               }}\n"
            ));
        }
        Ok(format!(
            "        let crawrad_base_fired = {reporting_call};\n\
             {had_fired_decl}\
             \x20       for fired in crawrad_base_fired {{\n\
             \x20           match fired {{\n\
             {match_arms}\
             \x20           }}\n\
             \x20       }}\n\
             {repaint}"
        ))
    } else {
        Ok(format!(
            "        let crawrad_base_fired = {reporting_call};\n\
             {had_fired_decl}\
             \x20       for fired in crawrad_base_fired {{\n\
             \x20           {base_path}.dispatch(fired);\n\
             \x20       }}\n\
             {repaint}"
        ))
    }
}

#[cfg(test)]
#[path = "test/events/test.rs"]
mod test;
