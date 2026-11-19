//! The gate of the Makepad shell's Object Inspector.
//!
//! It charges four things, and all four have failed somewhere in this house:
//!
//! 1. **Composition** — the panel reads the CATALOGUE. No property written by
//!    hand, no `match` by class, and a property new in `crawrad-registry` showing
//!    up without a line edited in the shell. The lack of it is what gave the
//!    Toolbox two lists, with `ActionList`, `ImageList` and `Timer` in the DSL
//!    and outside the table — three rows that reached the screen and armed
//!    nothing.
//! 2. **Coverage, derived** — every `PropKind` the registry declares has an
//!    editor here, and the list of kinds is read out of the **registry's own
//!    source text**. A kind that lands tomorrow is born charged. This is the
//!    one that closes the panel used to edit five of eleven.
//! 3. **No dead field** — every row either takes a gesture, or SAYS why it
//!    does not. It is the rule this house applied to the status bar and to the
//!    AI panel.
//! 4. **The wire** — the pure function is called, and the write goes through
//!    the catalogue of actions. A perfect `rows()` called by nobody passes
//!    every unit test (see the opening of `fios.rs`).
//!
//! What it does **not** cover: whether the `TextInput` of the row shows the
//! text on screen in the right colour and size. That is screen, and screen is
//! checked by looking.

use std::collections::BTreeSet;

use super::inspector::{self, Cell, Editor, Gesture, Outcome, Row, Slot, Tab};
use crawrad_ir::{Color, Document, Edges, Item, Node, Rect, Value};
use crawrad_registry::{Category, PropDesc, Props, Registry};

/// The target of the runtime that draws this shell's canvas.
const ALVO: &str = crawrad_runtime_makepad::ALVO;

/// The source the list of property kinds is read out of.
///
/// `include_str!` and not a disk read, for the same reason the effect coverage
/// gate gives: this way the text the test parses is, byte for byte, the text
/// the compiler just compiled. There is no window in which the gate measures
/// one version of the enum and the binary carries another.
const REGISTRY_PROPS: &str = include_str!("../../../../crawrad-registry/src/props.rs");

fn registro() -> Registry {
    Registry::builtin()
}

/// Nothing is folded open — the state most tests want.
fn fechado() -> BTreeSet<String> {
    BTreeSet::new()
}

/// A document with one component of the asked class, already carrying the
/// catalogue defaults — the same road `component.add` takes.
fn doc_com(classe: &str) -> (Document, Vec<String>) {
    let r = registro();
    let mut d = Document::new("form_main", "Form");
    let no = r
        .instantiate(classe, "alvo", Rect::new(10.0, 20.0, 100.0, 24.0))
        .unwrap_or_else(|| panic!("the class `{classe}` vanished from the catalog"));
    d.add_child("form_main", no).unwrap();
    (d, vec!["alvo".to_string()])
}

fn linhas(doc: Option<&Document>, sel: &[String], aba: Tab) -> Vec<Row> {
    // ALL categories open: the tests here measure which rows the
    // catalog produces, and a closed category would hide rows for a reason
    // other than what they are asking. The fold is tested by
    // `a_categoria_dobra_e_o_cabecalho_fica`, with its own set.
    inspector::rows(&registro(), doc, sel, ALVO, aba, &fechado(), &fechado())
}

fn props_de(doc: &Document, sel: &[String]) -> Vec<inspector::PropRow> {
    linhas(Some(doc), sel, Tab::Properties)
        .into_iter()
        .filter_map(|l| match l {
            Row::Prop(p) => Some(p),
            _ => None,
        })
        .collect()
}

fn editor_de(doc: &Document, sel: &[String], key: &str) -> Editor {
    props_de(doc, sel)
        .into_iter()
        .find(|p| p.key == key)
        .unwrap_or_else(|| panic!("the property `{key}` did not appear in the inspector"))
        .editor
}

/// The notices of a list of rows, for the assertions about the empty panel.
fn avisos(l: &[Row]) -> Vec<&'static str> {
    l.iter()
        .filter_map(|r| match r {
            Row::Notice(t) => Some(*t),
            _ => None,
        })
        .collect()
}

/// The cell of a slot, or `None` when the editor does not have that slot.
fn celula(e: &Editor, slot: Slot) -> Option<Cell> {
    e.cells().into_iter().find(|c| c.slot() == slot)
}

/// The text of a `Field`, a `Caption` or a `Press` cell.
fn texto(e: &Editor, slot: Slot) -> String {
    match celula(e, slot) {
        Some(Cell::Field { text, .. })
        | Some(Cell::Caption { text, .. })
        | Some(Cell::Press { text, .. }) => text,
        outro => panic!("slot {slot:?} is not text: {outro:?}"),
    }
}

/// The value a gesture on a slot produces, or a panic naming what came back.
fn valor(e: &Editor, slot: Slot, g: Gesture<'_>) -> Value {
    match e.gesture(slot, g) {
        Outcome::Set(v) => v,
        outro => panic!("the gesture did not return a value: {outro:?}"),
    }
}

// ---------------------------------------------------------------- the empty state

#[test]
fn sem_documento_o_painel_diz_por_que_esta_vazio() {
    assert_eq!(
        avisos(&linhas(None, &[], Tab::Properties)),
        vec![inspector::NO_FORM]
    );
}

#[test]
fn sem_selecao_o_painel_convida_a_clicar() {
    let (d, _) = doc_com("Button");
    assert_eq!(
        avisos(&linhas(Some(&d), &[], Tab::Properties)),
        vec![inspector::NOTHING_SELECTED, inspector::HOW_TO_SELECT],
        "the empty panel must state what is missing, and what to do — \
         a blank panel does not state if nothing is selected or if it broke"
    );
}

#[test]
fn classe_fora_do_catalogo_nao_vira_painel_em_branco() {
    let mut d = Document::new("form_main", "Form");
    d.add_child("form_main", Node::new("x", "ClasseQueNaoExiste"))
        .unwrap();
    assert_eq!(
        avisos(&linhas(Some(&d), &["x".to_string()], Tab::Properties)),
        vec![inspector::NO_DESCRIPTOR]
    );
}

// -------------------------------------------------------- composition

/// **The law of this iteration.** The Inspector reads the catalog, and nothing beyond it.
///
/// It scans the ENTIRE catalog, instantiates one component of each class, and
/// checks that **every** property declared by the descriptor for this target
/// appears exactly once, with the catalog's `display`.
#[test]
fn every_catalog_property_becomes_row_without_hardcoded_list() {
    let r = registro();
    let mut classes = 0;
    for desc in r.all() {
        if !desc.supports(ALVO) {
            continue;
        }
        let Some(no) = r.instantiate(desc.class, "alvo", Rect::new(0.0, 0.0, 80.0, 24.0)) else {
            continue;
        };
        let mut d = Document::new("form_main", "Form");
        d.add_child("form_main", no).unwrap();
        classes += 1;

        let vistas = props_de(&d, &["alvo".to_string()]);
        let mut esperadas: Vec<&str> = Vec::new();
        for c in [
            Category::Identity,
            Category::Appearance,
            Category::Data,
            Category::Layout,
            Category::Behavior,
            Category::Advanced,
        ] {
            esperadas.extend(
                desc.props_for(ALVO)
                    .filter(|p| p.category == c)
                    // a non-visual component has no geometry to show
                    .filter(|p| desc.visual || p.key != "rect")
                    .map(|p| p.key),
            );
            // the child collection closes Identity — `Columns (N) …`
            if c == Category::Identity
                && crawrad_actions::action_list::is_flat_collection(&r, desc.class)
            {
                esperadas.push(inspector::CHILD_COLLECTION_KEY);
            }
            if c == Category::Appearance && desc.props.iter().any(|p| p.bindable) {
                esperadas.push("binding.dataset");
                esperadas.push("binding.field");
                esperadas.push("binding.mode");
            }
        }
        let obtidas: Vec<&str> = vistas.iter().map(|p| p.key).collect();
        assert_eq!(
            obtidas, esperadas,
            "the class `{}` showed different properties than what the catalog \
             declares — or in the wrong order",
            desc.class
        );
        for p in &vistas {
            if p.key == inspector::CHILD_COLLECTION_KEY {
                assert_eq!(Some(p.label.clone()), desc.collection_label());
                continue;
            }
            if p.key.starts_with("binding.") {
                let expected_label = match p.key {
                    "binding.dataset" => "Dataset",
                    "binding.field" => "Field",
                    "binding.mode" => "Mode",
                    _ => "",
                };
                assert_eq!(p.label, expected_label);
                continue;
            }
            let cat = desc
                .props_for(ALVO)
                .find(|c| c.key == p.key)
                .expect("the key came from the catalog");
            assert_eq!(
                p.label, cat.display,
                "the class `{}` showed `{}` instead of the catalog's `display`",
                desc.class, p.label
            );
        }
    }
    assert!(
        classes > 10,
        "the scan found only {classes} classes — the catalog changed shape"
    );
}

