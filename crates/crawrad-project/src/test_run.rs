//! The Test panel's runs: `cargo test` of the user's package, one
//! target at a time, on a task — never on the UI thread.

use crate::test_runner::{
    Outcome as Verdict, OutputParts, RowParts, TargetRun, TestTarget, TestTree, package_dir,
    parse_list, parse_run, targets,
};
use crawrad_os::process::{CargoCall, Outcome};
use std::path::{Path, PathBuf};

/// What the panel asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestJob {
    /// The tests of every target (`-- --list`), none run.
    List,
    /// Every test of every target.
    RunAll,
    /// These tests, by target (Run failed, a file, one test).
    Run(Vec<(usize, Vec<String>)>),
}

/// What a job found.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TestReport {
    /// The package's folder, from its manifest.
    pub package_dir: Option<PathBuf>,
    pub targets: Vec<TestTarget>,
    pub lists: Vec<(usize, Vec<String>)>,
    pub runs: Vec<(usize, TargetRun)>,
    /// The whole package ran: tests no run named are gone.
    pub full: bool,
    /// Why nothing could run (no package, metadata failed).
    pub error: Option<String>,
}

/// Runs `job` for `package`, asking `cargo` (the args, from `dir`) each step
/// and telling `progress` which file runs.
pub fn perform(
    job: &TestJob,
    package: &str,
    root: &Path,
    mut cargo: impl FnMut(&[String]) -> Result<Outcome, String>,
    mut progress: impl FnMut(String),
) -> TestReport {
    let mut report = TestReport::default();
    let found = cargo(&[
        "metadata".into(),
        "--format-version".into(),
        "1".into(),
        "--no-deps".into(),
    ]);
    report.targets = match found {
        Ok(o) if o.success => {
            report.package_dir = package_dir(&o.stdout, package);
            targets(&o.stdout, package)
        }
        Ok(o) => {
            return failed(
                report,
                o.report().unwrap_or_else(|| "cargo metadata failed".into()),
            );
        }
        Err(e) => return failed(report, e),
    };
    if report.targets.is_empty() {
        return failed(report, format!("{package} has no test targets"));
    }
    let picked: Vec<(usize, Vec<String>)> = match job {
        TestJob::List | TestJob::RunAll => {
            (0..report.targets.len()).map(|t| (t, Vec::new())).collect()
        }
        TestJob::Run(sel) => sel.clone(),
    };
    report.full = *job == TestJob::RunAll;
    for (t, tests) in picked {
        let Some(target) = report.targets.get(t) else {
            continue;
        };
        let mut args = target.cargo_args(package, &tests);
        if *job == TestJob::List {
            args.push("--list".into());
            if let Ok(o) = cargo(&args) {
                report.lists.push((t, parse_list(&o.stdout)));
            }
            continue;
        }
        progress(format!("Running {}", target.source));
        let mut run = TargetRun::default();
        match cargo(&args) {
            Ok(o) => {
                run = parse_run(&o.stdout, root);
                if run.outcomes.is_empty() && !o.success {
                    run.error = Some(o.report().unwrap_or_else(|| "cargo test failed".into()));
                }
            }
            Err(e) => run.error = Some(e),
        }
        report.runs.push((t, run));
    }
    report
}

fn failed(mut report: TestReport, error: String) -> TestReport {
    report.error = Some(error);
    report
}

impl TestTree {
    /// Takes a job's report. New targets start the tree over; a full run
    /// forgets tests it did not name.
    pub fn apply(&mut self, report: TestReport) -> Option<String> {
        if report.error.is_some() {
            return report.error;
        }
        if let Some(dir) = report.package_dir {
            self.package_dir = dir;
        }
        if report.targets != self.targets {
            self.targets = report.targets;
            self.cases.clear();
            self.failures.clear();
            self.seconds.clear();
            self.selected = None;
        }
        for (t, names) in &report.lists {
            self.listed(*t, names);
        }
        let mut error = None;
        for (t, run) in report.runs {
            // A target that did not compile keeps the tests it listed.
            if report.full && run.error.is_none() {
                self.cases.retain(|c| c.target != t);
            }
            error = error.or_else(|| run.error.clone());
            self.ran(t, run);
        }
        error
    }
}

/// The runs the panel started, each on a task.
#[derive(Default)]
pub struct TestRuns {
    tasks: crawrad_os::tasks::TaskManager<TestReport>,
}

impl TestRuns {
    /// Starts `job` for `package` in the workspace at `root`.
    pub fn start(&mut self, job: TestJob, package: &str, root: &Path) {
        let (package, root): (String, PathBuf) = (package.into(), root.into());
        self.tasks.spawn("tests", move |reporter| {
            let cargo = |args: &[String]| {
                crawrad_os::process::wait(&mut CargoCall::new(args.to_vec(), &root).command())
            };
            let report = perform(&job, &package, &root, cargo, |text| reporter.progress(text));
            reporter.done(report)
        });
    }

    /// Whether a run is on.
    pub fn busy(&self) -> bool {
        self.tasks.is_running("tests")
    }

