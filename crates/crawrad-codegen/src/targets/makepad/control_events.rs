//! **The pointer, focus and keyboard events of the form's CONTROLS** —
//! generated, never written per class.
//!
//! Any control whose class declares `OnMouseDown` / `OnMouseUp` /
//! `OnMouseMove` / `OnMouseEnter` / `OnMouseLeave` and has a handler wired gets
//! them: the `App` observes the raw events and `crawrad_runtime_makepad::events::
//! PointerRouter` tells which control each belongs to. The widgets are
//! untouched — no wrapper per component — and a new class, ours or a third
//! party's, gets these events by declaring them in the catalogue.
//!
//! The focus works the same way: a class that declares `OnEnter` CAN take the
//! focus, a press on it gives it the focus (`OnExit` on the one that had it),
//! and `OnKeyDown` / `OnKeyUp` / `OnKeyPress` go to the control that has it —
//! `crawrad_runtime_makepad::events::FocusRouter`.
//!
//! The control's OWN events (a Button's click, a slider's change, a list's
//! selection) stay in the `Bind` table: those are actions of the widget.

use super::binding;
use super::components;
use super::components::codegen::AppState;
use super::layout::{handler, has_widget};
use crate::{Erro, identificador};
use crawrad_ir::{Document, Node};
use crawrad_note::note;

#[note("targets-makepad-control-events-pointer-events")]
pub const POINTER_EVENTS: [(&str, &str); 7] = [
    ("on_mouse_down", "Down"),
    // classic RAD fires it on the press, before the click is decided
    ("on_mouse_activate", "Activate"),
    ("on_mouse_up", "Up"),
    ("on_mouse_move", "Move"),
    ("on_mouse_enter", "Enter"),
    ("on_mouse_leave", "Leave"),
    ("on_mouse_wheel", "Wheel"),
];

#[note("targets-makepad-control-events-resize")]
const RESIZE: &str = "on_resize";

#[note("targets-makepad-control-events-click")]
pub const CLICK: (&str, &str) = ("on_click", "Click");
pub const DBL_CLICK: (&str, &str) = ("on_dbl_click", "DblClick");

#[note("targets-makepad-control-events-focus-events")]
pub const FOCUS_EVENTS: [(&str, &str); 5] = [
    ("on_enter", "Enter"),
    ("on_exit", "Exit"),
    ("on_key_down", "KeyDown"),
    ("on_key_up", "KeyUp"),
    ("on_key_press", "Char"),
];

#[note("targets-makepad-control-events-takes-focus")]
const TAKES_FOCUS: &str = "on_enter";

#[note("targets-makepad-control-events-controlevents")]
pub struct ControlEvents {
    pub app: AppState,
    pub needs_button: bool,
    pub needs_modifiers: bool,
    /// `crawrad_key`, Makepad's key code onto ours.
    pub needs_key: bool,
    /// The App's own widgets this needs in the form's DSL (the tooltip).
    pub dsl: String,
}

