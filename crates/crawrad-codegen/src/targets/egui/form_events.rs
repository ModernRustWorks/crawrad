//! **The form's own events**, dispatched by the generated app — the classic
//! RAD `TForm` half of the Object Inspector's event tab.
//!
//! # Why this file exists
//!
//! Measured on 17/09/2026, and it is the of the 31 events some
//! class in the catalogue offers, the generated egui app fired **five** —
//! `on_click`, `on_change`, `on_timer`, `on_create` and `on_close`. Everything
//! else went the whole way and stopped one step short: the Object Inspector
//! wired it, the `.crawform` stored it, the generator wrote the handler with
//! the right signature — and the application never called the function.
//!
//! That is the interface lying at its largest scale, because every visible
//! part of the path works. The user writes the body, runs, presses the key,
//! and nothing happens.
//!
//! The form alone declares **22** of those 31, including all eight that carry
//! a parameter, so this is where the number moves.
//!
//! # Why the form, and not each component
//!
//! The classic RAD answer is `KeyPreview`: the form sees the keyboard and the mouse
//! before its controls do. egui gives exactly that shape — `ui.input()` is the
//! frame's whole input, unfiltered — so the form's events cost one block at
//! the top of `show()` and no change to a single component.
//!
//! A component's own `OnKeyDown` is a different question (it needs focus, and
//! it needs the component's `Response`), and it is a different pass.
//!
//! # What is NOT dispatched, and why it is written here
//!
//! `OnHide`. The form **is** the window in a generated app: it is never
//! hidden and shown again, so a dispatch for it would be a line that can
//! never run. Saying so costs a paragraph; discovering it costs an afternoon.

use crate::targets::egui::layout::handler;
use crate::{Erro, identificador};
use crawrad_ir::Node;
use crawrad_note::note;

#[note("targets-egui-form-events-formevents")]
#[derive(Default)]
pub struct FormEvents {
    /// Free items for the top of the designer file: the frame-to-frame state
    /// and the conversions from egui's vocabulary to ours.
    pub items: String,
    /// The field the form struct carries, or empty.
    pub field: String,
    /// That field's initialiser inside `Default`, or empty.
    pub default: String,
    /// The block that runs at the top of `show()`, before the components —
    /// which is exactly what `KeyPreview` means.
    pub before: String,
    /// The block that runs at the bottom of `show()`, after the components.
    pub after: String,
    /// The whole body of `new()`.
    ///
    /// `OnCreate` is the one form event that does not happen in a frame, so it
    /// cannot live in `show()`. It lives here rather than back in the form
    /// generator because **which form events the application fires has to have
    /// one answer**: with the create dispatch in one file and the other twenty
    /// in another, a sweep that asks "does every event become a call?" has to
    /// know about both, and the day it stops knowing is the day one of them
    /// goes quiet.
    pub new_body: String,
}

impl FormEvents {
    #[note("targets-egui-form-events-formevents-empty")]
    pub fn empty(&self) -> bool {
        self.before.is_empty() && self.after.is_empty()
    }
}

