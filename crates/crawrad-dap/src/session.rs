//! One debugging session: the DAP handshake, the state, and the commands.
//!
//! Still language-agnostic and cargo-agnostic: it receives the program to
//! launch, and values are shown through whichever [`ValuePresenter`] the
//! caller hands it. The order of the handshake is the protocol's, not ours:
//!
//! ```text
//! initialize → launch (answer held) → «initialized» → setBreakpoints… →
//! configurationDone → launch answered → «stopped» / «terminated»
//! ```
//!
//! Every event is looked at, none is dropped: an `output` that arrives while
//! the session waits for `initialized` is the program's first line, and a
//! `stopped` that arrives while it waits for something else is still a stop.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde_json::{Map, Value, json};

use crate::client::DapClient;
use crate::connection::{DapConnection, DapError};
use crate::presenter::{FrameScoped, ValuePresenter, ValueSource};
use crate::protocol::{Event, Variable};
use crawrad_note::note;

#[note("session-start-timeout")]
pub const START_TIMEOUT: Duration = Duration::from_secs(60);

#[note("session-request-timeout")]
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

#[note("session-launchconfig")]
#[derive(Debug, Clone, Default)]
pub struct LaunchConfig {
    pub program: PathBuf,
    pub cwd: Option<PathBuf>,
    pub args: Vec<String>,
    /// Adapter-specific keys, merged into the `launch` arguments.
    pub extra: Map<String, Value>,
}

impl LaunchConfig {
    fn arguments(&self) -> Value {
        let mut args = self.extra.clone();
        args.insert("program".into(), json!(self.program));
        if let Some(cwd) = &self.cwd {
            args.insert("cwd".into(), json!(cwd));
        }
        if !self.args.is_empty() {
            args.insert("args".into(), json!(self.args));
        }
        Value::Object(args)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionState {
    /// The handshake is not done yet.
    Starting,
    Running,
    Stopped {
        thread_id: i64,
        reason: String,
    },
    Terminated,
}

#[note("session-sourcebreakpoint")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceBreakpoint {
    pub line: i64,
    pub log_message: Option<String>,
}

impl From<i64> for SourceBreakpoint {
    fn from(line: i64) -> Self {
        Self {
            line,
            log_message: None,
        }
    }
}

#[note("session-breakpoint")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Breakpoint {
    pub path: PathBuf,
    pub requested_line: i64,
    /// Where the adapter actually put it — it may move to the next line with code.
    pub line: Option<i64>,
    pub verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackFrame {
    pub id: i64,
    pub name: String,
    pub path: Option<PathBuf>,
    pub line: i64,
}

enum Link {
    Process(DapClient),
    Stream(DapConnection),
}

pub struct DebugSession {
    link: Link,
    state: SessionState,
    output: Vec<String>,
    exit_code: Option<i64>,
    breakpoints: BTreeMap<PathBuf, Vec<Breakpoint>>,
    /// The logpoints' messages, by file and by the line the adapter put them on.
    logpoints: BTreeMap<PathBuf, BTreeMap<i64, String>>,
    /// The step in flight: its command and the line it left from.
    step: Option<Step>,
}

#[note("session-step")]
#[derive(Debug, Clone)]
struct Step {
    command: &'static str,
    thread_id: i64,
    origin: Place,
}

#[note("session-place")]
#[derive(Debug, Clone, PartialEq, Eq)]
struct Place {
    line: (Option<PathBuf>, i64, String),
    instruction: Option<String>,
}

impl Place {
    #[note("session-place-same-line-elsewhere")]
    fn same_line_elsewhere(&self, origin: &Place) -> bool {
        self.line == origin.line
            && self.instruction.is_some()
            && origin.instruction.is_some()
            && self.instruction != origin.instruction
    }
}

#[note("session-max-step-retries")]
const MAX_STEP_RETRIES: usize = 8;

impl DebugSession {
    #[note("session-debugsession-launch")]
    pub fn launch<B: Clone + Into<SourceBreakpoint>>(
        adapter: &Path,
        config: &LaunchConfig,
        breakpoints: &BTreeMap<PathBuf, Vec<B>>,
    ) -> Result<Self, DapError> {
        let client = DapClient::spawn(adapter, &[])?;
        Self::start(Link::Process(client), config, breakpoints)
    }

