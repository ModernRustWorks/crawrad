// SINGLE SOURCE OF DRAWING — a file with TWO readers.
//
// This file is compiled here, via `include!` in `crawrad-runtime-egui/src/lib.rs`,
// AND is copied — verbatim, byte for byte — into the `<form>.designer.rs` that
// `crawrad-codegen` writes in the user's project. The same text, compiled twice,
// in two contexts. It is ADR 0011 applied to drawing: one fact with two
// readers, not a rule written twice.
//
// The `include!` is not laziness: it is the ONLY way that requires not a single extra word
// in the text. A `mod` would force each item here to be `pub(crate)` for
// `lib.rs` to reach it, and that visibility — which only serves our crate
// arrangement — would travel along into everyone's `designer.rs`. The text is the
// product; the runtime bends. `cargo clippy` still sees everything,
// because `include!` is real compilation — and so does `cargo fmt`, via the
// `#[cfg(any())] mod desenho_comum;` right below the `include!` in
// `lib.rs`, which exists solely for rustfmt to find the path.
//
// The user's project continues to know `eframe` and nothing else. This is NOT a
// dependency: it is generation, just like `designer.rs`. See `crawrad-codegen/src/rxp.rs`.
//
// ## Authoring rules, and all were measured
//
// 1. **No `//!` and no `#![...]` in this file.** In the user's project
//    this text enters the MIDDLE of `designer.rs`, after `use
//    eframe::egui;` — and an inner attribute or module doc after an item is
//    `error[E0753]`. Internal docs live in this `//` block, which sits outside
//    every section and is therefore never emitted.
// 2. **No `use` here.** egui paths are fully qualified
//    (`egui::Rect`, not `Rect`) because on one side `egui` is the crate and on the other
//    it is the `use eframe::egui;` written by the generator. A `use` here would not
//    resolve in both places.
// 3. **No hardcoded colors.** Colors come from `ui.visuals()`, and never from a
//    literal. This is the owner's decision on theming: the day
//    the theme becomes a form property, a file containing `Color32::from_rgb`
//    would need to be rewritten entirely.
// 4. **No internal types.** Neither `Ctl`, nor `crawrad_ir`, nor `crawrad_registry`:
//    none of that exists on the other side. Only `egui`, `std`, and the types declared
//    by this file itself.
// 5. **Identifiers are in ENGLISH.** This code originates inside the user's
//    project, and user code is English — the same reason why
//    `rxp.rs` emits `visible`/`enabled` and not `visivel`/`habilitado`. Comments
//    OUTSIDE sections (like this one) are internal, and stay in Portuguese.
//
// ## Sections, and why they exist
//
// The generator does not copy the entire file: it clips the section the
// form needs, via the `[CRAWRAD:INICIO name]` / `[CRAWRAD:FIM name]` markers.
// A form without `PageControl` does not get the tab strip, and a form without
// `Timer` does not get the tick — `designer.rs` is a file the user opens and
// reads, and ninety lines they do not use are ninety lines of noise.
//
// Marker lines are NOT emitted: what is output is exactly what is between
// them. Clipping is done by `crawrad_codegen::rxp::trecho`, and there are tests for
// unbalanced markers, repeated sections, and sections that no one emits.

// [CRAWRAD:INICIO abas]

/// Height of the tab strip.
///
/// This is the same number the designer uses to resolve a click on a tab, and
/// it has to stay that way: where the tab is drawn is where the click finds it.
const TAB_H: f32 = 24.0;

/// Width of one tab.
const TAB_W: f32 = 92.0;

/// Width of each of the two scroll buttons (`<` `>`) the strip shows when its
/// tabs do not fit.
const TAB_SCROLL_W: f32 = 16.0;

/// Which tabs fit on the strip, as `(first, count, arrows)`.
///
/// When the tabs do not fit, the `<` `>` buttons take the right end of the
/// strip and the window is the run of tabs that fits and holds the open one
/// (`active` is its position among the shown tabs). The designer resolves
/// clicks with the same rule, so the window you design is the one you run.
fn tab_window(width: f32, shown: usize, active: usize) -> (usize, usize, bool) {
    if shown == 0 {
        return (0, 0, false);
    }
    let mut avail = width;
    if shown as f32 * TAB_W <= avail {
        return (0, shown, false);
    }
    avail -= 2.0 * TAB_SCROLL_W;
    let per = (avail / TAB_W).floor().max(1.0) as usize;
    let count = per.min(shown);
    let active = active.min(shown - 1);
    let mut first = active.saturating_sub(count - 1);
    first = first.min(shown - count);
    (first, count, true)
}

/// Draws a page control: the tab strip and the body below it.
///
/// Returns `true` when the reader picked a different page.
///
/// `name` is what tells two page controls on the same form apart. Everything
/// on a form is drawn into ONE `Ui`, so `ui.id()` is the same for all of them;
/// without the name here, clicking a tab of the second page control would open
/// a page of the first.
///
/// Tabs have a FIXED width. a classic RAD IDE sizes them to their text; here the designer
/// resolves clicks against the model and has no font metrics, so a variable
/// width would mean measuring in two places and needing them to agree forever.
/// Fixed width trades fidelity for a better guarantee: where the tab is drawn
/// is where the click finds it, by construction.
///
/// Each tab is `(caption, hint)`: a non-empty hint shows over the tab.
fn page_control(
    ui: &mut egui::Ui,
    name: &str,
    r: egui::Rect,
    tabs: &[(&str, &str)],
    shown: &[bool],
    bottom: bool,
    active: &mut usize,
) -> bool {
    // Out of range falls back to the FIRST page, like a classic RAD IDE. Without this
    // the guards below never match and the control renders empty — while the
    // designer happily shows page one. This runs before the children are
    // drawn, because the parent is emitted before them.
    if *active >= tabs.len() {
        *active = 0;
    }
    let visuals = ui.visuals().clone();
    let stroke = visuals.widgets.noninteractive.bg_stroke;
    // `TabPosition` bottom: the strip under the pages
    let (body, strip_top) = if bottom {
        (
            egui::Rect::from_min_max(r.min, egui::pos2(r.max.x, r.max.y - TAB_H)),
            r.max.y - TAB_H,
        )
    } else {
        (
            egui::Rect::from_min_max(egui::pos2(r.min.x, r.min.y + TAB_H), r.max),
            r.min.y,
        )
    };
    ui.painter().rect_filled(body, 2.0, visuals.panel_fill);
    ui.painter()
        .rect_stroke(body, 2.0, stroke, egui::StrokeKind::Inside);

    let mut changed = false;
    let shown_indices: Vec<usize> = (0..tabs.len())
        .filter(|&i| shown.get(i).copied().unwrap_or(true))
        .collect();
    let active_pos = shown_indices
        .iter()
        .position(|&idx| idx == *active)
        .unwrap_or(0);
    let (first, count, arrows) = tab_window(r.width(), shown_indices.len(), active_pos);

    for slot in 0..count {
        let i = shown_indices[first + slot];
        let (caption, hint) = tabs[i];
        let tab = egui::Rect::from_min_size(
            egui::pos2(r.min.x + slot as f32 * TAB_W, strip_top),
            egui::vec2(TAB_W, TAB_H),
        );
        let resp = ui.interact(
            tab,
            ui.id().with(("craw_tab", name, i)),
            egui::Sense::click(),
        );
        let clicked = resp.clicked();
        // no hint while a menu is open: it would show over the menu
        if !hint.is_empty() && !egui::Popup::is_any_open(ui.ctx()) {
            resp.on_hover_text(hint);
        }
        let open = *active == i;
        ui.painter().rect_filled(
            tab,
            2.0,
            if open {
                visuals.panel_fill
            } else {
                visuals.faint_bg_color
            },
        );
        ui.painter()
            .rect_stroke(tab, 2.0, stroke, egui::StrokeKind::Inside);
        ui.painter().text(
            tab.center(),
            egui::Align2::CENTER_CENTER,
            caption,
            egui::FontId::proportional(12.0),
            visuals.text_color(),
        );
        if clicked && !open {
            *active = i;
            changed = true;
        }
    }

    if arrows {
        let lx = r.max.x - 2.0 * TAB_SCROLL_W;
        let prev_rect =
            egui::Rect::from_min_size(egui::pos2(lx, strip_top), egui::vec2(TAB_SCROLL_W, TAB_H));
        let next_rect = egui::Rect::from_min_size(
            egui::pos2(lx + TAB_SCROLL_W, strip_top),
            egui::vec2(TAB_SCROLL_W, TAB_H),
        );

        let prev_resp = ui.interact(
            prev_rect,
            ui.id().with(("craw_tab_prev", name)),
            egui::Sense::click(),
        );
        ui.painter()
            .rect_filled(prev_rect, 2.0, visuals.faint_bg_color);
        ui.painter()
            .rect_stroke(prev_rect, 2.0, stroke, egui::StrokeKind::Inside);
        ui.painter().text(
            prev_rect.center(),
            egui::Align2::CENTER_CENTER,
            "<",
            egui::FontId::proportional(12.0),
            visuals.text_color(),
        );

        let next_resp = ui.interact(
            next_rect,
            ui.id().with(("craw_tab_next", name)),
            egui::Sense::click(),
        );
        ui.painter()
            .rect_filled(next_rect, 2.0, visuals.faint_bg_color);
        ui.painter()
            .rect_stroke(next_rect, 2.0, stroke, egui::StrokeKind::Inside);
        ui.painter().text(
            next_rect.center(),
            egui::Align2::CENTER_CENTER,
            ">",
            egui::FontId::proportional(12.0),
            visuals.text_color(),
        );

        if prev_resp.clicked() && active_pos > 0 {
            *active = shown_indices[active_pos - 1];
            changed = true;
        }
        if next_resp.clicked() && active_pos + 1 < shown_indices.len() {
            *active = shown_indices[active_pos + 1];
            changed = true;
        }
    }

    changed
}
// [CRAWRAD:FIM abas]

