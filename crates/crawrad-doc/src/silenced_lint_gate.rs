//! The silenced lint gate: **compiler warnings are fixed, not silenced.**
//!
//! The owner's rule, 15/09/2026:
//!
//! > *"I do not want any such workaround making the compiler
//! > allow it. Rust has plenty of tools to suggest the right code, instead
//! > of working around the compiler."*
//!
//! # The disease this module cures
//!
//! An agent here needed to measure its own crate under
//! `cargo clippy --workspace -- -D warnings` and encountered two warnings in
//! a file from ANOTHER track (`crawrad-runtime-makepad/src/designer/regua.rs`).
//! Instead of measuring only its own crate, **it silenced both lints in the
//! other file** — "temporarily", promising to restore them via copy at the end.
//! The restoration did not happen. Hours later, another agent found the attributes
//! there, removed them, and fixed the lints for real — and it was **while fixing them**
//! that a `u32` overflow surfaced, which the silence would have buried.
//!
//! The two errors are independent, and the second is the serious one:
//!
//! 1. the restoration failed (an accident, and accidents happen);
//! 2. the practice was wrong **even if the restoration had succeeded**. The
//!    way to measure one's own crate is `cargo clippy -p <the-crate>`, which
//!    does not require anyone's permission. Silencing someone else's warning to make the
//!    workspace pass trades a true verdict for a bought green.
//!
//! And the loophole was structural: the "no `#[allow]`" rule lived only in briefings
//! and reviewer definitions. A `grep` across the tree found no guard
//! rejecting it. **A rule written in prose is a request**; this module is the
//! same rule as a test — it enforces it in `cargo test`, in the commit that
//! writes the attribute.
//!
//! # The rule
//!
//! Rejected is any **actual attribute**, in any `.rs` under `crates/`,
//! that relaxes a lint: `#[allow(…)]`, `#![allow(…)]` and the same two with
//! `expect` in place of `allow` (see [`ATRIBUTOS_QUE_AFROUXAM`]). Allowed
//! are `deny`, `warn`, and `forbid`, which tighten — the `#![forbid(unsafe_code)]` at
//! the top of this crate's `lib.rs` is a living example.
//!
//! The real work is not finding the word: it is knowing **when it is an
//! attribute**. On 15/09/2026, the tree had 21 occurrences of `allow(` outside
//! this gate, and **none** was our suppression — they are the prelude that the
//! generator WRITES inside a string, for the user's project, and prose
//! explaining the prohibition. A gate flagging them would be useless on day
//! one.
//!
//! That is why the recognizer is not a line `grep`: it is a lexical scanner
//! that erases everything from the source that is not code ([`mascarar`]) — string
//! bodies (raw, with or without `#`, byte or not), comment bodies,
//! doc-comments, and character literals — and only then looks for the attribute in
//! what remains. This is the same requirement that `target_command_gate`
//! solves in its own way (there, `cargo tree` passes and `cargo clippy` does not);
//! here the distinction is by **context**, not verb, because the flagged text
//! and the innocent text are **byte-for-byte identical**:
//!
//! ```text
//! crates/crawrad-codegen/tests/gate_fase3.rs:218  →  #![allow(non_snake_case)]
//! ```
//!
//! That line starts at column 1, with no indentation, and is legitimate:
//! it is inside an `r##"…"##` representing the user's file. No
//! line-based rule distinguishes that from a suppression; only lexical context does.
//!
//! **This is why this file does not need to exclude itself from the scan**,
//! unlike its two sibling gates, which assemble the needle dynamically so they
//! do not flag themselves. Here it is written out in full, dozens of times, in prose and
//! literals — and the gate stays green. If one day it turns red pointing
//! to itself, the news is great: the masker broke and announced it loudly,
//! instead of staying green out of blindness.
//!
//! # Tests are also checked, and the reason why
//!
//! There is no exception for `#[cfg(test)]`, for `tests/`, or for `examples/`.
//! A silenced lint in a test hides the exact same defect, and hides it
//! in the place where it would be cheapest to find: `dead_code` in a test is
//! almost always a test case that stopped being called. It costs nothing today — none
//! of our tests has a single such attribute — and the day to decide is today,
//! not the day when one of them becomes convenient.
//!
//! # What it does NOT check, stated in full
//!
//! 1. **Generated sample apps outside `crates/`.** Left out entirely, the only Rust code
//!    in the repository that is. That is where **generated** projects live —
//!    `formPrincipal.designer.rs` has an actual `#[allow(non_snake_case, dead_code,
//!    clippy::needless_update)]`, an actual attribute and not a string. It is the
//!    prelude chosen for the USER's project: the component name comes from
//!    them and might not be `snake_case`. Checking it would mean enforcing our
//!    rule on the user. The scan stops at `crates/` for the same reason
//!    `portao_ingles` stops there.
//! 2. **What the generator emits.** By the same reasoning, and it already passes
//!    on its own: the prelude lives inside literals
//!    (`crawrad-codegen/src/targets/egui/form.rs:175` and `stubs.rs:150,153`).
//! 3. **`#[allow]` written in a `Cargo.toml`, `.yml`, or `.md`.** This
//!    gate reads `.rs`. Lint configuration via manifest (`[lints]`) is
//!    another matter, with another discussion, and is not what happened.
//! 4. **Attributes assembled by macro** — a `concat!`/`stringify!` that
//!    produces the text, or a macro expanding it from fragments. Reading
//!    that requires expansion, not lexical scanning. Nobody here wrote such a thing, and
//!    anyone who does would be bypassing the gate **on purpose**, which is a
//!    different issue from a rushed agent.
//! 5. **Whether the remaining lint is properly fixed.** The gate guarantees that
//!    nobody silenced it; it does not guarantee that the fix was good. That is enforced by
//!    running `clippy`.

