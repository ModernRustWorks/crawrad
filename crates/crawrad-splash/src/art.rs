//! The picture behind the splash text: a night-to-ember gradient and our
//! mascot, a rounded ember gear with two eyes. Computed, not loaded, so the
//! splash has no image file to ship or decode, and it is sharp at any DPI.

use crawrad_note::note;
#[note("art-rgb")]
const fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | b as u32
}

pub(crate) const NIGHT: u32 = rgb(0x1d, 0x15, 0x34);
pub(crate) const EMBER: u32 = rgb(0xff, 0x8a, 0x2a);
pub(crate) const INK: u32 = rgb(0xf3, 0xee, 0xe9);
pub(crate) const MUTED: u32 = rgb(0xb9, 0xad, 0xb4);
pub(crate) const OK: u32 = rgb(0x7f, 0xdc, 0x9a);
const STOPS: [(f32, u32); 4] = [
    (0.0, NIGHT),
    (0.42, rgb(0x3a, 0x14, 0x30)),
    (0.78, rgb(0xa3, 0x22, 0x1c)),
    (1.0, rgb(0xe8, 0x34, 0x1c)),
];
const EYE: u32 = rgb(0x16, 0x12, 0x1d);
const GLINT: u32 = rgb(0xd9, 0xf3, 0xf0);

fn channel(c: u32, shift: u32) -> f32 {
    ((c >> shift) & 0xff) as f32
}

#[note("art-mix")]
pub(crate) fn mix(a: u32, b: u32, t: f32) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let one = |s| (channel(a, s) + (channel(b, s) - channel(a, s)) * t).round() as u32;
    (one(16) << 16) | (one(8) << 8) | one(0)
}

fn gradient(t: f32) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let at = STOPS
        .iter()
        .position(|(stop, _)| t <= *stop)
        .unwrap_or(STOPS.len() - 1)
        .max(1);
    let (lo, a) = STOPS[at - 1];
    let (hi, b) = STOPS[at];
    mix(a, b, (t - lo) / (hi - lo))
}

#[note("art-coverage")]
fn coverage(d: f32) -> f32 {
    (0.5 - d).clamp(0.0, 1.0)
}

#[note("art-ellipse")]
fn ellipse(x: f32, y: f32, rx: f32, ry: f32) -> f32 {
    let k = ((x / rx).powi(2) + (y / ry).powi(2)).sqrt();
    (k - 1.0) * rx.min(ry)
}

#[note("art-background")]
pub fn background(width: usize, height: usize) -> Vec<u32> {
    let (w, h) = (width as f32, height as f32);
    // The mascot sits low on the right and runs off the edges, as in the mock-up.
    let (cx, cy, radius) = (w * 0.80, h * 0.88, h * 0.47);
    let eye = |side: f32| (cx + side * radius * 0.24, cy - radius * 0.10);
    let mut pixels = Vec::with_capacity(width * height);
    for row in 0..height {
        for col in 0..width {
            let (x, y) = (col as f32 + 0.5, row as f32 + 0.5);
            // 160°: from the top left towards the bottom, slightly right.
            let mut c = gradient((x / w) * 0.34 + (y / h) * 0.94);
            let (dx, dy) = (x - cx, (y - cy) / 0.86);
            let angle = dy.atan2(dx);
            let edge = radius * (1.0 + 0.075 * (11.0 * angle).cos());
            c = mix(c, EMBER, coverage((dx * dx + dy * dy).sqrt() - edge));
            for side in [-1.0, 1.0] {
                let (ex, ey) = eye(side);
                let r = radius * 0.14;
                c = mix(c, EYE, coverage(ellipse(x - ex, y - ey, r * 0.78, r)));
                c = mix(
                    c,
                    GLINT,
                    coverage(ellipse(
                        x - ex - r * 0.2,
                        y - ey + r * 0.32,
                        r * 0.32,
                        r * 0.4,
                    )),
                );
            }
            pixels.push(c);
        }
    }
    pixels
}