/// Categories appear in the order of the egui shell, and empty ones do not appear.
#[test]
fn a_ordem_das_categorias_e_a_da_casca_egui() {
    let (d, sel) = doc_com("Button");
    let titulos: Vec<&str> = linhas(Some(&d), &sel, Tab::Properties)
        .into_iter()
        .filter_map(|l| match l {
            Row::Category { label, .. } => Some(label),
            _ => None,
        })
        .collect();
    let esperada: Vec<&str> = [
        Category::Identity,
        Category::Appearance,
        Category::Layout,
        Category::Behavior,
        Category::Advanced,
    ]
    .into_iter()
    .map(Category::label)
    .filter(|t| titulos.contains(t))
    .collect();
    assert_eq!(titulos, esperada, "the order of categories changed");
    assert!(
        !titulos.is_empty(),
        "no category appeared — the panel was left with no header at all"
    );
}

#[test]
fn category_without_properties_does_not_become_dangling_header() {
    let r = registro();
    for desc in r.all() {
        if !desc.supports(ALVO) {
            continue;
        }
        let Some(no) = r.instantiate(desc.class, "alvo", Rect::new(0.0, 0.0, 80.0, 24.0)) else {
            continue;
        };
        let mut d = Document::new("form_main", "Form");
        d.add_child("form_main", no).unwrap();

        let l = linhas(Some(&d), &["alvo".to_string()], Tab::Properties);
        for (i, r) in l.iter().enumerate() {
            if matches!(r, Row::Category { .. }) {
                assert!(
                    matches!(l.get(i + 1), Some(Row::Prop(_))),
                    "the class `{}` opened a category without a property \
                     underneath it",
                    desc.class
                );
            }
        }
    }
}

// ------------------------------------- THE DERIVED GUARD

/// The variants of `PropKind`, read out of the registry's source text.
///
/// **This is what makes the guard below derived and not a hand-written list.**
/// A hand-written list is the second maintenance point this test exists to
/// abolish: the twelfth kind would slip past it exactly the way it slips past
/// the compiler — every `match` on `PropKind` in this shell ends in a fall
/// through arm, by design, because the fall through is the honest `bad type`
/// row.
fn propkinds_declarados() -> BTreeSet<String> {
    let corpo = REGISTRY_PROPS
        .split_once("pub enum PropKind {")
        .expect("`crawrad-registry` no longer declares `pub enum PropKind`")
        .1
        .split_once("\n}")
        .expect("the `PropKind` declaration does not close")
        .0;
    let achados: BTreeSet<String> = corpo
        .lines()
        .map(str::trim)
        .filter(|l| !l.starts_with("//") && !l.starts_with('#'))
        .filter_map(|l| {
            let nome: String = l.chars().take_while(char::is_ascii_alphanumeric).collect();
            (!nome.is_empty() && nome.starts_with(char::is_uppercase)).then_some(nome)
        })
        .collect();
    assert!(
        achados.len() > 5,
        "the `PropKind` reader found only {} variant(s); it is broken, and \
         a broken reader leaves the guard green forever: {achados:?}",
        achados.len()
    );
    achados
}

/// One property of every kind, with a value of the shape the catalogue means.
///
/// Hand-written, and that is the half of the pair the guard measures: the
/// derived half is [`propkinds_declarados`], and the test below charges this
/// list against it. A kind added to the registry makes this list incomplete
/// and the guard red.
///
/// Every descriptor here comes out of `crawrad_registry::Props` — none is built
/// by hand — so a kind that changes shape in the catalogue changes shape here
/// too.
fn amostras() -> Vec<(PropDesc, Value)> {
    vec![
        (Props::name(), Value::Str("Button1".into())),
        (Props::lines("a\nb"), Value::Str("a\nb".into())),
        (Props::enabled(true), Value::Bool(true)),
        (Props::tab_order(7), Value::Int(7)),
        (Props::font_size(12.0), Value::Float(12.0)),
        (
            Props::color(Color::rgb(0x2B, 0x57, 0x9A)),
            Value::Color(Color::rgb(0x2B, 0x57, 0x9A)),
        ),
        (
            Props::margins(Edges::uniform(3.0)),
            Value::Edges(Edges::uniform(3.0)),
        ),
        // THE SAME `PropKind::Edges`, in another form: `Anchors` stores a
        // `Value::Map`. That is why coverage is by KIND and the choice of
        // editor is by (kind, value).
        (Props::anchors(), Props::anchors().default.clone()),
        (Props::align(), Value::Enum("None".into())),
        (Props::flat_items(), Value::List(Vec::new())),
        (Props::images(), Value::Ref(String::new())),
        (
            Props::rect(Rect::new(1.0, 2.0, 30.0, 40.0)),
            Value::Rect(Rect::new(1.0, 2.0, 30.0, 40.0)),
        ),
        (
            Props::datetime(crawrad_ir::DateTime::epoch()),
            Value::DateTime(crawrad_ir::DateTime::epoch()),
        ),
        (
            Props::file_path(""),
            Value::FilePath(crawrad_ir::FilePath::empty()),
        ),
        (
            Props::cursor(crawrad_ir::Cursor::Default),
            Value::Cursor(crawrad_ir::Cursor::Default),
        ),
        (
            Props::modal_result(crawrad_ir::ModalResult::None),
            Value::ModalResult(crawrad_ir::ModalResult::None),
        ),
        (
            Props::shortcut(crawrad_ir::Shortcut::NONE),
            Value::Shortcut(crawrad_ir::Shortcut::NONE),
        ),
        (
            Props::font(crawrad_ir::Font::default()),
            Value::Font(crawrad_ir::Font::default()),
        ),
        (
            Props::picture(crawrad_ir::Picture::empty()),
            Value::Picture(crawrad_ir::Picture::empty()),
        ), // A column of the dataset a sibling reference names.
        (Props::lookup_key(""), Value::Str(String::new())),
    ]
}

/// The variant name of a `PropKind`, without a `match` that gets outdated.
///
/// Extracted from derived `Debug`: `Rect`, `Int { min: 0, max: 1000 }`,
/// `Enum(["None"])` — the alphanumeric prefix is the name. A `match` here
/// **would prevent compilation** when a new variant is added, whereas this file
/// wants the opposite: for a new variant to turn the test RED, with a
/// message stating what is missing.
fn nome_do_kind(k: &crawrad_registry::PropKind) -> String {
    format!("{k:?}")
        .chars()
        .take_while(char::is_ascii_alphanumeric)
        .collect()
}

/// Builds the editor for a property outside the class catalog.
///
/// It exists because the samples above are standalone `PropDesc` items: not every class
/// in the catalog has a `Color` or a `Float`, and the guard cannot depend
/// on that.
fn monta_editor(desc: &PropDesc, valor: &Value, doc: &Document, no: &str, aberto: bool) -> Editor {
    let dono = no.to_string();
    let catalogo = crawrad_registry::Registry::builtin();
    let subject = inspector::editors::Subject {
        desc,
        registry: &catalogo,
        value: valor,
        node: &dono,
        doc,
        open: aberto,
    };
    inspector::editors::editor_for(&subject)
        .unwrap_or_else(|| panic!("no editor accepted `{}`", desc.key))
}