// [CRAWRAD:INICIO temporizador]

/// Whether a timer fires on this frame, and when it is next due.
///
/// `now` is egui's clock in seconds. `due` is the deadline this timer waits
/// for — `None` on the first frame it is seen, which starts the count instead
/// of firing straight away, the same thing `SetTimer` does on Windows.
///
/// **Late ticks are dropped, never queued.** If the app went three seconds
/// without drawing — busy, minimised, or on a slow machine — and the interval
/// is 500 ms, your handler runs ONCE, not six times.
///
/// That is the classic RAD answer, and a classic RAD IDE gets it from Windows: `WM_TIMER` does
/// not pile up in the queue. There is one message per timer, so a program that
/// stops pumping messages LOSES ticks rather than owing them. The alternative
/// — a catch-up loop — would call your handler six times inside one frame with
/// no time passing between the calls, which is almost never what "every half
/// second" is meant to do. If you need elapsed time, read the clock; do not
/// count ticks.
///
/// The new deadline is measured from NOW, not from the old deadline. That is
/// precisely where the late ticks are dropped: rebasing on the old one would
/// leave the timer overdue and pay the backlog off over the next few frames.
fn timer_due(now: f64, interval_ms: u64, due: &mut Option<f64>) -> bool {
    let interval = interval_ms as f64 / 1000.0;
    match *due {
        None => {
            *due = Some(now + interval);
            false
        }
        Some(deadline) if now >= deadline => {
            *due = Some(now + interval);
            true
        }
        _ => false,
    }
}

/// How long is left until the timer is due, for `request_repaint_after`.
///
/// Never negative: `Duration::from_secs_f64` panics on a negative number, and
/// an overdue timer wants the next frame right away.
fn timer_wait(now: f64, due: Option<f64>) -> std::time::Duration {
    let left = due.map_or(0.0, |d| d - now).max(0.0);
    std::time::Duration::from_secs_f64(left)
}
// [CRAWRAD:FIM temporizador]

// ## DECLARED DRAWING
//
// The two brushstrokes from the ADR 0036 vocabulary that do not fit on a single line. The
// other three — `Caixa`, `Moldura`, and `Texto` — the generator writes directly, because
// `rect_filled`, `rect_stroke`, and `painter().text` are already exactly those.
//
// Each is its own section, rather than a single section containing both: drawing that
// uses dashed lines without check marks would pull the check mark along, and an uncalled
// function is `dead_code` in the project of someone who cannot fix it.
//
// Color enters as `Stroke`, not loose. That is what canvas `Brush` does with the
// same two, and here there is a second reason: egui's color type name does not
// cross this file, because the guard preventing frozen colors searches for
// it. A signature with the type written out would fail by looking like what it is not.
//
// ## And they are `pub`, unlike the other two in this file
//
// `page_control` and `timer_due` are private because `crawrad-runtime-egui`
// CALLS both. These the compiler of this crate only compiles — what calls them is
// the generated application, on the other side of `include_str!`.
//
// Private, they would be `dead_code` in here, and our repository has no `#[allow]`.
// `pub` is the proper fix and not a silencer: this file EXISTS to be
// exported, and what the compiler on this side guarantees is that the crossing
// text COMPILES — which is precisely what `dead_code` would not say.
//
// `fraction` in `shared_geometry.rs` is `pub` for the same reason.

// [CRAWRAD:INICIO declarado_tracejado]

/// Draws a dashed outline around `r`.
///
/// This is what says *there is something here and it has no drawing of its
/// own* — a component that declared a placeholder rather than a look.
pub fn declared_dashed(ui: &egui::Ui, r: egui::Rect, stroke: egui::Stroke) {
    let sides = [
        [r.left_top(), r.right_top()],
        [r.right_top(), r.right_bottom()],
        [r.right_bottom(), r.left_bottom()],
        [r.left_bottom(), r.left_top()],
    ];
    for side in sides {
        ui.painter()
            .extend(egui::Shape::dashed_line(&side, stroke, 4.0, 3.0));
    }
}
// [CRAWRAD:FIM declarado_tracejado]

// [CRAWRAD:INICIO declarado_marca]

/// Draws a check mark inside `r`.
///
/// It is sized from the SHORTER side, so the mark stays a mark in a box of any
/// shape instead of stretching into a scrawl across a wide one.
pub fn declared_check(ui: &egui::Ui, r: egui::Rect, stroke: egui::Stroke) {
    let c = r.center();
    let s = r.width().min(r.height()) * 0.3;
    ui.painter().add(egui::Shape::line(
        vec![
            egui::pos2(c.x - s, c.y),
            egui::pos2(c.x - s * 0.25, c.y + s * 0.7),
            egui::pos2(c.x + s, c.y - s * 0.7),
        ],
        stroke,
    ));
}
// [CRAWRAD:FIM declarado_marca]

// ## THE BEVEL
//
// The designer and the generated application draw the same bevel: the runtime calls it,
// the generator clips it. Shadow and light come from `Visuals`, like the others.

// [CRAWRAD:INICIO bevel]

/// Draws a classic bevel in `r`: `shape` is `Box`, `Frame`, `TopLine`,
/// `BottomLine`, `LeftLine`, `RightLine` or `Spacer` (nothing); a lowered
/// bevel has its shadow on the top and left, a raised one on the bottom and
/// right.
pub fn paint_bevel(
    painter: &egui::Painter,
    r: egui::Rect,
    shape: &str,
    raised: bool,
    visuals: &egui::Visuals,
) {
    let shadow = egui::Stroke::new(1.0, visuals.extreme_bg_color);
    let light = egui::Stroke::new(
        1.0,
        visuals
            .widgets
            .noninteractive
            .fg_stroke
            .color
            .gamma_multiply(0.5),
    );
    let (first, second) = if raised {
        (light, shadow)
    } else {
        (shadow, light)
    };
    let r = r.shrink(0.5);
    let line = |a: (f32, f32), b: (f32, f32), stroke: egui::Stroke| {
        painter.line_segment([egui::pos2(a.0, a.1), egui::pos2(b.0, b.1)], stroke);
    };
    let (left, top, right, bottom) = (r.left(), r.top(), r.right(), r.bottom());
    match shape {
        "Box" => {
            line((left, top), (right, top), first);
            line((left, top), (left, bottom), first);
            line((left, bottom), (right, bottom), second);
            line((right, top), (right, bottom), second);
        }
        "Frame" => {
            // etched: one outline, and the other one pixel in
            for (d, stroke) in [(0.0, first), (1.0, second)] {
                line((left + d, top + d), (right - 1.0 + d, top + d), stroke);
                line((left + d, top + d), (left + d, bottom - 1.0 + d), stroke);
                line(
                    (left + d, bottom - 1.0 + d),
                    (right - 1.0 + d, bottom - 1.0 + d),
                    stroke,
                );
                line(
                    (right - 1.0 + d, top + d),
                    (right - 1.0 + d, bottom - 1.0 + d),
                    stroke,
                );
            }
        }
        "TopLine" => {
            line((left, top), (right, top), first);
            line((left, top + 1.0), (right, top + 1.0), second);
        }
        "BottomLine" => {
            line((left, bottom - 1.0), (right, bottom - 1.0), first);
            line((left, bottom), (right, bottom), second);
        }
        "LeftLine" => {
            line((left, top), (left, bottom), first);
            line((left + 1.0, top), (left + 1.0, bottom), second);
        }
        "RightLine" => {
            line((right - 1.0, top), (right - 1.0, bottom), first);
            line((right, top), (right, bottom), second);
        }
        _ => {}
    }
}
// [CRAWRAD:FIM bevel]

// ## THE CATEGORYPANEL
//
// The header with the chevron and the body outline: the designer and the generated
// application draw the same, and the header height enters as a parameter because
// the designer scales it with zoom.

// [CRAWRAD:INICIO category]

/// Draws a `CategoryPanel` in `r`: a header `header_height` tall with a
/// chevron (right when collapsed, down when open) and the caption, and the
/// body's outline when open. Returns the header, which is what a click folds.
pub fn paint_category(
    painter: &egui::Painter,
    r: egui::Rect,
    caption: &str,
    collapsed: bool,
    font: egui::FontId,
    visuals: &egui::Visuals,
    header_height: f32,
) -> egui::Rect {
    let header =
        egui::Rect::from_min_size(r.min, egui::vec2(r.width(), header_height.min(r.height())));
    let outline = visuals.widgets.noninteractive.bg_stroke;
    if !collapsed {
        painter.rect_stroke(r, 0.0, outline, egui::StrokeKind::Inside);
    }
    painter.rect(
        header,
        0.0,
        visuals.widgets.inactive.weak_bg_fill,
        outline,
        egui::StrokeKind::Inside,
    );
    let s = (header.height() * 0.18).max(2.0);
    let c = header.left_center() + egui::vec2(s * 3.0, 0.0);
    let points = if collapsed {
        vec![
            c + egui::vec2(-s * 0.5, -s),
            c + egui::vec2(s * 0.5, 0.0),
            c + egui::vec2(-s * 0.5, s),
        ]
    } else {
        vec![
            c + egui::vec2(-s, -s * 0.5),
            c + egui::vec2(s, -s * 0.5),
            c + egui::vec2(0.0, s * 0.5),
        ]
    };
    let ink = visuals.text_color();
    painter.add(egui::Shape::convex_polygon(points, ink, egui::Stroke::NONE));
    painter.text(
        header.left_center() + egui::vec2(s * 6.0, 0.0),
        egui::Align2::LEFT_CENTER,
        caption,
        font,
        ink,
    );
    header
}
// [CRAWRAD:FIM category]

