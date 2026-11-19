//! A debug session on its own thread, driven by commands and read by updates.
//!
//! The IDE must never wait on the debugger: launching takes seconds, a
//! `continue` may not stop for minutes, and the window keeps drawing all the
//! while. So the [`DebugSession`] lives on a thread of its own. The IDE sends
//! [`Command`]s and, once a frame, takes the [`Update`]s that arrived — the
//! same shape as the build, which runs on a thread and is polled
//! (`crawrad_os::process`).
//!
//! While the program runs, the thread waits for a stop in short slices
//! ([`SLICE`]), so a `Stop` or `Pause` sent meanwhile is acted on within one
//! slice. The step-again logic of [`DebugSession::wait_for_stop`] survives the
//! slicing: the step in flight is kept until a stop is reached.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};
use std::thread::JoinHandle;
use std::time::Duration;

use crate::connection::DapError;
use crate::presenter::ValuePresenter;
use crate::session::{
    Breakpoint, DebugSession, LaunchConfig, SessionState, SourceBreakpoint, StackFrame,
};
use crawrad_note::note;

#[note("controller-slice")]
pub const SLICE: Duration = Duration::from_millis(100);

#[note("controller-stack-depth")]
pub const STACK_DEPTH: i64 = 64;

#[note("controller-command")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Continue,
    StepOver,
    StepInto,
    StepOut,
    Pause,
    /// Ends the program and the session.
    Stop,
    /// Replaces the breakpoints of one file (one-based lines).
    SetBreakpoints(PathBuf, Vec<SourceBreakpoint>),
    /// The locals of another frame of the current stop (a click on the stack).
    SelectFrame(i64),
    /// The children of a variable (a click on its arrow).
    Expand(i64),
    /// The watches, evaluated in one frame (the panel's Watch section).
    Watch {
        frame_id: i64,
        expressions: Vec<String>,
    },
    /// The paths the editor writes after the lines around the stop.
    Inline {
        frame_id: i64,
        expressions: Vec<String>,
    },
}

#[note("controller-shownvariable")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShownVariable {
    pub name: String,
    pub value: String,
    pub type_name: Option<String>,
    pub reference: i64,
}

#[note("controller-stop")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stop {
    pub thread_id: i64,
    /// `breakpoint`, `step`, `pause`, `exception`…, as the adapter says it.
    pub reason: String,
    /// Innermost first: the first frame is where the program stands.
    pub frames: Vec<StackFrame>,
    /// The locals of the first frame.
    pub locals: Vec<ShownVariable>,
}

#[note("controller-update")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Update {
    /// The handshake is done and the program runs.
    Started,
    /// Where the adapter put each breakpoint.
    Breakpoints(BTreeMap<PathBuf, Vec<Breakpoint>>),
    Running,
    Stopped(Stop),
    /// The locals of the frame asked by [`Command::SelectFrame`].
    Locals {
        frame_id: i64,
        locals: Vec<ShownVariable>,
    },
    /// The children asked by [`Command::Expand`].
    Children {
        reference: i64,
        children: Vec<ShownVariable>,
    },
    /// The paths asked by [`Command::Inline`], in its order.
    Inline {
        frame_id: i64,
        values: Vec<ShownVariable>,
    },
    /// The watches asked by [`Command::Watch`], in its order.
    Watches {
        frame_id: i64,
        values: Vec<ShownVariable>,
    },
    /// A piece of the program's output (or the adapter's console).
    Output(String),
    /// The program ended; the session is over.
    Terminated {
        exit_code: Option<i64>,
    },
    /// The session could not go on; it is over.
    Failed(String),
}

#[note("controller-debugcontroller")]
pub struct DebugController {
    commands: Sender<Command>,
    updates: Receiver<Update>,
    thread: Option<JoinHandle<()>>,
}

#[note("controller-updatesink")]
struct UpdateSink<'a> {
    tx: &'a Sender<Update>,
    waker: fn(),
}

impl<'a> UpdateSink<'a> {
    fn send(&self, update: Update) {
        let _ = self.tx.send(update);
        (self.waker)();
    }
}

impl DebugController {
    #[note("controller-debugcontroller-start")]
    pub fn start<L>(launch: L, presenter: Box<dyn ValuePresenter + Send>, waker: fn()) -> Self
    where
        L: FnOnce() -> Result<DebugSession, DapError> + Send + 'static,
    {
        let (commands, command_rx) = channel();
        let (update_tx, updates) = channel();
        let failure = update_tx.clone();
        let thread = std::thread::Builder::new()
            .name("crawrad-debug-session".into())
            .spawn(move || run(launch, presenter.as_ref(), &command_rx, &update_tx, waker))
            .ok();
        if thread.is_none() {
            let _ = failure.send(Update::Failed(
                "the debug thread could not be started".into(),
            ));
            waker();
        }
        Self {
            commands,
            updates,
            thread,
        }
    }