/// **The definition of done, and the guard that maintains it.**
///
/// For every `PropKind` declared by `crawrad-registry`, the shell has an editor — and
/// the list of kinds comes from the registry source, not from a hand-written list here.
/// A new `PropKind` is tested upon arrival.
///
/// A guard with a hand-written list is what this house has already seen fail four times
/// in a single day.
#[test]
fn todo_propkind_do_catalogo_tem_editor_na_casca() {
    let declarados = propkinds_declarados();
    let cobertos: BTreeSet<String> = amostras()
        .iter()
        .map(|(p, _)| nome_do_kind(&p.kind))
        .collect();
    assert_eq!(
        declarados, cobertos,
        "the set of `PropKind` covered by the Inspector no longer matches what \
         `crawrad-registry` declares. A declared and uncovered kind is a \
         row that the panel shows and does not edit; a covered and undeclared one is \
         a leftover sample."
    );

    // And each sample produces a REAL editor — not a mute row.
    let (d, sel) = doc_com("Button");
    for (desc, valor) in amostras() {
        let editor = monta_editor(&desc, &valor, &d, &sel[0], false);
        assert_ne!(
            editor.template(),
            "RowMute",
            "`{}` ({}) fell on the mute row — it shows the value and does not edit it, \
             which is exactly the `no editor` that issue #137 came to remove from the screen",
            desc.key,
            nome_do_kind(&desc.kind)
        );
        assert!(
            !editor.cells().is_empty(),
            "`{}` returned an editor without any cells — the row would come out with the \
             name and nothing else",
            desc.key
        );
    }
}

/// **No row says `no editor`.**
///
/// The phrase was removed from the code, and this test prevents its return under
/// another name: it scans the entire catalog and checks that every property that is NOT
/// `readonly` in the catalog gets a real editor.
#[test]
fn no_editable_property_falls_on_mute_row() {
    let r = registro();
    let mut vistas = 0;
    for desc in r.all() {
        if !desc.supports(ALVO) {
            continue;
        }
        let Some(no) = r.instantiate(desc.class, "alvo", Rect::new(0.0, 0.0, 80.0, 24.0)) else {
            continue;
        };
        let mut d = Document::new("form_main", "Form");
        d.add_child("form_main", no).unwrap();

        for p in props_de(&d, &["alvo".to_string()]) {
            if p.key.starts_with("binding.") || p.key == inspector::CHILD_COLLECTION_KEY {
                continue;
            }
            let catalogo = desc
                .props_for(ALVO)
                .find(|c| c.key == p.key)
                .expect("the key came from the catalog");
            vistas += 1;
            if catalogo.readonly {
                continue;
            }
            assert_ne!(
                p.editor.template(),
                "RowMute",
                "`{}.{}` ({}) has no editor. Issue #137 removed `no editor` \
                 from the screen, and the mute row is only valid for `read-only`, `differs`, and \
                 `bad type`.",
                desc.class,
                p.key,
                nome_do_kind(&catalogo.kind)
            );
        }
    }
    assert!(
        vistas > 100,
        "the scan saw only {vistas} rows — the catalog changed shape"
    );
}

/// The phrase `no editor` is gone from the CODE, not just the screen.
#[test]
fn a_frase_no_editor_nao_existe_mais_na_casca() {
    use super::casca::CASCA;
    const PAINEL: &str = include_str!("../../../../crawrad-actions/src/inspector_model.rs");
    const MUDA: &str =
        include_str!("../../../../crawrad-actions/src/inspector_model/editors/mute.rs");
    for (arquivo, texto) in [
        ("src/ide_app.rs", CASCA),
        ("src/ide_app/inspector.rs", PAINEL),
        ("src/ide_app/inspector/editors/mute.rs", MUDA),
    ] {
        assert!(
            !texto.contains("\"no editor\""),
            "`no editor` returned to {arquivo}. It was the phrase for six of the eleven \
             `PropKind`s, and the core of issue #137 is specifically that it must not exist."
        );
    }
}

// --------------------------------------------- THE EDITORS, ONE BY ONE

/// The quarters of an open `Rect`: `Left`, `Top`, `Width`, `Height`.
fn rect_quarters(d: &Document, sel: &[String]) -> Vec<inspector::PropRow> {
    inspector::rows(
        &registro(),
        Some(d),
        sel,
        ALVO,
        Tab::Properties,
        &BTreeSet::from(["rect".to_string()]),
        &fechado(),
    )
    .into_iter()
    .filter_map(|l| match l {
        Row::Prop(p) if p.key == "rect" && p.depth > 0 => Some(p),
        _ => None,
    })
    .collect()
}

/// `Rect` folds like `Margins`: shut it reads the rectangle, open it is
/// four rows, and each one writes the WHOLE rectangle.
#[test]
fn rect_folds_into_four_quarters_each_writing_the_whole_rectangle() {
    let (d, sel) = doc_com("Button");
    let shut = editor_de(&d, &sel, "rect");
    assert_eq!(shut.template(), "RowFold");
    assert!(shut.children().is_empty(), "`Rect` was born open");
    assert_eq!(texto(&shut, Slot::Value), "10, 20, 100 × 24");
    assert!(
        matches!(
            shut.gesture(Slot::Fold, Gesture::Pressed),
            Outcome::Run("inspector.toggle_group", _)
        ),
        "the `Rect` chevron no longer goes through the catalogue"
    );

    let quarters = rect_quarters(&d, &sel);
    let names: Vec<&str> = quarters.iter().map(|p| p.label.as_str()).collect();
    assert_eq!(names, vec!["Left", "Top", "Width", "Height"]);
    for q in &quarters {
        assert_eq!(q.editor.template(), "SubText");
    }
    assert_eq!(texto(&quarters[0].editor, Slot::Value), "10");
    assert_eq!(texto(&quarters[2].editor, Slot::Value), "100");

    let Value::Rect(r) = valor(&quarters[0].editor, Slot::Value, Gesture::Typed("42")) else {
        panic!("`Left` did not hand back a rectangle");
    };
    assert_eq!(
        (r.x, r.y, r.w, r.h),
        (42.0, 20.0, 100.0, 24.0),
        "`Left` moved another quarter: each writes the WHOLE rectangle"
    );
    let Value::Rect(r) = valor(&quarters[3].editor, Slot::Value, Gesture::Typed("30")) else {
        panic!("`Height` did not hand back a rectangle");
    };
    assert_eq!((r.x, r.y, r.w, r.h), (10.0, 20.0, 100.0, 30.0));
}

/// The floor of the size is a limit of GESTURE, and `Left` has no floor.
#[test]
fn size_is_clamped_and_position_can_be_negative() {
    let (d, sel) = doc_com("Button");
    let quarters = rect_quarters(&d, &sel);
    let (left, width) = (&quarters[0].editor, &quarters[2].editor);
    let Value::Rect(r) = valor(width, Slot::Value, Gesture::Typed("1")) else {
        panic!("`Width` did not hand back a rectangle");
    };
    assert_eq!(r.w, 8.0, "a typed size below the floor goes in clamped");
    let Value::Rect(r) = valor(left, Slot::Value, Gesture::Typed("-30")) else {
        panic!("`Left` did not hand back a rectangle");
    };
    assert_eq!(
        r.x, -30.0,
        "a component at a negative coordinate is one dragged in from the edge"
    );
    assert!(
        matches!(
            width.gesture(Slot::Value, Gesture::Typed("eight")),
            Outcome::Refuse(_)
        ),
        "`Width` took a text that is not a number"
    );
}