// [CRAWRAD:INICIO list_view]

/// Height of a `ListView` column header, in points.
pub const LIST_VIEW_HEADER_H: f32 = 24.0;
/// Height of one `ListView` row, in points.
pub const LIST_VIEW_ROW_H: f32 = 20.0;

/// Paints a column header cell for a `ListView`.
pub fn paint_list_view_header_cell(
    painter: &egui::Painter,
    rect: egui::Rect,
    caption: &str,
    alignment: &str,
    visuals: &egui::Visuals,
    font: egui::FontId,
) {
    let stroke = visuals.widgets.noninteractive.bg_stroke;
    painter.rect(
        rect,
        0.0,
        visuals.widgets.inactive.weak_bg_fill,
        stroke,
        egui::StrokeKind::Inside,
    );
    let pad = 6.0;
    let (pos, anchor) = match alignment {
        "taRightJustify" => (
            rect.right_center() + egui::vec2(-pad, 0.0),
            egui::Align2::RIGHT_CENTER,
        ),
        "taCenter" => (rect.center(), egui::Align2::CENTER_CENTER),
        _ => (
            rect.left_center() + egui::vec2(pad, 0.0),
            egui::Align2::LEFT_CENTER,
        ),
    };
    painter.text(pos, anchor, caption, font, visuals.text_color());
}

/// Paints the designer look of a `ListView`.
pub fn paint_list_view(
    painter: &egui::Painter,
    rect: egui::Rect,
    columns: &[(&str, f32, &str)],
    items: &[(&str, &[&str])],
    selected: Option<usize>,
    row_select: bool,
    grid_lines: bool,
    show_column_headers: bool,
    visuals: &egui::Visuals,
    font: egui::FontId,
) {
    let outline = visuals.widgets.noninteractive.bg_stroke;
    painter.rect(
        rect,
        0.0,
        visuals.extreme_bg_color,
        outline,
        egui::StrokeKind::Inside,
    );
    let mut top = rect.min.y;
    let header_h = LIST_VIEW_HEADER_H;
    let row_h = LIST_VIEW_ROW_H;

    if show_column_headers && !columns.is_empty() {
        let mut x = rect.min.x;
        for (idx, &(caption, width, alignment)) in columns.iter().enumerate() {
            let w = if idx + 1 == columns.len() {
                (rect.max.x - x).max(width)
            } else {
                width
            };
            let cell = egui::Rect::from_min_size(
                egui::pos2(x, top),
                egui::vec2(w.min(rect.max.x - x), header_h),
            );
            if cell.width() > 0.0 {
                paint_list_view_header_cell(
                    painter,
                    cell,
                    caption,
                    alignment,
                    visuals,
                    font.clone(),
                );
            }
            x += width;
            if x >= rect.max.x {
                break;
            }
        }
        top += header_h;
    }

    let pad = 6.0;
    for (row_idx, &(caption, sub_items)) in items.iter().enumerate() {
        if top + row_h > rect.max.y {
            break;
        }
        let is_sel = selected == Some(row_idx);
        let row_rect =
            egui::Rect::from_min_size(egui::pos2(rect.min.x, top), egui::vec2(rect.width(), row_h));
        if is_sel && row_select {
            painter.rect_filled(row_rect, 0.0, visuals.selection.bg_fill);
        }

        let mut x = rect.min.x;
        let num_cols = columns.len().max(1);
        for col_idx in 0..num_cols {
            let col_w = if col_idx < columns.len() {
                columns[col_idx].1
            } else {
                80.0
            };
            let cell_rect = egui::Rect::from_min_size(
                egui::pos2(x, top),
                egui::vec2(col_w.min(rect.max.x - x), row_h),
            );

            if is_sel && !row_select && col_idx == 0 {
                painter.rect_filled(cell_rect, 0.0, visuals.selection.bg_fill);
            }

            let cell_text = if col_idx == 0 {
                caption
            } else if col_idx - 1 < sub_items.len() {
                sub_items[col_idx - 1]
            } else {
                ""
            };

            let align_str = if col_idx < columns.len() {
                columns[col_idx].2
            } else {
                "taLeftJustify"
            };

            let text_color = if is_sel && (row_select || col_idx == 0) {
                visuals.selection.stroke.color
            } else {
                visuals.text_color()
            };

            let (pos, anchor) = match align_str {
                "taRightJustify" => (
                    cell_rect.right_center() + egui::vec2(-pad, 0.0),
                    egui::Align2::RIGHT_CENTER,
                ),
                "taCenter" => (cell_rect.center(), egui::Align2::CENTER_CENTER),
                _ => (
                    cell_rect.left_center() + egui::vec2(pad, 0.0),
                    egui::Align2::LEFT_CENTER,
                ),
            };

            let cell_painter = painter.with_clip_rect(cell_rect.intersect(rect));
            cell_painter.text(pos, anchor, cell_text, font.clone(), text_color);

            if grid_lines {
                painter.line_segment(
                    [
                        egui::pos2(cell_rect.max.x, cell_rect.min.y),
                        egui::pos2(cell_rect.max.x, cell_rect.max.y),
                    ],
                    outline,
                );
            }

            x += col_w;
            if x >= rect.max.x {
                break;
            }
        }

        if grid_lines {
            painter.line_segment(
                [
                    egui::pos2(rect.min.x, row_rect.max.y),
                    egui::pos2(rect.max.x, row_rect.max.y),
                ],
                outline,
            );
        }

        top += row_h;
    }
}
// [CRAWRAD:FIM list_view]

// [CRAWRAD:INICIO message_dialog]

/// Draws a modal message box; returns the clicked button's result name ("ok", "cancel", "yes", "no") once, else None.
pub fn message_box(
    ctx: &egui::Context,
    id: &str,
    title: &str,
    text: &str,
    kind: &str,
    buttons: &str,
) -> Option<&'static str> {
    let resolved_title = if !title.is_empty() {
        title
    } else {
        match kind {
            "mtWarning" | "Warning" => "Warning",
            "mtError" | "Error" => "Error",
            "mtConfirmation" | "Confirmation" => "Confirm",
            _ => "Information",
        }
    };

    let btn_list: &[(&str, &'static str)] = match buttons {
        "mbOKCancel" => &[("OK", "ok"), ("Cancel", "cancel")],
        "mbYesNo" => &[("Yes", "yes"), ("No", "no")],
        "mbYesNoCancel" => &[("Yes", "yes"), ("No", "no"), ("Cancel", "cancel")],
        _ => &[("OK", "ok")],
    };

    let first_res = btn_list[0].1;
    let esc_res = match buttons {
        "mbOKCancel" | "mbYesNoCancel" => "cancel",
        "mbYesNo" => "no",
        _ => "ok",
    };

    let mut chosen: Option<&'static str> = None;
    if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
        chosen = Some(first_res);
    } else if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        chosen = Some(esc_res);
    }

    let modal_res = egui::Modal::new(egui::Id::new(id)).show(ctx, |ui| {
        let visuals = ui.visuals().clone();
        let (icon_glyph, icon_color) = match kind {
            "mtWarning" | "Warning" => ("⚠", visuals.warn_fg_color),
            "mtError" | "Error" => ("✖", visuals.error_fg_color),
            "mtConfirmation" | "Confirmation" => ("?", visuals.warn_fg_color),
            _ => ("ℹ", visuals.hyperlink_color),
        };

        let mut clicked = None;
        ui.set_min_width(320.0);
        ui.set_max_width(480.0);

        ui.heading(resolved_title);
        ui.separator();
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(icon_glyph)
                    .color(icon_color)
                    .size(24.0)
                    .strong(),
            );
            ui.add_space(10.0);
            ui.label(egui::RichText::new(text).size(13.0));
        });

        ui.add_space(14.0);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            for &(label, result_name) in btn_list.iter().rev() {
                if ui.button(label).clicked() {
                    clicked = Some(result_name);
                }
                ui.add_space(6.0);
            }
        });

        clicked
    });

    if chosen.is_none() {
        chosen = modal_res.inner;
    }

    if chosen.is_some() {
        ctx.request_repaint();
    }

    chosen
}
// [CRAWRAD:FIM message_dialog]

// [CRAWRAD:INICIO card]

