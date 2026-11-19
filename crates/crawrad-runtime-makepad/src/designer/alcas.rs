//! Selection and resize handle management for Makepad designer.

use crawrad_ir::Rect;
use crawrad_note::note;
use crawrad_runtime::Axis;
use makepad_widgets::MouseCursor;

pub const EIXO_DAS_ALCAS: [Axis; 8] = [
    Axis::Nwse,
    Axis::Ns,
    Axis::Nesw,
    Axis::We,
    Axis::Nwse,
    Axis::Ns,
    Axis::Nesw,
    Axis::We,
];

pub fn seta(e: Axis) -> MouseCursor {
    match e {
        Axis::Nwse => MouseCursor::NwseResize,
        Axis::Nesw => MouseCursor::NeswResize,
        Axis::We => MouseCursor::EwResize,
        Axis::Ns => MouseCursor::NsResize,
    }
}

pub fn alcas_do_formulario(r: Rect) -> [(f32, f32); 3] {
    let a = crawrad_runtime::handles(r.x, r.y, r.w, r.h);
    [a[3], a[5], a[4]]
}

#[note("designer-alcas-handle-at-point")]
pub fn handle_at_point(r: Rect, x: f32, y: f32) -> Option<Axis> {
    let meio = crawrad_runtime::HANDLE_TARGET / 2.0;
    crawrad_runtime::handles(r.x, r.y, r.w, r.h)
        .iter()
        .position(|(ax, ay)| (x - ax).abs() <= meio && (y - ay).abs() <= meio)
        .map(|i| EIXO_DAS_ALCAS[i])
}

#[note("designer-alcas-handle-at-point-in-form")]
pub fn handle_at_point_in_form(r: Rect, x: f32, y: f32) -> Option<Axis> {
    let meio = crawrad_runtime::HANDLE_TARGET / 2.0;
    let a = crawrad_runtime::handles(r.x, r.y, r.w, r.h);

    if (x - a[4].0).abs() <= meio && (y - a[4].1).abs() <= meio {
        return Some(EIXO_DAS_ALCAS[4]);
    }

    let right_edge = r.x + r.w;
    let on_e_handle = (x - a[3].0).abs() <= meio && (y - a[3].1).abs() <= meio;
    let on_e_border = (x - right_edge).abs() <= 6.0 && y >= r.y && y <= (r.y + r.h);
    if on_e_handle || on_e_border {
        return Some(EIXO_DAS_ALCAS[3]);
    }

    let bottom_edge = r.y + r.h;
    let on_s_handle = (x - a[5].0).abs() <= meio && (y - a[5].1).abs() <= meio;
    let on_s_border = (y - bottom_edge).abs() <= 6.0 && x >= r.x && x <= (r.x + r.w);
    if on_s_handle || on_s_border {
        return Some(EIXO_DAS_ALCAS[5]);
    }

    None
}
