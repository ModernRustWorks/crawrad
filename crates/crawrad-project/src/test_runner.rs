//! The Test panel's model: a package's test targets, the tests each one
//! lists, what a run of one says, and the tree the panel draws.
//!
//! Each target runs on its own (`cargo test -p <pkg> --lib`, `--bin x`,
//! `--test x`, `--doc`): cargo writes the `Running …` headers on stderr and the
//! results on stdout, so one run per target is what keeps every result with
//! its target — and gives the target's time, the only time stable `cargo test`
//! reports.

use crate::panic_report::{SourceSpot, find_panics};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// What kind of test target, and so which `cargo test` flag selects it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TargetKind {
    Lib,
    Bin(String),
    Test(String),
    Doc,
}

/// One test target of the package: what to pass cargo, and the file it starts from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestTarget {
    pub kind: TargetKind,
    /// The target's root file, relative to the package (`src/lib.rs`, `tests/api.rs`).
    pub source: String,
}

impl TestTarget {
    /// The `cargo test` arguments that select this target, then these tests
    /// by their exact names (all of them when none).
    pub fn cargo_args(&self, package: &str, tests: &[String]) -> Vec<String> {
        let mut args = vec!["test".into(), "-p".into(), package.into()];
        match &self.kind {
            TargetKind::Lib => args.push("--lib".into()),
            TargetKind::Bin(n) => args.extend(["--bin".into(), n.clone()]),
            TargetKind::Test(n) => args.extend(["--test".into(), n.clone()]),
            TargetKind::Doc => args.push("--doc".into()),
        }
        args.push("--".into());
        if !tests.is_empty() {
            args.extend(tests.iter().cloned());
            args.push("--exact".into());
        }
        args
    }
}

/// The folder of `package`'s manifest, from `cargo metadata` JSON.
pub fn package_dir(metadata: &str, package: &str) -> Option<PathBuf> {
    let json = serde_json::from_str::<serde_json::Value>(metadata).ok()?;
    let pkg = json["packages"]
        .as_array()?
        .iter()
        .find(|p| p["name"] == package)?;
    Path::new(pkg["manifest_path"].as_str()?)
        .parent()
        .map(Path::to_path_buf)
}

/// The test targets of `package`, from `cargo metadata` JSON: its lib (and
/// the lib's doctests), bins and integration tests, in that order.
pub fn targets(metadata: &str, package: &str) -> Vec<TestTarget> {
    let Ok(json) = serde_json::from_str::<serde_json::Value>(metadata) else {
        return Vec::new();
    };
    let Some(pkg) = json["packages"]
        .as_array()
        .and_then(|ps| ps.iter().find(|p| p["name"] == package))
    else {
        return Vec::new();
    };
    // Text, not Path: a Windows path from cargo must still split on Linux.
    let manifest = pkg["manifest_path"]
        .as_str()
        .unwrap_or_default()
        .replace('\\', "/");
    let dir = manifest.rsplit_once('/').map_or("", |(d, _)| d).to_string() + "/";
    let mut out = Vec::new();
    for t in pkg["targets"].as_array().into_iter().flatten() {
        let kinds: Vec<&str> = t["kind"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|k| k.as_str())
            .collect();
        let name = t["name"].as_str().unwrap_or_default().to_string();
        let src = t["src_path"]
            .as_str()
            .unwrap_or_default()
            .replace('\\', "/");
        let source = src.strip_prefix(&dir).unwrap_or(&src).to_string();
        let kind = if kinds.iter().any(|k| k.ends_with("lib")) {
            TargetKind::Lib
        } else if kinds.contains(&"bin") {
            TargetKind::Bin(name)
        } else if kinds.contains(&"test") {
            TargetKind::Test(name)
        } else {
            continue;
        };
        let doc = (kind == TargetKind::Lib && t["doctest"].as_bool() != Some(false)).then(|| {
            TestTarget {
                kind: TargetKind::Doc,
                source: source.clone(),
            }
        });
        out.push(TestTarget { kind, source });
        out.extend(doc);
    }
    out.sort_by(|a, b| a.kind.cmp(&b.kind));
    out
}

/// The test names `cargo test -- --list` printed (`name: test` lines).
pub fn parse_list(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter_map(|l| l.trim_end().strip_suffix(": test"))
        .map(str::to_string)
        .collect()
}

/// How a test ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Outcome {
    Failed,
    Passed,
    Ignored,
    /// Listed, not run yet.
    NotRun,
}