/// Paints a modern Card with rounded corners, elevation shadow, and optional header.
pub fn paint_card(
    painter: &egui::Painter,
    rect: egui::Rect,
    title: &str,
    subtitle: &str,
    corner_radius: f32,
    elevation: i64,
    show_header: bool,
    visuals: &egui::Visuals,
    font: egui::FontId,
) -> f32 {
    let r = corner_radius.max(0.0);
    if elevation > 0 {
        for i in 1..=elevation.min(3) {
            let offset_y = i as f32 * 1.5;
            let shadow_rect = rect.translate(egui::vec2(0.0, offset_y));
            let alpha = (0.04 * (4 - i) as f32).min(0.2);
            let shadow_color = visuals.window_shadow.color.gamma_multiply(alpha);
            painter.rect_filled(shadow_rect, r, shadow_color);
        }
    }

    let bg_fill = visuals.window_fill;
    let stroke = visuals.window_stroke;
    painter.rect(rect, r, bg_fill, stroke, egui::StrokeKind::Inside);

    if !show_header {
        return 0.0;
    }

    let has_subtitle = !subtitle.is_empty();
    let header_h: f32 = if has_subtitle { 42.0 } else { 28.0 };
    let header_rect = egui::Rect::from_min_size(
        rect.min,
        egui::vec2(rect.width(), header_h.min(rect.height())),
    );

    let title_color = visuals.text_color();
    let pad_x = 12.0;

    if has_subtitle {
        let title_pos = header_rect.left_top() + egui::vec2(pad_x, 6.0);
        let title_font = egui::FontId::new(font.size.max(12.0), font.family.clone());
        painter.text(
            title_pos,
            egui::Align2::LEFT_TOP,
            title,
            title_font.clone(),
            title_color,
        );
        painter.text(
            title_pos + egui::vec2(0.5, 0.0),
            egui::Align2::LEFT_TOP,
            title,
            title_font,
            title_color,
        );

        let sub_pos = header_rect.left_top() + egui::vec2(pad_x, 22.0);
        let sub_font = egui::FontId::new((font.size * 0.85).max(10.0), font.family);
        let sub_color = visuals.weak_text_color();
        painter.text(
            sub_pos,
            egui::Align2::LEFT_TOP,
            subtitle,
            sub_font,
            sub_color,
        );
    } else {
        let title_pos = header_rect.left_center() + egui::vec2(pad_x, 0.0);
        let title_font = egui::FontId::new(font.size.max(12.0), font.family);
        painter.text(
            title_pos,
            egui::Align2::LEFT_CENTER,
            title,
            title_font.clone(),
            title_color,
        );
        painter.text(
            title_pos + egui::vec2(0.5, 0.0),
            egui::Align2::LEFT_CENTER,
            title,
            title_font,
            title_color,
        );
    }

    // Thin divider under the header
    let divider_y = header_rect.bottom();
    let divider_stroke = visuals.widgets.noninteractive.bg_stroke;
    painter.line_segment(
        [
            egui::pos2(rect.min.x, divider_y),
            egui::pos2(rect.max.x, divider_y),
        ],
        divider_stroke,
    );

    header_h
}
// [CRAWRAD:FIM card]

// [CRAWRAD:INICIO badge]

/// Computes the badge text from caption, count, and max_count.
///
/// Returns None when the badge should be hidden (count == 0 and caption is empty).
/// When count > 0, returns the count formatted as a number, or "{max_count}+" if count > max_count.
/// When count == 0 and caption is non-empty, returns the caption.
pub fn badge_text(caption: &str, count: i64, max_count: i64) -> Option<String> {
    if count > 0 {
        if count > max_count {
            Some(format!("{max_count}+"))
        } else {
            Some(count.to_string())
        }
    } else if !caption.is_empty() {
        Some(caption.to_string())
    } else {
        None
    }
}

/// Draws a badge pill with centred text.
///
/// Corner radius is half the height (a pill). When `outline` is false, it is
/// filled with the kind's background and the text uses a contrasting colour.
/// When `outline` is true, the pill background is transparent, bounded by a
/// 1-point stroke in the kind's tone, and the text uses that same tone.
pub fn paint_badge(
    painter: &egui::Painter,
    rect: egui::Rect,
    text: &str,
    kind: &str,
    outline: bool,
    visuals: &egui::Visuals,
    font: egui::FontId,
) {
    let r = (rect.height() * 0.5).max(0.0);
    let (bg_fill, fg_color) = match kind {
        "bkDefault" => (visuals.widgets.inactive.bg_fill, visuals.text_color()),
        "bkPrimary" => (visuals.selection.bg_fill, visuals.selection.stroke.color),
        "bkInfo" => (visuals.hyperlink_color, visuals.extreme_bg_color),
        "bkWarning" => (visuals.warn_fg_color, visuals.extreme_bg_color),
        "bkDanger" => (visuals.error_fg_color, visuals.extreme_bg_color),
        _ => (visuals.selection.bg_fill, visuals.selection.stroke.color),
    };

    if outline {
        let stroke_color = match kind {
            "bkDefault" => visuals.text_color(),
            "bkPrimary" => visuals.selection.stroke.color,
            "bkInfo" => visuals.hyperlink_color,
            "bkWarning" => visuals.warn_fg_color,
            "bkDanger" => visuals.error_fg_color,
            _ => visuals.selection.stroke.color,
        };
        painter.rect_stroke(
            rect,
            r,
            egui::Stroke::new(1.0, stroke_color),
            egui::StrokeKind::Inside,
        );
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            font,
            stroke_color,
        );
    } else {
        painter.rect_filled(rect, r, bg_fill);
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            font,
            fg_color,
        );
    }
}
// [CRAWRAD:FIM badge]

// [CRAWRAD:INICIO chips]
/// Layout constants for chips.
pub const CHIP_HEIGHT: f32 = 24.0;
pub const CHIP_GAP: f32 = 6.0;
pub const CHIP_PAD_X: f32 = 8.0;
pub const CHIP_REMOVE_W: f32 = 14.0;
pub const CHIP_CHAR_W: f32 = 7.0;

/// Pure function computing top-left `(x, y)` relative coordinates for each chip.
///
/// Chips are laid out left to right, wrapping to the next line when a chip
/// would exceed `area_width`.
pub fn chips_layout(
    widths: &[f32],
    area_width: f32,
    chip_height: f32,
    gap: f32,
) -> Vec<(f32, f32)> {
    let mut positions = Vec::with_capacity(widths.len());
    let mut x = 0.0;
    let mut y = 0.0;
    for &w in widths {
        if x > 0.0 && x + w > area_width {
            x = 0.0;
            y += chip_height + gap;
        }
        positions.push((x, y));
        x += w + gap;
    }
    positions
}

/// Approximate chip width based on text length and optional remove button.
pub fn chip_width(text: &str, removable: bool) -> f32 {
    let text_w = text.chars().count() as f32 * CHIP_CHAR_W;
    let extra = if removable { CHIP_REMOVE_W } else { 0.0 };
    text_w + CHIP_PAD_X * 2.0 + extra
}

/// Draws an individual chip pill.
///
/// Every colour comes from `visuals`, never a literal:
/// - Unselected: `visuals.widgets.inactive.bg_fill` + `visuals.text_color()`
/// - Selected: `visuals.selection.bg_fill` + `visuals.selection.stroke.color`
/// - "×" remove icon in `visuals.weak_text_color()`
pub fn paint_chip(
    painter: &egui::Painter,
    rect: egui::Rect,
    text: &str,
    selected: bool,
    removable: bool,
    visuals: &egui::Visuals,
    font: egui::FontId,
) {
    let r = (rect.height() * 0.5).max(0.0);
    let (bg_fill, fg_color) = if selected {
        (visuals.selection.bg_fill, visuals.selection.stroke.color)
    } else {
        (visuals.widgets.inactive.bg_fill, visuals.text_color())
    };

    painter.rect_filled(rect, r, bg_fill);

    if removable {
        let text_center = egui::pos2(
            rect.min.x + (rect.width() - CHIP_REMOVE_W) * 0.5,
            rect.center().y,
        );
        painter.text(
            text_center,
            egui::Align2::CENTER_CENTER,
            text,
            font.clone(),
            fg_color,
        );
        let cross_center = egui::pos2(rect.max.x - CHIP_PAD_X, rect.center().y);
        painter.text(
            cross_center,
            egui::Align2::CENTER_CENTER,
            "×",
            font,
            visuals.weak_text_color(),
        );
    } else {
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            font,
            fg_color,
        );
    }
}
// [CRAWRAD:FIM chips]

// [CRAWRAD:INICIO breadcrumb]
/// Layout constants for breadcrumb.
pub const BREADCRUMB_GAP: f32 = 6.0;

/// Layout constants for chevron breadcrumb.
pub const CHEVRON_TIP: f32 = 12.0;
pub const CHEVRON_OVERLAP: f32 = 8.0;
pub const CHEVRON_PADS: (f32, f32, f32) = (14.0, 24.0, 20.0);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChevronPads {
    pub first_left: f32,
    pub notched_left: f32,
    pub right: f32,
}

impl Default for ChevronPads {
    fn default() -> Self {
        Self {
            first_left: CHEVRON_PADS.0,
            notched_left: CHEVRON_PADS.1,
            right: CHEVRON_PADS.2,
        }
    }
}

/// Pure function computing `(x, width)` for each chevron segment.
pub fn chevron_layout(
    widths: &[f32],
    tip: f32,
    overlap: f32,
    pads: ChevronPads,
) -> Vec<(f32, f32)> {
    let mut out = Vec::with_capacity(widths.len());
    let mut x = 0.0;
    for (i, &w) in widths.iter().enumerate() {
        let left_pad = if i == 0 {
            pads.first_left
        } else {
            pads.notched_left
        };
        let seg_w = left_pad + w + pads.right + tip;
        out.push((x, seg_w));
        x += seg_w - overlap;
    }
    out
}

/// Pure function computing horizontal `x` offsets for each breadcrumb segment.
///
/// Between segment `i` and segment `i + 1`, a separator of width `separator_width`
/// is placed with `gap` spacing on both sides.
pub fn breadcrumb_layout(widths: &[f32], separator_width: f32, gap: f32) -> Vec<f32> {
    let mut positions = Vec::with_capacity(widths.len());
    let mut x = 0.0;
    for &w in widths {
        positions.push(x);
        x += w + separator_width + 2.0 * gap;
    }
    positions
}

