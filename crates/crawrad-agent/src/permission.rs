//! Data models and a unified queue for ACP permission control.
//!
//! Implements `session/request_permission` with a FIFO queue, typed options
//! (`allow_once`, `allow_always`, `reject_once`, `reject_always`), session rules
//! keyed by tool and target, a cancellable timeout (not a refusal), and MCP auditing.
#![forbid(unsafe_code)]

use crawrad_note::note;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

#[note("permission-operationkind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationKind {
    Read,
    Write,
    Execute,
}

impl OperationKind {
    pub fn badge(&self) -> &'static str {
        match self {
            Self::Read => "READ",
            Self::Write => "WRITE",
            Self::Execute => "EXECUTE",
        }
    }
}

#[note("permission-permissionoptionkind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionOptionKind {
    AllowOnce,
    AllowAlways,
    RejectOnce,
    RejectAlways,
}

impl PermissionOptionKind {
    #[note("permission-permissionoptionkind-display-label")]
    pub fn display_label(&self) -> &'static str {
        match self {
            Self::AllowOnce => "Allow Once",
            Self::AllowAlways => "Always Allow",
            Self::RejectOnce => "Reject",
            Self::RejectAlways => "Never Allow",
        }
    }

    #[note("permission-permissionoptionkind-parse-kind")]
    pub fn parse_kind(raw: &str) -> Self {
        let s = raw.to_lowercase();
        if s.contains("always") && (s.contains("allow") || s.contains("approve")) {
            Self::AllowAlways
        } else if s.contains("always") && (s.contains("reject") || s.contains("deny")) {
            Self::RejectAlways
        } else if s.contains("reject") || s.contains("deny") {
            Self::RejectOnce
        } else {
            Self::AllowOnce
        }
    }
}

#[note("permission-permissionoption")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionOption {
    pub option_id: String,
    pub name: String,
    pub kind: PermissionOptionKind,
}

impl PermissionOption {
    pub fn new(
        option_id: impl Into<String>,
        name: impl Into<String>,
        kind: PermissionOptionKind,
    ) -> Self {
        Self {
            option_id: option_id.into(),
            name: name.into(),
            kind,
        }
    }

    #[note("permission-permissionoption-default-options")]
    pub fn default_options() -> Vec<Self> {
        vec![
            Self::new("allow_once", "Allow Once", PermissionOptionKind::AllowOnce),
            Self::new(
                "allow_always",
                "Always Allow for Session",
                PermissionOptionKind::AllowAlways,
            ),
            Self::new("reject_once", "Reject", PermissionOptionKind::RejectOnce),
            Self::new(
                "reject_always",
                "Never Allow for Session",
                PermissionOptionKind::RejectAlways,
            ),
        ]
    }
}

#[note("permission-permissiondecision")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionDecision {
    /// The user picked a specific option (by `option_id`).
    Selected(String),
    /// Cancelled by timeout or interruption (not a voluntary refusal).
    Cancelled,
}

#[note("permission-permissionrequest")]
#[derive(Clone)]
pub struct PermissionRequest {
    pub id: u64,
    pub session_id: String,
    pub tool_name: String,
    pub operation_kind: OperationKind,
    pub target: Option<String>,
    pub arguments_summary: String,
    pub options: Vec<PermissionOption>,
    pub response_tx: Arc<Mutex<Option<Sender<PermissionDecision>>>>,
    pub created_at: Instant,
    pub timeout_duration: Duration,
    /// The file change an edit tool asks for, shown as a diff instead of the card.
    pub proposed_edit: Option<crate::edit_ask::ProposedEdit>,
}

impl std::fmt::Debug for PermissionRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PermissionRequest")
            .field("id", &self.id)
            .field("session_id", &self.session_id)
            .field("tool_name", &self.tool_name)
            .field("target", &self.target)
            .field("arguments_summary", &self.arguments_summary)
            .finish()
    }
}

