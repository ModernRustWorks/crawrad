// THE TAB ORDER in the egui target.
//
// Same authoring contract as `control_router.rs` (read the header of
// `desenho_comum.rs`): this text is copied into the user's
// `<form>.designer.rs`, so no `use`, no `//!`, egui paths spelled out and
// English identifiers. It goes into the `designer.rs` of a form with a
// control that can take the focus (`crawrad-codegen`,
// `targets/egui/tab_order.rs`).
//
// Here it is compiled only in the tests (`lib.rs`): outside them nothing in
// the runtime calls it, and an item with no caller is `dead_code`.

// [CRAWRAD:INICIO tab_order]

/// One control in the Tab sequence, as the form leaves it while it draws:
/// its place (the `TabOrder` of each container down to it, then its own),
/// its rect, whether it keeps Tab for itself (a `Memo` with `WantTabs`), and
/// its name (what a `Label`'s `FocusControl` holds).
type CrawradTabStop = (&'static [i64], egui::Rect, bool, &'static str);

/// **Tab and Shift+Tab walk the controls in `TabOrder`**, as in classic RAD,
/// and not in the order they are drawn.
///
/// egui moves the focus in the order widgets ask for it, which is the
/// drawing order — the Z order here. So the form keeps its own sequence:
/// while it draws, each control that can take the focus leaves its stop
/// ([`CrawradTabStop`]); [`Self::after`] sorts them and finds the widget of each
/// one; and on the next frame [`Self::before`], which runs before any
/// control, answers a Tab by focusing the next control and cancelling egui's
/// own move.
#[derive(Default)]
struct CrawradTabOrder {
    /// The last frame's sequence: each control's widget, its rect, whether
    /// it keeps Tab, and its name.
    stops: Vec<(egui::Id, egui::Rect, bool, &'static str)>,
}

impl CrawradTabOrder {
    /// Before the controls draw: a Tab pressed this frame focuses the next
    /// control in the sequence, a Shift+Tab the previous one, wrapping round.
    /// Left to egui: a focus on a widget that is no control's (a menu), and
    /// a control that keeps Tab.
    fn before(&self, ui: &egui::Ui) {
        let forward = ui.input(|i| {
            i.events.iter().rev().find_map(|e| match e {
                egui::Event::Key {
                    key: egui::Key::Tab,
                    pressed: true,
                    modifiers,
                    ..
                } if !modifiers.any() => Some(true),
                egui::Event::Key {
                    key: egui::Key::Tab,
                    pressed: true,
                    modifiers,
                    ..
                } if modifiers.shift_only() => Some(false),
                _ => None,
            })
        });
        let Some(forward) = forward else {
            return;
        };
        let focused = ui.memory(|m| m.focused());
        let at = match focused {
            None => None,
            Some(id) => {
                // the widget itself, or one inside a control (a grid's cell)
                let corner = ui.ctx().viewport(|v| {
                    v.prev_pass
                        .widgets
                        .get(id)
                        .map(|w| crawrad_tab_corner(w.rect))
                });
                let found = self.stops.iter().position(|s| s.0 == id).or_else(|| {
                    let c = corner?;
                    self.stops.iter().position(|s| s.1.contains(c))
                });
                match found {
                    Some(i) => Some(i),
                    None => return,
                }
            }
        };
        if at.is_some_and(|i| self.stops[i].2) {
            return;
        }
        if let Some(next) = crawrad_tab_next(at, self.stops.len(), forward) {
            let id = self.stops[next].0;
            ui.memory_mut(|m| {
                m.request_focus(id);
                m.move_focus(egui::FocusDirection::None);
            });
        }
    }

    /// After the controls drew: the sequence for the next frame. Each stop
    /// takes the first widget that can take the focus and starts inside its
    /// rect and inside no smaller stop — a panel's edit belongs to the edit,
    /// not to the panel. A stop with no such widget is left out.
    fn after(&mut self, ui: &egui::Ui, stops: &mut [CrawradTabStop]) {
        // stable: equal places keep the drawing order, as classic RAD does
        stops.sort_by(|a, b| a.0.cmp(b.0));
        let widgets: Vec<(egui::Id, egui::Rect)> = ui.ctx().viewport(|v| {
            v.this_pass
                .widgets
                .get_layer(ui.layer_id())
                .filter(|w| w.sense.is_focusable() && w.enabled)
                .map(|w| (w.id, w.rect))
                .collect()
        });
        self.stops.clear();
        for (i, (_, rect, keeps_tab, name)) in stops.iter().enumerate() {
            let own = widgets.iter().find(|(_, w)| {
                let c = crawrad_tab_corner(*w);
                let inner = |j: usize| {
                    let (_, other, _, _) = &stops[j];
                    j != i && other.contains(c) && other.area() < rect.area()
                };
                rect.contains(c) && !(0..stops.len()).any(inner)
            });
            if let Some((id, _)) = own {
                self.stops.push((*id, *rect, *keeps_tab, name));
            }
        }
    }
}

/// Where a widget starts, just inside its top-left corner: the control's
/// rect holds it even when the widget grows past it (a multiline edit asks
/// for more rows than the rect has).
fn crawrad_tab_corner(widget: egui::Rect) -> egui::Pos2 {
    widget.min + egui::vec2(0.5, 0.5)
}

/// The stop after `at` in a sequence of `count` (before it, backwards),
/// wrapping round; with no focus, the first (backwards, the last).
fn crawrad_tab_next(at: Option<usize>, count: usize, forward: bool) -> Option<usize> {
    if count == 0 {
        return None;
    }
    Some(match (at, forward) {
        (None, true) => 0,
        (None, false) => count - 1,
        (Some(i), true) => (i + 1) % count,
        (Some(i), false) => (i + count - 1) % count,
    })
}
// [CRAWRAD:FIM tab_order]

// Only in a form with a `Label`, the one caller (else it is `dead_code`).
// [CRAWRAD:INICIO tab_focus]
impl CrawradTabOrder {
    /// A click on a `Label`: its `FocusControl`, by name, takes the
    /// keyboard. A name that is no stop (hidden, disabled, gone) does nothing.
    fn focus(&self, ui: &egui::Ui, name: &str) {
        if let Some(stop) = self.stops.iter().find(|s| s.3 == name) {
            let id = stop.0;
            ui.memory_mut(|m| m.request_focus(id));
        }
    }
}
// [CRAWRAD:FIM tab_focus]
