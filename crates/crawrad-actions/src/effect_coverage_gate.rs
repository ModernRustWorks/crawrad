//! The effect coverage gate: **every field of [`Effect`], and every variant of
//! it, is answered by every shell that receives one.**
//!
//! # The defect this module exists to lock
//!
//! [`Effect`] is a struct of nine `Option`s. A shell reads the fields it feels
//! like reading, and **ignoring one costs nothing**: no error, no warning, no
//! note. The action runs, the effect comes back correct, and the shell drops
//! it on the floor in complete silence.
//!
//! It happened, on 15/09/2026, five times at once. The Makepad shell
//! (`crawrad-ide/src/ide_app.rs`) ignored `history`, `file`, `view`,
//! `designer` and `zoom`. On screen: `Ctrl+Z` did not undo, `Ctrl+S` did not
//! save, `F12` did not switch the view, the zoom fired and nothing resized,
//! the control lock did not lock. Nothing in the compiler complained. A
//! reviewer found it by mutation, and only because he was told to go looking.
//!
//! This runs **against** the grain of the rest of the house. In the component
//! model, adding a property to a descriptor makes the Inspector show it by
//! itself — the catalogue is dynamic and new entries pull their own weight. In
//! [`Effect`] it is the opposite: adding a field **charges nobody**, and each
//! new field is one more chance of a silent drop, multiplied by the number of
//! shells (two, today).
//!
//! # The ruler
//!
//! The field list is **read out of the declaration of `Effect` itself**, as
//! source text ([`effect_fields`]). That is the whole point of the module. A
//! hand-written list would be the second maintenance point this gate exists to
//! abolish: the tenth field would slip past the gate exactly the way it slips
//! past the compiler.
//!
//! The set of files charged is derived too. Every function anywhere under
//! `crates/` that **receives an `Effect` as a parameter** is an applier
//! ([`appliers_in`]), and every applier must mention every field as
//! `binding.field` inside its own body. A third shell, or an MCP server, shows
//! up in the charge the day it is written — nobody has to remember to add it
//! here.
//!
//! An applier that legitimately answers only part of the effect goes into
//! [`WAIVERS`], **with the reason written out**. A waiver that no longer
//! matches anything is reported as an error: a dead waiver rots quietly and
//! turns into a forgotten licence.
//!
//! It lives in `crawrad-actions` and not next to the three `portao_*` of
//! `crawrad-doc`, and the argument is written out in the doc of [`crates_root`]:
//! those three are **source hygiene** and name no type; this one is the
//! contract of a single type with its consumers, and it reads that type's own
//! declaration.
//!
//! Comments and strings are masked away before the search ([`mask`]), so
//! **quoting a field is not handling it**. `ide_app.rs` has the literal text
//! `grep -n zoom src/ide_app.rs` inside a comment; without masking, deleting
//! the zoom arm would leave the gate green because of the comment that
//! explains the bug.
//!
//! The body is whitespace-compacted before the search, so the line break that
//! `rustfmt` happens to choose cannot change the verdict — the technique comes
//! from `crawrad-ide/src/test/ide_app/fios.rs`.
//!
//! # The second ruler: the VARIANT
//!
//! The field ruler above measures a field, and a field is an `Option`. So
//!
//! ```text
//! match effect.file {
//!     Some(FileEffect::Save) => self.save(),
//!     Some(_) => self.status = Some("not wired in this shell yet".into()),
//!     None => {}
//! }
//! ```
//!
//! reads `effect.file` and is **green** — while five of the six variants go
//! nowhere. That is not a hypothetical: it is the Makepad shell of 15/09/2026,
//! and it is how `Run` came to be a button that answered nothing.
//! `Some(_)` is exhaustive for `rustc`, so the compiler is silent too, and a
//! variant added to `FileEffect` tomorrow enters the same way — the whole shape
//! this module exists to break, one level down.
//!
//! So a field whose type is an `enum` **declared by this crate**
//! ([`effect_families`]) charges its variants by name, `Enum::Variant`, and the
//! rule is:
//!
//! > an applier that names **any** variant of a family must name **every**
//! > variant of it.
//!
//! The "any" is what keeps DELEGATION out of the charge, and delegation is the
//! better shape, not a loophole. `if let Some(c) = effect.layout {
//! self.apply_layout(cx, c) }` names no `LayoutCommand::…` and is not
//! charged — because the `match` on the other side takes a `LayoutCommand` and
//! **the compiler already demands every arm there**. Same for the egui shell's
//! `zoom_pedido(self.zoom, self.vivo, z)`. What this ruler chases is the arm
//! that stops halfway with a wildcard, which is the only shape `rustc` cannot
//! see.
//!
//! Naming is not wiring, and it is not meant to be: an honest
//! `Some(FileEffect::Close) => self.status = Some("not wired here yet".into())`
//! passes. That is the house rule already — *what has no wire SAYS it has no
//! wire* — and the point of the gate is that the sentence has to be written
//! **per variant**, so a new variant cannot inherit somebody else's excuse.
//!
//! # What this gate does NOT prove, said in full
//!
//! **It is a text test. It proves the call is written, not that it does the
//! right thing.** By construction:
//!
//! 1. It would have caught all five bugs of 15/09/2026, because all five were
//!    *total absence*: the field name appeared nowhere in the shell.
//! 2. It would **not** catch a field handled wrongly. `if let Some(_) =
//!    effect.history {}` passes here, and so does an arm that calls `redo`
//!    where it should call `undo`. This is a net against forgetting, not
//!    against being wrong. A gate that announces itself bigger than it is is
//!    worth less than no gate at all.
//! 3. It does not prove the code runs. The applier may sit behind a `match`
//!    arm that is never reached.
//! 4. It reads field access by name. A shell that destructures
//!    (`let Effect { zoom, .. } = e;`) would be charged as if it had dropped
//!    the field — a false red, which costs one reading of this file, and no
//!    shell does it today. A parameter typed `Option<Effect>` or
//!    `Vec<Effect>`, on the other hand, is **not** recognised as an applier,
//!    and that one is a false green.
//! 5. The variant ruler charges **names**, not arms. An applier that spells
//!    `FileEffect::Run` inside a `matches!` and does nothing with it is green,
//!    and so is one that runs the save on the `Run` arm. Same ceiling as
//!    above, one level down.
//! 6. It only knows the enums **this crate declares**. A field typed with an
//!    enum from `crawrad-doc` — or a struct, or a `String` — is charged for the
//!    field and for nothing else, because the search that finds the variants
//!    ([`enum_variants`]) reads `crawrad-actions/src` and nowhere else. Those
//!    types do not grow when the catalogue grows, which is what this ruler is
//!    watching for.
//! 7. A family can still be dodged by binding first and then writing a
//!    wildcard: `if let Some(a) = e.file { match a { Save => …, _ => … } }`
//!    names one variant under a bare `Save`, with no `FileEffect::` in front,
//!    and the gate sees nothing to charge. It is a *deliberate* rewrite of a
//!    working arm into a worse one, and it arrives in a diff a reviewer reads;
//!    the accidental version — `Some(_)` straight on the field — is the one
//!    that has actually happened twice, and it is covered.
//!
//! [`Effect`]: super::Effect

