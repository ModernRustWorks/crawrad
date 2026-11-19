//! The debug session as the IDE sees it: whether one exists,
//! where the program stands, and what it printed.
//!
//! The session itself runs on its own thread ([`crawrad_dap::DebugController`]);
//! this is the state the screen, the actions and the MCP read, through the
//! host (ADR 0044). The IDE pumps it once a frame with [`Debug::pump`] and
//! draws what changed.

use std::path::{Path, PathBuf};

use std::collections::BTreeMap;

use crawrad_dap::{Command, DebugController, ShownVariable, Stop, Update};
use crawrad_note::note;

#[note("debug-phase")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Phase {
    /// No session.
    #[default]
    Idle,
    /// The debugger is starting the program.
    Starting,
    Running,
    Stopped,
    /// The session is over; the text says how.
    Ended(String),
}

#[note("debug-debugevent")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DebugEvent {
    /// The program stopped: open this file at this (one-based) line.
    Stopped {
        file: Option<PathBuf>,
        line: u32,
        reason: String,
    },
    Resumed,
    /// The locals of a picked frame or the children of an opened variable
    /// arrived: the panel redraws, nothing else moves.
    Inspected,
    /// The session is over; the text is what the status bar says.
    Ended(String),
}

#[note("debug-stop-text")]
pub fn stop_text(file: &Path, line: u32, reason: &str) -> String {
    let name = file.file_name().unwrap_or_default().to_string_lossy();
    format!("stopped at {name}:{line} ({reason})")
}

#[note("debug-output-tail")]
pub const OUTPUT_TAIL: usize = 50;

#[derive(Default)]
pub struct Debug {
    controller: Option<DebugController>,
    phase: Phase,
    stop: Option<Stop>,
    output: Vec<String>,
    /// Whether the next `Run` goes under the debugger (`Run without
    /// debugging` clears it for one run).
    without_debugger_once: bool,
    /// The frame the Run and Debug panel shows: `0` is where the program
    /// stands; a click on another frame of the stack picks it.
    selected_frame: usize,
    /// The locals of the picked frame when it is not the top one; `None`
    /// while the session is still answering.
    frame_locals: Option<Vec<ShownVariable>>,
    /// The variables open in the panel (▾), by reference, with their
    /// children once the session has sent them.
    open: BTreeMap<i64, Option<Vec<ShownVariable>>>,
    /// The watched expressions. They outlive stops and sessions, as the watch list of a classic RAD IDE
    /// Watch List does.
    watches: Vec<String>,
    /// Their values in the frame the panel shows; `None` while on their way.
    watch_values: Option<Vec<ShownVariable>>,
    /// The paths asked for the editor's inline values, and their answer.
    inline_asked: Vec<String>,
    inline_answer: Option<Vec<ShownVariable>>,
}

impl Debug {
    #[note("debug-debug-start")]
    pub fn start(&mut self, controller: DebugController) {
        self.controller = Some(controller);
        self.phase = Phase::Starting;
        self.stop = None;
        self.output.clear();
        self.forget_inspection();
    }

    #[note("debug-debug-forget-inspection")]
    fn forget_inspection(&mut self) {
        self.selected_frame = 0;
        self.frame_locals = None;
        self.open.clear();
        self.watch_values = None;
        self.inline_asked.clear();
        self.inline_answer = None;
    }

    #[note("debug-debug-stop-view-in")]
    pub fn stop_view_in(
        &mut self,
        file: &Path,
        text: &str,
    ) -> (Option<usize>, Vec<(usize, String)>) {
        (
            self.execution_line_in(file),
            self.inline_values_in(file, text),
        )
    }

    #[note("debug-debug-inline-values-in")]
    pub fn inline_values_in(&mut self, file: &Path, text: &str) -> Vec<(usize, String)> {
        use crate::inline_values as iv;
        let Some(frame) = self
            .current_stop()
            .and_then(|s| s.frames.get(self.selected_frame))
        else {
            return Vec::new();
        };
        let (frame_id, line) = (frame.id, frame.line);
        let here = frame.path.as_deref().map(crate::breakpoints::file_key);
        if here != Some(crate::breakpoints::file_key(file)) || line < 1 {
            return Vec::new();
        }
        let locals: Vec<String> = self
            .shown_locals()
            .unwrap_or_default()
            .iter()
            .map(|v| v.name.clone())
            .collect();
        let names: Vec<&str> = locals.iter().map(String::as_str).collect();
        let lines = iv::paths(text, line as usize - 1, &names);
        let wanted = iv::expressions(&lines);
        if wanted != self.inline_asked {
            self.inline_asked = wanted.clone();
            self.inline_answer = None;
            if !wanted.is_empty() {
                self.send(Command::Inline {
                    frame_id,
                    expressions: wanted,
                });
            }
        }
        match &self.inline_answer {
            Some(values) => iv::texts(&lines, &self.inline_asked, values),
            None => Vec::new(),
        }
    }