/// `Margins` is the THIRD row shape: summary, and in-place expansion.
#[test]
fn o_margins_dobra_em_quatro_e_cada_lado_grava_os_quatro() {
    let (d, sel) = doc_com("Button");
    let fechada = editor_de(&d, &sel, "margins");
    assert_eq!(fechada.template(), "RowFold");
    assert!(
        fechada.children().is_empty(),
        "`Margins` was born open — the default is closed, with the summary"
    );
    assert!(
        matches!(
            fechada.gesture(Slot::Fold, Gesture::Pressed),
            Outcome::Run("inspector.toggle_group", _)
        ),
        "the `Margins` chevron stopped going through the catalog — folding is a \
         capability, and a capability that the shell resolves internally never reaches \
         the protocol"
    );

    let aberto = inspector::rows(
        &registro(),
        Some(&d),
        &sel,
        ALVO,
        Tab::Properties,
        &BTreeSet::from(["margins".to_string()]),
        &fechado(),
    );
    let filhos: Vec<inspector::PropRow> = aberto
        .into_iter()
        .filter_map(|l| match l {
            Row::Prop(p) if p.key == "margins" && p.depth > 0 => Some(p),
            _ => None,
        })
        .collect();
    let nomes: Vec<&str> = filhos.iter().map(|p| p.label.as_str()).collect();
    assert_eq!(nomes, vec!["Left", "Top", "Right", "Bottom"]);
    for f in &filhos {
        assert_eq!(f.editor.template(), "SubText");
    }

    let esquerda = &filhos[0].editor;
    let Value::Edges(e) = valor(esquerda, Slot::Value, Gesture::Typed("9")) else {
        panic!("the `Left` side did not return an `Edges`");
    };
    assert_eq!(e.left, 9.0, "the `Left` side failed to save what was typed");
    assert!(
        matches!(
            esquerda.gesture(Slot::Value, Gesture::Typed("-1")),
            Outcome::Set(Value::Edges(v)) if v.left == 0.0
        ),
        "a negative margin is the control intruding on its neighbor — it goes \
         in clamped to zero"
    );
}

/// `Anchors` is the SAME `PropKind::Edges` with a different value shape, and
/// therefore it is four checkboxes, not four numbers.
#[test]
fn o_anchors_dobra_em_quatro_caixas_e_nao_em_quatro_numeros() {
    let (d, sel) = doc_com("Button");
    let fechada = editor_de(&d, &sel, "anchors");
    assert_eq!(fechada.template(), "RowFold");
    assert_eq!(
        texto(&fechada, Slot::Value),
        "[akLeft,akTop]",
        "the summary of `Anchors` must come out with the classic RAD IDE names, as in the \
         reference screenshot — `left,top` are the document KEYS"
    );

    let aberto = inspector::rows(
        &registro(),
        Some(&d),
        &sel,
        ALVO,
        Tab::Properties,
        &BTreeSet::from(["anchors".to_string()]),
        &fechado(),
    );
    let filhos: Vec<inspector::PropRow> = aberto
        .into_iter()
        .filter_map(|l| match l {
            Row::Prop(p) if p.key == "anchors" && p.depth > 0 => Some(p),
            _ => None,
        })
        .collect();
    let nomes: Vec<&str> = filhos.iter().map(|p| p.label.as_str()).collect();
    assert_eq!(nomes, vec!["akLeft", "akTop", "akRight", "akBottom"]);
    for f in &filhos {
        assert_eq!(
            f.editor.template(),
            "SubCheck",
            "`Anchors` became four numbers — it is four anchors switched on or \
             off"
        );
    }
    let Value::Map(m) = valor(&filhos[3].editor, Slot::Value, Gesture::Toggled(true)) else {
        panic!("the anchor did not return a map");
    };
    assert_eq!(m.get("bottom"), Some(&Value::Bool(true)));
    assert_eq!(
        m.get("left"),
        Some(&Value::Bool(true)),
        "turning on `akBottom` erased the others — each box writes the ENTIRE map"
    );
}

/// Color shows the swatch AND `#RRGGBB`, and rejects non-color values.
#[test]
fn a_cor_tem_amostra_e_hexadecimal() {
    let (d, sel) = doc_com("Button");
    let azul = Color::rgb(0x2B, 0x57, 0x9A);
    let e = monta_editor(&Props::color(azul), &Value::Color(azul), &d, &sel[0], false);
    assert_eq!(e.template(), "RowColor");
    assert_eq!(
        celula(&e, Slot::Swatch),
        Some(Cell::Swatch {
            slot: Slot::Swatch,
            rgba: [0x2B, 0x57, 0x9A, 255],
        }),
        "the swatch vanished — `#2B579A` and `#2B5A97` look like the same string at a glance, \
         and it is what makes the row answer *what color is this?*"
    );
    assert_eq!(texto(&e, Slot::Value), "#2B579A");
    assert_eq!(
        texto(&e, Slot::Button),
        crawrad_designer::inspector_icon::OPEN_EDITOR
    );
    assert!(matches!(
        e.gesture(Slot::Button, Gesture::Pressed),
        Outcome::Run(inspector::ACTION_OPEN_EDITOR, _)
    ));
    assert_eq!(
        valor(&e, Slot::Value, Gesture::Typed("#FF0000")),
        Value::Color(Color::rgb(255, 0, 0))
    );
    assert!(
        matches!(
            e.gesture(Slot::Value, Gesture::Typed("blue")),
            Outcome::Refuse(porque) if porque.contains("blue")
        ),
        "the refusal must state what was typed"
    );
}

/// Reference lists DOCUMENT components, filtered by class.
#[test]
fn a_referencia_lista_o_que_o_documento_tem_e_nada_mais() {
    let r = registro();
    let mut d = Document::new("form_main", "Form");
    for (id, classe) in [
        ("alvo", "Button"),
        ("Lista1", "ImageList"),
        ("Lista2", "ImageList"),
        ("Outro", "CheckBox"),
    ] {
        let Some(no) = r.instantiate(classe, id, Rect::new(0.0, 0.0, 16.0, 16.0)) else {
            continue;
        };
        d.add_child("form_main", no).unwrap();
    }
    let e = monta_editor(
        &Props::images(),
        &Value::Ref(String::new()),
        &d,
        "alvo",
        false,
    );
    assert_eq!(e.template(), "RowChoice");
    let Some(Cell::Choice {
        options, current, ..
    }) = celula(&e, Slot::Value)
    else {
        panic!("the reference is no longer a combo");
    };
    assert_eq!(
        options,
        vec![
            "(none)".to_string(),
            "Lista1".to_string(),
            "Lista2".to_string()
        ],
        "the list must only bring components of the class that the property \
         accepts, with `(none)` in front: pointing a button to a `CheckBox` \
         is not an option that should exist"
    );
    assert_eq!(current, Some(0), "an empty reference is `(none)`");
    assert_eq!(
        valor(&e, Slot::Value, Gesture::Chose(1)),
        Value::Ref("Lista1".to_string())
    );
    assert_eq!(
        valor(&e, Slot::Value, Gesture::Chose(0)),
        Value::Ref(String::new()),
        "`(none)` must disconnect the reference — without it `Images` would \
         be a one-way street"
    );
}

/// Multiline text shows HOW MANY, and opens via the catalog.
#[test]
fn o_lines_mostra_a_contagem_e_abre_pela_acao() {
    let (d, sel) = doc_com("Memo");
    let e = editor_de(&d, &sel, "text");
    assert_eq!(e.template(), "RowOpen");
    assert_eq!(
        texto(&e, Slot::Value),
        "3 lines",
        "`Lines` on a Memo must state HOW MANY lines it has — text \
         stuck on a single line was the defect that the egui shell already paid for"
    );
    assert!(
        matches!(
            e.gesture(Slot::Button, Gesture::Pressed),
            Outcome::Run("inspector.open_editor", _)
        ),
        "the `…` of `Lines` stopped going through the catalog"
    );
    let abre = e
        .window_text()
        .expect("`Lines` stopped stating what text the window opens with");
    assert_eq!(
        abre.lines().count(),
        3,
        "the window opens with the document text, and not with a summary of it"
    );
    assert!(matches!(
        e.window_value("a single line"),
        Outcome::Set(Value::Str(_))
    ));
}

