//! # crawrad-codegen — from the document to a project that compiles
//!
//! The `.crawform` is given; here it turns into the **code** of a real
//! Cargo crate: `main.rs`, the designer, and the user file, assembled from the
//! tree.
//!
//! ## What this crate does NOT write, and why
//!
//! `Cargo.toml` and `.gitignore`. They are created by `cargo init`, and that is where
//! they must come from (ADR 0018):
//! our template said `edition = "2021"` on the same day `cargo init` on the
//! same machine said `"2024"`, and nobody noticed for months. A hand-written
//! template is born correct and rots in silence — `cargo`'s comes from the
//! installed toolchain and keeps pace on its own.
//!
//! **Nothing of ours remains in the manifest.** There was an `isolar_workspace` that
//! added an empty `[workspace]` to the package and cut off the inheritance left by
//! `cargo init`; it existed because the IDE wrote the group root
//! *after* `cargo`. By writing it before, `cargo new` already finds the right
//! root and the problem never arises — and a patch that no path
//! applies is worse than none, because it looks like protection.
//!
//! ## One trait, one generator per target
//!
//! This is ADR 0004: the
//! project declares what it generates for, and each target is an implementor of
//! [`Gerador`]. There is no scattered `if alvo == "egui"` — there is one generator per target,
//! just as there is one runtime per target.
//!
//! ## The non-negotiable rule
//!
//! **User code is never overwritten.** Generated files state this
//! in their header and can be deleted at any time; the stub file
//! is born once and then only **grows**, when a new event appears. A
//! generator that erases the work of those who use it is a generator nobody uses
//! twice.
//!
//! ## What the generated project does NOT have
//!
//! A dependency on our crates. Neither `crawrad-ir`, nor runtime, nothing. What comes out of here
//! is a project that depends only on the target's toolkit — and that is the proof that this is
//! generation, not a disguised interpreter.

// No `unsafe`, and the compiler guarantees it. Since ADR 0013 this holds for the
// whole product — there is no longer an FFI adapter where it could live.
#![forbid(unsafe_code)]

pub mod dependencies;
pub mod emission;
pub mod events;
pub mod link;
pub mod locales;
pub mod scaffold;
pub mod targets;

pub use dependencies as dependencias;
pub use link as elo;

pub use dependencies::{
    Dependencia, component_dependencies, dependencias_do_projeto, documento_tem_bindings,
    has_bound_controls, has_bound_grid, locale_dependencies, minimum_rustc,
};
pub use targets::AlvoGerador;
pub use targets::egui as rxp;
pub use targets::egui::{GeradorRxp, LocalesScope, has_locales, set_has_locales};
pub use targets::makepad::GeradorMakepad;

// EN Aliases (ADR 0030)
pub use AlvoGerador as GeneratorTarget;
pub use Gerador as Generator;
pub use GeradorMakepad as MakepadGenerator;
pub use GeradorRxp as RxpGenerator;
pub type GeneratedFile = Arquivo;
pub type FileOwner = Dono;
pub type CodeAddition = Adicao;
pub type CodeAnchor = Ancora;

use std::path::{Path, PathBuf};

#[note("sufixo-gerado")]
pub const SUFIXO_GERADO: &str = ".designer.rs";

#[note("generated-file")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arquivo {
    /// Path relative to the project root (`src/ui.rs`).
    pub caminho: PathBuf,
    pub conteudo: String,
    /// Who owns this file.
    pub dono: Dono,
    /// What to append when the file **already exists** and does not contain it.
    ///
    /// This is how the stub file grows without being rewritten: the user hooks up a
    /// new event in the inspector, and the handler appears at the end of their file without
    /// touching a single line of what they wrote.
    pub adicoes: Vec<Adicao>,
}

#[note("adicao")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Adicao {
    /// The name of the `fn` that this addition brings.
    ///
    /// The addition is skipped when it is **already defined** in the file — and
    /// what answers that is `elo`, which masks comments and strings before
    /// searching. It used to be a raw `contains("fn nome(")`, and failed in both
    /// directions, both breaking the user's build without a clue:
    ///
    /// - **False positive:** a `// TODO: fn btn_okClick(` in a comment
    ///   convinced the generator that the handler existed. It was not created,
    ///   `.designer.rs` called it anyway, and `cargo` stopped at *no method
    ///   named*. The handler "vanished".
    /// - **False negative:** the user reformats to `fn btn_okClick (` — a
    ///   space — and the generator duplicated the definition.
    pub nome_da_fn: String,
    /// The self-contained text, for when it has to go to the end of the
    /// file — must carry its own `impl`.
    pub texto: String,
    /// The right place, when it still exists.
    ///
    /// The user file is born with a marker inside the `impl`; finding
    /// it, the new handler goes there and the file continues with a single `impl`.
    /// If the user deleted the marker, `texto` goes to the end — ugly, but
    /// nothing is lost, and touching what they wrote would be worse.
    pub ancora: Option<Ancora>,
}