use std::path::{Path, PathBuf};

/// The source of this crate's `lib.rs`, which is where `Effect` is declared.
///
/// `include_str!` and not a disk read: this way the text the gate parses is,
/// byte for byte, the text the compiler just compiled. There is no window in
/// which the gate measures one version of the struct and the binary carries
/// another.
const ACTIONS_LIB: &str = include_str!("lib.rs");

/// The struct whose fields are charged.
const EFFECT: &str = "Effect";

/// Directories the sweep does not descend into.
///
/// Same list, and same reasons, as the three gates in `crawrad-doc`: `target/`
/// holds generated code that is not ours to judge, `.git/` holds objects and
/// not source, and `.claude/` holds the worktrees of this house's agents —
/// descending there would sweep *other branches* and blame this commit for
/// code that is not its own.
const SKIPPED_DIRS: &[&str] = &["target", ".git", ".claude", "node_modules"];

/// Appliers that are allowed to answer only part of the effect, each with the
/// reason written by hand.
///
/// The entry is `(path from crates/, function name, reason)`. Getting in is
/// expensive on purpose: a path and a sentence in the diff, which a reviewer
/// reads, instead of the cheap and invisible alternative, which is a field
/// quietly missing from a thousand-line file.
///
/// A waiver that matches no applier is an error — see
/// `no_waiver_is_dead` in the tests.
const WAIVERS: &[(&str, &str, &str)] = &[(
    "crawrad-actions/tests/agent_builds_form.rs",
    "apply",
    "The agent of the integration test has no screen and no disk: it is the \
     MCP applier of ADR 0005 reduced to what an agent can observe. Charging \
     it for `status`, `copy_text`, `view`, `designer`, `zoom` or \
     `explorer` would be charging a headless caller for the parts of the \
     effect that only exist because there is a window.",
)];