/// The real width of `text` in `font`: measured by the painter's fonts, so a
/// proportional font ("m" wide, "i" narrow) leaves even gaps.
pub fn breadcrumb_text_width(painter: &egui::Painter, text: &str, font: &egui::FontId) -> f32 {
    let ctx = painter.ctx();
    let colour = ctx.style_of(ctx.theme()).visuals.text_color();
    painter
        .layout_no_wrap(text.to_owned(), font.clone(), colour)
        .size()
        .x
}

/// Draws breadcrumb navigation trail.
///
/// Every colour comes from `visuals`, never a literal:
/// - Links: `visuals.hyperlink_color` (underlined when hovered)
/// - Current segment (last): `visuals.strong_text_color()` drawn bold
/// - Separator: `visuals.weak_text_color()`
pub fn paint_breadcrumb(
    painter: &egui::Painter,
    rect: egui::Rect,
    items: &[&str],
    separator: &str,
    hovered: Option<usize>,
    visuals: &egui::Visuals,
    font: egui::FontId,
) {
    if items.is_empty() {
        return;
    }
    let widths: Vec<f32> = items
        .iter()
        .map(|it| breadcrumb_text_width(painter, it, &font))
        .collect();
    let sep_w = breadcrumb_text_width(painter, separator, &font);
    let positions = breadcrumb_layout(&widths, sep_w, BREADCRUMB_GAP);

    let count = items.len();
    let y_center = rect.center().y;
    let clip = rect.intersect(painter.clip_rect());
    let painter = painter.with_clip_rect(clip);

    for (i, (&x, text)) in positions.iter().zip(items.iter()).enumerate() {
        let abs_x = rect.min.x + x;
        if abs_x >= rect.max.x {
            break;
        }
        let is_last = i + 1 == count;
        let w = widths[i];
        let pos = egui::pos2(abs_x, y_center);

        if is_last {
            let color = visuals.strong_text_color();
            painter.text(pos, egui::Align2::LEFT_CENTER, *text, font.clone(), color);
            painter.text(
                pos + egui::vec2(0.5, 0.0),
                egui::Align2::LEFT_CENTER,
                *text,
                font.clone(),
                color,
            );
        } else {
            let color = visuals.hyperlink_color;
            painter.text(pos, egui::Align2::LEFT_CENTER, *text, font.clone(), color);
            if hovered == Some(i) {
                let line_y = (y_center + font.size * 0.5 + 1.0).min(rect.max.y - 1.0);
                painter.line_segment(
                    [egui::pos2(abs_x, line_y), egui::pos2(abs_x + w, line_y)],
                    egui::Stroke::new(1.0, color),
                );
            }

            let sep_x = abs_x + w + BREADCRUMB_GAP;
            if sep_x < rect.max.x {
                let sep_pos = egui::pos2(sep_x, y_center);
                painter.text(
                    sep_pos,
                    egui::Align2::LEFT_CENTER,
                    separator,
                    font.clone(),
                    visuals.weak_text_color(),
                );
            }
        }
    }
}

/// Draws powerline chevron breadcrumb navigation trail.
pub fn paint_breadcrumb_chevron(
    painter: &egui::Painter,
    rect: egui::Rect,
    items: &[&str],
    current_idx: usize,
    hovered: Option<usize>,
    segment_color: egui::Rgba,
    current_color: egui::Rgba,
    font: egui::FontId,
) {
    if items.is_empty() {
        return;
    }
    let widths: Vec<f32> = items
        .iter()
        .map(|it| breadcrumb_text_width(painter, it, &font))
        .collect();
    let pads = ChevronPads::default();
    let positions = chevron_layout(&widths, CHEVRON_TIP, CHEVRON_OVERLAP, pads);

    let tip = CHEVRON_TIP;
    let y = rect.min.y;
    let h = rect.height();
    let half_h = h * 0.5;

    let clip = rect.intersect(painter.clip_rect());
    let painter = painter.with_clip_rect(clip);

    for (i, (&(rel_x, w), &item)) in positions.iter().zip(items.iter()).enumerate() {
        let x = rect.min.x + rel_x;
        if x >= rect.max.x {
            break;
        }
        let is_current = i == current_idx;
        let mut fill = if is_current {
            current_color
        } else {
            segment_color
        };
        if hovered == Some(i) && !is_current {
            // the block's own colour, a step lighter: no colour of its own
            fill = fill + egui::Rgba::from_rgba_premultiplied(0.06, 0.06, 0.06, 0.0);
        }

        if i == 0 {
            let pts = vec![
                egui::pos2(x, y),
                egui::pos2(x + w - tip, y),
                egui::pos2(x + w, y + half_h),
                egui::pos2(x + w - tip, y + h),
                egui::pos2(x, y + h),
            ];
            painter.add(egui::Shape::convex_polygon(pts, fill, egui::Stroke::NONE));
        } else {
            let top_pts = vec![
                egui::pos2(x, y),
                egui::pos2(x + w - tip, y),
                egui::pos2(x + w, y + half_h),
                egui::pos2(x + tip, y + half_h),
            ];
            let bot_pts = vec![
                egui::pos2(x + tip, y + half_h),
                egui::pos2(x + w, y + half_h),
                egui::pos2(x + w - tip, y + h),
                egui::pos2(x, y + h),
            ];
            painter.add(egui::Shape::convex_polygon(
                top_pts,
                fill,
                egui::Stroke::NONE,
            ));
            painter.add(egui::Shape::convex_polygon(
                bot_pts,
                fill,
                egui::Stroke::NONE,
            ));
        }

        let text_x = if i == 0 {
            x + pads.first_left
        } else {
            x + pads.notched_left
        };
        let text_pos = egui::pos2(text_x, y + half_h);
        let ctx = painter.ctx();
        let text_color = ctx.style_of(ctx.theme()).visuals.strong_text_color();
        painter.text(
            text_pos,
            egui::Align2::LEFT_CENTER,
            item,
            font.clone(),
            text_color,
        );
        if is_current {
            painter.text(
                text_pos + egui::vec2(0.5, 0.0),
                egui::Align2::LEFT_CENTER,
                item,
                font.clone(),
                text_color,
            );
        }
    }
}
// [CRAWRAD:FIM breadcrumb]

// [CRAWRAD:INICIO pagination]
/// A slot in a pagination control.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageSlot {
    Prev,
    Page(usize),
    Gap,
    Next,
}

/// Constants for pagination layout.
pub const PAGINATION_CELL: f32 = 28.0;
pub const PAGINATION_GAP: f32 = 4.0;

/// Pure function computing which page buttons and gap slots are shown.
///
/// - `page_count <= max_buttons`: every page.
/// - Otherwise first and last pages are always shown, current page with neighbours in middle,
///   and `Gap` where pages are skipped; exactly `max(max_buttons, 5)` page+gap slots.
/// - `arrows`: `Prev` first and `Next` last.
/// - `page_index` past the end is clamped to `page_count - 1`.
pub fn pagination_slots(
    page_count: usize,
    page_index: usize,
    max_buttons: usize,
    arrows: bool,
) -> Vec<PageSlot> {
    if page_count == 0 {
        return Vec::new();
    }
    let page_index = page_index.min(page_count - 1);
    let max_b = max_buttons.max(5);

    if page_count <= max_b {
        let mut slots = Vec::with_capacity(page_count + if arrows { 2 } else { 0 });
        if arrows {
            slots.push(PageSlot::Prev);
        }
        for i in 0..page_count {
            slots.push(PageSlot::Page(i));
        }
        if arrows {
            slots.push(PageSlot::Next);
        }
        return slots;
    }

    let mid_count = max_b - 4;
    let half_left = mid_count / 2;
    let half_right = mid_count - half_left - 1;

    let mut page_slots = Vec::with_capacity(max_b);
    if page_index <= half_left + 2 {
        for i in 0..(max_b - 2) {
            page_slots.push(PageSlot::Page(i));
        }
        page_slots.push(PageSlot::Gap);
        page_slots.push(PageSlot::Page(page_count - 1));
    } else if page_index + half_right >= page_count - 3 {
        page_slots.push(PageSlot::Page(0));
        page_slots.push(PageSlot::Gap);
        for i in (page_count - (max_b - 2))..page_count {
            page_slots.push(PageSlot::Page(i));
        }
    } else {
        page_slots.push(PageSlot::Page(0));
        page_slots.push(PageSlot::Gap);
        let start_mid = page_index - half_left;
        let end_mid = page_index + half_right;
        for i in start_mid..=end_mid {
            page_slots.push(PageSlot::Page(i));
        }
        page_slots.push(PageSlot::Gap);
        page_slots.push(PageSlot::Page(page_count - 1));
    }

    let mut slots = Vec::with_capacity(page_slots.len() + if arrows { 2 } else { 0 });
    if arrows {
        slots.push(PageSlot::Prev);
    }
    slots.extend(page_slots);
    if arrows {
        slots.push(PageSlot::Next);
    }
    slots
}

/// The measured width of `text` in `font`.
pub fn pagination_text_width(painter: &egui::Painter, text: &str, font: &egui::FontId) -> f32 {
    let ctx = painter.ctx();
    let colour = ctx.style_of(ctx.theme()).visuals.text_color();
    painter
        .layout_no_wrap(text.to_owned(), font.clone(), colour)
        .size()
        .x
}