impl PartialEq for PermissionRequest {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.session_id == other.session_id
            && self.tool_name == other.tool_name
            && self.target == other.target
            && self.arguments_summary == other.arguments_summary
    }
}

impl Eq for PermissionRequest {}

impl PermissionRequest {
    pub fn new(
        id: u64,
        session_id: impl Into<String>,
        tool_name: impl Into<String>,
        target: Option<String>,
        arguments_summary: impl Into<String>,
        options: Vec<PermissionOption>,
        response_tx: Sender<PermissionDecision>,
        timeout_duration: Duration,
    ) -> Self {
        let t_name = tool_name.into();
        let kind = classify_tool(&t_name);
        let opts = if options.is_empty() {
            PermissionOption::default_options()
        } else {
            options
        };
        Self {
            id,
            session_id: session_id.into(),
            tool_name: t_name,
            operation_kind: kind,
            target,
            arguments_summary: arguments_summary.into(),
            options: opts,
            response_tx: Arc::new(Mutex::new(Some(response_tx))),
            created_at: Instant::now(),
            timeout_duration,
            proposed_edit: None,
        }
    }

    #[note("permission-permissionrequest-with-proposed-edit")]
    pub fn with_proposed_edit(mut self, edit: Option<crate::edit_ask::ProposedEdit>) -> Self {
        self.proposed_edit = edit;
        self
    }

    #[note("permission-permissionrequest-respond")]
    pub fn respond(&self, decision: PermissionDecision) -> bool {
        if let Ok(mut guard) = self.response_tx.lock()
            && let Some(tx) = guard.take()
        {
            let _ = tx.send(decision);
            return true;
        }
        false
    }

    #[note("permission-permissionrequest-is-expired")]
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.timeout_duration
    }
}

#[note("permission-sessionpermissions")]
#[derive(Debug, Default, Clone)]
pub struct SessionPermissions {
    pub always_allowed: HashSet<(String, Option<String>)>,
    pub always_rejected: HashSet<(String, Option<String>)>,
}

impl SessionPermissions {
    pub fn new() -> Self {
        Self::default()
    }

    #[note("permission-sessionpermissions-allow-always")]
    pub fn allow_always(&mut self, tool: impl Into<String>, target: Option<String>) {
        self.always_allowed.insert((tool.into(), target));
    }

    #[note("permission-sessionpermissions-reject-always")]
    pub fn reject_always(&mut self, tool: impl Into<String>, target: Option<String>) {
        self.always_rejected.insert((tool.into(), target));
    }

    #[note("permission-sessionpermissions-is-allowed")]
    pub fn is_allowed(&self, tool: &str, target: Option<&str>) -> bool {
        // 1. Exact match of (tool, target)
        if self
            .always_allowed
            .contains(&(tool.to_string(), target.map(str::to_string)))
        {
            return true;
        }
        // 2. Whether the tool was allowed globally (no specific target)
        if target.is_some() && self.always_allowed.contains(&(tool.to_string(), None)) {
            return true;
        }
        false
    }

    #[note("permission-sessionpermissions-is-rejected")]
    pub fn is_rejected(&self, tool: &str, target: Option<&str>) -> bool {
        if self
            .always_rejected
            .contains(&(tool.to_string(), target.map(str::to_string)))
        {
            return true;
        }
        if target.is_some() && self.always_rejected.contains(&(tool.to_string(), None)) {
            return true;
        }
        false
    }

    #[note("permission-sessionpermissions-clear")]
    pub fn clear(&mut self) {
        self.always_allowed.clear();
        self.always_rejected.clear();
    }
}

#[note("permission-permissionqueue")]
#[derive(Default, Clone, Debug)]
pub struct PermissionQueue {
    requests: VecDeque<Arc<PermissionRequest>>,
}

impl PermissionQueue {
    pub fn new() -> Self {
        Self::default()
    }

