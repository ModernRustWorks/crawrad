//! **THE EMITTABLE PIECE GATE** — authoring rules for everything residing in
//! `crawrad-component/src/pieces/`.
//!
//! # Why it was born, and what it found on day one
//!
//! The rules were written down — in the header of `strings.rs`, in full, alongside
//! the compiler error number. **And nobody checked them.**
//!
//! On 18/09/2026, while writing the second piece (`anchors.rs`), I broke two
//! of them in the same afternoon: I started the file with `//!` and then put a comment
//! in Portuguese in the middle. Both were caught by accident — by the generated project
//! gate, which only checks what the test form emits.
//!
//! And that accident hid a third one: **`strings.rs` had carried the same
//! fuse for weeks**. Its header mentions `crawrad_ir` to say *"do not use
//! anything of ours"*, and the `o_projeto_gerado_nao_depende_de_nenhum_crate_nosso` gate
//! searches for `crawrad_` anywhere in the generated file — including comments, and
//! rightly so: whoever opens the project has no way of knowing that it is just prose.
//!
//! It never exploded purely by **luck**: it is only emitted when the form has
//! `Memo`, `ListBox`, or `ComboBox`, and that test's form had none.
//!
//! # Why it scans the FOLDER, not a list
//!
//! A new piece enters this scan automatically, the day the file appears.
//! A list written here would be a third copy of something the file system
//! already knows.

use std::path::{Path, PathBuf};

/// The pieces folder — the toolbox, and nothing else.
fn pasta() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("crawrad-component")
        .join("src")
        .join("pieces")
}

/// Every file emitted verbatim — that is, all except `mod.rs`.
///
/// `mod.rs` is excluded **on purpose** and explains why itself: it is the
/// only one that does not travel, which is why it can carry house prose, in
/// Portuguese, explaining why each piece exists.
fn pecas() -> Vec<(String, String)> {
    let mut achadas: Vec<(String, String)> = std::fs::read_dir(pasta())
        .expect("pieces folder exists")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("rs"))
        .filter(|p| p.file_name().and_then(|x| x.to_str()) != Some("mod.rs"))
        .map(|p| {
            let nome = p
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let texto = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{nome}: {e}"));
            (nome, texto)
        })
        .collect();
    achadas.sort();
    achadas
}

/// **There are pieces to measure**, otherwise the gate stays green without checking anything.
#[test]
fn ha_peca_para_medir() {
    let achadas = pecas();
    assert!(
        achadas.len() >= 2,
        "expected at least `strings` and `anchors`, found {}: {:?}",
        achadas.len(),
        achadas.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );
}

/// **No `//!` and no `#![…]`.**
///
/// The text lands in the MIDDLE of a file, after other items, and an inner attribute
/// or doc comment after an item is `error[E0753]` — the user's project fails
/// to compile, with the message pointing to a line they never wrote.
#[test]
fn nenhuma_peca_abre_com_doc_de_modulo() {
    for (nome, texto) in pecas() {
        for (i, linha) in texto.lines().enumerate() {
            let t = linha.trim_start();
            assert!(
                !t.starts_with("//!") && !t.starts_with("#!["),
                "{nome}:{}: emitted piece cannot have `//!` or `#![…]` — it lands \
                 in the middle of a file, which is `error[E0753]` in the user's \
                 project:\n  {linha}",
                i + 1
            );
        }
    }
}

/// **No piece NAMES one of our crates**, not even in comments.
///
/// The `o_projeto_gerado_nao_depende_de_nenhum_crate_nosso` gate searches for
/// `crawrad_` across the entire file, and rightly so: whoever opens the generated project has
/// no way of knowing that line is just prose.
///
/// The rule is stronger than it seems: not even to SAY that one should not use it.
#[test]
fn nenhuma_peca_nomeia_um_crate_nosso() {
    for (nome, texto) in pecas() {
        for agulha in ["crawrad_", "crawrad-"] {
            assert!(
                !texto.contains(agulha),
                "{nome}: mentions `{agulha}`, and this piece's entire text goes \
                 into someone else's project — not even in comments"
            );
        }
    }
}

/// **No piece contains Portuguese.**
///
/// The same list as `o_codigo_gerado_nao_tem_portugues`, and the same reason (ADR
/// 0030): whoever opens the file is reading Rust in their application.
///
/// That gate scans what the TEST form emits; this one scans the source.
/// A piece that the test form does not use bypassed it completely — that is how a
/// Portuguese comment slipped into `anchors.rs` and was only caught because that
/// test's form had anchors.
#[test]
fn nenhuma_peca_tem_portugues() {
    const RASTROS: &[&str] = &[
        "arquivo",
        "código",
        "codigo",
        "formulário",
        "formulario",
        "usuário",
        "usuario",
        "gerado",
        "você",
        "aqui",
        "bracos",
        "braços",
        "defeito",
        "conta",
        "peça",
        "peca ",
        "não",
        "nao ",
        "que e ",
    ];
    for (nome, texto) in pecas() {
        for r in RASTROS {
            assert!(
                !texto.contains(r),
                "{nome}: contains Portuguese — `{r}`. The entire file goes into \
                 someone else's project; internal prose belongs in `pieces/mod.rs`, \
                 which is not emitted."
            );
        }
    }
}

/// **No piece imports anything.**
///
/// On the other side none of our crates exist, and a piece that required a `use`
/// from outside `std` would not be emittable — it would be a disguised dependency.
#[test]
fn nenhuma_peca_importa_de_fora_do_std() {
    for (nome, texto) in pecas() {
        for (i, linha) in texto.lines().enumerate() {
            let t = linha.trim_start();
            let Some(caminho) = t.strip_prefix("use ") else {
                continue;
            };
            assert!(
                caminho.starts_with("std::") || caminho.starts_with("core::"),
                "{nome}:{}: emitted piece may only import from `std`:\n  {linha}",
                i + 1
            );
        }
    }
}