use std::path::{Path, PathBuf};

/// Lint-level attributes that **relax** checks, and are therefore forbidden.
///
/// Two, and the list is closed by the language, not by us: `rustc` has
/// exactly five lint-level attributes — `allow`, `expect`, `warn`,
/// `deny`, and `forbid` — and only these two let the warning pass silently. The
/// other three tighten the rule and are welcome.
///
/// `expect` is included **because it is not an oversight**: it is the same
/// silence under another name (warns if the lint does not trigger, and silences when
/// it does). A guard forbidding only `allow` would be publishing its
/// own backdoor — and a known backdoor is worse than no backdoor
/// at all, because its green status becomes an argument.
pub(super) const ATRIBUTOS_QUE_AFROUXAM: &[&str] = &["allow", "expect"];

/// Directories into which the scan does not descend.
///
/// `target/` because generated code lives there — that of `gate_fase3` and
/// third-party dependencies, which fill `wgl_bindings.rs` with attributes
/// that are not ours to judge. `.git/` because git objects are not source code.
/// `.claude/` because **that is where worktrees of agents here live**:
/// descending into them would scan OTHER branches and flag this commit for code
/// that does not belong to it. It is the same list as in `portao_do_comando_por_alvo`, and for the
/// same reasons.
pub(super) const PASTAS_FORA: &[&str] = &["target", ".git", ".claude", "node_modules"];

/// Files in which a silencing attribute is accepted, **with the reason
/// written by hand**.
///
/// It is empty, and that is the news: on 15/09/2026 the entire tree was measured
/// and there is not **a single** suppression of ours. The list exists so that opting out is
/// expensive and visible — a path and an explanation in the diff, read by a reviewer — instead
/// of cheap and invisible, like an attribute in the middle of a thousand-line
/// file.
///
/// The criterion for inclusion: the lint must be **wrong about this code**, and the
/// text must state why. "To make the gate pass" is not a reason; for
/// that, `cargo clippy -p <your-crate>` exists.
///
/// An exception that no longer matches anything is reported as an error by the test itself:
/// a dead exception rots silently and becomes a forgotten license.
pub(super) const EXCECOES: &[(&str, &str)] = &[];

