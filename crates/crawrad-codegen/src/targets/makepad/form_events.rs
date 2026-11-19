//! **The form's own events**, dispatched by the generated Makepad app — the
//! twin of `targets/egui/form_events.rs`, and the same 21 of 22.
//!
//! # Why a twin and not shared code
//!
//! What is shared is already shared, and it is the part that must not diverge:
//! the handler signature (`crate::events::method_params`), the payload types
//! (`crate::events::PAYLOAD_TABLE`, the same text byte for byte on both
//! sides), and the key vocabulary (`crate::events::key_variants`).
//!
//! What is here is the part that **cannot** be shared: egui hands you the
//! frame's whole input in `ui.input()`, Makepad hands you one `Event` at a
//! time in `handle_event`. The house rule is the one the owner wrote — *what
//! the IDE does is classic RAD's, how it is written is Rust's* — and here it reads:
//! the CONTRACT is one, the wiring is each framework's.
//!
//! # Where the two frameworks really differ, measured
//!
//! - **The key vocabulary is not a rename.** Ours is egui's list adopted
//!   whole, so that side is an identity map. Makepad's `KeyCode` is a
//!   different vocabulary with a different shape (`KeyQ`, `Key0`, `Numpad3`),
//!   so this file carries a real translation table — and the things it cannot
//!   say are said as `Key::Unknown`, one by one, with the reason.
//! - **Nothing is immediate here.** A handler that changes a component in
//!   egui shows up on the very next line, because the next line draws it. In
//!   Makepad the widget tree has to be told, so every dispatch that can have
//!   changed something is followed by `sync_ui`.
//! - **`OnPaint` is the exception to that**, and it has to be: firing it on
//!   `Event::Draw` and calling `sync_ui` would mark the tree dirty on every
//!   frame it draws, which is a redraw loop at 100% of a core. It is
//!   dispatched, and what it changes shows on the next frame that draws for
//!   another reason. That sentence goes into the generated file too.
//!
//! # What is NOT dispatched
//!
//! `OnHide`, and for the same reason as the other target: the form IS the
//! window of a generated app, and it is never hidden to come back.

use super::layout::handler;
use crate::{Erro, identificador};
use crawrad_ir::Node;
use crawrad_note::note;

#[note("targets-makepad-form-events-formevents")]
#[derive(Default)]
pub struct FormEvents {
    /// Free items for the top of the designer file.
    pub items: String,
    /// The extra field on the `App` struct, or empty.
    pub field: String,
    /// The block that goes into `AppMain::handle_event`.
    pub handle: String,
    /// The `OnCreate` call inside `AppMain::handle_event`.
    pub on_create: String,
}

impl FormEvents {
    #[note("targets-makepad-form-events-formevents-empty")]
    pub fn empty(&self) -> bool {
        self.handle.is_empty()
    }
}

