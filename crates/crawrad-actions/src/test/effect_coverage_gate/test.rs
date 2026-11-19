//! The tests of the effect coverage gate.
//!
//! Four of them are the mutation proof, and they run **against the real source
//! of the shells**, mutated in memory:
//!
//! - [`a_shell_that_drops_a_field_is_named_by_the_gate`] takes the Makepad
//!   shell as it is on disk, erases its reading of one field, and demands the
//!   gate turn red naming the field and the file;
//! - [`a_new_field_of_the_effect_is_charged_from_every_shell`] adds a field
//!   that no shell can possibly handle and demands **every** shell turn red.
//!   That second behaviour is the whole reason the gate exists: a tenth field
//!   in `Effect` must cost a red in two files, not a silent drop in two
//!   windows;
//! - [`a_shell_that_stops_halfway_through_a_family_is_named_by_the_gate`] does
//!   the same one level down, and the mutation it applies is **an earlier defect
//!   itself**: it deletes the `FileEffect::Run` arm of the Makepad shell, which
//!   is exactly the state the owner found when he pressed `Run` and the IDE
//!   answered nothing;
//! - [`a_variant_added_to_a_family_is_charged_from_every_shell`] adds a variant
//!   no shell can possibly handle and demands every shell turn red — the day
//!   `FileEffect` grows a seventh.
//!
//! Mutating in memory and not on disk is not a shortcut, it is the only honest
//! way here: `crates/crawrad-ide/` belongs to another front, and a gate that
//! needs to edit someone else's file to prove itself is a gate that will be
//! run once and never again.

use super::*;

/// **The shells that APPLY effect** — and until 17/09/2026 there were two.
///
/// ADR 0032 removed the egui shell per owner request — *"the IDE WILL ONLY BE IN
/// MAKEPAD"* —, and with it `crawrad-ide/src/main.rs` stopped applying effects:
/// today that file is the root of the Makepad binary and `apply_effect` lives in
/// `ide_app/effects.rs` (cut out of `ide_app.rs`).
///
/// It remains a LIST, not a single loose item, because the question it asks
/// did not change: *is every shell that applies effect charged?* On the day there
/// is a second one — an MCP applier, for example — it enters here and becomes
/// charged for free.
///
/// And it is a CONSTANT, not a literal in the `for`, because `clippy` rejects
/// loop over a single element — with reason — and this house does not silence lints.
const APPLIERS: &[&str] = &[MAKEPAD_EFFECTS];

/// Where the Makepad shell's `apply_effect` lives.
const MAKEPAD_EFFECTS: &str = "crawrad-ide/src/ide_app/effects.rs";

/// The real Makepad shell, as text.
fn makepad_shell() -> String {
    let path = crates_root().join(MAKEPAD_EFFECTS);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the Makepad shell is gone from {}: {e}", path.display()))
}

// ---------------------------------------------------------------------------
// The gate itself
// ---------------------------------------------------------------------------

/// **The gate.** Every applier answers every field of `Effect`.
#[test]
fn every_applier_answers_every_field_of_the_effect() {
    let fields = effect_fields(actions_lib());
    assert!(
        !fields.is_empty(),
        "no field was read out of `struct Effect`. The parser broke, and a \
         broken parser here is a gate that is green because it is blind."
    );

    let mut complaints = Vec::new();
    for applier in all_appliers() {
        if waiver(&applier).is_some() {
            continue;
        }
        let missing = untouched_fields(&applier, &fields);
        if !missing.is_empty() {
            complaints.push(format!(
                "{}:{} `{}` never reads {} of `{}`",
                applier.file,
                applier.line,
                applier.function,
                missing
                    .iter()
                    .map(|f| format!("`{f}`"))
                    .collect::<Vec<_>>()
                    .join(", "),
                applier.binding,
            ));
        }
    }

    assert!(
        complaints.is_empty(),
        "a field of `Effect` is being dropped on the floor, which costs \
         nothing at compile time and everything on screen — the action runs, \
         the effect comes back correct and the window does not move:\n  {}\n\n\
         Either handle the field, or put the applier in `WAIVERS` with the \
         reason written out.",
        complaints.join("\n  "),
    );
}