#[note("ancora")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ancora {
    /// The line before which the text enters.
    pub marca: String,
    /// The text at this position — here it is already inside the `impl`, so
    /// it does not take one.
    pub texto: String,
}

#[note("file-owner")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dono {
    /// Owned by the generator. Rewritten on every generation, without asking.
    Gerador,
    /// Owned by the user. Born once and never touched again — not even to "fix" it.
    Usuario,
    /// Owned by the generator the first time, by the user thereafter.
    ///
    /// **Today no file in the plan is born with this owner.** It used to be
    /// `Cargo.toml`, and since ADR 0018
    /// the manifest belongs to the user **from the first second** — `cargo init`
    /// creates it, so it never passes through "ours once".
    ///
    /// The variant stays. It is not an internal detail:
    /// ADR 0016
    /// promoted it to UI vocabulary, and the Explorer draws a badge per
    /// variant with exhaustive `match`. Removing an owner is a UI decision, and
    /// is not made down here for lack of momentary use — `escrever()`
    /// continues to honor the semantics, and the day a target emits a skeleton
    /// that the user evolves, it is standing ready.
    PrimeiraVez,
}

#[note("generator-trait")]
pub trait Gerador {
    #[note("gerador-alvo")]
    fn alvo(&self) -> &'static str;

    #[note("gerador-classes")]
    fn classes(&self) -> &'static [&'static str];

    #[note("gerador-gerar")]
    fn gerar(
        &self,
        doc: &crawrad_ir::Document,
        nome_do_crate: &str,
        reg: &crawrad_registry::Registry,
    ) -> Result<Vec<Arquivo>, Erro> {
        self.generate_with_base(doc, nome_do_crate, reg, None, false)
    }

    #[note("gerador-generate-with-base")]
    fn generate_with_base(
        &self,
        doc: &crawrad_ir::Document,
        nome_do_crate: &str,
        reg: &crawrad_registry::Registry,
        base: Option<&crawrad_ir::Document>,
        is_base: bool,
    ) -> Result<Vec<Arquivo>, Erro> {
        let bases = base.map(std::slice::from_ref).unwrap_or_default();
        self.generate_with_bases(doc, nome_do_crate, reg, bases, is_base)
    }

    #[note("gerador-generate-with-bases")]
    fn generate_with_bases(
        &self,
        doc: &crawrad_ir::Document,
        nome_do_crate: &str,
        reg: &crawrad_registry::Registry,
        bases: &[crawrad_ir::Document],
        is_base: bool,
    ) -> Result<Vec<Arquivo>, Erro>;
}

#[note("resolve-bases")]
pub(crate) fn resolve_bases(bases: &[crawrad_ir::Document]) -> Option<crawrad_ir::Document> {
    crawrad_ir::Document::resolve_chain(bases)
}

impl Gerador for Box<dyn Gerador> {
    fn alvo(&self) -> &'static str {
        (**self).alvo()
    }

    fn classes(&self) -> &'static [&'static str] {
        (**self).classes()
    }

    fn gerar(
        &self,
        doc: &crawrad_ir::Document,
        nome_do_crate: &str,
        reg: &crawrad_registry::Registry,
    ) -> Result<Vec<Arquivo>, Erro> {
        (**self).gerar(doc, nome_do_crate, reg)
    }

    fn generate_with_base(
        &self,
        doc: &crawrad_ir::Document,
        nome_do_crate: &str,
        reg: &crawrad_registry::Registry,
        base: Option<&crawrad_ir::Document>,
        is_base: bool,
    ) -> Result<Vec<Arquivo>, Erro> {
        (**self).generate_with_base(doc, nome_do_crate, reg, base, is_base)
    }

    fn generate_with_bases(
        &self,
        doc: &crawrad_ir::Document,
        nome_do_crate: &str,
        reg: &crawrad_registry::Registry,
        bases: &[crawrad_ir::Document],
        is_base: bool,
    ) -> Result<Vec<Arquivo>, Erro> {
        (**self).generate_with_bases(doc, nome_do_crate, reg, bases, is_base)
    }
}

#[note("desligamento")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Desligamento {
    /// The `fn` in the user's file. It is still on disk when this is
    /// returned: what removes it is the next generation, via `rxp::podar`.
    pub handler: String,
    /// The `(node, event)` pairs that point to it **now**, and which the IDE must
    /// unbind. Delivered as a list because the handler may be shared, in which
    /// case all bindings are removed together: a surviving binding would cause
    /// `.designer.rs` to call a method that no longer exists.
    pub vinculos: Vec<(crawrad_ir::NodeId, String)>,
}

#[note("vinculos-vencidos")]
pub fn vinculos_vencidos(raiz: &Path, doc: &crawrad_ir::Document) -> Vec<Desligamento> {
    stale_bindings_editing(raiz, doc, &|_| None)
}