#[note("targets-makepad-form-events-form-events")]
pub fn form_events(root: &Node) -> Result<FormEvents, Erro> {
    let h = |e: &str| -> Result<Option<String>, Erro> {
        match handler(root, e) {
            Some(alvo) => Ok(Some(identificador(&alvo)?)),
            None => Ok(None),
        }
    };

    let show = h("on_show")?;
    // The size of the form's Canvas for OnPaint
    let form_rect = root
        .rect()
        .unwrap_or(crawrad_ir::Rect::new(0.0, 0.0, 640.0, 480.0));
    let (form_w, form_h) = (form_rect.w, form_rect.h);
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

    // `OnCreate` — the design-time values first, the user's handler last. Same
    // contract as the other target, and classic RAD's: runs once in `handle_event`
    // during initial sync before `sync_ui`, so whatever the handler sets is
    // synchronized before the form is first drawn.
    let on_create = match h("on_create")? {
        Some(f) => format!("            self.form.{f}();\n"),
        None => String::new(),
    };
    let mut out = FormEvents {
        on_create,
        ..Default::default()
    };

    // ---------------------------------------------------------------- state
    // FIELD BY FIELD, and not the whole struct.
    //
    // A field nobody reads is `dead_code` in the generated binary, and the
    // designer is rewritten whole on the next generation — a warning there is
    // a warning aimed at someone who cannot fix it. Measured on a form with
    // only `OnShow` wired: three warnings, all of them this.
    let clicks = click.is_some() || dbl_click.is_some();
    let state: String = [
        (show.is_some(), "    shown: bool,\n"),
        (mouse_enter.is_some(), "    mouse_inside: bool,\n"),
        (clicks, "    press: Option<(DVec2, f64)>,\n"),
        (
            dbl_click.is_some(),
            "    last_click: Option<(DVec2, f64)>,\n",
        ),
    ]
    .iter()
    .filter(|(precisa, _)| *precisa)
    .map(|(_, linha)| *linha)
    .collect();
    if !state.is_empty() {
        out.items.push_str(&format!("{STATE_HEAD}{state}}}\n"));
        out.field
            .push_str("    #[rust]\n    pub crawrad_form: FormState,\n");
    }

    // ------------------------------------------------------------ the block
    let mut arms = String::new();

    if show.is_some() || paint.is_some() {
        arms.push_str("            Event::Draw(_) => {\n");
        if let Some(f) = &show {
            arms.push_str(&format!(
                "                // `OnShow` — once, the first time this form is drawn.\n\
                 \x20               if !self.crawrad_form.shown {{\n\
                 \x20                   self.crawrad_form.shown = true;\n\
                 \x20                   self.form.{f}();\n\
                 \x20                   self.sync_ui(cx);\n\
                 \x20               }}\n"
            ));
        }
        if let Some(f) = &paint {
            arms.push_str(&format!(
                "                // `OnPaint` — once per frame drawn, which is when classic RAD\n\
                 \x20               // calls it.\n\
                 \x20               //\n\
                 \x20               // **No `sync_ui` here, and it cannot be there.** Marking\n\
                 \x20               // the tree dirty from inside a draw is a redraw loop that\n\
                 \x20               // eats a whole core. What you change here shows on the\n\
                 \x20               // next frame that draws for another reason.\n\
                 \x20               // (its Canvas goes to the form's paint box, which\n\
                 \x20               // redraws only when what it drew changed.)\n\
                 {}",
                super::components::paint_box::canvas_to_box(
                    &format!("self.form.{f}(&mut canvas);"),
                    form_w,
                    form_h,
                    "crawrad_form_paint",
                )
                .replace("        ", "                ")
            ));
        }
        arms.push_str("            }\n");
    }

    if let Some(f) = &activate {
        arms.push_str(&format!(
            "            // `OnActivate` — the window became the active one.\n\
             \x20           Event::AppGotFocus => {{\n\
             \x20               self.form.{f}();\n\
             \x20               self.sync_ui(cx);\n\
             \x20           }}\n"
        ));
    }
    if let Some(f) = &deactivate {
        arms.push_str(&format!(
            "            // `OnDeactivate` — another window took the keyboard.\n\
             \x20           Event::AppLostFocus => {{\n\
             \x20               self.form.{f}();\n\
             \x20               self.sync_ui(cx);\n\
             \x20           }}\n"
        ));
    }
    if let Some(f) = &resize {
        arms.push_str(&format!(
            "            // `OnResize` — Makepad reports the window's geometry, and the\n\
             \x20           // form fills the window.\n\
             \x20           Event::WindowGeomChange(_) => {{\n\
             \x20               self.form.{f}();\n\
             \x20               self.sync_ui(cx);\n\
             \x20           }}\n"
        ));
    }

    // ------------------------------------------------------------ the mouse
    if mouse_down.is_some() || mouse_activate.is_some() || clicks {
        arms.push_str("            Event::MouseDown(e) => {\n");
        if let Some(f) = &mouse_down {
            arms.push_str(&format!(
                "                if let Some(b) = crawrad_button(e.button) {{\n\
                 \x20                   self.form.{f}(MouseEvent {{\n\
                 \x20                       button: b,\n\
                 \x20                       x: e.abs.x as f32,\n\
                 \x20                       y: e.abs.y as f32,\n\
                 \x20                       shift: crawrad_shift(&e.modifiers),\n\
                 \x20                   }});\n\
                 \x20               }}\n"
            ));
        }
        if let Some(f) = &mouse_activate {
            arms.push_str(&format!(
                "                // `OnMouseActivate` — classic RAD fires it on the press,\n\
                 \x20               // before the click is decided. `hit_test` is Win32's\n\
                 \x20               // `HTCLIENT`: this form has no frame of its own.\n\
                 \x20               if let Some(b) = crawrad_button(e.button) {{\n\
                 \x20                   self.form.{f}(MouseActivateEvent {{\n\
                 \x20                       button: b,\n\
                 \x20                       shift: crawrad_shift(&e.modifiers),\n\
                 \x20                       x: e.abs.x as f32,\n\
                 \x20                       y: e.abs.y as f32,\n\
                 \x20                       hit_test: 1,\n\
                 \x20                   }});\n\
                 \x20               }}\n"
            ));
        }
        if clicks {
            arms.push_str(
                "                // Where and when the press landed — the click is decided\n\
                 \x20               // on the release, with Makepad's OWN thresholds.\n\
                 \x20               self.crawrad_form.press = Some((e.abs, e.time));\n",
            );
        }
        arms.push_str(
            "                self.sync_ui(cx);\n\
             \x20           }\n",
        );
    }

    if mouse_up.is_some() || clicks {
        arms.push_str("            Event::MouseUp(e) => {\n");
        if let Some(f) = &mouse_up {
            arms.push_str(&format!(
                "                if let Some(b) = crawrad_button(e.button) {{\n\
                 \x20                   self.form.{f}(MouseEvent {{\n\
                 \x20                       button: b,\n\
                 \x20                       x: e.abs.x as f32,\n\
                 \x20                       y: e.abs.y as f32,\n\
                 \x20                       shift: crawrad_shift(&e.modifiers),\n\
                 \x20                   }});\n\
                 \x20               }}\n"
            ));
        }
        if clicks {
            arms.push_str(
                "                // A CLICK is a press and a release close together, and\n\
                 \x20               // `TAP_COUNT_DISTANCE` / `TAP_COUNT_TIME` are Makepad's\n\
                 \x20               // own numbers — borrowed and not invented, so a click\n\
                 \x20               // here and a click in a widget mean the same thing.\n\
                 \x20               if let Some((abs, time)) = self.crawrad_form.press.take()\n\
                 \x20                   && (e.abs - abs).length() < makepad_widgets::makepad_platform::event::finger::TAP_COUNT_DISTANCE\n\
                 \x20               {\n",
            );
            if let Some(f) = &dbl_click {
                arms.push_str(&format!(
                    "                        let double = self\n\
                     \x20                           .crawrad_form\n\
                     \x20                           .last_click\n\
                     \x20                           .is_some_and(|(p, t)| {{\n\
                     \x20                               time - t < makepad_widgets::makepad_platform::event::finger::TAP_COUNT_TIME\n\
                     \x20                                   && (abs - p).length() < makepad_widgets::makepad_platform::event::finger::TAP_COUNT_DISTANCE\n\
                     \x20                           }});\n\
                     \x20                       if double {{\n\
                     \x20                           self.form.{f}();\n\
                     \x20                       }}\n"
                ));
            }
            if let Some(f) = &click {
                arms.push_str(&format!("                        self.form.{f}();\n"));
            }
            // Same rule as the `mouse_inside` of the leave: only `OnDblClick`
            // ever reads this, so with only `OnClick` wired the line does not
            // go out.
            if dbl_click.is_some() {
                arms.push_str(
                    "                        self.crawrad_form.last_click = Some((abs, time));\n",
                );
            }
            arms.push_str("                }\n");
        }
        arms.push_str(
            "                self.sync_ui(cx);\n\
             \x20           }\n",
        );
    }

    if mouse_move.is_some() || mouse_enter.is_some() {
        arms.push_str("            Event::MouseMove(e) => {\n");
        if let Some(f) = &mouse_enter {
            arms.push_str(&format!(
                "                // `OnMouseEnter` — Makepad reports the LEAVE and not the\n\
                 \x20               // enter, so the enter is the first move after one.\n\
                 \x20               if !self.crawrad_form.mouse_inside {{\n\
                 \x20                   self.crawrad_form.mouse_inside = true;\n\
                 \x20                   self.form.{f}();\n\
                 \x20               }}\n"
            ));
        }
        if let Some(f) = &mouse_move {
            arms.push_str(&format!(
                "                self.form.{f}(MouseMoveEvent {{\n\
                 \x20                   x: e.abs.x as f32,\n\
                 \x20                   y: e.abs.y as f32,\n\
                 \x20                   shift: crawrad_shift(&e.modifiers),\n\
                 \x20               }});\n"
            ));
        }
        arms.push_str(
            "                self.sync_ui(cx);\n\
             \x20           }\n",
        );
    }

    if let Some(f) = &mouse_leave {
        // The bookkeeping line only goes out when someone READS it — that is,
        // when `OnMouseEnter` is wired too. Written and never read is a
        // `dead_code` warning in the user's project.
        let arma = if mouse_enter.is_some() {
            "                self.crawrad_form.mouse_inside = false;\n"
        } else {
            ""
        };
        arms.push_str(&format!(
            "            // `OnMouseLeave` — the pointer left the window.\n\
             \x20           Event::MouseLeave(_) => {{\n\
             {arma}\
             \x20               self.form.{f}();\n\
             \x20               self.sync_ui(cx);\n\
             \x20           }}\n"
        ));
    }

    if let Some(f) = &mouse_wheel {
        arms.push_str(&format!(
            "            // `OnMouseWheel` — wheel, trackpad or touch scroll.\n\
             \x20           Event::Scroll(_) => {{\n\
             \x20               self.form.{f}();\n\
             \x20               self.sync_ui(cx);\n\
             \x20           }}\n"
        ));
    }

    // --------------------------------------------------------- the keyboard
    if let Some(f) = &key_down {
        arms.push_str(&format!(
            "            Event::KeyDown(e) => {{\n\
             \x20               self.form.{f}(KeyEvent {{\n\
             \x20                   key: crawrad_key(e.key_code),\n\
             \x20                   shift: crawrad_shift(&e.modifiers),\n\
             \x20               }});\n\
             \x20               self.sync_ui(cx);\n\
             \x20           }}\n"
        ));
    }
    if let Some(f) = &key_up {
        arms.push_str(&format!(
            "            Event::KeyUp(e) => {{\n\
             \x20               self.form.{f}(KeyEvent {{\n\
             \x20                   key: crawrad_key(e.key_code),\n\
             \x20                   shift: crawrad_shift(&e.modifiers),\n\
             \x20               }});\n\
             \x20               self.sync_ui(cx);\n\
             \x20           }}\n"
        ));
    }
    if let Some(f) = &key_press {
        arms.push_str(&format!(
            "            // `OnKeyPress` is the other half of the keyboard: the\n\
             \x20           // CHARACTER that was typed, accents and dead keys already\n\
             \x20           // resolved. In Makepad that is `TextInput`, and it does not\n\
             \x20           // fire for a key that produces no character.\n\
             \x20           Event::TextInput(e) => {{\n\
             \x20               for ch in e.input.chars() {{\n\
             \x20                   self.form.{f}(ch);\n\
             \x20               }}\n\
             \x20               self.sync_ui(cx);\n\
             \x20           }}\n"
        ));
    }

    // ------------------------------------------------------- closing down
    if let Some(f) = &close_query {
        arms.push_str(&format!(
            "            // `OnCloseQuery` gets the VETO, and Makepad takes it: setting\n\
             \x20           // `accept_close` to false keeps the window open, exactly as\n\
             \x20           // in a classic RAD IDE.\n\
             \x20           Event::WindowCloseRequested(e) => {{\n\
             \x20               let mut can_close = true;\n\
             \x20               self.form.{f}(&mut can_close);\n\
             \x20               if !can_close {{\n\
             \x20                   e.accept_close.set(false);\n\
             \x20               }}\n\
             \x20           }}\n"
        ));
    }
    if let Some(f) = &close {
        arms.push_str(&format!(
            "            // `OnClose` — the window is going, and nothing here calls it\n\
             \x20           // off. The veto was the step before.\n\
             \x20           Event::WindowClosed(_) => {{\n\
             \x20               self.form.{f}();\n\
             \x20           }}\n"
        ));
    }
    if let Some(f) = &destroy {
        arms.push_str(&format!(
            "            // `OnDestroy` — the last line this application runs.\n\
             \x20           Event::Shutdown => {{\n\
             \x20               self.form.{f}();\n\
             \x20           }}\n"
        ));
    }

    if !arms.is_empty() {
        out.handle = format!(
            "        // ---- THE FORM'S OWN EVENTS --------------------------------\n\
             \x20       //\n\
             \x20       // Before `self.ui.handle_event`, and that is the classic\n\
             \x20       // `KeyPreview`: the form sees the keyboard and the mouse before\n\
             \x20       // its components do.\n\
             \x20       match event {{\n\
             {arms}\x20           _ => {{}}\n\
             \x20       }}\n"
        );
    }

    // ---------------------------------------------------------- conversions
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

    Ok(out)
}

