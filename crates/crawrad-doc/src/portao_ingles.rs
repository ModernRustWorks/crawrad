//! The English gate: **every error message the user reads originates in
//! English**.
//!
//! House policy is from 30/08/2026 — comments, doc, and ADR in
//! PORTUGUESE; **strings reaching the screen, in ENGLISH**. The title above is the LAW;
//! what this gate **covers** of it is stated in full in the "What it does
//! NOT enforce" section at the end of this block. Reading both is mandatory for anyone
//! relying on it: a guard that advertises itself as bigger than it is is worth less than
//! no guard at all, because its green status becomes an argument.
//!
//! The gate scans **the entire workspace** in two passes, and each of them
//! was born from a rejection:
//!
//! 1. **the text of the `#[error("…")]` attribute**, across the entire `crates/`. The
//!    first draft covered only `crawrad-doc` and `crawrad-ir`, and even so
//!    advertised itself as total; the two omitted folders had four messages in
//!    Portuguese, and one reached the screen attached to an English prefix —
//!    `cannot plan: 'Botão OK' não é um identificador Rust válido`, in the
//!    Explorer;
//! 2. **the literal passed to whatever only forwards**. `#[error("{0}")]` is not a
//!    message, it is plumbing: the phrase is born at the call site, three
//!    files ahead. The attribute of `RuntimeError::Backend` was
//!    flawless and the bar said `runtime: documento sem raiz`, written in
//!    `crawrad-runtime-egui/src/lib.rs:671`. The second pass finds the variant in
//!    the first and goes after the literals passed to it.
//!
//! A gate that promises more than it covers is the "green becomes an argument" that
//! this very module condemns right below. Two consecutive rejections for the same
//! defect — coverage smaller than the promise — and both in places that the list of
//! limits did not name. Hence why it is today what it is: exhaustive, and with the loophole
//! declared before the reviewer finds it.
//!
//! Where each one appears, and why all of them count:
//!
//! - `IrError`, `ContractError`, and `DocError` — `rejected: <mensagem>` in the bar
//!   (`crawrad-ide/src/main.rs:367` and `:1853`), the raw phrase in `undo`/`redo`
//!   with an empty stack (`:1914` and `:1931`), and `Document::load`/`save` joined with
//!   the file path (`:1191` and `:2960`);
//! - `crawrad_codegen::Erro` — `cannot plan: <mensagem>` in the Explorer column
//!   (`crawrad-ide/src/explorer.rs:640`) and `generate: <mensagem>` in the bar
//!   (`main.rs:2289` and `:2346`);
//! - `crawrad_runtime::RuntimeError` — `runtime: <mensagem>` in the bar
//!   (`main.rs:1848` and `:3941`).
//!
//! These are the words the user reads **when something goes wrong**, which is the
//! worst moment for them not to understand.
//!
//! Twenty-nine phrases arrived in Portuguese until 07/09/2026 — 25 in attributes
//! and 4 at call sites — because **there was no guard whatsoever**: each batch
//! added the next, and the ruler only appeared in a review, months later.
//! This module is the ruler in the right place — it checks in `cargo test`, in the commit
//! that writes the message.
//!
//! # Why an ALLOWED word list, and not a forbidden one
//!
//! The obvious criterion is accents: rejecting non-ASCII. It catches `não`, `nó`,
//! `transação` — and **lets through** `nome invalido`, `id duplicado`, `remova antes`,
//! which are Portuguese without a single accent. A filter that catches half the problem
//! is worse than none, because it grants permission: green becomes an argument.
//!
//! The second criterion is a list of forbidden Portuguese words (`não`,
//! `para`, `que`, `uma`). It catches more, and errs in the same direction: whoever writes
//! the 23rd message will not remember to add to the list the word they themselves
//! just used. A forbidden list ages silently.
//!
//! The criterion of this gate is the inverse, and it is the only one that **fails on the
//! safe side**: every prose word must be in [`LEXICO`]. A new word in
//! English costs one line in the list — visible in the diff, deliberate, the right
//! amount of friction. A Portuguese word **does not pass by accident**: it would
//! have to be written in the middle of a list of English words, by hand,
//! and nobody does that without noticing. It is the same design as `COMMON` in ADR 0002:
//! the door is narrow and only opens in writing.
//!
//! What is **not** prose passes freely, and without hand-written exceptions:
//! `{placeholder}` vanishes before counting; tokens with digits, `_`, `::`, or `.` are
//! code (`crawrad_ir::contract::COMMON`, `win32.{0}`); CamelCase tokens are
//! identifiers (`CrawRAD`, `RemoveNode`). UPPERCASE is **not** an escape —
//! `COMMON` counts as `common` and is checked in the lexicon, otherwise shouting
//! `TODOS OS RUNTIMES` would be enough to cross the gate.
//!
//! # What it does NOT enforce, stated in full
//!
//! The list is short on purpose, and is here **in full** so that nobody reads
//! the title of this module as a promise bigger than what it fulfills:
//!
//! 1. **Payload that is not a literal.** The second pass reads
//!    `Backend("texto")`, and only that. `Backend(e.to_string())`,
//!    `Backend(format!("…"))`, or `Backend(msg)` pass silently, and silence
//!    is deliberate only for the first case: `Backend` exists to wrap
//!    **external** phrases — from the toolkit, from the operating system —, which are not
//!    ours to translate, and flagging them would be a guaranteed false positive at each
//!    legitimate site. Our own `format!` escapes along with it, and that is the real
//!    loophole. Closing it requires reading expressions, not text: a `format!` with an
//!    inner literal is recognizable, but `Backend(frase_de_erro(x))` is not without a
//!    syntax tree — and `syn` as a core test dependency is a high price
//!    for a defensive branch. **Tracked as a known gap.**
//! 2. **Screen text that is not an error.** These exist in crates other than the
//!    shell, and were checked one by one on 07/09/2026:
//!    `crawrad_designer::AVISO_TRAVADO` (`crawrad-designer/src/lib.rs:37`), the
//!    `status` of the selection loop (`crawrad-designer/src/lib.rs:889`), and the
//!    category labels of `crawrad-registry` (`crawrad-registry/src/lib.rs:34`)
//!    — **all already in English**. None has a shape that text scanning
//!    recognizes: there is no attribute, marker, or common function uniting them, and
//!    inventing one just for the gate would be the ruler governing the design. If one
//!    reverts to Portuguese, human eyes in review must catch it.
//! 3. **`Command::label()`** (`crawrad-doc/src/command.rs:79`) is in Portuguese
//!    and stays. It is an undo label, nobody reads it on screen today — measured: there is
//!    not a single caller in the workspace, and the label that the bar shows comes from
//!    `Transaction.label`, which the shell writes in English. Translating it is the
//!    owner's decision, not this gate's. **It is a loaded gun**: the day someone
//!    wires it to the undo stack, the bar speaks Portuguese without the gate blinking — and
//!    that is why the debt is tracked as a known gap, not just here.
//! 4. **`#[error(concat!("a", "b"))]`** is only half read: the scanner reads the
//!    first literal and misses the rest. Nobody writes like that here, and the
//!    remedy would cost a macro parser.
//! 5. **`#[error(fmt = caminho::da_funcao)]`** (thiserror 2.0) has no literal
//!    at all. This does NOT pass silently: the gate flags it as opaque and requires
//!    rewriting with a literal — see [`Achado::Opaco`]. Escaping it requires
//!    writing the phrase in another file on purpose, and that is no accident.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// The English words that visible messages can use today.
///
/// In alphabetical order, so the next person can find theirs. The vocabulary is the same
/// that the shell already exposes in the bar (`nothing to save`, `unknown class`,
/// `rejected`): error messages are no place to invent terminology.
const LEXICO: &[&str] = &[
    "a",
    "accent",
    "activation",
    "active",
    "adapter",
    "affinity",
    "all",
    "along",
    "already",
    "always",
    "an",
    "and",
    "any",
    "appears",
    "are",
    "area",
    "as",
    "ascii",
    "at",
    "audio",
    "axes",
    "base",
    "be",
    "because",
    "becomes",
    "border",
    "box",
    "buffer",
    "built",
    "but",
    "by",
    "can",
    "cancelled",
    "cannot",
    "canonical",
    "capability",
    "carrying",
    "catalogue",
    "channel",
    "check",
    "child",
    "children",
    "circle",
    "claims",
    "clashes",
    "class",
    "closed",
    "code",
    "collection",
    "collision",
    "colours",
    "comes",
    "command",
    "common",
    "component",
    "components",
    "conflict",
    "connection",
    "context",
    "continue",
    "contribution",
    "core",
    "could",
    "crashed",
    "currently",
    "cycle",
    "dashed",
    "deactivation",
    "declare",
    "declared",
    "declares",
    "default",
    "descendants",
    "deserialization",
    "detected",
    "device",
    "digit",
    "digits",
    "dim",
    "disabled",
    "document",
    "does",
    "draws",
    "duplicate",
    "egui",
    "either",
    "empty",
    "error",
    "database",
    "even",
    "event",
    "every",
    "executable",
    "execution",
    "exist",
    "exit",
    "extension",
    "external",
    "failed",
    "field",
    "file",
    "fill",
    "first",
    "follow",
    "for",
    "form",
    "format",
    "found",
    "fraction",
    "frame",
    "from",
    "generated",
    "gives",
    "goes",
    "grows",
    "has",
    "header",
    "hold",
    "holds",
    "honor",
    "icon",
    "id",
    "ide",
    "identifier",
    "in",
    "index",
    "indexed",
    "inherit",
    "inherited",
    "inherits",
    "initialized",
    "input",
    "inset",
    "install",
    "installed",
    "internal",
    "into",
    "invalid",
    "io",
    "ipc",
    "is",
    "it",
    "items",
    "its",
    "json",
    "key",
    "kind",
    "know",
    "known",
    "knows",
    "least",
    "letter",
    "letters",
    "like",
    "lowercase",
    "lsp",
    "make",
    "malformed",
    "manifest",
    "mismatch",
    "missing",
    "more",
    "move",
    "must",
    "name",
    "named",
    "native",
    "needs",
    "never",
    "newer",
    "next",
    "no",
    "node",
    "not",
    "nothing",
    "of",
    "one",
    "only",
    "or",
    "other",
    "out",
    "package",
    "paint",
    "paints",
    "parameter",
    "parent",
    "per",
    "points",
    "prefix",
    "probably",
    "process",
    "project",
    "prop",
    "property",
    "protocol",
    "provides",
    "pull",
    "reachable",
    "read",
    "recorded",
    "rect",
    "redo",
    "registered",
    "registration",
    "remove",
    "removed",
    "rename",
    "reorder",
    "request",
    "required",
    "requires",
    "resource",
    "responded",
    "roles",
    "root",
    "runtime",
    "rust",
    "same",
    "say",
    "says",
    "sdk",
    "self",
    "serialization",
    "server",
    "shape",
    "shared",
    "so",
    "somebody",
    "source",
    "speaks",
    "start",
    "started",
    "step",
    "stream",
    "stroke",
    "such",
    "supported",
    "surface",
    "synthesis",
    "takes",
    "target",
    "text",
    "than",
    "that",
    "the",
    "there",
    "this",
    "timed",
    "to",
    "toml",
    "tools",
    "transaction",
    "transcription",
    "transport",
    "twice",
    "type",
    "underscores",
    "undo",
    "unexpected",
    "unexpectedly",
    "unknown",
    "update",
    "usable",
    "use",
    "valid",
    "value",
    "values",
    "verb",
    "version",
    "view",
    "was",
    "where",
    "which",
    "with",
    "words",
    "write",
    "x",
    "y",
];