/// The `crates/` directory — our crates, and only them.
///
/// Scanning by PATH rather than by dependency is what makes this possible:
/// `crawrad-doc` does not depend on `crawrad-codegen` or the shell, and will not start
/// depending on them. **Reading file text does not create a link in the graph** — this is the argument
/// that `portao_ingles` already uses two directories up.
///
/// Why the gate lives in `crawrad-doc`: because its two siblings live here, and
/// a guard one looks for is a guard one finds — `ls crates/crawrad-doc/src`
/// shows the three `portao_*` side by side. `crawrad-actions` was considered,
/// as it already hosts the mutation ratchet, and rejected: its ratchet addresses the
/// **actions catalog**, which is a design matter; this gate addresses
/// source hygiene, which is the concern of the other two. Changing its home would not
/// change a single line of scanning; it would only change where the next person looks.
pub(super) fn raiz_dos_crates() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// Erases everything from the source that is **not code**, preserving line and column.
///
/// Every character in a string, char, or comment becomes a space;
/// `\n` is always preserved, so that the match's line number remains
/// the file's line number. What comes out here has the same layout as the
/// original with whitespace blanks, and [`lints_silenciados`] searches
/// over it.
///
/// What it understands, because the tree uses all of this:
///
/// - `//`, `///`, `//!` to the end of the line;
/// - `/* … */` **nested**, as Rust requires;
/// - `"…"` with `\"` and line continuation with `\` at the end;
/// - `r"…"`, `r#"…"#`, `r##"…"##` — this is the form in `gate_fase3.rs`, and is the
///   only one where the flagged text and the innocent text are identical;
/// - `b"…"` and `br#"…"#`;
/// - `'a'` and `'\''` as literals, but lifetime `'a` **is not**: treating
///   `&'static str` as the opening of a literal would erase the rest of the file up to
///   the next single quote, and a blind gate stays green forever.
pub(super) fn mascarar(fonte: &str) -> String {
    let cs: Vec<char> = fonte.chars().collect();
    let n = cs.len();
    let mut saida = String::with_capacity(fonte.len());
    let mut i = 0usize;

    while i < n {
        let c = cs[i];

        // Line comment — `//`, `///`, and `//!` are the same to us.
        if c == '/' && cs.get(i + 1) == Some(&'/') {
            let mut j = i;
            while j < n && cs[j] != '\n' {
                j += 1;
            }
            apagar(&cs, i, j, &mut saida);
            i = j;
            continue;
        }

        // Block comment, with nesting.
        if c == '/' && cs.get(i + 1) == Some(&'*') {
            let mut j = i + 2;
            let mut profundidade = 1usize;
            while j < n && profundidade > 0 {
                if cs[j] == '/' && cs.get(j + 1) == Some(&'*') {
                    profundidade += 1;
                    j += 2;
                } else if cs[j] == '*' && cs.get(j + 1) == Some(&'/') {
                    profundidade -= 1;
                    j += 2;
                } else {
                    j += 1;
                }
            }
            let j = j.min(n);
            apagar(&cs, i, j, &mut saida);
            i = j;
            continue;
        }

        if let Some(fim) = fim_da_string_crua(&cs, i) {
            apagar(&cs, i, fim, &mut saida);
            i = fim;
            continue;
        }

        if let Some(fim) = fim_da_string(&cs, i) {
            apagar(&cs, i, fim, &mut saida);
            i = fim;
            continue;
        }

        if let Some(fim) = fim_do_caractere(&cs, i) {
            apagar(&cs, i, fim, &mut saida);
            i = fim;
            continue;
        }

        saida.push(c);
        i += 1;
    }
    saida
}

/// Copies `de..ate` as spaces, except for newlines.
fn apagar(cs: &[char], de: usize, ate: usize, saida: &mut String) {
    for c in &cs[de..ate] {
        saida.push(if *c == '\n' { '\n' } else { ' ' });
    }
}

/// The index right after a raw string starting at `i`, if it starts one.
///
/// Recognizes `r"`, `r#"`, `r##"`… and byte versions (`br#"`). The left boundary
/// check ensures that an `r` at the end of an identifier
/// (`let separador = "…"`) is not read as a prefix; and requiring a quote
/// after the `#` is what causes the raw identifier `r#fn` to be excluded on its own.
pub(super) fn fim_da_string_crua(cs: &[char], i: usize) -> Option<usize> {
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
    let mut cercas = 0usize;
    while cs.get(j) == Some(&'#') {
        cercas += 1;
        j += 1;
    }
    if cs.get(j) != Some(&'"') {
        return None;
    }
    j += 1;
    // The closing delimiter is a quote followed by exactly the same hashes.
    while j < cs.len() {
        if cs[j] == '"' && (1..=cercas).all(|k| cs.get(j + k) == Some(&'#')) {
            return Some(j + 1 + cercas);
        }
        j += 1;
    }
    // Without a closing delimiter, the file does not compile; erasing to the end is what `rustc`
    // would do and is the safe side for us (the complaint drops, the build has already failed).
    Some(cs.len())
}