/// Draws pagination page buttons.
///
/// Colours come strictly from `visuals`:
/// - Current page: `visuals.selection.bg_fill` with text `visuals.selection.stroke.color` drawn bold
/// - Outlined cell: `visuals.widgets.noninteractive.bg_stroke`, hover `visuals.widgets.hovered.bg_stroke`, text `visuals.text_color()`
/// - Disabled slots (first/last arrows): drawn at half opacity
/// - Gap: drawn "…", not clickable
pub fn paint_pagination(
    painter: &egui::Painter,
    rect: egui::Rect,
    slots: &[PageSlot],
    page_index: usize,
    hovered: Option<usize>,
    visuals: &egui::Visuals,
    font: egui::FontId,
) {
    if slots.is_empty() {
        return;
    }
    let clip = rect.intersect(painter.clip_rect());
    let painter = painter.with_clip_rect(clip);

    let h = rect.height();
    let mut x = rect.min.x;

    let max_page = slots
        .iter()
        .filter_map(|s| match s {
            PageSlot::Page(p) => Some(*p),
            _ => None,
        })
        .max();

    for (slot_idx, slot) in slots.iter().enumerate() {
        let label = match slot {
            PageSlot::Prev => "‹".to_string(),
            PageSlot::Next => "›".to_string(),
            PageSlot::Gap => "…".to_string(),
            PageSlot::Page(p) => (p + 1).to_string(),
        };

        let text_w = pagination_text_width(&painter, &label, &font);
        let cell_w = h.max(text_w + 8.0);
        let cell_rect = egui::Rect::from_min_size(egui::pos2(x, rect.min.y), egui::vec2(cell_w, h));

        if cell_rect.min.x >= rect.max.x {
            break;
        }

        match slot {
            PageSlot::Page(p) if *p == page_index => {
                painter.rect_filled(cell_rect, 2.0, visuals.selection.bg_fill);
                let text_color = visuals.selection.stroke.color;
                let text_pos = cell_rect.center();
                painter.text(
                    text_pos,
                    egui::Align2::CENTER_CENTER,
                    &label,
                    font.clone(),
                    text_color,
                );
                painter.text(
                    text_pos + egui::vec2(0.5, 0.0),
                    egui::Align2::CENTER_CENTER,
                    &label,
                    font.clone(),
                    text_color,
                );
            }
            PageSlot::Gap => {
                let text_pos = cell_rect.center();
                painter.text(
                    text_pos,
                    egui::Align2::CENTER_CENTER,
                    &label,
                    font.clone(),
                    visuals.text_color(),
                );
            }
            PageSlot::Prev | PageSlot::Next | PageSlot::Page(_) => {
                let is_disabled = match slot {
                    PageSlot::Prev => page_index == 0,
                    PageSlot::Next => max_page.is_some_and(|mp| page_index >= mp),
                    _ => false,
                };
                let is_hovered = hovered == Some(slot_idx);
                let outline_stroke = if is_hovered {
                    visuals.widgets.hovered.bg_stroke
                } else {
                    // the inactive stroke is zero wide in the stock themes: the
                    // outline is the separator line's
                    visuals.widgets.noninteractive.bg_stroke
                };

                if is_disabled {
                    let mut p = painter.clone();
                    p.multiply_opacity(0.5);
                    p.rect_stroke(
                        cell_rect,
                        2.0,
                        visuals.widgets.noninteractive.bg_stroke,
                        egui::StrokeKind::Inside,
                    );
                    p.text(
                        cell_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        &label,
                        font.clone(),
                        visuals.text_color(),
                    );
                } else {
                    painter.rect_stroke(cell_rect, 2.0, outline_stroke, egui::StrokeKind::Inside);
                    painter.text(
                        cell_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        &label,
                        font.clone(),
                        visuals.text_color(),
                    );
                }
            }
        }

        x += cell_w + PAGINATION_GAP;
    }
}
// [CRAWRAD:FIM pagination]

// [CRAWRAD:INICIO drawer]
/// Layout constants for drawer.
pub const DRAWER_HEADER_HEIGHT: f32 = 32.0;

/// Pure function computing the close button rectangle for a drawer.
///
/// When side is "dsRight", the close button is at the left of the header.
/// Otherwise ("dsLeft"), the close button is at the right of the header.
pub fn drawer_close_rect(rect: egui::Rect, side: &str) -> egui::Rect {
    let size = DRAWER_HEADER_HEIGHT;
    if side == "dsRight" {
        egui::Rect::from_min_size(rect.min, egui::vec2(size, size))
    } else {
        egui::Rect::from_min_size(
            egui::pos2(rect.max.x - size, rect.min.y),
            egui::vec2(size, size),
        )
    }
}

/// Helper to draw a dashed rectangle outline.
pub fn draw_dashed_rect(
    painter: &egui::Painter,
    rect: egui::Rect,
    stroke: egui::Stroke,
    dash_len: f32,
    gap_len: f32,
) {
    let draw_dashed_line = |p1: egui::Pos2, p2: egui::Pos2| {
        let dir = p2 - p1;
        let len = dir.length();
        if len <= 0.0 {
            return;
        }
        let unit = dir / len;
        let mut dist = 0.0;
        while dist < len {
            let next = (dist + dash_len).min(len);
            painter.line_segment([p1 + unit * dist, p1 + unit * next], stroke);
            dist += dash_len + gap_len;
        }
    };

    let lt = rect.left_top();
    let rt = rect.right_top();
    let rb = rect.right_bottom();
    let lb = rect.left_bottom();

    draw_dashed_line(lt, rt);
    draw_dashed_line(rt, rb);
    draw_dashed_line(rb, lb);
    draw_dashed_line(lb, lt);
}

/// Paints a drawer side panel.
///
/// Every colour comes from `visuals`:
/// - Background: `visuals.window_fill`
/// - Border: `visuals.window_stroke`
/// - Shadow: `visuals.window_shadow.color` on edge opposite to `side`
/// - Header text: `visuals.text_color()`
/// - Close button: `visuals.widgets.hovered` / `visuals.weak_text_color()`
pub fn paint_drawer(
    painter: &egui::Painter,
    rect: egui::Rect,
    caption: &str,
    side: &str,
    show_header: bool,
    close_hovered: bool,
    visuals: &egui::Visuals,
    font: egui::FontId,
) {
    // Shadow opposite to side (dsLeft -> shadow on right edge; dsRight -> shadow on left edge)
    for i in 1..=3 {
        let offset = i as f32 * 2.0;
        let shadow_rect = if side == "dsRight" {
            egui::Rect::from_min_max(
                egui::pos2(rect.min.x - offset, rect.min.y),
                egui::pos2(rect.min.x, rect.max.y),
            )
        } else {
            egui::Rect::from_min_max(
                egui::pos2(rect.max.x, rect.min.y),
                egui::pos2(rect.max.x + offset, rect.max.y),
            )
        };
        let alpha = (0.05 * (4 - i) as f32).min(0.2);
        let shadow_color = visuals.window_shadow.color.gamma_multiply(alpha);
        painter.rect_filled(shadow_rect, 0.0, shadow_color);
    }

    // Panel background & border
    let bg_fill = visuals.window_fill;
    let stroke = visuals.window_stroke;
    painter.rect(rect, 0.0, bg_fill, stroke, egui::StrokeKind::Inside);

    if !show_header {
        return;
    }

    let header_rect = egui::Rect::from_min_size(
        rect.min,
        egui::vec2(rect.width(), DRAWER_HEADER_HEIGHT.min(rect.height())),
    );

    let title_color = visuals.text_color();
    let title_font = egui::FontId::new(font.size.max(13.0), font.family.clone());

    let close_btn_rect = drawer_close_rect(header_rect, side);
    let title_pos = if side == "dsRight" {
        egui::pos2(close_btn_rect.max.x + 8.0, header_rect.center().y)
    } else {
        egui::pos2(header_rect.min.x + 12.0, header_rect.center().y)
    };

    painter.text(
        title_pos,
        egui::Align2::LEFT_CENTER,
        caption,
        title_font,
        title_color,
    );

    let close_color = if close_hovered {
        visuals.text_color()
    } else {
        visuals.weak_text_color()
    };
    if close_hovered {
        painter.rect_filled(
            close_btn_rect.shrink(4.0),
            2.0,
            visuals.widgets.hovered.bg_fill,
        );
    }
    let close_font = egui::FontId::new(14.0, font.family);
    painter.text(
        close_btn_rect.center(),
        egui::Align2::CENTER_CENTER,
        "×",
        close_font,
        close_color,
    );

    let divider_y = header_rect.bottom();
    let divider_stroke = visuals.widgets.noninteractive.bg_stroke;
    painter.line_segment(
        [
            egui::pos2(rect.min.x, divider_y),
            egui::pos2(rect.max.x, divider_y),
        ],
        divider_stroke,
    );
}
// [CRAWRAD:FIM drawer]

// [CRAWRAD:INICIO multi_select]
/// Returns the summary text for a `MultiSelect` control.
///
/// Filters `items` to those whose text appears in `selected`, preserving `items` order.
/// If no items are selected, returns `hint`.
/// If `fits(&joined)` is true, returns the joined string (`"A, B, C"`).
/// Otherwise returns `"{n} selected"`.
pub fn multi_select_summary(
    items: &[String],
    selected: &[String],
    hint: &str,
    mut fits: impl FnMut(&str) -> bool,
) -> String {
    let checked: Vec<&str> = items
        .iter()
        .filter(|it| selected.iter().any(|s| s == *it))
        .map(|s| s.as_str())
        .collect();

    if checked.is_empty() {
        return hint.to_string();
    }

    let joined = checked.join(", ");
    if fits(&joined) {
        joined
    } else {
        format!("{} selected", checked.len())
    }
}

/// Toggles whether `items[index]` is present in `selected`, keeping `selected`
/// sorted in `items` order.
pub fn toggle_selected(items: &[String], selected: &mut Vec<String>, index: usize) {
    if index >= items.len() {
        return;
    }
    let target = &items[index];
    if let Some(pos) = selected.iter().position(|s| s == target) {
        selected.remove(pos);
    } else {
        selected.push(target.clone());
        selected.sort_by_key(|s| items.iter().position(|it| it == s).unwrap_or(usize::MAX));
    }
}

