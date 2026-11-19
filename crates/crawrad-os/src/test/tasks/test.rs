use super::*;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

// The waker is global: every test here runs one at a time, so the wake count
// in `wake_is_called_on_every_event` is this file's own.
static SERIAL: Mutex<()> = Mutex::new(());

fn serial() -> std::sync::MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
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