    #[note("controller-debugcontroller-launch-process")]
    pub fn launch_process<B: Clone + Into<SourceBreakpoint> + Send + 'static>(
        adapter: PathBuf,
        config: LaunchConfig,
        breakpoints: BTreeMap<PathBuf, Vec<B>>,
        presenter: Box<dyn ValuePresenter + Send>,
        waker: fn(),
    ) -> Self {
        Self::start(
            move || DebugSession::launch(&adapter, &config, &breakpoints),
            presenter,
            waker,
        )
    }

    #[note("controller-debugcontroller-send")]
    pub fn send(&self, command: Command) -> bool {
        self.commands.send(command).is_ok()
    }

    #[note("controller-debugcontroller-take-updates")]
    pub fn take_updates(&self) -> Vec<Update> {
        let mut out = Vec::new();
        loop {
            match self.updates.try_recv() {
                Ok(update) => out.push(update),
                Err(TryRecvError::Empty | TryRecvError::Disconnected) => return out,
            }
        }
    }
}

impl Drop for DebugController {
    #[note("controller-debugcontroller-drop")]
    fn drop(&mut self) {
        let _ = self.commands.send(Command::Stop);
        if let Some(thread) = self.thread.take() {
            // The thread answers `Stop` within one slice plus one request.
            let _ = thread.join();
        }
    }
}

fn run<L>(
    launch: L,
    presenter: &dyn ValuePresenter,
    commands: &Receiver<Command>,
    updates: &Sender<Update>,
    waker: fn(),
) where
    L: FnOnce() -> Result<DebugSession, DapError>,
{
    let updates = UpdateSink { tx: updates, waker };
    let mut session = match launch() {
        Ok(session) => session,
        Err(e) => {
            updates.send(Update::Failed(format!("the debugger did not start: {e}")));
            return;
        }
    };
    updates.send(Update::Started);
    updates.send(Update::Breakpoints(session.breakpoints().clone()));
    let mut sent_output = 0;
    loop {
        forward_output(&session, &mut sent_output, &updates);
        // COMMANDS FIRST: a Stop sent while running is not kept waiting.
        loop {
            match commands.try_recv() {
                Ok(command) => {
                    if let Some(done) = obey(&mut session, command, presenter, &updates) {
                        forward_output(&session, &mut sent_output, &updates);
                        updates.send(done);
                        return;
                    }
                }
                Err(TryRecvError::Empty) => break,
                // The IDE dropped the handle without a Stop: end the program.
                Err(TryRecvError::Disconnected) => {
                    let _ = session.terminate();
                    return;
                }
            }
        }
        match session.state().clone() {
            SessionState::Stopped { .. } | SessionState::Starting => {
                // Nothing runs: wait for the next command, without spinning.
                match commands.recv_timeout(SLICE) {
                    Ok(command) => {
                        if let Some(done) = obey(&mut session, command, presenter, &updates) {
                            forward_output(&session, &mut sent_output, &updates);
                            updates.send(done);
                            return;
                        }
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                        let _ = session.terminate();
                        return;
                    }
                }
            }
            SessionState::Terminated => {
                forward_output(&session, &mut sent_output, &updates);
                updates.send(Update::Terminated {
                    exit_code: session.exit_code(),
                });
                return;
            }
            SessionState::Running => match session.wait_for_stop(SLICE) {
                Ok(SessionState::Stopped { thread_id, reason }) => {
                    let (thread_id, reason) = (*thread_id, reason.clone());
                    forward_output(&session, &mut sent_output, &updates);
                    if reason == "breakpoint"
                        && session.has_logpoints()
                        && log_and_continue(&mut session, thread_id, presenter, &updates)
                    {
                        continue;
                    }
                    let stop = describe_stop(&session, thread_id, reason, presenter);
                    updates.send(Update::Stopped(stop));
                }
                Ok(_) => {}
                Err(DapError::Timeout(_)) => {}
                Err(DapError::Closed) => {
                    forward_output(&session, &mut sent_output, &updates);
                    updates.send(Update::Terminated {
                        exit_code: session.exit_code(),
                    });
                    return;
                }
                Err(e) => {
                    updates.send(Update::Failed(e.to_string()));
                    let _ = session.terminate();
                    return;
                }
            },
        }
    }
}

