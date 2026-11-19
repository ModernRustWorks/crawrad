//! The IDE's splash screen: what the IDE is doing while its own
//! window does not exist yet.
//!
//! The IDE's window waits for the GPU shaders, and on a first start that took
//! 13 s. The splash comes up in well under a second, on a thread of its
//! own, and follows the start on its own, from the shader counter the
//! vendored Makepad keeps (`makepad_platform::shader_progress`):
//!
//! 1. **Starting the IDE** — until the first shaders are asked for;
//! 2. **Preparing graphics `N / M`** — while they compile, "(cached)" when
//!    every one so far came from the disk cache;
//! 3. **Showing the window** — until the IDE's window is on the screen,
//!    which closes the splash.
//!
//! The IDE calls [`open`] once, at the top of `main`; nothing else. The look
//! is the mock-up the owner approved (25/10/2026). The window itself is
//! `crawrad_os::splash_window`, the only place with the `unsafe` it needs.
//!
//! # Platforms
//!
//! Everything in this crate is plain Rust and the same on every target: the
//! steps, the picture (computed, no image file) and where each line goes.
//! Only the window is per system, and today only Windows has one, where the
//! wait is long (DirectX's shader compiler keeps nothing between starts).
//! Linux (X11/Wayland) and macOS (AppKit) each need their own window in
//! `crawrad_os::splash_window`; until then `open` shows nothing there and the
//! IDE starts as before. The owner wants the idea on every system
//! (25/10/2026): measure the wait on Linux first, then the X11 window.

#![forbid(unsafe_code)]

mod art;

use crawrad_note::note;
use crawrad_os::splash_window::{Fill, Frame, Painter, Text};
use std::time::{Duration, Instant};

#[note("shadercount")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ShaderCount {
    pub done: usize,
    pub total: usize,
    pub compiled: usize,
}

impl ShaderCount {
    fn from_tuple((done, total, compiled): (usize, usize, usize)) -> ShaderCount {
        ShaderCount {
            done,
            total,
            compiled,
        }
    }
}

#[note("phases")]
pub const PHASES: [&str; 3] = [
    "Starting the IDE",
    "Preparing graphics",
    "Showing the window",
];

#[note("phase")]
pub fn phase(shaders: ShaderCount, before: usize) -> usize {
    let now = if shaders.total == 0 {
        0
    } else if shaders.done < shaders.total {
        1
    } else {
        2
    };
    now.max(before)
}

#[note("step")]
#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    pub label: String,
    pub took: Option<Duration>,
}

#[note("steps")]
#[derive(Debug, Default)]
pub struct Steps {
    list: Vec<Step>,
    started: Option<Instant>,
}

impl Steps {
    #[note("steps-begin")]
    pub fn begin(&mut self, label: &str, now: Instant) {
        if let (Some(last), Some(started)) = (self.list.last_mut(), self.started) {
            last.took = Some(now - started);
        }
        self.list.push(Step {
            label: label.to_string(),
            took: None,
        });
        self.started = Some(now);
    }

    pub fn list(&self) -> &[Step] {
        &self.list
    }

    #[note("steps-current-line")]
    pub fn current_line(&self, shaders: ShaderCount) -> String {
        let Some(step) = self.list.last() else {
            return String::new();
        };
        if shaders.total == 0 || shaders.done >= shaders.total {
            return step.label.clone();
        }
        let source = if shaders.compiled == 0 {
            " (cached)"
        } else {
            ""
        };
        format!(
            "{}{source}  {} / {}",
            step.label, shaders.done, shaders.total
        )
    }

    #[note("steps-fraction")]
    pub fn fraction(&self, expected: usize, shaders: ShaderCount) -> f32 {
        let done = self.list.len().saturating_sub(1) as f32;
        let running = if shaders.total > 0 {
            shaders.done as f32 / shaders.total as f32
        } else {
            0.0
        };
        ((done + running) / expected.max(1) as f32).clamp(0.0, 1.0)
    }
}