#[note("stale-bindings-editing")]
pub fn stale_bindings_editing(
    raiz: &Path,
    doc: &crawrad_ir::Document,
    editing: &dyn Fn(&Path) -> Option<String>,
) -> Vec<Desligamento> {
    let Some(relativo) = rxp::unidade_do_usuario(doc) else {
        return Vec::new();
    };
    let destino = raiz.join(relativo);
    let Some(fonte) = editing(&destino).or_else(|| std::fs::read_to_string(&destino).ok()) else {
        return Vec::new();
    };
    // The same neighbors that pruning will check, and for the same reason: if
    // someone calls the handler, it is not removed — and therefore its binding
    // cannot be removed either, otherwise a live `fn` would remain with no event pointing to it.
    let vizinhos = neighbours_editing(raiz, &destino, editing).unwrap_or_default();
    let vinculos = rxp::vinculos(doc);
    let mut fora: Vec<Desligamento> = rxp::podaveis(&fonte, &[], &vizinhos)
        .into_iter()
        .filter_map(|(_, handler)| {
            let meus: Vec<(crawrad_ir::NodeId, String)> = vinculos
                .iter()
                .filter(|(f, _, _)| *f == handler)
                .map(|(_, id, ev)| (id.clone(), ev.clone()))
                .collect();
            // Without a binding there is nothing to unbind: this is a handler the user
            // already unbound by hand, and pruning removes it on its own during this generation.
            (!meus.is_empty()).then_some(Desligamento {
                handler,
                vinculos: meus,
            })
        })
        .collect();
    fora.sort_by(|a, b| a.handler.cmp(&b.handler));
    fora
}

// HERE LIVED `frase_do_desligamento`, and it said
// `"1 empty handler unwired — Ctrl+Z restores it"`.
//
// Removed by owner decision, 08/09/2026, against the design that proposed it: *"1) a
// classic RAD IDE does NOT offer Ctrl+Z for this cleanup after it is applied. 2) A classic IDE
// cleans up, however many there are"*. The sentence existed to sell undo as
// our differentiator — and offering a `Ctrl+Z` in the bar that the reference
// product does not provide is promising a contract, not reporting behavior.
//
// The transaction remains, and it is no courtesy: document mutations go through
// `crawrad-doc`, period (ADR 0006). What was removed was the PROMISE on screen, not the
// mechanism — verified by the reviewer that the state after undo is
// consistent and the project compiles.
//
// And the bar does not stay silent: what reports pruning is `Relatorio::frase_da_poda`,
// which refers to the FILE (`"2 empty handlers removed"`) and is appended by the
// shell's `frase_da_geracao`. One phrase per gesture; two reporting the same
// gesture in halves was what made this one seem necessary.

#[note("rotulo-do-desligamento")]
pub const ROTULO_DO_DESLIGAMENTO: &str = "Remove empty handlers";

#[note("escrever")]
pub fn escrever(raiz: &Path, arquivos: &[Arquivo]) -> Result<Relatorio, Erro> {
    // Before writing, not after: if the form changed name, the
    // user's file goes with it. Afterward would be too late — the new,
    // empty stub would be in its place.
    let mut r = Relatorio {
        renomeados: migrar_rename(raiz, arquivos)?,
        ..Default::default()
    };
    // The generated files as they were BEFORE this write: a handler they call
    // was bound until now, which tells a handler renamed by `event.rename`
    // from one the user renamed by hand.
    let generated_before: String = arquivos
        .iter()
        .filter(|a| a.caminho.to_string_lossy().ends_with(SUFIXO_GERADO))
        .filter_map(|a| std::fs::read_to_string(raiz.join(&a.caminho)).ok())
        .collect();
    for a in arquivos {
        let destino = raiz.join(&a.caminho);
        let existe = destino.exists();
        if existe && matches!(a.dono, Dono::Usuario | Dono::PrimeiraVez) {
            desbomar(&destino)?;
            let (cresceu, podados) = crescer_e_podar(raiz, &destino, a, &generated_before)?;
            if cresceu {
                r.cresceram.push(a.caminho.clone());
            } else if podados.is_empty() {
                r.preservados.push(a.caminho.clone());
            }
            r.podados.extend(podados);
            continue;
        }
        if let Some(pai) = destino.parent() {
            std::fs::create_dir_all(pai).map_err(|e| Erro::Io {
                caminho: pai.display().to_string(),
                fonte: e,
            })?;
        }
        gravar(&destino, &a.conteudo)?;
        if existe {
            r.reescritos.push(a.caminho.clone());
        } else {
            r.criados.push(a.caminho.clone());
        }
    }
    r.removidos = varrer_orfaos(raiz, arquivos)?;
    Ok(r)
}