/// The collection roundtrips to and from the window without losing an item.
#[test]
fn collection_roundtrips_window_as_text() {
    use inspector::editors::collection::{from_text, to_text};
    let arvore = vec![
        Item::with_children(
            "Node1",
            vec![
                Item::new("SubNode1"),
                Item::with_children("SubNode2", vec![Item::new("Grandchild")]),
            ],
        ),
        Item::new("Node2"),
    ];
    let texto = to_text(&arvore);
    assert_eq!(
        texto, "Node1\n  SubNode1\n  SubNode2\n    Grandchild\nNode2",
        "pad grammar is two spaces per level — the same as every \
         outliner, every `yaml`, and every Markdown list"
    );
    assert_eq!(
        from_text(&texto, true),
        arvore,
        "the collection did not survive roundtripping through the window"
    );

    // Flat: indentation is whitespace and nothing else.
    assert_eq!(
        from_text("  One\n  Two", false),
        vec![Item::new("  One"), Item::new("  Two")],
        "in a flat collection there is no nesting to read"
    );
    // A blank line is not an item.
    assert_eq!(from_text("One\n\n   \nTwo", true).len(), 2);
    // And an overly deep indent is clamped to the deepest existing level.
    assert_eq!(
        from_text("One\n        Deep", true),
        vec![Item::with_children("One", vec![Item::new("Deep")])],
        "two levels of indent under a leaf is a typo — \
         inventing an empty parent would put things into the document that no one wrote"
    );
}

/// And the collection row shows the count and opens via the catalog.
#[test]
fn collection_shows_count_and_opens_via_action() {
    let (d, sel) = doc_com("TreeView");
    let e = editor_de(&d, &sel, "items");
    // `Items` on `TreeView` is `so_leitura()` in the catalog — the hierarchical
    // sample — AND HAS A WINDOW BEHIND IT. It gets `…`, not a mute row.
    //
    // The opposite was written here previously, and that was the defect: the owner selected a
    // `TreeView` and saw `4 items` with a gray `read-only` next to it and **no
    // path to the editor**. The collection editor exists, is wired, and nothing
    // on screen led to it.
    //
    // `readonly` in the catalog means *there is no field to type into ON THIS
    // ROW* — and a window is not a field on the row. It is the rule from the egui shell,
    // word for word: `if p.readonly || janela.is_some()`
    // (`crates/crawrad-ide/src/inspector.rs`), with `…` inside that branch.
    assert_eq!(
        e.template(),
        "RowOpen",
        "`Items` on `TreeView` returned to being a mute row: the collection editor \
         exists and nothing on screen leads to it"
    );

    let editavel = monta_editor(
        &Props::tree_items(),
        &Item::list_to(&[Item::with_children("A", vec![Item::new("B")])]),
        &d,
        &sel[0],
        false,
    );
    assert_eq!(editavel.template(), "RowOpen");
    assert_eq!(
        texto(&editavel, Slot::Value),
        "2 items",
        "the summary counts the ENTIRE tree, descendants included"
    );
    assert!(
        matches!(
            editavel.gesture(Slot::Button, Gesture::Pressed),
            Outcome::Run("inspector.open_editor", _)
        ),
        "the `…` of a collection stopped going through the catalog"
    );
    assert_eq!(
        editavel.window_text().as_deref(),
        Some("A\n  B"),
        "the collection window opens with one item per line"
    );
}

/// `Align` opens the DRAWN SELECTOR, and the button holds the value.
#[test]
fn align_is_button_opening_custom_selector() {
    let (d, sel) = doc_com("Button");
    let e = editor_de(&d, &sel, "align");
    assert_eq!(
        e.template(),
        "RowPick",
        "`Align` returned to being a combo with the names — the owner sent the classic \
         RAD IDE screenshot and said *with egui it was already like this*: it is a form drawing \
         where you click on the region"
    );
    assert_eq!(texto(&e, Slot::Button), "None");
    assert!(
        matches!(
            e.gesture(Slot::Button, Gesture::Pressed),
            Outcome::Run("inspector.open_editor", _)
        ),
        "the `Align` button stopped going through the catalog"
    );
    assert!(
        e.window_text().is_none(),
        "what `Align` opens is not a text pad"
    );
}

/// **No region of the selector invents an alignment.**
///
/// The five names written by the shell into the regions, plus the button's `None`, must
/// be in `crawrad_registry::ALIGN_OPTIONS`. A region that named a
/// value outside the catalog would write an alignment to the document that nothing
/// draws — and nothing would catch it.
#[test]
fn toda_regiao_do_seletor_de_align_e_um_valor_do_catalogo() {
    use super::casca::CASCA;
    let bloco = CASCA
        .split_once("const ALIGN_ZONES")
        .expect("the selector regions table vanished from the shell")
        .1;
    let bloco = bloco
        .split_once("];")
        .expect("the regions table does not close")
        .0;
    let citados: Vec<&str> = bloco
        .match_indices('"')
        .map(|(i, _)| i)
        .collect::<Vec<_>>()
        .chunks(2)
        .filter_map(|p| match p {
            [a, b] => Some(&bloco[a + 1..*b]),
            _ => None,
        })
        .collect();
    assert_eq!(
        citados.len(),
        5,
        "the selector no longer has five regions: {citados:?}"
    );
    for nome in &citados {
        assert!(
            crawrad_registry::ALIGN_OPTIONS.contains(nome),
            "the region `{nome}` is not an `ALIGN_OPTIONS` value — it would write \
             an alignment that nothing draws"
        );
    }
    // And the sixth value is the button's `None`, which the shell reads from the catalog instead
    // of hardcoding.
    let sobra: Vec<&&str> = crawrad_registry::ALIGN_OPTIONS
        .iter()
        .filter(|o| !citados.contains(o))
        .collect();
    assert_eq!(
        sobra,
        vec![&crawrad_registry::ALIGN_OPTIONS[0]],
        "the only value without a region is the catalog's first, which is the button \
         under the canvas"
    );
}

/// And an `Enum` that is NOT `Align` remains the catalog combo.
#[test]
fn os_demais_conjuntos_fechados_continuam_sendo_combo() {
    let (d, sel) = doc_com("ScrollBar");
    let e = editor_de(&d, &sel, "kind");
    assert_eq!(e.template(), "RowChoice");
    let Some(Cell::Choice { options, .. }) = celula(&e, Slot::Value) else {
        panic!("the `Kind` of a ScrollBar is no longer a combo");
    };
    assert!(
        !options.is_empty(),
        "the options come from the descriptor, and the descriptor was left with none"
    );
}

/// The combo does not pretend an option when the document specifies another.
#[test]
fn a_combo_nao_finge_a_opcao_que_o_documento_nao_tem() {
    let (mut d, sel) = doc_com("ScrollBar");
    d.node_mut("alvo")
        .unwrap()
        .props
        .insert("kind".into(), Value::Enum("sbSeiLa".into()));
    let e = editor_de(&d, &sel, "kind");
    assert_eq!(
        celula(&e, Slot::Value).and_then(|c| match c {
            Cell::Choice { current, .. } => Some(current),
            _ => None,
        }),
        Some(None),
        "with an out-of-set value the combo must not fall back to the first item: \
         it would state the first option for a document that states something else"
    );
}

// -------------------------------------------- no dead fields remain