    #[note("debug-debug-watches")]
    pub fn watches(&self) -> &[String] {
        &self.watches
    }

    #[note("debug-debug-watch-values")]
    pub fn watch_values(&self) -> Option<&[ShownVariable]> {
        self.watch_values.as_deref().filter(|_| self.is_stopped())
    }

    #[note("debug-debug-add-watch")]
    pub fn add_watch(&mut self, expression: &str) -> bool {
        let e = expression.trim();
        if e.is_empty() || self.watches.iter().any(|w| w == e) {
            return false;
        }
        self.watches.push(e.to_string());
        self.evaluate_watches();
        true
    }

    #[note("debug-debug-remove-watch")]
    pub fn remove_watch(&mut self, index: usize) -> Option<String> {
        (index < self.watches.len()).then(|| {
            let gone = self.watches.remove(index);
            if let Some(values) = self.watch_values.as_mut()
                && index < values.len()
            {
                values.remove(index);
            }
            gone
        })
    }

    #[note("debug-debug-evaluate-watches")]
    fn evaluate_watches(&mut self) {
        if self.watches.is_empty() {
            return;
        }
        let Some(frame) = self
            .current_stop()
            .and_then(|s| s.frames.get(self.selected_frame))
            .map(|f| f.id)
        else {
            return;
        };
        self.watch_values = None;
        let expressions = self.watches.clone();
        self.send(Command::Watch {
            frame_id: frame,
            expressions,
        });
    }

    #[note("debug-debug-selected-frame")]
    pub fn selected_frame(&self) -> usize {
        self.selected_frame
    }

    #[note("debug-debug-select-frame")]
    pub fn select_frame(&mut self, index: usize) -> Option<(PathBuf, u32)> {
        let frame = self.current_stop()?.frames.get(index)?.clone();
        if index != self.selected_frame {
            self.selected_frame = index;
            self.frame_locals = None;
            self.open.clear();
            if index != 0 {
                self.send(Command::SelectFrame(frame.id));
            }
            self.evaluate_watches();
        }
        Some((frame.path?, u32::try_from(frame.line).ok()?))
    }

    #[note("debug-debug-shown-locals")]
    pub fn shown_locals(&self) -> Option<&[ShownVariable]> {
        let stop = self.current_stop()?;
        if self.selected_frame == 0 {
            Some(&stop.locals)
        } else {
            self.frame_locals.as_deref()
        }
    }

    #[note("debug-debug-toggle-variable")]
    pub fn toggle_variable(&mut self, reference: i64) -> bool {
        if reference == 0 || !self.is_stopped() {
            return false;
        }
        if self.open.remove(&reference).is_some() {
            return false;
        }
        self.open.insert(reference, None);
        self.send(Command::Expand(reference));
        true
    }

    #[note("debug-debug-children")]
    pub fn children(&self, reference: i64) -> Option<Option<&[ShownVariable]>> {
        self.open.get(&reference).map(|c| c.as_deref())
    }

    pub fn phase(&self) -> &Phase {
        &self.phase
    }

    #[note("debug-debug-is-active")]
    pub fn is_active(&self) -> bool {
        matches!(
            self.phase,
            Phase::Starting | Phase::Running | Phase::Stopped
        )
    }

    pub fn is_stopped(&self) -> bool {
        self.phase == Phase::Stopped
    }

    #[note("debug-debug-current-stop")]
    pub fn current_stop(&self) -> Option<&Stop> {
        self.stop.as_ref().filter(|_| self.is_stopped())
    }

    #[note("debug-debug-location")]
    pub fn location(&self) -> Option<(&Path, u32)> {
        let top = self.current_stop()?.frames.first()?;
        Some((top.path.as_deref()?, u32::try_from(top.line).ok()?))
    }

    #[note("debug-debug-execution-line-in")]
    pub fn execution_line_in(&self, file: &Path) -> Option<usize> {
        let (here, line) = self.location()?;
        let same = crate::breakpoints::file_key(here) == crate::breakpoints::file_key(file);
        same.then(|| (line as usize).saturating_sub(1))
    }

    #[note("debug-debug-output")]
    pub fn output(&self) -> &[String] {
        &self.output
    }

