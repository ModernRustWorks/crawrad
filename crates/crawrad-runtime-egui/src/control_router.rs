// THE CONTROL EVENT ROUTER on the egui target — counterpart to
// `crawrad_runtime_makepad::events`, with the same rules.
//
// Same authoring contract as `desenho_comum.rs` (read its header): this
// text is copied into the user's `<form>.designer.rs`, so no
// `use`, no `//!`, fully qualified egui paths, and identifiers in
// English. It only enters the `designer.rs` of a form where some control
// has an event that the widget itself does not deliver (`crawrad-codegen`,
// `targets/egui/control_events.rs`).
//
// Here it is compiled only in tests (`lib.rs`): outside them nothing in the runtime
// calls it, and an item without callers is `dead_code`.

// [CRAWRAD:INICIO control_router]

/// What a control gets from the pointer, as [`CrawradControlRouter`] routes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CrawradPointer {
    /// A press, with its button.
    Down(egui::PointerButton),
    /// The release, with its button. It goes to the control the press went
    /// to, even outside it: the mouse capture.
    Up(egui::PointerButton),
    Move,
    Enter,
    Leave,
    /// A press and a release on the same control.
    Click,
    /// A second click on the same control within
    /// [`CRAWRAD_DOUBLE_CLICK_SECONDS`].
    DblClick,
    /// The wheel turned with the pointer over the control.
    Wheel,
}

/// What a control gets from the focus and the keyboard.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CrawradFocus {
    Enter,
    Exit,
    KeyDown(egui::Key),
    KeyUp(egui::Key),
    /// A typed character, dead keys already resolved.
    Char(char),
}

/// One raw input, already told whether it happened over this form (and not
/// over a popup or a window drawn on top of it).
#[derive(Clone, Copy, Debug, PartialEq)]
enum CrawradRaw {
    Press(egui::Pos2, egui::PointerButton),
    /// Where, the button, and whether it was over this form.
    Release(egui::Pos2, egui::PointerButton, bool),
    Move(egui::Pos2),
    /// The wheel turned with the pointer here.
    Wheel(egui::Pos2),
    /// The pointer left the form.
    Gone,
    /// A key, and whether it went down.
    Key(egui::Key, bool),
    Char(char),
    /// egui moved its keyboard focus to a widget that starts here: a Tab
    /// or a widget that asked for it.
    Focused(egui::Pos2),
}

/// The pointer events of one frame, as `(control, kind, local position)`.
type CrawradPointerHits = Vec<(usize, CrawradPointer, egui::Vec2)>;

/// The focus and keyboard events of one frame, as `(control, kind)`.
type CrawradFocusHits = Vec<(usize, CrawradFocus)>;

/// The classic double-click time.
const CRAWRAD_DOUBLE_CLICK_SECONDS: f64 = 0.5;

/// **The pointer, focus and keyboard events of the form's controls.**
///
/// egui's widgets report their own actions (a button's click, an edit's
/// change) and nothing else. So the form OBSERVES the frame's raw input, and
/// this router tells which control each event belongs to, from the rects the
/// controls left behind while drawing. The rules are the classic RAD ones:
///
/// - an event goes to the TOPMOST control under the pointer: the last of the
///   list, which is in drawing order;
/// - the positions are local to that control;
/// - the release goes to the control the press went to (the capture);
/// - `Enter` / `Leave` are the change of the control under the pointer;
/// - a press on a control that CAN take the focus gives it the focus (`Exit`
///   to the one that had it, `Enter` to it), a press anywhere else leaves the
///   focus where it is, and the keys go to the control that has it;
/// - so does egui moving its own focus into such a control: Tab and
///   Shift+Tab fire `Exit` and `Enter` as a press does.
#[derive(Clone, Debug, Default)]
struct CrawradControlRouter {
    hovered: Option<usize>,
    pressed: Option<usize>,
    /// The last click: the control and when, for the double click.
    last_click: Option<(usize, f64)>,
    focused: Option<usize>,
    /// The widget egui had focused when the last frame was routed: only a
    /// CHANGE of it moves the control focus, so a press on a panel is not
    /// undone by the edit that keeps egui's focus.
    widget_focus: Option<egui::Id>,
}