/// **The house rule applied row by row**: whatever lacks wiring SAYS it
/// lacks it.
#[test]
fn no_inspector_row_is_dead_field() {
    let r = registro();
    let mut mudas = 0;
    for desc in r.all() {
        if !desc.supports(ALVO) {
            continue;
        }
        let Some(no) = r.instantiate(desc.class, "alvo", Rect::new(0.0, 0.0, 80.0, 24.0)) else {
            continue;
        };
        let mut d = Document::new("form_main", "Form");
        d.add_child("form_main", no).unwrap();

        for p in props_de(&d, &["alvo".to_string()]) {
            if p.editor.template() != "RowMute" {
                continue;
            }
            mudas += 1;
            let nota = texto(&p.editor, Slot::Note);
            let mostrado = texto(&p.editor, Slot::Value);
            assert!(
                !nota.is_empty(),
                "`{}.{}` is not editable and does not say why — it is a dead field",
                desc.class,
                p.key
            );
            assert!(
                !mostrado.is_empty(),
                "`{}.{}` is not editable nor does it show the value: the row serves \
                 no purpose",
                desc.class,
                p.key
            );
        }
    }
    // THE CONTRAST no longer comes from `read-only`, and the switch is intentional.
    //
    // The three `so_leitura()` in the catalog are collections — `tree_items`,
    // `flat_items_sample`, and `images_data` (`crawrad-registry/src/props.rs`) —, and
    // all three have a WINDOW behind them. Ever since the window took precedence over
    // `readonly` (see note in the `editors` module), no property in the
    // builtin catalog produces a mute row: they receive `…`.
    //
    // The mute row still exists, and its case is a document that stores
    // a value of ANOTHER TYPE — a handwritten `.crawform`, or a catalog
    // that changed the type of an already-saved key. That is what provides contrast,
    // and it is constructed here rather than searched in the catalog.
    let _ = mudas;
    let (mut torto, sel) = doc_com("Button");
    torto
        .node_mut(&sel[0])
        .expect("the node is there")
        .props
        .insert("text".into(), crawrad_ir::Value::Int(7));
    let muda = editor_de(&torto, &sel, "text");
    assert_eq!(
        muda.template(),
        "RowMute",
        "a value of another type stopped becoming a mute row — the scan above \
         lost contrast, and a filter finding nothing would pass"
    );
    assert!(
        !texto(&muda, Slot::Note).is_empty(),
        "the mute row for the wrong value does not state why it does not edit"
    );
}

/// A document with a value of the wrong type does not become `0` on screen.
#[test]
fn valor_de_tipo_errado_nao_vira_zero_na_tela() {
    let (mut d, sel) = doc_com("ScrollBar");
    d.node_mut("alvo")
        .unwrap()
        .props
        .insert("position".into(), Value::Str("not a number".into()));
    let e = editor_de(&d, &sel, "position");
    assert_eq!(e.template(), "RowMute");
    assert_eq!(texto(&e, Slot::Value), "not a number");
    assert_eq!(
        texto(&e, Slot::Note),
        inspector::editors::mute::BAD_TYPE,
        "a value of the wrong type must appear as is, with the note — a \
         numeric field showing `0` would overwrite what was there"
    );
}

// --------------------------------------------------- multi-selection

#[test]
fn com_varios_selecionados_a_lista_e_a_intersecao() {
    let r = registro();
    let mut d = Document::new("form_main", "Form");
    for (id, classe) in [("a", "Button"), ("b", "CheckBox")] {
        let no = r
            .instantiate(classe, id, Rect::new(0.0, 0.0, 80.0, 24.0))
            .unwrap();
        d.add_child("form_main", no).unwrap();
    }
    let sel = vec!["a".to_string(), "b".to_string()];
    let chaves: Vec<&str> = props_de(&d, &sel).iter().map(|p| p.key).collect();
    assert!(
        chaves.contains(&"enabled"),
        "what both have in common vanished: {chaves:?}"
    );
    assert!(
        !chaves.contains(&"checked"),
        "the `Checked` property, which only CheckBox has, entered the intersection: {chaves:?}"
    );
}

#[test]
fn com_varios_selecionados_o_valor_que_diverge_nao_mente() {
    let r = registro();
    let mut d = Document::new("form_main", "Form");
    for (id, enabled) in [("a", true), ("b", false)] {
        let mut no = r
            .instantiate("Button", id, Rect::new(0.0, 0.0, 80.0, 24.0))
            .unwrap();
        no.props.insert("enabled".into(), Value::Bool(enabled));
        d.add_child("form_main", no).unwrap();
    }
    let sel = vec!["a".to_string(), "b".to_string()];
    let e = editor_de(&d, &sel, "enabled");
    assert_eq!(e.template(), "RowMute");
    assert_eq!(texto(&e, Slot::Value), inspector::editors::mute::VARIOUS);
    assert_eq!(
        texto(&e, Slot::Note),
        inspector::editors::mute::DIFFERS,
        "with differing values the box cannot appear unchecked: checking and \
         unchecking would set both to the same value without anyone asking"
    );
}

#[test]
fn o_name_de_varios_nao_se_edita() {
    let r = registro();
    let mut d = Document::new("form_main", "Form");
    for id in ["a", "b"] {
        let no = r
            .instantiate("Button", id, Rect::new(0.0, 0.0, 80.0, 24.0))
            .unwrap();
        d.add_child("form_main", no).unwrap();
    }
    let sel = vec!["a".to_string(), "b".to_string()];
    assert_eq!(
        editor_de(&d, &sel, "name").template(),
        "RowMute",
        "two components cannot be renamed through the same field: \
         `component.rename` would reject the second due to duplicate name"
    );
}

/// An ABSENT value in the node defaults to the catalog default.
#[test]
fn missing_property_defaults_without_divergence() {
    let r = registro();
    let mut d = Document::new("form_main", "Form");
    let a = r
        .instantiate("Button", "a", Rect::new(0.0, 0.0, 80.0, 24.0))
        .unwrap();
    let mut b = a.clone();
    b.id = "b".to_string();
    b.set_name("b");
    b.props.remove("enabled");
    d.add_child("form_main", a).unwrap();
    d.add_child("form_main", b).unwrap();

    let sel = vec!["a".to_string(), "b".to_string()];
    assert_eq!(
        editor_de(&d, &sel, "enabled").template(),
        "RowCheck",
        "a property that neither has explicitly written takes the DEFAULT — and the \
         default is what drawing uses. Comparing only what is written would make \
         two equal components appear divergent"
    );
}

// ------------------------------------------------ text typed by the user

#[test]
fn o_numero_recusado_e_dito_em_vez_de_engolido() {
    let (d, sel) = doc_com("ScrollBar");
    let e = editor_de(&d, &sel, "position");
    let Outcome::Refuse(porque) = e.gesture(Slot::Value, Gesture::Typed("abc")) else {
        panic!("the numeric field swallowed `abc`");
    };
    assert!(
        porque.contains("abc"),
        "the refusal must state what was typed: {porque}"
    );
    assert_eq!(
        valor(&e, Slot::Value, Gesture::Typed(" 42 ")),
        Value::Int(42)
    );
}

#[test]
fn o_limite_do_catalogo_grampeia_o_que_foi_digitado() {
    let (d, sel) = doc_com("Button");
    let e = editor_de(&d, &sel, "tab_order");
    assert_eq!(
        valor(&e, Slot::Value, Gesture::Typed("99999")),
        Value::Int(1000),
        "the catalog limit is a GESTURE limit: what is typed enters \
         clamped, and nothing rewrites the document on its own"
    );
    assert_eq!(valor(&e, Slot::Value, Gesture::Typed("-5")), Value::Int(0));
}

#[test]
fn o_decimal_recusa_o_que_nao_e_finito() {
    let (d, sel) = doc_com("Button");
    let e = monta_editor(
        &Props::font_size(12.0),
        &Value::Float(12.0),
        &d,
        &sel[0],
        false,
    );
    for bicho in ["nan", "inf", "-inf"] {
        assert!(
            matches!(
                e.gesture(Slot::Value, Gesture::Typed(bicho)),
                Outcome::Refuse(_)
            ),
            "`{bicho}` entered into a font size — a non-finite number \
             is a component that ceases to be drawn"
        );
    }
    assert_eq!(
        valor(&e, Slot::Value, Gesture::Typed("13.5")),
        Value::Float(13.5)
    );
}

#[test]
fn non_text_field_ignores_typed_text() {
    let (d, sel) = doc_com("Button");
    for key in ["enabled", "align"] {
        let e = editor_de(&d, &sel, key);
        assert!(
            matches!(
                e.gesture(Slot::Value, Gesture::Typed("anything")),
                Outcome::Ignore
            ),
            "`{key}` accepted typed text in a slot that is not a field"
        );
    }
}