    #[note("debug-debug-snapshot")]
    pub fn snapshot(&self) -> serde_json::Value {
        use serde_json::json;
        let phase = match &self.phase {
            Phase::Idle => json!("idle"),
            Phase::Starting => json!("starting"),
            Phase::Running => json!("running"),
            Phase::Stopped => json!("stopped"),
            Phase::Ended(how) => json!({ "ended": how }),
        };
        let stop = self.current_stop().map(|stop| {
            json!({
                "reason": stop.reason,
                "frames": stop.frames.iter().map(|f| json!({
                    "name": f.name,
                    "file": f.path.as_ref().map(|p| p.display().to_string()),
                    "line": f.line,
                })).collect::<Vec<_>>(),
                "locals": stop.locals.iter().map(|v| json!({
                    "name": v.name,
                    "value": v.value,
                    "type": v.type_name,
                    "expandable": v.reference != 0,
                })).collect::<Vec<_>>(),
            })
        });
        let tail = self.output.len().saturating_sub(OUTPUT_TAIL);
        json!({
            "phase": phase,
            "stop": stop,
            "output": self.output[tail..].concat(),
        })
    }

    #[note("debug-debug-send")]
    pub fn send(&mut self, command: Command) -> bool {
        let resumes = matches!(
            command,
            Command::Continue | Command::StepOver | Command::StepInto | Command::StepOut
        );
        let sent = self.is_active() && self.controller.as_ref().is_some_and(|c| c.send(command));
        if sent && resumes && self.is_stopped() {
            self.phase = Phase::Running;
            self.stop = None;
        }
        sent
    }

    #[note("debug-debug-run-without-debugger-once")]
    pub fn run_without_debugger_once(&mut self) {
        self.without_debugger_once = true;
    }

    pub fn take_run_without_debugger(&mut self) -> bool {
        std::mem::take(&mut self.without_debugger_once)
    }

    #[note("debug-debug-pump")]
    pub fn pump(&mut self) -> Vec<DebugEvent> {
        let Some(controller) = &self.controller else {
            return Vec::new();
        };
        let mut events = Vec::new();
        for update in controller.take_updates() {
            if let Some(event) = self.apply(update) {
                events.push(event);
            }
        }
        if matches!(self.phase, Phase::Ended(_)) {
            // The thread is done; dropping the handle joins it at once.
            self.controller = None;
        }
        events
    }

    pub(crate) fn apply(&mut self, update: Update) -> Option<DebugEvent> {
        match update {
            Update::Started => {
                self.phase = Phase::Running;
                None
            }
            Update::Running => {
                self.phase = Phase::Running;
                self.stop = None;
                self.forget_inspection();
                Some(DebugEvent::Resumed)
            }
            Update::Stopped(stop) => {
                let top = stop.frames.first();
                let event = DebugEvent::Stopped {
                    file: top.and_then(|f| f.path.clone()),
                    line: top.and_then(|f| u32::try_from(f.line).ok()).unwrap_or(0),
                    reason: stop.reason.clone(),
                };
                self.phase = Phase::Stopped;
                self.stop = Some(stop);
                self.forget_inspection();
                self.evaluate_watches();
                Some(event)
            }
            Update::Output(text) => {
                self.output.push(text);
                None
            }
            Update::Terminated { exit_code } => {
                let text = match exit_code {
                    Some(code) => format!("the program ended (exit code {code})"),
                    None => "the program ended".to_string(),
                };
                self.end(text)
            }
            Update::Failed(why) => self.end(why),
            Update::Locals { frame_id, locals } => {
                let picked = self
                    .current_stop()
                    .and_then(|s| s.frames.get(self.selected_frame))
                    .map(|f| f.id);
                if picked == Some(frame_id) && self.selected_frame != 0 {
                    self.frame_locals = Some(locals);
                    return Some(DebugEvent::Inspected);
                }
                None
            }
            Update::Children {
                reference,
                children,
            } => {
                let slot = self.open.get_mut(&reference)?;
                *slot = Some(children);
                Some(DebugEvent::Inspected)
            }
            Update::Watches { frame_id, values } => {
                let picked = self
                    .current_stop()
                    .and_then(|s| s.frames.get(self.selected_frame))
                    .map(|f| f.id);
                // An answer for another frame, or for a list that changed
                // since it was asked, is stale.
                if picked != Some(frame_id) || values.len() != self.watches.len() {
                    return None;
                }
                self.watch_values = Some(values);
                Some(DebugEvent::Inspected)
            }
            Update::Inline { frame_id, values } => {
                let picked = self
                    .current_stop()
                    .and_then(|s| s.frames.get(self.selected_frame))
                    .map(|f| f.id);
                // An answer for another frame, or for paths that changed since
                // they were asked, is stale.
                if picked != Some(frame_id) || values.len() != self.inline_asked.len() {
                    return None;
                }
                self.inline_answer = Some(values);
                Some(DebugEvent::Inspected)
            }
            Update::Breakpoints(_) => None,
        }
    }

