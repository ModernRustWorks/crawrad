//! The gate itself, plus the proof it can go red.

use super::{MAX_RUNTIME_BORDER_SIZE_WRITES, runtime_writes, scan};
use std::path::{Path, PathBuf};

fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// **No new runtime write of `border_size`** — and the count only goes down.
#[test]
fn border_size_is_not_written_at_run_time() {
    let found = scan(&crates_dir());
    let n = found.len();
    match n.cmp(&MAX_RUNTIME_BORDER_SIZE_WRITES) {
        std::cmp::Ordering::Greater => panic!(
            "`border_size` written at run time draws nothing on a plain `View` (it has              no such field: the app logs `no matching field`), and the source does not              show which widget a `live!` block lands on. Declare the border in the DSL              and switch an INSTANCE value instead (a colour).              Found {n} (max {MAX_RUNTIME_BORDER_SIZE_WRITES}):
  {}",
            found.join("
  ")
        ),
        std::cmp::Ordering::Less => panic!(
            "the runtime writes went down to {n}: lower MAX_RUNTIME_BORDER_SIZE_WRITES              to {n} in uniform_write_gate.rs in this PR"
        ),
        std::cmp::Ordering::Equal => {}
    }
}

/// **The detector tells a runtime write from the static DSL** — and it sees the
/// two shapes that shipped: the old ListView's literal and `look::border`'s value.
#[test]
fn a_runtime_write_is_told_from_the_dsl() {
    let literal = "cell.apply_over(cx, live! { draw_bg: { border_size: 1.0 } });";
    let value = "w.apply_over(\n  cx,\n  live! { draw_bg: { border_size: (w) } },\n);";
    assert_eq!(runtime_writes(literal), vec![1]);
    assert_eq!(runtime_writes(value), vec![3]);
    let dsl = concat!(
        "live_",
        "design! { X = <View> { draw_bg: { border_size: 1.0 } } }"
    );
    assert!(
        runtime_writes(dsl).is_empty(),
        "the static DSL is where it belongs"
    );
    let other = "live! { draw_bg: { line_color: (c) } }";
    assert!(runtime_writes(other).is_empty());
}