    #[note("permission-permissionqueue-push")]
    pub fn push(&mut self, req: impl Into<Arc<PermissionRequest>>) {
        self.requests.push_back(req.into());
    }

    #[note("permission-permissionqueue-current")]
    pub fn current(&self) -> Option<Arc<PermissionRequest>> {
        self.requests.front().cloned()
    }

    #[note("permission-permissionqueue-peek")]
    pub fn peek(&self) -> Option<Arc<PermissionRequest>> {
        self.current()
    }

    #[note("permission-permissionqueue-pop")]
    pub fn pop(&mut self) -> Option<Arc<PermissionRequest>> {
        self.requests.pop_front()
    }

    #[note("permission-permissionqueue-remove-by-id")]
    pub fn remove_by_id(&mut self, id: u64) -> Option<Arc<PermissionRequest>> {
        if let Some(pos) = self.requests.iter().position(|r| r.id == id) {
            self.requests.remove(pos)
        } else {
            None
        }
    }

    #[note("permission-permissionqueue-len")]
    pub fn len(&self) -> usize {
        self.requests.len()
    }

    #[note("permission-permissionqueue-is-empty")]
    pub fn is_empty(&self) -> bool {
        self.requests.is_empty()
    }

    #[note("permission-permissionqueue-clear")]
    pub fn clear(&mut self) {
        self.clear_and_cancel_all();
    }

    #[note("permission-permissionqueue-clear-and-cancel-all")]
    pub fn clear_and_cancel_all(&mut self) {
        while let Some(req) = self.requests.pop_front() {
            req.respond(PermissionDecision::Cancelled);
        }
    }
}

#[note("permission-classify-tool")]
pub fn classify_tool(tool_name: &str) -> OperationKind {
    let lower = tool_name.to_lowercase();
    if lower.starts_with("run.")
        || lower.starts_with("build.")
        || lower == "project.build"
        || lower == "project.add_dependency"
        || lower == "project.install_extension"
        || lower.contains("exec")
        || lower.contains("command")
        || lower.contains("bash")
        || lower.contains("cargo")
    {
        OperationKind::Execute
    } else if matches!(lower.as_str(), "edit" | "multiedit" | "notebookedit")
        || lower.starts_with("designer.add")
        || lower.starts_with("designer.set")
        || lower.starts_with("designer.delete")
        || lower.starts_with("designer.wire")
        || lower.starts_with("editor.apply")
        || lower == "code.edit"
        || lower == "project.uninstall_extension"
        || lower == "editor.edit_file"
        || lower.starts_with("project.create")
        || lower.starts_with("project.delete")
        || lower.starts_with("form.mutate")
        || lower.contains("add_")
        || lower.contains("set_")
        || lower.contains("delete_")
        || lower.contains("wire_")
        || lower.contains("mutate")
        || lower.contains("write")
        || lower.contains("create")
        || lower.contains("remove")
        || lower.contains("apply_edit")
        || lower.contains("edit_")
        || lower.ends_with(".add")
        || lower.ends_with(".set")
        || lower.ends_with(".delete")
    {
        OperationKind::Write
    } else {
        OperationKind::Read
    }
}

#[note("permission-is-read-only-tool")]
pub fn is_read_only_tool(tool_name: &str) -> bool {
    classify_tool(tool_name) == OperationKind::Read
}

#[note("permission-permissionmanager")]
#[derive(Default)]
pub struct PermissionManager {
    pub queue: PermissionQueue,
    pub session_perms: SessionPermissions,
}

impl PermissionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn active(&self) -> Option<Arc<PermissionRequest>> {
        self.queue.current()
    }

    pub fn pop_active(&mut self) -> Option<Arc<PermissionRequest>> {
        self.queue.pop()
    }

    pub fn enqueue(&mut self, req: impl Into<Arc<PermissionRequest>>) {
        self.queue.push(req);
    }

    pub fn clear(&mut self) {
        self.queue.clear_and_cancel_all();
    }

    #[note("permission-permissionmanager-classify-tool")]
    pub fn classify_tool(tool_name: &str) -> OperationKind {
        classify_tool(tool_name)
    }
}