#[note("targets-makepad-control-events-control")]
struct Control {
    field: String,
    guard: String,
    focusable: bool,
    focus_widget: String,
    pointer: Vec<(&'static str, String)>,
    focus: Vec<(&'static str, String)>,
    /// The class has a `Hint`: the pointer resting on it shows it.
    hint: bool,
    /// What its secondary press does: open its `PopupMenu`.
    popup: Option<String>,
    /// Its `OnResize` handler.
    resize: Option<String>,
    /// What its click does to the focus: a `Label`'s `FocusControl`.
    focus_link: Option<String>,
}

#[note("targets-makepad-control-events-wired")]
fn wired(
    desc: &crawrad_registry::ComponentDesc,
    n: &Node,
    events: &[(&'static str, &'static str)],
) -> Vec<(&'static str, String)> {
    events
        .iter()
        .filter(|(e, _)| desc.events.iter().any(|d| d.name == *e))
        .filter_map(|(e, kind)| handler(n, e).map(|h| (*kind, h)))
        .collect()
}

#[note("targets-makepad-control-events-push-arm")]
fn push_arm(arms: &mut Vec<(String, String)>, pattern: String, call: String) {
    match arms.iter_mut().find(|(p, _)| *p == pattern) {
        Some((_, c)) => {
            c.push_str(";\n                    ");
            c.push_str(&call);
        }
        None => arms.push((pattern, call)),
    }
}

#[note("targets-makepad-control-events-dispatch")]
fn dispatch(scrutinee: &str, arms: &[(String, String)]) -> String {
    let body = |call: &str| {
        format!(
            "                    {call};\n\
             \x20                   called = true;\n"
        )
    };
    match arms {
        [(pattern, call)] => format!(
            "                if let {pattern} = {scrutinee} {{\n{}                }}\n",
            body(call)
        ),
        _ => {
            let mut m = format!("                match {scrutinee} {{\n");
            for (pattern, call) in arms {
                m.push_str(&format!(
                    "                {pattern} => {{\n{}                }}\n",
                    body(call)
                ));
            }
            m.push_str("                _ => {}\n                }\n");
            m
        }
    }
}

/// Every control that can take the keyboard: the names a `FocusControl`
/// may hold (its id, and its identifier) and the widget that takes it.
fn focus_targets(
    reg: &crawrad_registry::Registry,
    doc: &Document,
    emitted: &[&Node],
) -> Result<Vec<(Vec<String>, String)>, Erro> {
    let mut out = Vec::new();
    for n in emitted {
        let Some(desc) = reg.get(&n.class) else {
            continue;
        };
        if !has_widget(reg, &n.class) || !desc.events.iter().any(|d| d.name == TAKES_FOCUS) {
            continue;
        }
        let field = identificador(n.name())?;
        let widget = components::for_class(&n.class)
            .and_then(|c| c.focus_path(doc, n, &field))
            .unwrap_or_else(|| {
                format!("self.ui.widget(id!({}))", components::text_path(n, &field))
            });
        let mut names = vec![n.id.clone(), field];
        names.dedup();
        out.push((names, widget));
    }
    Ok(out)
}

/// The call a `Label`'s click makes: read its `FocusControl` at run time and
/// focus that control's widget. None on a form with no such control.
fn focus_link(field: &str, targets: &[(Vec<String>, String)]) -> Option<String> {
    let arm = |(names, widget): &(Vec<String>, String)| {
        let pattern: Vec<String> = names.iter().map(|s| crate::literal(s)).collect();
        (pattern.join(" | "), format!("{widget}.set_key_focus(cx)"))
    };
    let read = format!("self.form.{field}.focus_control.as_str()");
    match targets {
        [] => None,
        // one target: a `match` of one arm is clippy's `single_match`
        [only] => {
            let (pattern, call) = arm(only);
            Some(format!(
                "if matches!({read}, {pattern}) {{\n\
                 \x20                       {call};\n\
                 \x20                   }}"
            ))
        }
        _ => {
            let mut m = format!("match {read} {{\n");
            for (pattern, call) in targets.iter().map(arm) {
                m.push_str(&format!("                        {pattern} => {call},\n"));
            }
            m.push_str("                        _ => {}\n                    }");
            Some(m)
        }
    }
}

pub fn control_events(
    reg: &crawrad_registry::Registry,
    doc: &Document,
    emitted: &[&Node],
) -> Result<Option<ControlEvents>, Erro> {
    let targets = focus_targets(reg, doc, emitted)?;
    let mut controls = Vec::new();
    for n in emitted {
        if !has_widget(reg, &n.class) {
            continue;
        }
        let Some(desc) = reg.get(&n.class) else {
            continue;
        };
        let field = identificador(n.name())?;
        // a click the widget does not report itself (no row in the Bind
        // table, no call in its own actions — a third party's included, whose
        // click is the declared `view.finger_up`: a PaintBox, a Shape) is the
        // router's press-and-release
        let mut pointer_events = POINTER_EVENTS.to_vec();
        for ev in [CLICK, DBL_CLICK] {
            let own = match handler(n, ev.0) {
                Some(h) => {
                    binding::binding_of(&n.class, ev.0).is_some()
                        || components::emit_actions_com(reg, doc, n, &field, emitted)?
                            .contains(&format!("self.form.{}()", identificador(&h)?))
                }
                None => true,
            };
            if !own {
                pointer_events.push(ev);
            }
        }
        // a disabled control gets no event at all
        let guard = if components::has_enabled_field(&n.class) {
            format!("if self.form.{field}.enabled ")
        } else {
            String::new()
        };
        let popup = if components::for_class(&n.class)
            .is_some_and(|c| c.struct_definition().contains("pub popup_menu:"))
        {
            super::menus::popup_open(doc, emitted, &field)?
        } else {
            None
        };
        let resize = if desc.events.iter().any(|d| d.name == RESIZE) {
            handler(n, RESIZE)
        } else {
            None
        };
        let focus_widget = components::for_class(&n.class)
            .and_then(|c| c.focus_path(doc, n, &field))
            .unwrap_or_else(|| {
                format!("self.ui.widget(id!({}))", components::text_path(n, &field))
            });
        let focus_link = components::for_class(&n.class)
            .filter(|c| c.struct_definition().contains("pub focus_control:"))
            .and_then(|_| focus_link(&field, &targets));
        controls.push(Control {
            focus_link,
            resize,
            popup,
            pointer: wired(desc, n, &pointer_events),
            focus: wired(desc, n, &FOCUS_EVENTS),
            focusable: desc.events.iter().any(|d| d.name == TAKES_FOCUS),
            focus_widget,
            hint: components::for_class(&n.class)
                .is_some_and(|c| c.struct_definition().contains("pub hint:")),
            field,
            guard,
        });
    }
    let any_focus = controls.iter().any(|c| !c.focus.is_empty());
    // a control is in the list when it has a pointer handler, or when it can
    // take the focus and someone listens to the focus: a press on ANY such
    // control moves it, handler or not; and a control with a `Hint`,
    // whose tooltip shows while the pointer rests on it
    controls.retain(|c| {
        !c.pointer.is_empty()
            || (any_focus && c.focusable)
            || c.hint
            || c.popup.is_some()
            || c.resize.is_some()
            || c.focus_link.is_some()
    });
    if controls.is_empty() {
        return Ok(None);
    }

    let (mut needs_button, mut needs_modifiers, mut needs_key) = (false, false, false);
    // no hint while a menu is open — it would show over the menu
    let no_menu_open = match super::menus::none_open(doc, emitted)? {
        Some(c) => format!(" && {c}"),
        None => String::new(),
    };
    let mut rects = String::new();
    let mut focusable = Vec::new();
    // the widget of each control that can take the focus, to tell
    // which one has the keyboard (a Tab moves it)
    let mut key_widgets = String::new();
    let mut pointer_arms = Vec::new();
    let mut focus_arms = Vec::new();
    let mut sizes = String::new();
    let mut resize_arms = Vec::new();
    for (index, c) in controls.iter().enumerate() {
        let (field, guard) = (&c.field, &c.guard);
        rects.push_str(&format!(
            "            {guard}{{ rects.push(({index}usize, self.ui.widget(id!({field})).area().rect(cx))); }}\n"
        ));
        if let Some(h) = &c.resize {
            sizes.push_str(&format!(
                "            {guard}{{ sizes.push(({index}usize, self.ui.widget(id!({field})).area().rect(cx).size)); }}\n"
            ));
            push_arm(
                &mut resize_arms,
                index.to_string(),
                format!("self.form.{}()", identificador(h)?),
            );
        }
        if c.focusable {
            focusable.push(index.to_string());
            let fw = &c.focus_widget;
            key_widgets.push_str(&format!("                ({index}usize, {fw}),\n"));
        }
        // the pointer comes in, the hint shows (if there is one and
        // ShowHint says so); it goes out or presses, the hint goes
        if c.hint {
            push_arm(
                &mut pointer_arms,
                format!("({index}, CrawradPointer::Enter)"),
                format!(
                    "if CRAWRAD_FORM_SHOW_HINT && self.form.{field}.show_hint && !self.form.{field}.hint.is_empty(){no_menu_open} {{\n\
                     \x20                       self.ui.tooltip(id!(crawrad_tooltip)).show_with_options(cx, hit.at + dvec2(12.0, 20.0), &self.form.{field}.hint);\n\
                     \x20                   }}"
                ),
            );
            for kind in ["Leave", "Down"] {
                push_arm(
                    &mut pointer_arms,
                    format!("({index}, CrawradPointer::{kind})"),
                    "self.ui.tooltip(id!(crawrad_tooltip)).hide(cx)".to_string(),
                );
            }
        }
        // a click on a Label gives its FocusControl the keyboard
        if let Some(link) = &c.focus_link {
            push_arm(
                &mut pointer_arms,
                format!("({index}, CrawradPointer::Click)"),
                link.clone(),
            );
        }
        // the secondary press opens the control's PopupMenu
        if let Some(open) = &c.popup {
            push_arm(
                &mut pointer_arms,
                format!("({index}, CrawradPointer::Down)"),
                open.clone(),
            );
        }
        for (kind, h) in &c.pointer {
            let h = identificador(h)?;
            let call = match *kind {
                "Down" | "Up" => {
                    needs_button = true;
                    needs_modifiers = true;
                    format!(
                        "if let Some(b) = hit.button.and_then(crawrad_button) {{\n\
                         \x20                       self.form.{h}(MouseEvent {{ button: b, x: hit.x, y: hit.y, shift: crawrad_shift(&hit.modifiers) }});\n\
                         \x20                   }}"
                    )
                }
                "Activate" => {
                    needs_button = true;
                    needs_modifiers = true;
                    format!(
                        "if let Some(b) = hit.button.and_then(crawrad_button) {{\n\
                         \x20                       self.form.{h}(MouseActivateEvent {{ button: b, shift: crawrad_shift(&hit.modifiers), x: hit.x, y: hit.y, hit_test: 1 }});\n\
                         \x20                   }}"
                    )
                }
                "Move" => {
                    needs_modifiers = true;
                    format!(
                        "self.form.{h}(MouseMoveEvent {{ x: hit.x, y: hit.y, shift: crawrad_shift(&hit.modifiers) }})"
                    )
                }
                _ => format!("self.form.{h}()"),
            };
            // `Activate` is the press too: one arm, both calls
            let kind = if *kind == "Activate" { "Down" } else { kind };
            push_arm(
                &mut pointer_arms,
                format!("({index}, CrawradPointer::{kind})"),
                call,
            );
        }
        for (kind, h) in &c.focus {
            let h = identificador(h)?;
            let call = match *kind {
                "KeyDown" | "KeyUp" => {
                    needs_key = true;
                    needs_modifiers = true;
                    format!(
                        "if let Some(k) = hit.key {{\n\
                         \x20                       self.form.{h}(KeyEvent {{ key: crawrad_key(k), shift: crawrad_shift(&hit.modifiers) }});\n\
                         \x20                   }}"
                    )
                }
                "Char" => format!(
                    "if let Some(ch) = hit.ch {{\n\
                     \x20                       self.form.{h}(ch);\n\
                     \x20                   }}"
                ),
                _ => format!("self.form.{h}()"),
            };
            focus_arms.push((format!("({index}, CrawradFocus::{kind})"), call));
        }
    }

    let pointer_loop = if pointer_arms.is_empty() {
        String::new()
    } else {
        format!(
            "            for hit in &hits {{\n{}            }}\n",
            dispatch("(hit.control, hit.kind)", &pointer_arms)
        )
    };
    let (focus_route, focus_uses, focus_field) = if focus_arms.is_empty() {
        (String::new(), "", "")
    } else {
        (
            format!(
                "            let crawrad_key_focus = [\n{key_widgets}            ]\n\
                 \x20           .iter()\n\
                 \x20           .find(|(_, w)| w.key_focus(cx))\n\
                 \x20           .map(|(c, _)| *c);\n\
                 \x20           for hit in &self.crawrad_focus.route(event, &hits, &[{}], crawrad_key_focus) {{\n{}            }}\n",
                focusable.join(", "),
                dispatch("(hit.control, hit.kind)", &focus_arms)
            ),
            "use crawrad_runtime_makepad::events::{Focus as CrawradFocus, FocusRouter as CrawradFocusRouter};\n",
            "    #[rust]\n    pub crawrad_focus: CrawradFocusRouter,\n",
        )
    };
    // `OnResize`: the sizes after a draw are read on the next frame, which
    // every draw asks for
    let (resize_route, resize_use, resize_field) = if resize_arms.is_empty() {
        (String::new(), "", "")
    } else {
        (
            format!(
                "            if let Event::Draw(_) = event {{\n\
                 \x20               cx.new_next_frame();\n\
                 \x20           }}\n\
                 \x20           let mut sizes = Vec::new();\n\
                 {sizes}\
                 \x20           for control in self.crawrad_sizes.resized(&sizes) {{\n{}            }}\n",
                dispatch("control", &resize_arms)
            ),
            "use crawrad_runtime_makepad::events::SizeWatch as CrawradSizeWatch;\n",
            "    #[rust]\n    pub crawrad_sizes: CrawradSizeWatch,\n",
        )
    };
    // the kind enum is named only where a pointer arm names it
    let pointer_use = if pointer_arms.is_empty() {
        ""
    } else {
        "Pointer as CrawradPointer, "
    };
    let handle = format!(
        "        // ---- THE CONTROLS' POINTER, FOCUS AND KEYBOARD EVENTS --\n\
         \x20       // Observed, not taken: the widgets still get this event below.\n\
         \x20       {{\n\
         \x20           let mut rects = Vec::new();\n\
         {rects}\
         \x20           let hits = self.crawrad_pointer.route(event, &rects);\n\
         \x20           let mut called = false;\n\
         {pointer_loop}\
         {focus_route}\
         {resize_route}\
         \x20           if called {{\n\
         \x20               self.sync_ui(cx);\n\
         \x20           }}\n\
         \x20       }}\n"
    );
    Ok(Some(ControlEvents {
        app: AppState {
            uses: format!(
                "use crawrad_runtime_makepad::events::{{{pointer_use}PointerRouter as CrawradPointerRouter}};\n{focus_uses}{resize_use}"
            ),
            field: format!(
                "    #[rust]\n    pub crawrad_pointer: CrawradPointerRouter,\n{focus_field}{resize_field}"
            ),
            handle,
        },
        needs_button,
        needs_modifiers,
        needs_key,
        dsl: if controls.iter().any(|c| c.hint)
            || components::grid::any_bound_grid_has_hint(doc, emitted)
        {
            TOOLTIP_DSL.to_string()
        } else {
            String::new()
        },
    }))
}

#[note("targets-makepad-control-events-tooltip-dsl")]
pub const TOOLTIP_DSL: &str = "                crawrad_tooltip = <Tooltip> {\n\
     \x20                   content: <View> {\n\
     \x20                       flow: Overlay, width: Fit, height: Fit,\n\
     \x20                       <RoundedView> {\n\
     \x20                           width: Fit, height: Fit,\n\
     \x20                           padding: { left: 8.0, right: 8.0, top: 4.0, bottom: 4.0 },\n\
     \x20                           draw_bg: { color: #2b2d33, border_size: 1.0, border_color: #4a4d55, border_radius: 3.0 }\n\
     \x20                           tooltip_label = <Label> {\n\
     \x20                               width: Fit,\n\
     \x20                               draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }, color: #e4e5e8 }\n\
     \x20                           }\n\
     \x20                       }\n\
     \x20                   }\n\
     \x20               }\n";

#[cfg(test)]
#[path = "../../test/control_events/test.rs"]
mod tests;
