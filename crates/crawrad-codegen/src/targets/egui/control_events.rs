//! **The pointer, focus and keyboard events of the form's controls** in the
//! egui target — generated, never written per class. The twin of the Makepad
//! target's `control_events`, with the same rules.
//!
//! egui's widgets report their own actions (a button's click, an edit's
//! change) and nothing else, so a class that declares `OnMouseDown` or
//! `OnEnter` was mute here unless its emitter wired that one event by hand.
//! Now each control leaves its rect behind while it draws, and after the
//! components the form hands the frame's raw input to `CrawradControlRouter`
//! (`crawrad-runtime-egui/src/control_router.rs`, copied into the designer file),
//! which tells which control each event belongs to.
//!
//! An event the control's own emitter already calls (a `Button`'s click, an
//! `Edit`'s keys) stays with the emitter: the router takes only what nobody
//! else delivers, so nothing fires twice.

use super::recorte;
use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};
use crawrad_ir::{Node, NodeId};
use crawrad_note::note;
use std::collections::BTreeMap;

#[note("targets-egui-control-events-router")]
const ROUTER: &str = include_str!("../../../../crawrad-runtime-egui/src/control_router.rs");

#[note("targets-egui-control-events-pointer-events")]
const POINTER_EVENTS: [(&str, &str); 9] = [
    ("on_mouse_activate", "Activate"),
    ("on_mouse_down", "Down"),
    ("on_mouse_up", "Up"),
    ("on_mouse_move", "Move"),
    ("on_mouse_enter", "Enter"),
    ("on_mouse_leave", "Leave"),
    ("on_click", "Click"),
    ("on_dbl_click", "DblClick"),
    ("on_mouse_wheel", "Wheel"),
];

#[note("targets-egui-control-events-resize")]
const RESIZE: &str = "on_resize";

#[note("targets-egui-control-events-focus-events")]
const FOCUS_EVENTS: [(&str, &str); 5] = [
    ("on_enter", "Enter"),
    ("on_exit", "Exit"),
    ("on_key_down", "KeyDown"),
    ("on_key_up", "KeyUp"),
    ("on_key_press", "Char"),
];

#[note("targets-egui-control-events-takes-focus")]
const TAKES_FOCUS: &str = "on_enter";