    #[note("session-debugsession-launch-over")]
    pub fn launch_over<B: Clone + Into<SourceBreakpoint>>(
        connection: DapConnection,
        config: &LaunchConfig,
        breakpoints: &BTreeMap<PathBuf, Vec<B>>,
    ) -> Result<Self, DapError> {
        Self::start(Link::Stream(connection), config, breakpoints)
    }

    fn start<B: Clone + Into<SourceBreakpoint>>(
        link: Link,
        config: &LaunchConfig,
        breakpoints: &BTreeMap<PathBuf, Vec<B>>,
    ) -> Result<Self, DapError> {
        let mut session = Self {
            link,
            state: SessionState::Starting,
            output: Vec::new(),
            exit_code: None,
            breakpoints: BTreeMap::new(),
            logpoints: BTreeMap::new(),
            step: None,
        };
        session.conn().request(
            "initialize",
            json!({
                "clientID": "crawrad",
                "clientName": "CrawRAD",
                "adapterID": "crawrad",
                "linesStartAt1": true,
                "columnsStartAt1": true,
                "pathFormat": "path",
                "supportsMemoryReferences": true,
            }),
            REQUEST_TIMEOUT,
        )?;
        let launch = session.conn().send_request("launch", config.arguments())?;
        session.pump_until(START_TIMEOUT, |e| e.event == "initialized")?;
        for (path, wanted) in breakpoints {
            let wanted: Vec<SourceBreakpoint> = wanted.iter().cloned().map(Into::into).collect();
            session.set_breakpoints(path, &wanted)?;
        }
        session
            .conn()
            .request("configurationDone", json!({}), REQUEST_TIMEOUT)?;
        launch.wait(START_TIMEOUT)?;
        if session.state == SessionState::Starting {
            session.state = SessionState::Running;
        }
        Ok(session)
    }

    fn conn(&self) -> &DapConnection {
        match &self.link {
            Link::Process(client) => client.connection(),
            Link::Stream(connection) => connection,
        }
    }

    pub fn state(&self) -> &SessionState {
        &self.state
    }

    #[note("session-debugsession-output")]
    pub fn output(&self) -> &[String] {
        &self.output
    }

    pub fn exit_code(&self) -> Option<i64> {
        self.exit_code
    }

    pub fn breakpoints(&self) -> &BTreeMap<PathBuf, Vec<Breakpoint>> {
        &self.breakpoints
    }

    #[note("session-debugsession-set-breakpoints")]
    pub fn set_breakpoints(
        &mut self,
        path: &Path,
        wanted: &[SourceBreakpoint],
    ) -> Result<Vec<Breakpoint>, DapError> {
        let body = self.conn().request(
            "setBreakpoints",
            json!({
                "source": { "path": path },
                "breakpoints": wanted.iter().map(|b| json!({ "line": b.line })).collect::<Vec<_>>(),
            }),
            REQUEST_TIMEOUT,
        )?;
        let answered = body["breakpoints"].as_array().cloned().unwrap_or_default();
        let mut messages = BTreeMap::new();
        let result: Vec<Breakpoint> = wanted
            .iter()
            .enumerate()
            .map(|(i, want)| {
                let b = answered.get(i);
                let line = b.and_then(|b| b["line"].as_i64());
                if let Some(message) = &want.log_message {
                    messages.insert(line.unwrap_or(want.line), message.clone());
                }
                Breakpoint {
                    path: path.to_path_buf(),
                    requested_line: want.line,
                    line,
                    verified: b.and_then(|b| b["verified"].as_bool()).unwrap_or(false),
                }
            })
            .collect();
        self.breakpoints.insert(path.to_path_buf(), result.clone());
        self.logpoints.insert(path.to_path_buf(), messages);
        Ok(result)
    }

    #[note("session-debugsession-has-logpoints")]
    pub fn has_logpoints(&self) -> bool {
        self.logpoints.values().any(|lines| !lines.is_empty())
    }