#[note("targets-egui-form-events-form-events")]
pub fn form_events(root: &Node) -> Result<FormEvents, Erro> {
    let h = |e: &str| -> Result<Option<String>, Erro> {
        match handler(root, e) {
            Some(alvo) => Ok(Some(identificador(&alvo)?)),
            None => Ok(None),
        }
    };

    let show = h("on_show")?;
    let activate = h("on_activate")?;
    let deactivate = h("on_deactivate")?;
    let resize = h("on_resize")?;
    let paint = h("on_paint")?;
    let click = h("on_click")?;
    let dbl_click = h("on_dbl_click")?;
    let mouse_enter = h("on_mouse_enter")?;
    let mouse_leave = h("on_mouse_leave")?;
    let mouse_wheel = h("on_mouse_wheel")?;
    let mouse_down = h("on_mouse_down")?;
    let mouse_up = h("on_mouse_up")?;
    let mouse_move = h("on_mouse_move")?;
    let mouse_activate = h("on_mouse_activate")?;
    let key_down = h("on_key_down")?;
    let key_up = h("on_key_up")?;
    let key_press = h("on_key_press")?;
    let close_query = h("on_close_query")?;
    let close = h("on_close")?;
    let destroy = h("on_destroy")?;

    let mut out = FormEvents::default();
    let mut before = String::new();

    // `OnCreate` — the design-time values first, the user's handler last. That
    // order is the contract, and it is classic RAD's: whatever the handler sets
    // wins, because it runs after.
    out.new_body = match h("on_create")? {
        Some(f) => format!(
            "        let mut form = Self::default();\n\
             \x20       form.{f}();\n\
             \x20       form\n"
        ),
        None => "        Self::default()\n".to_string(),
    };

    // ---------------------------------------------------------------- state
    //
    // Four of these events are EDGES — they fire on a change, not on a
    // condition — so they need last frame to compare against.
    //
    // FIELD BY FIELD, and not the whole struct: a field nobody reads is
    // `dead_code` in the generated binary, and the designer is rewritten whole
    // on the next generation — a warning there is aimed at someone who cannot
    // fix it.
    let remembers_show = show.is_some();
    let remembers_focus = activate.is_some() || deactivate.is_some();
    let remembers_size = resize.is_some();
    let remembers_mouse = mouse_enter.is_some() || mouse_leave.is_some();
    let state: String = [
        (remembers_show, "    shown: bool,\n"),
        (remembers_size, "    size: egui::Vec2,\n"),
        (remembers_focus, "    focused: bool,\n"),
        (remembers_mouse, "    mouse_inside: bool,\n"),
    ]
    .iter()
    .filter(|(precisa, _)| *precisa)
    .map(|(_, linha)| *linha)
    .collect();
    if !state.is_empty() {
        out.items.push_str(&format!("{STATE_HEAD}{state}}}\n"));
        out.field.push_str("    crawrad_form: FormState,\n");
        out.default
            .push_str("            crawrad_form: FormState::default(),\n");
    }

    // ------------------------------------------------------------ the block
    if !(show.is_none()
        && activate.is_none()
        && deactivate.is_none()
        && resize.is_none()
        && click.is_none()
        && dbl_click.is_none()
        && mouse_enter.is_none()
        && mouse_leave.is_none()
        && mouse_wheel.is_none()
        && mouse_down.is_none()
        && mouse_up.is_none()
        && mouse_move.is_none()
        && mouse_activate.is_none()
        && key_down.is_none()
        && key_up.is_none()
        && key_press.is_none()
        && close_query.is_none()
        && close.is_none()
        && destroy.is_none())
    {
        before.push_str(
            "        // ---- THE FORM'S OWN EVENTS ----------------------------------\n\
             \x20       //\n\
             \x20       // Before the components, and that is the classic `KeyPreview`: the\n\
             \x20       // form sees the keyboard and the mouse first.\n\
             \x20       let form_rect = ui.max_rect();\n",
        );
    }

    if let Some(f) = &show {
        before.push_str(&format!(
            "        // `OnShow` — once, on the first frame this form is drawn.\n\
             \x20       if !self.crawrad_form.shown {{\n\
             \x20           self.crawrad_form.shown = true;\n\
             \x20           self.{f}();\n\
             \x20       }}\n"
        ));
    }

    if remembers_focus {
        before.push_str(
            "        // `OnActivate` / `OnDeactivate` — the window gaining or losing\n\
             \x20       // the keyboard. The first frame counts as gaining it, which is\n\
             \x20       // what a classic RAD IDE does when the form opens.\n\
             \x20       let form_focused = ui.input(|i| i.focused);\n\
             \x20       if form_focused != self.crawrad_form.focused {\n\
             \x20           self.crawrad_form.focused = form_focused;\n",
        );
        match (&activate, &deactivate) {
            (Some(a), Some(d)) => before.push_str(&format!(
                "            if form_focused {{\n\
                 \x20               self.{a}();\n\
                 \x20           }} else {{\n\
                 \x20               self.{d}();\n\
                 \x20           }}\n"
            )),
            (Some(a), None) => before.push_str(&format!(
                "            if form_focused {{\n\
                 \x20               self.{a}();\n\
                 \x20           }}\n"
            )),
            (None, Some(d)) => before.push_str(&format!(
                "            if !form_focused {{\n\
                 \x20               self.{d}();\n\
                 \x20           }}\n"
            )),
            (None, None) => unreachable!("remembers_focus is exactly these two"),
        }
        before.push_str("        }\n");
    }

    if let Some(f) = &resize {
        before.push_str(&format!(
            "        // `OnResize` — every time the form's size actually changes,\n\
             \x20       // including the first one, when it goes from nothing to the\n\
             \x20       // designer size. A classic RAD IDE fires it then too.\n\
             \x20       if form_rect.size() != self.crawrad_form.size {{\n\
             \x20           self.crawrad_form.size = form_rect.size();\n\
             \x20           self.{f}();\n\
             \x20       }}\n"
        ));
    }

    if remembers_mouse {
        before.push_str(
            "        // `OnMouseEnter` / `OnMouseLeave` — the pointer crossing the\n\
             \x20       // form's edge. An EDGE, not a state: it fires on the frame the\n\
             \x20       // crossing happens and not on every frame after it.\n\
             \x20       let mouse_inside = ui\n\
             \x20           .input(|i| i.pointer.hover_pos())\n\
             \x20           .is_some_and(|p| form_rect.contains(p));\n\
             \x20       if mouse_inside != self.crawrad_form.mouse_inside {\n\
             \x20           self.crawrad_form.mouse_inside = mouse_inside;\n",
        );
        if let Some(f) = &mouse_enter {
            before.push_str(&format!(
                "            if mouse_inside {{\n\
                 \x20               self.{f}();\n\
                 \x20           }}\n"
            ));
        }
        if let Some(f) = &mouse_leave {
            before.push_str(&format!(
                "            if !mouse_inside {{\n\
                 \x20               self.{f}();\n\
                 \x20           }}\n"
            ));
        }
        before.push_str("        }\n");
    }

    if let Some(f) = &mouse_wheel {
        before.push_str(&format!(
            "        // `OnMouseWheel` — the RAW wheel event, and not egui's\n\
             \x20       // `smooth_scroll_delta`: that one keeps decaying for several\n\
             \x20       // frames after the gesture, and it would fire your handler\n\
             \x20       // over and over for one turn of the wheel.\n\
             \x20       if ui.input(|i| {{\n\
             \x20           i.events\n\
             \x20               .iter()\n\
             \x20               .any(|e| matches!(e, egui::Event::MouseWheel {{ .. }}))\n\
             \x20       }}) {{\n\
             \x20           self.{f}();\n\
             \x20       }}\n"
        ));
    }

    if click.is_some() || dbl_click.is_some() {
        before.push_str(
            "        // `OnClick` / `OnDblClick` — on the form's BACKGROUND, which is\n\
             \x20       // what clicking the form means in a classic RAD IDE. This interaction is\n\
             \x20       // registered before the components, so any component drawn on\n\
             \x20       // top of the same point takes the click instead.\n\
             \x20       let form_background = ui.interact(\n\
             \x20           form_rect,\n\
             \x20           ui.id().with(\"crawrad_form_background\"),\n\
             \x20           egui::Sense::click(),\n\
             \x20       );\n",
        );
        if let Some(f) = &click {
            before.push_str(&format!(
                "        if form_background.clicked() {{\n\
                 \x20           self.{f}();\n\
                 \x20       }}\n"
            ));
        }
        if let Some(f) = &dbl_click {
            before.push_str(&format!(
                "        if form_background.double_clicked() {{\n\
                 \x20           self.{f}();\n\
                 \x20       }}\n"
            ));
        }
    }

    // ----------------------------------------------- the frame's raw events
    //
    // One pass over `i.events`, because that is where the keyboard and the
    // mouse both are. The coordinates handed to the user are RELATIVE TO THE
    // FORM — the same numbers the designer showed while drawing it, and not
    // the screen's.
    let uses_raw_input = key_down.is_some()
        || key_up.is_some()
        || key_press.is_some()
        || mouse_down.is_some()
        || mouse_up.is_some()
        || mouse_move.is_some()
        || mouse_activate.is_some();
    if uses_raw_input {
        before.push_str(
            "        // The frame's raw input, in one pass. The coordinates handed to\n\
             \x20       // your handler are RELATIVE TO THE FORM — the very numbers the\n\
             \x20       // designer showed you, not the screen's.\n\
             \x20       let form_origin = form_rect.min;\n\
             \x20       let form_modifiers = ui.input(|i| i.modifiers);\n\
             \x20       let form_input = ui.input(|i| i.events.clone());\n\
             \x20       for form_event in &form_input {\n\
             \x20           match form_event {\n",
        );
        if key_down.is_some() || key_up.is_some() {
            before.push_str(
                "                egui::Event::Key {\n\
                 \x20                   key,\n\
                 \x20                   pressed,\n\
                 \x20                   modifiers,\n\
                 \x20                   ..\n\
                 \x20               } => {\n\
                 \x20                   let e = KeyEvent {\n\
                 \x20                       key: crawrad_key(*key),\n\
                 \x20                       shift: crawrad_shift(modifiers),\n\
                 \x20                   };\n",
            );
            if let Some(f) = &key_down {
                before.push_str(&format!(
                    "                    if *pressed {{\n\
                     \x20                       self.{f}(e);\n\
                     \x20                   }}\n"
                ));
            }
            if let Some(f) = &key_up {
                before.push_str(&format!(
                    "                    if !*pressed {{\n\
                     \x20                       self.{f}(e);\n\
                     \x20                   }}\n"
                ));
            }
            before.push_str("                }\n");
        }
        if let Some(f) = &key_press {
            before.push_str(&format!(
                "                // `OnKeyPress` is the other half of the keyboard: the\n\
                 \x20               // CHARACTER that was typed, accents and dead keys\n\
                 \x20               // already resolved.\n\
                 \x20               egui::Event::Text(typed) => {{\n\
                 \x20                   for ch in typed.chars() {{\n\
                 \x20                       self.{f}(ch);\n\
                 \x20                   }}\n\
                 \x20               }}\n"
            ));
        }
        if mouse_down.is_some() || mouse_up.is_some() || mouse_activate.is_some() {
            before.push_str(
                "                egui::Event::PointerButton {\n\
                 \x20                   pos,\n\
                 \x20                   button,\n\
                 \x20                   pressed,\n\
                 \x20                   modifiers,\n\
                 \x20               } => {\n\
                 \x20                   // A button this vocabulary has no name for — the\n\
                 \x20                   // two extra ones some mice carry — is not an\n\
                 \x20                   // event we can describe, so it is not reported.\n\
                 \x20                   if let Some(b) = crawrad_button(*button) {\n",
            );
            if mouse_down.is_some() || mouse_up.is_some() {
                before.push_str(
                    "                        let e = MouseEvent {\n\
                     \x20                           button: b,\n\
                     \x20                           x: pos.x - form_origin.x,\n\
                     \x20                           y: pos.y - form_origin.y,\n\
                     \x20                           shift: crawrad_shift(modifiers),\n\
                     \x20                       };\n",
                );
                if let Some(f) = &mouse_down {
                    before.push_str(&format!(
                        "                        if *pressed {{\n\
                         \x20                           self.{f}(e);\n\
                         \x20                       }}\n"
                    ));
                }
                if let Some(f) = &mouse_up {
                    before.push_str(&format!(
                        "                        if !*pressed {{\n\
                         \x20                           self.{f}(e);\n\
                         \x20                       }}\n"
                    ));
                }
            }
            if let Some(f) = &mouse_activate {
                before.push_str(&format!(
                    "                        // `OnMouseActivate` — classic RAD fires it on the\n\
                     \x20                       // press, before the click is decided.\n\
                     \x20                       // `hit_test` is Win32's `HTCLIENT`: this\n\
                     \x20                       // form has no frame of its own to report.\n\
                     \x20                       if *pressed {{\n\
                     \x20                           self.{f}(MouseActivateEvent {{\n\
                     \x20                               button: b,\n\
                     \x20                               shift: crawrad_shift(modifiers),\n\
                     \x20                               x: pos.x - form_origin.x,\n\
                     \x20                               y: pos.y - form_origin.y,\n\
                     \x20                               hit_test: 1,\n\
                     \x20                           }});\n\
                     \x20                       }}\n"
                ));
            }
            before.push_str(
                "                    }\n\
                 \x20               }\n",
            );
        }
        if let Some(f) = &mouse_move {
            before.push_str(&format!(
                "                // `PointerMoved` carries no modifiers of its own, so the\n\
                 \x20               // ones in force this frame are the honest answer.\n\
                 \x20               egui::Event::PointerMoved(pos) => {{\n\
                 \x20                   self.{f}(MouseMoveEvent {{\n\
                 \x20                       x: pos.x - form_origin.x,\n\
                 \x20                       y: pos.y - form_origin.y,\n\
                 \x20                       shift: crawrad_shift(&form_modifiers),\n\
                 \x20                   }});\n\
                 \x20               }}\n"
            ));
        }
        before.push_str(
            "                _ => {}\n\
             \x20           }\n\
             \x20       }\n",
        );
    }

    // ------------------------------------------------------- closing down
    //
    // The three of them are one gesture, and the ORDER is the contract:
    // `OnCloseQuery` may refuse, and `OnClose`/`OnDestroy` only run once it
    // has not.
    if close_query.is_some() || close.is_some() || destroy.is_some() {
        before.push_str(
            "        // The window close was asked for — the X, Alt+F4, the task bar.\n\
             \x20       if ui.input(|i| i.viewport().close_requested()) {\n",
        );
        if let Some(f) = &close_query {
            before.push_str(&format!(
                "            // `OnCloseQuery` gets the VETO. Set it to `false` and the\n\
                 \x20           // window stays open, exactly as in a classic RAD IDE.\n\
                 \x20           let mut can_close = true;\n\
                 \x20           self.{f}(&mut can_close);\n\
                 \x20           if can_close {{\n"
            ));
        }
        let inner = if close_query.is_some() {
            "            "
        } else {
            "        "
        };
        if let Some(f) = &close {
            before.push_str(&format!("{inner}    self.{f}();\n"));
        }
        if let Some(f) = &destroy {
            before.push_str(&format!(
                "{inner}    // `OnDestroy` — the last line this form runs.\n\
                 {inner}    self.{f}();\n"
            ));
        }
        if close_query.is_some() {
            before.push_str(
                "            } else {\n\
                 \x20               ui.ctx()\n\
                 \x20                   .send_viewport_cmd(egui::ViewportCommand::CancelClose);\n\
                 \x20           }\n",
            );
        }
        before.push_str("        }\n");
    }

    // ---------------------------------------------------------- conversions
    if uses_raw_input {
        if key_down.is_some() || key_up.is_some() {
            out.items.push_str(&key_match());
        }
        if mouse_down.is_some() || mouse_up.is_some() || mouse_activate.is_some() {
            out.items.push_str(BUTTON);
        }
        if key_down.is_some()
            || key_up.is_some()
            || mouse_down.is_some()
            || mouse_up.is_some()
            || mouse_move.is_some()
            || mouse_activate.is_some()
        {
            out.items.push_str(MODIFIERS);
        }
    }

    if let Some(f) = &paint {
        // `OnPaint` draws on the form's Canvas, once per frame, UNDER
        // the controls (it runs before them), which is the classic RAD IDE's
        // `Form.Canvas`
        before.push_str(&format!(
            "        // `OnPaint` — once per frame, under the controls.\n\
             \x20       {{\n\
             \x20           let form_rect = ui.max_rect();\n\
             \x20           let mut canvas = Canvas::new(form_rect.width(), form_rect.height());\n\
             \x20           self.{f}(&mut canvas);\n\
             {}\
             \x20       }}\n",
            super::components::paint_box::replay("canvas", "form_rect")
        ));
    }

    out.before = before;
    Ok(out)
}