impl Outcome {
    /// The tree's mark.
    pub fn mark(self) -> &'static str {
        match self {
            Self::Passed => "\u{2713}",
            Self::Failed => "\u{2717}",
            Self::Ignored => "\u{25cb}",
            Self::NotRun => "\u{00b7}",
        }
    }
}

/// Why a test failed: where, a one-line summary, and what it printed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure {
    /// The assert's place, when the output names one in the project.
    pub spot: Option<SourceSpot>,
    /// `left 3 · right 4` for an `assert_eq!`, else the panic's first line.
    pub summary: String,
    /// The test's whole captured output.
    pub output: String,
}

/// What one target's run said.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TargetRun {
    pub outcomes: Vec<(String, Outcome)>,
    pub failures: BTreeMap<String, Failure>,
    /// `finished in 0.40s`.
    pub seconds: Option<f64>,
    /// Why the target did not run at all (it did not compile).
    pub error: Option<String>,
}

/// Reads a run of one target from its stdout; `root` is the package folder.
pub fn parse_run(stdout: &str, root: &Path) -> TargetRun {
    let mut run = TargetRun::default();
    let lines: Vec<&str> = stdout.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if let Some(rest) = line.strip_prefix("test ")
            && let Some((name, verdict)) = rest.rsplit_once(" ... ")
        {
            let outcome = match verdict.trim() {
                "ok" => Outcome::Passed,
                "FAILED" => Outcome::Failed,
                v if v.starts_with("ignored") => Outcome::Ignored,
                _ => Outcome::NotRun,
            };
            run.outcomes.push((name.to_string(), outcome));
        } else if let Some(name) = line
            .strip_prefix("---- ")
            .and_then(|r| r.strip_suffix(" stdout ----"))
        {
            let mut body = Vec::new();
            while let Some(next) = lines.get(i + 1) {
                if next.starts_with("---- ") || *next == "failures:" {
                    break;
                }
                body.push(*next);
                i += 1;
            }
            let output = body.join("\n").trim().to_string();
            run.failures
                .insert(name.to_string(), failure(name, output, root));
        } else if line.starts_with("test result:") {
            run.seconds = line
                .rsplit_once("finished in ")
                .and_then(|(_, s)| s.trim().trim_end_matches('s').parse().ok());
        }
        i += 1;
    }
    run
}

/// A failure read from a test's output; a doctest's place comes from its name.
fn failure(name: &str, output: String, root: &Path) -> Failure {
    let panic = find_panics(&output, root).into_iter().next();
    let summary = panic
        .as_ref()
        .map(|p| summary(&p.message))
        .unwrap_or_else(|| {
            output
                .lines()
                .find(|l| !l.trim().is_empty())
                .unwrap_or_default()
                .trim()
                .to_string()
        });
    let spot = doc_spot(name).or_else(|| {
        panic
            .map(|p| p.spot)
            .filter(|s| !Path::new(&s.file).is_absolute())
    });
    Failure {
        spot,
        summary,
        output,
    }
}

/// `left 3 · right 4` from an `assert_eq!` message, else its first line.
fn summary(message: &str) -> String {
    let value = |key: &str| {
        message
            .lines()
            .find_map(|l| l.trim().strip_prefix(key).map(|v| v.trim().to_string()))
    };
    match (value("left:"), value("right:")) {
        (Some(l), Some(r)) => format!("left {l} \u{00b7} right {r}"),
        _ => message.lines().next().unwrap_or_default().to_string(),
    }
}

/// `src\lib.rs - double (line 4)` names its own place.
fn doc_spot(name: &str) -> Option<SourceSpot> {
    let (file, rest) = name.split_once(" - ")?;
    let line = rest
        .rsplit_once("(line ")?
        .1
        .strip_suffix(')')?
        .parse()
        .ok()?;
    Some(SourceSpot {
        file: file.to_string(),
        line,
        column: 1,
    })
}

/// The file a unit test lives in: its module path under the target's folder
/// (`math::tests::x` in `src/lib.rs` → `src/math.rs`), the deepest that exists;
/// else the target's own file. A doctest's file is in its name.
pub fn test_file(target: &TestTarget, test: &str, package_dir: &Path) -> String {
    if target.kind == TargetKind::Doc {
        return test
            .split_once(" - ")
            .map(|(f, _)| f.replace('\\', "/"))
            .unwrap_or_else(|| target.source.clone());
    }
    let base = Path::new(&target.source)
        .parent()
        .unwrap_or(Path::new(""))
        .to_path_buf();
    let parts: Vec<&str> = test.split("::").collect();
    let mut found = target.source.clone();
    let mut dir = base.clone();
    for part in parts.iter().take(parts.len().saturating_sub(1)) {
        let file = dir.join(format!("{part}.rs"));
        let module = dir.join(part).join("mod.rs");
        match [file, module]
            .into_iter()
            .find(|f| package_dir.join(f).is_file())
        {
            Some(f) => found = f.to_string_lossy().replace('\\', "/"),
            None => break,
        }
        dir = dir.join(part);
    }
    found
}