/// **The second gate.** Every applier that names one variant of a family names
/// every variant of it.
///
/// This was an earlier issue, and the regression is what it cost: the Makepad shell read
/// `effect.file`, handled `FileEffect::Save` and sent the other five to a
/// `Some(_)`. The field gate above was green, `rustc` was silent, and `Run` was
/// a button that answered *"this action is not wired in the Makepad shell
/// yet"*.
#[test]
fn every_applier_answers_every_variant_it_started_answering() {
    let families = all_families();
    assert!(
        !families.is_empty(),
        "no family was read out of `struct Effect`. A blind ruler is green \
         forever, which is worse than no ruler."
    );

    let mut complaints = Vec::new();
    for applier in all_appliers() {
        if waiver(&applier).is_some() {
            continue;
        }
        let missing = untouched_variants(&applier, &families);
        if !missing.is_empty() {
            complaints.push(format!(
                "{}:{} `{}` names some variants of the family and not {}",
                applier.file,
                applier.line,
                applier.function,
                missing
                    .iter()
                    .map(|v| format!("`{v}`"))
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }
    }

    assert!(
        complaints.is_empty(),
        "an applier stops halfway through a family of `Effect`. `Some(_)` is \
         exhaustive for `rustc`, so nothing else will ever tell you:\n  {}\n\n\
         Name the variant — wiring it is best, and an honest \"not wired here \
         yet\" written per variant is allowed. What is not allowed is a \
         wildcard that also swallows the variant that arrives tomorrow.",
        complaints.join("\n  "),
    );
}

/// No waiver outlives the applier it excused.
///
/// A dead waiver rots quietly and turns into a forgotten licence: the day
/// someone writes a function with that same name in that same file, it is born
/// excused and nobody notices.
#[test]
fn no_waiver_is_dead() {
    let appliers = all_appliers();
    for (file, function, _) in WAIVERS {
        assert!(
            appliers
                .iter()
                .any(|a| a.file == *file && a.function == *function),
            "the waiver for `{function}` in `{file}` matches no applier. \
             Either the function moved and the waiver has to follow, or it is \
             gone and the waiver has to go with it."
        );
    }
}

// ---------------------------------------------------------------------------
// Mutation proof
// ---------------------------------------------------------------------------

/// **Mutation 1** — a shell that stops reading a field is named, by field and
/// by file.
///
/// The mutation is the cheapest possible version of the bug of 15/09/2026:
/// the reading of `zoom` is rewritten into a second reading of `transaction`,
/// which is exactly what "the shell ignores the field" looks like. The whole
/// `crawrad-ide` suite stays green through that change; this gate does not.
///
/// The mutation is written in terms of the binding the gate itself parsed, so
/// renaming the parameter in the shell does not silently defuse the proof.
#[test]
fn a_shell_that_drops_a_field_is_named_by_the_gate() {
    let source = makepad_shell();
    let fields = effect_fields(actions_lib());
    let before = appliers_in(MAKEPAD_EFFECTS, &source);
    let applier = before
        .iter()
        .find(|a| a.function == "apply_effect")
        .expect("`apply_effect` is gone from the Makepad shell");
    assert!(
        untouched_fields(applier, &fields).is_empty(),
        "the Makepad shell already drops a field; the mutation below would be \
         measuring the wrong thing."
    );

    // THE MUTATION IS PRECISE, and it has to be: `zoom` is a PREFIX of
    // `ui_zoom`, which joined the effect on 17/09/2026. A plain
    // `str::replace` also rewrote `effect.ui_zoom` into
    // `effect.transaction_da_ide`, the gate reported two dropped fields instead
    // of one, and this test failed for a reason that had nothing to do with
    // what it measures.
    //
    // A field whose name is a prefix of another is not a mistake to design
    // around — it is normal — so the mutation is what had to become exact.
    let mutated = replace_field(&source, &applier.binding, "zoom", "transaction");
    assert_ne!(mutated, source, "the mutation changed nothing");

    let after = appliers_in(MAKEPAD_EFFECTS, &mutated);
    let applier = after
        .iter()
        .find(|a| a.function == "apply_effect")
        .expect("the mutation broke the parsing of `apply_effect`");
    assert_eq!(
        untouched_fields(applier, &fields),
        vec!["zoom".to_string()],
        "with the zoom arm gone, the gate has to name `zoom` and nothing else. \
         Naming nothing is the silence this module exists to break; naming \
         everything is noise nobody reads."
    );
}

/// **Mutation 2** — a field added to `Effect` is charged from **every** shell.
///
/// The field cannot exist yet in any shell, which is the point: this is what
/// the tenth field of `Effect` will look like on the day it is written. Both
/// shells turn red until they answer it, and that is the behaviour that
/// justifies the gate — the compiler says nothing at all about this.
#[test]
fn a_new_field_of_the_effect_is_charged_from_every_shell() {
    let mut fields = effect_fields(actions_lib());
    fields.push("a_brand_new_field".to_string());

    let charged: Vec<String> = all_appliers()
        .into_iter()
        .filter(|a| waiver(a).is_none())
        .filter(|a| untouched_fields(a, &fields) == vec!["a_brand_new_field".to_string()])
        .map(|a| a.file)
        .collect();

    for shell in APPLIERS {
        assert!(
            charged.iter().any(|f| f == shell),
            "a field added to `Effect` did not charge `{shell}`. The gate \
             would let the next field be dropped exactly the way the last five \
             were. Charged: {charged:?}"
        );
    }
}

/// **Mutation 3 — and it is an earlier defect, replayed.**
///
/// The mutation rewrites the Makepad shell's `FileEffect::Run` arm into a second
/// `FileEffect::Save`, which is the *exact* state the file was in on 15/09/2026:
/// `Save` handled, everything else swallowed by a `Some(_)`. On that build the
/// owner pressed `Run` and the IDE answered nothing.
///
/// The whole `crawrad-ide` suite stays green through that change, and so does
/// `rustc` — `Some(_)` is exhaustive. This gate does not, and it has to name
/// **`FileEffect::Run` and nothing else**: naming nothing is the silence the
/// module exists to break, naming everything is noise nobody reads.
#[test]
fn a_shell_that_stops_halfway_through_a_family_is_named_by_the_gate() {
    let source = makepad_shell();
    let families = all_families();
    let before = appliers_in(MAKEPAD_EFFECTS, &source);
    let applier = before
        .iter()
        .find(|a| a.function == "apply_effect")
        .expect("`apply_effect` is gone from the Makepad shell");
    assert!(
        untouched_variants(applier, &families).is_empty(),
        "the Makepad shell already stops halfway through a family; the \
         mutation below would be measuring the wrong thing: {:?}",
        untouched_variants(applier, &families),
    );

    let mutated = source.replace("FileEffect::Run", "FileEffect::Save");
    assert_ne!(mutated, source, "the mutation changed nothing");

    let after = appliers_in(MAKEPAD_EFFECTS, &mutated);
    let applier = after
        .iter()
        .find(|a| a.function == "apply_effect")
        .expect("the mutation broke the parsing of `apply_effect`");
    assert_eq!(
        untouched_variants(applier, &families),
        vec!["FileEffect::Run".to_string()],
        "with the `Run` arm gone, the gate has to name `FileEffect::Run` and \
         nothing else. That arm disappearing is not a hypothesis — it is what \
         issue #147 was."
    );
}

/// **Mutation 4** — a variant added to a family is charged from **every** shell.
///
/// The variant cannot exist yet in any shell, which is the point: this is what
/// the seventh `FileEffect` will look like on the day it is written. Both shells
/// turn red until they name it, and naming it is cheap — an honest *"not wired
/// here yet"* passes. What does not pass is inheriting the neighbour's excuse.
#[test]
fn a_variant_added_to_a_family_is_charged_from_every_shell() {
    let mut families = all_families();
    let file = families
        .iter_mut()
        .find(|f| f.enum_name == "FileEffect")
        .expect("the `FileEffect` family is gone from `Effect`");
    file.variants.push("AVariantNobodyWrote".into());

    let charged: Vec<String> = all_appliers()
        .into_iter()
        .filter(|a| waiver(a).is_none())
        .filter(|a| {
            untouched_variants(a, &families) == vec!["FileEffect::AVariantNobodyWrote".to_string()]
        })
        .map(|a| a.file)
        .collect();

    for shell in APPLIERS {
        assert!(
            charged.iter().any(|f| f == shell),
            "a variant added to `FileEffect` did not charge `{shell}`. The gate \
             would let the next variant be swallowed exactly the way `Run` \
             was. Charged: {charged:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// The parser, measured on purpose-built text
// ---------------------------------------------------------------------------

/// The field list comes out of the declaration, doc-comments and all.
#[test]
fn the_field_list_is_read_out_of_the_struct() {
    let source = r#"
/// Doc that names `zoom` and `status` without declaring them.
#[derive(Debug, Default)]
pub struct Effect {
    pub transaction: Option<Transaction>,
    /// A doc-comment mentioning selection twice.
    pub selection: Option<Vec<NodeId>>,
    pub table: std::collections::HashMap<String, u8>,
    quiet: Option<u8>,
    pub(crate) middle: Option<u8>,
}
"#;
    assert_eq!(
        effect_fields(source),
        vec!["transaction", "selection", "table", "quiet", "middle"],
        "the parser has to read every field and only the fields: the ones \
         named in prose are not fields, a type with a comma in it is not two \
         fields, and a private field is still a field."
    );
}

/// Naming a field in a comment is not handling it.
///
/// This is not a hypothetical: `ide_app.rs` carries the line
/// `grep -n zoom src/ide_app.rs` inside the comment that explains the bug.
/// Without masking, deleting the zoom arm would leave the gate green because
/// of the comment that documents the very defect.
#[test]
fn quoting_a_field_in_a_comment_is_not_handling_it() {
    let source = r#"
impl Shell {
    fn apply(&mut self, effect: crawrad_actions::Effect) {
        // The zoom used to fall on the floor here: `grep -n effect.zoom`.
        let _ = "effect.zoom";
        if let Some(tx) = effect.transaction {
            drop(tx);
        }
    }
}
"#;
    let appliers = appliers_in("x.rs", source);
    assert_eq!(appliers.len(), 1, "one applier, one function");
    assert_eq!(
        untouched_fields(&appliers[0], &["transaction".into(), "zoom".into()]),
        vec!["zoom".to_string()],
        "a field named only in a comment or a string is a field nobody \
         handles. Quoting is not doing."
    );
}

/// The search is anchored on the binding, not on the field name.
///
/// The egui shell really does write `self.zoom = novo` inside the applier. A
/// gate that looked for the bare word `zoom` would call that handling, and
/// would be green for a shell that never read the effect at all.
#[test]
fn a_field_of_self_does_not_answer_for_a_field_of_the_effect() {
    let source = r#"
impl Shell {
    fn apply_effect(&mut self, e: Effect) {
        self.zoom = 1.0;
        self.zoom_applied = self.zoom;
        self.status = String::new();
        if let Some(t) = e.transaction {
            drop(t);
        }
    }
}
"#;
    let appliers = appliers_in("x.rs", source);
    assert_eq!(
        untouched_fields(
            &appliers[0],
            &["transaction".into(), "zoom".into(), "status".into()]
        ),
        vec!["zoom".to_string(), "status".to_string()],
        "`self.zoom` is the shell's own state, not the effect's field."
    );
}

/// A line break chosen by `rustfmt` does not change the verdict.
#[test]
fn the_verdict_survives_the_formatter() {
    let source = "
impl Shell {
    fn apply(&mut self, effect: Effect) {
        if let Some(z) = effect
            .zoom
        {
            drop(z);
        }
    }
}
";
    let appliers = appliers_in("x.rs", source);
    assert!(
        untouched_fields(&appliers[0], &["zoom".into()]).is_empty(),
        "compacting the body is what makes the search immune to the line \
         break `rustfmt` happens to choose."
    );
}

/// A struct field typed `Effect` is not an applier.
///
/// Reading backwards from the word `Effect` would find a comma before the
/// binding here and mistake the field for a parameter. Reading forward from
/// `fn` cannot.
#[test]
fn a_struct_field_typed_effect_is_not_an_applier() {
    let source = r#"
pub struct Request {
    pub who: String,
    pub effect: Effect,
}

/// A function pointer type is not a declaration either.
pub struct Action {
    pub execute: fn(&Context) -> Effect,
}
"#;
    assert!(
        appliers_in("x.rs", source).is_empty(),
        "only a function that RECEIVES an `Effect` is an applier."
    );
}

/// A signature with no body borrows nobody else's body.
///
/// A trait declares `fn apply(&mut self, e: Effect);` and stops. Looking for
/// "the next `{`" would hand that signature the body of the function below it
/// — and the verdict printed against the trait would be a measurement of
/// somebody else's code, in either direction.
#[test]
fn a_signature_with_no_body_is_not_an_applier() {
    let source = r#"
pub trait Applier {
    fn apply(&mut self, e: Effect);
}

fn other(e: Effect) {
    let _ = e.zoom;
}
"#;
    let names: Vec<String> = appliers_in("x.rs", source)
        .into_iter()
        .map(|a| a.function)
        .collect();
    assert_eq!(names, vec!["other"], "a signature has no body to charge");
}

/// The reference and the full path are the same parameter.
#[test]
fn the_effect_is_recognised_behind_a_reference_and_a_path() {
    let source = r#"
fn a(e: &Effect) { let _ = e.zoom; }
fn b(e: crawrad_actions::Effect) { let _ = e.zoom; }
fn c(e: &mut crawrad_actions::Effect) { let _ = e.zoom; }
fn d(e: Option<Effect>) { let _ = e; }
"#;
    let names: Vec<String> = appliers_in("x.rs", source)
        .into_iter()
        .map(|a| a.function)
        .collect();
    assert_eq!(
        names,
        vec!["a", "b", "c"],
        "`Option<Effect>` is deliberately not an applier — see the doc of \
         `effect_parameter`."
    );
}

// ---------------------------------------------------------------------------
// The variant ruler, measured on purpose-built text
// ---------------------------------------------------------------------------

/// The variants come out of the declaration, in every shape a variant has.
#[test]
fn the_variant_list_is_read_out_of_the_enum() {
    let source = r#"
/// A doc that names `Ghost` without declaring it.
#[derive(Debug)]
pub enum FileEffect {
    Save,
    /// A doc-comment, with `Generate` quoted inside it.
    #[cfg(windows)]
    Reload,
    OpenInEditor(Option<PathInProject>),
    Move { panel: Panel, side: DockSide },
    First = 1,
}

enum Empty {}

pub struct NotAnEnum {
    pub field: u8,
}
"#;
    assert_eq!(
        enums_in(source),
        vec![
            (
                "FileEffect".to_string(),
                vec![
                    "Save".to_string(),
                    "Reload".to_string(),
                    "OpenInEditor".to_string(),
                    "Move".to_string(),
                    "First".to_string(),
                ]
            ),
            ("Empty".to_string(), Vec::new()),
        ],
        "every shape a variant has is one variant: bare, with an attribute in \
         front, with a tuple payload, with a struct body whose commas are not \
         separators, and with a discriminant."
    );
}

/// A field that is not an `Option` of one of our enums is no family at all.
///
/// This is limit 6 of the module header, measured: `Transaction` comes from
/// `crawrad-doc` and `String` from nowhere, and inventing a variant list for
/// either would be the gate guessing.
#[test]
fn only_an_option_of_one_of_our_enums_is_a_family() {
    let source = r#"
pub struct Effect {
    pub transaction: Option<Transaction>,
    pub selection: Option<Vec<NodeId>>,
    pub status: Option<String>,
    pub file: Option<FileEffect>,
    pub raw: FileEffect,
}
"#;
    let declared = vec![(
        "FileEffect".to_string(),
        vec!["Save".to_string(), "Run".to_string()],
    )];
    assert_eq!(
        effect_families(source, &declared),
        vec![Family {
            field: "file".to_string(),
            enum_name: "FileEffect".to_string(),
            variants: vec!["Save".to_string(), "Run".to_string()],
        }],
        "`Option<Vec<NodeId>>` is not a family, and neither is a field that is \
         not an `Option`: the ruler charges what the shell reads out of an \
         `Option` and matches on."
    );
}

/// **An applier that DELEGATES is not charged, and that is the rule.**
///
/// `apply_layout` takes a `LayoutCommand` and matches it — and there the
/// compiler already demands every arm, which is stricter than this gate will
/// ever be. Charging the delegating applier would push the house towards the
/// worse shape: a fat `match` in the shell instead of a typed function.
#[test]
fn delegating_a_family_is_not_stopping_halfway_through_it() {
    let source = r#"
impl Shell {
    fn apply(&mut self, e: Effect) {
        if let Some(command) = e.layout {
            self.apply_layout(command);
        }
        match e.file {
            Some(FileEffect::Save) => self.save(),
            Some(_) => {}
            None => {}
        }
    }
}
"#;
    let families = vec![
        Family {
            field: "layout".into(),
            enum_name: "LayoutCommand".into(),
            variants: vec!["Show".into(), "Hide".into()],
        },
        Family {
            field: "file".into(),
            enum_name: "FileEffect".into(),
            variants: vec!["Save".into(), "Run".into()],
        },
    ];
    let appliers = appliers_in("x.rs", source);
    assert_eq!(
        untouched_variants(&appliers[0], &families),
        vec!["FileEffect::Run".to_string()],
        "the delegated family must not be charged and the half-answered one \
         must be. Anything else and the gate is either blind or noisy."
    );
}

/// Naming a variant in a comment is not handling it.
///
/// `ide_app.rs` really does carry `FileEffect::Run` inside the comment that
/// explains the earlier defect. Without masking, deleting the arm would leave the gate
/// green because of the prose that documents the very defect — the same trap
/// the field ruler already fell into once with `zoom`.
#[test]
fn quoting_a_variant_in_a_comment_is_not_handling_it() {
    let source = r#"
impl Shell {
    fn apply(&mut self, e: Effect) {
        // The `FileEffect::Run` arm used to live here, and it answered nothing.
        let _ = "FileEffect::Run";
        match e.file {
            Some(FileEffect::Save) => self.save(),
            Some(_) => {}
            None => {}
        }
    }
}
"#;
    let families = vec![Family {
        field: "file".into(),
        enum_name: "FileEffect".into(),
        variants: vec!["Save".into(), "Run".into()],
    }];
    let appliers = appliers_in("x.rs", source);
    assert_eq!(
        untouched_variants(&appliers[0], &families),
        vec!["FileEffect::Run".to_string()],
        "a variant named only in a comment or a string is a variant nobody \
         handles. Quoting is not doing."
    );
}

/// The full path answers, and a longer name does not.
///
/// One shell writes `Some(FileEffect::Save)` and the other
/// `Some(crawrad_actions::FileEffect::Save)`; both are the same arm. And a
/// hypothetical `SaveAs` must not answer for `Save`, which is what the
/// boundary on the right is for.
#[test]
fn the_variant_is_recognised_behind_its_full_path_and_only_whole() {
    let source = r#"
fn a(e: Effect) {
    match e.file {
        Some(crawrad_actions::FileEffect::Save) => {}
        Some(FileEffect::SaveAs) => {}
        _ => {}
    }
}
"#;
    let families = vec![Family {
        field: "file".into(),
        enum_name: "FileEffect".into(),
        variants: vec!["Save".into(), "SaveAs".into(), "Run".into()],
    }];
    let appliers = appliers_in("x.rs", source);
    assert_eq!(
        untouched_variants(&appliers[0], &families),
        vec!["FileEffect::Run".to_string()],
        "the fully pathed variant has to answer, and `SaveAs` must not \
         answer for `Save`."
    );
}

/// Replaces `{binding}.{from}` with `{binding}.{to}` — and **only** the whole field.
///
/// `str::replace` does not work: a field whose name is a prefix of another —
/// `zoom` and `ui_zoom` — would be rewritten together, and the mutation would
/// knock down two arms instead of one. The boundary is checked by the following
/// character: letter, digit or `_` means the name continues.
fn replace_field(source: &str, binding: &str, from: &str, to: &str) -> String {
    let needle = format!("{binding}.{from}");
    let mut out = String::with_capacity(source.len());
    let mut rest = source;
    while let Some(i) = rest.find(&needle) {
        let after = rest[i + needle.len()..].chars().next();
        out.push_str(&rest[..i]);
        if after.is_some_and(|c| c.is_alphanumeric() || c == '_') {
            // It's another field that only begins identically.
            out.push_str(&needle);
        } else {
            out.push_str(&format!("{binding}.{to}"));
        }
        rest = &rest[i + needle.len()..];
    }
    out.push_str(rest);
    out
}
