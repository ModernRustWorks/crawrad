//! **Project creation**, headless: what `cargo` does, in what order, and
//! what the progress window reports while it happens.
//!
//! # Why it left `main.rs`
//!
//! The same reason as [`crate::generator`], and the same day of the same conclusion:
//! the Makepad shell is another `[[bin]]` and cannot reach what lives in the root
//! of the egui shell. While this state machine was in there, **`New Project` did not
//! exist in this IDE when running in Makepad** — the button was not there, the action
//! was not in the menu, and the path ended in a method the other binary does not see.
//!
//! What crossed over was the entire state machine: the plan in steps, the channel
//! with the thread, the count of *"step 3 of 5"*, the outcome with reservation and
//! `create_project` that calls `cargo`. None of this knew `egui` — it was together
//! by file accident.
//!
//! # What did NOT come
//!
//! The UI drawing. Each shell renders the window according to its framework, and both
//! ask this file the same thing: *what is the plan, which step is it on, how long
//! has it been running, and what was the outcome at the end*.

use std::path::{Path, PathBuf};

use crawrad_ir::Document;

use crate::{generator, workspace};
use crawrad_note::note;
use crawrad_os::process;

// --------------------------------------------- project creation in flight

#[note("creation-creationstep")]
#[derive(Debug)]
pub enum CreationStep {
    /// The entire plan, in order, **before** the first step.
    Plan(Vec<workspace::Step>),
    /// This step started now.
    Started(workspace::Step),
}

#[note("creation-creationprogress")]
#[derive(Debug)]
pub enum CreationProgress {
    Running(CreationStep),
    Done(Result<CreatedProject, workspace::Refusal>),
}

#[note("creation-creation")]
pub struct Creation {
    pub rx: std::sync::mpsc::Receiver<CreationProgress>,
    /// The requested type — it determines the label for each [`workspace::Step`].
    pub project_type: workspace::ProjectType,
    /// The chosen directory, so the dialog can tell WHERE this is happening.
    pub root: PathBuf,
    /// The announced plan. Empty until the first `CreationStep::Plan` arrives —
    /// this takes milliseconds, and the dialog says `starting…` in that gap
    /// rather than inventing a list that might not match what runs.
    pub plan: Vec<workspace::Step>,
    /// How many steps have already completed.
    pub completed: usize,
    /// The current step in progress.
    pub current: Option<workspace::Step>,
    /// When it started — the dialog shows ELAPSED time, not an estimate.
    ///
    /// An estimate would require measurement we do not have: `cargo add` takes
    /// 6.8 s with a warm index and much more the first time of the day, and a bar
    /// promising "2 s remaining" that exceeds ten is worse than none. Elapsed time
    /// is a number that is always accurate.
    pub since: std::time::Instant,
    /// The outcome, when available. It decides whether the dialog closes or stays.
    pub outcome: Option<Outcome>,
    /// Is the dialog visible on screen?
    ///
    /// `false` after `Hide`, `Esc` or the window's close button. Creation
    /// **continues** — what the user dismissed was the window, not the work —,
    /// and the outcome goes to the status bar as before. This prevents the dialog
    /// from becoming the trap the user identified: a modal that cannot close.
    pub show: bool,
}

impl Creation {
    #[note("creation-creation-advance")]
    pub fn advance(&mut self, step: CreationStep) {
        match step {
            CreationStep::Plan(plan) => self.plan = plan,
            CreationStep::Started(step) => {
                // The previous step finished because the next one STARTED:
                // `create_project` is sequential, and asking it for a second notice
                // of "finished" would be twice the lines for the same fact — with
                // the risk of one of the two missing in one of the branches.
                if self.current.is_some() {
                    self.completed += 1;
                }
                self.current = Some(step);
            }
        }
    }

    #[note("creation-creation-phrase")]
    pub fn phrase(&self) -> String {
        let Some(current) = self.current else {
            if self.plan.is_empty() {
                return "starting…".to_string();
            }
            return "finishing…".to_string();
        };
        let total = self.plan.len();
        let label = current.label(self.project_type);
        if total == 0 {
            return label;
        }
        // `completed + 1` is the number of the step in progress: `completed` counts
        // those that finished, and whoever is on screen is the next one.
        format!("Step {} of {total} — {label}", self.completed + 1)
    }

    #[note("creation-creation-settle")]
    #[must_use = "`false` instructs to discard creation state; ignoring it leaves `New Project…` unresponsive"]
    pub fn settle(&mut self, outcome: Outcome) -> bool {
        if outcome.closes_automatically() || !self.show {
            return false;
        }
        self.outcome = Some(outcome);
        true
    }
}