#[note("tipo-do-form")]
pub fn tipo_do_form(nome: &str) -> String {
    nome.split('_')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut c = p.chars();
            match c.next() {
                Some(primeira) => primeira.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

#[note("migrar-rename")]
fn migrar_rename(raiz: &Path, arquivos: &[Arquivo]) -> Result<Vec<(PathBuf, PathBuf)>, Erro> {
    let src = raiz.join("src");
    let Some(novo) = base_do_designer(arquivos.iter().map(|a| a.caminho.as_path())) else {
        return Ok(Vec::new());
    };
    let Ok(entradas) = std::fs::read_dir(&src) else {
        return Ok(Vec::new());
    };
    let caminhos: Vec<PathBuf> = entradas.flatten().map(|e| e.path()).collect();
    let mut velhos: Vec<String> = caminhos
        .iter()
        .filter_map(|c| base_do_designer(std::iter::once(c.as_path())))
        .filter(|n| *n != novo)
        .collect();
    if velhos.len() != 1 {
        return Ok(Vec::new());
    }
    let velho = velhos.pop().unwrap_or_default();
    let de = src.join(format!("{velho}.rs"));
    let para = src.join(format!("{novo}.rs"));
    if !de.exists() || para.exists() {
        return Ok(Vec::new());
    }

    // Both the type AND the module rename together, otherwise the `use` in the
    // loaded file will not compile. Replacement is by whole word: a user's
    // `FormMainX` does not become `FormPrincipalX`.
    //
    // The module entered this when it ceased to be named `ui`. The fixed name
    // resulted in two `mod ui` in the same `main.rs` as soon as a second
    // form existed — immediate collision, and a name that did not say what it belonged to.
    let texto = std::fs::read_to_string(&de).map_err(|e| Erro::Io {
        caminho: de.display().to_string(),
        fonte: e,
    })?;
    let texto = trocar_palavra(&texto, &tipo_do_form(&velho), &tipo_do_form(&novo));
    let texto = trocar_palavra(
        &texto,
        &format!("{velho}_designer"),
        &format!("{novo}_designer"),
    );
    gravar(&para, &texto)?;
    std::fs::remove_file(&de).map_err(|e| Erro::Io {
        caminho: de.display().to_string(),
        fonte: e,
    })?;
    Ok(vec![(
        PathBuf::from("src").join(format!("{velho}.rs")),
        PathBuf::from("src").join(format!("{novo}.rs")),
    )])
}

#[note("base-do-designer")]
fn base_do_designer<'a>(mut caminhos: impl Iterator<Item = &'a Path>) -> Option<String> {
    caminhos.find_map(|c| {
        c.file_name()?
            .to_str()?
            .strip_suffix(SUFIXO_GERADO)
            .map(str::to_string)
    })
}

#[note("trocar-palavra")]
fn trocar_palavra(texto: &str, de: &str, para: &str) -> String {
    // `para.is_empty()` is the last gate, and it is real: with an empty
    // target this function **erases** — `fn _ajuda(_f: &FormMain)` became
    // `fn _ajuda(_f: &)` in the user file, and the project stopped compiling.
    //
    // Validation in `identificador` already makes this unreachable today, but it is
    // here where the damage happens, and whatever destroys is accountable to its own
    // contract. A guard, not `debug_assert!`: `debug_assert!` vanishes in
    // release, and it is in release where the user file lives.
    if de == para || de.is_empty() || para.is_empty() {
        return texto.to_string();
    }
    let e_parte = |c: char| c.is_alphanumeric() || c == '_';
    let bytes = texto.as_bytes();
    let mut saida = String::with_capacity(texto.len());
    let mut i = 0;
    while let Some(achou) = texto[i..].find(de) {
        let ini = i + achou;
        let fim = ini + de.len();
        let antes = texto[..ini].chars().next_back().is_some_and(e_parte);
        let depois = bytes
            .get(fim)
            .and_then(|_| texto[fim..].chars().next())
            .is_some_and(e_parte);
        saida.push_str(&texto[i..ini]);
        saida.push_str(if antes || depois { de } else { para });
        i = fim;
    }
    saida.push_str(&texto[i..]);
    saida
}

#[note("varrer-orfaos")]
fn varrer_orfaos(raiz: &Path, arquivos: &[Arquivo]) -> Result<Vec<PathBuf>, Erro> {
    let src = raiz.join("src");
    let Ok(entradas) = std::fs::read_dir(&src) else {
        return Ok(Vec::new());
    };
    let nossos: Vec<&Path> = arquivos.iter().map(|a| a.caminho.as_path()).collect();
    let mut removidos = Vec::new();
    for e in entradas.flatten() {
        let caminho = e.path();
        let Some(nome) = caminho.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !nome.ends_with(SUFIXO_GERADO) {
            continue;
        }
        let relativo = PathBuf::from("src").join(nome);
        if nossos.contains(&relativo.as_path()) {
            continue;
        }
        std::fs::remove_file(&caminho).map_err(|e| Erro::Io {
            caminho: caminho.display().to_string(),
            fonte: e,
        })?;
        removidos.push(relativo);
    }
    removidos.sort();
    Ok(removidos)
}

#[note("write-file")]
fn gravar(destino: &Path, conteudo: &str) -> Result<(), Erro> {
    // This `strip_prefix` is NOT what migrates legacy projects — what strips the BOM
    // from those that already have it is `desbomar()`, which runs before here in `escrever`.
    // It is defense in depth, and today it is dead code: mutating it does not cause
    // any test to fail, and that was verified.
    //
    // It stays because it costs one line and what it prevents is costly. `read_to_string`
    // returns the BOM as a CHARACTER, so any path that re-reads our
    // file and writes it back would stack a SECOND BOM; two
    // consecutive ones stop rustc with `unknown start of token: \u{feff}`, and that already
    // happened for real during a form rename.
    let conteudo = conteudo.strip_prefix('\u{feff}').unwrap_or(conteudo);
    gravar_bytes(destino, conteudo.as_bytes())
}

#[note("gravar-bytes")]
fn gravar_bytes(destino: &Path, bytes: &[u8]) -> Result<(), Erro> {
    let mut nome = destino.as_os_str().to_os_string();
    nome.push(".crawrad-tmp");
    let temporario = PathBuf::from(nome);

    // Both halves in a single expression, and **one** cleanup for both.
    // Separating them across two `?` already had a cost: cleanup was hung on `rename`,
    // and a `write` that failed halfway (disk full) exited through `?` leaving a
    // HALF-FINISHED `.crawrad-tmp` in the user's `src/`. It is not corruption — cargo
    // ignores what is not `.rs` —, but it is garbage carrying their filename, and
    // whoever encounters it thinks they lost code.
    //
    // With a single exit path, there is no way for one half to be fixed and the
    // other forgotten. This is intentional: it is cheaper than remembering.
    let feito =
        std::fs::write(&temporario, bytes).and_then(|()| std::fs::rename(&temporario, destino));

    if let Err(fonte) = feito {
        // Best-effort on purpose. If `write` failed on OPEN, there is no
        // temp file to remove; and if removal fails, the error that matters
        // to the caller is the one above, not the cleanup failure.
        let _ = std::fs::remove_file(&temporario);
        return Err(Erro::Io {
            caminho: destino.display().to_string(),
            fonte,
        });
    }
    Ok(())
}

#[note("desbomar")]
fn desbomar(destino: &Path) -> Result<bool, Erro> {
    if destino.extension().is_none_or(|e| e != "rs") {
        return Ok(false);
    }
    let b = std::fs::read(destino).map_err(|e| Erro::Io {
        caminho: destino.display().to_string(),
        fonte: e,
    })?;
    if !b.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Ok(false);
    }
    // Via `gravar_bytes`, not `fs::write`: this is the user file,
    // and an interrupted write would leave it truncated. See there.
    gravar_bytes(destino, &b[3..])?;
    Ok(true)
}

