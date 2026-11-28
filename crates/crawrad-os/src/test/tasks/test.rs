use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

// The lock is NOT this file's own, and that was the bug: `watch/test.rs` wakes
// the same global counter from a `notify` thread. See `test/serial/mod.rs`.
use crate::serial_lock;

fn serial() -> std::sync::MutexGuard<'static, ()> {
    let guard = serial_lock::serial();
    // Installed here, not in the one test that reads the count: `set_waker` holds
    // a `OnceLock`, so whichever test the runner starts with installs it for the
    // whole process. Counting from the first test is what lets `wait_for_events`
    // below guarantee no test leaves a wake behind it.
    set_waker(counting_waker);
    guard
}

fn wait_for_events<T: Send + 'static>(
    mgr: &mut TaskManager<T>,
    expected_count: usize,
) -> Vec<(TaskId, TaskEvent<T>)> {
    let start = Instant::now();
    let mut collected = Vec::new();
    while collected.len() < expected_count && start.elapsed() < Duration::from_secs(5) {
        collected.extend(mgr.drain());
        if collected.len() >= expected_count {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    collected
}

#[test]
fn progress_then_done_arrive_in_order_and_task_leaves_running() {
    let _serial = serial();
    let mut mgr = TaskManager::new();
    let id = mgr.spawn("fake", |reporter| {
        reporter.progress("step 1");
        reporter.done("success");
    });
    assert!(mgr.is_running("fake"));
    let events = wait_for_events(&mut mgr, 2);
    assert_eq!(events.len(), 2);
    assert_eq!(events[0], (id, TaskEvent::Progress("step 1".to_string())));
    assert_eq!(events[1], (id, TaskEvent::Done("success")));
    assert!(mgr.running().is_empty());
    assert!(!mgr.is_running("fake"));
    assert!(mgr.is_idle());
}

#[test]
fn failed_arrives_as_failed() {
    let _serial = serial();
    let mut mgr: TaskManager<()> = TaskManager::new();
    let id = mgr.spawn("fake", |reporter| {
        reporter.failed("something broke");
    });
    let events = wait_for_events(&mut mgr, 1);
    assert_eq!(events.len(), 1);
    assert_eq!(
        events[0],
        (id, TaskEvent::Failed("something broke".to_string()))
    );
    assert!(mgr.is_idle());
}

#[test]
fn job_that_returns_without_answering_yields_failed() {
    let _serial = serial();
    let mut mgr: TaskManager<()> = TaskManager::new();
    let id = mgr.spawn("fake", |_reporter| {
        // Returns without calling done or failed
    });
    let events = wait_for_events(&mut mgr, 1);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], (id, TaskEvent::Failed(NO_ANSWER.to_string())));
    assert!(mgr.is_idle());
}

#[test]
fn cancel_stops_job_looping_on_is_cancelled() {
    let _serial = serial();
    let mut mgr: TaskManager<()> = TaskManager::new();
    let id = mgr.spawn("fake", |reporter| {
        while !reporter.is_cancelled() {
            std::thread::sleep(Duration::from_millis(5));
        }
        reporter.failed("cancelled");
    });
    assert!(!mgr.cancel(TaskId(999999)));
    assert!(mgr.cancel(id));
    let events = wait_for_events(&mut mgr, 1);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], (id, TaskEvent::Failed("cancelled".to_string())));
    assert!(mgr.is_idle());
}

static WAKE_COUNT: AtomicUsize = AtomicUsize::new(0);

fn counting_waker() {
    WAKE_COUNT.fetch_add(1, Ordering::SeqCst);
}

#[test]
fn wake_is_called_on_every_event() {
    let _serial = serial();
    set_waker(counting_waker);
    let before = WAKE_COUNT.load(Ordering::SeqCst);
    let mut mgr = TaskManager::new();
    let _id = mgr.spawn("fake", |reporter| {
        reporter.progress("working");
        reporter.done(42);
    });
    let events = wait_for_events(&mut mgr, 2);
    assert_eq!(events.len(), 2);
    let after = WAKE_COUNT.load(Ordering::SeqCst);
    assert_eq!(after - before, 2);
}

/// **A file event wakes the same counter a task does.**
///
/// This is why the lock in `test/serial/mod.rs` sits above both test files instead
/// of inside this one. `watch/test.rs` starts a real `ProjectWatcher` and waits up
/// to five seconds for the OS to deliver its events; each delivery calls
/// `tasks::wake()` from a `notify` thread, which knows no lock. While this file
/// held only its own lock, such an event could land between the two reads in
/// `wake_is_called_on_every_event` and make two events look like three.
///
/// If this test ever stops seeing the file event, the shared lock has become
/// decoration and can go.
#[test]
fn wakes_from_a_file_event_reach_the_same_counter() {
    let _serial = serial();
    let root = std::env::temp_dir().join(format!(
        "crawrad_wake_counter_{}_{}",
        std::process::id(),
        WAKE_COUNT.load(Ordering::SeqCst)
    ));
    std::fs::create_dir_all(&root).unwrap();
    let watcher = crate::watch::ProjectWatcher::start(&root).expect("watcher");
    std::thread::sleep(Duration::from_millis(100));

    let before = WAKE_COUNT.load(Ordering::SeqCst);
    std::fs::write(root.join("touched.rs"), "fn main() {}").unwrap();

    let deadline = Instant::now() + Duration::from_secs(5);
    while WAKE_COUNT.load(Ordering::SeqCst) == before && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    let moved = WAKE_COUNT.load(Ordering::SeqCst) > before;

    drop(watcher);
    let _ = std::fs::remove_dir_all(&root);

    assert!(
        moved,
        "a file event did not reach the wake counter; the shared lock guards nothing"
    );
}