#[note("targets-makepad-form-events-key-match")]
pub(super) fn key_match() -> String {
    let mut s = String::from(
        "\n\
         /// Makepad's keyboard onto the one `OnKeyDown` and `OnKeyUp` report.\n\
         ///\n\
         /// A few keys have no name in this vocabulary — Caps Lock, Num Lock,\n\
         /// Print Screen, Scroll Lock, Pause and the keypad's `*` — and they come\n\
         /// through as `Key::Unknown`. The modifier keys are reported by Makepad\n\
         /// without a side, so they arrive as the LEFT one.\n\
         fn crawrad_key(k: KeyCode) -> Key {\n\
         \x20   match k {\n",
    );
    for (makepad, nosso) in KEY_TABLE {
        s.push_str(&format!("        KeyCode::{makepad} => Key::{nosso},\n"));
    }
    s.push_str(
        "    }\n\
         }\n",
    );
    s
}

#[note("targets-makepad-form-events-key-table")]
const KEY_TABLE: &[(&str, &str)] = &[
    ("Escape", "Escape"),
    ("Back", "BrowserBack"),
    ("Backtick", "Backtick"),
    ("Key0", "Num0"),
    ("Key1", "Num1"),
    ("Key2", "Num2"),
    ("Key3", "Num3"),
    ("Key4", "Num4"),
    ("Key5", "Num5"),
    ("Key6", "Num6"),
    ("Key7", "Num7"),
    ("Key8", "Num8"),
    ("Key9", "Num9"),
    ("Minus", "Minus"),
    ("Equals", "Equals"),
    ("Backspace", "Backspace"),
    ("Tab", "Tab"),
    ("KeyQ", "Q"),
    ("KeyW", "W"),
    ("KeyE", "E"),
    ("KeyR", "R"),
    ("KeyT", "T"),
    ("KeyY", "Y"),
    ("KeyU", "U"),
    ("KeyI", "I"),
    ("KeyO", "O"),
    ("KeyP", "P"),
    ("LBracket", "OpenBracket"),
    ("RBracket", "CloseBracket"),
    ("ReturnKey", "Enter"),
    ("KeyA", "A"),
    ("KeyS", "S"),
    ("KeyD", "D"),
    ("KeyF", "F"),
    ("KeyG", "G"),
    ("KeyH", "H"),
    ("KeyJ", "J"),
    ("KeyK", "K"),
    ("KeyL", "L"),
    ("Semicolon", "Semicolon"),
    ("Quote", "Quote"),
    ("Backslash", "Backslash"),
    ("KeyZ", "Z"),
    ("KeyX", "X"),
    ("KeyC", "C"),
    ("KeyV", "V"),
    ("KeyB", "B"),
    ("KeyN", "N"),
    ("KeyM", "M"),
    ("Comma", "Comma"),
    ("Period", "Period"),
    ("Slash", "Slash"),
    // Makepad reports no side for the four modifiers. The contract has one,
    // so the left is the answer — and this is the loss, written down.
    ("Control", "ControlLeft"),
    ("Alt", "AltLeft"),
    ("Shift", "ShiftLeft"),
    ("Logo", "SuperLeft"),
    ("Space", "Space"),
    ("Capslock", "Unknown"),
    ("F1", "F1"),
    ("F2", "F2"),
    ("F3", "F3"),
    ("F4", "F4"),
    ("F5", "F5"),
    ("F6", "F6"),
    ("F7", "F7"),
    ("F8", "F8"),
    ("F9", "F9"),
    ("F10", "F10"),
    ("F11", "F11"),
    ("F12", "F12"),
    ("PrintScreen", "Unknown"),
    ("ScrollLock", "Unknown"),
    ("Pause", "Unknown"),
    ("Insert", "Insert"),
    ("Delete", "Delete"),
    ("Home", "Home"),
    ("End", "End"),
    ("PageUp", "PageUp"),
    ("PageDown", "PageDown"),
    // The keypad reports the same `Key` as the number row: egui does not
    // separate them, so the contract does not either.
    ("Numpad0", "Num0"),
    ("Numpad1", "Num1"),
    ("Numpad2", "Num2"),
    ("Numpad3", "Num3"),
    ("Numpad4", "Num4"),
    ("Numpad5", "Num5"),
    ("Numpad6", "Num6"),
    ("Numpad7", "Num7"),
    ("Numpad8", "Num8"),
    ("Numpad9", "Num9"),
    ("NumpadEquals", "Equals"),
    ("NumpadSubtract", "Minus"),
    ("NumpadAdd", "Plus"),
    ("NumpadDecimal", "Period"),
    ("NumpadMultiply", "Unknown"),
    ("NumpadDivide", "Slash"),
    ("Numlock", "Unknown"),
    ("NumpadEnter", "Enter"),
    ("ArrowUp", "ArrowUp"),
    ("ArrowDown", "ArrowDown"),
    ("ArrowLeft", "ArrowLeft"),
    ("ArrowRight", "ArrowRight"),
    ("Unknown", "Unknown"),
];