#[note("creation-outcome")]
#[derive(Debug, PartialEq)]
pub enum Outcome {
    /// Created and opened, without issues. **The dialog closes** — green creation has
    /// nothing to show, and it is the same choice ADR 0023 made for compilation.
    Opened(PathBuf),
    /// Created, **opened**, and with a warning to read. The dialog **stays** with it.
    ///
    /// This branch was born from a revision, and the defect it fixes is the bug of
    /// this entire PR returning through the door the PR opened. The warning fell into
    /// [`Outcome::Stay`] — and `Stay` means **did not open** —, so the project
    /// created with a warning remained entirely on disk while the IDE stayed silent
    /// on top of the old project, without even the `Open` button.
    ///
    /// **And the most common warning is no failure at all.** `cargo new` inside an
    /// external workspace adopts the package and reports it; this happens in any folder
    /// that lives under a workspace, which includes every project this IDE created up
    /// to 08/09/2026 (they are all group+member, i.e., workspace roots) and any cloned
    /// Rust repository. The most probable path for the warning was one where everything
    /// went right.
    ///
    /// The cause was the type: *opens?* and *does the dialog stay?* are independent
    /// questions, and a two-branch enum only answered yes/no and no/yes.
    ///
    /// **Without `offer`**, and now for the right reason — the folder IS open, and an
    /// `Open` button for what is already on screen is a button that does nothing.
    OpenedWithWarning { root: PathBuf, phrase: String },
    /// **Did not open**, and the dialog stays with the phrase explaining why.
    ///
    /// Three cases fall here — refused creation, opening blocked by unsaved work,
    /// and thread that died without a verdict —, and it is deliberate that they are
    /// a single type: what they have in common is *nothing changed on screen, and
    /// disappearing would hide the reason*.
    Stay {
        phrase: String,
        /// The directory that the `Open` button would open, when there is one to offer.
        ///
        /// It is the fix for the user report seen from the other side: they pointed
        /// `New Project…` twice to the same folder because that was the project
        /// they wanted. The button is the response.
        offer: Option<PathBuf>,
    },
}

impl Outcome {
    #[note("creation-outcome-root-to-open")]
    pub fn root_to_open(&self) -> Option<&Path> {
        match self {
            Self::Opened(r) => Some(r),
            Self::OpenedWithWarning { root, .. } => Some(root),
            Self::Stay { .. } => None,
        }
    }

    #[note("creation-outcome-closes-automatically")]
    pub fn closes_automatically(&self) -> bool {
        matches!(self, Self::Opened(_))
    }

    #[note("creation-outcome-phrase")]
    pub fn phrase(&self) -> Option<&str> {
        match self {
            Self::Opened(_) => None,
            Self::OpenedWithWarning { phrase, .. } | Self::Stay { phrase, .. } => Some(phrase),
        }
    }

    #[note("creation-outcome-on-screen")]
    pub fn on_screen(&self) -> Option<(String, Option<PathBuf>)> {
        match self {
            Self::Opened(_) => None,
            // The folder IS open: an `Open` button for what is already on screen is
            // a button that does nothing.
            Self::OpenedWithWarning { phrase, .. } => Some((phrase.clone(), None)),
            Self::Stay { phrase, offer } => Some((phrase.clone(), offer.clone())),
        }
    }
}

#[note("creation-creation-outcome")]
pub fn creation_outcome(
    project_root: &Path,
    done: Result<CreatedProject, workspace::Refusal>,
    permission: Result<(), String>,
) -> Outcome {
    let created = match done {
        Ok(c) => c,
        Err(refusal) => {
            return Outcome::Stay {
                phrase: refusal.phrase,
                // A project already exists in the chosen folder: opening is the exit,
                // and it was what the user wanted in both attempts. Nothing was overwritten —
                // the offer is to OPEN.
                offer: refusal.open_instead.then(|| project_root.to_path_buf()),
            };
        }
    };
    if let Err(phrase) = permission {
        return Outcome::Stay {
            // The path goes into the phrase, and is the part that cannot be missing:
            // the project EXISTS on disk, and a refusal without the path leaves the user
            // searching for what they just created.
            phrase: format!(
                "created {} — but it did not open: {phrase}",
                created.root.display()
            ),
            offer: Some(created.root),
        };
    }
    match created.warning {
        None => Outcome::Opened(created.root),
        // OPENS, and the dialog stays with the warning. Both halves, and neither
        // dismisses the other: open because the project exists in full and the gesture was
        // "create and take me there"; the dialog stays because this warning in a status bar
        // is exactly what no one read this time.
        Some(warning) => Outcome::OpenedWithWarning {
            root: created.root,
            phrase: warning,
        },
    }
}

