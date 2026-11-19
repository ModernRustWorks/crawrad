//! The gate of the EXPLORER panel of the Makepad shell.
//!
//! # What it guards, and what it deliberately does NOT
//!
//! The rule that decides **who owns a file** is not measured here: it lives in
//! `explorer_model.rs` and has twenty-six tests of its own, which both shells
//! inherit because both compile that file. Re-testing it here would be a
//! second ruler over one rule, and the second is the one that goes out of date.
//!
//! What is measured here is what this panel adds: the ORDER of the rows, the
//! template each one asks for, the drawing each one asks for, and the two
//! pieces of state the panel owns — the folds and the legend. The fold test is
//! the reason this file exists, and the comment in `explorer.rs` of the egui
//! shell says why in one line:
//!
//! > *"a field there would be one more state to clear when the document
//! > changes, and forgetting to clear it has already cost dearly in this
//! > house."*
//!
//! The defect it is talking about wiped the user's rename in silence. A
//! comment asking for care would not have caught it; a test that switches
//! document and charges the state does.
//!
//! # The oracle for a row is its TEMPLATE, not its type
//!
//! A row is a `Box<dyn ExplorerRow>` (see `ide_app/explorer/row.rs`), so there
//! is no variant to match on — and that is the point of the shape, not a cost
//! of it. What the tests below ask a row is what the panel asks it: which
//! template it wants, what it says, and what a click on it runs.

use std::collections::{BTreeSet, HashSet};
use std::path::{Path, PathBuf};

use super::explorer::{self, Activation, Folds, Row, row::ExplorerRow};

/// The Makepad shell — the whole file, as text.
use super::casca::CASCA;

/// The theme of the Makepad shell — where the row templates live.
const TEMA: &str = super::tests::TEMA_COMPLETO;

/// A name long enough to have broken the panel, and it is the real one.
///
/// `formPrincipal.designer.rs` is what the reference form generates, and it is
/// the longest name the list shows. It is spelled out here and not made up,
/// because a test about "a long name" that uses `aaaaaaaa.rs` stops being
/// about the screen the owner looked at.
const NOME_LONGO: &str = "formPrincipal.designer.rs";

/// The bytes of Codicons — the SAME ones the theme embeds.
const CODICONS: &[u8] = include_bytes!("../../../assets/fontes/codicon.ttf");

/// `CASCA` with no whitespace at all — immune to whatever `rustfmt` picks.
fn compacto() -> String {
    CASCA.chars().filter(|c| !c.is_whitespace()).collect()
}