#[test]
fn o_numero_aparece_sem_casa_decimal_inventada() {
    let (d, sel) = doc_com("Button");
    assert_eq!(texto(&editor_de(&d, &sel, "tab_order"), Slot::Value), "0");
    let e = monta_editor(
        &Props::font_size(3.0),
        &Value::Float(3.0),
        &d,
        &sel[0],
        false,
    );
    assert_eq!(
        texto(&e, Slot::Value),
        "3",
        "a `3.00` on screen would be the IDE inventing precision that the property \
         does not have — and whoever typed over it would have to delete two digits"
    );
}

// ------------------------------------------------------------ the header

#[test]
fn o_cabecalho_mostra_o_nome_e_a_classe() {
    let (d, sel) = doc_com("Button");
    let c = inspector::header(Some(&d), &sel);
    assert!(c.contains("Button"), "{c}");
    assert!(
        !c.contains("alvo · "),
        "the header shows the NAME, and `instantiate` gives identical name and id; \
         if they diverge one day, the name is what stays (ADR 0008): {c}"
    );
    assert_eq!(inspector::header(Some(&d), &[]), "—");
    assert_eq!(inspector::header(None, &[]), "—");
    assert_eq!(
        inspector::header(Some(&d), &["a".to_string(), "b".to_string()]),
        "2 items selected"
    );
}

/// The multiline summary is the SAME as the egui lines editor.
#[test]
fn lines_summary_matches_lines_editor() {
    // The file was renamed on 17/09/2026 when the Lines Editor gained the
    // Makepad shell: the SCREENLESS part moved to `lines_editor_model.rs`,
    // which BOTH shells include. Continuing to read egui drawing here
    // would measure the wrong file — passing green without measuring anything.
    // It lives in `crawrad-designer`.
    const MODELO: &str = include_str!("../../../../crawrad-designer/src/lines_editor_model.rs");
    for forma in ["\"(empty)\"", "\"1 line\"", "{n} lines"] {
        assert!(
            MODELO.contains(forma),
            "the lines editor model stopped writing {forma} — the two \
             shells would summarize the same text with two grammars"
        );
    }
    let (d, sel) = doc_com("Memo");
    assert_eq!(texto(&editor_de(&d, &sel, "text"), Slot::Value), "3 lines");
}

/// **Every control offers `Hint` and `ShowHint`**, from the catalogue's
/// base properties.
#[test]
fn every_control_offers_hint_and_show_hint() {
    for class in ["Button", "Edit", "Label", "Panel", "ListBox"] {
        let (d, sel) = doc_com(class);
        let keys: Vec<String> = props_de(&d, &sel)
            .into_iter()
            .map(|p| p.key.to_string())
            .collect();
        for k in ["hint", "show_hint"] {
            assert!(
                keys.iter().any(|x| x == k),
                "{class}: `{k}` is not in the inspector: {keys:?}"
            );
        }
    }
}
/// **The Memo's keys and limit are in the Object Inspector**, from the
/// catalogue like every other row.
#[test]
fn the_memo_shows_word_wrap_want_returns_want_tabs_and_max_length() {
    let (d, sel) = doc_com("Memo");
    let keys: Vec<String> = props_de(&d, &sel)
        .into_iter()
        .map(|p| p.key.to_string())
        .collect();
    for k in ["word_wrap", "want_returns", "want_tabs", "max_length"] {
        assert!(
            keys.iter().any(|x| x == k),
            "`{k}` is not in the Memo's inspector: {keys:?}"
        );
    }
}

/// **The Grid's design-time cells and its Editing switch are in the
/// Object Inspector**, from the catalogue like every other row.
#[test]
fn the_grid_shows_cells_and_editing() {
    let (d, sel) = doc_com("Grid");
    let props = props_de(&d, &sel);
    for (key, label) in [("cell_text", "Cells"), ("editing", "Editing")] {
        assert!(
            props.iter().any(|p| p.key == key && p.label == label),
            "`{key}` (`{label}`) is not in the Grid's inspector: {props:?}"
        );
    }
}

/// **The ComboBox's Sorted is in the Object Inspector**, as the ListBox's.
#[test]
fn the_combo_box_shows_sorted() {
    let (d, sel) = doc_com("ComboBox");
    let props = props_de(&d, &sel);
    assert!(
        props
            .iter()
            .any(|p| p.key == "sorted" && p.label == "Sorted"),
        "`sorted` (`Sorted`) is not in the ComboBox's inspector: {props:?}"
    );
}
// ------------------------------------------------- composition per file

/// **A new editor costs one file, and nothing more.**
///
/// The proof is indirect and is the best a test can give: no property template
/// name is written in the drawing loop — it asks the editor.
/// If someone brings back the central `match`, the loop starts mentioning the
/// name of a template.
#[test]
fn nenhum_molde_de_linha_esta_escrito_no_laco_de_desenho() {
    use super::casca::CASCA;
    let moldes: BTreeSet<&'static str> = inspector::editors::samples()
        .iter()
        .map(|e| e.template())
        .collect();
    assert!(
        moldes.len() > 6,
        "the samples cover only {} template(s) — decomposition shrank",
        moldes.len()
    );
    let desenho = CASCA
        .split_once("fn escrever_linha")
        .expect("row drawing exists")
        .1;
    let desenho = &desenho[..desenho.len().min(6000)];
    for molde in &moldes {
        assert!(
            !desenho.contains(&format!("live_id!({molde})")),
            "the drawing loop mentioned template `{molde}` again — the template \
             name lives in the editor, and a `match` here is undone decomposition"
        );
    }
}

/// Every editor names a template that the DSL can have.
///
/// The second half — that the template EXISTS — belongs to the DSL gate
/// (`dsl_gate.rs`), which instantiates each one. Here stays only the first: the
/// name is not empty and has no spaces, because `LiveId::from_str` on a malformed
/// name returns an empty `WidgetRef` and the row draws NOTHING, silently.
#[test]
fn todo_editor_nomeia_um_molde_plausivel() {
    for editor in inspector::editors::samples() {
        let nome = editor.template();
        assert!(
            !nome.is_empty() && nome.chars().all(|c| c.is_ascii_alphanumeric()),
            "`{nome}` is not a template name that the DSL can have"
        );
    }
}

/// The panel drawings exist in Codicons.
///
/// A code point the font does not have is an **empty box** on screen, and
/// nothing complains. The `cmap` reader is the one from `icon.rs` alongside,
/// borrowed instead of copied: a third reader would be the third place to fix
/// on the day the font changes format.
#[test]
fn todo_desenho_do_inspector_existe_na_codicons() {
    let tabela = super::icon_tests::codicons_code_points();
    for desenho in inspector::icon::ALL {
        for c in desenho.chars() {
            assert!(
                tabela.contains(&(c as u32)),
                "the glyph U+{:04X} is not in Codicons — it would appear as an \
                 empty box on the Inspector row",
                c as u32
            );
        }
    }
}

// ---------------------------------------------------------------------------
// THE `Events` TAB
// ---------------------------------------------------------------------------

/// Events of a class, directly from the tab rows.
fn events_of(classe: &str) -> Vec<inspector::EventRow> {
    let (d, sel) = doc_com(classe);
    linhas(Some(&d), &sel, Tab::Events)
        .into_iter()
        .filter_map(|l| match l {
            Row::Event(e) => Some(e),
            _ => None,
        })
        .collect()
}

/// Every catalog event becomes a row, and no list is written by hand.
#[test]
fn every_catalog_event_becomes_row_without_hardcoded_list() {
    let r = registro();
    for desc in r.all() {
        if desc.events.is_empty() {
            continue;
        }
        let Some(_) = r.instantiate(desc.class, "alvo", Rect::new(0.0, 0.0, 10.0, 10.0)) else {
            continue;
        };
        let l = events_of(desc.class);
        assert_eq!(
            l.len(),
            desc.events.len(),
            "the `Events` tab of `{}` shows {} event(s) and the catalog declares {}",
            desc.class,
            l.len(),
            desc.events.len()
        );
        for (linha, ev) in l.iter().zip(desc.events.iter()) {
            assert_eq!(linha.event, ev.name);
            assert_eq!(linha.label, ev.display);
            assert_eq!(linha.signature, ev.signature);
        }
    }
}