/// What surrounds a word without being part of it.
///
/// The em dash is included: it separates the two halves of the `FutureFormat` phrase
/// (`— update CrawRAD`) and is legitimate English punctuation, unlike an
/// accent **inside** a word, which is what the gate hunts down.
const PONTUACAO: &[char] = &[
    '\'', '"', ',', '.', ':', ';', '(', ')', '[', ']', '?', '!', '`', '-', '>', '<', '—', '…',
];

/// Why a word was rejected.
#[derive(Debug, PartialEq, Eq)]
enum Veredito {
    /// Non-ASCII letter **inside** a word: accent, cedilla, tilde.
    Acentuada,
    /// Prose word that is not in [`LEXICO`].
    ForaDoLexico,
}

/// Judges **a single** raw word, already split by whitespace.
///
/// `None` means "nothing to enforce": loose punctuation, code,
/// identifier.
fn julgar(bruto: &str) -> Option<Veredito> {
    let t = bruto.trim_matches(|c: char| PONTUACAO.contains(&c));
    if t.is_empty() {
        return None;
    }
    // 1. Accents come BEFORE everything. Without this order, `não` would fall under the code
    //    rule (the `ã` is not an ASCII letter) and come out acquitted as if it were an
    //    identifier — the easiest loophole to open here.
    if !t.is_ascii() {
        return Some(Veredito::Acentuada);
    }
    // 2. Structural mark: digit, `_`, `::`, `.`, `/`. It is code, not prose.
    if t.chars().any(|c| !c.is_ascii_alphabetic()) {
        return None;
    }
    // 3. CamelCase — uppercase after the first letter, with lowercase in
    //    between: `CrawRAD`, `RemoveNode`. Full UPPERCASE does NOT qualify here on
    //    purpose: it falls into the lexicon as a regular word.
    let camel = t.chars().skip(1).any(|c| c.is_ascii_uppercase())
        && t.chars().any(|c| c.is_ascii_lowercase());
    if camel {
        return None;
    }
    if LEXICO.contains(&t.to_ascii_lowercase().as_str()) {
        None
    } else {
        Some(Veredito::ForaDoLexico)
    }
}