#[note("targets-egui-form-events-control-key-items")]
pub fn control_key_items(form: &FormEvents) -> String {
    conversion_items(form, false, true, true)
}

#[note("targets-egui-form-events-conversion-items")]
pub fn conversion_items(form: &FormEvents, button: bool, modifiers: bool, key: bool) -> String {
    let mut s = String::new();
    if key && !form.items.contains("fn crawrad_key(") {
        s.push_str(&key_match());
    }
    if modifiers && !form.items.contains("fn crawrad_shift(") {
        s.push_str(MODIFIERS);
    }
    if button && !form.items.contains("fn crawrad_button(") {
        s.push_str(BUTTON);
    }
    s
}

#[note("targets-egui-form-events-key-match")]
fn key_match() -> String {
    let mut s = String::from(
        "\n\
         /// egui's key vocabulary is the one `Key` was taken from, so this is a\n\
         /// rename and never a decision.\n\
         ///\n\
         /// **There is no `_` arm, and that is deliberate.** `egui::Key` is not\n\
         /// `#[non_exhaustive]`, so this match is exhaustive: the day egui adds a\n\
         /// key, this stops compiling and says which one. The alternative — a\n\
         /// wildcard falling to `Key::Unknown` — is an `unreachable pattern`\n\
         /// warning today and a key silently reported as unknown tomorrow.\n\
         ///\n\
         /// `Key::Unknown` still exists in the contract: the other target's\n\
         /// keyboard has one.\n\
         fn crawrad_key(k: egui::Key) -> Key {\n\
         \x20   match k {\n",
    );
    for v in crate::events::key_variants() {
        if v == "Unknown" {
            continue;
        }
        s.push_str(&format!("        egui::Key::{v} => Key::{v},\n"));
    }
    s.push_str(
        "    }\n\
         }\n",
    );
    s
}

