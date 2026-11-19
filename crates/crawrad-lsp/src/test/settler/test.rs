//! The engine: one job at a time, settled and cancelled when superseded.

use super::{Settle, Settler};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{RecvTimeoutError, channel};
use std::time::{Duration, Instant};

fn wait_for_wakes(wakes: &AtomicUsize, expected: usize) {
    let t = Instant::now();
    while wakes.load(Ordering::SeqCst) < expected && t.elapsed() < Duration::from_secs(2) {
        std::thread::sleep(Duration::from_millis(1));
    }
}

/// A burst of 5 jobs sent within the quiet time yields ONE answer, for the last job.
#[test]
fn burst_within_quiet_time_yields_one_answer_for_last_job() {
    let settler = Settler::new(
        "test-burst",
        Settle::Quiet(Duration::from_millis(200)),
        |job: i32, _| Ok(job * 10),
    );
    for i in 1..=5 {
        assert!(settler.send(i));
    }
    let answer = settler
        .answers()
        .recv_timeout(Duration::from_secs(2))
        .expect("received answer");
    assert_eq!(answer, 50);
    assert!(
        settler
            .answers()
            .recv_timeout(Duration::from_millis(100))
            .is_err()
    );
}

/// `Err(Some(newer))` from `work` runs `newer` next and answers it.
#[test]
fn err_some_newer_runs_newer_next_and_answers_it() {
    let (started_tx, started_rx) = channel::<()>();
    let runs = Arc::new(AtomicUsize::new(0));
    let r = Arc::clone(&runs);
    let settler = Settler::new("test-superseded", Settle::Newest, move |job: i32, jobs| {
        r.fetch_add(1, Ordering::SeqCst);
        if job == 1 {
            let _ = started_tx.send(());
            let started = Instant::now();
            loop {
                if let Some(newer) = jobs.try_iter().last() {
                    return Err(Some(newer));
                }
                assert!(started.elapsed() < Duration::from_secs(2));
                std::thread::sleep(Duration::from_millis(1));
            }
        }
        Ok(job * 10)
    });
    assert!(settler.send(1));
    started_rx
        .recv_timeout(Duration::from_secs(2))
        .expect("work(1) started");
    assert!(settler.send(2));
    let answer = settler
        .answers()
        .recv_timeout(Duration::from_secs(2))
        .expect("received answer");
    assert_eq!(answer, 20);
    assert_eq!(runs.load(Ordering::SeqCst), 2);
    assert!(
        settler
            .answers()
            .recv_timeout(Duration::from_millis(100))
            .is_err()
    );
}

/// `Err(None)` answers nothing and the next sent job is answered.
#[test]
fn err_none_answers_nothing_and_next_sent_job_is_answered() {
    let settler = Settler::new("test-err-none", Settle::Newest, |job: i32, _| {
        if job == 1 {
            return Err(None);
        }
        Ok(job * 10)
    });
    assert!(settler.send(1));
    assert!(
        settler
            .answers()
            .recv_timeout(Duration::from_millis(100))
            .is_err()
    );

    assert!(settler.send(2));
    let answer = settler
        .answers()
        .recv_timeout(Duration::from_secs(2))
        .expect("received answer");
    assert_eq!(answer, 20);
}

/// `Settle::Newest` drains: jobs queued while one is in `work` are drained to the newest.
#[test]
fn settle_newest_drains_queued_jobs() {
    let (started_tx, started_rx) = channel::<()>();
    let (release_tx, release_rx) = channel::<()>();
    let runs = Arc::new(AtomicUsize::new(0));
    let r = Arc::clone(&runs);
    let settler = Settler::new("test-drains", Settle::Newest, move |job: i32, _| {
        r.fetch_add(1, Ordering::SeqCst);
        if job == 1 {
            let _ = started_tx.send(());
            let _ = release_rx.recv();
        }
        Ok(job * 10)
    });

    assert!(settler.send(1));
    started_rx
        .recv_timeout(Duration::from_secs(2))
        .expect("work(1) started");

    // While job 1 is inside work, queue jobs 2 and 3
    assert!(settler.send(2));
    assert!(settler.send(3));

    // Release job 1
    assert!(release_tx.send(()).is_ok());

    let first = settler
        .answers()
        .recv_timeout(Duration::from_secs(2))
        .expect("first answer");
    assert_eq!(first, 10);

    let second = settler
        .answers()
        .recv_timeout(Duration::from_secs(2))
        .expect("second answer");
    assert_eq!(second, 30);

    // Job 2 never ran: exactly 2 runs (job 1 and job 3)
    assert_eq!(runs.load(Ordering::SeqCst), 2);
    assert!(
        settler
            .answers()
            .recv_timeout(Duration::from_millis(100))
            .is_err()
    );
}

/// Dropping the `Settler` ends its thread and drops `work`.
#[test]
fn dropping_settler_ends_its_thread() {
    let (sentinel_tx, sentinel_rx) = channel::<()>();
    let settler = Settler::new("test-drop", Settle::Newest, move |job: i32, _| {
        let _ = &sentinel_tx;
        Ok(job * 10)
    });
    // Worker is waiting on jobs.recv(). Drop settler to close the channel.
    drop(settler);
    assert_eq!(
        sentinel_rx.recv_timeout(Duration::from_secs(2)),
        Err(RecvTimeoutError::Disconnected)
    );
}

/// The waker is called once per answer.
#[test]
fn waker_is_called_once_per_answer() {
    let wakes = Arc::new(AtomicUsize::new(0));
    let settler = Settler::new("test-waker", Settle::Newest, |job: i32, _| {
        if job == 1 {
            return Err(None);
        }
        Ok(job * 10)
    });
    let w = Arc::clone(&wakes);
    settler.set_waker(move || {
        w.fetch_add(1, Ordering::SeqCst);
    });

    // Job 1 returns Err(None): no answer, no wake
    assert!(settler.send(1));
    assert!(
        settler
            .answers()
            .recv_timeout(Duration::from_millis(100))
            .is_err()
    );
    assert_eq!(wakes.load(Ordering::SeqCst), 0);

    // Job 2 returns Ok(20): 1 answer, 1 wake
    assert!(settler.send(2));
    assert_eq!(
        settler
            .answers()
            .recv_timeout(Duration::from_secs(2))
            .unwrap(),
        20
    );
    wait_for_wakes(&wakes, 1);
    assert_eq!(wakes.load(Ordering::SeqCst), 1);

    // Job 3 returns Ok(30): 2nd answer, 2nd wake
    assert!(settler.send(3));
    assert_eq!(
        settler
            .answers()
            .recv_timeout(Duration::from_secs(2))
            .unwrap(),
        30
    );
    wait_for_wakes(&wakes, 2);
    assert_eq!(wakes.load(Ordering::SeqCst), 2);
}
