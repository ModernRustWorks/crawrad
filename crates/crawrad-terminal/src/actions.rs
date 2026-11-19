use crawrad_note::note;
#[note("actions-terminalaction")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalAction {
    pub id: String,
    pub title: String,
    pub command: String,
    pub category: String,
}

#[note("actions-terminalsnippet")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalSnippet {
    pub id: String,
    pub title: String,
    pub snippet: String,
    pub category: String,
}

#[note("actions-terminalcatalog")]
pub struct TerminalCatalog;

impl TerminalCatalog {
    pub fn default_actions() -> Vec<TerminalAction> {
        vec![
            TerminalAction {
                id: "git_pull".to_string(),
                title: "Git Pull".to_string(),
                command: "git pull --rebase".to_string(),
                category: "ACTIONS".to_string(),
            },
            TerminalAction {
                id: "git_push".to_string(),
                title: "Git Push".to_string(),
                command: "git push origin HEAD".to_string(),
                category: "ACTIONS".to_string(),
            },
            TerminalAction {
                id: "build_debug".to_string(),
                title: "Build (Debug)".to_string(),
                command: "cargo build".to_string(),
                category: "ACTIONS".to_string(),
            },
            TerminalAction {
                id: "list_tasks".to_string(),
                title: "List Tasks".to_string(),
                command: "cargo check --workspace".to_string(),
                category: "ACTIONS".to_string(),
            },
        ]
    }

    pub fn default_snippets() -> Vec<TerminalSnippet> {
        vec![
            TerminalSnippet {
                id: "git_status".to_string(),
                title: "Git Status".to_string(),
                snippet: "git status -sb".to_string(),
                category: "PERSONAL".to_string(),
            },
            TerminalSnippet {
                id: "git_log".to_string(),
                title: "Git Log (Recent)".to_string(),
                snippet: "git log -n 5 --oneline".to_string(),
                category: "PERSONAL".to_string(),
            },
            TerminalSnippet {
                id: "dir_list".to_string(),
                title: "Directory List".to_string(),
                snippet: "Get-ChildItem".to_string(),
                category: "PERSONAL".to_string(),
            },
            TerminalSnippet {
                id: "cargo_test".to_string(),
                title: "Cargo Test All".to_string(),
                snippet: "cargo test --workspace".to_string(),
                category: "PERSONAL".to_string(),
            },
        ]
    }
}

#[note("actions-quick-action-output")]
pub fn quick_action_output(action: &str) -> Option<&'static str> {
    match action {
        "pull" => Some("\n$ git pull --rebase\nCurrent branch main is up to date.\nPS > "),
        "push" => Some("\n$ git push origin HEAD\nEverything up-to-date.\nPS > "),
        "build" => Some(
            "\n$ cargo build\nFinished `dev` profile [unoptimized + debuginfo] in 1.42s\nPS > ",
        ),
        "tasks" => Some(
            "\n$ cargo check --workspace\nChecking workspace targets... Finished in 0.88s (0 errors, 0 warnings)\nPS > ",
        ),
        "test" => Some(
            "\n$ cargo test --workspace\nrunning 475 tests... test result: ok. 475 passed; 0 failed\nPS > ",
        ),
        _ => None,
    }
}