/// A DSL of a file with the comments **taken out**, and then compacted.
///
/// Taking the comments out is what makes an assertion about the DSL mean
/// something: this theme explains `RightWrap` in prose four times, and a test
/// that searched the raw text would be green because of the paragraph that
/// says why `RightWrap` is NOT there.
///
/// `://` is protected first — `font("crate://self/…")` is a real value in this
/// file, and eating the rest of that line would silently move the boundaries
/// every slice below depends on.
fn dsl_sem_comentarios(fonte: &str) -> String {
    const URL: &str = "\u{0}URL\u{0}";
    fonte
        .replace("://", URL)
        .lines()
        .map(|l| l.split_once("//").map(|p| p.0).unwrap_or(l))
        .collect::<String>()
        .replace(URL, "://")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

/// The body of one `live_design!` template, from its name to the next one.
fn molde(fonte: &str, nome: &str, seguinte: &str) -> String {
    let d = dsl_sem_comentarios(fonte);
    let inicio = d
        .find(&format!("pub{nome}=<"))
        .unwrap_or_else(|| panic!("template `{nome}` vanished from the theme"));
    let fim = d[inicio..]
        .find(&format!("pub{seguinte}=<"))
        .unwrap_or_else(|| panic!("template `{seguinte}` vanished from the theme"));
    d[inicio..inicio + fim].to_string()
}

/// A clean root under `%TEMP%`, like the `crawrad-codegen` tests do.
fn raiz(nome: &str) -> PathBuf {
    let r = std::env::temp_dir().join(nome);
    let _ = std::fs::remove_dir_all(&r);
    std::fs::create_dir_all(&r).expect("create test root");
    r
}

/// A group with one member on disk, and the path of a form inside it.
fn grupo_com_membro(nome: &str, membro: &str) -> (PathBuf, PathBuf) {
    let r = raiz(nome);
    std::fs::write(
        r.join("Cargo.toml"),
        format!("[workspace]\nmembers = [\"{membro}\"]\n"),
    )
    .expect("write group manifest");
    let pasta = r.join(membro);
    std::fs::create_dir_all(pasta.join("src")).expect("create member");
    std::fs::write(
        pasta.join("Cargo.toml"),
        format!("[package]\nname = \"{membro}\"\n"),
    )
    .expect("write member manifest");
    let form = pasta.join("src").join("form_main.crawform");
    std::fs::write(&form, "{}").expect("write design");
    (r, form)
}

/// The rows of a panel rooted at `r`, with the legend open or closed.
fn linhas(r: &Path, documento: &Path, dobras: &BTreeSet<PathBuf>, legenda: bool) -> Vec<Row> {
    let plano = Some(Ok(Vec::new()));
    let unsaved = BTreeSet::new();
    explorer::rows(&explorer::View {
        problems: &explorer::ProblemMap::new(),
        unsaved: &unsaved,
        root: Some(r),
        document: Some(documento),
        plan: &plano,
        folds: dobras,
        legend_open: legenda,
        open_editors_open: false,
        outline_open: false,
        timeline_open: false,
        dependencies_open: false,
        timeline: Some(&[]),
    })
}

// --------------------------------------------------------------- the folds

/// **The guard this file exists for.**
///
/// Switching document rebuilds the fold state, so the state of one document can
/// never be read as the state of another. It is measured through the only door
/// there is — [`Folds::open`] — because that is the point: there is no second
/// call anybody could forget to make.
#[test]
fn folds_start_over_when_the_document_changes() {
    let a_form = Path::new("/g/a/src/form_a.crawform");
    let a_member = Path::new("/g/a");
    let b_form = Path::new("/g/b/src/form_b.crawform");
    let b_member = Path::new("/g/b");

    let mut folds = Folds::default();
    // Document A: the active member starts open, as in egui.
    assert!(folds.open(Some(a_form), Some(a_member)).contains(a_member));
    // And the user opens a second member manually.
    folds.toggle(Some(a_form), Some(a_member), Path::new("/g/outro"));
    assert_eq!(folds.open(Some(a_form), Some(a_member)).len(), 2);

    // Document B: NOTHING from A survives.
    let depois = folds.open(Some(b_form), Some(b_member));
    assert_eq!(
        depois.iter().collect::<Vec<_>>(),
        vec![&b_member.to_path_buf()],
        "the fold of the previous document crossed the switch. It is state \
         someone forgets to clear, and in this house that oversight has already \
         silently wiped the user's rename."
    );
}

/// `toggle` also cannot write to another document's set.
#[test]
fn a_toggle_on_a_new_document_never_writes_into_the_old_set() {
    let a = Path::new("/g/a/src/form_a.crawform");
    let b = Path::new("/g/b/src/form_b.crawform");
    let mut folds = Folds::default();
    folds.toggle(Some(a), Some(Path::new("/g/a")), Path::new("/g/x"));
    // Without any cleanup call in between: `toggle` of document B starts
    // from B's set.
    folds.toggle(Some(b), Some(Path::new("/g/b")), Path::new("/g/y"));
    let set = folds.open(Some(b), Some(Path::new("/g/b")));
    assert!(!set.contains(Path::new("/g/x")));
    assert!(set.contains(Path::new("/g/y")));
}

/// **The legend starts CLOSED, and closes again when the document changes.**
///
/// The decision is the owner's: the legend consumed a third of a narrow panel,
/// always. A new document that brought it open would undo the decision on the
/// first form switch — and it is exactly the type of state that one
/// forgets to clear.
#[test]
fn the_legend_starts_closed_and_closes_again_on_a_new_document() {
    let a = Path::new("/g/a/src/form_a.crawform");
    let b = Path::new("/g/b/src/form_b.crawform");
    let mut folds = Folds::default();
    assert!(
        !folds.legend_open(Some(a), Some(Path::new("/g/a"))),
        "the legend started OPEN — it is the third of the panel the owner ordered removed"
    );
    folds.toggle_legend(Some(a), Some(Path::new("/g/a")));
    assert!(folds.legend_open(Some(a), Some(Path::new("/g/a"))));
    assert!(
        !folds.legend_open(Some(b), Some(Path::new("/g/b"))),
        "the open legend crossed the document switch"
    );
}

/// Closing the member node removes its children from the list — and only them.
#[test]
fn a_closed_member_hides_its_files_and_nothing_else() {
    let (r, form) = grupo_com_membro("crawrad_explorer_dobra", "app");
    let mut folds = Folds::default();

    let abertas = folds.open(Some(&form), Some(&r.join("app"))).clone();
    let aberto = linhas(&r, &form, &abertas, false);
    folds.toggle(Some(&form), Some(&r.join("app")), &r.join("app"));
    let fechadas = folds.open(Some(&form), Some(&r.join("app"))).clone();
    let fechado = linhas(&r, &form, &fechadas, false);

    assert!(
        aberto.len() > fechado.len(),
        "closing the member removed nothing from the list"
    );
    assert!(
        fechado.iter().any(|l| l.template().starts_with("Member")),
        "closing the member removed the member itself from the list — there would be no \
         way to reopen it"
    );
    assert!(
        !fechado.iter().any(|l| l.template().starts_with("Form")),
        "the closed member still shows its designs"
    );
}

/// **A closed legend shows the HEADER and nothing else.**
///
/// This is what separates *collapsed* from *deleted*: closed, it remains on screen as
/// a single row, and anyone who has never seen the badges has a place to ask. In this house,
/// what is removed is later forgotten.
#[test]
fn a_closed_legend_still_shows_its_header() {
    let (r, form) = grupo_com_membro("crawrad_explorer_legenda", "app");
    let mut folds = Folds::default();
    let dobras = folds.open(Some(&form), Some(&r.join("app"))).clone();

    let fechada = linhas(&r, &form, &dobras, false);
    let aberta = linhas(&r, &form, &dobras, true);

    let cabecalho = |v: &[Row]| {
        v.iter()
            .filter(|l| l.template() == "Section" && l.text() == explorer::CAPTION_LEGEND)
            .count()
    };
    assert_eq!(
        cabecalho(&fechada),
        1,
        "the closed legend did not leave the header on screen — anyone who has never seen the \
         badges would have nowhere to ask"
    );
    assert_eq!(cabecalho(&aberta), 1);
    assert!(
        aberta.len() > fechada.len(),
        "opening the legend added no rows"
    );
    assert!(
        !fechada.iter().any(|l| l.template().starts_with("Legend")),
        "the closed legend still lists the pairs"
    );
}

/// The header chevron CHANGES with state — otherwise it lies about what it does.
#[test]
fn the_section_chevron_says_which_way_it_goes() {
    let (r, form) = grupo_com_membro("crawrad_explorer_chevron", "app");
    let mut folds = Folds::default();
    let dobras = folds.open(Some(&form), Some(&r.join("app"))).clone();

    let marca = |aberta: bool| {
        linhas(&r, &form, &dobras, aberta)
            .into_iter()
            .find(|l| l.template() == "Section")
            .and_then(|l| l.chevron())
            .expect("legend header has chevron")
    };
    assert_eq!(marca(false), explorer::icon::CHEVRON_CLOSED);
    assert_eq!(marca(true), explorer::icon::CHEVRON_OPEN);
}

// ------------------------------------------------------- the click is an ACTION

/// **A member node folds VIA THE CATALOGUE, and takes the folder with it.**
///
/// The house rule is one line: *can an agent do this via a
/// tool, without a mouse?* Before this cycle the shell folded the node by modifying its
/// own field, and the answer was no through any door.
#[test]
fn folding_a_member_runs_a_catalogue_action_with_its_own_folder() {
    let (r, form) = grupo_com_membro("crawrad_explorer_acao", "app");
    let mut folds = Folds::default();
    let dobras = folds.open(Some(&form), Some(&r.join("app"))).clone();

    let membro = linhas(&r, &form, &dobras, false)
        .into_iter()
        .find(|l| l.template().starts_with("Member"))
        .expect("the list has the member node");
    match membro.activation() {
        Activation::Run(acao, args) => {
            assert_eq!(acao, "explorer.toggle_folder");
            assert_eq!(
                args.as_text("path").map(PathBuf::from),
                Some(r.join("app")),
                "the argument is not the folder of the drawn row — reading the \
                 tree again on click is the window where what was \
                 drawn and what runs disagree"
            );
        }
        outra => panic!("the member node stopped being an action: {outra:?}"),
    }
}

/// And the legend header too.
#[test]
fn folding_the_legend_runs_a_catalogue_action() {
    let (r, form) = grupo_com_membro("crawrad_explorer_acao_legenda", "app");
    let mut folds = Folds::default();
    let dobras = folds.open(Some(&form), Some(&r.join("app"))).clone();

    let secao = linhas(&r, &form, &dobras, false)
        .into_iter()
        .find(|l| l.template() == "Section")
        .expect("the list has the legend header");
    assert_eq!(
        secao.activation(),
        Activation::Run(
            explorer::ACTION_TOGGLE_LEGEND,
            crawrad_actions::Args::none()
        ),
    );
}

/// **Every action a row triggers exists in the catalogue.**
///
/// Without this, `execute_with` would answer `unknown action: …` in the status bar
/// and the chevron would remain still — a control that accepts the gesture and does nothing,
/// which is the family of defect that this shell already removed from the bar.
#[test]
fn every_action_a_row_asks_for_is_in_the_catalogue() {
    for linha in explorer::sample_rows() {
        let Activation::Run(id, _) = linha.activation() else {
            continue;
        };
        assert!(
            crawrad_actions::by_id(id).is_some(),
            "row `{}` triggers `{id}`, which is not in the catalog — clicking \
             would answer `unknown action` and the panel would not move",
            linha.template()
        );
    }
}

/// The OPEN form is not a target: reopening it would discard undo.
#[test]
fn the_open_form_is_not_a_target() {
    let (r, form) = grupo_com_membro("crawrad_explorer_form_atual", "app");
    let mut folds = Folds::default();
    let dobras = folds.open(Some(&form), Some(&r.join("app"))).clone();

    let atual = linhas(&r, &form, &dobras, false)
        .into_iter()
        .find(|l| l.template() == "FormCurrent")
        .expect("the open form is in the list");
    assert_eq!(atual.activation(), Activation::Inert);
}

// ----------------------------------------------------------- the root and the list

/// The group root is the folder of the manifest that DECLARES the member.
#[test]
fn the_group_root_is_the_folder_whose_manifest_declares_the_member() {
    let (r, form) = grupo_com_membro("crawrad_explorer_raiz", "app");
    assert_eq!(explorer::group_root_of(&form).as_deref(), Some(r.as_path()));
}

/// A loose package is its own group — the same that `cargo` sees.
#[test]
fn a_lone_package_is_its_own_group() {
    let r = raiz("crawrad_explorer_solto");
    std::fs::create_dir_all(r.join("src")).expect("create src");
    std::fs::write(r.join("Cargo.toml"), "[package]\nname = \"solo\"\n").expect("manifest");
    let form = r.join("src").join("form_main.crawform");
    std::fs::write(&form, "{}").expect("design");
    assert_eq!(explorer::group_root_of(&form).as_deref(), Some(r.as_path()));
}

/// With no folder, the panel SAYS what happened instead of being blank.
#[test]
fn with_no_folder_the_panel_says_so() {
    let plano = None;
    let dobras = BTreeSet::new();
    let unsaved = BTreeSet::new();
    let lista = explorer::rows(&explorer::View {
        problems: &explorer::ProblemMap::new(),
        unsaved: &unsaved,
        root: None,
        document: None,
        plan: &plano,
        folds: &dobras,
        legend_open: false,
        open_editors_open: false,
        outline_open: false,
        timeline_open: false,
        dependencies_open: false,
        timeline: Some(&[]),
    });
    assert_eq!(lista.len(), 1);
    assert_eq!(lista[0].template(), "Note");
    assert_eq!(lista[0].text(), explorer::NO_FOLDER);
}

/// A file's hint always says something, and never the same for everyone.
///
/// It is the hover tooltip from the egui shell, and in this shell it is the ONLY way
/// for the phrase to reach the user — there is no `on_hover_text` here, and the reason is
/// measured in the `selo` of `CrawradBadgeRow`.
#[test]
fn every_file_hint_says_something_and_they_differ() {
    use crate::view_models::explorer_model::FileState;
    use crawrad_codegen::Dono;
    use explorer::rows::file::hint;
    let estados = [
        (FileState::OnDisk, Some(Dono::Gerador)),
        (FileState::OnDisk, Some(Dono::Usuario)),
        (FileState::Pending, Some(Dono::PrimeiraVez)),
        (FileState::User, Some(Dono::Usuario)),
        (FileState::Stale, None),
    ];
    let frases: BTreeSet<String> = estados.iter().map(|(e, d)| hint(*e, *d)).collect();
    assert_eq!(
        frases.len(),
        estados.len(),
        "two combinations of (state, owner) give the SAME phrase. A one-word \
         badge is short enough to be guessed wrong, and the phrase is what \
         separates work that stays from work that vanishes."
    );
    assert!(frases.iter().all(|f| f.len() > 20));
}

// --------------------------------------------------------------- the DRAWINGS

/// The code points covered by Codicons, read from the file's `cmap`.
///
/// It is the same thirty-line reader that the chrome gate uses
/// (`src/test/ide_app/icon.rs`), and it repeats because `crawrad-ide` is a
/// BINARY crate: there is no way to share test code between the two binaries.
fn codicons_code_points() -> HashSet<u32> {
    let u16_at = |o: usize| u16::from_be_bytes([CODICONS[o], CODICONS[o + 1]]);
    let u32_at = |o: usize| {
        u32::from_be_bytes([
            CODICONS[o],
            CODICONS[o + 1],
            CODICONS[o + 2],
            CODICONS[o + 3],
        ])
    };
    let tables = u16_at(4) as usize;
    let cmap = (0..tables)
        .map(|i| 12 + 16 * i)
        .find(|&o| &CODICONS[o..o + 4] == b"cmap")
        .map(|o| u32_at(o + 8) as usize)
        .expect("Codicons has `cmap`");

    let mut out = HashSet::new();
    for i in 0..u16_at(cmap + 2) as usize {
        let sub = cmap + u32_at(cmap + 8 + 8 * i) as usize;
        if u16_at(sub) != 4 {
            continue;
        }
        let segs = u16_at(sub + 6) as usize / 2;
        for s in 0..segs {
            let end = u16_at(sub + 14 + 2 * s) as u32;
            let start = u16_at(sub + 16 + 2 * segs + 2 * s) as u32;
            if start == 0xFFFF {
                continue;
            }
            out.extend(start..=end);
        }
    }
    out
}

/// **Every icon a row requests exists in Codicons.**
///
/// A code point that the font does not have is an EMPTY BOX on the screen, and nothing
/// complains. The family has only one face — there is no fallback behind it.
#[test]
fn every_explorer_icon_exists_in_codicons() {
    let tabela = codicons_code_points();
    assert!(
        tabela.len() > 500,
        "the `cmap` reader only found {} code points; it is broken, and \
         an oracle that lies is worse than no oracle",
        tabela.len()
    );
    for linha in explorer::sample_rows() {
        for (onde, glifo) in [("icon", linha.icon()), ("chevron", linha.chevron())] {
            let Some(g) = glifo else { continue };
            assert_eq!(
                g.chars().count(),
                1,
                "the `{onde}` of row `{}` has more than one character. An icon \
                 font has no SPACE: what is left is an empty box.",
                linha.template()
            );
            let cp = g.chars().next().unwrap() as u32;
            assert!(
                tabela.contains(&cp),
                "the `{onde}` of row `{}` points to U+{cp:04X}, which \
                 Codicons does NOT have. On screen this is an empty box, and nothing \
                 complains.",
                linha.template()
            );
        }
    }
    // The countermeasure: a text character is NOT there. Without this the test
    // would pass with a `cmap` reader returning the entire alphabet.
    assert!(!tabela.contains(&('A' as u32)));
}

/// **The user's file and the generator's file are not drawn the same.**
///
/// The core promise of the product is that what the user typed survives the
/// next `Generate`. It is stated three times — the drawing, the COLOR of the drawing, and the
/// badge — and here we require what nothing else covers: two owners must choose
/// different templates, because the template is what carries the color.
#[test]
fn the_users_file_and_ours_are_not_drawn_the_same() {
    use crate::view_models::explorer_model::FileState;
    use crawrad_codegen::Dono;
    use explorer::rows::file::File;
    let molde =
        |dono| File::new(Path::new("src/main.rs"), FileState::OnDisk, Some(dono), 1).template();
    assert_ne!(
        molde(Dono::Gerador),
        molde(Dono::Usuario),
        "the user's file and the one rewritten on each generation request the SAME \
         template. The icon color is one of the three carriers of the most \
         valuable information in this panel."
    );
    assert_eq!(
        molde(Dono::PrimeiraVez),
        molde(Dono::Usuario),
        "`PrimeiraVez` is ours ONCE and the user's from then on — painting it \
         as ours would be the panel promising less than the generator delivers"
    );
}

/// The manifest is not a `.rs`, and is not drawn like one.
#[test]
fn the_manifest_has_a_drawing_of_its_own() {
    use crate::view_models::explorer_model::FileState;
    use crawrad_codegen::Dono;
    use explorer::rows::file::File;
    let icone = |nome: &str| {
        File::new(Path::new(nome), FileState::OnDisk, Some(Dono::Usuario), 1)
            .icon()
            .expect("every file has an icon")
    };
    assert_eq!(icone("Cargo.toml"), explorer::icon::MANIFEST);
    assert_eq!(icone("src/main.rs"), explorer::icon::SOURCE);
}

// ------------------------------------------------------------ the DSL gate

/// **Every template requested by a row is declared in the DSL.**
///
/// The central trap of Makepad: a `kind` that matches no template makes
/// `PortalList::item` return an empty widget and the row draws NOTHING, without
/// crashing (`portal_list.rs:601` only warns in the log).
///
/// The names come from `ExplorerRow::template`, not from a list written here:
/// written twice, this test would stay green for a name that the drawing
/// no longer uses.
#[test]
fn every_explorer_template_is_declared_in_the_dsl() {
    let c = compacto();
    for linha in explorer::sample_rows() {
        let nome = linha.template();
        assert!(
            c.contains(&format!("{nome}=<")),
            "template `{nome}`, requested by an Explorer row, is not \
             declared in the panel DSL"
        );
    }
}

/// And the list is populated by code, not written in the DSL.
#[test]
fn the_explorer_list_is_fed_from_rust() {
    let c = compacto();
    assert!(
        c.contains("explorer_list=<PortalList>"),
        "Explorer stopped being a code-fed list"
    );
    assert!(
        c.contains("self.linhas_do_explorer()"),
        "the Explorer rows stopped being assembled from the document: the panel \
         would show whatever was on screen in the previous frame"
    );
}

/// **The row painter has no `match` on row type.**
///
/// It is the composition rule of this cycle: a new row type costs CREATING A
/// FILE, not editing a `match` that lives in another. The painter writes the
/// four contract names and nothing else; what a template does not declare returns
/// an empty widget, and writing to an empty widget does nothing.
#[test]
fn the_row_painter_knows_nothing_about_kinds_of_row() {
    let corpo = CASCA
        .split_once("fn escrever_linha_do_explorer")
        .expect("Explorer painter still exists")
        .1
        .split_once("\n    }")
        .expect("painter body terminates")
        .0;
    assert!(
        !corpo.contains("match "),
        "the Explorer painter has a `match` again: a new row type would \
         cost an edit here in addition to its own file — and the `match` \
         that was here ended in `_ =>`, so the new type entered muted"
    );
    for campo in ["face.marca", "face.icone", "face.nome", "face.selo"] {
        assert!(
            corpo.contains(campo),
            "the painter stopped writing `{campo}` — the row contract \
             of this panel has four names, and what is not written remains \
             blank"
        );
    }
}

// ------------------------------------------------ the BADGE does not drop a line

/// **The badge never drops to the next line, and the mechanism belongs to the framework.**
///
/// # The defect this test locks
///
/// Seen on screen by the owner on 16/09/2026: `formPrincipal.designer.rs` — the longest
/// name in the list — pushed `Rewritten` to the NEXT line,
/// aligned to the left, while `main.rs  Rewritten` stayed on the same line.
/// Broken like this, the badge reads like **its own row in the tree**: glancing at it,
/// one counts an extra item, and the badge is the core product contract — who
/// owns the file. Ambiguity about WHICH file it belongs to is the worst
/// possible place for it.
///
/// # Why the proof is the SHAPE, and not a pixel
///
/// What guarantees the fix is not the length of this name: it is that in
/// `Flow::Right` a child with `width: Fill` is DEFERRED (`turtle.rs:165-177`) and the
/// others are laid out before it (`turtle.rs:392-394`). With the name deferred, the
/// badge is positioned without depending on the size of the name, and **there is no column
/// width where it drops to the next line**. Measuring a pixel would prove one name at one
/// width; measuring the shape proves all.
///
/// Pixel measurement, in fact, is not available without a window: `Label::draw_text`
/// is a private field (`makepad-widgets-1.0.0/src/label.rs:206`), so
/// `DrawText::layout` — which is `&mut Cx` and would run headless
/// (`makepad-draw-1.0.0/src/shader/draw_text.rs:323`) — has nowhere to get
/// the `text_style` that the template applied.
#[test]
fn the_badge_can_never_drop_to_the_next_line() {
    let face = molde(TEMA, "CrawradBadgeRow", "CrawradBadgeRowMuted");
    assert!(
        !face.contains("flow:RightWrap"),
        "the badge row returned to `RightWrap`. That flow breaks the BADGE \
         onto the next line (`turtle.rs:571-578`), and that was exactly \
         what the owner saw: `Rewritten` of `formPrincipal.designer.rs` reading \
         like an extra item in the tree."
    );
    assert!(
        face.contains("flow:Right,"),
        "the badge row `face` stopped being `flow: Right` — that is what \
         causes the name's `Fill` to be DEFERRED (`turtle.rs:165-177`) and the badge to be \
         placed first, without depending on the size of the name"
    );
    let nome = face
        .split_once("nome=<CrawradLabel>{")
        .expect("the badge row still has `nome`")
        .1;
    assert!(
        nome.starts_with("width:Fill"),
        "`nome` in the badge row stopped being `Fill`: without the deferred child, \
         the badge contends for the line with it again"
    );
    let selo = face
        .split_once("selo=<CrawradLabel>{")
        .expect("the badge row still has `selo`")
        .1;
    assert!(
        !selo.starts_with("width:Fill"),
        "the BADGE became `Fill`. Two deferred children split the remainder in half \
         (`turtle.rs:392-394`), and the badge would occupy half a column"
    );
}

/// And the long name remains ONE row of the model, carrying its own badge.
///
/// The other way to "fix" what the owner saw would be for the panel to
/// emit the badge as its own row — which would leave the screen looking like the defect.
/// This test is what prevents that fix.
#[test]
fn a_long_file_name_is_still_one_row_carrying_its_own_badge() {
    use crate::view_models::explorer_model::FileState;
    use crawrad_codegen::Dono;
    use explorer::rows::file::File;
    let linha = File::new(
        Path::new(&format!("src/{NOME_LONGO}")),
        FileState::OnDisk,
        Some(Dono::Gerador),
        1,
    );
    assert_eq!(
        linha.text(),
        NOME_LONGO,
        "the name was truncated by the model"
    );
    assert_eq!(
        linha.badge(),
        crate::view_models::explorer_model::badge_for(Dono::Gerador),
        "the badge stopped traveling on the same line as the name — detached, it reads as \
         an extra item in the tree"
    );
}

/// **Click dispatch also knows nothing about which kinds of rows exist.**
#[test]
fn the_click_dispatch_knows_nothing_about_kinds_of_row() {
    let c = compacto();
    assert!(
        c.contains("linhas.get(i).map(|l|l.activation())"),
        "Explorer click stopped asking the ROW what it does"
    );
    for variante in [
        "explorer::Activation::Run(acao,args)",
        "explorer::Activation::OpenForm(caminho)",
        "explorer::Activation::Inert",
    ] {
        assert!(
            c.contains(variante),
            "click dispatch stopped handling `{variante}` — and the `match` \
             on an `Activation` is exhaustive on purpose: a new variant does not \
             compile until someone chooses what it does"
        );
    }
    // AND THE ROW NOTE IS WRITTEN AFTER THE COMMAND.
    //
    // `Activation::Explain` was removed on 17/09/2026: the file row started
    // OPENING the file in a tab (`editor.open`), and the phrase indicating the owner —
    // the most valuable information in the panel (ADR 0016) — moved to the slot
    // `ExplorerRow::note`, which is not a command.
    //
    // The ORDER is what this section guards: written before `execute_with`, its
    // generic `opened …` would overwrite the phrase, and the panel would return to not
    // stating who owns the file — without anything breaking.
    assert!(
        c.contains("nota=linhas.get(i).and_then(|l|l.note());"),
        "Explorer click stopped asking the row for its note"
    );
    // The order is measured by POSITION, not by text proximity:
    // `compacto()` removes whitespace and **leaves comments**, so
    // a paragraph between the two lines would separate one from the other without changing
    // anything in the program.
    let escrita = c
        .find("ifletSome(frase)=nota{self.status=Some(frase);")
        .expect(
            "the row note stopped being written to the bar. The panel stops \
             saying who owns the file, and nothing breaks",
        );
    let despacho = c
        .find("matchescolhida{")
        .expect("Explorer click dispatch vanished");
    assert!(
        escrita > despacho,
        "the row note is written BEFORE the command. `execute_with` writes \
         its own status after it, the generic `opened …` erases it, and the \
         panel returns to not saying who owns the file"
    );
}

/// Additional card sections in the modern-editor style (Ref 29) appear in the list
/// and fold independently.
#[test]
fn the_explorer_has_independent_card_sections() {
    let (r, form) = grupo_com_membro("crawrad_explorer_cards", "app");
    let mut folds = Folds::default();
    let dobras = folds.open(Some(&form), Some(&r.join("app"))).clone();
    let unsaved = BTreeSet::new();
    let view = explorer::View {
        problems: &explorer::ProblemMap::new(),
        unsaved: &unsaved,
        root: Some(&r),
        document: Some(&form),
        plan: &Some(Ok(Vec::new())),
        folds: &dobras,
        legend_open: false,
        open_editors_open: true,
        outline_open: true,
        timeline_open: true,
        dependencies_open: true,
        timeline: Some(&[]),
    };
    let out = explorer::rows(&view);
    for caption in [
        explorer::CAPTION_OUTLINE,
        explorer::CAPTION_TIMELINE,
        explorer::CAPTION_DEPENDENCIES,
        explorer::CAPTION_OPEN_EDITORS,
    ] {
        assert!(
            out.iter()
                .any(|l| l.template() == "Section" && l.text() == caption),
            "section `{caption}` vanished from the Explorer list"
        );
    }
}

/// The Timeline section lists local history entries when present, or a Note when empty.
#[test]
fn timeline_section_renders_entries_when_present_and_note_when_empty() {
    let (r, form) = grupo_com_membro("crawrad_explorer_timeline", "app");
    let mut folds = Folds::default();
    let dobras = folds.open(Some(&form), Some(&r.join("app"))).clone();
    let unsaved = BTreeSet::new();

    let lines = vec![
        "Save · 14:32:10 · today".to_string(),
        "External · 09:15:30 · 18/10".to_string(),
    ];
    let view_with_entries = explorer::View {
        problems: &explorer::ProblemMap::new(),
        unsaved: &unsaved,
        root: Some(&r),
        document: Some(&form),
        plan: &Some(Ok(Vec::new())),
        folds: &dobras,
        legend_open: false,
        open_editors_open: false,
        outline_open: false,
        timeline_open: true,
        dependencies_open: false,
        timeline: Some(&lines),
    };
    let rows_with_entries = explorer::rows(&view_with_entries);
    let timeline_rows: Vec<&Row> = rows_with_entries
        .iter()
        .filter(|row| row.template() == "File" && row.text().contains(" · "))
        .collect();
    assert_eq!(timeline_rows.len(), 2);
    assert_eq!(timeline_rows[0].text(), "Save · 14:32:10 · today");
    assert_eq!(
        timeline_rows[0].activation(),
        Activation::Run(
            explorer::ACTION_OPEN_TIMELINE_ENTRY,
            crawrad_actions::Args::one("index", crawrad_ir::Value::Int(0)),
        )
    );
    assert_eq!(timeline_rows[1].text(), "External · 09:15:30 · 18/10");
    assert_eq!(
        timeline_rows[1].activation(),
        Activation::Run(
            explorer::ACTION_OPEN_TIMELINE_ENTRY,
            crawrad_actions::Args::one("index", crawrad_ir::Value::Int(1)),
        )
    );

    let view_empty = explorer::View {
        timeline: Some(&[]),
        ..view_with_entries
    };
    let rows_empty = explorer::rows(&view_empty);
    let note_row = rows_empty
        .iter()
        .find(|row| row.text() == "No local history for this file");
    assert!(note_row.is_some());
    if let Some(note) = note_row {
        assert_eq!(note.template(), "Note");
    }

    let view_off = explorer::View {
        timeline: None,
        ..view_with_entries
    };
    let rows_off = explorer::rows(&view_off);
    assert!(
        rows_off
            .iter()
            .any(|row| row.text().starts_with("Local history is off")),
        "history off must say so, not 'no history'"
    );
    assert!(
        !rows_off
            .iter()
            .any(|row| row.text() == "No local history for this file")
    );
}