/// The index right after a regular string starting at `i`, if it starts one.
pub(super) fn fim_da_string(cs: &[char], i: usize) -> Option<usize> {
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

/// The index right after a character literal starting at `i`.
///
/// **Rust's single quote is ambiguous**, and that is where the risk of a
/// blind gate lies: `'a'` is a literal, `&'a str` is a lifetime, and `'outer:` is
/// a loop label. The rule: with a backslash after the quote, it is a literal
/// (`'\''`, `'\n'`, `'\u{1F600}'`); without it, it is only a literal if the closing
/// quote is at the third character. Anything else is a lifetime,
/// and the quote remains as code.
pub(super) fn fim_do_caractere(cs: &[char], i: usize) -> Option<usize> {
    if cs.get(i) != Some(&'\'') {
        return None;
    }
    if cs.get(i + 1) == Some(&'\\') {
        // `\u{1F600}` is the longest escape in the language; the margin is generous and the
        // limit exists only to avoid scanning the entire file by mistake.
        let limite = (i + 16).min(cs.len());
        let miolo = cs.get(i + 2..limite)?;
        return miolo.iter().position(|c| *c == '\'').map(|k| i + 3 + k);
    }
    (cs.get(i + 2) == Some(&'\'')).then_some(i + 3)
}

/// Silencing attributes in a source: the line (1-based) and the name.
///
/// Searches within the **masked** text, and only inside the brackets of a `#[…]` or
/// `#![…]`. Staying inside the brackets is what keeps `x.expect("…")` — which has
/// dozens of legitimate calls per file here — from becoming a violation, without
/// needing any handwritten exception.
///
/// Inspects the ENTIRE attribute, not just the name right after the bracket: that is
/// how `#[cfg_attr(test, allow(dead_code))]` gets caught. It is the same silence under
/// an extra layer, and a gate that let it pass would be teaching a
/// workaround.
pub(super) fn lints_silenciados(fonte: &str) -> Vec<(usize, String)> {
    let mascarado = mascarar(fonte);
    let cs: Vec<char> = mascarado.chars().collect();
    let mut achados = Vec::new();
    let mut linha = 1usize;
    let mut i = 0usize;

    while i < cs.len() {
        if cs[i] == '\n' {
            linha += 1;
            i += 1;
            continue;
        }
        let abre = if cs[i] == '#' && cs.get(i + 1) == Some(&'[') {
            Some(i + 1)
        } else if cs[i] == '#' && cs.get(i + 1) == Some(&'!') && cs.get(i + 2) == Some(&'[') {
            Some(i + 2)
        } else {
            None
        };
        let Some(abre) = abre else {
            i += 1;
            continue;
        };
        let fecha = fim_do_colchete(&cs, abre);
        let mut linha_do_achado = linha;
        for j in (abre + 1)..fecha {
            if cs[j] == '\n' {
                linha_do_achado += 1;
                continue;
            }
            if let Some(nome) = nome_que_afrouxa(&cs, j) {
                achados.push((linha_do_achado, nome));
            }
        }
        // `#[…]` can span multiple lines (`rustfmt` breaks long ones);
        // the counter must traverse it, not blindly jump past it.
        for c in &cs[i..fecha] {
            if *c == '\n' {
                linha += 1;
            }
        }
        i = fecha;
    }
    achados
}

/// The index of the `]` closing the bracket opened at `abre`, or the end of the source.
fn fim_do_colchete(cs: &[char], abre: usize) -> usize {
    let mut profundidade = 0usize;
    for (j, c) in cs.iter().enumerate().skip(abre) {
        match c {
            '[' => profundidade += 1,
            ']' => {
                profundidade -= 1;
                if profundidade == 0 {
                    return j;
                }
            }
            _ => {}
        }
    }
    cs.len()
}

/// The relaxing name, if `j` is its start followed by `(`.
///
/// Requires boundaries on both sides: no letter, digit, or `_` before, and a
/// `(` after (possibly with spaces in between, which `rustfmt` sometimes adds). This is what
/// separates `allow(` from `allow_isto(` and `desallow(`.
fn nome_que_afrouxa(cs: &[char], j: usize) -> Option<String> {
    if j > 0 && (cs[j - 1].is_alphanumeric() || cs[j - 1] == '_') {
        return None;
    }
    for nome in ATRIBUTOS_QUE_AFROUXAM {
        let letras: Vec<char> = nome.chars().collect();
        if cs.len() < j + letras.len() || cs[j..j + letras.len()] != letras[..] {
            continue;
        }
        let depois = cs[j + letras.len()..]
            .iter()
            .find(|c| !c.is_whitespace())
            .copied();
        if depois == Some('(') {
            return Some((*nome).to_string());
        }
    }
    None
}

/// Every `.rs` in a directory, descending into subdirectories except those in
/// [`PASTAS_FORA`].
pub(super) fn varrer(dir: &Path, achados: &mut Vec<PathBuf>) {
    let Ok(entradas) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entradas.flatten() {
        let p = e.path();
        if p.is_dir() {
            if p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| PASTAS_FORA.contains(&n))
            {
                continue;
            }
            varrer(&p, achados);
        } else if p.extension().is_some_and(|x| x == "rs") {
            achados.push(p);
        }
    }
}

/// The file path relative to the crates root, with forward slashes.
pub(super) fn caminho_curto(arquivo: &Path, raiz: &Path) -> String {
    arquivo
        .strip_prefix(raiz)
        .unwrap_or(arquivo)
        .display()
        .to_string()
        .replace('\\', "/")
}

#[cfg(test)]
#[path = "test/silenced_lint_gate/test.rs"]
mod tests;