/// The `crates/` directory — our crates, and only ours.
///
/// Sweeping by PATH and not by dependency is what makes this possible:
/// `crawrad-actions` does not depend on `crawrad-ide` and never will. **Reading
/// the text of a file creates no edge in the graph**, the same argument
/// `crawrad-doc`'s `portao_ingles` has been using for weeks.
///
/// # Why the gate lives here and not in `crawrad-doc`
///
/// `crawrad-doc` already hosts three `portao_*` siblings, and "a gate you can
/// find is a gate you look for" is a real argument. It loses to a stronger
/// one: **those three are source hygiene** — English on screen, no silenced
/// lint, no target-specific command — rules that hold over the whole tree and
/// name no type. This one is the contract of a single type with its consumers,
/// and it reads that type's own declaration. If `Effect` gains a field, is
/// renamed or moves, the gate has to move with it; living in the same file
/// tree as the struct is what makes that a one-file change instead of a
/// cross-crate hunt. `crawrad-actions` also already hosts the mutation ratchet,
/// which is the other gate in the house that guards a design decision rather
/// than a spelling one.
fn crates_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// An applier: a function that receives an [`Effect`] and is therefore
/// responsible for every field of it.
///
/// [`Effect`]: super::Effect
#[derive(Debug, Clone)]
pub(super) struct Applier {
    /// Path from `crates/`, with forward slashes.
    pub(super) file: String,
    /// The name of the function.
    pub(super) function: String,
    /// The name the parameter was given — `effect` in one shell, `e` in the
    /// other. The search is anchored on it, which is what keeps `self.zoom`
    /// from answering for `e.zoom`.
    pub(super) binding: String,
    /// The body of the function, masked and whitespace-compacted.
    pub(super) body: String,
    /// The line the function is declared on, counted from 1.
    pub(super) line: usize,
}

/// The names of the fields of `struct Effect`, in declaration order.
///
/// Reads the masked source, finds the declaration, and splits the body on
/// top-level commas. Masking is what makes a field named inside a doc-comment
/// invisible — every field of `Effect` has one, and several of them quote the
/// names of their neighbours.
///
/// Accepts private fields as well as `pub` ones: a gate that only saw `pub`
/// would publish its own back door.
pub(super) fn effect_fields(source: &str) -> Vec<String> {
    effect_entries(source)
        .into_iter()
        .map(|(name, _)| name)
        .collect()
}

/// The fields of `struct Effect` as **name and declared type**, in declaration
/// order.
///
/// The type is what the variant ruler starts from: `Option<FileEffect>` is a
/// family, `Option<String>` is not. Splitting this out of [`effect_fields`] is
/// what keeps the struct parsed in one place — two parsers over the same
/// declaration is exactly the second maintenance point this module exists to
/// abolish.
pub(super) fn effect_entries(source: &str) -> Vec<(String, String)> {
    let masked = mask(source);
    let head = masked
        .find(&format!("struct {EFFECT}"))
        .unwrap_or_else(|| panic!("`struct {EFFECT}` is gone from the source of crawrad-actions"));
    let open = head
        + masked[head..]
            .find('{')
            .unwrap_or_else(|| panic!("`struct {EFFECT}` has no body"));
    let body = block_at(&masked, open);

    split_top_level(body, ',')
        .into_iter()
        .filter_map(field_entry)
        .collect()
}

/// The name and type declared by one entry of a struct body, if it declares
/// one.
fn field_entry(chunk: &str) -> Option<(String, String)> {
    let mut rest = chunk.trim();
    if let Some(after) = rest.strip_prefix("pub") {
        // `pub`, `pub(crate)`, `pub(super)`, `pub(in path)`.
        rest = match after.strip_prefix('(') {
            Some(inside) => inside.split_once(')')?.1.trim(),
            None => after.trim(),
        };
    }
    let (name, ty) = rest.split_once(':')?;
    let name = name.trim();
    is_ident(name).then(|| (name.to_string(), compact(ty)))
}