#[note("size")]
const SIZE: (f32, f32) = (640.0, 400.0);
const LEFT: f32 = 42.0;
#[note("column")]
const COLUMN: f32 = 300.0;
const ROW: f32 = 21.0;
#[note("give-up")]
const GIVE_UP: Duration = Duration::from_secs(120);

fn text(text: &str, area: [f32; 4], color: u32, size: f32, weight: i32) -> Text {
    Text {
        text: text.to_string(),
        area,
        color,
        size,
        weight,
        align_right: false,
    }
}

#[note("layout")]
pub fn layout(steps: &Steps, shaders: ShaderCount) -> Frame {
    let mut frame = Frame::default();
    frame.texts.push(text(
        "Welcome to",
        [LEFT, 30.0, 600.0, 72.0],
        art::EMBER,
        33.0,
        600,
    ));
    frame.texts.push(text(
        "CrawRAD",
        [LEFT - 3.0, 66.0, 600.0, 146.0],
        art::INK,
        70.0,
        800,
    ));
    frame.texts.push(text(
        "VISUAL RAD FOR RUST",
        [LEFT, 146.0, 600.0, 168.0],
        art::MUTED,
        13.5,
        400,
    ));
    let lines = steps.list();
    let base = 322.0 - ROW * lines.len() as f32;
    for (i, step) in lines.iter().enumerate() {
        let top = base + ROW * i as f32;
        let running = step.took.is_none();
        frame.fills.push(Fill {
            area: [LEFT, top + 7.0, LEFT + 7.0, top + 14.0],
            color: if running { art::EMBER } else { art::OK },
        });
        let (label, color) = if running {
            (steps.current_line(shaders), art::INK)
        } else {
            (step.label.clone(), art::MUTED)
        };
        frame.texts.push(text(
            &label,
            [LEFT + 16.0, top, COLUMN, top + ROW],
            color,
            13.5,
            400,
        ));
        if let Some(took) = step.took {
            let mut secs = text(
                &format!("{:.1} s", took.as_secs_f32()),
                [COLUMN - 80.0, top, COLUMN, top + ROW],
                art::MUTED,
                13.5,
                400,
            );
            secs.align_right = true;
            frame.texts.push(secs);
        }
    }
    let fraction = steps.fraction(PHASES.len(), shaders);
    frame.fills.push(Fill {
        area: [LEFT, 360.0, COLUMN, 363.0],
        color: art::mix(art::NIGHT, art::INK, 0.18),
    });
    frame.fills.push(Fill {
        area: [LEFT, 360.0, LEFT + (COLUMN - LEFT) * fraction, 363.0],
        color: art::EMBER,
    });
    frame
}

#[note("splash")]
struct Splash {
    progress: fn() -> (usize, usize, usize),
    steps: Steps,
    phase: usize,
    opened: Instant,
}

impl Painter for Splash {
    fn size(&self) -> (f32, f32) {
        SIZE
    }

    fn picture(&self, width: usize, height: usize) -> Vec<u32> {
        art::background(width, height)
    }

    fn frame(&mut self, other_window_up: bool) -> Option<Frame> {
        if other_window_up || self.opened.elapsed() > GIVE_UP {
            return None;
        }
        let shaders = ShaderCount::from_tuple((self.progress)());
        let phase = phase(shaders, self.phase);
        while self.phase < phase {
            self.phase += 1;
            self.steps.begin(PHASES[self.phase], Instant::now());
        }
        Some(layout(&self.steps, shaders))
    }
}

#[note("open")]
pub fn open(progress: fn() -> (usize, usize, usize)) {
    let mut steps = Steps::default();
    steps.begin(PHASES[0], Instant::now());
    crawrad_os::splash_window::spawn(Splash {
        progress,
        steps,
        phase: 0,
        opened: Instant::now(),
    });
}

#[cfg(test)]
#[path = "test/test.rs"]
mod test;
