//! The two tests of the silenced lint gate: **the scan** and **the gate of the
//! gate**.
//!
//! The recognizer lives in the module; here lives what enforces it. The separation is the
//! rule in this repository for test files (`src/test/<modulo>/test.rs`, as
//! `command.rs` already does), and it fits especially well in this gate: the test
//! writes `#[allow(…)]` spelled out dozens of times, inside literals, and
//! this file **is also scanned** by the scan in the other test. If the
//! masker breaks, this file is the first to report it.

use super::{EXCECOES, caminho_curto, lints_silenciados, mascarar, raiz_dos_crates, varrer};

/// Minimum number of `.rs` files the scan must find.
///
/// Measured on 2026-09-15: **219** in `crates/`, not counting `target/`. The threshold
/// is generous because new files are normal life and this number cannot become
/// another count to keep up to date. What it hunts is a **blind**
/// scan: root moved, `read_dir` failed to open, gate that
/// starts finding nothing and turns green because of that.
const ARQUIVOS_MEDIDOS: usize = 150;

/// Number of `allow(` occurrences the scan must **see and dismiss**.
///
/// Measured on 2026-09-15: **21**, not counting those in this gate — the prelude
/// that the generator writes for the user's project
/// (`crawrad-codegen/src/targets/egui/form.rs:175` and `stubs.rs:150,153`), the
/// mock user file in `gate_fase3.rs`, and the prose explaining the
/// prohibition. None is our suppression.
///
/// This threshold is the opposite of the other: it guarantees that the gate keeps
/// **reading** the prohibited text and deciding that it is not an attribute. If one
/// day it drops, either someone deleted the generator, or the scan stopped
/// seeing the files where the distinction is hard — and it is precisely in those
/// that a rushed gate would fail.
const CITACOES_MEDIDAS: usize = 15;

/// Number of attributes (`#[…]`/`#![…]`) the masked text must preserve.
///
/// Measured on 2026-09-15: **1969** openings in raw text and **1728** in
/// masked text — the 241 that disappear were inside strings or comments,
/// and disappearing is their job. The threshold protects against the most dangerous defect of this
/// design:
/// a masker getting lost — on a lifetime single quote, on a
/// raw string with fences — and **deleting real code**. It would turn green
/// from blindness, which is the worst way to turn green.
const ATRIBUTOS_MEDIDOS: usize = 1200;

#[test]
fn nenhum_lint_e_silenciado_nos_nossos_crates() {
    let raiz = raiz_dos_crates();
    let mut arquivos = Vec::new();
    varrer(&raiz, &mut arquivos);
    arquivos.sort();

    assert!(
        arquivos.len() >= ARQUIVOS_MEDIDOS,
        "the scan found {} file(s) — the crates path moved, \
         and a gate that finds nothing to enforce stays green forever",
        arquivos.len()
    );

    // Constructed, not written out: our `grep` searches for the literal form, and
    // this file should not appear in it as if it were another occurrence
    // to inspect.
    let citacao = format!("{}(", "allow");

    let mut queixas: Vec<String> = Vec::new();
    let mut citacoes = 0usize;
    let mut atributos = 0usize;

    for arquivo in &arquivos {
        let Ok(fonte) = std::fs::read_to_string(arquivo) else {
            continue;
        };
        let nome = caminho_curto(arquivo, &raiz);
        // Occurrences from the gate ITSELF do not count. This file and the module
        // next to it write the attribute spelled out dozens of times, and leaving
        // them in the count would satisfy the lower bound on its own — the guard
        // proving to itself that it reads, without reading anyone.
        if !nome.contains("silenced_lint_gate") {
            citacoes += fonte.matches(citacao.as_str()).count();
        }
        atributos += mascarar(&fonte).matches("#[").count();

        let achados = lints_silenciados(&fonte);
        if achados.is_empty() {
            continue;
        }
        if EXCECOES.iter().any(|(alvo, _)| nome.ends_with(alvo)) {
            continue;
        }
        let linhas: Vec<&str> = fonte.lines().collect();
        for (linha, nome_do_lint) in achados {
            let texto = linhas.get(linha - 1).unwrap_or(&"").trim();
            queixas.push(format!("{nome}:{linha}: `{nome_do_lint}` in `{texto}`"));
        }
    }

    assert!(
        citacoes >= CITACOES_MEDIDAS,
        "the scan saw only {citacoes} citation(s) of the forbidden attribute, and \
         expected at least {CITACOES_MEDIDAS} — either the prelude that the generator \
         writes for the user project is gone, or the gate stopped reading the \
         files where telling attribute from text is hard"
    );
    assert!(
        atributos >= ATRIBUTOS_MEDIDOS,
        "the masked text kept only {atributos} attribute(s), and expected at \
         least {ATRIBUTOS_MEDIDOS} — the masker is deleting CODE, and \
         a blind gate stays green forever"
    );

    let mortas: Vec<&str> = EXCECOES
        .iter()
        .map(|(alvo, _)| *alvo)
        .filter(|alvo| {
            !arquivos
                .iter()
                .any(|a| caminho_curto(a, &raiz).ends_with(alvo))
        })
        .collect();
    assert!(
        mortas.is_empty(),
        "these exceptions do not point to any file: {mortas:?} — a dead exception \
         rots quietly and becomes forgotten license; remove them"
    );

    assert!(
        queixas.is_empty(),
        "compiler warning was SILENCED instead of fixed:\n{}\n\n\
         The way forward is to fix the lint. Rust says what is wrong and almost \
         always suggests the right form; the attribute only hides the message.\n\
         If you arrived here measuring your own crate under `-D warnings` and the \
         warning is from ANOTHER area, the command is `cargo clippy -p <your-crate>` \
         — never silence someone else's file to make the workspace pass. That was \
         exactly what happened in `designer/regua.rs` in 09/2026: the \
         promised restoration did not happen, and the real fix, made \
         afterwards, revealed a `u32` overflow that silence had buried.\n\
         If the lint is WRONG about this code, the escape hatch is written: a \
         line in `EXCECOES`, with the reason, visible in the diff.",
        queixas.join("\n")
    );
}