#[note("controller-obey")]
fn obey(
    session: &mut DebugSession,
    command: Command,
    presenter: &dyn ValuePresenter,
    updates: &UpdateSink<'_>,
) -> Option<Update> {
    let resumed = match command {
        Command::Continue => Some(session.continue_()),
        Command::StepOver => Some(session.step_over()),
        Command::StepInto => Some(session.step_into()),
        Command::StepOut => Some(session.step_out()),
        Command::Pause => {
            if matches!(session.state(), SessionState::Running)
                && let Err(e) = session.pause()
            {
                updates.send(Update::Output(format!("pause failed: {e}\n")));
            }
            None
        }
        Command::Stop => {
            let _ = session.terminate();
            return Some(Update::Terminated {
                exit_code: session.exit_code(),
            });
        }
        Command::SetBreakpoints(path, lines) => {
            if session.set_breakpoints(&path, &lines).is_ok() {
                updates.send(Update::Breakpoints(session.breakpoints().clone()));
            }
            None
        }
        Command::SelectFrame(frame_id) => {
            let locals = shown(session, session.locals(frame_id), frame_id, presenter);
            updates.send(Update::Locals { frame_id, locals });
            None
        }
        Command::Watch {
            frame_id,
            expressions,
        } => {
            let values = expressions
                .iter()
                .map(|e| session.watch(e, frame_id, presenter))
                .collect();
            updates.send(Update::Watches { frame_id, values });
            None
        }
        Command::Inline {
            frame_id,
            expressions,
        } => {
            let values = expressions
                .iter()
                .map(|e| session.watch(e, frame_id, presenter))
                .collect();
            updates.send(Update::Inline { frame_id, values });
            None
        }
        Command::Expand(reference) => {
            // The frame for `evaluate` is the one the stop showed; children are
            // fetched by reference and need none of their own.
            let children = shown(session, session.children(reference), 0, presenter);
            updates.send(Update::Children {
                reference,
                children,
            });
            None
        }
    };
    match resumed {
        // Stepping a running program is refused by the session without
        // asking the adapter; the IDE's buttons are disabled then anyway.
        Some(Ok(())) => {
            updates.send(Update::Running);
        }
        Some(Err(e)) => {
            updates.send(Update::Output(format!("{e}\n")));
        }
        None => {}
    }
    None
}

#[note("controller-log-and-continue")]
fn log_and_continue(
    session: &mut DebugSession,
    thread_id: i64,
    presenter: &dyn ValuePresenter,
    updates: &UpdateSink<'_>,
) -> bool {
    let Some(top) = session
        .stack(thread_id, 1)
        .ok()
        .and_then(|f| f.into_iter().next())
    else {
        return false;
    };
    let Some(template) = top
        .path
        .as_deref()
        .and_then(|p| session.logpoint_at(p, top.line))
        .map(str::to_string)
    else {
        return false;
    };
    let line = render_log_message(&template, |expr| {
        session.watch(expr, top.id, presenter).value
    });
    updates.send(Update::Output(format!("{line}\n")));
    if let Err(e) = session.continue_() {
        updates.send(Update::Output(format!("{e}\n")));
    }
    true
}

#[note("controller-render-log-message")]
pub fn render_log_message(template: &str, mut value: impl FnMut(&str) -> String) -> String {
    let mut out = String::new();
    let mut rest = template;
    while let Some(i) = rest.find(['{', '\\']) {
        out.push_str(&rest[..i]);
        rest = &rest[i..];
        if let Some(after) = rest.strip_prefix("\\{") {
            out.push('{');
            rest = after;
        } else if let Some(after) = rest.strip_prefix('{') {
            match after.find('}') {
                Some(end) => {
                    out.push_str(&value(after[..end].trim()));
                    rest = &after[end + 1..];
                }
                None => {
                    out.push_str(rest);
                    rest = "";
                }
            }
        } else {
            out.push('\\');
            rest = &rest[1..];
        }
    }
    out.push_str(rest);
    out
}

fn describe_stop(
    session: &DebugSession,
    thread_id: i64,
    reason: String,
    presenter: &dyn ValuePresenter,
) -> Stop {
    let frames = session.stack(thread_id, STACK_DEPTH).unwrap_or_default();
    let locals = match frames.first() {
        Some(top) => shown(session, session.locals(top.id), top.id, presenter),
        None => Vec::new(),
    };
    Stop {
        thread_id,
        reason,
        frames,
        locals,
    }
}

fn shown(
    session: &DebugSession,
    variables: Result<Vec<crate::protocol::Variable>, DapError>,
    frame_id: i64,
    presenter: &dyn ValuePresenter,
) -> Vec<ShownVariable> {
    variables
        .unwrap_or_default()
        .iter()
        .map(|v| ShownVariable {
            name: v.name.clone(),
            value: session.present(v, frame_id, presenter),
            type_name: v.type_name.clone(),
            reference: v.variables_reference,
        })
        .collect()
}

fn forward_output(session: &DebugSession, sent: &mut usize, updates: &UpdateSink<'_>) {
    let output = session.output();
    for piece in output.iter().skip(*sent) {
        updates.send(Update::Output(piece.clone()));
    }
    *sent = output.len();
}
