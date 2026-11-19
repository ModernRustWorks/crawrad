//! **The pointer events of ANY control** (every class that
//! declares them).
//!
//! The catalogue declares `OnMouseDown` / `OnMouseUp` / `OnMouseMove` /
//! `OnMouseEnter` / `OnMouseLeave` on many classes, and the Makepad target
//! delivered none of them: makepad 1.0.0's widgets handle the hits inside and
//! report only their own actions (a `Button` says `Clicked`, never where the
//! pointer was). Wrapping each widget would mean one hand-written wrapper per
//! class, and taking the pointer capture from the widget would break its own
//! click.
//!
//! So the generated `App` only OBSERVES the raw events (`Event::MouseDown` /
//! `MouseUp` / `MouseMove` / `MouseLeave`) — the widgets still get every one of
//! them — and a [`PointerRouter`] tells which control each one belongs to, from
//! the rects of the controls that have a handler. The rules are the classic
//! RAD ones:
//!
//! - an event goes to the TOPMOST control under the pointer (the last of the
//!   list: the generator lists them in drawing order);
//! - `x` / `y` are local to that control;
//! - the release goes to the control the press went to, even outside it (the
//!   mouse capture);
//! - `Enter` / `Leave` are the change of the control under the pointer, and
//!   leaving the window leaves it;
//! - a second click on the same control within half a second is a double
//!   click.

use crawrad_note::note;
use makepad_widgets::*;

#[note("events-pointer")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pointer {
    Down,
    Up,
    Move,
    Enter,
    Leave,
    /// A press and a release on the same control (the release inside it).
    Click,
    /// A second click on the same control within [`DOUBLE_CLICK_SECONDS`].
    DblClick,
    /// The wheel turned with the pointer over the control.
    Wheel,
}

#[note("events-double-click-seconds")]
pub const DOUBLE_CLICK_SECONDS: f64 = 0.5;

#[note("events-clickrecord")]
pub type ClickRecord = Option<(usize, f64)>;

#[note("events-second-click")]
pub fn second_click(last: &mut ClickRecord, row: usize, now: f64) -> bool {
    let double = matches!(*last, Some((r, t)) if r == row && now - t <= DOUBLE_CLICK_SECONDS);
    *last = if double { None } else { Some((row, now)) };
    double
}

#[note("events-raw")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Raw {
    Down,
    Up,
    Move,
    /// The wheel turned.
    Wheel,
    /// The pointer left the window.
    Out,
}

#[note("events-pointerhit")]
#[derive(Clone, Debug)]
pub struct PointerHit {
    pub control: usize,
    pub kind: Pointer,
    pub x: f32,
    pub y: f32,
    /// Where the pointer is in the window (a tooltip is placed there).
    pub at: DVec2,
    /// The mouse button, for `Down` and `Up`.
    pub button: Option<MouseButton>,
    pub modifiers: KeyModifiers,
}

#[note("events-pointerrouter")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PointerRouter {
    hovered: Option<usize>,
    pressed: Option<usize>,
    /// The last click: the control and when, for the double click.
    last_click: Option<(usize, u64)>,
}

#[note("events-topmost")]
fn topmost(at: DVec2, rects: &[(usize, Rect)]) -> Option<(usize, Rect)> {
    rects.iter().rev().find(|(_, r)| r.contains(at)).copied()
}

impl PointerRouter {
    #[note("events-pointerrouter-route-at")]
    pub fn route_at(
        &mut self,
        raw: Raw,
        at: DVec2,
        time: f64,
        rects: &[(usize, Rect)],
    ) -> Vec<(usize, Pointer, DVec2)> {
        let local = |i: usize| {
            let r = rects
                .iter()
                .find(|(c, _)| *c == i)
                .map(|(_, r)| r.pos)
                .unwrap_or_default();
            at - r
        };
        let under = topmost(at, rects).map(|(i, _)| i);
        let mut out = Vec::new();
        match raw {
            Raw::Down => {
                self.pressed = under;
                if let Some(i) = under {
                    out.push((i, Pointer::Down, local(i)));
                }
            }
            Raw::Up => {
                let pressed = self.pressed.take();
                if let Some(i) = pressed.or(under) {
                    out.push((i, Pointer::Up, local(i)));
                }
                // released on the very control it was pressed on: a click
                if let Some(i) = pressed.filter(|p| under == Some(*p)) {
                    out.push((i, Pointer::Click, local(i)));
                    // the second click on the same control, soon enough
                    let ms = (time * 1000.0) as u64;
                    match self.last_click {
                        Some((c, t))
                            if c == i
                                && ms.saturating_sub(t) as f64 <= DOUBLE_CLICK_SECONDS * 1000.0 =>
                        {
                            out.push((i, Pointer::DblClick, local(i)));
                            self.last_click = None;
                        }
                        _ => self.last_click = Some((i, ms)),
                    }
                }
            }
            Raw::Move => {
                if under != self.hovered {
                    if let Some(i) = self.hovered {
                        out.push((i, Pointer::Leave, local(i)));
                    }
                    if let Some(i) = under {
                        out.push((i, Pointer::Enter, local(i)));
                    }
                    self.hovered = under;
                }
                if let Some(i) = under {
                    out.push((i, Pointer::Move, local(i)));
                }
            }
            Raw::Wheel => {
                if let Some(i) = under {
                    out.push((i, Pointer::Wheel, local(i)));
                }
            }
            Raw::Out => {
                if let Some(i) = self.hovered.take() {
                    out.push((i, Pointer::Leave, local(i)));
                }
            }
        }
        out
    }