/// The words of a message that the gate rejects.
fn palavras_reprovadas(msg: &str) -> Vec<(String, Veredito)> {
    // `{0}`, `{node}`, and `{source}` are stripped before evaluation: what is inside them is
    // a field name, and field names are code.
    let mut prosa = String::new();
    let mut dentro = false;
    for c in msg.chars() {
        match c {
            '{' => dentro = true,
            '}' => {
                dentro = false;
                prosa.push(' ');
            }
            _ if !dentro => prosa.push(c),
            _ => {}
        }
    }
    prosa
        .split_whitespace()
        .filter_map(|p| julgar(p).map(|v| (p.to_string(), v)))
        .collect()
}

/// What the scan extracts from an `#[error(…)]`.
#[derive(Debug, PartialEq, Eq)]
enum Achado {
    /// The phrase, read from the literal. This is the overwhelmingly common form.
    Frase(String),
    /// The variant **has no phrase**: it forwards the entire field
    /// (`#[error("{0}")]`), and whoever writes the text is the call site.
    ///
    /// Carries the variant name (`Backend`) so the second pass can hunt down
    /// the literals passed to it. This was how
    /// `RuntimeError::Backend("documento sem raiz")` slipped past the gate in a
    /// review: the attribute was spotless, and the phrase lived three files
    /// away.
    Repassa(String),
    /// Attribute **without a literal to read** — the `#[error(fmt = f)]` of
    /// thiserror 2.0, which delegates the phrase to a function elsewhere.
    ///
    /// Not ignored: a gate that stays silent on what it does not understand becomes
    /// an open invitation to write there. The test flags it and requires the form
    /// it knows how to measure.
    Opaco(String),
}