    /// The reports that came in since the last call, and the latest progress line.
    pub fn pump(&mut self) -> (Vec<TestReport>, Option<String>) {
        let mut reports = Vec::new();
        let mut said = None;
        for (_, event) in self.tasks.drain() {
            match event {
                crawrad_os::tasks::TaskEvent::Done(r) => reports.push(r),
                crawrad_os::tasks::TaskEvent::Progress(text) => said = Some(text),
                crawrad_os::tasks::TaskEvent::Failed(e) => reports.push(TestReport {
                    error: Some(e),
                    ..Default::default()
                }),
            }
        }
        (reports, said)
    }
}

/// The head, rows and Test output the views paint.
pub type ViewParts = (
    ([(String, bool); 3], String),
    Vec<RowParts>,
    Option<OutputParts>,
);

/// What the user did in the Tests panel or its Test output.
#[derive(Debug, Clone, PartialEq)]
pub enum PanelInput {
    RunAll,
    RunFailed,
    Refresh,
    Chip(usize),
    Filter(String),
    Row(usize),
    RunRow(usize),
    /// The Test output's link to the assert.
    Spot,
}

/// What the shell does next.
#[derive(Debug, Clone, PartialEq)]
pub enum PanelEffect {
    /// Repaint the panel.
    Paint,
    /// Say this on the status bar (a run ended: the editor marks changed too).
    Status(String),
    /// Open this file at this 1-based line and column; `output` shows the Test output.
    Go {
        place: (PathBuf, usize, usize),
        output: bool,
    },
}

/// The Tests panel's whole state: the tree, its runs, what runs now.
#[derive(Default)]
pub struct TestPanel {
    pub tree: TestTree,
    runs: TestRuns,
    note: Option<String>,
}

impl TestPanel {
    /// The panel opened: the first time, its tests are listed. `target` is
    /// the workspace root and the package to test.
    pub fn open(&mut self, target: Option<(PathBuf, String)>) -> PanelEffect {
        match self.tree.cases.is_empty() {
            true => self.start(TestJob::List, target),
            false => PanelEffect::Paint,
        }
    }

    fn start(&mut self, job: TestJob, target: Option<(PathBuf, String)>) -> PanelEffect {
        if self.runs.busy() {
            return PanelEffect::Paint;
        }
        let Some((root, package)) = target else {
            return PanelEffect::Status("Tests: open a project first".into());
        };
        self.runs.start(job, &package, &root);
        (self.tree.package, self.tree.root) = (package, root);
        self.note = Some("Building tests".into());
        PanelEffect::Paint
    }

    /// Takes one input.
    pub fn input(&mut self, input: PanelInput, target: Option<(PathBuf, String)>) -> PanelEffect {
        const CHIPS: [Verdict; 3] = [Verdict::Passed, Verdict::Failed, Verdict::Ignored];
        let go = |place, output| PanelEffect::Go { place, output };
        match input {
            PanelInput::RunAll => return self.start(TestJob::RunAll, target),
            PanelInput::RunFailed => return self.start(TestJob::Run(self.tree.failed()), target),
            PanelInput::Refresh => return self.start(TestJob::List, target),
            PanelInput::Chip(i) => self.tree.toggle(CHIPS[i.min(2)]),
            PanelInput::Filter(text) => self.tree.filter = text,
            PanelInput::Row(i) => {
                return self
                    .tree
                    .click(i)
                    .map_or(PanelEffect::Paint, |p| go(p, true));
            }
            PanelInput::RunRow(i) => {
                if let Some(sel) = self.tree.run_for(i) {
                    return self.start(TestJob::Run(sel), target);
                }
            }
            PanelInput::Spot => {
                return self
                    .tree
                    .output_place()
                    .map_or(PanelEffect::Paint, |p| go(p, false));
            }
        }
        PanelEffect::Paint
    }

    /// What came in from the runs: nothing, a progress line, or a run's end.
    pub fn pump(&mut self) -> Option<PanelEffect> {
        let (reports, said) = self.runs.pump();
        let mut effect = said.map(|text| {
            self.note = Some(text);
            PanelEffect::Paint
        });
        for report in reports {
            self.note = None;
            effect = Some(PanelEffect::Status(self.take(report)));
        }
        effect
    }

    /// Applies a run's report; returns the status line it leaves.
    fn take(&mut self, report: TestReport) -> String {
        match self.tree.apply(report) {
            Some(error) => format!("Tests did not run: {error}"),
            None if self.tree.count(Verdict::Failed) > 0 => {
                let failed = self.tree.chip(Verdict::Failed).0;
                format!("{failed} \u{00b7} {}", self.tree.summary())
            }
            None => self.tree.summary(),
        }
    }

    /// The head and rows the view paints, and the Test output.
    pub fn view(&self) -> ViewParts {
        let head = self.tree.view_head(self.note.as_deref());
        (head, self.tree.view_rows(), self.tree.output())
    }
}

#[cfg(test)]
#[path = "test/test_run/test.rs"]
mod test_run_test;
