//! **WHERE A SINGLE LINE OF TEXT GOES** in a box of fixed height.
//!
//! Makepad places text by its line box, which keeps room for descenders (g, p,
//! y), so centring that box puts the letters about 1 px low. The rule here
//! centres the **capitals** instead. Measured at 11 pt (14.7 px): the capitals
//! are 0.68 of the pixel size tall and start 0.227 below the box top, so their
//! middle is 0.567 of the size below it.
//!
//! It lives in the box for the same reason as [`crate::fraction`]: four
//! runtime controls (Chips, Breadcrumb, Pagination, MultiSelect) carried their
//! own copy of the sum, and the generator needs it too, for the padding it
//! gives a theme widget (Edit). One answer, so they cannot drift.

/// Points to pixels: Makepad's `font_size` is in points (`pts * 96 / 72`).
///
/// # Examples
///
/// ```rust
/// use crawrad_component::text_line::font_px;
///
/// assert_eq!(font_px(72.0), 96.0);
/// ```
pub fn font_px(points: f32) -> f32 {
    points * 96.0 / 72.0
}

/// Computes the top offset to center text capitals within a box.
///
/// Where the line box starts in a box `box_h` tall so that the capitals of a
/// `font_px` text sit in the middle.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::text_line::cap_centred_top;
///
/// let top = cap_centred_top(40.0, 10.0);
/// assert_eq!(top, 20.0 - 5.67);
/// ```
pub fn cap_centred_top(box_h: f32, font_px: f32) -> f32 {
    box_h / 2.0 - 0.567 * font_px
}

/// Default arrow tip width for chevron segments in design units.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::text_line::CHEVRON_TIP;
///
/// assert_eq!(CHEVRON_TIP, 12.0);
/// ```
pub const CHEVRON_TIP: f32 = 12.0;

/// Default horizontal overlap into the next chevron segment in design units.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::text_line::CHEVRON_OVERLAP;
///
/// assert_eq!(CHEVRON_OVERLAP, 8.0);
/// ```
pub const CHEVRON_OVERLAP: f32 = 8.0;

/// Default padding tuple for chevron segments in design units.
///
/// Default padding tuple `(first_left, other_left, right)` for chevron segments in design units.
///
/// Segment 0 has ~14 px left padding; other segments have ~24 px left padding to clear the notch;
/// right padding is ~20 px to clear the arrow tip.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::text_line::CHEVRON_PADS;
///
/// assert_eq!(CHEVRON_PADS, (14.0, 24.0, 20.0));
/// ```
pub const CHEVRON_PADS: (f32, f32, f32) = (14.0, 24.0, 20.0);

/// Paddings for chevron segments.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::text_line::ChevronPads;
///
/// let pads = ChevronPads::from((14.0, 24.0, 20.0));
/// assert_eq!(pads.first_left, 14.0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChevronPads {
    pub first_left: f32,
    pub other_left: f32,
    pub right: f32,
}

impl From<(f32, f32, f32)> for ChevronPads {
    fn from((first_left, other_left, right): (f32, f32, f32)) -> Self {
        Self {
            first_left,
            other_left,
            right,
        }
    }
}

/// Pure function computing horizontal `(x, width)` for each chevron segment.
///
/// Segment 0 has a straight left edge with padding `pads.first_left`.
/// Subsequent segments have a notched left edge with padding `pads.other_left`.
/// Each segment's right edge has an arrow tip with padding `pads.right`.
/// Adjacent segments overlap by `overlap` pixels: segment `i + 1` starts at `x_i + width_i - overlap`.
///
/// # Examples
///
/// ```rust
/// use crawrad_component::text_line::{CHEVRON_OVERLAP, CHEVRON_PADS, CHEVRON_TIP, chevron_layout};
///
/// let layout = chevron_layout(&[50.0], CHEVRON_TIP, CHEVRON_OVERLAP, CHEVRON_PADS);
/// assert_eq!(layout.len(), 1);
/// assert_eq!(layout[0].0, 0.0);
/// ```
pub fn chevron_layout(
    widths: &[f32],
    tip: f32,
    overlap: f32,
    pads: impl Into<ChevronPads>,
) -> Vec<(f32, f32)> {
    let pads = pads.into();
    let mut positions = Vec::with_capacity(widths.len());
    let mut x = 0.0;
    for (i, &w) in widths.iter().enumerate() {
        let left_pad = if i == 0 {
            pads.first_left
        } else {
            pads.other_left
        };
        let block_w = (left_pad + w + pads.right).max(tip);
        positions.push((x, block_w));
        x += block_w - overlap;
    }
    positions
}

#[cfg(test)]
#[path = "test/text_line/test.rs"]
mod tests;