/// Every applier declared in `source`.
///
/// Walks the `fn` declarations of the masked text, reads the parameter list of
/// each, and keeps the ones with a parameter typed `Effect` — bare, behind a
/// reference, or fully pathed as `crawrad_actions::Effect`.
///
/// Searching forward from `fn` and not backward from `Effect` is what keeps a
/// *struct field* typed `Effect` from being mistaken for a parameter: a field
/// declaration is preceded by a comma too.
pub(super) fn appliers_in(file: &str, source: &str) -> Vec<Applier> {
    let masked = mask(source);
    let cs: Vec<char> = masked.chars().collect();
    let mut found = Vec::new();
    let mut i = 0usize;

    while i + 2 < cs.len() {
        if !starts_fn_keyword(&cs, i) {
            i += 1;
            continue;
        }
        let Some((function, after_name)) = read_fn_name(&cs, i + 2) else {
            i += 2;
            continue;
        };
        // The first `(` after the name is the parameter list; a generic list
        // between them is skipped by looking for the parenthesis, not by
        // parsing the angle brackets.
        let Some(open) = cs[after_name..].iter().position(|c| *c == '(') else {
            break;
        };
        let open = after_name + open;
        let close = match_delimiter(&cs, open, '(', ')');
        let params: String = cs[open + 1..close].iter().collect();
        let binding = split_top_level(&params, ',')
            .into_iter()
            .find_map(effect_parameter);

        if let Some(binding) = binding {
            // The body opens at the first `{` after the parameter list; a
            // return type cannot contain one. A `;` arriving first means there
            // is no body — a signature in a trait, or an `extern` declaration —
            // and reading on would hand this function the body of the NEXT one,
            // which is a verdict about somebody else's code.
            let body_start = cs[close..].iter().position(|c| *c == '{' || *c == ';');
            if let Some(brace) = body_start.filter(|k| cs[close + k] == '{') {
                let brace = close + brace;
                let end = match_delimiter(&cs, brace, '{', '}');
                let body: String = cs[brace + 1..end].iter().collect();
                found.push(Applier {
                    file: file.to_string(),
                    function,
                    binding,
                    body: compact(&body),
                    line: cs[..i].iter().filter(|c| **c == '\n').count() + 1,
                });
            }
        }
        i = open;
    }
    found
}

/// `true` when `fn` starts a declaration at `i`.
///
/// It is [`starts_keyword`], and the word boundary it demands is what drops the
/// function-pointer type `fn(&Context) -> Effect` of the `Action` struct — a
/// type, not a declaration, and it has no body to charge.
fn starts_fn_keyword(cs: &[char], i: usize) -> bool {
    starts_keyword(cs, i, "fn")
}

/// The name declared after a keyword, and the index just past it.
fn read_fn_name(cs: &[char], from: usize) -> Option<(String, usize)> {
    let start = from + cs[from..].iter().position(|c| !c.is_whitespace())?;
    let mut end = start;
    while end < cs.len() && (cs[end].is_alphanumeric() || cs[end] == '_') {
        end += 1;
    }
    (end > start).then(|| (cs[start..end].iter().collect(), end))
}

/// The binding name, when this parameter is typed `Effect`.
///
/// `Option<Effect>` and `Vec<Effect>` deliberately do not match: they are not
/// what an applier looks like, and pretending to read them would mean guessing
/// which field of a wrapper to charge.
fn effect_parameter(param: &str) -> Option<String> {
    let (name, ty) = param.split_once(':')?;
    let name = name.trim();
    if !is_ident(name) {
        return None;
    }
    let mut ty = ty.trim().trim_start_matches('&').trim();
    if let Some(rest) = ty.strip_prefix("mut ") {
        ty = rest.trim();
    }
    let last = ty.rsplit("::").next()?.trim();
    (last == EFFECT).then(|| name.to_string())
}

/// The fields of `Effect` this applier never mentions.
///
/// The needle is `binding.field`, with a word boundary on both sides. The
/// boundary on the left is what keeps `self.zoom` — a real line of the egui
/// shell — from answering for `e.zoom`; the one on the right is what keeps
/// `self.zoom_aplicado` from answering for `zoom`.
pub(super) fn untouched_fields(applier: &Applier, fields: &[String]) -> Vec<String> {
    fields
        .iter()
        .filter(|field| !mentions(&applier.body, &applier.binding, field))
        .cloned()
        .collect()
}

/// Does the compacted `body` read `binding.field`?
fn mentions(body: &str, binding: &str, field: &str) -> bool {
    contains_word(body, &format!("{binding}.{field}"))
}