/// One test in the tree.
#[derive(Debug, Clone, PartialEq)]
pub struct Case {
    pub target: usize,
    pub name: String,
    pub file: String,
    pub outcome: Outcome,
}

/// The panel's state: targets, their tests, what the last run said, filters.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TestTree {
    pub package: String,
    /// The workspace root (the panics' paths are relative to it) and the package's folder.
    pub root: PathBuf,
    pub package_dir: PathBuf,
    pub targets: Vec<TestTarget>,
    pub cases: Vec<Case>,
    pub failures: BTreeMap<(usize, String), Failure>,
    pub seconds: BTreeMap<usize, f64>,
    /// Folded file nodes: `<target>:<file>`.
    pub folded: BTreeSet<String>,
    /// Outcomes hidden by the chips.
    pub hidden: BTreeSet<Outcome>,
    pub filter: String,
    /// The case shown in Test output.
    pub selected: Option<usize>,
}

/// A row of the tree: the package, its files (a target's doctests are one
/// file node), their tests.
#[derive(Debug, Clone, PartialEq)]
pub enum TreeRow {
    Package {
        label: String,
        mark: Outcome,
        note: String,
    },
    File {
        target: usize,
        file: String,
        label: String,
        mark: Outcome,
        note: String,
        open: bool,
    },
    Case {
        case: usize,
        name: String,
        mark: Outcome,
        selected: bool,
    },
}

impl TestTree {
    /// Lists `names` for target `t`, forgetting what an earlier list said of it.
    pub fn listed(&mut self, t: usize, names: &[String]) {
        let Some(target) = self.targets.get(t).cloned() else {
            return;
        };
        self.cases.retain(|c| c.target != t);
        for name in names {
            let file = test_file(&target, name, &self.package_dir);
            self.cases.push(Case {
                target: t,
                name: name.clone(),
                file,
                outcome: Outcome::NotRun,
            });
        }
    }

    /// Takes a run of target `t`.
    pub fn ran(&mut self, t: usize, run: TargetRun) {
        for (name, outcome) in run.outcomes {
            match self
                .cases
                .iter_mut()
                .find(|c| c.target == t && c.name == name)
            {
                Some(c) => c.outcome = outcome,
                None => {
                    let file = self
                        .targets
                        .get(t)
                        .map(|tg| test_file(tg, &name, &self.package_dir))
                        .unwrap_or_default();
                    self.cases.push(Case {
                        target: t,
                        name,
                        file,
                        outcome,
                    });
                }
            }
        }
        self.failures.retain(|(ft, _), _| *ft != t);
        for (name, f) in run.failures {
            self.failures.insert((t, name), f);
        }
        if let Some(s) = run.seconds {
            self.seconds.insert(t, s);
        }
    }

    /// How many tests ended `outcome`.
    pub fn count(&self, outcome: Outcome) -> usize {
        self.cases.iter().filter(|c| c.outcome == outcome).count()
    }

    /// `44 tests · 3.2 s`.
    pub fn summary(&self) -> String {
        let total: f64 = self.seconds.values().sum();
        match self.seconds.is_empty() {
            true => format!("{} tests", self.cases.len()),
            false => format!("{} tests \u{00b7} {total:.1} s", self.cases.len()),
        }
    }

    /// A chip's text (`Failed 2`) and whether its outcome is shown.
    pub fn chip(&self, outcome: Outcome) -> (String, bool) {
        (
            format!("{} {}", outcome.word(), self.count(outcome)),
            !self.hidden.contains(&outcome),
        )
    }

    /// Shows or hides an outcome.
    pub fn toggle(&mut self, outcome: Outcome) {
        if !self.hidden.remove(&outcome) {
            self.hidden.insert(outcome);
        }
    }

    fn shown(&self, c: &Case) -> bool {
        !self.hidden.contains(&c.outcome)
            && c.name.to_lowercase().contains(&self.filter.to_lowercase())
    }

