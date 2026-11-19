//! **One job at a time, after the typing settles, cancelled when a newer one arrives** —
//! the engine under the inlay hints, the occurrences, the completions and the code lenses.

use std::sync::Arc;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, channel};
use std::time::Duration;

use crate::asker::Waker;
use crawrad_note::note;

#[note("settler-settle")]
#[derive(Clone, Copy)]
pub(crate) enum Settle {
    /// Wait this long without a newer request (typing, the caret moving).
    Quiet(Duration),
    /// Only take the newest already waiting; do not wait.
    Newest,
}

#[note("settler-settler")]
pub(crate) struct Settler<J, A> {
    jobs: Sender<J>,
    answers: Receiver<A>,
    waker: Waker,
}

impl<J: Send + 'static, A: Send + 'static> Settler<J, A> {
    #[note("settler-settler-new")]
    pub(crate) fn new(
        name: &str,
        settle: Settle,
        work: impl FnMut(J, &Receiver<J>) -> Result<A, Option<J>> + Send + 'static,
    ) -> Self {
        let (jobs, job_rx) = channel::<J>();
        let (answer_tx, answers) = channel();
        let waker: Waker = Arc::default();
        let wake = Arc::clone(&waker);
        let _ = std::thread::Builder::new()
            .name(name.into())
            .spawn(move || serve(&job_rx, &answer_tx, &wake, settle, work));
        Self {
            jobs,
            answers,
            waker,
        }
    }

    #[note("settler-settler-set-waker")]
    pub(crate) fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        if let Ok(mut w) = self.waker.lock() {
            *w = Some(Box::new(wake));
        }
    }

    #[note("settler-settler-send")]
    pub(crate) fn send(&self, job: J) -> bool {
        self.jobs.send(job).is_ok()
    }

    #[note("settler-settler-answers")]
    pub(crate) fn answers(&self) -> &Receiver<A> {
        &self.answers
    }
}

fn serve<J, A>(
    jobs: &Receiver<J>,
    answers: &Sender<A>,
    waker: &Waker,
    settle: Settle,
    mut work: impl FnMut(J, &Receiver<J>) -> Result<A, Option<J>>,
) {
    let mut queued: Option<J> = None;
    loop {
        let mut job = match queued.take() {
            Some(job) => job,
            None => match jobs.recv() {
                Ok(job) => job,
                Err(_) => return,
            },
        };
        match settle {
            Settle::Quiet(duration) => loop {
                match jobs.recv_timeout(duration) {
                    Ok(newer) => job = newer,
                    Err(RecvTimeoutError::Timeout) => break,
                    Err(RecvTimeoutError::Disconnected) => return,
                }
            },
            Settle::Newest => {
                while let Ok(newer) = jobs.try_recv() {
                    job = newer;
                }
            }
        }
        let answer = match work(job, jobs) {
            Ok(answer) => answer,
            Err(newer) => {
                queued = newer;
                continue;
            }
        };
        if answers.send(answer).is_err() {
            return;
        }
        if let Some(wake) = waker.lock().ok().as_deref().and_then(Option::as_ref) {
            wake();
        }
    }
}

#[cfg(test)]
#[path = "test/settler/test.rs"]
mod tests;