#[note("crescer-e-podar")]
fn crescer_e_podar(
    raiz: &Path,
    destino: &Path,
    arquivo: &Arquivo,
    generated_before: &str,
) -> Result<(bool, Vec<String>), Erro> {
    let atual = std::fs::read_to_string(destino).map_err(|e| Erro::Io {
        caminho: destino.display().to_string(),
        fonte: e,
    })?;

    let vizinhas = vizinhas(raiz, destino)?;
    let migrated =
        migrar_handlers_renomeados(&atual, &arquivo.adicoes, &vizinhas, generated_before);
    let grown = crescido(&migrated, &arquivo.adicoes);
    let cresceu = grown != atual;

    // **The additions ARE the list of what is bound.** `stubs` emits one per
    // event referenced in the document, so whoever is here is exactly what the
    // Inspector points to now. A second list, assembled another way, would be
    // a second truth about the same document — and the diverging one would be what
    // deletes the wrong handler.
    //
    // A handler **shared** by two components comes for free: it
    // appears once per bound event, so unbinding one of them leaves it in the
    // list because of the other, and it stays.
    let ligados: Vec<&str> = arquivo
        .adicoes
        .iter()
        .map(|a| a.nome_da_fn.as_str())
        .collect();
    let (final_, podados) = crate::rxp::podar(&grown, &ligados, &vizinhas);

    if final_ != atual {
        gravar(destino, &final_)?;
    }
    Ok((cresceu, podados))
}