    /// The worst outcome among `cases`: a failure marks its file and target.
    fn worst<'a>(cases: impl Iterator<Item = &'a Case>) -> Outcome {
        cases.map(|c| c.outcome).min().unwrap_or(Outcome::NotRun)
    }

    /// The rows the panel draws, folded files closed.
    pub fn rows(&self) -> Vec<TreeRow> {
        let shown: Vec<(usize, &Case)> = self
            .cases
            .iter()
            .enumerate()
            .filter(|(_, c)| self.shown(c))
            .collect();
        if shown.is_empty() {
            return Vec::new();
        }
        let failed = shown
            .iter()
            .filter(|(_, c)| c.outcome == Outcome::Failed)
            .count();
        let note = match failed {
            0 => format!("{} tests", shown.len()),
            n => format!("{n} failed"),
        };
        let mark = Self::worst(shown.iter().map(|(_, c)| *c));
        let mut rows = vec![TreeRow::Package {
            label: self.package.clone(),
            mark,
            note,
        }];
        let files: BTreeSet<(usize, &str)> = shown
            .iter()
            .map(|(_, c)| (c.target, c.file.as_str()))
            .collect();
        for (t, file) in files {
            let mut in_file: Vec<&(usize, &Case)> = shown
                .iter()
                .filter(|(_, c)| c.target == t && c.file == file)
                .collect();
            // By name: a run lists in the order the threads finished.
            in_file.sort_by(|a, b| a.1.name.cmp(&b.1.name));
            let open = !self.folded.contains(&format!("{t}:{file}"));
            let doc = self
                .targets
                .get(t)
                .is_some_and(|tg| tg.kind == TargetKind::Doc);
            let label = if doc {
                format!("Doc-tests ({file})")
            } else {
                file.to_string()
            };
            let note = match self.seconds.get(&t) {
                Some(s) => format!("{s:.1} s"),
                None => in_file.len().to_string(),
            };
            let mark = Self::worst(in_file.iter().map(|(_, c)| *c));
            rows.push(TreeRow::File {
                target: t,
                file: file.to_string(),
                label,
                mark,
                note,
                open,
            });
            if open {
                rows.extend(in_file.iter().map(|(i, c)| TreeRow::Case {
                    case: *i,
                    name: Self::short(&c.name, doc),
                    mark: c.outcome,
                    selected: self.selected == Some(*i),
                }));
            }
        }
        rows
    }

    /// A test's name under its file: the last path segment, or a doctest's item and line.
    fn short(name: &str, doc: bool) -> String {
        match doc {
            true => name.split_once(" - ").map_or(name, |(_, r)| r).to_string(),
            false => name.rsplit("::").next().unwrap_or(name).to_string(),
        }
    }

    /// Folds or opens a file row.
    pub fn fold(&mut self, row: &TreeRow) {
        let TreeRow::File { target, file, .. } = row else {
            return;
        };
        let key = format!("{target}:{file}");
        if !self.folded.remove(&key) {
            self.folded.insert(key);
        }
    }

    /// The failure of case `i`, if it failed.
    pub fn failure(&self, i: usize) -> Option<&Failure> {
        let c = self.cases.get(i)?;
        self.failures.get(&(c.target, c.name.clone()))
    }

    /// Where a click on case `i` goes: its assert when it failed, else its file.
    pub fn place(&self, i: usize) -> Option<(PathBuf, usize, usize)> {
        let c = self.cases.get(i)?;
        match self.failure(i).and_then(|f| f.spot.as_ref()) {
            Some(s) => Some((self.spot_file(s), s.line as usize, s.column as usize)),
            None => Some((self.package_dir.join(&c.file), 1, 1)),
        }
    }

    /// A spot's file on disk: under the workspace root, else the package.
    fn spot_file(&self, s: &SourceSpot) -> PathBuf {
        s.resolve(&self.root, &self.package_dir)
            .unwrap_or_else(|| self.package_dir.join(&s.file))
    }

    /// The failed tests by target: what Run failed runs.
    pub fn failed(&self) -> Vec<(usize, Vec<String>)> {
        let mut by: BTreeMap<usize, Vec<String>> = BTreeMap::new();
        for c in self.cases.iter().filter(|c| c.outcome == Outcome::Failed) {
            by.entry(c.target).or_default().push(c.name.clone());
        }
        by.into_iter().collect()
    }

    /// The marks failures leave on `file`: 0-based line and byte, and the message.
    pub fn marks(&self, file: &Path) -> Vec<(usize, usize, String)> {
        (0..self.cases.len())
            .filter_map(|i| {
                let f = self.failure(i)?;
                let s = f.spot.as_ref()?;
                (self.spot_file(s) == file).then(|| {
                    (
                        (s.line as usize).saturating_sub(1),
                        (s.column as usize).saturating_sub(1),
                        format!("failed: {}", f.summary),
                    )
                })
            })
            .collect()
    }
}