/// **The gate of the gate.** The scan above is only valid if the recognizer
/// knows the difference between writing the attribute and mentioning it.
///
/// Each case is an entire source, not a line snippet, because that is how
/// the recognizer works: it needs context to decide. The
/// cases that **pass** are the ones that matter — they are what a `grep`-based
/// gate would fail, and the first of them is literally
/// `crates/crawrad-codegen/tests/gate_fase3.rs:218`, starting at column 1.
///
/// Mutation proof, measured on 2026-09-15:
/// - replacing the body of `mascarar` with `fonte.to_string()` (without deleting anything)
///   fails the **five** cases where text is inside a string or comment,
///   and fails the other two tests in this file along with them;
/// - restricting the search to the first name after `#[` fails `cfg_attr`;
/// - removing `expect` from `ATRIBUTOS_QUE_AFROUXAM` fails its case.
///
/// The last three cases (`forbid`/`deny`/`warn`, the `.expect(…)` method call and
/// `allow_isto`) survive these mutations on purpose: they do not measure the
/// masker, they measure the boundary between what is an attribute and what is an
/// identifier boundary.
#[test]
fn o_reconhecedor_separa_o_atributo_do_texto_que_o_cita() {
    let cerca = "#".repeat(2);
    let casos: &[(&str, &str, bool)] = &[
        (
            "the common attribute",
            "#[allow(dead_code)]\nfn f() {}\n",
            true,
        ),
        ("the module attribute", "#![allow(non_snake_case)]\n", true),
        (
            "split by rustfmt",
            "#[allow(\n    clippy::too_many_arguments\n)]\nfn f() {}\n",
            true,
        ),
        (
            "the same silence in disguise",
            "#[cfg_attr(test, allow(dead_code))]\nfn f() {}\n",
            true,
        ),
        ("silence with another name", "#[expect(dead_code)]\n", true),
        (
            "after a lifetime, which cannot consume the rest",
            "fn f<'a>(s: &'a str) -> char { 'x' }\n#[allow(dead_code)]\nfn g() {}\n",
            true,
        ),
        // --- from here down, everything passes ---
        (
            "the prelude inside a normal string, as the generator writes it",
            "let p = \"#![allow(non_snake_case)]\\n\";\n",
            false,
        ),
        (
            "the user file inside a raw string, at column 1",
            &format!(
                "const A: &str = r{cerca}\"//! user file\n#![allow(non_snake_case)]\n\"{cerca};\n"
            ),
            false,
        ),
        (
            "citation in a line comment",
            "// never write #[allow(dead_code)] here\nfn f() {}\n",
            false,
        ),
        (
            "citation in a doc-comment",
            "/// `#[allow(dead_code)]` is forbidden.\nfn f() {}\n",
            false,
        ),
        (
            "citation in a nested block comment",
            "/* /* #[allow(dead_code)] */ still inside */\nfn f() {}\n",
            false,
        ),
        (
            "what tightens, and is welcome",
            "#![forbid(unsafe_code)]\n#![deny(warnings)]\n#[warn(clippy::all)]\n",
            false,
        ),
        (
            "the homonymous method, which is half a tree",
            "let v = x.expect(\"no root\");\nlet w = y.allow(1);\n",
            false,
        ),
        (
            "name that only starts the same",
            "#[allow_isto(dead_code)]\n",
            false,
        ),
    ];

    let errados: Vec<&str> = casos
        .iter()
        .filter(|(_, fonte, acusa)| lints_silenciados(fonte).is_empty() == *acusa)
        .map(|(nome, _, _)| *nome)
        .collect();
    assert!(
        errados.is_empty(),
        "the recognizer answered the opposite of what these cases deserve: {errados:?}"
    );
}

/// The reported line is the file line — otherwise the complaint sends the dev to
/// the wrong place, and a lying complaint costs more than no complaint.
#[test]
fn a_queixa_aponta_a_linha_certa() {
    let fonte = "//! doc\n\nfn a() {}\n\n#[allow(dead_code)]\nfn b() {}\n";
    assert_eq!(lints_silenciados(fonte), vec![(5, "allow".to_string())]);

    // With a three-line block comment and a raw string in the middle, the
    // counter must cross them without losing track.
    let cerca = "#".repeat(2);
    let fonte = format!(
        "/* one\n   comment\n   of three */\nconst A: &str = r{cerca}\"\n#[allow(x)]\n\"{cerca};\n#[expect(dead_code)]\nfn b() {{}}\n"
    );
    assert_eq!(lints_silenciados(&fonte), vec![(7, "expect".to_string())]);
}