    #[note("session-debugsession-logpoint-at")]
    pub fn logpoint_at(&self, path: &Path, line: i64) -> Option<&str> {
        let disk = |p: &Path| std::fs::canonicalize(p).ok();
        self.logpoints
            .iter()
            .find(|(file, _)| {
                file.as_path() == path || disk(file).is_some_and(|f| Some(f) == disk(path))
            })
            .and_then(|(_, lines)| lines.get(&line))
            .map(String::as_str)
    }

    #[note("session-debugsession-pump")]
    pub fn pump(&mut self, timeout: Duration) -> Result<Option<Event>, DapError> {
        match self.conn().next_event(timeout) {
            Ok(event) => {
                self.apply(&event);
                Ok(Some(event))
            }
            Err(DapError::Timeout(_)) => Ok(None),
            Err(DapError::Closed) => {
                self.state = SessionState::Terminated;
                Err(DapError::Closed)
            }
            Err(e) => Err(e),
        }
    }

    fn apply(&mut self, event: &Event) {
        match event.event.as_str() {
            "stopped" => {
                self.state = SessionState::Stopped {
                    thread_id: event.body["threadId"].as_i64().unwrap_or(0),
                    reason: event.body["reason"].as_str().unwrap_or("").to_string(),
                }
            }
            "continued" => self.state = SessionState::Running,
            "exited" => self.exit_code = event.body["exitCode"].as_i64(),
            "terminated" => self.state = SessionState::Terminated,
            "output" => {
                if let Some(text) = event.body["output"].as_str() {
                    self.output.push(text.to_string());
                }
            }
            _ => {}
        }
    }

    fn pump_until(
        &mut self,
        timeout: Duration,
        done: impl Fn(&Event) -> bool,
    ) -> Result<(), DapError> {
        let deadline = Instant::now() + timeout;
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            if left.is_zero() {
                return Err(DapError::Timeout("waiting for the adapter".into()));
            }
            if let Some(event) = self.pump(left)?
                && done(&event)
            {
                return Ok(());
            }
        }
    }

    #[note("session-debugsession-wait-for-stop")]
    pub fn wait_for_stop(&mut self, timeout: Duration) -> Result<&SessionState, DapError> {
        let deadline = Instant::now() + timeout;
        let mut retries = 0;
        loop {
            if !matches!(
                self.state,
                SessionState::Stopped { .. } | SessionState::Terminated
            ) {
                let left = deadline.saturating_duration_since(Instant::now());
                self.pump_until(left, |e| e.event == "stopped" || e.event == "terminated")?;
            }
            let Some(step) = self.step.take() else {
                return Ok(&self.state);
            };
            let SessionState::Stopped { thread_id, reason } = &self.state else {
                return Ok(&self.state);
            };
            if reason != "breakpoint" || *thread_id != step.thread_id || retries >= MAX_STEP_RETRIES
            {
                return Ok(&self.state);
            }
            let here = self.where_is(step.thread_id)?;
            let repeat = if step.command == "continue" {
                here.same_line_elsewhere(&step.origin)
            } else {
                here.line == step.origin.line
            };
            if !repeat {
                return Ok(&self.state);
            }
            retries += 1;
            self.send_step(step.command, step.thread_id)?;
            self.step = Some(step);
        }
    }

    fn where_is(&self, thread_id: i64) -> Result<Place, DapError> {
        let body = self.conn().request(
            "stackTrace",
            json!({ "threadId": thread_id, "levels": 1 }),
            REQUEST_TIMEOUT,
        )?;
        let top = &body["stackFrames"][0];
        Ok(Place {
            line: (
                top["source"]["path"].as_str().map(PathBuf::from),
                top["line"].as_i64().unwrap_or(0),
                top["name"].as_str().unwrap_or("").to_string(),
            ),
            instruction: top["instructionPointerReference"]
                .as_str()
                .map(str::to_string),
        })
    }

    fn stopped_thread(&self) -> Result<i64, DapError> {
        match &self.state {
            SessionState::Stopped { thread_id, .. } => Ok(*thread_id),
            other => Err(DapError::Failed {
                command: "step".into(),
                message: format!("the program is not stopped ({other:?})"),
            }),
        }
    }