#[cfg(test)]
#[path = "../../test/form_events_makepad/test.rs"]
mod test;

#[note("targets-makepad-form-events-state-head")]
const STATE_HEAD: &str = r#"
/// What the form's own events remember between events.
///
/// `OnShow` fires once, `OnMouseEnter` is an edge, and a CLICK is a press and
/// a release that have to be compared to each other — none of them can be
/// answered by the event in hand alone.
#[derive(Clone, Copy, Debug, Default)]
pub struct FormState {
"#;

pub(super) const BUTTON: &str = r#"
/// Makepad's `MouseButton` is a set of bits, and it names five buttons.
/// The classic RAD vocabulary — and ours — has three, so the two extra ones are
/// reported as nothing at all rather than as a left click that never happened.
///
/// **The parameter is spelled out with its full path on purpose.** This file
/// defines a `MouseButton` of its own, and a local item beats a glob import:
/// a bare `MouseButton` here would be OURS, and the function would refuse the
/// very value the event hands it.
fn crawrad_button(b: makepad_widgets::MouseButton) -> Option<MouseButton> {
    if b.is_primary() {
        Some(MouseButton::Left)
    } else if b.is_secondary() {
        Some(MouseButton::Right)
    } else if b.is_middle() {
        Some(MouseButton::Middle)
    } else {
        None
    }
}
"#;

pub(super) const MODIFIERS: &str = r#"
/// The three modifiers classic RAD's `TShiftState` reports. Makepad's `logo` — the
/// Windows or Command key — is not among them, because classic RAD does not have
/// it and the contract follows classic RAD here.
fn crawrad_shift(m: &KeyModifiers) -> ShiftState {
    ShiftState {
        shift: m.shift,
        ctrl: m.control,
        alt: m.alt,
    }
}
"#;
