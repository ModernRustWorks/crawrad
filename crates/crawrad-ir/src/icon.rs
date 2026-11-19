//! The closed vocabulary of a class icon (16×16 grid, one ink, theme-tinted).
//!
//! The CPU rasterizer is an exact port of Makepad 1.0 `Sdf2d`
//! (`makepad-draw-1.0.0/src/shader/std.rs`) so the data draws the same pixels
//! the shader drew.

use crawrad_note::note;
#[note("icon-grid")]
pub const GRID: f32 = 16.0;

#[note("icon-stroke")]
pub const STROKE: f32 = 1.25;

#[note("icon-iconstep")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconStep {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    ClosePath,
    Circle {
        x: f32,
        y: f32,
        r: f32,
    },
    Box {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        r: f32,
    },
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    },
    Fill,
    Stroke,
}

#[note("icon-iconerror")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IconError {
    Empty,
    NothingPainted,
    LineWithoutStart(usize),
    OutOfGrid(usize),
    NotFinite(usize),
}

impl std::fmt::Display for IconError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "an icon drawing cannot be empty"),
            Self::NothingPainted => write!(f, "the last step of an icon must be fill or stroke"),
            Self::LineWithoutStart(i) => {
                write!(
                    f,
                    "step {i} draws a line before any move_to set a start point"
                )
            }
            Self::OutOfGrid(i) => {
                write!(
                    f,
                    "step {i} uses coordinates or dimensions outside the 16x16 grid"
                )
            }
            Self::NotFinite(i) => write!(f, "step {i} contains a non-finite number"),
        }
    }
}

impl std::error::Error for IconError {}

#[note("icon-validate")]
pub fn validate(steps: &[IconStep]) -> Result<(), IconError> {
    if steps.is_empty() {
        return Err(IconError::Empty);
    }
    let Some(last) = steps.last() else {
        return Err(IconError::Empty);
    };
    if !matches!(last, IconStep::Fill | IconStep::Stroke) {
        return Err(IconError::NothingPainted);
    }

    let mut has_move_to = false;
    for (idx, step) in steps.iter().enumerate() {
        match *step {
            IconStep::MoveTo(x, y) => {
                if !x.is_finite() || !y.is_finite() {
                    return Err(IconError::NotFinite(idx));
                }
                if !(0.0..=GRID).contains(&x) || !(0.0..=GRID).contains(&y) {
                    return Err(IconError::OutOfGrid(idx));
                }
                has_move_to = true;
            }
            IconStep::LineTo(x, y) => {
                if !x.is_finite() || !y.is_finite() {
                    return Err(IconError::NotFinite(idx));
                }
                if !has_move_to {
                    return Err(IconError::LineWithoutStart(idx));
                }
                if !(0.0..=GRID).contains(&x) || !(0.0..=GRID).contains(&y) {
                    return Err(IconError::OutOfGrid(idx));
                }
            }
            IconStep::ClosePath => {
                if !has_move_to {
                    return Err(IconError::LineWithoutStart(idx));
                }
            }
            IconStep::Circle { x, y, r } => {
                if !x.is_finite() || !y.is_finite() || !r.is_finite() {
                    return Err(IconError::NotFinite(idx));
                }
                if !(0.0..=GRID).contains(&x) || !(0.0..=GRID).contains(&y) || r < 0.0 {
                    return Err(IconError::OutOfGrid(idx));
                }
            }
            IconStep::Box { x, y, w, h, r } => {
                if !x.is_finite()
                    || !y.is_finite()
                    || !w.is_finite()
                    || !h.is_finite()
                    || !r.is_finite()
                {
                    return Err(IconError::NotFinite(idx));
                }
                if !(0.0..=GRID).contains(&x)
                    || !(0.0..=GRID).contains(&y)
                    || w < 0.0
                    || h < 0.0
                    || r < 0.0
                    || x + w > GRID
                    || y + h > GRID
                {
                    return Err(IconError::OutOfGrid(idx));
                }
            }
            IconStep::Rect { x, y, w, h } => {
                if !x.is_finite() || !y.is_finite() || !w.is_finite() || !h.is_finite() {
                    return Err(IconError::NotFinite(idx));
                }
                if !(0.0..=GRID).contains(&x)
                    || !(0.0..=GRID).contains(&y)
                    || w < 0.0
                    || h < 0.0
                    || x + w > GRID
                    || y + h > GRID
                {
                    return Err(IconError::OutOfGrid(idx));
                }
            }
            IconStep::Fill | IconStep::Stroke => {}
        }
    }
    Ok(())
}

#[note("icon-ink-alpha")]
pub fn ink_alpha(coverage: f32) -> f32 {
    // Half a pixel of coverage already reads as full ink: a 1.25-unit stroke
    // at one pixel per unit lands as two half-covered columns, and taking the
    // coverage literally drew it at half strength.
    ((coverage * 2.0).min(1.0) * 4.0).round() / 4.0
}

#[note("icon-rasterize")]
pub fn rasterize(steps: &[IconStep], half_stroke: f32, side: u32) -> Vec<f32> {
    rasterize_oversampled(steps, half_stroke, side, 1)
}

