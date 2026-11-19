//! The gate of the CENTRE: which view is in front, and whether the text it
//! shows was derived at that instant.
//!
//! # The defect it locks
//!
//! The `Code` tab of this shell was an **empty text box**: `toggle_view`
//! flipped two `visible` flags and nothing ever wrote into `code_view`. It
//! compiled, it looked like it worked, and the owner would have read a blank
//! screen. The wiring assertions below are the ones that would have gone red.
//!
//! The second, worse shape is the one the owner named on 15/09/2026: showing
//! the `.designer.rs` of **before** the edit. `show_center` re-derives on
//! every entry, and `o_texto_do_centro_e_derivado_na_hora` charges that call.

use std::path::Path;

use crawrad_doc::Doc;
use crawrad_ir::Document;
use crawrad_registry::Registry;

use super::source::{self, Center};

/// The Makepad shell — the whole file, as text.
use super::casca::CASCA;

/// `CASCA` with no whitespace at all — immune to whatever `rustfmt` picks.
fn compacto() -> String {
    CASCA.chars().filter(|c| !c.is_whitespace()).collect()
}

/// The body of an `impl IdeApp` method, compacted.
///
/// The end is the next method declaration, because every method in this `impl`
/// is indented by four spaces. It is the same tool as in `fios.rs`, and it
/// exists here so that an `assert` targeting one method does not match
/// text written in another.
fn method_body(nome: &str) -> String {
    let agulha = format!("fn {nome}(");
    let i = CASCA
        .find(&agulha)
        .unwrap_or_else(|| panic!("method `{nome}` disappeared from src/ide_app.rs"));
    let resto = &CASCA[i + agulha.len()..];
    let f = resto.find("\n    fn ").unwrap_or(resto.len());
    resto[..f].chars().filter(|c| !c.is_whitespace()).collect()
}

/// A form with one button, which is enough for the generator to have something
/// to say.
fn formulario() -> Document {
    let r = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(crawrad_ir::Rect::new(0.0, 0.0, 400.0, 300.0));
    let botao = r
        .instantiate(
            "Button",
            "Button1",
            crawrad_ir::Rect::new(8.0, 8.0, 75.0, 25.0),
        )
        .expect("catalog lost Button");
    d.add_child("form_main", botao).unwrap();
    d
}

// ----------------------------------------------------------- the three views

#[test]
fn f12_toggles_between_design_and_unit() {
    assert_eq!(Center::Design.toggle_code(), Center::Code);
    assert_eq!(Center::Code.toggle_code(), Center::Design);
    // From within FORM SOURCE, `F12` leads to the unit — not back
    // to design: it means "the other half of the pair", and the other half of
    // "the form as text" is still the unit.
    assert_eq!(Center::FormSource.toggle_code(), Center::Code);
}

#[test]
fn alt_f12_always_goes_to_source() {
    // The rule is from the catalog (`ViewEffect::ToggleFormSource`): "always go
    // TO source, regardless of where it came from, and only return from inside it — so
    // the key has a single response".
    assert_eq!(Center::Design.toggle_form_source(), Center::FormSource);
    assert_eq!(Center::Code.toggle_form_source(), Center::FormSource);
    assert_eq!(Center::FormSource.toggle_form_source(), Center::Design);
}

#[test]
fn initial_view_is_design() {
    assert_eq!(Center::default(), Center::Design);
    assert!(!Center::default().is_text());
    assert!(Center::Code.is_text() && Center::FormSource.is_text());
}

#[test]
fn tab_label_shows_design_mode() {
    // `Alt+F12` has no button: without the label saying `Design Code`, the only
    // hint that the key did anything would be that the center changed — and whoever
    // does not know the shortcut would have no way to get back.
    assert_eq!(Center::Design.design_tab_label(), "Design");
    assert_eq!(Center::FormSource.design_tab_label(), "Design Code");
    assert_eq!(Center::Code.design_tab_label(), "Design");
}

// ------------------------------------------------------------------ the text

#[test]
fn form_source_comes_from_open_document() {
    let texto = source::form_source(&formulario(), "meu_app").expect("generation failed");
    assert!(
        texto.contains("Button1"),
        "form source does not mention the component that is in the document"
    );
}