/// The `#[error(…)]` attributes of a file, in the order they appear.
///
/// Reads the **attribute text**, rather than the assembled `Display`, for two reasons:
/// there is no need to construct a variant of each error — new variants are counted
/// automatically, which is the lesson from the `crawrad-actions` guard that enumerated
/// targets and stayed green — and the message is measured where it is written.
fn mensagens(fonte: &str) -> Vec<Achado> {
    const ABRE: &str = "#[error(";
    let mut saida = Vec::new();
    let mut resto = fonte;
    while let Some(p) = resto.find(ABRE) {
        let depois = &resto[p + ABRE.len()..];
        match literal(depois) {
            Some((msg, fim)) => {
                saida.push(match variante_repassadora(&msg, &depois[fim..]) {
                    Some(nome) => Achado::Repassa(nome),
                    None => Achado::Frase(msg),
                });
                resto = &depois[fim..];
            }
            None => {
                // `#[error(transparent)]` is legitimate and has no phrase: the
                // underlying error is responsible for it, and it is scanned as well.
                let corpo = depois.split(")]").next().unwrap_or("").trim();
                if corpo != "transparent" {
                    saida.push(Achado::Opaco(corpo.to_string()));
                }
                resto = depois;
            }
        }
    }
    saida
}

/// The variant name, if its message is **only a forwarded field**.
///
/// `#[error("{0}")]` is not a message: it is plumbing. The phrase the user reads
/// originates where the variant is constructed, and that is where the gate must look.
///
/// The ruler is strict: the entire literal must be **a single** `{field}` and nothing
/// more. `#[error("{path}: {source}")]` on `IrError::Io` does NOT qualify — it has
/// prose (the colon), `path` is a file path, and `source` is another
/// error, which the scan already inspects at its own site.
fn variante_repassadora(literal: &str, depois: &str) -> Option<String> {
    let miolo = literal.trim();
    let so_um_campo = miolo.starts_with('{')
        && miolo.ends_with('}')
        && miolo.len() > 2
        && !miolo[1..miolo.len() - 1].contains(['{', '}']);
    if !so_um_campo {
        return None;
    }
    // the identifier right after the attribute: `)]` + whitespace + `Backend(String)`
    let nome: String = depois
        .trim_start_matches([')', ']'])
        .trim_start()
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();
    (!nome.is_empty()).then_some(nome)
}