#[note("targets-egui-form-events-state-head")]
const STATE_HEAD: &str = r#"
/// What the form's own events remember between frames.
///
/// `OnShow` fires once, `OnResize` only when the size really changed, and
/// `OnMouseEnter`/`OnMouseLeave` are edges — all of them need to know what the
/// previous frame looked like, and a frame has nowhere else to keep it.
#[derive(Clone, Copy, Debug, Default)]
struct FormState {
"#;

const BUTTON: &str = r#"
/// egui reports two extra buttons some mice carry. The classic RAD vocabulary — and
/// ours — has three, so the extras are reported as nothing at all rather than
/// as a left click that never happened.
fn crawrad_button(b: egui::PointerButton) -> Option<MouseButton> {
    match b {
        egui::PointerButton::Primary => Some(MouseButton::Left),
        egui::PointerButton::Secondary => Some(MouseButton::Right),
        egui::PointerButton::Middle => Some(MouseButton::Middle),
        _ => None,
    }
}
"#;

#[cfg(test)]
#[path = "../../test/form_events/test.rs"]
mod test;

const MODIFIERS: &str = r#"
/// The three modifiers classic RAD's `TShiftState` reports. egui's `command` and
/// `mac_cmd` are not among them: on Windows and Linux `command` IS `ctrl`, and
/// reporting it twice would make `ctrl` true for a reason the user did not ask
/// about.
fn crawrad_shift(m: &egui::Modifiers) -> ShiftState {
    ShiftState {
        shift: m.shift,
        ctrl: m.ctrl,
        alt: m.alt,
    }
}
"#;