#[note("migrar-handlers-renomeados")]
fn migrar_handlers_renomeados(
    atual: &str,
    adicoes: &[Adicao],
    vizinhas: &[String],
    generated_before: &str,
) -> String {
    let mut text = atual.to_string();
    let bound: Vec<&str> = adicoes.iter().map(|a| a.nome_da_fn.as_str()).collect();

    for addition in adicoes {
        let handler_name = &addition.nome_da_fn;
        if crate::elo::localizar_handler(&text, handler_name).is_some() {
            continue;
        }

        let Some((comp, event)) = extract_marker_pair(&addition.texto) else {
            continue;
        };

        struct Candidate {
            fn_name: String,
            x: String,
            marker_range: std::ops::Range<usize>,
        }

        let mut candidates = Vec::new();
        for (f, local) in crate::elo::handlers_do_arquivo(&text) {
            if bound.contains(&f.as_str()) {
                continue;
            }
            let Some(ext) = crate::elo::extensao_do_handler(&text, local.byte) else {
                continue;
            };
            let Some((marker_start, marker_end, marker_line)) =
                marker_line_above(&text, ext.item.start)
            else {
                continue;
            };
            let Some((x, ev)) = parse_marker_line(marker_line) else {
                continue;
            };
            let renamed = if x == comp {
                crate::elo::citado_fora(generated_before, &f, 0..0)
            } else {
                crawrad_ir::is_derived_handler_name(x, event, &f)
            };
            if ev == event && renamed {
                candidates.push(Candidate {
                    fn_name: f,
                    x: x.to_string(),
                    marker_range: marker_start..marker_end,
                });
            }
        }

        if candidates.len() == 1 {
            let candidate = &candidates[0];
            if !vizinhas
                .iter()
                .any(|v| crate::elo::citado_fora(v, &candidate.fn_name, 0..0))
            {
                let old_marker = format!("`{}.{event}`", candidate.x);
                let new_marker = format!("`{comp}.{event}`");
                let line = &text[candidate.marker_range.clone()];
                let new_line = line.replace(&old_marker, &new_marker);
                text.replace_range(candidate.marker_range.clone(), &new_line);
                text = trocar_palavra(&text, &candidate.fn_name, handler_name);
            }
        }
    }

    text
}

fn parse_marker_line(line: &str) -> Option<(&str, &str)> {
    let trimmed = line.trim();
    let content = trimmed.strip_prefix("/// `")?.strip_suffix('`')?;
    let (comp, event) = content.split_once('.')?;
    if comp.is_empty() || event.is_empty() {
        return None;
    }
    Some((comp, event))
}

fn extract_marker_pair(text: &str) -> Option<(&str, &str)> {
    for line in text.lines() {
        if let Some(pair) = parse_marker_line(line) {
            return Some(pair);
        }
    }
    None
}

fn marker_line_above(text: &str, fn_start: usize) -> Option<(usize, usize, &str)> {
    let marker_end = fn_start.checked_sub(1)?;
    let marker_end = if marker_end > 0 && text.as_bytes().get(marker_end - 1) == Some(&b'\r') {
        marker_end - 1
    } else {
        marker_end
    };
    let marker_start = text[..marker_end].rfind('\n').map_or(0, |p| p + 1);
    Some((marker_start, marker_end, &text[marker_start..marker_end]))
}

#[note("vizinhas")]
fn vizinhas(raiz: &Path, destino: &Path) -> Result<Vec<String>, Erro> {
    neighbours_editing(raiz, destino, &|_| None)
}

#[note("neighbours-editing")]
fn neighbours_editing(
    raiz: &Path,
    destino: &Path,
    editing: &dyn Fn(&Path) -> Option<String>,
) -> Result<Vec<String>, Erro> {
    let mut fontes = Vec::new();
    let mut pilha = vec![raiz.join("src")];
    while let Some(pasta) = pilha.pop() {
        let Ok(entradas) = std::fs::read_dir(&pasta) else {
            continue;
        };
        for e in entradas.flatten() {
            let caminho = e.path();
            if caminho.is_dir() {
                pilha.push(caminho);
                continue;
            }
            if caminho == destino || caminho.extension().is_none_or(|x| x != "rs") {
                continue;
            }
            let e_designer = caminho
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with(SUFIXO_GERADO));
            if e_designer {
                continue;
            }
            if let Some(t) = editing(&caminho).or_else(|| std::fs::read_to_string(&caminho).ok()) {
                fontes.push(t);
            }
        }
    }
    Ok(fontes)
}