/// The literals passed to `Variante(` in this file.
///
/// Searches for `Backend("…"`, and only the literal form: when the argument is a
/// variable or `e.to_string()`, the text is external (toolkit error, OS
/// error) and translating it is not our responsibility. See boundary 2 at the top.
fn literais_entregues(fonte: &str, variante: &str) -> Vec<String> {
    let agulha = format!("{variante}(");
    let mut saida = Vec::new();
    let mut resto = fonte;
    while let Some(p) = resto.find(&agulha) {
        let depois = &resto[p + agulha.len()..];
        // **The quote may not be adjacent to the parenthesis, due to
        // `rustfmt`.** A long message becomes, canonically:
        //
        // ```
        // RuntimeError::Backend(
        //     "the document has no root, open a form first".into(),
        // )
        // ```
        //
        // and house policy pushes PRECISELY towards long messages — "say what
        // failed and what to do". Without trimming, the form imposed by the formatter
        // slips past the scanner with a clean `cargo fmt --check`: green
        // gate, Portuguese intact. This was verified by planting the phrase in that form.
        //
        // `Backend(String),` in the declaration has no quotes and is skipped automatically.
        let aparado = depois.trim_start();
        if aparado.starts_with('"')
            && let Some((texto, fim)) = literal(aparado)
        {
            saida.push(texto);
            resto = &aparado[fim..];
            continue;
        }
        resto = depois;
    }
    saida
}

/// The first string literal of the attribute, and the index right after it.
///
/// Understands Rust line continuation — a backslash at line end consumes the
/// newline and following indentation — because that is how long messages in
/// `ContractError` are written; without this it would arrive here with indentation
/// glued to the words.
fn literal(s: &str) -> Option<(String, usize)> {
    let cs: Vec<(usize, char)> = s.char_indices().collect();
    let mut i = 0usize;
    // The opening quote must come before the `)` closing the attribute.
    loop {
        let (_, c) = *cs.get(i)?;
        i += 1;
        match c {
            '"' => break,
            ')' => return None,
            _ => {}
        }
    }
    let mut texto = String::new();
    while let Some(&(pos, c)) = cs.get(i) {
        i += 1;
        match c {
            '"' => return Some((texto, pos + 1)),
            '\\' => {
                let (_, escapado) = *cs.get(i)?;
                i += 1;
                match escapado {
                    '\n' => {
                        while cs.get(i).is_some_and(|(_, d)| d.is_whitespace()) {
                            i += 1;
                        }
                    }
                    'n' | 't' | 'r' => texto.push(' '),
                    outro => texto.push(outro),
                }
            }
            _ => texto.push(c),
        }
    }
    None
}