/// Does `body` contain `needle` with a word boundary on both sides?
///
/// The boundary on the left is what keeps `self.zoom` — a real line of the
/// egui shell — from answering for `e.zoom`, and what lets the fully pathed
/// `crawrad_actions::FileEffect::Run` answer for `FileEffect::Run` (a `:` is not an
/// identifier byte). The one on the right is what keeps `self.zoom_aplicado`
/// from answering for `zoom`, and `FileEffect::SaveAs` from answering for
/// `FileEffect::Save`.
fn contains_word(body: &str, needle: &str) -> bool {
    let bytes = body.as_bytes();
    body.match_indices(needle).any(|(at, _)| {
        let before_ok = at == 0 || !is_ident_byte(bytes[at - 1]);
        let after = at + needle.len();
        let after_ok = after >= bytes.len() || !is_ident_byte(bytes[after]);
        before_ok && after_ok
    })
}

// ------------------------------------------------------- the variant ruler

/// A field of [`Effect`] whose type is an `enum` **this crate declares**, with
/// every variant it can carry.
///
/// [`Effect`]: super::Effect
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Family {
    /// The field of `Effect` that carries it — `file`, `view`, `zoom`.
    pub(super) field: String,
    /// The name of the enum, as it is written at the call sites: `FileEffect`.
    pub(super) enum_name: String,
    /// Every variant, in declaration order.
    pub(super) variants: Vec<String>,
}

/// Every family of `Effect`: the fields typed `Option<SomeEnumOfOurs>`.
///
/// `source` is the declaration of `Effect` (this crate's `lib.rs`); `declared`
/// is every enum this crate declares, from [`crate_enums`]. Split that way for
/// the same reason the rest of the module is: the parsing is testable on
/// purpose-built text, and the disk sweep happens once.
///
/// A field typed with something that is **not** one of our enums — `String`,
/// `Vec<NodeId>`, `Transaction` — simply produces no family, and is charged by
/// the field ruler alone. See limit 6 in the module header.
pub(super) fn effect_families(source: &str, declared: &[(String, Vec<String>)]) -> Vec<Family> {
    effect_entries(source)
        .into_iter()
        .filter_map(|(field, ty)| {
            let inner = option_inner(&ty)?;
            let name = inner.rsplit("::").next()?.trim();
            let variants = declared
                .iter()
                .find(|(declared_name, _)| declared_name == name)?
                .1
                .clone();
            Some(Family {
                field,
                enum_name: name.to_string(),
                variants,
            })
        })
        .collect()
}

/// The type inside `Option<…>`, or `None` when the type is not an `Option`.
///
/// Anchored on the LAST `<` of the prefix and the matching close, so
/// `Option<Vec<NodeId>>` answers `Vec<NodeId>` and not `Vec<NodeId`.
fn option_inner(ty: &str) -> Option<&str> {
    let ty = ty.trim();
    let inside = ty
        .strip_prefix("Option<")
        .or_else(|| ty.strip_prefix("std::option::Option<"))?;
    inside.strip_suffix('>').map(str::trim)
}

/// Every `enum` declared in `dir` and its variants, in declaration order.
///
/// Swept from disk, and **not** from a hand-written list: `LayoutCommand`
/// lives in `panel.rs` and `ExplorerCommand` in `explorer.rs`, and a list here
/// would be a third place to remember on the day a family moves file. The
/// argument is the same one [`effect_fields`] makes about the field list.
///
/// Files under a `test` directory are skipped. A fixture enum in a test — and
/// the tests below declare several — must not answer for the real one.
pub(super) fn crate_enums(dir: &Path) -> Vec<(String, Vec<String>)> {
    let mut files = Vec::new();
    sweep(dir, &mut files);
    files.sort();
    let mut found = Vec::new();
    for file in files {
        if file
            .components()
            .any(|c| c.as_os_str().to_str() == Some("test"))
        {
            continue;
        }
        let Ok(source) = std::fs::read_to_string(&file) else {
            continue;
        };
        found.extend(enums_in(&source));
    }
    found
}

/// Every `enum` declared in `source`, with its variants.
pub(super) fn enums_in(source: &str) -> Vec<(String, Vec<String>)> {
    let masked = mask(source);
    let cs: Vec<char> = masked.chars().collect();
    let mut found = Vec::new();
    let mut i = 0usize;

    while i + 4 < cs.len() {
        if !starts_keyword(&cs, i, "enum") {
            i += 1;
            continue;
        }
        let Some((name, after_name)) = read_fn_name(&cs, i + 4) else {
            i += 4;
            continue;
        };
        // The body opens at the first `{` after the name. A `;` or a `(`
        // arriving first is not an enum declaration we can read, and guessing
        // is how a gate starts measuring somebody else's braces.
        let Some(k) = cs[after_name..]
            .iter()
            .position(|c| *c == '{' || *c == ';' || *c == '(')
        else {
            break;
        };
        if cs[after_name + k] != '{' {
            i = after_name + k;
            continue;
        }
        let open = after_name + k;
        let end = match_delimiter(&cs, open, '{', '}');
        let body: String = cs[open + 1..end].iter().collect();
        found.push((
            name,
            split_top_level(&body, ',')
                .into_iter()
                .filter_map(leading_ident)
                .collect(),
        ));
        i = end.max(open + 1);
    }
    found
}

