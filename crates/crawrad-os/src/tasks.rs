//! Framework-free background task manager waking the UI with a signal.

use crawrad_note::note;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, OnceLock};

pub type Waker = fn();

#[note("tasks-no-answer")]
pub const NO_ANSWER: &str = "the task ended without answering";

static WAKER: OnceLock<Waker> = OnceLock::new();

#[note("tasks-set-waker")]
pub fn set_waker(w: Waker) {
    let _ = WAKER.set(w);
}

#[note("tasks-wake")]
pub fn wake() {
    if let Some(w) = WAKER.get() {
        w();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TaskId(pub u64);

#[derive(Debug, PartialEq)]
pub enum TaskEvent<T> {
    Progress(String),
    Done(T),
    Failed(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaskInfo {
    pub id: TaskId,
    pub kind: &'static str,
    pub progress: Option<String>,
}

pub struct Reporter<T> {
    id: TaskId,
    tx: Sender<(TaskId, TaskEvent<T>)>,
    cancelled: Arc<AtomicBool>,
    finished: bool,
}

impl<T> Reporter<T> {
    pub fn progress(&self, text: impl Into<String>) {
        let _ = self.tx.send((self.id, TaskEvent::Progress(text.into())));
        wake();
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    pub fn done(mut self, value: T) {
        self.finished = true;
        let _ = self.tx.send((self.id, TaskEvent::Done(value)));
        wake();
    }

    pub fn failed(mut self, error: impl Into<String>) {
        self.finished = true;
        let _ = self.tx.send((self.id, TaskEvent::Failed(error.into())));
        wake();
    }
}

impl<T> Drop for Reporter<T> {
    fn drop(&mut self) {
        if !self.finished {
            self.finished = true;
            let _ = self
                .tx
                .send((self.id, TaskEvent::Failed(NO_ANSWER.to_string())));
            wake();
        }
    }
}

pub struct TaskManager<T> {
    next: u64,
    tx: Sender<(TaskId, TaskEvent<T>)>,
    rx: Receiver<(TaskId, TaskEvent<T>)>,
    running: Vec<(TaskInfo, Arc<AtomicBool>)>,
}

impl<T: Send + 'static> Default for TaskManager<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Send + 'static> TaskManager<T> {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            next: 1,
            tx,
            rx,
            running: Vec::new(),
        }
    }

    #[note("tasks-taskmanager-spawn")]
    pub fn spawn(
        &mut self,
        kind: &'static str,
        job: impl FnOnce(Reporter<T>) + Send + 'static,
    ) -> TaskId {
        let id = TaskId(self.next);
        self.next += 1;
        let cancelled = Arc::new(AtomicBool::new(false));
        let info = TaskInfo {
            id,
            kind,
            progress: None,
        };
        self.running.push((info, Arc::clone(&cancelled)));
        let reporter = Reporter {
            id,
            tx: self.tx.clone(),
            cancelled,
            finished: false,
        };
        std::thread::spawn(move || {
            job(reporter);
        });
        id
    }

    #[note("tasks-taskmanager-drain")]
    pub fn drain(&mut self) -> Vec<(TaskId, TaskEvent<T>)> {
        let mut events = Vec::new();
        while let Ok(event) = self.rx.try_recv() {
            match &event.1 {
                TaskEvent::Progress(text) => {
                    if let Some((info, _)) =
                        self.running.iter_mut().find(|(info, _)| info.id == event.0)
                    {
                        info.progress = Some(text.clone());
                    }
                }
                TaskEvent::Done(_) | TaskEvent::Failed(_) => {
                    self.running.retain(|(info, _)| info.id != event.0);
                }
            }
            events.push(event);
        }
        events
    }

    #[note("tasks-taskmanager-cancel")]
    pub fn cancel(&mut self, id: TaskId) -> bool {
        if let Some((_, cancelled)) = self.running.iter().find(|(info, _)| info.id == id) {
            cancelled.store(true, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    pub fn running(&self) -> Vec<TaskInfo> {
        self.running.iter().map(|(info, _)| info.clone()).collect()
    }

    pub fn is_running(&self, kind: &'static str) -> bool {
        self.running.iter().any(|(info, _)| info.kind == kind)
    }

    pub fn is_idle(&self) -> bool {
        self.running.is_empty()
    }
}

#[cfg(test)]
#[path = "test/tasks/test.rs"]
mod tests;
