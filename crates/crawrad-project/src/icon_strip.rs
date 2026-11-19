//! The pure half of the class icon strip: which cells it holds and their pixels.
//!
//! The shell only uploads the pixels as a texture; everything that can be decided
//! without a GPU lives here, so the catalogue's third-party icons are counted and
//! rasterized where a test reaches them without a window.

use crawrad_ir::icon::{IconStep, rasterize_oversampled};

use crate::icons::ClassIcon;
use crawrad_note::note;

#[note("icon-strip-icon-stroke")]
pub const ICON_STROKE: f32 = crawrad_ir::icon::STROKE;

#[note("icon-strip-oversample")]
pub const OVERSAMPLE: u32 = 4;

#[note("icon-strip-extra-icons")]
pub fn extra_icons(reg: &crawrad_registry::Registry) -> Vec<(String, Vec<IconStep>)> {
    let mut extras: Vec<(String, Vec<IconStep>)> = reg
        .all()
        .filter(|d| ClassIcon::of_class(d.class).is_none())
        .filter_map(|d| Some((d.class.to_string(), icon_of(d)?)))
        .collect();
    extras.sort_by(|a, b| a.0.cmp(&b.0));
    extras
}

#[note("icon-strip-icon-of")]
pub fn icon_of(d: &crawrad_registry::ComponentDesc) -> Option<Vec<IconStep>> {
    if let Some(own) = ClassIcon::of_class(d.class) {
        return Some(own.steps().to_vec());
    }
    if !d.icon.is_empty() {
        return Some(d.icon.clone());
    }
    // An inheriting component with no icon of its own wears its base's.
    d.inherits
        .and_then(ClassIcon::of_class)
        .map(|base| base.steps().to_vec())
}

#[note("icon-strip-strip-pixels")]
pub fn strip_pixels(side: u32, extra: &[(String, Vec<IconStep>)]) -> (usize, usize, Vec<u32>) {
    let count = ClassIcon::ALL.len() + extra.len();
    let cell = (side * OVERSAMPLE) as usize;
    let width = count * cell;
    let height = cell;
    let mut buf = vec![0u32; width * height];
    let half_stroke = ICON_STROKE * 0.5;

    let all_steps = ClassIcon::ALL
        .iter()
        .map(|icon| icon.steps())
        .chain(extra.iter().map(|(_, steps)| steps.as_slice()));

    for (k, steps) in all_steps.enumerate() {
        let coverage = rasterize_oversampled(steps, half_stroke, side, OVERSAMPLE);
        for iy in 0..cell {
            for ix in 0..cell {
                let a = coverage[iy * cell + ix];
                let b = (a * 255.0).round() as u32;
                let pixel = (b << 24) | (b << 16) | (b << 8) | b;
                let x = k * cell + ix;
                let y = iy;
                buf[y * width + x] = pixel;
            }
        }
    }

    (width, height, buf)
}

#[note("icon-strip-iconcells")]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct IconCells {
    extra: Vec<(String, Vec<IconStep>)>,
}

impl IconCells {
    pub fn new(extra: Vec<(String, Vec<IconStep>)>) -> Self {
        Self { extra }
    }

    #[note("icon-strip-iconcells-extra")]
    pub fn extra(&self) -> &[(String, Vec<IconStep>)] {
        &self.extra
    }

    #[note("icon-strip-iconcells-kind-of")]
    pub fn kind_of(&self, class: &str) -> Option<f64> {
        ClassIcon::of_class(class).map(ClassIcon::kind).or_else(|| {
            self.extra
                .iter()
                .position(|(c, _)| c == class)
                .map(|idx| (ClassIcon::ALL.len() + idx) as f64)
        })
    }

    #[note("icon-strip-iconcells-count")]
    pub fn count(&self) -> f64 {
        (ClassIcon::ALL.len() + self.extra.len()) as f64
    }
}

#[cfg(test)]
#[path = "test/icon_strip/test.rs"]
mod test;