impl CrawradControlRouter {
    /// This frame's events for the controls in `rects` (`(control, rect)`, in
    /// drawing order); `focusable` are the controls that can take the focus.
    fn route(
        &mut self,
        ui: &egui::Ui,
        rects: &[(usize, egui::Rect)],
        focusable: &[usize],
    ) -> (CrawradPointerHits, CrawradFocusHits) {
        let layer = ui.layer_id();
        let ctx = ui.ctx().clone();
        // a popup or a window over the form takes its pointer
        let on_form = |p: egui::Pos2| ctx.layer_id_at(p).is_none_or(|l| l == layer);
        let (events, time, hover) = ui.input(|i| (i.events.clone(), i.time, i.pointer.hover_pos()));
        let mut pointer = Vec::new();
        let mut focus = Vec::new();
        for event in &events {
            let raws: Vec<CrawradRaw> = match event {
                egui::Event::PointerButton {
                    pos,
                    button,
                    pressed: true,
                    ..
                } if on_form(*pos) => vec![CrawradRaw::Press(*pos, *button)],
                egui::Event::PointerButton {
                    pos,
                    button,
                    pressed: false,
                    ..
                } => vec![CrawradRaw::Release(*pos, *button, on_form(*pos))],
                egui::Event::PointerMoved(pos) if on_form(*pos) => vec![CrawradRaw::Move(*pos)],
                egui::Event::PointerMoved(_) | egui::Event::PointerGone => vec![CrawradRaw::Gone],
                egui::Event::MouseWheel { .. } => hover
                    .filter(|p| on_form(*p))
                    .map(CrawradRaw::Wheel)
                    .into_iter()
                    .collect(),
                egui::Event::Key { key, pressed, .. } => vec![CrawradRaw::Key(*key, *pressed)],
                egui::Event::Text(typed) => typed.chars().map(CrawradRaw::Char).collect(),
                _ => Vec::new(),
            };
            for raw in raws {
                self.step(raw, time, rects, focusable, &mut pointer, &mut focus);
            }
        }
        let now = ui.memory(|m| m.focused());
        if now != self.widget_focus {
            self.widget_focus = now;
            let corner = now.and_then(|id| {
                ctx.viewport(|v| {
                    v.this_pass
                        .widgets
                        .get(id)
                        .map(|w| w.rect.min + egui::vec2(0.5, 0.5))
                })
            });
            if let Some(p) = corner {
                self.step(
                    CrawradRaw::Focused(p),
                    time,
                    rects,
                    focusable,
                    &mut pointer,
                    &mut focus,
                );
            }
        }
        (pointer, focus)
    }

    /// Gives the focus to `c`: `Exit` to the control that had it, `Enter` to
    /// `c`; nothing when `c` has it already or cannot take it.
    fn focus_to(&mut self, c: usize, focusable: &[usize], focus: &mut CrawradFocusHits) {
        if focusable.contains(&c) && self.focused != Some(c) {
            if let Some(old) = self.focused.replace(c) {
                focus.push((old, CrawradFocus::Exit));
            }
            focus.push((c, CrawradFocus::Enter));
        }
    }