#[test]
fn editing_document_updates_form_source() {
    // This is the owner's criterion, measured without a screen: edit a property and
    // form source MUST reflect the edit. This is what makes re-deriving on
    // each entry worthwhile.
    let antes = source::form_source(&formulario(), "meu_app").expect("generation failed");

    let mut d = Doc::new(formulario());
    d.apply(
        crawrad_doc::Transaction::user("renomear").with(crawrad_doc::Command::SetProp {
            node: "Button1".into(),
            key: "text".into(),
            value: Some(crawrad_ir::Value::Str("Gravar".into())),
        }),
    )
    .expect("transaction failed");
    let depois = source::form_source(d.ir(), "meu_app").expect("generation failed");

    assert_ne!(
        antes, depois,
        "changing a property did not change form source: screen \
         would show text from before editing, and the user would believe it"
    );
    assert!(depois.contains("Gravar"));
}

#[test]
fn package_name_comes_from_manifest() {
    assert_eq!(
        source::package_name_in("[package]\nname = \"minha_app\"\nversion = \"0.1.0\"\n"),
        Some("minha_app".into())
    );
    // The `name` of a DEPENDENCY does not count: it is in another section.
    assert_eq!(
        source::package_name_in("[dependencies.serde]\nname = \"serde\"\n"),
        None
    );
    assert_eq!(source::package_name_in("[workspace]\nmembers = []\n"), None);
}

#[test]
fn member_is_folder_above_src() {
    assert_eq!(
        source::member_of(Path::new("/proj/app/src/form_main.crawform")),
        Some(std::path::PathBuf::from("/proj/app"))
    );
    // Outside a `src/` there is no member: this is the layout assumed across the entire IDE
    // (ADR 0017), and inventing one would cause `Cargo.toml` to be read from the wrong place.
    assert_eq!(
        source::member_of(Path::new("/proj/form_main.crawform")),
        None
    );
}

#[test]
fn non_existent_unit_reports_missing() {
    // WITHOUT A DOCUMENT there is nothing to plan, so the view shows the message. This is the only
    // case where it remains.
    let texto = source::user_unit(Path::new("/nao/existe/src/form.crawform"), None);
    assert!(
        texto.contains("no generated unit on disk yet"),
        "unit view was blank when the file does not exist — \
         blank is indistinguishable from an empty file"
    );
    assert!(
        texto.contains("Alt+F12"),
        "unit view does not state which gateway WORKS without generation"
    );
}

#[test]
fn form_unit_is_sibling_with_rs_extension() {
    assert_eq!(
        source::unit_path(Path::new("/proj/app/src/form_main.crawform")),
        std::path::PathBuf::from("/proj/app/src/form_main.rs")
    );
}

// -------------------------------------------------------------- the wirings

/// **Wiring** — text is written to the box upon entering the view.
#[test]
fn center_text_is_derived_on_demand() {
    let c = compacto();
    assert!(
        c.contains("lettexto=self.texto_do_centro();self.code_view().set_text(cx,&texto);"),
        "code view stopped receiving text. It goes back to being the \
         EMPTY box this shell had: two `visible` toggling and nothing written."
    );
    // And `CodeView` is REGISTERED. Without the editor crate's `live_design`,
    // Makepad cannot find the live definition, the view comes up empty, and the only place
    // where this shows up is the log — no compilation error, no broken
    // screen, just a blank box.
    assert!(
        c.contains("makepad_code_editor::live_design(cx);"),
        "`CodeView` is no longer registered. It fails silently again: \
         `Can't find live definition of CodeView` in log, and empty box on screen"
    );
    let project_source: String = include_str!("../../../../crawrad-project/src/source.rs")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    assert!(
        c.contains("source::center_content(")
            && project_source.contains("form_source(doc,&package)"),
        "form source is no longer PLANNED on demand. Showing text \
         derived without re-deriving it is the defect the owner named on \
         15/09/2026 — and `pousar_na_linha` of the egui shell already documented."
    );
    assert!(
        project_source.contains("user_unit(path,doc)")
            && c.contains("self.host.doc().map(|d|d.ir()),"),
        "unit view stopped reading disk — or stopped receiving the
         DOCUMENT, which is where the skeleton comes from when the file does not
         exist yet"
    );
}