    fn end(&mut self, text: String) -> Option<DebugEvent> {
        self.phase = Phase::Ended(text.clone());
        self.stop = None;
        self.forget_inspection();
        Some(DebugEvent::Ended(text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crawrad_dap::session::StackFrame;

    fn stop_at(line: i64) -> Stop {
        Stop {
            thread_id: 1,
            reason: "breakpoint".into(),
            frames: vec![StackFrame {
                id: 9,
                name: "FormMain::btn_ok_click".into(),
                path: Some(PathBuf::from("C:/app/src/form_main.rs")),
                line,
            }],
            locals: Vec::new(),
        }
    }

    #[test]
    fn a_stop_gives_the_place_to_show_and_a_resume_takes_it_away() {
        let mut debug = Debug::default();
        assert_eq!(debug.apply(Update::Started), None);
        assert_eq!(debug.phase(), &Phase::Running);
        assert_eq!(
            debug.apply(Update::Stopped(stop_at(12))),
            Some(DebugEvent::Stopped {
                file: Some(PathBuf::from("C:/app/src/form_main.rs")),
                line: 12,
                reason: "breakpoint".into(),
            })
        );
        assert_eq!(
            debug.location(),
            Some((Path::new("C:/app/src/form_main.rs"), 12))
        );
        assert_eq!(debug.apply(Update::Running), Some(DebugEvent::Resumed));
        assert_eq!(debug.location(), None);
        debug.apply(Update::Output("hello\n".into()));
        assert_eq!(debug.output(), ["hello\n".to_string()]);
    }

    #[test]
    fn the_end_says_how_and_the_session_is_no_longer_active() {
        let mut debug = Debug::default();
        debug.apply(Update::Stopped(stop_at(3)));
        assert_eq!(
            debug.apply(Update::Terminated { exit_code: Some(0) }),
            Some(DebugEvent::Ended("the program ended (exit code 0)".into()))
        );
        assert!(!debug.is_active());
        assert_eq!(debug.location(), None);
        // With no session, a command goes nowhere.
        assert!(!debug.send(Command::Continue));
    }

    #[test]
    fn a_debugger_that_fails_to_start_ends_the_session_with_its_reason() {
        let mut debug = Debug::default();
        debug.start(DebugController::start(
            || Err(crawrad_dap::DapError::Closed),
            Box::new(crawrad_dap::AdapterText),
            || {},
        ));
        assert!(debug.is_active());
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        let mut events = Vec::new();
        while events.is_empty() && std::time::Instant::now() < deadline {
            events = debug.pump();
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        let Some(DebugEvent::Ended(why)) = events.last() else {
            panic!("expected the session to end, got {events:?}");
        };
        assert!(why.starts_with("the debugger did not start"), "{why}");
        assert!(!debug.is_active());
    }

    #[test]
    fn a_step_takes_the_stop_away_before_the_session_answers() {
        // A session that is still starting keeps its command channel open and
        // answers nothing: whatever `debug.state` reads right after the step
        // comes from the step itself, not from the session.
        //
        // The session is held "starting" by a channel the test owns, and not
        // by a sleep: under load (the Linux gate in docker) a 300 ms sleep ran
        // out before the step was sent, the session had already ended, and
        // `send` answered false.
        let (release, held) = std::sync::mpsc::channel::<()>();
        let mut debug = Debug::default();
        debug.start(DebugController::start(
            move || {
                let _ = held.recv();
                Err(crawrad_dap::DapError::Closed)
            },
            Box::new(crawrad_dap::AdapterText),
            || {},
        ));
        debug.apply(Update::Stopped(stop_at(12)));
        assert!(debug.send(Command::StepOver));
        assert_eq!(debug.phase(), &Phase::Running);
        assert_eq!(debug.location(), None);
        assert_eq!(debug.snapshot()["stop"], serde_json::Value::Null);
        // the session may end now
        drop(release);
    }

    #[test]
    fn a_breakpoint_change_leaves_the_stop_where_it_is() {
        // held "starting" by a channel, not a sleep (see the test above)
        let (release, held) = std::sync::mpsc::channel::<()>();
        let mut debug = Debug::default();
        debug.start(DebugController::start(
            move || {
                let _ = held.recv();
                Err(crawrad_dap::DapError::Closed)
            },
            Box::new(crawrad_dap::AdapterText),
            || {},
        ));
        debug.apply(Update::Stopped(stop_at(12)));
        assert!(debug.send(Command::SetBreakpoints(
            PathBuf::from("C:/app/src/form_main.rs"),
            vec![3.into()]
        )));
        assert_eq!(debug.phase(), &Phase::Stopped);
        assert_eq!(
            debug.location(),
            Some((Path::new("C:/app/src/form_main.rs"), 12))
        );
        drop(release);
    }

    #[test]
    fn run_without_debugging_is_asked_once_and_answered_once() {
        let mut debug = Debug::default();
        assert!(!debug.take_run_without_debugger());
        debug.run_without_debugger_once();
        assert!(debug.take_run_without_debugger());
        assert!(!debug.take_run_without_debugger());
    }
}