/// The complaint for a message, ready for failure reporting.
fn cobrar(onde: &str, msg: &str) -> Vec<String> {
    palavras_reprovadas(msg)
        .into_iter()
        .map(|(palavra, v)| {
            let motivo = match v {
                Veredito::Acentuada => "non-ASCII character",
                Veredito::ForaDoLexico => "not in LEXICON",
            };
            format!("{onde}: `{palavra}` ({motivo}) em \"{msg}\"")
        })
        .collect()
}

/// Every `.rs` file in a directory, recursing into subfolders.
fn varrer(dir: &Path, achados: &mut Vec<PathBuf>) {
    let Ok(entradas) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entradas.flatten() {
        let p = e.path();
        if p.is_dir() {
            // `target/` is excluded: it houses projects generated by
            // `gate_fase3`, and generated code is not a message written by
            // anyone.
            if p.file_name().is_some_and(|n| n == "target") {
                continue;
            }
            varrer(&p, achados);
        } else if p.extension().is_some_and(|x| x == "rs") && !e_este_arquivo(&p) {
            achados.push(p);
        }
    }
}

/// The gate does not measure itself.
///
/// It mentions `#[error("…")]` in its documentation and constructs mock attributes in the
/// scanner's own test — all of them would appear as real messages in
/// Portuguese, and the gate would reject itself indefinitely. Same precaution as
/// `a_abertura_da_ide` in the shell, which constructs the needle instead of writing it literally.
///
/// This creates no loophole: this entire module is `#[cfg(test)]` and declares
/// no errors. Errors exist only in production code, and production code is what
/// remains of the scan.
fn e_este_arquivo(p: &Path) -> bool {
    p.file_name().is_some_and(|n| n == "portao_ingles.rs")
}

/// The entire `crates/` folder — **the entire** workspace.
///
/// Scanning by PATH rather than by dependency is what makes this possible:
/// `crawrad-doc` does not depend on `crawrad-codegen` nor `crawrad-runtime`, and
/// will not depend on them. **Reading file text does not create an edge in the graph** — this is the
/// same argument as `mutacoes_fora_do_catalogo` in `crawrad-actions`, which reads the
/// shell without depending on it.
///
/// Why does the gate live in `crawrad-doc`, then? Because it must live in
/// some crate, and this is the lowest one that already had reason to read foreign source
/// (every `IrError` leaves here wrapped in a `DocError::Ir`). Moving houses would not
/// change a single line of the scan. The alternative — putting the gate in the shell —
/// would be worse: the shell is the only crate no other crate compiles, and the gate
/// must run early.
///
/// The ENTIRE folder, rather than a list of crates: currently only four have `#[error(`,
/// and **that list is precisely what must not be hardcoded here**. A new error
/// enum in a new crate is counted automatically — the `crawrad-actions` guard
/// that enumerated four `include_str!` turned green the day the fifth one
/// was born, and that lesson cost a batch.
fn raiz_dos_crates() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// Minimum number of messages the scan must find.
///
/// The number measured on 07/09/2026, tight: **27 in attributes** (22 between
/// `crawrad-ir` and `crawrad-doc`, 3 in `crawrad-codegen`, 2 in `crawrad-runtime`) and **4
/// passed at call sites** to `RuntimeError::Backend`.
///
/// The third one in `crawrad-runtime` is specifically the `#[error("{0}")]` of `Backend`,
/// which does not count as a message: it is not one, it is plumbing — the four that
/// pass through it are what count.
///
/// It is `>=` rather than `==` on purpose — new messages are normal life and must not
/// break the build for whoever writes them; what this line guards against is a **blind**
/// scan, one that finds nothing and stays green because of it.
const MENSAGENS_MEDIDAS: usize = 31;