/// Two classes, two lists — proof that the list comes from the catalog.
#[test]
fn button_and_memo_have_different_event_lists() {
    let botao: Vec<&str> = events_of("Button").iter().map(|e| e.event).collect();
    let memo: Vec<&str> = events_of("Memo").iter().map(|e| e.event).collect();
    assert!(
        !botao.is_empty(),
        "`Button` was left with no event in the catalog"
    );
    assert!(
        !memo.is_empty(),
        "`Memo` was left with no event in the catalog"
    );
    assert_ne!(
        botao, memo,
        "`Button` and `Memo` started showing the SAME list of events. \
         This is the symptom of a handwritten list in the shell: the catalog's \
         is per class."
    );
}

/// No event name is hardcoded in the shell.
#[test]
fn no_event_name_is_hardcoded_in_shell() {
    use super::casca::CASCA;
    const PAINEL: &str = include_str!("../../../../crawrad-actions/src/inspector_model.rs");
    let r = registro();
    let nomes: BTreeSet<&str> = r
        .all()
        .flat_map(|d| d.events.iter().map(|e| e.name))
        .collect();
    assert!(
        !nomes.is_empty(),
        "the catalog was left with no events at all"
    );
    for nome in nomes {
        for (arquivo, texto) in [
            ("src/ide_app.rs", CASCA),
            ("src/ide_app/inspector.rs", PAINEL),
        ] {
            assert!(
                !texto.contains(&format!("\"{nome}\"")),
                "`{nome}` is written in {arquivo}. Reading is DYNAMIC: \
                 a new event in `crawrad-registry` appears without editing a line \
                 in the shell, and a name mentioned here is the handwritten list returning."
            );
        }
    }
}

/// The WIRED handler appears on the row; unlinked ones appear empty.
#[test]
fn a_linha_mostra_o_handler_que_o_documento_tem() {
    let (mut d, sel) = doc_com("Button");
    let primeiro = events_of("Button")
        .first()
        .map(|e| e.event)
        .expect("`Button` was left with no event");
    assert!(
        events_of("Button").iter().all(|e| e.handler.is_empty()),
        "a newly created component appeared with a wired handler"
    );
    d.node_mut(&sel[0])
        .expect("the node vanished")
        .events
        .insert(primeiro.to_string(), "Button1Click".to_string());
    let ligado = linhas(Some(&d), &sel, Tab::Events)
        .into_iter()
        .find_map(|l| match l {
            Row::Event(e) if e.event == primeiro => Some(e),
            _ => None,
        })
        .expect("the wired event row vanished");
    assert_eq!(
        ligado.handler, "Button1Click",
        "the row stopped showing the handler that the document has"
    );
}

/// With multiple items selected, the events tab refuses and SAYS why.
#[test]
fn events_tab_refuses_multi_selection() {
    let r = registro();
    let mut d = Document::new("form_main", "Form");
    for id in ["a", "b"] {
        let no = r
            .instantiate("Button", id, Rect::new(0.0, 0.0, 80.0, 24.0))
            .expect("`Button` vanished from the catalog");
        d.add_child("form_main", no).unwrap();
    }
    let sel = vec!["a".to_string(), "b".to_string()];
    assert_eq!(
        avisos(&linhas(Some(&d), &sel, Tab::Events)),
        vec![inspector::EVENTS_SINGLE_ONLY]
    );
}

/// A class without catalog events SAYS it has none — it does not stay blank.
#[test]
fn events_tab_explains_empty_catalog_events() {
    let r = registro();
    let Some(sem) = r.all().find(|d| d.events.is_empty()) else {
        return;
    };
    let (d, sel) = doc_com(sem.class);
    assert_eq!(
        avisos(&linhas(Some(&d), &sel, Tab::Events)),
        vec![inspector::NO_EVENTS],
        "the `Events` tab of `{}` was left blank instead of stating why",
        sem.class
    );
}

/// The two tabs are those of a classic RAD IDE, and `Properties` is the one that opens.
///
/// # What this test stopped proving, and why
///
/// It also read `src/inspector.rs` of the egui shell and checked that both
/// labels existed there — *"both shells show the SAME IDE"*. ADR 0032
/// removed that shell per owner request, and with it the other side of the comparison.
/// The test was NOT deleted because of inconvenience: what went away was the oracle that ceased
/// to exist.
///
/// What remains is still what matters, and never came from egui: the two
/// labels are those of the classic Object Inspector, and `Properties` is the tab that
/// opens — which is what is seen when selecting a component.
#[test]
fn the_two_tabs_are_the_classic_ones() {
    let rotulos: Vec<&str> = Tab::ALL.iter().map(|a| a.label()).collect();
    assert_eq!(rotulos, vec!["Properties", "Events"]);
    assert_eq!(Tab::default(), Tab::Properties);
}

// ---------------------------------------------------------------------------
// THE FOLDING CATEGORY — what the owner requested looking at the reference image
// ---------------------------------------------------------------------------

#[test]
fn a_categoria_dobra_e_o_cabecalho_fica() {
    // The owner compared the Makepad shell with the screenshot of the egui Object Inspector
    // and pointed out what was missing: *"the ▼/▶ group arrows"*. In the egui shell each
    // category is an `egui::CollapsingHeader`
    // (`crates/crawrad-ide/src/inspector.rs`), and here the categories were
    // inert labels.
    let (d, sel) = doc_com("Button");

    let aberto = inspector::rows(
        &registro(),
        Some(&d),
        &sel,
        ALVO,
        Tab::Properties,
        &fechado(),
        &fechado(),
    );
    let de_layout = |linhas: &[Row]| -> usize {
        linhas
            .iter()
            .filter(|r| matches!(r, Row::Prop(p) if p.depth == 0 && p.key == "rect"))
            .count()
    };
    assert_eq!(
        de_layout(&aberto),
        1,
        "with everything open, `Rect` must be in the list — this test stopped \
         reading the right place"
    );

    let fechada = inspector::rows(
        &registro(),
        Some(&d),
        &sel,
        ALVO,
        Tab::Properties,
        &fechado(),
        &BTreeSet::from(["Layout".to_string()]),
    );
    assert_eq!(
        de_layout(&fechada),
        0,
        "the closed `Layout` category continued showing its properties"
    );

    // AND THE HEADER REMAINS. That is what makes the fold reversible: disappearing along with
    // the content, the category would have nothing on screen to reopen — and the
    // user would lose `Rect`, `Anchors`, `Align`, and `Margins` all at once, with
    // no way back via the mouse.
    let cabecalho = |linhas: &[Row], alvo: &str| -> Option<bool> {
        linhas.iter().find_map(|r| match r {
            Row::Category { label, open } if *label == alvo => Some(*open),
            _ => None,
        })
    };
    assert_eq!(
        cabecalho(&fechada, "Layout"),
        Some(false),
        "the `Layout` header vanished along with the rows — the fold became a \
         one-way path"
    );
    assert_eq!(
        cabecalho(&aberto, "Layout"),
        Some(true),
        "the open header does not declare itself open, and the chevron comes out wrong"
    );

    // The OTHER categories are not affected: folding `Layout` cannot take
    // `Behavior` along with it.
    assert!(
        fechada
            .iter()
            .any(|r| matches!(r, Row::Prop(p) if p.key == "enabled")),
        "closing `Layout` took along a property from another category"
    );
}

#[test]
fn a_avancada_nasce_fechada_e_as_outras_nascem_abertas() {
    // It is `default_open(cat != Category::Avancado)` from the egui shell, and the reason
    // is the same: an Object Inspector that opens with everything closed requires four
    // clicks before the first edit, and `Advanced` is touched least.
    let fechadas = inspector::categories_closed_on_open();
    assert_eq!(
        fechadas,
        BTreeSet::from([crawrad_registry::Category::Advanced.label().to_string()]),
        "the seed of closed categories changed. It is DERIVED from `ORDER`, and \
         not a handwritten list — a new category in the catalog enters through the \
         filter"
    );
}