/// **Wiring** — catalog `Alt+F12` reaches the center.
#[test]
fn catalog_alt_f12_is_bound() {
    let c = compacto();
    assert!(
        c.contains("ViewEffect::ToggleFormSource)=>self.toggle_form_source(cx)"),
        "`Alt+F12` fell back into `Some(_)` of \"unwired\". It is the \
         command the owner named: form source, generated on demand."
    );
    assert!(
        c.contains("ViewEffect::ToggleDesignCode)=>self.toggle_view(cx)"),
        "`F12` stopped toggling center view"
    );
    for id in ["view.toggle_code", "view.toggle_form_source"] {
        assert!(
            crawrad_actions::by_id(id).is_some(),
            "action `{id}` disappeared from catalog, and shell calls it by id"
        );
    }
}

/// **Wiring** — the center tab strip controls the same three views.
///
/// # The form changed TWICE, and the rule did not
///
/// The two tabs were `Button`, and the ACTIVE one was hardcoded in the DSL. The highlight
/// never moved, and what indicated which one was open was a bullet `•` next to the
/// label — the owner looked at it and said *"the bottom one DOESN'T EVEN LOOK LIKE a TAB"*. They became
/// `RadioButtonTab`, the same widget used for the Object Inspector tabs.
///
/// On 17/09/2026 the FIXED `Code` tab was removed: the owner asked for *"tabs for each .RS
/// opened in the editor"*, and the form unit became one of the file
/// tabs. Only ONE radio tab remained — the design tab —, and what this gate checks
/// changed with it: there is no longer an index to map to a view, there is the seam between
/// tabs and [`Center`], which `centro_das_abas` handles in a single place.
#[test]
fn center_tab_strip_uses_same_views() {
    let c = compacto();
    assert!(
        c.contains("abas_do_centro.as_radio_button_set().selected(cx,actions)"),
        "footer strip is no longer a radio button set. Either it reverted \
         to two `Button`s — and highlight gets stuck again —, or \
         nothing listens to clicks"
    );
    assert!(
        c.contains("self.abas.show_design();"),
        "clicking design tab stopped bringing down file tab. \
         Canvas would appear with a `.rs` tab still lit up above"
    );
    assert!(
        c.contains("self.show_center(cx,volta);"),
        "design tab stopped switching view. It is the only mouse \
         gateway to undo `Alt+F12`"
    );
    // THE TWO STRIPS COVER DIFFERENT THINGS, as the footer `Code` proves.
    //
    // The TOP one covers FILES — one tab per `.rs`. The BOTTOM one covers the
    // FORM: its design or its unit. This was the correction requested by the owner
    // on 17/09/2026 looking at the screen: *"RS tab on the upper part, down here
    // only Design/Code"*.
    //
    // And they meet at a single point: footer `Code` opens the unit of
    // THIS form, through the SAME path as `F12` — a second path
    // would open two tabs for the same file.
    assert!(
        c.contains("Some(form)=>self.abrir_no_editor(cx,source::unit_path(&form)),"),
        "footer `Code` stopped opening form unit through single \
         gateway. With a second gateway, same unit opens in two tabs"
    );
    assert!(
        c.contains("(id!(center_tabs_bar.tab_code),\"Code\")"),
        "footer `Code` tab disappeared. It is the pair of `Design` and \
         refers to the FORM, not an arbitrary file"
    );
    // AND THE FILE STRIP IS AT THE TOP, in another row — not next to the pair.
    assert!(
        c.contains("id!(editor_tabs_bar.editor_tabs)"),
        "file strip returned to footer. There it contends for the same \
         bar with a tab about something else, which is why owner \
         said it wouldn't work that way"
    );
    assert!(
        !c.contains("id!(center_tabs_bar.editor_tabs)"),
        "a file strip path remains inside footer"
    );
    // AND THE SEAM BETWEEN THE TWO STATES LIVES IN ONE PLACE ONLY.
    //
    // `self.abas` indicates which file is in front and `self.centro` indicates what the
    // screen draws. If each caller decided the pair on its own, the
    // divergence would be invisible: an active tab over the canvas.
    assert!(
        c.contains("fncentro_das_abas(&self)->Center{"),
        "`centro_das_abas` disappeared. It matches foreground tab with view"
    );
    for chamada in [
        "self.abas.close(i);self.show_center(cx,self.centro_das_abas());",
        "self.switch_tab(cx,|a|a.activate(i));self.show_center(cx,self.centro_das_abas());",
    ] {
        assert!(
            c.contains(chamada),
            "file strip stopped matching tab with view: `{chamada}`"
        );
    }
    assert!(
        c.contains("(id!(center_tabs_bar.tab_design),centro.design_tab_label())"),
        "design tab label stopped showing which mode it is in"
    );
    // And it is written at STARTUP too, not only when the view changes.
    //
    // Without this the strip starts BLANK: two tabs without text and neither active —
    // observed in screenshots. `show_center` only runs when the view changes, and at
    // startup nothing changes. That is why the writer is its own function, with
    // two callers.
    assert!(
        method_body("write_panel_chrome").contains("self.write_center_tabs(cx);"),
        "center tab strip stopped being written on startup. It is \
         born blank again, and clicking anywhere on it switches view \
         with nothing explaining why"
    );
}