#[test]
fn toda_mensagem_visivel_ao_usuario_nasce_em_ingles() {
    let raiz = raiz_dos_crates();
    let mut arquivos = Vec::new();
    varrer(&raiz, &mut arquivos);
    assert!(
        arquivos.len() >= 30,
        "the scan found {} file(s) — the crates path moved, \
         and a gate that finds nothing to enforce stays green forever",
        arquivos.len()
    );

    // The path relative to `crates/`, not just the filename: with ten
    // crates scanned, `lib.rs` alone does not tell whom to hold accountable.
    let fontes: Vec<(String, String)> = arquivos
        .iter()
        .map(|a| {
            let nome = a
                .strip_prefix(&raiz)
                .unwrap_or(a)
                .display()
                .to_string()
                .replace('\\', "/");
            let fonte = std::fs::read_to_string(a).expect("scanned file opens");
            (nome, fonte)
        })
        .collect();

    let mut vistas = 0usize;
    let mut queixas: Vec<String> = Vec::new();
    let mut repassadoras: BTreeSet<String> = BTreeSet::new();

    // First pass: text residing in the attribute.
    for (nome, fonte) in &fontes {
        for achado in mensagens(fonte) {
            let msg = match achado {
                Achado::Frase(m) => m,
                Achado::Repassa(variante) => {
                    repassadoras.insert(variante);
                    continue;
                }
                Achado::Opaco(corpo) => {
                    queixas.push(format!(
                        "{nome}: `#[error({corpo})]` has no literal to read — \
                         write the phrase in the attribute, otherwise the gate cannot check it"
                    ));
                    continue;
                }
            };
            vistas += 1;
            queixas.extend(cobrar(nome, &msg));
        }
    }

    // Second pass: text passed to a forwarding variant. It needs the
    // entire first pass — the variant may be declared in one crate and constructed
    // in another, which is precisely the case for `Backend`: declared in
    // `crawrad-runtime`, written in `crawrad-runtime-egui` and `-mock`.
    for (nome, fonte) in &fontes {
        for variante in &repassadoras {
            for msg in literais_entregues(fonte, variante) {
                vistas += 1;
                queixas.extend(cobrar(&format!("{nome} ({variante})"), &msg));
            }
        }
    }

    // Complaints BEFORE the count, and the order is deliberate: an `#[error(fmt = …)]`
    // that REPLACES an existing message lowers the count by one, and the test
    // would fail saying "the scan missed a folder" — completely untrue, and
    // sends the next person looking in the wrong place. Failing is the same; pointing
    // is not.
    assert!(
        queixas.is_empty(),
        "user-visible message is not in English ({} complaint(s)):\n  {}\n\n\
         Every string the user READS is in English (project policy, 30/08/2026) — \
         comments and doc remain in Portuguese. These phrases reach the IDE status \
         bar via `rejected: <message>` from `Ide::apply`.\n\
         If the word IS ALREADY English and the gate does not recognize it, add it \
         to the `LEXICO` of this module — one line, and it is visible in the diff.",
        queixas.len(),
        queixas.join("\n  ")
    );

    assert!(
        vistas >= MENSAGENS_MEDIDAS,
        "the gate found {vistas} message(s) and the last count was \
         {MENSAGENS_MEDIDAS} — the scan missed a folder"
    );
}

mod testes_do_portao {
    //! The gate measuring itself. Without this, a bug in the scanner leaves it
    //! green because it finds nothing — the most common failure mode for guards reading
    //! source code.

    use super::*;

    #[test]
    fn o_acento_reprova() {
        for p in ["não", "nó", "transação"] {
            assert_eq!(julgar(p), Some(Veredito::Acentuada), "{p}");
        }
    }

    #[test]
    fn o_portugues_sem_acento_tambem_reprova() {
        // The reason the gate is not an accent filter: these four
        // would pass through it completely.
        for p in ["duplicado", "remova", "invalido", "componente"] {
            assert_eq!(julgar(p), Some(Veredito::ForaDoLexico), "{p}");
        }
    }