#[note("icon-rasterize-oversampled")]
pub fn rasterize_oversampled(
    steps: &[IconStep],
    half_stroke: f32,
    side: u32,
    factor: u32,
) -> Vec<f32> {
    if side == 0 || factor == 0 {
        return Vec::new();
    }
    let u = side as f32 / GRID;
    let aa = u / 2.0_f32.sqrt();
    let samples = side * factor;
    let step = u * factor as f32;
    let mut coverage = Vec::with_capacity((samples * samples) as usize);

    for iy in 0..samples {
        let pos_y = (iy as f32 + 0.5) / step;
        for ix in 0..samples {
            let pos_x = (ix as f32 + 0.5) / step;
            let mut pen = Pen::new((pos_x, pos_y), aa, half_stroke);
            for &step in steps {
                pen.execute(step);
            }
            coverage.push(pen.result());
        }
    }

    coverage
}

#[derive(Debug)]
struct Pen {
    pos: (f32, f32),
    aa: f32,
    half_stroke: f32,
    result: f32,
    shape: f32,
    clip: f32,
    has_clip: bool,
    last: (f32, f32),
    start: (f32, f32),
}

impl Pen {
    fn new(pos: (f32, f32), aa: f32, half_stroke: f32) -> Self {
        Self {
            pos,
            aa,
            half_stroke,
            result: 0.0,
            shape: 1e20,
            clip: -1e20,
            has_clip: false,
            last: (0.0, 0.0),
            start: (0.0, 0.0),
        }
    }

    fn calc_blur(&self, w: f32) -> f32 {
        (-w * self.aa).clamp(0.0, 1.0)
    }

    fn execute(&mut self, step: IconStep) {
        match step {
            IconStep::MoveTo(x, y) => {
                self.last = (x, y);
                self.start = (x, y);
            }
            IconStep::LineTo(x, y) => {
                self.line_to((x, y));
            }
            IconStep::ClosePath => {
                self.line_to(self.start);
            }
            IconStep::Circle { x, y, r } => {
                let dx = self.pos.0 - x;
                let dy = self.pos.1 - y;
                let dist = (dx * dx + dy * dy).sqrt() - r;
                self.shape = self.shape.min(dist);
            }
            IconStep::Box { x, y, w, h, r } => {
                let p = (self.pos.0 - x, self.pos.1 - y);
                let size = (w * 0.5, h * 0.5);
                let r2 = 2.0 * r;
                let bp_x = ((p.0 - size.0).abs() - (size.0 - r2)).max(0.0);
                let bp_y = ((p.1 - size.1).abs() - (size.1 - r2)).max(0.0);
                let dist = (bp_x * bp_x + bp_y * bp_y).sqrt() - r2;
                self.shape = self.shape.min(dist);
            }
            IconStep::Rect { x, y, w, h } => {
                let s = (w * 0.5, h * 0.5);
                let d_x = (x - self.pos.0 + s.0).abs() - s.0;
                let d_y = (y - self.pos.1 + s.1).abs() - s.1;
                let dm_x = d_x.min(0.0);
                let dm_y = d_y.min(0.0);
                let max_d_x = d_x.max(0.0);
                let max_d_y = d_y.max(0.0);
                let dist = dm_x.max(dm_y) + (max_d_x * max_d_x + max_d_y * max_d_y).sqrt();
                self.shape = self.shape.min(dist);
            }
            IconStep::Fill => {
                let f = self.calc_blur(self.shape);
                self.result = f + self.result * (1.0 - f);
                if self.has_clip {
                    let f2 = 1.0 - self.calc_blur(-self.clip);
                    self.result = f2 + self.result * (1.0 - f2);
                }
                self.shape = 1e20;
                self.clip = -1e20;
                self.has_clip = false;
            }
            IconStep::Stroke => {
                let f = self.calc_blur(self.shape.abs() - self.half_stroke);
                self.result = f + self.result * (1.0 - f);
                self.shape = 1e20;
                self.clip = -1e20;
                self.has_clip = false;
            }
        }
    }

    fn line_to(&mut self, p: (f32, f32)) {
        let pa = (self.pos.0 - self.last.0, self.pos.1 - self.last.1);
        let ba = (p.0 - self.last.0, p.1 - self.last.1);
        let ba_dot = ba.0 * ba.0 + ba.1 * ba.1;
        let h = if ba_dot == 0.0 {
            0.0
        } else {
            ((pa.0 * ba.0 + pa.1 * ba.1) / ba_dot).clamp(0.0, 1.0)
        };
        let cross = pa.0 * ba.1 - pa.1 * ba.0;
        let s = if cross > 0.0 {
            1.0
        } else if cross < 0.0 {
            -1.0
        } else {
            0.0
        };
        let diff = (pa.0 - ba.0 * h, pa.1 - ba.1 * h);
        let dist = (diff.0 * diff.0 + diff.1 * diff.1).sqrt();
        self.shape = self.shape.min(dist);
        self.clip = self.clip.max(dist * s);
        self.has_clip = true;
        self.last = p;
    }

    fn result(&self) -> f32 {
        self.result.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
#[path = "test/icon/test.rs"]
mod test;