static GLOBAL_PERMISSIONS: OnceLock<Arc<Mutex<PermissionManager>>> = OnceLock::new();

#[note("permission-global")]
pub fn global() -> &'static Arc<Mutex<PermissionManager>> {
    GLOBAL_PERMISSIONS.get_or_init(|| Arc::new(Mutex::new(PermissionManager::new())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_queue_keeps_order_and_independent_answers() {
        let mut queue = PermissionQueue::new();
        assert!(queue.is_empty());

        let (tx1, rx1) = std::sync::mpsc::channel();
        let (tx2, rx2) = std::sync::mpsc::channel();

        let req1 = PermissionRequest::new(
            1,
            "session-1",
            "edit_file",
            Some("file1.rs".into()),
            "args1",
            vec![],
            tx1,
            Duration::from_secs(30),
        );

        let req2 = PermissionRequest::new(
            2,
            "session-1",
            "bash",
            Some("cargo check".into()),
            "args2",
            vec![],
            tx2,
            Duration::from_secs(30),
        );

        queue.push(req1);
        queue.push(req2);

        assert_eq!(queue.len(), 2);
        assert_eq!(queue.current().unwrap().id, 1);

        // Answer request 1 with AllowOnce
        let p1 = queue.pop().unwrap();
        assert_eq!(p1.id, 1);
        p1.respond(PermissionDecision::Selected("allow_once".into()));

        // Request 2 is now at the top
        assert_eq!(queue.len(), 1);
        let p2 = queue.pop().unwrap();
        assert_eq!(p2.id, 2);
        p2.respond(PermissionDecision::Selected("reject_once".into()));

        // Check that each receiver got exclusively its own decision
        assert_eq!(
            rx1.recv().unwrap(),
            PermissionDecision::Selected("allow_once".into())
        );
        assert_eq!(
            rx2.recv().unwrap(),
            PermissionDecision::Selected("reject_once".into())
        );
    }

    #[test]
    fn test_session_permissions_keyed_by_tool_and_target() {
        let mut perms = SessionPermissions::new();
        assert!(!perms.is_allowed("edit_file", Some("src/main.rs")));

        // Allow only src/main.rs
        perms.allow_always("edit_file", Some("src/main.rs".to_string()));
        assert!(perms.is_allowed("edit_file", Some("src/main.rs")));
        // Another target on the same tool is NOT allowed automatically
        assert!(!perms.is_allowed("edit_file", Some("src/lib.rs")));

        // Allow the tool globally
        perms.allow_always("read_file", None);
        assert!(perms.is_allowed("read_file", Some("any_file.rs")));
        assert!(perms.is_allowed("read_file", None));
    }

    #[test]
    fn test_auto_approve_reads_identifies_read_tools() {
        assert!(is_read_only_tool("read_file"));
        assert!(is_read_only_tool("fs/read_text_file"));
        assert!(is_read_only_tool("list_directory"));
        assert!(is_read_only_tool("form.read"));
        assert!(is_read_only_tool("designer.get_state"));

        assert!(!is_read_only_tool("write_file"));
        assert!(!is_read_only_tool("edit_file"));
        assert!(!is_read_only_tool("bash"));
        assert!(!is_read_only_tool("component.add"));
    }

    #[test]
    fn test_timeout_results_in_cancelled() {
        let (tx, rx) = std::sync::mpsc::channel();
        let req = PermissionRequest::new(
            99,
            "session-1",
            "bash",
            None,
            "",
            vec![],
            tx,
            Duration::from_millis(10),
        );
        std::thread::sleep(Duration::from_millis(15));
        assert!(req.is_expired());
        req.respond(PermissionDecision::Cancelled);
        assert_eq!(rx.recv().unwrap(), PermissionDecision::Cancelled);
    }
}