    /// The pure core: what one raw input means for the controls.
    fn step(
        &mut self,
        raw: CrawradRaw,
        time: f64,
        rects: &[(usize, egui::Rect)],
        focusable: &[usize],
        pointer: &mut CrawradPointerHits,
        focus: &mut CrawradFocusHits,
    ) {
        let under = |p: egui::Pos2| {
            rects
                .iter()
                .rev()
                .find(|(_, r)| r.contains(p))
                .map(|(c, _)| *c)
        };
        let local = |c: usize, p: egui::Pos2| {
            rects
                .iter()
                .find(|(i, _)| *i == c)
                .map(|(_, r)| p - r.min)
                .unwrap_or_default()
        };
        match raw {
            CrawradRaw::Press(p, button) => {
                self.pressed = under(p);
                if let Some(c) = self.pressed {
                    pointer.push((c, CrawradPointer::Down(button), local(c, p)));
                    self.focus_to(c, focusable, focus);
                }
            }
            CrawradRaw::Release(p, button, over_form) => {
                let pressed = self.pressed.take();
                let here = if over_form { under(p) } else { None };
                if let Some(c) = pressed.or(here) {
                    pointer.push((c, CrawradPointer::Up(button), local(c, p)));
                }
                // released on the very control it was pressed on: a click
                if let Some(c) = pressed.filter(|c| here == Some(*c)) {
                    pointer.push((c, CrawradPointer::Click, local(c, p)));
                    match self.last_click {
                        Some((last, t))
                            if last == c && time - t <= CRAWRAD_DOUBLE_CLICK_SECONDS =>
                        {
                            pointer.push((c, CrawradPointer::DblClick, local(c, p)));
                            self.last_click = None;
                        }
                        _ => self.last_click = Some((c, time)),
                    }
                }
            }
            CrawradRaw::Move(p) => {
                let now = under(p);
                if now != self.hovered {
                    if let Some(c) = self.hovered {
                        pointer.push((c, CrawradPointer::Leave, local(c, p)));
                    }
                    if let Some(c) = now {
                        pointer.push((c, CrawradPointer::Enter, local(c, p)));
                    }
                    self.hovered = now;
                }
                if let Some(c) = now {
                    pointer.push((c, CrawradPointer::Move, local(c, p)));
                }
            }
            CrawradRaw::Wheel(p) => {
                if let Some(c) = under(p) {
                    pointer.push((c, CrawradPointer::Wheel, local(c, p)));
                }
            }
            CrawradRaw::Gone => {
                if let Some(c) = self.hovered.take() {
                    pointer.push((c, CrawradPointer::Leave, egui::Vec2::ZERO));
                }
            }
            CrawradRaw::Key(key, down) => {
                // the keys of a control that is not drawn (hidden, disabled)
                // go nowhere
                if let Some(c) = self.focused.filter(|c| rects.iter().any(|(i, _)| i == c)) {
                    let kind = if down {
                        CrawradFocus::KeyDown(key)
                    } else {
                        CrawradFocus::KeyUp(key)
                    };
                    focus.push((c, kind));
                }
            }
            CrawradRaw::Char(ch) => {
                if let Some(c) = self.focused.filter(|c| rects.iter().any(|(i, _)| i == c)) {
                    focus.push((c, CrawradFocus::Char(ch)));
                }
            }
            CrawradRaw::Focused(p) => {
                // the topmost control that can take the focus: an edit on a
                // panel, not the panel
                let owner = rects
                    .iter()
                    .rev()
                    .find(|(c, r)| r.contains(p) && focusable.contains(c))
                    .map(|(c, _)| *c);
                if let Some(c) = owner {
                    self.focus_to(c, focusable, focus);
                }
            }
        }
    }
}
// [CRAWRAD:FIM control_router]

// [CRAWRAD:INICIO size_watch]

/// **Classic RAD's `OnResize` for the form's controls**: the size each one
/// was last drawn with, and which changed. Copied into the designer file only
/// when a control has an `OnResize` handler.
#[derive(Clone, Debug, Default)]
struct CrawradSizeWatch {
    sizes: Vec<(usize, egui::Vec2)>,
}

impl CrawradSizeWatch {
    /// The controls of `sizes` (`(control, size)`) whose size changed since
    /// the last frame they were drawn in. The first size a control shows is
    /// remembered, not announced; an empty one is a control not drawn yet.
    fn resized(&mut self, sizes: &[(usize, egui::Vec2)]) -> Vec<usize> {
        let mut out = Vec::new();
        for &(c, size) in sizes {
            if size == egui::Vec2::ZERO {
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
// [CRAWRAD:FIM size_watch]