/// `true` when `word` starts a declaration at `i`.
///
/// Word boundary on the left and whitespace on the right — the same ruler
/// [`starts_fn_keyword`] uses, and for the same reason: `renum` is not `enum`,
/// and `enums_in` as an identifier is not a declaration either.
fn starts_keyword(cs: &[char], i: usize, word: &str) -> bool {
    let n = word.len();
    if !word
        .chars()
        .enumerate()
        .all(|(k, c)| cs.get(i + k) == Some(&c))
    {
        return false;
    }
    if i > 0 && (cs[i - 1].is_alphanumeric() || cs[i - 1] == '_') {
        return false;
    }
    cs.get(i + n).is_some_and(|c| c.is_whitespace())
}

/// The identifier a variant declaration starts with, skipping its attributes.
///
/// `#[serde(rename = "x")] Salvar` is `Salvar`, and so are `Float(Panel)`,
/// `Move { panel: Panel }` and `Primeiro = 1`.
fn leading_ident(chunk: &str) -> Option<String> {
    let cs: Vec<char> = chunk.chars().collect();
    let mut i = 0usize;
    loop {
        while i < cs.len() && cs[i].is_whitespace() {
            i += 1;
        }
        if cs.get(i) != Some(&'#') {
            break;
        }
        // An attribute: skip to the end of its bracket group. `#![…]` cannot
        // appear on a variant, and `[` is the only thing that can follow — a
        // `#` with no bracket after it is not source we can read, and guessing
        // is how a gate starts measuring somebody else's brackets.
        let k = cs[i..].iter().position(|c| *c == '[')?;
        i = match_delimiter(&cs, i + k, '[', ']') + 1;
    }
    let start = i;
    while i < cs.len() && (cs[i].is_alphanumeric() || cs[i] == '_') {
        i += 1;
    }
    (i > start).then(|| cs[start..i].iter().collect())
}

/// The variants of each family this applier leaves unnamed.
///
/// **A family the applier names nothing of is not charged at all**, and that is
/// the rule, not an oversight: it is what an applier that DELEGATES looks like
/// — `self.apply_layout(cx, command)` — and on the other side of the
/// delegation the compiler already demands every arm. The charge is for the
/// applier that started naming variants and stopped halfway, which is the only
/// shape `rustc` cannot see. See the module header.
pub(super) fn untouched_variants(applier: &Applier, families: &[Family]) -> Vec<String> {
    let mut missing = Vec::new();
    for family in families {
        let named: Vec<&String> = family
            .variants
            .iter()
            .filter(|v| contains_word(&applier.body, &format!("{}::{v}", family.enum_name)))
            .collect();
        if named.is_empty() || named.len() == family.variants.len() {
            continue;
        }
        missing.extend(
            family
                .variants
                .iter()
                .filter(|v| !named.contains(v))
                .map(|v| format!("{}::{v}", family.enum_name)),
        );
    }
    missing
}

/// Every `.rs` file under `dir`, skipping [`SKIPPED_DIRS`].
pub(super) fn sweep(dir: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| SKIPPED_DIRS.contains(&n))
            {
                continue;
            }
            sweep(&path, found);
        } else if path.extension().is_some_and(|x| x == "rs") {
            found.push(path);
        }
    }
}

/// Every applier of the whole tree, in a stable order.
///
/// Files that do not even spell `Effect` are skipped before masking, which is
/// what keeps this from being a full lexical pass over two hundred files.
pub(super) fn all_appliers() -> Vec<Applier> {
    let root = crates_root();
    let mut files = Vec::new();
    sweep(&root, &mut files);
    files.sort();

    let mut found = Vec::new();
    for file in files {
        let Ok(source) = std::fs::read_to_string(&file) else {
            continue;
        };
        if !source.contains(EFFECT) {
            continue;
        }
        found.extend(appliers_in(&short_path(&file, &root), &source));
    }
    found
}

/// Every family of `Effect`, read from this crate's own source.
///
/// The declaration of `Effect` comes from [`actions_lib`] — compiled text, the
/// invariant the module header claims — and the enums from a sweep of
/// `crawrad-actions/src`, because two of them do not live in `lib.rs`.
pub(super) fn all_families() -> Vec<Family> {
    effect_families(actions_lib(), &crate_enums(&actions_src()))
}

