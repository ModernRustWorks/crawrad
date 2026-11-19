//! Cursor icon mapping for Makepad visual designer.

use crawrad_note::note;
use crawrad_runtime::Axis;
use makepad_widgets::MouseCursor;

#[note("designer-cursor-cursor-para-eixo")]
pub fn cursor_para_eixo(eixo: Axis) -> MouseCursor {
    match eixo {
        Axis::Nwse => MouseCursor::NwseResize,
        Axis::Nesw => MouseCursor::NeswResize,
        Axis::We => MouseCursor::EwResize,
        Axis::Ns => MouseCursor::NsResize,
    }
}
