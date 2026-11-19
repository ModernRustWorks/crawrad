use super::{ShaderCount, Steps, art};
use std::time::{Duration, Instant};

#[test]
fn a_step_gets_its_time_when_the_next_one_begins() {
    let t0 = Instant::now();
    let mut steps = Steps::default();
    steps.begin("Starting the IDE", t0);
    assert_eq!(steps.list()[0].took, None);
    steps.begin("Preparing graphics", t0 + Duration::from_millis(400));
    assert_eq!(steps.list()[0].took, Some(Duration::from_millis(400)));
    assert_eq!(steps.list()[1].took, None);
}

#[test]
fn the_shader_counter_rides_on_the_running_step_until_it_is_done() {
    let mut steps = Steps::default();
    steps.begin("Preparing graphics", Instant::now());
    let none = ShaderCount::default();
    assert_eq!(steps.current_line(none), "Preparing graphics");
    let cold = ShaderCount {
        done: 47,
        total: 206,
        compiled: 47,
    };
    assert_eq!(steps.current_line(cold), "Preparing graphics  47 / 206");
    let warm = ShaderCount {
        done: 47,
        total: 206,
        compiled: 0,
    };
    assert_eq!(
        steps.current_line(warm),
        "Preparing graphics (cached)  47 / 206"
    );
    let over = ShaderCount {
        done: 206,
        total: 206,
        compiled: 206,
    };
    assert_eq!(steps.current_line(over), "Preparing graphics");
}

#[test]
fn the_bar_counts_finished_steps_and_the_shaders_of_the_running_one() {
    let t0 = Instant::now();
    let mut steps = Steps::default();
    steps.begin("a", t0);
    steps.begin("b", t0);
    let half = ShaderCount {
        done: 50,
        total: 100,
        compiled: 50,
    };
    assert!((steps.fraction(3, half) - 0.5).abs() < 1e-6);
    assert_eq!(
        steps.fraction(0, ShaderCount::default()),
        1.0,
        "never divides by zero"
    );
}

#[test]
fn the_picture_fills_the_window_and_keeps_its_colours() {
    let pixels = art::background(64, 40);
    assert_eq!(pixels.len(), 64 * 40);
    let red = |c: u32| c >> 16;
    assert!(
        red(pixels[0]) <= red(art::NIGHT) + 4,
        "the top left is night"
    );
    assert!(
        red(pixels[64 * 39]) > red(pixels[0]),
        "the bottom is warmer"
    );
    assert!(pixels.contains(&art::EMBER), "the mascot is drawn");
}

#[test]
fn the_phase_follows_the_counter_and_never_goes_back() {
    use super::phase;
    let none = ShaderCount::default();
    let half = ShaderCount {
        done: 3,
        total: 9,
        compiled: 3,
    };
    let all = ShaderCount {
        done: 9,
        total: 9,
        compiled: 9,
    };
    assert_eq!(phase(none, 0), 0);
    assert_eq!(phase(half, 0), 1);
    assert_eq!(phase(all, 1), 2);
    let later_batch = ShaderCount {
        done: 9,
        total: 12,
        compiled: 9,
    };
    assert_eq!(
        phase(later_batch, 2),
        2,
        "a later batch does not reopen the step"
    );
}

#[test]
fn the_layout_keeps_steps_and_bar_left_of_the_mascot() {
    let t0 = Instant::now();
    let mut steps = Steps::default();
    steps.begin("Starting the IDE", t0);
    steps.begin("Preparing graphics", t0 + Duration::from_millis(500));
    let frame = super::layout(
        &steps,
        ShaderCount {
            done: 10,
            total: 100,
            compiled: 10,
        },
    );
    let texts: Vec<&str> = frame.texts.iter().map(|t| t.text.as_str()).collect();
    assert!(texts.contains(&"Preparing graphics  10 / 100"));
    assert!(texts.contains(&"0.5 s"));
    assert!(frame.fills.iter().all(|f| f.area[2] <= super::COLUMN));
    assert!(
        frame
            .texts
            .iter()
            .skip(3)
            .all(|t| t.area[2] <= super::COLUMN)
    );
}