/// The `src/` of this crate — where the families are declared.
fn actions_src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Is this applier waived, and by which reason?
pub(super) fn waiver(applier: &Applier) -> Option<&'static str> {
    WAIVERS
        .iter()
        .find(|(file, function, _)| *file == applier.file && *function == applier.function)
        .map(|(_, _, reason)| *reason)
}

/// The path of a file from the root of the crates, with forward slashes.
fn short_path(file: &Path, root: &Path) -> String {
    file.strip_prefix(root)
        .unwrap_or(file)
        .display()
        .to_string()
        .replace('\\', "/")
}

/// `source` with every run of whitespace collapsed to one space, and with the
/// spaces around a `.` removed.
///
/// Collapsing is what makes the search immune to `rustfmt`: `effect.zoom` and
/// `effect\n            .zoom` become the same string.
///
/// **Not** removing whitespace altogether, which is what `fios.rs` does and
/// what this function did on its first draft: erasing every space welds the
/// words together, `match effect.history` becomes `matcheffect.history`,
/// and the word boundary the search needs on the left of the binding is gone.
/// Measured: the first draft accused three fields that the Makepad shell has
/// been handling all along — every one of them read by a `match` instead of an
/// `if let`. `fios.rs` does not feel it because it looks for whole phrases;
/// here the boundary is the entire ruler.
fn compact(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut pending_space = false;
    for c in source.chars() {
        if c.is_whitespace() {
            pending_space = !out.is_empty();
            continue;
        }
        if pending_space {
            if c != '.' && !out.ends_with('.') {
                out.push(' ');
            }
            pending_space = false;
        }
        out.push(c);
    }
    out
}

/// Is `text` a single Rust identifier?
fn is_ident(text: &str) -> bool {
    !text.is_empty()
        && !text.starts_with(|c: char| c.is_ascii_digit())
        && text.chars().all(|c| c.is_alphanumeric() || c == '_')
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b >= 0x80
}

/// The text between the brace at `open` and the brace that closes it.
///
/// Works on bytes because `open` came from `str::find`, which counts bytes,
/// and a brace is always one byte wide.
fn block_at(source: &str, open: usize) -> &str {
    let bytes = source.as_bytes();
    let mut depth = 0usize;
    for (i, b) in bytes.iter().enumerate().skip(open) {
        match b {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return &source[open + 1..i];
                }
            }
            _ => {}
        }
    }
    &source[open + 1..]
}

/// The index of the delimiter that closes the one at `open`, or the end.
fn match_delimiter(cs: &[char], open: usize, opener: char, closer: char) -> usize {
    let mut depth = 0usize;
    for (i, c) in cs.iter().enumerate().skip(open) {
        if *c == opener {
            depth += 1;
        } else if *c == closer {
            depth -= 1;
            if depth == 0 {
                return i;
            }
        }
    }
    cs.len()
}

