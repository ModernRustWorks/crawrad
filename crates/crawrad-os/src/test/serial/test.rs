//! The one lock every test in this crate that can WAKE must hold.
//!
//! `tasks::wake()` reaches a single process-wide waker, and `set_waker` holds a
//! `OnceLock`, so once a test installs a counting waker it stays installed for
//! the life of the test binary. Any thread that wakes from then on is counted —
//! whichever test is measuring.
//!
//! **A lock private to one test file does not cover that.** `watch/test.rs`
//! starts a real `ProjectWatcher`, writes files and waits up to five seconds for
//! the OS to deliver the events; every delivery calls `tasks::wake()` from a
//! `notify` thread. Meanwhile `tasks/test.rs` held only its own file's lock while
//! asserting that two events produced exactly two wakes, and a file event landing
//! in those microseconds made it read three. That is why it failed and passed on
//! the same commit.
//!
//! So the lock lives here, above both, and every test that wakes or causes a wake
//! takes THIS one. `wakes_from_a_file_event_reach_the_same_counter` in
//! `tasks/test.rs` proves the interference is real rather than theoretical.
use std::sync::{Mutex, MutexGuard};

static SERIAL: Mutex<()> = Mutex::new(());

/// Takes the crate-wide lock, recovering it if another test panicked holding it.
pub fn serial() -> MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}