#[note("crescido")]
fn crescido(atual: &str, adicoes: &[Adicao]) -> String {
    let mut novo = atual.to_string();
    for a in adicoes {
        // THE SAME question that navigation asks, and that is what matters: whatever
        // decides whether the handler exists and whatever finds where it is cannot have
        // different criteria.
        if crate::elo::localizar_handler(&novo, &a.nome_da_fn).is_some() {
            continue;
        }
        // The marker changed language when the generated code became
        // entirely English, and it may be in CRLF — what answers both
        // is [`crate::rxp::marca_em`]. Not finding the anchor sends the
        // handler to the end of the file: loses nothing, but splits the `impl`
        // in two.
        match (&a.ancora, crate::rxp::marca_em(&novo)) {
            (Some(anc), Some((pos, crlf))) => {
                // THE BLOCK FOLLOWS THE FILE'S LINE ENDINGS. A `\n` snippet
                // inside a `\r\n` file makes the unit mixed, and the
                // next editor normalizes the whole thing — an entire file
                // `git diff` because of a three-line handler.
                match crlf {
                    true => novo.insert_str(pos, &anc.texto.replace('\n', "\r\n")),
                    false => novo.insert_str(pos, &anc.texto),
                }
            }
            _ => novo.push_str(&a.texto),
        }
    }
    novo
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Relatorio {
    pub criados: Vec<PathBuf>,
    pub reescritos: Vec<PathBuf>,
    /// Existed and content was not touched: belong to the user.
    ///
    /// "Content" is literal. The file may have lost the BOM from a project
    /// generated before 01/09/2026 — three bytes nobody typed —, and nothing
    /// beyond that. See `desbomar`.
    pub preservados: Vec<PathBuf>,
    /// The user file that followed a form rename:
    /// `(from, to)`.
    pub renomeados: Vec<(PathBuf, PathBuf)>,
    /// Left over from an old name and deleted. Only `src/*.designer.rs`.
    pub removidos: Vec<PathBuf>,
    /// Existed and received what was missing, without losing what they already had.
    pub cresceram: Vec<PathBuf>,
    /// Event handlers removed from the user file: the IDE created them,
    /// nobody filled them in, and the document no longer references them.
    ///
    /// See [`crate::rxp::podar`] for the rule and for the objection heard
    /// before it was adopted.
    ///
    /// **A pruned-only file is not included in `preservados`.** "Preserved"
    /// means intact content, and that word must continue to mean what it
    /// means — it proves, in the bar, that user code was not
    /// touched. A file that shrank appears here, by the name of what was removed.
    pub podados: Vec<String>,
}

impl Relatorio {
    #[note("relatorio-frase-da-poda")]
    pub fn frase_da_poda(&self) -> Option<String> {
        match self.podados.len() {
            0 => None,
            1 => Some("1 empty handler removed".into()),
            n => Some(format!("{n} empty handlers removed")),
        }
    }
}

#[note("codegen-error")]
#[derive(Debug, thiserror::Error)]
pub enum Erro {
    #[error("the document has no root")]
    SemRaiz,

    #[error("'{0}' is not a valid Rust identifier — rename the component")]
    NomeInvalido(String),

    #[error("{caminho}: {fonte}")]
    Io {
        caminho: String,
        #[source]
        fonte: std::io::Error,
    },

    /// The document carries a property or an event its class does not
    /// declare. Refused before a single file is planned, so no stub is ever
    /// written for an event that does not exist.
    #[error("{0}")]
    Catalog(#[from] crawrad_registry::class_property::ClassError),

    #[error("{0}")]
    Message(String),
}

// ------------------------------------------------------------------ helpers

#[note("reservadas")]
const RESERVADAS: &[&str] = &[
    // In use (2015)
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", // In use, starting from 2018
    "async", "await", "dyn",
    // Reserved for future use: they do nothing, yet cannot be an
    // identifier. This is the family that was missing.
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "try", "typeof",
    "unsized", "virtual", "yield", // Reserved by edition 2024
    "gen",
    // Infrastructure methods reserved by the generator (collides with pub fn new() / show())
    "new", "show",
];

#[note("identificador")]
pub fn identificador(nome: &str) -> Result<String, Erro> {
    let ok = !nome.is_empty()
        && !nome.starts_with(|c: char| c.is_ascii_digit())
        && nome.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        // The hole through which user code escaped. `___` passed all the
        // guards above — it is ASCII, does not start with a digit, and is a
        // legitimate Rust identifier — yet `tipo_do_form("___")`
        // yields `""`, because `split('_')` leaves no part behind. From that came
        // `impl  {` and a `trocar_palavra` with an empty target, which erases.
        //
        // The question is delegated to `crawrad-ir` on purpose: "a name requires a letter or
        // digit" is not a Rust rule, it is a rule of any target that derives
        // names from names. Calling the same function, both ends cannot
        // diverge. (`_` alone falls here too, and rightly so: it is a wildcard,
        // not an identifier.)
        && crawrad_ir::name_has_alphanumeric(nome);
    if !ok || RESERVADAS.contains(&nome) {
        return Err(Erro::NomeInvalido(nome.to_string()));
    }
    Ok(nome.to_string())
}

pub use crawrad_ir::{
    default_handler_name as handler_padrao, is_derived_handler_name as e_nome_derivado,
    legacy_handler_name as handler_padrao_legado,
};
use crawrad_note::note;

#[note("literal")]
pub fn literal(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            outro => out.push(outro),
        }
    }
    out.push('"');
    out
}