/// Splits `source` on `sep`, ignoring separators nested in any bracket.
///
/// Angle brackets count, because `Option<Vec<NodeId>>` is a type and
/// `HashMap<K, V>` would otherwise be read as two fields.
fn split_top_level(source: &str, sep: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in source.char_indices() {
        match c {
            '(' | '[' | '{' | '<' => depth += 1,
            ')' | ']' | '}' | '>' => depth -= 1,
            _ if c == sep && depth == 0 => {
                parts.push(&source[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    if start < source.len() {
        parts.push(&source[start..]);
    }
    parts
}

/// Blanks out everything that is **not code**, keeping line breaks.
///
/// Every character of a string, a char literal or a comment becomes a space;
/// `\n` always survives, so the line number of a finding stays the line number
/// of the file. This is the same job `crawrad-doc`'s `portao_do_lint_silenciado`
/// does for its own search, written again here on purpose: that one lives in a
/// `#[cfg(test)]` module of a crate that has nothing to do with `Effect`, and
/// promoting a test-only tool to the public API of `crawrad-doc` would be a
/// bigger change to someone else's gate than a lexer is worth.
///
/// What it understands, because the tree uses all of it:
///
/// - `//`, `///`, `//!` to the end of the line;
/// - `/* … */` **nested**, as Rust requires;
/// - `"…"` with `\"`;
/// - `r"…"`, `r#"…"#`, `r##"…"##`, and the byte forms;
/// - `'a'` and `'\''` as literals, but `'a` as a lifetime is **not** — reading
///   `&'static str` as the start of a literal would blank the rest of the file
///   and a blind gate stays green forever.
pub(super) fn mask(source: &str) -> String {
    let cs: Vec<char> = source.chars().collect();
    let n = cs.len();
    let mut out = String::with_capacity(source.len());
    let mut i = 0usize;

    while i < n {
        let c = cs[i];

        if c == '/' && cs.get(i + 1) == Some(&'/') {
            let mut j = i;
            while j < n && cs[j] != '\n' {
                j += 1;
            }
            blank(&cs, i, j, &mut out);
            i = j;
            continue;
        }

        if c == '/' && cs.get(i + 1) == Some(&'*') {
            let mut j = i + 2;
            let mut depth = 1usize;
            while j < n && depth > 0 {
                if cs[j] == '/' && cs.get(j + 1) == Some(&'*') {
                    depth += 1;
                    j += 2;
                } else if cs[j] == '*' && cs.get(j + 1) == Some(&'/') {
                    depth -= 1;
                    j += 2;
                } else {
                    j += 1;
                }
            }
            let j = j.min(n);
            blank(&cs, i, j, &mut out);
            i = j;
            continue;
        }

        if let Some(end) = end_of_raw_string(&cs, i) {
            blank(&cs, i, end, &mut out);
            i = end;
            continue;
        }
        if let Some(end) = end_of_string(&cs, i) {
            blank(&cs, i, end, &mut out);
            i = end;
            continue;
        }
        if let Some(end) = end_of_char(&cs, i) {
            blank(&cs, i, end, &mut out);
            i = end;
            continue;
        }

        out.push(c);
        i += 1;
    }
    out
}

/// Copies `from..to` as spaces, keeping the line breaks.
fn blank(cs: &[char], from: usize, to: usize, out: &mut String) {
    for c in &cs[from..to] {
        out.push(if *c == '\n' { '\n' } else { ' ' });
    }
}

/// The index just past a raw string starting at `i`, if one starts there.
fn end_of_raw_string(cs: &[char], i: usize) -> Option<usize> {
    if i > 0 && (cs[i - 1].is_alphanumeric() || cs[i - 1] == '_') {
        return None;
    }
    let mut j = i;
    if cs.get(j) == Some(&'b') {
        j += 1;
    }
    if cs.get(j) != Some(&'r') {
        return None;
    }
    j += 1;
    let mut hashes = 0usize;
    while cs.get(j) == Some(&'#') {
        hashes += 1;
        j += 1;
    }
    if cs.get(j) != Some(&'"') {
        return None;
    }
    j += 1;
    while j < cs.len() {
        if cs[j] == '"' && (1..=hashes).all(|k| cs.get(j + k) == Some(&'#')) {
            return Some(j + 1 + hashes);
        }
        j += 1;
    }
    Some(cs.len())
}

/// The index just past a plain string starting at `i`, if one starts there.
fn end_of_string(cs: &[char], i: usize) -> Option<usize> {
    let mut j = i;
    if cs.get(j) == Some(&'b') {
        if i > 0 && (cs[i - 1].is_alphanumeric() || cs[i - 1] == '_') {
            return None;
        }
        j += 1;
    }
    if cs.get(j) != Some(&'"') {
        return None;
    }
    j += 1;
    while j < cs.len() {
        match cs[j] {
            '\\' => j += 2,
            '"' => return Some(j + 1),
            _ => j += 1,
        }
    }
    Some(cs.len())
}

/// The index just past a char literal starting at `i`.
///
/// **Rust's single quote is ambiguous**: `'a'` is a literal, `&'a str` is a
/// lifetime and `'outer:` is a loop label. The ruler: with a backslash after
/// the quote it is a literal; without one, it is a literal only when the
/// closing quote is the third character.
fn end_of_char(cs: &[char], i: usize) -> Option<usize> {
    if cs.get(i) != Some(&'\'') {
        return None;
    }
    if cs.get(i + 1) == Some(&'\\') {
        let limit = (i + 16).min(cs.len());
        let inside = cs.get(i + 2..limit)?;
        return inside.iter().position(|c| *c == '\'').map(|k| i + 3 + k);
    }
    (cs.get(i + 2) == Some(&'\'')).then_some(i + 3)
}

/// The source of this crate's `lib.rs`, for the tests.
pub(super) fn actions_lib() -> &'static str {
    ACTIONS_LIB
}

#[cfg(test)]
#[path = "test/effect_coverage_gate/test.rs"]
mod tests;
