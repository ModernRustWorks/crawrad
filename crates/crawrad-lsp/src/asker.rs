//! **One request at a time, off the window's thread, the newest only** —
//! the engine under Go to Definition, Hover and Format Document.
//!
//! The editor hands over a ticket and the work (a closure that asks the
//! server); one thread runs it. Requests that pile up while one is out are
//! not all asked: only the newest is, because the user moved on. An answer is
//! taken by its ticket; any other is dropped.

use crawrad_note::note;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};

type Work<T> = Box<dyn FnOnce() -> T + Send>;
pub(crate) type Waker = Arc<Mutex<Option<Box<dyn Fn() + Send>>>>;

pub struct Asker<T> {
    jobs: Sender<(u64, Work<T>)>,
    answers: Receiver<(u64, T)>,
    waker: Waker,
}

impl<T: Send + 'static> Asker<T> {
    #[note("asker-asker-new")]
    pub fn new(name: &str) -> Self {
        let (jobs, job_rx) = channel::<(u64, Work<T>)>();
        let (answer_tx, answers) = channel();
        let waker: Waker = Arc::default();
        let wake = Arc::clone(&waker);
        let _ = std::thread::Builder::new()
            .name(name.into())
            .spawn(move || serve(&job_rx, &answer_tx, &wake));
        Self {
            jobs,
            answers,
            waker,
        }
    }

    #[note("asker-asker-set-waker")]
    pub fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        if let Ok(mut w) = self.waker.lock() {
            *w = Some(Box::new(wake));
        }
    }

    #[note("asker-asker-ask")]
    pub fn ask(&self, ticket: u64, work: impl FnOnce() -> T + Send + 'static) -> bool {
        self.jobs.send((ticket, Box::new(work))).is_ok()
    }

    #[note("asker-asker-take")]
    pub fn take(&self, ticket: u64) -> Option<T> {
        self.answers
            .try_iter()
            .filter(|(t, _)| *t == ticket)
            .map(|(_, a)| a)
            .last()
    }
}

fn serve<T>(jobs: &Receiver<(u64, Work<T>)>, answers: &Sender<(u64, T)>, waker: &Waker) {
    while let Ok(mut job) = jobs.recv() {
        // Only the newest request is worth asking.
        if let Some(newer) = jobs.try_iter().last() {
            job = newer;
        }
        let (ticket, work) = job;
        if answers.send((ticket, work())).is_err() {
            return;
        }
        if let Some(wake) = waker.lock().ok().as_deref().and_then(Option::as_ref) {
            wake();
        }
    }
}

#[cfg(test)]
#[path = "test/asker/test.rs"]
mod tests;