/// The unit view does not tell the user to generate in the OTHER shell.
///
/// # The lie, and why it is of the most expensive kind
///
/// The phrase said *"The Makepad shell does not write to disk yet: run
/// `Generate` in the egui shell"*. It was true when written and ceased to
/// be true when `FileEffect::Generate` was wired — since then this shell generates,
/// compiles, and launches.
///
/// A message that outlives the gap it describes is worse than no message
/// at all: it tells the user to leave the IDE to do what the IDE already does, and the user
/// has no way of knowing the message is outdated.
///
/// # Why the test binds the PHRASE to the WIRING
///
/// Checking only the text would lock today's phrasing without saying why, and the day
/// `Generate` leaves this shell the test would stay green demanding
/// a message that has become false again — in reverse. Here both halves are
/// tested together: **because** the wiring exists, the message must not send the user away.
#[test]
fn unit_view_does_not_trigger_generate_in_other_shell() {
    let casca = compacto();
    assert!(
        casca.contains("Some(crawrad_actions::FileEffect::Generate)=>self.gerar_projeto(cx),"),
        "`Generate` left this shell. If it really left, unit view \
         message must say where to generate again — and this test is \
         where that decision appears"
    );
    assert!(
        !source::NOT_GENERATED_YET.contains("egui shell"),
        "unit view reverted to asking to generate in egui shell, but \
         `FileEffect::Generate` is wired HERE: message tells user to leave the \
         IDE to do what the IDE already does"
    );
    // And it continues to state WHERE to generate — removing the falsehood without putting
    // the truth in its place would leave the user stranded.
    assert!(
        source::NOT_GENERATED_YET.contains("Generate"),
        "message stopped stating what writes the unit"
    );
}

/// Without the file on disk, the view shows the SKELETON that generation would write.
///
/// # What the owner requested, looking at the screen
///
/// > *"this `.rs` in egui already had the initial top part, which was the form code,
/// > so event code could be added subsequently"*
/// >
/// > *"we need this back: when creating a new project, already have this
/// > header with `use` for the form, and leave it ready to add event blocks"*
///
/// A view that answers *"there is no unit on disk"* is technically true and
/// useless: what the user went there to see is WHERE their code goes, and that structure
/// exists before any file.
#[test]
fn without_file_on_disk_view_shows_generation_skeleton() {
    let doc = formulario();
    let texto = source::user_unit(Path::new("/nao/existe/src/form_main.crawform"), Some(&doc));

    assert!(
        texto.starts_with(source::NOT_ON_DISK_YET),
        "skeleton appeared WITHOUT stating it is not on disk yet — \
         user would look for a file that does not exist"
    );
    assert!(
        texto.contains("_designer::*;"),
        "skeleton lacks form `use`. It is the entire top: without \
         form type the lower `impl` does not compile, and that \
         header was exactly what owner asked for"
    );
    assert!(
        texto.contains("impl "),
        "skeleton lacks `impl` where handlers live"
    );
    assert!(
        texto.contains("New event handlers go here."),
        "skeleton lost the MARK. It indicates where next \
         event block goes — and `event.bind` uses it to insert handler without \
         touching the rest"
    );
    // And the skeleton is DERIVED, not hardcoded here: it is the same file that
    // generation produces, selected by the owner that the generator marks.
    let do_gerador = crawrad_codegen::targets::AlvoGerador::default()
        .create_generator()
        .gerar(&doc, "app", &crawrad_registry::Registry::builtin())
        .expect("plan failed")
        .into_iter()
        .find(|a| a.dono == crawrad_codegen::Dono::Usuario)
        .expect("generator stopped producing user file");
    assert!(
        texto.ends_with(&do_gerador.conteudo),
        "view skeleton diverged from what generation writes. It must \
         be the SAME text: a second copy of header here would grow \
         stale, because nothing generates from this function"
    );
}