    #[test]
    fn o_grito_em_caixa_alta_nao_e_escape() {
        // `TODOS` was the word in the original `ContractError` message. If
        // UPPERCASE counted as an identifier, it would pass through.
        assert_eq!(julgar("TODOS"), Some(Veredito::ForaDoLexico));
        assert_eq!(julgar("EVERY"), None);
    }

    #[test]
    fn codigo_e_identificador_passam() {
        for t in [
            "crawrad_ir::contract::COMMON",
            "win32.",
            "CrawRAD",
            "RemoveNode",
            "—",
            "->",
        ] {
            assert_eq!(julgar(t), None, "{t}");
        }
    }

    #[test]
    fn o_varredor_le_o_literal_inteiro() {
        let fonte = "#[error(transparent)]\n#[error(\"one two\")]\n#[error(\n    \"three \\\n     four\"\n)]\n";
        assert_eq!(
            mensagens(fonte),
            vec![
                Achado::Frase("one two".into()),
                Achado::Frase("three four".into())
            ]
        );
    }

    #[test]
    fn a_mensagem_montada_por_funcao_e_acusada_em_vez_de_ignorada() {
        // `#[error(fmt = …)]` from thiserror 2.0 forwards the phrase to another
        // file. The gate cannot read it — and what it does with what it does not
        // understand is to FLAG it, because staying silent is the loophole it exists to close.
        // `transparent`, which also has no literal, remains legitimate: the
        // phrase comes from the underlying error, which is also scanned.
        let fonte = "#[error(transparent)]\n#[error(fmt = ola::mundo)]\n";
        assert_eq!(
            mensagens(fonte),
            vec![Achado::Opaco("fmt = ola::mundo".into())]
        );
    }

    #[test]
    fn a_variante_que_so_repassa_e_reconhecida_e_a_que_tem_prosa_nao() {
        // `#[error("{0}")]` is plumbing and becomes `Repassa`;
        // `#[error("{path}: {source}")]` on `IrError::Io` does NOT, because it has
        // prose (the colon) and because `source` is already scanned at its own
        // site. Confusing the two would make the gate hunt `Io("…")` across the
        // workspace.
        let so_repassa = "#[error(\"{0}\")]\n    Backend(String),\n";
        assert_eq!(
            mensagens(so_repassa),
            vec![Achado::Repassa("Backend".into())]
        );

        let tem_prosa = "#[error(\"{path}: {source}\")]\n    Io { path: String },\n";
        assert_eq!(
            mensagens(tem_prosa),
            vec![Achado::Frase("{path}: {source}".into())]
        );
    }

    #[test]
    fn o_texto_entregue_no_sitio_de_chamada_e_lido() {
        // The rejection case, in the exact form it was written.
        let fonte = "        .ok_or_else(|| RuntimeError::Backend(\"documento sem raiz\".into()))?;\n\
                     let outro = Backend(msg);\n";
        assert_eq!(
            literais_entregues(fonte, "Backend"),
            vec!["documento sem raiz".to_string()],
            "either lost the literal, or invented a phrase where the argument is a variable"
        );
        // The variant declaration is not a call site and must not be included.
        assert!(literais_entregues("    Backend(String),\n", "Backend").is_empty());
    }

    #[test]
    fn o_placeholder_nao_entra_na_conta() {
        // `{node}` and `{source}` are field names; if included, the gate
        // would enforce lexicon checks on code.
        assert!(palavras_reprovadas("in node '{node}': {source}").is_empty());
    }

    #[test]
    fn a_frase_em_portugues_seria_pega() {
        // Proof that the gate carries load: this is the message that
        // `DocError::RemoveWithChildren` had prior to this batch.
        let antiga = "'{0}' tem filhos: remova a descendência antes (ou use uma \
                      transação com um RemoveNode por nó)";
        let pegas: Vec<String> = palavras_reprovadas(antiga)
            .into_iter()
            .map(|(p, _)| p)
            .collect();
        assert!(
            pegas.len() >= 6,
            "the gate only saw {pegas:?} of Portuguese in this phrase"
        );
    }
}