/// Paints the closed field of a `MultiSelect` control.
///
/// Draws a field matching the ComboBox:
/// - Background: `visuals.widgets.inactive.bg_fill`
/// - Outline: `visuals.widgets.noninteractive.bg_stroke`
/// - Text: `visuals.text_color()`
/// - Chevron `▾`: `visuals.text_color()`
pub fn paint_multi_select(
    painter: &egui::Painter,
    rect: egui::Rect,
    summary: &str,
    visuals: &egui::Visuals,
    font: egui::FontId,
) {
    let clip = rect.intersect(painter.clip_rect());
    let painter = painter.with_clip_rect(clip);

    let bg_fill = visuals.widgets.inactive.bg_fill;
    let stroke = visuals.widgets.noninteractive.bg_stroke;
    painter.rect(rect, 2.0, bg_fill, stroke, egui::StrokeKind::Inside);

    let chevron_w = 20.0;
    let text_rect = egui::Rect::from_min_max(
        egui::pos2(rect.min.x + 8.0, rect.min.y),
        egui::pos2(rect.max.x - chevron_w, rect.max.y),
    );

    let text_color = visuals.text_color();
    let text_painter = painter.with_clip_rect(text_rect.intersect(clip));
    text_painter.text(
        text_rect.left_center(),
        egui::Align2::LEFT_CENTER,
        summary,
        font.clone(),
        text_color,
    );

    let chevron_pos = egui::pos2(rect.max.x - 10.0, rect.center().y);
    painter.text(
        chevron_pos,
        egui::Align2::CENTER_CENTER,
        "▾",
        font,
        text_color,
    );
}
// [CRAWRAD:FIM multi_select]

// [CRAWRAD:INICIO check_list_box]
/// Pure function to update `checked` when item `index` is toggled `on` or off.
///
/// Adds or removes `items[index]` without duplicates; index past the end does nothing.
pub fn set_checked_item(items: &[String], checked: &mut Vec<String>, index: usize, on: bool) {
    if index >= items.len() {
        return;
    }
    let target = &items[index];
    if on {
        if !checked.iter().any(|c| c == target) {
            checked.push(target.clone());
        }
    } else {
        checked.retain(|c| c != target);
    }
}

/// Paints a `CheckListBox` control.
///
/// Draws a container box and vertical list of rows, each with a check box.
pub fn paint_check_list_box(
    painter: &egui::Painter,
    rect: egui::Rect,
    items: &[String],
    checked: &[String],
    visuals: &egui::Visuals,
    font: egui::FontId,
) {
    let clip = rect.intersect(painter.clip_rect());
    let painter = painter.with_clip_rect(clip);

    let bg_fill = visuals.extreme_bg_color;
    let stroke = visuals.widgets.inactive.bg_stroke;
    painter.rect(rect, 2.0, bg_fill, stroke, egui::StrokeKind::Inside);

    let row_h = 22.0;
    let box_size = 14.0;
    let pad_x = 6.0;
    let gap_x = 6.0;

    let mut y = rect.min.y + 2.0;
    for it in items {
        if y + row_h > rect.max.y {
            break;
        }

        let box_y = y + (row_h - box_size) * 0.5;
        let check_rect = egui::Rect::from_min_size(
            egui::pos2(rect.min.x + pad_x, box_y),
            egui::vec2(box_size, box_size),
        );

        painter.rect(
            check_rect,
            2.0,
            visuals.widgets.inactive.bg_fill,
            visuals.widgets.inactive.bg_stroke,
            egui::StrokeKind::Inside,
        );

        let is_checked = checked.iter().any(|c| c == it);
        if is_checked {
            painter.text(
                check_rect.center(),
                egui::Align2::CENTER_CENTER,
                "✔",
                font.clone(),
                visuals.text_color(),
            );
        }

        let text_w = (rect.max.x - check_rect.max.x - gap_x - pad_x).max(0.0);
        let text_rect = egui::Rect::from_min_size(
            egui::pos2(check_rect.max.x + gap_x, y),
            egui::vec2(text_w, row_h),
        );
        let text_painter = painter.with_clip_rect(text_rect.intersect(clip));
        text_painter.text(
            text_rect.left_center(),
            egui::Align2::LEFT_CENTER,
            it,
            font.clone(),
            visuals.text_color(),
        );

        y += row_h;
    }
}
// [CRAWRAD:FIM check_list_box]

// [CRAWRAD:INICIO progress]

/// The gap between two blocks of a progress bar that is not `Smooth`.
pub const PROGRESS_BLOCK_GAP: f32 = 2.0;

/// The blocks of a progress bar that is not `Smooth`: the filled part of
/// `rect`, cut in the classic chunks along the bar's length. A horizontal bar
/// fills from the left, a vertical one from the bottom. Each block is two
/// thirds of the bar's thickness long, as the classic bar draws them.
pub fn progress_blocks(rect: egui::Rect, fraction: f32, vertical: bool) -> Vec<egui::Rect> {
    let (len, thick) = if vertical {
        (rect.height(), rect.width())
    } else {
        (rect.width(), rect.height())
    };
    let filled = len * fraction.clamp(0.0, 1.0);
    let block = (thick * 2.0 / 3.0).max(4.0);
    let mut blocks = Vec::new();
    let mut at = 0.0;
    while at < filled {
        let end = (at + block).min(len);
        blocks.push(if vertical {
            egui::Rect::from_min_max(
                egui::pos2(rect.min.x, rect.max.y - end),
                egui::pos2(rect.max.x, rect.max.y - at),
            )
        } else {
            egui::Rect::from_min_max(
                egui::pos2(rect.min.x + at, rect.min.y),
                egui::pos2(rect.min.x + end, rect.max.y),
            )
        });
        at = end + PROGRESS_BLOCK_GAP;
    }
    blocks
}

/// A progress bar the `egui::ProgressBar` cannot draw: a vertical one, or one
/// that is not `Smooth` (drawn in blocks).
pub fn paint_progress(
    ui: &egui::Ui,
    rect: egui::Rect,
    fraction: f32,
    vertical: bool,
    smooth: bool,
) {
    let visuals = ui.visuals();
    let painter = ui.painter();
    painter.rect_filled(rect, 0.0, visuals.extreme_bg_color);
    let fill = visuals.selection.bg_fill;
    if smooth {
        let f = fraction.clamp(0.0, 1.0);
        let full = if vertical {
            egui::Rect::from_min_max(
                egui::pos2(rect.min.x, rect.max.y - rect.height() * f),
                rect.max,
            )
        } else {
            egui::Rect::from_min_max(
                rect.min,
                egui::pos2(rect.min.x + rect.width() * f, rect.max.y),
            )
        };
        painter.rect_filled(full, 0.0, fill);
    } else {
        for block in progress_blocks(rect, fraction, vertical) {
            painter.rect_filled(block, 0.0, fill);
        }
    }
}

// [CRAWRAD:FIM progress]

// [CRAWRAD:INICIO char_case]
/// The edit's text held to its `CharCase`: `ecUpperCase` and
/// `ecLowerCase` turn what was typed, `ecNormal` leaves it.
pub fn char_cased(text: &str, char_case: &str) -> String {
    match char_case {
        "ecUpperCase" => text.to_uppercase(),
        "ecLowerCase" => text.to_lowercase(),
        _ => text.to_string(),
    }
}
// [CRAWRAD:FIM char_case]

// [CRAWRAD:INICIO mask_edit]
/// Where the cursor of the text box `id` stands, as a character index (the
/// end of `text` when it has none) — a `MaskEdit` reshapes its text on every
/// change and puts the cursor back on the next editable position.
pub fn text_cursor(ctx: &egui::Context, id: egui::Id, text: &str) -> usize {
    egui::TextEdit::load_state(ctx, id)
        .and_then(|state| state.cursor.char_range())
        .map_or(text.chars().count(), |range| range.primary.index.0)
}

/// Puts the cursor of the text box `id` on the character index `at`.
pub fn set_text_cursor(ctx: &egui::Context, id: egui::Id, at: usize) {
    let mut state = egui::TextEdit::load_state(ctx, id).unwrap_or_default();
    let cursor = egui::text::CCursor::new(at);
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::one(cursor)));
    state.store(ctx, id);
}

/// A text box's edited text held by `shape` (its `EditMask`):
/// `shape` takes the text and the cursor (a character index) and answers the
/// new text and the cursor's new place.
pub fn hold_text(
    ctx: &egui::Context,
    id: egui::Id,
    text: &mut String,
    shape: impl FnOnce(&str, usize) -> (String, usize),
) {
    let (held, at) = shape(text, text_cursor(ctx, id, text));
    if held != *text {
        *text = held;
    }
    set_text_cursor(ctx, id, at);
}
// [CRAWRAD:FIM mask_edit]

// [CRAWRAD:INICIO scroll_bar]
/// Side of the two end buttons of a scroll bar, in screen points.
///
/// Square: a bar's button is as wide as the bar is thick, up to this ceiling,
/// so a thick bar does not become two buttons and no track.
pub const SCROLL_BAR_BUTTON: f32 = 16.0;

/// How much of the track the thumb takes up when it fits.
pub const SCROLL_BAR_THUMB_SHARE: f32 = 0.3;

/// The smallest thumb one can still grab, when the track allows it.
pub const SCROLL_BAR_MIN_THUMB: f32 = 8.0;