    fn resume(&mut self, command: &'static str) -> Result<(), DapError> {
        let thread = self.stopped_thread()?;
        self.step = Some(Step {
            command,
            thread_id: thread,
            origin: self.where_is(thread)?,
        });
        self.send_step(command, thread)
    }

    fn send_step(&mut self, command: &str, thread: i64) -> Result<(), DapError> {
        let mut args = json!({ "threadId": thread });
        // Steps go by LINE, like the classic step-over key, not by the adapter's statement.
        if command != "continue" {
            args["granularity"] = json!("line");
        }
        self.conn().request(command, args, REQUEST_TIMEOUT)?;
        self.state = SessionState::Running;
        Ok(())
    }

    pub fn continue_(&mut self) -> Result<(), DapError> {
        self.resume("continue")
    }

    pub fn step_over(&mut self) -> Result<(), DapError> {
        self.resume("next")
    }

    pub fn step_into(&mut self) -> Result<(), DapError> {
        self.resume("stepIn")
    }

    pub fn step_out(&mut self) -> Result<(), DapError> {
        self.resume("stepOut")
    }

    #[note("session-debugsession-pause")]
    pub fn pause(&mut self) -> Result<(), DapError> {
        let threads = self.conn().request("threads", json!({}), REQUEST_TIMEOUT)?;
        let thread = threads["threads"][0]["id"].as_i64().unwrap_or(1);
        self.conn()
            .request("pause", json!({ "threadId": thread }), REQUEST_TIMEOUT)?;
        Ok(())
    }

    #[note("session-debugsession-terminate")]
    pub fn terminate(&mut self) -> Result<(), DapError> {
        let result = self.conn().request(
            "disconnect",
            json!({ "terminateDebuggee": true }),
            REQUEST_TIMEOUT,
        );
        self.state = SessionState::Terminated;
        result.map(|_| ())
    }

    pub fn stack(&self, thread_id: i64, levels: i64) -> Result<Vec<StackFrame>, DapError> {
        let body = self.conn().request(
            "stackTrace",
            json!({ "threadId": thread_id, "levels": levels }),
            REQUEST_TIMEOUT,
        )?;
        Ok(body["stackFrames"]
            .as_array()
            .map(|frames| {
                frames
                    .iter()
                    .map(|f| StackFrame {
                        id: f["id"].as_i64().unwrap_or(0),
                        name: f["name"].as_str().unwrap_or("").to_string(),
                        path: f["source"]["path"].as_str().map(PathBuf::from),
                        line: f["line"].as_i64().unwrap_or(0),
                    })
                    .collect()
            })
            .unwrap_or_default())
    }

    #[note("session-debugsession-locals")]
    pub fn locals(&self, frame_id: i64) -> Result<Vec<Variable>, DapError> {
        let body =
            self.conn()
                .request("scopes", json!({ "frameId": frame_id }), REQUEST_TIMEOUT)?;
        let reference = body["scopes"][0]["variablesReference"]
            .as_i64()
            .unwrap_or(0);
        if reference == 0 {
            return Ok(Vec::new());
        }
        self.conn().children(reference)
    }

    pub fn children(&self, variables_reference: i64) -> Result<Vec<Variable>, DapError> {
        self.conn().children(variables_reference)
    }

    #[note("session-debugsession-watch")]
    pub fn watch(
        &self,
        expression: &str,
        frame_id: i64,
        presenter: &dyn ValuePresenter,
    ) -> crate::ShownVariable {
        use crate::presenter::ValueSource;
        match self.conn().evaluate(expression, Some(frame_id)) {
            Ok(v) => crate::ShownVariable {
                name: expression.to_string(),
                value: self.present(&v, frame_id, presenter),
                type_name: v.type_name.clone(),
                reference: v.variables_reference,
            },
            Err(e) => crate::ShownVariable {
                name: expression.to_string(),
                value: format!("<error: {e}>"),
                type_name: None,
                reference: 0,
            },
        }
    }

    pub fn present(
        &self,
        variable: &Variable,
        frame_id: i64,
        presenter: &dyn ValuePresenter,
    ) -> String {
        let scoped = FrameScoped {
            source: self.conn(),
            frame_id,
        };
        presenter
            .present(variable, &scoped)
            .unwrap_or_else(|| variable.value.clone())
    }
}