#[note("creation-createdproject")]
#[derive(Debug)]
pub struct CreatedProject {
    pub root: PathBuf,
    /// A non-essential step failed. The project EXISTS — saying so in the bar is
    /// honest; refusing the whole project because of this would discard what
    /// is already finished.
    ///
    /// There are two today, and that is why the phrase can bring both together, separated
    /// by a dash: `cargo add` (typically: no network) and code generation (see
    /// [`generator::generate_for_new_project`]).
    pub warning: Option<String>,
}

#[note("creation-cargo-with-output")]
pub fn cargo_with_output(args: &[String], at: &Path) -> Result<String, String> {
    let output = process::wait(&mut process::CargoCall::new(args.to_vec(), at).command())?;
    // Stripping comes here, and not inside [`launcher`]: `never` there is a request
    // to an external program, and this text is read by `starts_with` right below and
    // by `adopting_workspace`. Requesting and verifying costs one pass over a ten-line string.
    let txt = workspace::without_color(&output.merged());
    if output.success {
        return Ok(txt);
    }
    Err(workspace::first_error(&txt))
}

#[note("creation-create-project")]
pub fn create_project(
    parent: &Path,
    name: &str,
    project_type: workspace::ProjectType,
    form: Option<Document>,
    report: &dyn Fn(CreationStep),
) -> Result<CreatedProject, workspace::Refusal> {
    let cargo = |args: &[String], at: &Path| cargo_with_output(args, at).map(|_| ());

    // **Collisions are refused here, before anything else.** With the typed name,
    // this phrase explains itself — and it is the only thing between the gesture
    // and `cargo`, so its `Nothing was written` is true without needing to undo anything.
    if let Some(refusal) = workspace::refusal_occupied_folder(parent, name) {
        return Err(refusal);
    }

    // `pick_folder` returns an existing folder, so this is inert in normal flow.
    // It stays because creating the chosen folder destroys nothing.
    std::fs::create_dir_all(parent).map_err(|e| format!("could not create the folder: {e}"))?;

    // THE PLAN BEFORE THE FIRST STEP, which allows the dialog to say "2 of 3"
    // instead of an indeterminate bar. It is the same function the dialog reads,
    // and comes from here because this is where `form` is a fact.
    report(CreationStep::Plan(workspace::steps_for(
        project_type,
        form.is_some(),
    )));

    // `cargo new` is the FIRST write, and the only thing that creates a folder.
    //
    // The order "group root comes first" (ADR 0018, Amendment 2) leaves along with
    // the group: it existed so `cargo new` would find OUR root instead of the user's
    // workspace, and without our root there are no two workspaces to conflict.
    // Measured on 08/09/2026 inside an external workspace: cargo registers the package
    // in `members` there and writes `edition.workspace = true` — the package opens
    // (the upper workspace is real and reachable), and what happens is what would happen
    // in terminal. What warns about this is `warning` below, because modifying someone's
    // file might be cargo's doing, but **staying silent** about it would be ours.
    report(CreationStep::Started(workspace::Step::Package));
    let registered_in = match cargo_with_output(&workspace::new_args(name, project_type), parent) {
        Ok(output) => workspace::adopting_workspace(&output),
        Err(e) => return Err(workspace::refusal_new(name, &e)),
    };

    // **The project is the name folder**, one single level (ADR 0017, Amendment 3).
    let member = parent.join(name);
    // A LIST, not an `Option`: from here down there are TWO steps that can fail
    // without bringing down the project — `cargo add` and code generation —, and one
    // overwriting the other's warning would hide half of what happened. The third warning
    // is not a failure: it is someone else's manifest that `cargo new` edited.
    let mut warnings: Vec<String> = Vec::new();
    if let Some(at) = registered_in {
        // **A user file modified silently is the exact thing this product exists
        // NOT to do.** `cargo new` inside an external workspace adds the package to
        // `members` there — measured twice on 08/09/2026, including in this repo's
        // `Cargo.toml`. Previously, "group root first" prevented this; without group,
        // the choice is between reimplementing cargo's decision (the opposite of ADR 0018)
        // and **reporting**. We report.
        warnings.push(format!(
            "`{name}` was added as a member of the workspace at {at} — cargo does \
             that, and it edited that Cargo.toml"
        ));
    }

    let target = crate::generator::active_target();
    let with_form = form.is_some() || project_type.with_form();
    let uses_binding = form
        .as_ref()
        .is_some_and(crawrad_codegen::documento_tem_bindings);
    let deps = crawrad_codegen::dependencias_do_projeto(target, with_form, uses_binding);

    if !deps.is_empty() {
        report(CreationStep::Started(workspace::Step::Dependency));
        for dep in &deps {
            let mut args = vec!["add".to_string()];
            args.extend(dep.args_cargo_add());
            if let Err(e) = cargo(&args, &member) {
                warnings.push(format!(
                    "project created, but `cargo add` for `{}` failed: {e}",
                    dep.nome()
                ));
            }
        }
    }

    if let Some(warning) = form
        .as_ref()
        .and_then(|d| third_party_dependencies(d, &member))
    {
        warnings.push(warning);
    }

    // `form` decides, not `project_type`: whoever prepared the model already asked
    // `project_type.with_form()`, and asking again here would be the same condition
    // written in two places — the pair that diverges at the first fix only one receives.
    if let Some(doc) = form {
        report(CreationStep::Started(workspace::Step::Code));
        let src = member.join("src");
        std::fs::create_dir_all(&src).map_err(|e| format!("could not create src/: {e}"))?;
        doc.save(src.join("form_main.crawform"))
            .map_err(|e| format!("could not write the form: {e}"))?;
        // And the CODE is born along with the form — the reason is in
        // [`generator::generate_for_new_project`]. Failing here does not undo anything:
        // it becomes a warning, and `Generate` remains one click away.
        if let Err(e) = generator::generate_for_new_project(&member, &doc) {
            warnings.push(e);
        }
    }

    Ok(CreatedProject {
        // **The PROJECT folder, not the chosen one.** It is what the IDE opens, and is
        // the line where `test\test` from the report ceased to exist forever.
        root: member,
        warning: (!warnings.is_empty()).then(|| warnings.join(" — ")),
    })
}