/// A tree row as the Tests view paints it (`tests_panel_view::TestsRowParts`).
pub type RowParts = (u8, u8, String, String, bool, bool);

/// The Test output as the view paints it (`tests_panel_view::TestOutputParts`).
pub type OutputParts = (u8, String, String, Vec<String>, String);

impl Outcome {
    /// The chip's word.
    pub fn word(self) -> &'static str {
        match self {
            Self::Passed => "Passed",
            Self::Failed => "Failed",
            Self::Ignored => "Ignored",
            Self::NotRun => "Not run",
        }
    }

    /// The view's mark: 0 failed, 1 passed, 2 ignored, 3 not run.
    pub fn code(self) -> u8 {
        self as u8
    }
}

impl TestTree {
    /// The rows as the view paints them.
    pub fn view_rows(&self) -> Vec<RowParts> {
        self.rows()
            .into_iter()
            .map(|r| match r {
                TreeRow::Package { label, mark, note } => {
                    (0, mark.code(), label, note, true, false)
                }
                TreeRow::File {
                    label,
                    mark,
                    note,
                    open,
                    ..
                } => (1, mark.code(), label, note, open, false),
                TreeRow::Case {
                    name,
                    mark,
                    selected,
                    ..
                } => {
                    let note = if mark == Outcome::Ignored {
                        "ignored"
                    } else {
                        ""
                    };
                    (2, mark.code(), name, note.to_string(), false, selected)
                }
            })
            .collect()
    }

    /// The chips (Passed, Failed, Ignored) and the summary, or what is running.
    pub fn view_head(&self, running: Option<&str>) -> ([(String, bool); 3], String) {
        let chips = [Outcome::Passed, Outcome::Failed, Outcome::Ignored].map(|o| self.chip(o));
        let summary = match running {
            Some(text) => format!("{text}\u{2026}"),
            None => self.summary(),
        };
        (chips, summary)
    }

    /// A click on row `row`: a file folds, a test is picked and its place returned.
    pub fn click(&mut self, row: usize) -> Option<(PathBuf, usize, usize)> {
        match self.rows().get(row)? {
            TreeRow::Case { case, .. } => {
                self.selected = Some(*case);
                self.place(*case)
            }
            other => {
                self.fold(&other.clone());
                None
            }
        }
    }

    /// What a double click on row `row` runs: its file's tests, or the test.
    pub fn run_for(&self, row: usize) -> Option<Vec<(usize, Vec<String>)>> {
        match self.rows().get(row)? {
            TreeRow::Case { case, .. } => {
                let c = self.cases.get(*case)?;
                Some(vec![(c.target, vec![c.name.clone()])])
            }
            TreeRow::File { target, file, .. } => {
                let in_file = self
                    .cases
                    .iter()
                    .filter(|c| c.target == *target && c.file == *file);
                Some(vec![(*target, in_file.map(|c| c.name.clone()).collect())])
            }
            TreeRow::Package { .. } => None,
        }
    }

    /// The picked test's output: its mark, full name, `in file · time`, its
    /// lines, and its assert's place (the link in the lines).
    pub fn output(&self) -> Option<OutputParts> {
        let i = self.selected?;
        let c = self.cases.get(i)?;
        let time = self
            .seconds
            .get(&c.target)
            .map(|s| format!(" \u{00b7} {s:.1} s"));
        let place = format!("in {}{}", c.file, time.unwrap_or_default());
        let (lines, spot) = match self.failure(i) {
            Some(f) => (
                f.output.lines().map(str::to_string).collect(),
                f.spot.as_ref().map(|s| s.label()).unwrap_or_default(),
            ),
            None => (vec![c.outcome.word().to_string()], String::new()),
        };
        let name = format!("{}::{}", self.package, c.name);
        Some((c.outcome.code(), name, place, lines, spot))
    }

    /// Where the output's link goes: the picked test's assert.
    pub fn output_place(&self) -> Option<(PathBuf, usize, usize)> {
        self.place(self.selected?)
    }
}

#[cfg(test)]
#[path = "test/test_runner/test.rs"]
mod test_runner_test;