#[note("targets-egui-control-events-control")]
struct Control {
    index: usize,
    pointer: Vec<(&'static str, String)>,
    focus: Vec<(&'static str, String)>,
    /// Its `OnResize` handler.
    resize: Option<String>,
}

#[note("targets-egui-control-events-controlevents")]
pub struct ControlEvents {
    controls: BTreeMap<NodeId, Control>,
    focusable: Vec<usize>,
}

#[note("targets-egui-control-events-routed")]
fn routed(
    desc: &crawrad_registry::ComponentDesc,
    n: &Node,
    body: &str,
    events: &[(&'static str, &'static str)],
) -> Result<Vec<(&'static str, String)>, Erro> {
    let mut out = Vec::new();
    for (event, kind) in events {
        if !desc.events.iter().any(|d| d.name == *event) {
            continue;
        }
        let Some(h) = handler(n, event) else {
            continue;
        };
        let h = identificador(&h)?;
        if !body.contains(&format!("self.{h}(")) {
            out.push((*kind, h));
        }
    }
    Ok(out)
}

#[note("targets-egui-control-events-plan")]
pub fn plan(
    reg: &crawrad_registry::Registry,
    drawn: &[&Node],
    body: impl Fn(&Node) -> Result<String, Erro>,
) -> Result<Option<ControlEvents>, Erro> {
    let mut found = Vec::new();
    for n in drawn {
        let Some(desc) = reg.get(&n.class) else {
            continue;
        };
        let candidate = POINTER_EVENTS
            .iter()
            .chain(FOCUS_EVENTS.iter())
            .any(|(e, _)| handler(n, e).is_some())
            || handler(n, RESIZE).is_some();
        let (pointer, focus, resize) = if candidate {
            let own = body(n)?;
            (
                routed(desc, n, &own, &POINTER_EVENTS)?,
                routed(desc, n, &own, &FOCUS_EVENTS)?,
                routed(desc, n, &own, &[(RESIZE, "Resize")])?
                    .pop()
                    .map(|(_, h)| h),
            )
        } else {
            (Vec::new(), Vec::new(), None)
        };
        let focusable = desc.events.iter().any(|d| d.name == TAKES_FOCUS);
        found.push((n.id.clone(), pointer, focus, focusable, resize));
    }
    // a press on ANY control that can take the focus moves it, handler or
    // not, so with someone listening to the focus they are all in the list
    let any_focus = found.iter().any(|(_, _, f, _, _)| !f.is_empty());
    let mut controls = BTreeMap::new();
    let mut focusable = Vec::new();
    for (id, pointer, focus, can_focus, resize) in found {
        if pointer.is_empty() && resize.is_none() && !(any_focus && can_focus) {
            continue;
        }
        let index = controls.len();
        if can_focus && any_focus {
            focusable.push(index);
        }
        controls.insert(
            id,
            Control {
                index,
                pointer,
                focus,
                resize,
            },
        );
    }
    if controls.is_empty() {
        return Ok(None);
    }
    Ok(Some(ControlEvents {
        controls,
        focusable,
    }))
}

#[note("targets-egui-control-events-push-arm")]
fn push_arm(arms: &mut Vec<(String, String)>, pattern: String, call: String) {
    match arms.iter_mut().find(|(p, _)| *p == pattern) {
        Some((_, c)) => c.push_str(&call),
        None => arms.push((pattern, call)),
    }
}

#[note("targets-egui-control-events-dispatch")]
fn dispatch(scrutinee: &str, arms: &[(String, String)]) -> String {
    match arms {
        [(pattern, call)] => {
            // the calls are written for a `match` arm, one level deeper
            let call: String = call
                .lines()
                .map(|l| format!("{}\n", l.strip_prefix("    ").unwrap_or(l)))
                .collect();
            format!("                if let {pattern} = {scrutinee} {{\n{call}                }}\n")
        }
        _ => {
            let mut m = format!("                match {scrutinee} {{\n");
            for (pattern, call) in arms {
                m.push_str(&format!(
                    "                    {pattern} => {{\n{call}                    }}\n"
                ));
            }
            m.push_str("                    _ => {}\n                }\n");
            m
        }
    }
}

impl ControlEvents {
    #[note("targets-egui-control-events-controlevents-rect-line")]
    pub fn rect_line(&self, n: &Node, name: &str) -> String {
        match self.controls.get(&n.id) {
            Some(c) => format!(
                "        if self.{name}.visible && self.{name}.enabled {{\n\
                 \x20           crawrad_rects.push(({i}, r.intersect(ui.clip_rect())));\n\
                 {size}\
                 \x20       }}\n",
                i = c.index,
                // the whole size, not the clipped one: scrolling is no resize
                size = if c.resize.is_some() {
                    format!("            crawrad_sizes.push(({}, r.size()));\n", c.index)
                } else {
                    String::new()
                }
            ),
            None => String::new(),
        }
    }

    #[note("targets-egui-control-events-controlevents-any-resize")]
    fn any_resize(&self) -> bool {
        self.controls.values().any(|c| c.resize.is_some())
    }

    #[note("targets-egui-control-events-controlevents-before")]
    pub fn before(&self) -> String {
        let mut s = "        // The controls' rects, in drawing order: the router's map of\n\
         \x20       // who is on top (see `CrawradControlRouter`).\n\
         \x20       let mut crawrad_rects: Vec<(usize, egui::Rect)> = Vec::new();\n"
            .to_string();
        if self.any_resize() {
            s.push_str("        let mut crawrad_sizes: Vec<(usize, egui::Vec2)> = Vec::new();\n");
        }
        s
    }

    #[note("targets-egui-control-events-controlevents-field")]
    pub fn field(&self) -> (&'static str, &'static str) {
        if self.any_resize() {
            (
                "    crawrad_router: CrawradControlRouter,\n    crawrad_sizes: CrawradSizeWatch,\n",
                "            crawrad_router: CrawradControlRouter::default(),\n            crawrad_sizes: CrawradSizeWatch::default(),\n",
            )
        } else {
            (
                "    crawrad_router: CrawradControlRouter,\n",
                "            crawrad_router: CrawradControlRouter::default(),\n",
            )
        }
    }

    #[note("targets-egui-control-events-controlevents-items")]
    pub fn items(&self) -> String {
        let mut s = recorte(ROUTER, "control_router");
        if self.any_resize() {
            s.push_str(&recorte(ROUTER, "size_watch"));
        }
        s
    }

    #[note("targets-egui-control-events-controlevents-after")]
    pub fn after(&self) -> (String, bool, bool, bool) {
        let (mut button, mut modifiers, mut key, mut at) = (false, false, false, false);
        let mut pointer_arms = Vec::new();
        let mut focus_arms = Vec::new();
        for c in self.controls.values() {
            let i = c.index;
            for (kind, h) in &c.pointer {
                let (pattern, call) = match *kind {
                    "Activate" | "Down" | "Up" => {
                        let (button_kind, payload) = if *kind == "Activate" {
                            (
                                "Down",
                                "MouseActivateEvent { button: b, shift: crawrad_shift(&modifiers), x: at.x, y: at.y, hit_test: 1 }"
                                    .to_string(),
                            )
                        } else {
                            (
                                *kind,
                                "MouseEvent { button: b, x: at.x, y: at.y, shift: crawrad_shift(&modifiers) }"
                                    .to_string(),
                            )
                        };
                        (button, modifiers, at) = (true, true, true);
                        (
                            format!("({i}, CrawradPointer::{button_kind}(button))"),
                            format!(
                                "                        if let Some(b) = crawrad_button(button) {{\n\
                                 \x20                           self.{h}({payload});\n\
                                 \x20                       }}\n"
                            ),
                        )
                    }
                    "Move" => {
                        (modifiers, at) = (true, true);
                        (
                            format!("({i}, CrawradPointer::Move)"),
                            format!(
                                "                        self.{h}(MouseMoveEvent {{ x: at.x, y: at.y, shift: crawrad_shift(&modifiers) }});\n"
                            ),
                        )
                    }
                    _ => (
                        format!("({i}, CrawradPointer::{kind})"),
                        format!("                        self.{h}();\n"),
                    ),
                };
                push_arm(&mut pointer_arms, pattern, call);
            }
            for (kind, h) in &c.focus {
                let (pattern, call) = match *kind {
                    "KeyDown" | "KeyUp" => {
                        (modifiers, key) = (true, true);
                        (
                            format!("({i}, CrawradFocus::{kind}(k))"),
                            format!(
                                "                        self.{h}(KeyEvent {{ key: crawrad_key(k), shift: crawrad_shift(&modifiers) }});\n"
                            ),
                        )
                    }
                    "Char" => (
                        format!("({i}, CrawradFocus::Char(ch))"),
                        format!("                        self.{h}(ch);\n"),
                    ),
                    _ => (
                        format!("({i}, CrawradFocus::{kind})"),
                        format!("                        self.{h}();\n"),
                    ),
                };
                push_arm(&mut focus_arms, pattern, call);
            }
        }
        let pointer_loop = if pointer_arms.is_empty() {
            String::new()
        } else {
            format!(
                "            for (control, kind, {}) in pointer {{\n{}            }}\n",
                if at { "at" } else { "_" },
                dispatch("(control, kind)", &pointer_arms)
            )
        };
        let focus_loop = if focus_arms.is_empty() {
            String::new()
        } else {
            format!(
                "            for (control, kind) in focus {{\n{}            }}\n",
                dispatch("(control, kind)", &focus_arms)
            )
        };
        let mut resize_arms = Vec::new();
        for c in self.controls.values() {
            if let Some(h) = &c.resize {
                push_arm(
                    &mut resize_arms,
                    c.index.to_string(),
                    format!("                        self.{h}();\n"),
                );
            }
        }
        let resize_loop = if resize_arms.is_empty() {
            String::new()
        } else {
            format!(
                "            for control in self.crawrad_sizes.resized(&crawrad_sizes) {{\n{}            }}\n",
                dispatch("control", &resize_arms)
            )
        };
        let focusable: Vec<String> = self.focusable.iter().map(|i| i.to_string()).collect();
        let code = format!(
            "        // ---- THE CONTROLS' POINTER, FOCUS AND KEYBOARD EVENTS -------\n\
             \x20       // Observed, not taken: the widgets above already had this\n\
             \x20       // frame's input. The router tells which control each event\n\
             \x20       // belongs to; an event a control's own widget reports is not\n\
             \x20       // routed, so nothing fires twice.\n\
             \x20       {{\n\
             \x20           let ({}, {}) = self.crawrad_router.route(ui, &crawrad_rects, &[{}]);\n\
             {}\
             {pointer_loop}\
             {focus_loop}\
             {resize_loop}\
             \x20       }}\n",
            if pointer_arms.is_empty() {
                "_"
            } else {
                "pointer"
            },
            if focus_arms.is_empty() { "_" } else { "focus" },
            focusable.join(", "),
            if modifiers {
                "            let modifiers = ui.input(|i| i.modifiers);\n"
            } else {
                ""
            },
        );
        (code, button, modifiers, key)
    }
}

#[cfg(test)]
#[path = "../../test/egui_control_events/test.rs"]
mod tests;