#[note("creation-missing-dependencies")]
pub fn missing_dependencies(
    manifest: &str,
    deps: Vec<crawrad_codegen::Dependencia>,
) -> Vec<crawrad_codegen::Dependencia> {
    let mut in_deps = false;
    let mut present = std::collections::HashSet::new();

    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_deps = trimmed == "[dependencies]";
            if let Some(name) = trimmed
                .strip_prefix("[dependencies.")
                .and_then(|rest| rest.strip_suffix(']'))
            {
                present.insert(name.trim().to_string());
            }
            continue;
        }
        if !in_deps || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        for dep in &deps {
            let name = dep.nome();
            let eq_prefix = format!("{name} =");
            let eq_tight = format!("{name}=");
            let dot_prefix = format!("{name}.");
            if trimmed.starts_with(&eq_prefix)
                || trimmed.starts_with(&eq_tight)
                || trimmed.starts_with(&dot_prefix)
            {
                present.insert(name.to_string());
            }
        }
    }

    deps.into_iter()
        .filter(|dep| !present.contains(dep.nome()))
        .collect()
}

#[note("creation-new-form-name")]
pub fn new_form_name(folder: &Path) -> (String, String) {
    let has_form = std::fs::read_dir(folder).is_ok_and(|mut entries| {
        entries.any(|e| e.is_ok_and(|e| e.path().extension().is_some_and(|x| x == "crawform")))
    });
    if !has_form {
        return ("form_main".to_string(), "Form1".to_string());
    }
    let idx = (1..)
        .find(|i| !folder.join(format!("form_{i}.crawform")).exists())
        .unwrap_or(1);
    (format!("form_{idx}"), format!("Form{idx}"))
}

#[note("creation-new-form-action")]
pub const NEW_FORM_ACTION: &str = "file.new_form";

#[note("creation-new-form-hint")]
pub fn new_form_hint(framework_missing: bool) -> &'static str {
    if framework_missing {
        "Adds src/form_main.crawform and the framework dependency"
    } else {
        "Adds src/form_main.crawform"
    }
}