#[note("grid-cells-init")]
pub fn grid_cells_init(node: &crawrad_ir::Node) -> String {
    match node.props.get("cell_text") {
        Some(crawrad_ir::Value::Str(s)) if !s.is_empty() => format!(
            "crawrad_runtime::string_grid::Cells::from_text({})",
            literal(s)
        ),
        _ => "Default::default()".to_string(),
    }
}

#[note("numero")]
pub fn numero(v: f32) -> String {
    if v.fract() == 0.0 {
        format!("{v:.1}")
    } else {
        format!("{v}")
    }
}

#[note("sem-acelerador")]
pub fn sem_acelerador(t: &str) -> String {
    let mut saida = String::with_capacity(t.len());
    let mut chars = t.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '&' {
            if chars.peek() == Some(&'&') {
                chars.next();
                saida.push('&');
            }
            continue;
        }
        saida.push(c);
    }
    saida
}

#[note("classes-de-terceiro")]
pub fn classes_de_terceiro<'a>(
    doc: &crawrad_ir::Document,
    reg: &'a crawrad_registry::Registry,
) -> Vec<&'a str> {
    let mut v: Vec<&str> = doc
        .nodes
        .values()
        .filter_map(|n| reg.get(&n.class))
        .filter(|d| !d.crate_module.is_empty())
        .map(|d| d.class)
        .collect();
    v.sort_unstable();
    v.dedup();
    v
}

#[cfg(test)]
#[path = "test/lib/test.rs"]
mod tests;

#[cfg(test)]
#[path = "test/class_by_name/test.rs"]
mod class_by_name_test;

#[cfg(test)]
#[path = "test/reference_to_code/test.rs"]
mod reference_to_code_test;

#[cfg(test)]
#[path = "test/emittable_piece/test.rs"]
mod emittable_piece_test;

#[cfg(test)]
#[path = "test/piece_traverses/test.rs"]
mod piece_traverses_test;

#[cfg(test)]
#[path = "test/verbs/test.rs"]
mod verbs_test;

#[cfg(test)]
#[path = "test/verb_bind/test.rs"]
mod verb_bind_test;

#[cfg(test)]
#[path = "test/verb_child/test.rs"]
mod verb_child_test;

#[cfg(test)]
#[path = "test/third_party_generates/test.rs"]
mod third_party_generates_test;

#[cfg(test)]
#[path = "test/scaffold/test.rs"]
mod scaffold_test;

#[cfg(test)]
#[path = "test/portadb_to_code/test.rs"]
mod portadb_to_code_test;

#[cfg(test)]
#[path = "test/data_binding_to_code/test.rs"]
mod data_binding_to_code_test;

#[cfg(test)]
#[path = "test/gauge_does_both/test.rs"]
mod gauge_does_both_test;

#[cfg(test)]
#[path = "test/drawing_traverses/test.rs"]
mod drawing_traverses_test;

#[cfg(test)]
#[path = "test/the_box/test.rs"]
mod the_box_test;

#[cfg(test)]
#[path = "test/third_party_event/test.rs"]
mod third_party_event_test;

#[cfg(test)]
#[path = "test/shared_geometry/test.rs"]
mod shared_geometry_test;

#[cfg(test)]
#[path = "test/handler_name/test.rs"]
mod handler_name_test;

#[cfg(test)]
#[path = "test/no_allow/test.rs"]
mod no_allow_test;

#[cfg(test)]
#[path = "test/tab_sheet/test.rs"]
mod tab_sheet_test;

#[cfg(test)]
#[path = "test/char_case/test.rs"]
mod char_case_test;

#[cfg(test)]
#[path = "test/state_is_read/test.rs"]
mod state_is_read_test;

#[cfg(test)]
#[path = "test/dead_egui_api/test.rs"]
mod dead_egui_api_test;

#[cfg(test)]
#[path = "test/mark_crlf/test.rs"]
mod mark_crlf_test;

#[cfg(test)]
#[path = "test/name_translation/test.rs"]
mod name_translation_test;

#[cfg(test)]
#[path = "test/codegen_parity/test.rs"]
mod codegen_parity_test;

#[cfg(test)]
#[path = "test/selection_past_end/test.rs"]
mod selection_past_end_test;

#[cfg(test)]
#[path = "test/list_box_events/test.rs"]
mod list_box_events_test;

#[cfg(test)]
#[path = "test/emitted_declared_crates/test.rs"]
mod emitted_declared_crates_test;

#[cfg(test)]
#[path = "test/rename_handler/test.rs"]
mod rename_handler_test;

#[cfg(test)]
#[path = "test/bound_grid/test.rs"]
mod bound_grid_test;

#[cfg(test)]
#[path = "test/property_map/test.rs"]
mod property_map_test;

#[cfg(test)]
#[path = "test/dataset_actions/test.rs"]
mod dataset_actions_test;

#[cfg(test)]
#[path = "test/show_hint_const/test.rs"]
mod show_hint_const_test;
