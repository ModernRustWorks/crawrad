//! The engine: answers by ticket, and only the newest of a pile is asked.

use super::Asker;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

fn wait<T: Send + 'static>(a: &Asker<T>, ticket: u64) -> T {
    let started = Instant::now();
    loop {
        if let Some(x) = a.take(ticket) {
            return x;
        }
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "ticket {ticket}"
        );
        std::thread::sleep(Duration::from_millis(5));
    }
}

/// **An answer comes back under its own ticket.**
#[test]
fn an_answer_comes_back_under_its_ticket() {
    let a = Asker::new("test-asker");
    a.ask(7, || "seven");
    assert_eq!(wait(&a, 7), "seven");
    assert_eq!(a.take(7), None, "once");
}

/// **A pile of requests while one is out: only the newest is asked; an
/// older ticket gets nothing.**
#[test]
fn only_the_newest_of_a_pile_is_asked() {
    let a = Asker::new("test-asker");
    let ran = Arc::new(AtomicUsize::new(0));
    let (release, gate) = channel::<()>();
    // the first holds the thread until released
    a.ask(1, move || {
        let _ = gate.recv();
        1
    });
    std::thread::sleep(Duration::from_millis(50));
    for t in 2..=5 {
        let ran = Arc::clone(&ran);
        a.ask(t, move || {
            ran.fetch_add(1, Ordering::SeqCst);
            t
        });
    }
    release.send(()).unwrap();
    assert_eq!(wait(&a, 5), 5);
    assert_eq!(ran.load(Ordering::SeqCst), 1, "2, 3 and 4 were never asked");
    assert_eq!(
        a.take(1),
        None,
        "an older answer is dropped by the newest take"
    );
}