/// The gap between the thumb and the bar wall, on the short axis.
pub const SCROLL_BAR_INSET: f32 = 2.0;

/// The parts of a scroll bar, in screen points.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollBarParts {
    /// The starting button: on the left when horizontal, on top when vertical.
    pub start: egui::Rect,
    /// The ending button.
    pub end: egui::Rect,
    /// The thumb, already placed at the bar's position.
    pub thumb: egui::Rect,
    /// Is the bar wider than it is tall?
    pub horizontal: bool,
}

impl ScrollBarParts {
    /// Where the track starts and how long it is, along the main axis.
    pub fn track(&self) -> (f32, f32) {
        if self.horizontal {
            (self.start.max.x, self.end.min.x - self.start.max.x)
        } else {
            (self.start.max.y, self.end.min.y - self.start.max.y)
        }
    }

    /// Where the thumb starts and how long it is, along the main axis.
    pub fn thumb_span(&self) -> (f32, f32) {
        if self.horizontal {
            (self.thumb.min.x, self.thumb.width())
        } else {
            (self.thumb.min.y, self.thumb.height())
        }
    }
}

/// The parts of the bar that fills `rect`, with the thumb at `fraction`
/// (0.0 at the start, 1.0 at the end).
///
/// It withstands any rectangle, the empty one included: what is left comes
/// out as a zero-area rectangle, which paints as nothing.
pub fn scroll_bar_parts(rect: egui::Rect, fraction: f32) -> ScrollBarParts {
    let horizontal = rect.width() > rect.height();
    let (along, thick) = if horizontal {
        (rect.width(), rect.height())
    } else {
        (rect.height(), rect.width())
    };
    let along = along.max(0.0);
    let thick = thick.max(0.0);
    // The short side rules the button.
    let button = thick.min(SCROLL_BAR_BUTTON);
    // The track is what is left between the two buttons, and zero when
    // nothing is.
    let track = (along - button * 2.0).max(0.0);
    let thumb = (track * SCROLL_BAR_THUMB_SHARE)
        .max(SCROLL_BAR_MIN_THUMB)
        .min(track);
    let advance = button + fraction.clamp(0.0, 1.0) * (track - thumb);
    // On a bar thinner than the gap there is no gap: the thumb never paints
    // outside the bar.
    let gap = SCROLL_BAR_INSET.min((thick * 0.5 - 0.5).max(0.0));
    let thumb_thick = (thick - gap * 2.0).max(0.0);
    let o = rect.min;
    if horizontal {
        ScrollBarParts {
            start: egui::Rect::from_min_size(o, egui::vec2(button, thick)),
            end: egui::Rect::from_min_size(
                egui::pos2(o.x + along - button, o.y),
                egui::vec2(button, thick),
            ),
            thumb: egui::Rect::from_min_size(
                egui::pos2(o.x + advance, o.y + gap),
                egui::vec2(thumb, thumb_thick),
            ),
            horizontal,
        }
    } else {
        ScrollBarParts {
            start: egui::Rect::from_min_size(o, egui::vec2(thick, button)),
            end: egui::Rect::from_min_size(
                egui::pos2(o.x, o.y + along - button),
                egui::vec2(thick, button),
            ),
            thumb: egui::Rect::from_min_size(
                egui::pos2(o.x + gap, o.y + advance),
                egui::vec2(thumb_thick, thumb),
            ),
            horizontal,
        }
    }
}

/// How far `position` is between `min` and `max`, from 0.0 to 1.0.
pub fn scroll_bar_fraction(position: i64, min: i64, max: i64) -> f32 {
    let (lo, hi) = (min.min(max), min.max(max));
    if hi == lo {
        return 0.0;
    }
    ((position.clamp(lo, hi) - lo) as f64 / (hi - lo) as f64) as f32
}

/// Paints a scroll bar inside `rect`: sunken track, the two raised buttons
/// with their arrow, and the thumb.
///
/// A disabled bar keeps its track and buttons, greys its arrows and shows no
/// thumb: a thumb promises a drag, and a disabled bar has nothing to drag.
pub fn paint_scroll_bar(
    painter: &egui::Painter,
    rect: egui::Rect,
    fraction: f32,
    enabled: bool,
    visuals: &egui::Visuals,
) {
    if rect.width() < 1.0 || rect.height() < 1.0 {
        return;
    }
    let w = &visuals.widgets;
    painter.rect(
        rect,
        2.0,
        visuals.extreme_bg_color,
        w.noninteractive.bg_stroke,
        egui::StrokeKind::Inside,
    );
    let parts = scroll_bar_parts(rect, fraction);
    let arrow = if enabled {
        visuals.text_color()
    } else {
        visuals.weak_text_color()
    };
    let (back, forth) = if parts.horizontal {
        (egui::vec2(-1.0, 0.0), egui::vec2(1.0, 0.0))
    } else {
        (egui::vec2(0.0, -1.0), egui::vec2(0.0, 1.0))
    };
    for (button, direction) in [(parts.start, back), (parts.end, forth)] {
        painter.rect(
            button,
            2.0,
            w.inactive.bg_fill,
            w.noninteractive.bg_stroke,
            egui::StrokeKind::Inside,
        );
        // The arrow sits in the largest square centred in the button, so a
        // thick bar does not stretch it.
        let side = button.width().min(button.height());
        if side >= 4.0 {
            let c = button.center();
            let tip = direction * side * 0.22;
            let wing = egui::vec2(-direction.y, direction.x) * side * 0.22;
            painter.add(egui::Shape::convex_polygon(
                vec![c + tip, c - tip + wing, c - tip - wing],
                arrow,
                egui::Stroke::NONE,
            ));
        }
    }
    if enabled {
        painter.rect(
            parts.thumb,
            2.0,
            w.active.bg_fill,
            w.noninteractive.bg_stroke,
            egui::StrokeKind::Inside,
        );
    }
}

/// A scroll bar the user drives, filling `rect`; `true` when the position
/// moved.
///
/// The classic gestures: a button moves one `small_change`, a click on the
/// track jumps one `large_change` towards the pointer, and the thumb drags.
/// With the focus, the arrow keys step, PageUp/PageDown jump, and Home/End go
/// to the ends.
pub fn scroll_bar(
    ui: &mut egui::Ui,
    name: &str,
    rect: egui::Rect,
    position: &mut i64,
    (min, max): (i64, i64),
    (small_change, large_change): (i64, i64),
    enabled: bool,
) -> bool {
    let enabled = enabled && ui.is_enabled();
    let (lo, hi) = (min.min(max), min.max(max));
    let id = ui.id().with(name);
    let sense = if enabled {
        egui::Sense::click_and_drag()
    } else {
        egui::Sense::hover()
    };
    let resp = ui.interact(rect, id, sense);
    let parts = scroll_bar_parts(rect, scroll_bar_fraction(*position, lo, hi));
    let along = |p: egui::Pos2| if parts.horizontal { p.x } else { p.y };
    let small = small_change.max(1);
    let large = large_change.max(1);
    let grab_id = id.with("grab");
    let mut target = (*position).clamp(lo, hi);
    if enabled {
        let pressed = resp.is_pointer_button_down_on() && ui.input(|i| i.pointer.primary_pressed());
        let pointer = ui.input(|i| i.pointer.interact_pos());
        let (thumb_start, thumb_len) = parts.thumb_span();
        if let (true, Some(p)) = (pressed, pointer) {
            resp.request_focus();
            if parts.start.contains(p) {
                target -= small;
            } else if parts.end.contains(p) {
                target += small;
            } else if parts.thumb.contains(p) {
                ui.data_mut(|d| d.insert_temp(grab_id, along(p) - thumb_start));
            } else if along(p) < thumb_start {
                target -= large;
            } else {
                target += large;
            }
        }
        let grab = ui.data(|d| d.get_temp::<f32>(grab_id));
        if resp.is_pointer_button_down_on() {
            if let (Some(grab), Some(p)) = (grab, pointer) {
                let (track_start, track_len) = parts.track();
                let travel = track_len - thumb_len;
                if travel > 0.0 {
                    let ratio = ((along(p) - grab - track_start) / travel).clamp(0.0, 1.0);
                    target = lo + (ratio as f64 * (hi - lo) as f64).round() as i64;
                }
            }
        } else if grab.is_some() {
            ui.data_mut(|d| d.remove::<f32>(grab_id));
        }
        if resp.has_focus() {
            ui.memory_mut(|m| {
                m.set_focus_lock_filter(
                    id,
                    egui::EventFilter {
                        horizontal_arrows: true,
                        vertical_arrows: true,
                        ..Default::default()
                    },
                )
            });
            let (back, forth) = if parts.horizontal {
                (egui::Key::ArrowLeft, egui::Key::ArrowRight)
            } else {
                (egui::Key::ArrowUp, egui::Key::ArrowDown)
            };
            ui.input(|i| {
                if i.key_pressed(back) {
                    target -= small;
                }
                if i.key_pressed(forth) {
                    target += small;
                }
                if i.key_pressed(egui::Key::PageUp) {
                    target -= large;
                }
                if i.key_pressed(egui::Key::PageDown) {
                    target += large;
                }
                if i.key_pressed(egui::Key::Home) {
                    target = lo;
                }
                if i.key_pressed(egui::Key::End) {
                    target = hi;
                }
            });
        }
    }
    let target = target.clamp(lo, hi);
    let changed = target != *position;
    *position = target;
    paint_scroll_bar(
        ui.painter(),
        rect,
        scroll_bar_fraction(*position, lo, hi),
        enabled,
        ui.visuals(),
    );
    changed
}
// [CRAWRAD:FIM scroll_bar]