#[note("creation-third-party-dependencies")]
pub fn third_party_dependencies(doc: &crawrad_ir::Document, destination: &Path) -> Option<String> {
    let inst = crate::installed::ide_catalog();
    let mut missing = Vec::new();
    let target = crate::generator::active_target();

    let manifest =
        std::fs::read_to_string(destination.join(crate::workspace::MANIFEST)).unwrap_or_default();
    let framework_deps = crawrad_codegen::dependencias_do_projeto(
        target,
        true,
        crawrad_codegen::documento_tem_bindings(doc),
    );
    for dep in missing_dependencies(&manifest, framework_deps) {
        let mut args = vec!["add".to_string()];
        args.extend(dep.args_cargo_add());
        if let Err(e) = cargo_with_output(&args, destination) {
            missing.push(format!("`cargo add` for `{}` failed: {e}", dep.nome()));
        }
    }

    // What the built-in components on the form need (a bound Grid).
    // Only what the manifest lacks: `cargo add` of a path crate already
    // there rewrites the user's line (it adds `version = "0.1.0"`).
    for dep in missing_dependencies(
        &manifest,
        crawrad_codegen::component_dependencies(doc, target),
    ) {
        let mut args = vec!["add".to_string()];
        args.extend(dep.args_cargo_add());
        if let Err(e) = cargo_with_output(&args, destination) {
            missing.push(format!("`cargo add` for `{}` failed: {e}", dep.nome()));
        }
    }
    for c in crawrad_codegen::classes_de_terceiro(doc, &inst.catalog) {
        if let Some(p) = inst.origin.get(c) {
            let args = ["add", "--path", &p.to_string_lossy()].map(String::from);
            if let Err(e) = cargo_with_output(&args, destination) {
                missing.push(format!("`cargo add --path` for `{c}` failed: {e}"));
            }
        } else {
            missing.push(format!("`{c}` has no crate to depend on"));
        }
    }
    (!missing.is_empty()).then(|| missing.join("; "))
}

/// Creates the next free `unit_N.rs` in the folder.
pub fn create_unit_file(folder: &Path) -> Result<PathBuf, std::io::Error> {
    std::fs::create_dir_all(folder)?;
    let mut idx = 1;
    let mut dest = folder.join(format!("unit_{idx}.rs"));
    while dest.exists() {
        idx += 1;
        dest = folder.join(format!("unit_{idx}.rs"));
    }
    let content = format!("//! Unit unit_{idx}.\n\npub fn setup() {{\n    //\n}}\n");
    std::fs::write(&dest, content)?;
    Ok(dest)
}

/// Creates the next free form in the folder.
/// A form that was created, and whether its code could be generated with it.
///
/// Mirrors [`CreatedProject`]: the form EXISTS even when generation failed, so the
/// warning is said and the form is kept. Refusing the form over this would throw
/// away what is already on disk.
#[note("creation-createdform")]
pub struct CreatedForm {
    pub path: PathBuf,
    pub warning: Option<String>,
}

#[note("creation-create-form-file")]
pub fn create_form_file(
    folder: &Path,
    reg: &crawrad_registry::Registry,
) -> Result<CreatedForm, String> {
    std::fs::create_dir_all(folder).map_err(|e| format!("Failed to create folder: {e}"))?;
    let (stem, caption) = new_form_name(folder);
    let dest = folder.join(format!("{stem}.crawform"));
    let mut doc = crawrad_ir::Document::new(&stem, "Form");
    if let Some(mut root) = reg.instantiate(
        "Form",
        doc.root.clone(),
        crawrad_ir::Rect::new(0.0, 0.0, 640.0, 420.0),
    ) {
        root.set_text(&caption);
        doc.nodes.insert(doc.root.clone(), root);
    }
    let json = doc
        .to_json()
        .map_err(|e| format!("Failed to serialize form: {e}"))?;
    std::fs::write(&dest, json).map_err(|e| format!("Failed to create form: {e}"))?;
    // THE CODE IS BORN WITH THE FORM, exactly as it is when a project is created a
    // few lines above. Without this the `.crawform` existed alone: `cargo` never saw
    // the form, the Explorer had no unit to list, and the only way to make the form
    // real was to find `Generate` — which nothing on screen said was needed.
    //
    // A form outside a crate member (`<member>/src/`) has nowhere to generate into.
    // That is not a failure: the form is kept, and the phrase says why no code came
    // with it instead of leaving the absence unexplained.
    let warning = match crate::source::member_of(&dest) {
        Some(member) => generator::generate_for_new_project(&member, &doc).err(),
        None => Some(format!(
            "form created, but no code was generated: `{}` is not inside a crate's              `src/`, so there is no package to generate into",
            folder.display()
        )),
    };
    Ok(CreatedForm {
        path: dest,
        warning,
    })
}

/// Creates the next free subfolder in the folder.
pub fn create_subfolder(folder: &Path) -> Result<PathBuf, std::io::Error> {
    let mut idx = 1;
    let mut dest = folder.join(format!("folder_{idx}"));
    while dest.exists() {
        idx += 1;
        dest = folder.join(format!("folder_{idx}"));
    }
    std::fs::create_dir_all(&dest)?;
    Ok(dest)
}

#[cfg(test)]
#[path = "test/creation/test.rs"]
mod test;