    #[note("events-pointerrouter-route")]
    pub fn route(&mut self, event: &Event, rects: &[(usize, Rect)]) -> Vec<PointerHit> {
        let (raw, at, time, button, modifiers) = match event {
            Event::MouseDown(e) => (Raw::Down, e.abs, e.time, Some(e.button), e.modifiers),
            Event::MouseUp(e) => (Raw::Up, e.abs, e.time, Some(e.button), e.modifiers),
            Event::MouseMove(e) => (Raw::Move, e.abs, e.time, None, e.modifiers),
            Event::MouseLeave(e) => (Raw::Out, e.abs, e.time, None, e.modifiers),
            Event::Scroll(e) => (Raw::Wheel, e.abs, e.time, None, e.modifiers),
            _ => return Vec::new(),
        };
        self.route_at(raw, at, time, rects)
            .into_iter()
            .map(|(control, kind, p)| PointerHit {
                control,
                kind,
                x: p.x as f32,
                y: p.y as f32,
                at,
                button: matches!(kind, Pointer::Down | Pointer::Up)
                    .then_some(button)
                    .flatten(),
                modifiers,
            })
            .collect()
    }
}

#[note("events-sizewatch")]
#[derive(Clone, Debug, Default)]
pub struct SizeWatch {
    sizes: Vec<(usize, DVec2)>,
}

impl SizeWatch {
    #[note("events-sizewatch-resized")]
    pub fn resized(&mut self, sizes: &[(usize, DVec2)]) -> Vec<usize> {
        let mut out = Vec::new();
        for &(c, size) in sizes {
            if size == DVec2::default() {
                continue;
            }
            match self.sizes.iter_mut().find(|(i, _)| *i == c) {
                Some((_, s)) if *s != size => {
                    *s = size;
                    out.push(c);
                }
                Some(_) => {}
                None => self.sizes.push((c, size)),
            }
        }
        out
    }
}

#[note("events-focus")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Focus {
    /// The control got the focus (`OnEnter`).
    Enter,
    /// The control lost it (`OnExit`).
    Exit,
    KeyDown,
    KeyUp,
    /// A typed character (`OnKeyPress`), dead keys already resolved.
    Char,
}

#[note("events-focushit")]
#[derive(Clone, Debug)]
pub struct FocusHit {
    pub control: usize,
    pub kind: Focus,
    /// The key, for `KeyDown` / `KeyUp`.
    pub key: Option<KeyCode>,
    /// The character, for `Char`.
    pub ch: Option<char>,
    pub modifiers: KeyModifiers,
}

#[note("events-focusrouter")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FocusRouter {
    focused: Option<usize>,
    /// The control whose widget had the keyboard focus at the last event:
    /// only a CHANGE of it moves the focus, so a press on a panel is not
    /// undone by the edit that keeps the keyboard.
    key_focus: Option<usize>,
}

impl FocusRouter {
    #[note("events-focusrouter-focused")]
    pub fn focused(&self) -> Option<usize> {
        self.focused
    }

    #[note("events-focusrouter-focus")]
    pub fn focus(&mut self, to: usize) -> Vec<(usize, Focus)> {
        if self.focused == Some(to) {
            return Vec::new();
        }
        let mut out = Vec::new();
        if let Some(old) = self.focused.replace(to) {
            out.push((old, Focus::Exit));
        }
        out.push((to, Focus::Enter));
        out
    }

    #[note("events-focusrouter-route")]
    pub fn route(
        &mut self,
        event: &Event,
        pointer: &[PointerHit],
        focusable: &[usize],
        key_focus: Option<usize>,
    ) -> Vec<FocusHit> {
        let hit = |control: usize, kind: Focus, modifiers: KeyModifiers| FocusHit {
            control,
            kind,
            key: None,
            ch: None,
            modifiers,
        };
        let mut out = Vec::new();
        for p in pointer {
            if p.kind == Pointer::Down && focusable.contains(&p.control) {
                for (c, k) in self.focus(p.control) {
                    out.push(hit(c, k, p.modifiers));
                }
            }
        }
        // the Tab that moved it returned before this router ran: the change
        // is seen on the next event, its own key release
        if key_focus != self.key_focus {
            self.key_focus = key_focus;
            if let Some(c) = key_focus.filter(|c| focusable.contains(c)) {
                for (c, k) in self.focus(c) {
                    out.push(hit(c, k, KeyModifiers::default()));
                }
            }
        }
        let Some(to) = self.focused else {
            return out;
        };
        match event {
            Event::KeyDown(e) => out.push(FocusHit {
                key: Some(e.key_code),
                ..hit(to, Focus::KeyDown, e.modifiers)
            }),
            Event::KeyUp(e) => out.push(FocusHit {
                key: Some(e.key_code),
                ..hit(to, Focus::KeyUp, e.modifiers)
            }),
            Event::TextInput(e) => {
                for ch in e.input.chars() {
                    out.push(FocusHit {
                        ch: Some(ch),
                        ..hit(to, Focus::Char, KeyModifiers::default())
                    });
                }
            }
            _ => {}
        }
        out
    }
}
#[cfg(test)]
#[path = "test/events/test.rs"]
mod tests;
