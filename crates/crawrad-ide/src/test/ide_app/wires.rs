//! The gate of WIRES — what connects the pure function to the screen.
//!
//! # The defect this file exists to lock down
//!
//! The shell has plenty of tests for pure functions: [`documento::resumo`],
//! [`documento::linhas_da_arvore`], and `paleta::linhas` are covered number
//! for number. But **a perfect `resumo()` called by no one passes
//! everything**. Measured by mutation, in a single line:
//!
//! ```text
//! - let direita = documento::resumo(self.doc.as_ref());
//! + let direita = String::from("no selection    19 nodes    rev 0");
//! ```
//!
//! → `cargo test -p crawrad-ide` stayed **476 + 36, 0 failed**. The fiction that
//! this shell claimed to have killed returned to the screen and nothing blinked. The same was true
//! for `Undo` becoming `=> {}`, `Save` becoming `=> {}`, and `rt.open(d.ir())` leaving
//! `reabrir_no_canvas` — in all three the designer becomes mute again, and in all three the
//! suite stays green.
//!
//! # What this gate does NOT prove, and it is important to state
//!
//! It reads `src/ide_app.rs` as **text**. So it proves that the CALL is
//! written in the right place in the file — nothing beyond that:
//!
//! - it does not prove that the call was executed at runtime (the `match`
//!   might never be reached);
//! - it does not prove that what it returned reached the screen with the right drawing;
//! - it does not prove that the value is correct — a wrong `resumo()` and call passes
//!   green here, and its unit test is responsible for that.
//!
//! It is the ceiling of text testing, and it is the ceiling of what fits without a real
//! window: instantiating `IdeApp` requires `Cx`, a window, and an event loop. What
//! this gate provides is that **undoing the wire costs a red test**, which is what
//! was missing.
//!
//! The technique is that of `paleta.rs` (`o_campo_filter_da_toolbox_nao_e_ui_morta`):
//! the file is read without any whitespace, so that whatever line break
//! `rustfmt` chooses does not break the test.

/// The Makepad shell — the entire file, as text.
use super::casca::CASCA;

/// `CASCA` with no whitespace at all.
///
/// Compacting is what makes the search immune to `rustfmt`: `=> self.salvar()` and
/// `=>\n    self.salvar()` become the same thing.
/// The debugger glue, where `Run` decides between continuing and
/// running.
const DEPURACAO: &str = include_str!("../../ide_app/debug.rs");

fn compacto() -> String {
    CASCA.chars().filter(|c| !c.is_whitespace()).collect()
}

/// The snippet of `CASCA` from `inicio` to `fim`, already compacted.
///
/// Slicing before asserting is what prevents a `d.undo()` written in the
/// `Refazer` arm from answering for the `Desfazer` arm.
fn trecho(inicio: &str, fim: &str) -> String {
    let i = CASCA
        .find(inicio)
        .unwrap_or_else(|| panic!("`{inicio}` disappeared from src/ide_app.rs"));
    let resto = &CASCA[i..];
    let f = resto
        .find(fim)
        .unwrap_or_else(|| panic!("`{fim}` disappeared from src/ide_app.rs after `{inicio}`"));
    resto[..f].chars().filter(|c| !c.is_whitespace()).collect()
}

/// The body of a method in `impl IdeApp`, compacted.
///
/// The end is the next method declaration — `\n    fn ` — because every method
/// in this `impl` is indented by four spaces.
fn method_body(nome: &str) -> String {
    let agulha = format!("fn {nome}(");
    let i = CASCA
        .find(&agulha)
        .unwrap_or_else(|| panic!("method `{nome}` disappeared from src/ide_app.rs"));
    let resto = &CASCA[i + agulha.len()..];
    // The next method, whatever its visibility: the split of `ide_app.rs`
    // widened private methods to `pub(super)`, and a body that only stopped at
    // `fn ` ran on into its neighbours.
    let f = [
        "\n    fn ",
        "\n    pub fn ",
        "\n    pub(crate) fn ",
        "\n    pub(super) fn ",
    ]
    .iter()
    .filter_map(|next| resto.find(next))
    .min()
    .unwrap_or(resto.len());
    resto[..f].chars().filter(|c| !c.is_whitespace()).collect()
}

/// How many times `agulha` appears in `CASCA`.
fn vezes(agulha: &str) -> usize {
    CASCA.matches(agulha).count()
}

/// **Wire 1** — the status bar is CALCULATED, not hardcoded.
///
/// The mutation this test catches is the one the reviewer made by hand: replacing the call
/// with `String::from("no selection    19 nodes    rev 0")`. With the literal, the
/// bar goes back to lying about the open form and the entire suite stays
/// green.
#[test]
fn status_bar_comes_from_document_not_literal() {
    let c = compacto();
    assert!(
        c.contains("documento::counters(self.host.doc())"),
        "the right cells are no longer computed by \
         `documento::counters`. A literal there is the fiction this shell came to \
         remove from the screen: the `19 nodes` was right BY ACCIDENT."
    );
    assert!(
        c.contains("documento::file_label(self.host.active_file(),self.falha.as_deref())"),
        "the message cell no longer shows the REAL path of the open file \
         (or the failure). The hardcoded path pointing to `Downloads` was the \
         first of the three fictions."
    );
    // AND THE CALCULATED CELLS REACH THE SCREEN. Calculating and not writing amounts
    // to the same as writing a literal, and it is the half of the wire this file exists
    // to check.
    assert!(
        c.contains("letcelulas=self.celulas_do_rodape();"),
        "`sincronizar` stopped assembling the footer cells"
    );
    assert!(
        c.contains("self.escrever_rodape(cx,&celulas);"),
        "the cells are computed and do not reach any label"
    );
    assert!(
        method_body("escrever_rodape").contains("self.ui.label(slot).set_text(cx,texto);"),
        "`escrever_rodape` stopped writing. An empty `for` over the slots \
         passes every text gate and leaves the bar blank"
    );
    // THE COUNT OF ERRORS AND WARNINGS COMES FROM THE BUILD, and not from a number hardcoded here.
    assert!(
        c.contains("diagnostics_model::parse_diagnostics(&outcome.stdout)"),
        "the problems cell stopped reading the `cargo` verdict. A fixed \
         number there is the same family as `19 nodes` being right by accident"
    );
}

/// **Wire 2** — `Undo` and `Redo` mutate the document.
///
/// The mutation: `Some(crawrad_actions::Historico::Desfazer) => {}`. Both buttons
/// go back to doing nothing, **silently** — no status bar, no log.
/// Slicing by `match` arm is what prevents the neighbor's `d.redo()` from
/// answering for `Desfazer`.
#[test]
fn undo_and_redo_reach_document() {
    let desfazer = trecho("History::Undo", "History::Redo");
    assert!(
        desfazer.contains("d.undo()"),
        "the `History::Undo` arm of `apply_effect` does not call `Doc::undo`. \
         The Undo button becomes mute again, and nothing on screen reports it."
    );
    // `fim` is searched in the RAW text before compacting — which is why it comes
    // written as `rustfmt` leaves it.
    let refazer = trecho("History::Redo", "None => {}");
    assert!(
        refazer.contains("d.redo()"),
        "the `History::Redo` arm of `apply_effect` does not call `Doc::redo`."
    );
    let c = compacto();
    assert!(
        c.contains("self.apply_effect(cx,efeito)"),
        "the action effect is no longer applied: `apply_effect` became a \
         perfect function that no one calls, which is exactly the defect this \
         file locks down."
    );
}

/// **Wire 3** — `Save` saves.
///
/// The mutation: `Some(crawrad_actions::FileEffect::Save) => {}`. The Save button starts
/// never saving, and the `saved` badge in the title keeps saying `saved` because
/// the revision never changes. Nothing blinks.
#[test]
fn save_writes_form_to_disk() {
    let c = compacto();
    assert!(
        c.contains("Some(crawrad_actions::FileEffect::Save)=>self.salvar(cx)"),
        "the `FileEffect::Save` arm of `apply_effect` does not call `salvar()`.          The Save button goes back to never saving."
    );
    // Since Save All the form's half lives in `save_form`, and `salvar`
    // hands it over. Both bodies are read: a `salvar` that stopped calling
    // `save_form` is the mute button again, and so is a `save_form` that
    // stopped writing.
    assert!(
        method_body("salvar").contains("self.save_form(cx);"),
        "`salvar()` stopped handing the form over to `save_form`. The \
         Save button goes back to never saving."
    );
    // The form is written where its `Name` puts it, through
    // `save_form_file`, which also marks the revision it wrote.
    let salvar = method_body("save_form");
    assert!(
        salvar.contains("self.save_form_file()"),
        "`save_form` stopped saving the form through `save_form_file`."
    );
    let file = method_body("save_form_file");
    assert!(
        file.contains("crawrad_project::form_file::save(&d.own_ir(),"),
        "`salvar()` stopped writing the document to the path it          came from. A `salvar()` that does not save is the same mute button, one layer          deeper."
    );
    assert!(
        file.contains("self.rev_salva=d.revision()"),
        "`salvar()` does not mark the saved revision, and the `saved`/`modified`          badge in the title goes back to meaning nothing."
    );
    // **AND `Ctrl+S` GENERATES**, which is what PRUNES the empty handler (ADR 0025) and
    // proposes disconnecting the remaining binding. Without this call, the classic RAD IDE gesture
    // — created by mistake, wrote nothing, saved, and the block disappears along with the
    // binding — stops happening, and nothing in the build complains.
    assert!(
        salvar.contains("self.gerar_projeto(cx);"),
        "`Ctrl+S` in this shell stopped generating. What prunes the empty handler is          generation, and without it the empty block stays in the file and the name stays in the          Object Inspector"
    );
    assert!(
        salvar.contains("crawrad_project::generator::should_generate_on_save("),
        "the decision to generate on save stopped coming from the function that BOTH shells          read — and two rules for the same question diverge"
    );
}

/// **Wire 4** — the document reaches the canvas.
///
/// The mutation: removing `rt.open(d.ir())` and `rt.set_selection(...)` from
/// `reabrir_no_canvas`. The designer **opens blank** — and since the fake
/// form in DSL is already gone, there isn't even a mockup to disguise it.
#[test]
fn document_is_delivered_to_canvas() {
    let corpo = method_body("reabrir_no_canvas");
    assert!(
        corpo.contains("rt.open(d.ir())"),
        "`reabrir_no_canvas` no longer delivers the document to the canvas \
         runtime. The designer opens blank."
    );
    assert!(
        corpo.contains("rt.set_selection(d.selection())"),
        "the document selection no longer reaches the canvas: the eight handles of \
         `crawrad-runtime` have nothing to draw."
    );
    assert!(
        corpo.contains("None=>rt.close()"),
        "without a document the canvas must CLOSE. Leaving the previous one on screen \
         shows a form that no one opened."
    );

    // And the wire of the wire: a correct `reabrir_no_canvas` that no one calls is the
    // same defect one layer up.
    assert!(
        vezes("self.reabrir_no_canvas(cx)") >= 2,
        "`reabrir_no_canvas` is barely called. It must respond at \
         least upon loading the file and upon editing the document; if only the \
         definition remains, the screen freezes on the first frame."
    );
    assert!(
        vezes("self.sincronizar(cx)") >= 2,
        "`sincronizar` is barely called, and it is what writes the status \
         bar, the title, and the `saved` badge."
    );
}

/// The gesture loop keeps leaving the canvas and returning to it.
///
/// It is not one of the four mutated wires, and it is included because it is the SAME defect:
/// `crawrad-designer` has 6,800 tested lines that are only worthwhile if someone
/// feeds them ([ADR 0027]).
#[test]
fn canvas_gesture_passes_through_designer() {
    let corpo = method_body("bombear");
    assert!(
        corpo.contains("drain_events()"),
        "the shell stopped draining the canvas gesture"
    );
    for gesto in ["pointer_down(", "pointer_move(", "pointer_up("] {
        assert!(
            corpo.contains(&format!("self.designer.{gesto}")),
            "the gesture `{gesto}` no longer reaches `crawrad-designer`: the shell \
             would go back to having to decide hit-testing and handles, which is the error \
             ADR 0027 came to forbid"
        );
    }
}

// -------------------------------------------------------------------------
// THE OBJECT INSPECTOR WIRES
// -------------------------------------------------------------------------
//
// The entire panel comes from a pure function over the catalog, and it has the 23
// tests in `test/ide_app/inspector.rs`. All of them would pass with the panel
// disconnected from the screen — it is the opening of this file, under another name.

/// **Wire 5** — Inspector rows are CALCULATED from the catalog, and drawn.
///
/// The mutation this test catches: replacing `self.linhas_do_inspector()` in
/// `handle_draw_2d` with `Vec::new()`. The panel stays blank forever,
/// no unit test blinks, and the owner goes back to being unable to judge the
/// design of a `CheckBox`.
#[test]
fn inspector_rows_reach_screen() {
    let corpo = method_body("linhas_do_inspector");
    assert!(
        corpo.contains("inspector::rows_with_columns("),
        "the Inspector stopped reading the catalog. A hand-written list here \
         would be the Toolbox again — two sources, and the second aging silently."
    );
    assert!(
        corpo.contains("crawrad_project::generator::target_key()"),
        "the Inspector stopped filtering properties by the project's OUTPUT \
         framework (#491, ADR 0002): a property the target does not draw \
         would appear as if it changed the generated app."
    );
    assert!(
        corpo.contains("self.designer.registry"),
        "the Inspector started loading a SECOND catalog. Two \
         `Registry::builtin()` calls are two catalogs to diverge."
    );

    let desenho = if CASCA.contains("fn draw_2d(") {
        method_body("draw_2d")
    } else {
        method_body("handle_draw_2d")
    };
    assert!(
        desenho.contains("self.linhas_do_inspector()"),
        "drawing stopped requesting Inspector rows: the panel stays \
         blank and nothing complains"
    );
    assert!(
        desenho.contains("inspetor.borrow_mut_if_eq(&proximo)"),
        "the Inspector list is no longer populated during the draw step: a \
         `PortalList` that no one populates draws zero rows"
    );
    assert!(
        desenho.contains("Self::escrever_linha("),
        "the Inspector row is no longer written: templates would be born with the \
         blank text declared by the DSL"
    );
}

/// **Wire 6** — each row's template comes from the EDITOR ITSELF, not from the property
/// name or the class.
///
/// It is the law of composition for this round, written where it can be broken. A
/// `match p.key` here would resurrect the hand-written table that Toolbox
/// just lost — and a `match p.editor` as well: the
/// template name lives in the editor, which is one file per type in
/// `ide_app/inspector/editors/`.
#[test]
fn row_template_comes_from_type_not_name() {
    let desenho = if CASCA.contains("fn draw_2d(") {
        method_body("draw_2d")
    } else {
        method_body("handle_draw_2d")
    };
    assert!(
        desenho.contains("LiveId::from_str(p.editor.template())"),
        "template dispatch stopped asking the editor. A `match` by \
         type here brings back the central `match`: adding a property \
         type would again cost a line in this shell instead of a \
         file."
    );
    // And no property name or class name hardcoded in the dispatch. The list
    // is short on purpose: these are the keys and classes that a `match` by
    // name would use first.
    for nome in [
        "\"text\"",
        "\"caption\"",
        "\"align\"",
        "\"checked\"",
        "\"position\"",
        "\"Button\"",
        "\"CheckBox\"",
        "\"ScrollBar\"",
    ] {
        assert!(
            !desenho.contains(
                &nome
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>()
            ),
            "Inspector drawing started deciding by {nome}. The editor comes \
             from `PropKind`, and a new property in the catalog must appear without \
             a line edited in the shell."
        );
    }
}

/// **Wire 7** — saving goes through the ACTION CATALOG, and nothing else.
///
/// The mutation: building a `Transaction` here. It would work on screen and vanish
/// from the protocol — the RTA agent and MCP would have no way to change a property
/// (ADR 0005), and the `crawrad-actions` ratchet would go from 5 to 6.
#[test]
fn inspector_save_passes_through_catalog() {
    let corpo = method_body("write_property");
    assert!(
        corpo.contains("\"component.set_property\""),
        "the Inspector stopped saving through the catalog action"
    );
    // The Name goes through the Rename everywhere card, and its Rename
    // still goes through the catalogue.
    assert!(corpo.contains("start_component_rename"));
    assert!(
        method_body("execute_component_rename_all").contains("\"component.rename\""),
        "`Name` stopped going through `component.rename`: a `set_property` with the \
         `name` key would bypass uniqueness and invalid name \
         rejection, silently (ADR 0008)"
    );
    assert!(
        !CASCA.contains("Transaction::"),
        "a handwritten transaction appeared in the Makepad shell. It does not exist \
         for RTA nor for MCP, and the `crawrad-actions` ratchet increases (ADR 0005)."
    );
}

/// **Wire 7b** — whatever changes the document REPAINTS the screen, and that lives at the PORT.
///
/// # The defect family this test closes
///
/// Until 2026-09-17 `apply_effect` applied the transaction and left: whoever
/// repainted was the CALLER, and each one called `reabrir_no_canvas` +
/// `sincronizar` on its own. Whoever forgot was left with the changed document
/// and the screen unchanged — the worst possible outcome, because nothing complains.
///
/// That was exactly what happened with PALETTE DRAG. Measured with a probe:
/// `Drop` arrived, the class and point were right, `component.add`
/// ran, and the document went to `rev=1` — and the bar kept saying `20 nodes
/// rev 0`, with the canvas missing the component. It wasn't that dragging didn't
/// work; it was that the screen didn't know.
///
/// The mutation this test catches is the return of that design: removing the
/// repaint from the port and handing it back to callers. It compiles again, it passes
/// on screen again for callers that remember — and the next caller is born
/// broken again.
#[test]
fn document_mutator_repaints_via_own_port() {
    let corpo = method_body("apply_effect");
    assert!(
        corpo.contains("self.reabrir_no_canvas(cx);"),
        "the actions port stopped reopening the canvas. Each caller must \
         remember again, and whichever forgets changes the document without changing a pixel \
         — that is how palette drag broke silently"
    );
    assert!(
        corpo.contains("self.sincronizar(cx);"),
        "the actions port stopped synchronizing panels: Structure, \
         Inspector, and the status bar would keep showing the PREVIOUS document"
    );
    assert!(
        corpo.contains("letantes=self") && corpo.contains("letdepois=self"),
        "the port lost the BEFORE and AFTER snapshot. Without it, either it repaints \
         the entire IDE on every gesture that changed nothing, or it goes back to never \
         repainting"
    );
    assert!(
        !method_body("colocar").contains("reabrir_no_canvas"),
        "`colocar` repaints externally again. Repainting belongs to the port: \
         scattering the call is what caused the next caller to be born without it"
    );
}

/// **Wire 8** — what the user types is confirmed, and refusals are reported.
#[test]
fn typed_inspector_input_is_confirmed() {
    let corpo = method_body("confirmar_edicao");
    assert!(
        corpo.contains("editor.gesture(e.slot,inspector::Gesture::Typed(&e.texto))"),
        "typed text conversion disappeared: the field would accept anything \
         and save something else"
    );
    // Refusal is reported, and it became a single place since the four
    // outputs of `Outcome` come from three different gestures, and three copies
    // would be three chances for one of them to swallow the message.
    let resultado = method_body("aplicar_resultado");
    assert!(
        resultado.contains("inspector::Outcome::Refuse(porque)=>{self.status=Some(porque)"),
        "invalid number refusal stopped being reported. Typing `abc` in \
         `Position` and leaving the field would become silent nothingness — a dead field \
         in different clothing."
    );
    assert!(
        vezes("self.confirmar_edicao(cx)") >= 2,
        "`confirmar_edicao` is barely called: it responds to `Enter` and \
         loss of focus, which are the two moments when a classic RAD IDE's \
         Object Inspector saves"
    );
}

/// **Wire 9** — checkbox and combobox write what the gesture said.
#[test]
fn inspector_checkbox_and_combo_are_not_dead_ui() {
    let c = compacto();
    assert!(
        c.contains("item.check_box(caminho).changed(actions)"),
        "the Inspector checkbox stopped being listened to: checking and \
         unchecking would not change the document, and the checkmark on canvas \
         would remain what it was"
    );
    assert!(
        c.contains("item.drop_down(caminho).selected(actions)"),
        "the Inspector combo stopped being listened to: choosing `Align` would \
         not change anything"
    );
    assert!(
        c.contains("item.text_input(caminho)"),
        "the Inspector text input stopped being listened to"
    );
}

/// **Wire 10** — the panel header says WHO is selected.
///
/// It is written before the status bar shortcut on purpose: there is a gesture that
/// changes selection without changing a single letter on the bar, and after `return` the panel
/// would say the name of the previous component.
#[test]
fn inspector_header_comes_from_document() {
    let corpo = method_body("sincronizar");
    assert!(
        corpo.contains("inspector::header("),
        "the Inspector header stopped being calculated from the document"
    );
    let ate_o_atalho = corpo
        .split("if(esquerda.clone(),direita.clone())==self.barra_na_tela")
        .next()
        .unwrap_or("");
    assert!(
        ate_o_atalho.contains("inspector::header("),
        "the Inspector header ended up AFTER the status bar shortcut: \
         switching selection without changing the bar would leave the panel \
         displaying the name of the previous component"
    );
}

/// **Wire 11 — `Run` reaches the child process.**
///
/// What the owner saw on 2026-09-16: *"Run here doesn't work yet, so no
/// way to test"*. The button was wired to the catalog, the action returned
/// `FileEffect::Run` — and the `apply_effect` arm sent everything other than
/// `Salvar` to a `Some(_)` with an apology message. The entire chain
/// compiled, and no test turned red.
///
/// This wire checks the entire chain, link by link, because cutting ANY of
/// them returns the same symptom — a button that responds and does nothing:
///
/// 1. the `FileEffect::Run` arm calls `rodar_projeto`;
/// 2. `rodar_projeto` GENERATES before compiling, and gives up if generation was refused;
/// 3. it spawns `cargo build` in a THREAD, via `launcher`;
/// 4. it arms the timer — without a timer Makepad sleeps and the verdict is never
///    collected;
/// 5. the timer is handled in `handle_event`;
/// 6. the green verdict LAUNCHES, supervised by `AppRun` with `CargoCall::run`.
///
/// What it does **not** prove is in the header of this file: that the call was
/// executed, and that what appeared on the screen was correct.
#[test]
fn run_generates_compiles_and_launches() {
    let c = compacto();

    // 1. THE ARM. The variant gate in `crawrad-actions` already rejects the
    //    generic `Some(_)`; here we check where the arm points.
    // The arm goes through `run_or_continue`: with a
    // program PAUSED in the debugger, the same gesture continues (the `F9` of a classic RAD IDE);
    // without a session, it falls through to `rodar_projeto`. Both links are checked.
    let depuracao: String = DEPURACAO.chars().filter(|c| !c.is_whitespace()).collect();
    let continuar = depuracao
        .split("fnrun_or_continue")
        .nth(1)
        .and_then(|resto| resto.split("pub(super)fn").next())
        .expect("`run_or_continue` disappeared from src/ide_app/debug.rs");
    assert!(
        continuar.contains("self.rodar_projeto(cx)"),
        "without a debug session, `Run` stopped falling through to `rodar_projeto`."
    );
    assert!(
        c.contains("Some(crawrad_actions::FileEffect::Run)=>self.run_or_continue(cx)"),
        "the `FileEffect::Run` arm of `apply_effect` does not call `rodar_projeto`. \
         Issue #147 is back: the button dispatches, the action returns the \
         right effect, and nothing happens."
    );
    assert!(
        c.contains("Some(crawrad_actions::FileEffect::Generate)=>self.gerar_projeto(cx)"),
        "the `FileEffect::Generate` arm does not call `gerar_projeto`."
    );

    // 2. GENERATE FIRST, and give up when generation was refused.
    // Through `prepare_build`, which generates whenever a form is open
    // and only saves for a project with no form (a console app).
    let rodar = method_body("rodar_projeto");
    let preparar = method_body("prepare_build");
    assert!(
        rodar.contains("if!self.prepare_build(cx){return;}")
            && preparar.contains("self.gerar_projeto(cx)")
            && preparar.contains("ifself.host.doc().is_none(){returnself.save_before_build(cx);}"),
        "`Run` stopped GENERATING before compiling. Running what was on disk \
         while the design on screen is already different is the lie that costs an hour \
         of debugging."
    );
    assert!(
        preparar.contains("crawrad_project::generator::refused"),
        "`Run` stopped checking generator refusal. Without this it calls \
         cargo on code that was not written, and the user reads the compiler error \
         instead of ours."
    );
    assert!(
        rodar
            .find("self.prepare_build(cx)")
            .zip(rodar.find("self.build_tasks.spawn(\"build\""))
            .is_some_and(|(gerar, build)| gerar < build),
        "generation ended up AFTER triggering cargo. The order is the entire \
         function."
    );

    // 3 and 4. THE BUILD TASK.
    assert!(
        rodar.contains("CargoCall::build(") && rodar.contains("self.build_tasks.spawn(\"build\""),
        "`Run` no longer triggers `cargo build` in a task: `cargo run` on the          thread only returns when the user application CLOSES."
    );

    // 5. THE SIGNAL IS HANDLED.
    assert!(
        c.contains("ifmatches!(event,Event::Signal){self.bombear_pacotes(cx);self.bombear_criacao(cx);self.bombear_build(cx);self.bombear_mcp(cx);self.pump_environment(cx);self.pump_disk_changes(cx);self.pump_debugging(cx);self.bombear_audio(cx);self.bombear_ia(cx);self.pump_doctest(cx);self.pump_dataset_columns(cx);}"),
        "the UI signal no longer pumps task managers in `handle_event`."
    );

    // 6. THE GREEN VERDICT LAUNCHES, and launch releases the child.
    let bombear = method_body("bombear_build");
    assert!(
        bombear.contains("crawrad_os::process::AfterBuild::Launch=>{")
            && bombear.contains("self.lancar_projeto(cx,&artefatos)"),
        "green verdict stopped launching. Compiling without running is half of \
         `Run`, and the half the owner sees."
    );
    let lancar = method_body("lancar_projeto");
    // Two needles and not an entire phrase: `compacto()` removes ALL whitespace,
    // and `&mut crate::` would become `&mutcrate::` — a needle that no one
    // can read is a needle the next person deletes.
    assert!(
        lancar.contains("self.app_run.start("),
        "the launch no longer goes through `AppRun`, which waits for the app on \
         a task (never on the UI thread) and reads its panic when it ends (#926)."
    );
    assert!(
        lancar.contains("crawrad_os::process::CargoCall::run(&pacote,&raiz).command()"),
        "what is launched stopped coming from `CargoCall::run`. A second hand-written \
         argument list was how issue #67 started."
    );
    assert!(
        lancar.contains("has_binary(&raiz,&pacote)"),
        "the launch safety belt disappeared. In a `--lib` project `cargo run` \
         responds `a bin target must be available`, and in an application launched \
         WITHOUT CONSOLE that phrase appears nowhere."
    );
}

/// **Wire** — `Preview` reaches the CANVAS, and the canvas REPAINTS.
///
/// # The defect this test locks down, and it was measured on screen
///
/// The arm was written correctly — it put `Modo::Vivo` in the canvas runtime — and
/// `Preview` still didn't work. Two probes showed why: the
/// mutated canvas pointer was the SAME one that draws, the mode was already `Vivo`
/// in it, **and the last painted frame was before the click**. The arm asked for
/// `self.ui.redraw(cx)`, which marks the root and does not descend to the canvas.
///
/// The user saw exactly what they had complained about: clicking `Preview`, the status
/// bar changing, and the ruler, grid, and handles remaining on screen.
///
/// What repaints the canvas is the canvas itself — which is what `reabrir_no_canvas` does on
/// the last line, and is the shell's standard.
#[test]
fn preview_reaches_canvas_and_canvas_repaints() {
    let braco = trecho(
        "DesignerMode::ToggleLive) = efeito.designer",
        "DesignerMode::ToggleLock)",
    );
    assert!(
        braco.contains("c.runtime_mut().set_mode(mode)"),
        "`Preview` stopped notifying the canvas runtime — ruler, grid, and \
         handles stay on screen and the form never becomes an application"
    );
    assert!(
        braco.contains("canvas.redraw(cx);"),
        "the `Preview` arm mutated the canvas and did not request a frame ON IT. Measured \
         on screen: with `self.ui.redraw(cx)` the mode became `Vivo` on the \
         right pointer and the drawing remained the one from before the click"
    );
    // Contrast: without this, a `trecho` that sliced empty would pass both.
    assert!(
        braco.contains("self.vivo=!self.vivo;"),
        "the `Preview` arm no longer toggles — this test stopped reading the \
         right place"
    );
}

/// **Wire** — the gesture loop REQUESTS A FRAME from the canvas.
///
/// # The symptom, in the owner's words
///
/// > *"don't know why, but for Rust the HANDLES: you click and it takes forever
/// > for it to appear on the clicked component"*
///
/// Measured with a screenshot taken right after the click: the Inspector already showed
/// `btn_ok · Button` with the eleven properties, the status bar said
/// `btn_ok selected`, the AI panel said `btn_ok` — **and the canvas had no
/// handle at all**. The chrome repainted because each `set_text` dirties its widget;
/// the canvas, which nobody touched, kept the previous frame and only repainted
/// when the next gesture woke it up. Hence the "forever": it lasted until the
/// next mouse movement.
///
/// It is the SAME defect as the `Preview` arm, and that is why the two tests are
/// neighbors: `self.ui.redraw(cx)` marks the root and **does not descend to the canvas**.
#[test]
fn gesture_loop_requests_frame_from_canvas() {
    let corpo = method_body("bombear");
    assert!(
        corpo.contains("rt.set_selection(d.selection());"),
        "the gesture loop stopped delivering the selection to runtime — this test \
         stopped reading the right place"
    );
    assert!(
        corpo.contains("canvas.redraw(cx);"),
        "the gesture loop changed selection on canvas and did not request a frame ON IT. The \
         Inspector and bar update immediately, and the handle only appears on the \
         next gesture — measured in screenshot"
    );
}

/// **Wire** — the chrome INSIDE panels is rewritten after `load_state`.
///
/// # The defect, and it was invisible to the suite
///
/// The `Properties | Events` strip of the Object Inspector came out **empty**: the strip
/// existed, both buttons responded to clicks — two clicks on a component
/// really led to `Events` — and neither had text. The `X` of each
/// dockable panel came out without a glyph for the same reason.
///
/// The cause: both were written in `montar_cromo`, and the last line of that
/// method is `load_arrangement`, which does `load_state` — and `Dock` **recreates the
/// widget of each item**. Everything written before was lost.
///
/// And nothing complained: `set_text` on a `WidgetRef` that dock already discarded is not
/// an error. It is the same family as `DesignCanvas` drawing an empty document, which
/// the doc of `repovoar_paineis` already recounts — and that is why the fix is
/// to enter that function, rather than writing another loose line.
#[test]
fn panel_chrome_is_rewritten_after_layout() {
    let repovoar = method_body("repovoar_paineis");
    assert!(
        repovoar.contains("self.write_panel_chrome(cx);"),
        "`repovoar_paineis` stopped rewriting chrome inside \
         panels. The `Properties | Events` strip comes out EMPTY again and the `X` of \
         each panel comes out without a glyph, with no error at all"
    );
    // And it is rewritten AFTER arrangement, not before: written before,
    // `load_state` clears it — which is exactly the defect this test locks down.
    let montar = method_body("montar_cromo");
    let arranjo = montar
        .find("self.load_arrangement(cx);")
        .expect("`montar_cromo` stopped loading the saved arrangement");
    let cromo = montar
        .find("self.write_panel_chrome(cx);")
        .expect("`montar_cromo` stopped writing panel chrome");
    assert!(
        cromo > arranjo,
        "panel chrome was written BEFORE `load_arrangement` again. \
         `load_state` recreates the widget of each dock item, so text is \
         lost — and `set_text` on a discarded widget is not an error"
    );
}

/// **Wire** — the Makepad caption bar is removed, and OUR buttons respond.
///
/// # The two halves, and one without the other is worse than none
///
/// The owner asked to remove the duplicate bar *"several times"*: there are two title bars
/// stacked, the framework's with the hardcoded label `"Makepad"` and
/// ours right below it, which already has title, menus, and the same three buttons.
///
/// Except the three that **worked** were the framework's
/// (`makepad-widgets-1.0.0/src/window.rs:426-439`, which call `WindowHandle`
/// in a private field). Ours were just drawings: three `Button`s with their own
/// shader and nobody listening. Removing the bar without wiring ours would leave the
/// window unable to minimize, maximize, or close.
///
/// That is why both things are checked together here: whoever deletes one will
/// trip over the other.
#[test]
fn framework_bar_removed_and_our_window_buttons_respond() {
    let apply = method_body("after_apply");
    assert!(
        apply.contains("self.ui.view(id!(caption_bar)).set_visible(cx,false);"),
        "the Makepad caption bar returned. There are TWO title bars \
         stacked, and the top one eats 27 pixels of the footer"
    );
    assert!(
        apply.contains("self.ui.view(id!(windows_buttons)).set_visible(cx,false);"),
        "the framework window buttons returned. They are bound by their OWN \
         call (`window.rs:173`), so hiding only the parent does not take \
         them along"
    );

    // And our three are wired. `corpo_do_metodo` does not work here: they live
    // in `handle_actions`, which is long — the search is across the entire file,
    // compacted.
    let casca = compacto();
    for (botao, op) in [
        ("btn_min", "CxOsOp::MinimizeWindow(janela)"),
        ("btn_close", "CxOsOp::CloseWindow(janela)"),
    ] {
        assert!(
            casca.contains(&format!(
                "self.ui.button(id!(top_titlebar.{botao})).clicked(actions)"
            )),
            "no one listens to `{botao}`. With the framework bar gone, it is \
             the ONLY port for this window command — and a painted button \
             that does nothing is the family `Preview` just paid for"
        );
        assert!(casca.contains(op), "`{botao}` stopped sending `{op}`");
    }
    // THE maximize/restore PAIR are TWO buttons listened to, one at a time.
    for botao in ["btn_max", "btn_restore"] {
        assert!(
            casca.contains(&format!(
                "self.ui.button(id!(top_titlebar.{botao})).clicked(actions)"
            )),
            "no one listens to `{botao}`"
        );
    }
    assert!(
        casca.contains("self.toggle_maximized(cx);"),
        "the maximize/restore pair stopped going through `toggle_maximized`"
    );

    // AND MAXIMIZING MEASURES THE WORK AREA, instead of sending the framework's.
    //
    // This is the gate the owner demanded THREE times: *"the maximized IDE (…) the
    // footer is clipped"*, *"maximizing last time still clipped the
    // footer"*, and *"DO NOT MAXIMIZE ONLY THE OUTER PART"*. The framework's
    // `MaximizeWindow` falls back to `ShowWindow` (`win32_window.rs:810-815`), and Windows
    // maximizes a `WS_POPUP` to the ENTIRE MONITOR — the taskbar covering
    // the IDE's status bar.
    //
    // Going back to sending only it is one line, and it doesn't break the build: it breaks the
    // screen, which is the defect family this entire file exists to
    // catch.
    assert!(
        casca.contains("crawrad_os::monitor_at(centre_in_physical_pixels(&geom))"),
        "maximizing stopped asking the system where the taskbar \
         ends. Without this question no one above `crawrad-os` knows — Makepad's \
         `WindowGeom` does not carry the usable area"
    );
    // AND WHAT PLACES THE WINDOW IS THE ADAPTER, and not the two `CxOsOp`s.
    //
    // The owner's THIRD complaint was this: with `CxOsOp` the frame moved and the
    // DRAWING stayed at the old size. The measurement is in
    // `crawrad_os::place_main_window` — `Win32App::do_callback` REMOVES the callback
    // before calling (`win32_app.rs:294-304`), and `CxOsOp`s are drained from
    // inside it, so the synchronous `WM_SIZE` from `MoveWindow` drops into a void.
    //
    // Going back to `CxOsOp` compiles, passes every unit test, and brings back the
    // black bar on screen.
    assert!(
        casca.contains("crawrad_os::place_main_window(area.work)"),
        "maximizing stopped placing the window via the adapter. With `CxOsOp` \
         the frame moves and drawing stays behind — that was the complaint `DO NOT \
         MAXIMIZE ONLY THE OUTER PART`"
    );
    assert!(
        casca.contains("crawrad_os::place_main_window(anterior)"),
        "restore stopped returning the window to the saved rectangle"
    );
    // THE CALL, not the NAME: `compacto()` removes whitespace and KEEPS
    // comments, and the header of `maximize.rs` cites both ops in full
    // to explain why they were removed. Checking the name would fail its own
    // explanation.
    for op in [
        "push_unique_platform_op(CxOsOp::RepositionWindow",
        "push_unique_platform_op(CxOsOp::ResizeWindow",
    ] {
        assert!(
            !casca.contains(op),
            "`{op}` returned. It moves the frame and DOES NOT notify the framework \
             renderer — the drawing stays at the old size"
        );
    }
    // The pair goes AND COMES BACK. The framework's button only went (`window.rs:430-435` checks
    // `is_fullscreen`, which `MaximizeWindow` does not set).
    assert!(
        casca.contains("CxOsOp::RestoreWindow(window)"),
        "the maximize button stopped restoring. It goes back to being the framework \
         one: maximizes and never restores"
    );
    // AND THE WINDOW FITS THE SCREEN IT OPENS ON — the other half of the same defect,
    // which the owner measured at 1044x788 in a work area of 1024x720.
    assert!(
        casca.contains("self.fit_on_screen_once(cx);"),
        "no one calls `fit_on_screen_once`. The DSL requests `inner_size` of \
         1440x900 and no monitor is required to have that"
    );
}

/// **THE LINK** — the gesture leading from drawing to code (ADR 0015).
///
/// # What this gate locks down
///
/// The link is a chain of four links, and **any of them silently breaking
/// returns the shell to its previous state**: a double click that does nothing. None
/// of them breaks the build when removed, and none of them shows up in the suite — it is the
/// same reason this file was created.
///
/// 1. the double tap is RECOGNIZED (and by the framework's rule, not by our own
///    constant);
/// 2. it finds the LINE — using the rectangles that drawing measured;
/// 3. the handler is bound via the catalog ACTION, never via a `Transaction`
///    written here;
/// 4. and the double click on the FORM reaches the same port, instead of being
///    discarded as it was.
#[test]
fn double_click_link_has_wire_on_both_sides() {
    let casca = compacto();

    // 1. The double tap, with the framework's ruler.
    assert!(
        casca.contains("letduplo=crawrad_runtime_makepad::canvas::is_double_tap(self.ultimo_toque,md.abs,md.time,);"),
        "no one counts double tap on `MouseDown`. Without this, double click \
         on the `Events` tab is a single click, and nothing happens"
    );
    assert!(
        include_str!("../../../../crawrad-runtime-makepad/src/canvas/brush.rs").contains(
            "use makepad_widgets::makepad_platform::event::{TAP_COUNT_DISTANCE, TAP_COUNT_TIME};"
        ),
        "double click started having its OWN ruler. The constants are from the \
         framework on purpose — hardcoded here, the IDE would have two ideas of \
         what a double click is"
    );

    // 2. The line, via measured rectangles.
    assert!(
        casca.contains("eventos.push((i,item.area().rect(cx)));"),
        "drawing stopped storing the rectangle of event rows. Raw \
         `MouseDown` has no `PortalList` to ask which row was \
         under the pointer"
    );
    assert!(
        casca.contains(
            "ifduplo&&letSome(i)=self.event_line_at(md.abs){self.abrir_handler_da_linha(cx,i);}"
        ),
        "double click on the `Events` tab lost its wire"
    );

    // 3. The ACTION is what binds it, and the panel row shows that it bound.
    let ligar = method_body("bind_event");
    assert!(
        ligar.contains(r#"self.execute_with(cx,"event.bind",args);"#),
        "handler binding stopped going through the catalog action"
    );
    assert!(
        ligar.contains("self.redraw_panel_list(cx,Panel::Inspector,"),
        "the `Events` tab row goes back to being BLANK after binding: \
         `self.ui.redraw` does not reach a `PortalList` inside a dock item"
    );

    // 4. And the double click on the form, which was discarded.
    assert!(
        casca.contains("ifletRuntimeEvent::PointerDblClick{x,y}=ev{"),
        "double click on the form was discarded again — it was a \
         plain `continue`, and it is the reason the editor exists (ADR 0015)"
    );
    assert!(
        casca.contains("crawrad_codegen::elo::elo_do_duplo_clique(n,desc)"),
        "the shell started deciding on its own which event double click opens. The \
         rule is one, and both shells read the SAME one"
    );
}

/// **THE ITEMS EDITOR** — the `…` dialog for a collection.
///
/// # What this gate locks down
///
/// That all five buttons keep calling the five functions of the MODEL. Replacing
/// `items_editor::new_child(e)` with `{}` leaves `New SubItem` mute, and nothing
/// in the build complains — it is the same defect family that caused this file
/// to be born. The model has its own tests (`test/items_editor/test.rs`, on the
/// egui side); what is missing is proving that it is CONNECTED on this side.
///
/// And that the rule keeps coming from there: a `new_child` written in this shell
/// would be a second idea of what a sub-item is, and it already diverged once.
#[test]
fn five_items_editor_commands_call_model() {
    let casca = compacto();
    // Each button carries its command in `ITEMS_BOTOES`; `items_editor::run`
    // maps the command to the model (its own test is in crawrad-actions).
    for command in [
        "ItemsCommand::NewItem)",
        "ItemsCommand::NewSubItem,",
        "ItemsCommand::Delete)",
        "ItemsCommand::MoveUp)",
        "ItemsCommand::MoveDown)",
        "items_editor::run(e,cmd);",
    ] {
        assert!(
            casca.contains(command),
            "`{command}` disappeared from Items Editor. The button remains on screen and \
             does nothing anymore — and the build does not complain"
        );
    }
    // And what OPENS the dialog is the line's EDITOR, not a `match` by key:
    // the question lives in the model (`inspector_model::window_for`).
    assert!(
        casca.contains("inspector::window_for(") && !casca.contains("p.editor.window_items()"),
        "the shell started deciding on its own which property opens the Items \
         Editor. Asking `is the key `items`?` here is the shell knowing about \
         collections again — and that was how a `ListBox` opened the TREE \
         editor once"
    );
    assert!(
        casca.contains("inspector::Window::Items=>self.open_items(cx,node),"),
        "the `…` of a collection stopped opening the Items Editor"
    );
    // The value leaves through the USUAL PORT.
    let gravar = method_body("gravar_items");
    assert!(
        gravar.contains("self.write_property(cx,alvos,ITEMS_KEY,valor);"),
        "the Items Editor started writing to the document outside \
         `component.set_property`. No `Transaction` is born in this shell"
    );
}

/// The dialog TITLE comes from the CLASS, via a function.
///
/// A `"TreeView Items Editor"` literal here would be the bug that the egui shell
/// already paid for: a two-arm `if` choosing between two literals for THREE
/// components, with `ListBox` getting the name of `TreeView`.
#[test]
fn items_editor_title_comes_from_class() {
    let casca = compacto();
    assert!(
        casca.contains("items_editor::editor_title(&estado.node_class)"),
        "the Items Editor title stopped coming from the component class"
    );
    assert!(
        !CASCA.contains("Items Editor\""),
        "there is a hand-written Items Editor title in this shell. It belongs to \
         `editor_title`, and the reason is in its doc"
    );
}

/// **Wire** — the editor tab strip draws, listens, and closes.
///
/// # What this gate locks down
///
/// The owner requested on 2026-09-17, looking at a modern editor: *"we need tabs for
/// each open .RS in the editor"*. The model has its own gate and is pure
/// (`crate::view_models::editor_tabs`) — sixteen tests on name disambiguation,
/// closing, and the neighbor inheriting the foreground.
///
/// A perfect model that no one calls passes all of them. This one checks the
/// CALL, which is what this entire file exists to do:
///
/// 1. the list is POPULATED, with the labels disambiguated by the model;
/// 2. the highlight indicates which tab is in front;
/// 3. clicking the label OPENS and clicking `X` CLOSES — and `X`, which lives
///    inside the tab, is tested FIRST, otherwise closing would also open;
/// 4. `F12` and the link's double click enter through the SAME port, otherwise the same
///    unit opens in two tabs.
#[test]
fn a_tira_de_abas_do_editor_desenha_escuta_e_fecha() {
    let c = compacto();

    // 1. The list is populated, and labels come from the model.
    assert!(
        c.contains("self.abas.labels().iter().zip(self.abas.files())"),
        "the strip stopped using model labels. Writing the file name \
         here would lose disambiguation — two identical `inspector.rs` side by \
         side, and no way to tell which is which"
    );
    assert!(
        c.contains("rotulo.one_line()"),
        "the tab stopped using the single line assembled by the model"
    );
    // AND THE DIRTY FILE BADGE, which is what separates *haven't saved yet* from
    // *saved*. Without it the editor saves silently or doesn't save silently, and
    // the user has no way of knowing which state they are in.
    assert!(
        c.contains("self.host.buffer(arquivo).is_some()")
            && c.contains("draw_bg:{dirty:(dirty)}draw_text:{dirty:(dirty)}"),
        "the tab stopped marking the file with unsaved changes"
    );
    assert!(
        c.contains("draw_rows!{cx,proximo,abas_do_editor,rotulos_das_abas,")
            && c.contains("list.set_item_range($cx,0,$rows.len());"),
        "the strip stopped reporting how many tabs it has, and a `PortalList` without \
         a range draws ZERO items silently"
    );
    assert!(
        c.contains("item.button(id!(tab_label)).set_text(cx,rotulo);"),
        "the tab stopped writing the file name"
    );
    assert!(
        c.contains("set_text(cx,icon::Chrome::Close.glyph());"),
        "the tab's `X` stopped coming from the icon catalog (ADR 0026)"
    );

    // 2. The highlight.
    assert!(
        c.contains("letmarca=f64::from(u8::from(aba_a_frente==Some(i)));"),
        "the strip stopped reporting which tab is in front. All lit, or \
         none — and whoever reads the screen does not know which file they are seeing"
    );

    // 3. Both buttons, and the ORDER between them.
    let fechar = c
        .find("item.button(id!(tab_close)).clicked(actions)")
        .expect("no one listens to the tab's `X`");
    let abrir = c
        .find("item.button(id!(tab_label)).clicked(actions)")
        .expect("no one listens to the tab label");
    assert!(
        fechar < abrir,
        "`X` stopped being tested before the label. It lives INSIDE the tab, \
         so a click on it that also counts as a click on the tab would open the \
         file that was just closed"
    );

    // 4. The single port.
    assert!(
        method_body("abrir_no_editor").contains("self.switch_tab(cx,|a|_=a.open(arquivo));"),
        "the file-opening port stopped adding the tab"
    );
    // The tab's kept caret waits for the next frame; an explicit landing
    // (Go to Definition, a reference, a breakpoint) must win over it.
    assert!(
        method_body("pousar_cursor")
            .contains("self.pouso_pendente=self.pouso_pendente.and(Some(*pouso));"),
        "an explicit landing no longer replaces the kept caret pending for the next frame"
    );
    assert!(
        method_body("toggle_view").contains("self.abrir_no_editor(cx,source::unit_path(&form));"),
        "`F12` stopped opening the unit THROUGH THE PORT. With a second port, \
         the same unit opens in two tabs"
    );
    assert!(
        c.contains("self.abrir_no_editor(cx,unidade.clone());"),
        "the double click link stopped opening the unit through the single port \
         (ADR 0015)"
    );
    // And clicking in Explorer arrives via the CATALOG, like every row in the panel.
    assert!(
        c.contains(
            "Some(crawrad_actions::FileEffect::OpenInTab(rel))=>self.abrir_aba_relativa(cx,&rel),"
        ),
        "the `OpenInTab` effect lost its arm in this shell. The Explorer row \
         goes back to being a click that only informs"
    );
}

/// **Wire** — UI zoom reaches the screen, and the pointer follows.
///
/// # What this gate locks down
///
/// The owner demanded twice: *"zoom is not in the designer area only, it is across
/// THE ENTIRE IDE, WITHOUT EXCEPTION"* and *"NEVER CHANGED, ALWAYS BEEN THIS, LOOK AT EGUI,
/// it was always like that there"*.
///
/// The scale is pure and has its own gate (`ide_app/ui_zoom.rs`). A perfect
/// scale that no one calls passes all of them — this checks the three
/// calls that make it happen, and the third is the one no one
/// guesses.
#[test]
fn ui_zoom_reaches_screen_with_pointer() {
    let c = compacto();

    // 1. The effect has an arm.
    assert!(
        c.contains("ifletSome(pedido)=efeito.ui_zoom{self.aplicar_zoom_da_ide(cx,pedido);}"),
        "`ui_zoom` of `Efeito` lost its arm. The command falls to the \
         ground again — the defect family that the `Efeito` coverage gate \
         exists to prevent"
    );

    // 2. The lever is the framework's `dpi_override`, and it is WRITTEN AND NUDGED.
    //
    // Writing alone doesn't change a pixel: the framework only applies the field when
    // a geometry change ARRIVES (`os/windows/windows.rs:115-117`).
    assert!(
        c.contains("cx.windows[janela].dpi_override=Some(base*fator);"),
        "UI zoom stopped touching `dpi_override`, which is the only \
         lever scaling the entire IDE in this framework"
    );
    assert!(
        c.contains("crawrad_os::nudge_resize();"),
        "UI zoom stopped nudging the window. `dpi_override` remains \
         written and the screen does not change — and nothing complains"
    );

    // 3. THE POINTER IS SCALED, and this is the half the framework does not do.
    //
    // Measured: `macos.rs` remaps the position on every pointer event
    // (`:356-382`) and `os/windows/windows.rs` NEVER does it. Without this
    // line the IDE draws at 150% and every click lands where the 100% drawing
    // was — and this compiles, passes the whole suite, and only shows up with mouse use.
    let escala = c
        .find("letescalado=self.com_ponteiro_na_escala(cx,event);")
        .expect(
            "the pointer stopped scaling with the drawing. With an enlarged \
             UI, every click misses the target",
        );
    let usa = c
        .find("letevent=escalado.as_ref().unwrap_or(event);")
        .expect("the corrected event is computed and not used");
    assert!(
        escala < usa,
        "the corrected event is used before being computed"
    );
    // And it is the FIRST thing in `handle_event`: everything below measures position.
    let porta = c
        .find("fnhandle_event(&mutself,cx:&mutCx,event:&Event){")
        .expect("`handle_event` disappeared");
    assert!(
        escala > porta && escala - porta < 1200,
        "pointer correction stopped being the first thing in \
         `handle_event`. Whatever precedes it measures the wrong position"
    );
}

/// **Wire** — the `Design`/`Code` strip TURNS OFF the other tab, otherwise clicking dies.
///
/// # The defect, measured in framework source
///
/// The owner reported: *"Design Code, mouse click does not work"*.
///
/// Makepad's `RadioButton` only emits `Clicked` if it is in `active.off`
/// (`makepad-widgets-1.0.0/src/radio_button.rs:1361-1365`), and what turns off the
/// others in the set is `RadioButtonSet::selected` (`:1424-1429`) — which only
/// runs **when a click arrives**.
///
/// Turning one on in the writer and never turning the other off leaves both `on`
/// after the first view switch, and from then on **neither emits**:
/// `FingerUp` arrives, `animator_in_state(active.off)` is false, and the action is not
/// generated.
///
/// # Why this needs a gate
///
/// Because it disappears without noise. The widget draws, highlight is correct, the suite stays
/// green, and the strip becomes a static drawing — it is the same defect family as the painted `Preview` that
/// did nothing, and the reason this file exists.
#[test]
fn center_strip_disables_background_tab() {
    let c = compacto();
    assert!(
        c.contains("r.animator_play(cx,id!(active.off));"),
        "the `Design`/`Code` strip writer stopped TURNING OFF the tab not \
         in front. Both stay lit, and from then on neither emits \
         `Clicked` — the strip becomes a drawing, and nothing breaks"
    );
    // AND THE ORDER: turn off BEFORE turning on.
    //
    // Inverted, `select` activates the new one and `animator_play` right after
    // deactivates the one that was just activated when both are the same — the case of
    // `Alt+F12`, where the center switches mode without switching tabs.
    let desliga = c
        .find("r.animator_play(cx,id!(active.off));")
        .expect("already asserted above");
    let liga = c
        .find(".radio_button(acesa).select(cx,&mutScope::empty());")
        .expect("the writer stopped activating the foreground tab");
    assert!(
        desliga < liga,
        "the strip activates the tab before turning off the other. When both are the \
         same, turning off undoes activating"
    );
}

/// **Wire** — the EDITOR keyboard is not the designer keyboard.
///
/// # The two defects this gate locks down, and both were seen on screen
///
/// The owner, with the code view open: *"editor only enter works, arrow,
/// typing"*.
///
/// **1. `Enter` MUTATED a read-only file.** This is a framework flaw, and
/// has been measured: in `CodeEditor` every arm that mutates text checks
/// `if !self.read_only` — typing (`code_editor.rs:845`), deleting (`:874`),
/// `Tab` (`:872`), cutting (`:912`), pasting (`:922`). The `ReturnKey` arm
/// (`:861-869`) **does not check**, and calls `session.enter()` regardless.
/// The `CodeView` in this shell starts as `read_only: true` (`code_view.rs:22`), so
/// the status bar said `Read only` while the key split the line.
///
/// **2. Arrow keys nudged the component behind the code.** They are designer
/// gestures, and with the editor in front what the user requests is to move the CARET.
/// Without the guard both happened on the same keypress — and the half that is not
/// visible is the worst.
#[test]
fn editor_keyboard_is_distinct_from_designer() {
    let c = compacto();

    // 1. `Enter` is swallowed, and BEFORE `ui` sees the event.
    assert!(
        c.contains(
            "ifself.menu_keys(cx,event)||self.enter_no_editor_somente_leitura(cx,event){return;}"
        ),
        "`Enter` reached the read-only editor again. The framework does not \
         guard this arm, and the key SPLITS the line of a file the status bar \
         swears cannot be edited"
    );
    let engole = c
        .find("ifself.menu_keys(cx,event)||self.enter_no_editor_somente_leitura(cx,event){return;}")
        .expect("already asserted above");
    let ao_ui = c
        .find("self.ui.handle_event(cx,event,&mutScope::empty());")
        .expect("`ui` stopped receiving events");
    assert!(
        engole < ao_ui,
        "the `Enter` guard ended up AFTER `ui` sees the event — and the editor \
         has already split the line when it responds"
    );

    // And it checks for FOCUS, not for the view: with a dialog over the
    // code, `Enter` belongs to the dialog.
    assert!(
        method_body("editor_com_foco").contains("cx.has_key_focus(area)"),
        "the keyboard guard stopped checking for focus. It starts killing \
         `Enter` for every dialog opened over code"
    );
    // **AND ONLY WHILE THE VIEW IS READ-ONLY.** Since the editor became editable
    // (2026-09-17), swallowing `Enter` on an editable file would be the opposite
    // defect: a line is typed and the next cannot be started.
    assert!(
        method_body("enter_no_editor_somente_leitura").contains("self.editavel.is_some()"),
        "the `Enter` guard applied to editable files again, killing the \
         editor's `Enter`"
    );

    // 2. The designer gesture does not pass through the editor — and the catalog ACTION
    // keeps passing through, otherwise `Ctrl+S` and `F12` would die as well.
    assert!(
        c.contains("letfront=teclado::Front::of(c.is_design(),c.is_text());"),
        "the bridge stopped asking if the editor is in front"
    );
    assert!(
        c.contains("ifteclado::blocked(&comando,teclado_do_texto,front){continue;}"),
        "the arrow key pushed the component with code in front again — or, \
         worse, the catalog action stopped passing and `Ctrl+S` died with the \
         editor open"
    );

    // 3. **AND THE EDITING KEYBOARD BELONGS TO THE EDITOR when it has focus.**
    //
    // Without this, `Ctrl+Z` with the caret in a file undoes the last FORM
    // transaction and `Del` deletes the selected component — behind a screen
    // nobody is looking at.
    assert!(
        c.matches("teclado::blocked(").count() == 1 && !c.contains("teclado::not_here("),
        "`Ctrl+Z` and `Del` drove the designer again with the caret inside \
         a file"
    );

    // 4. Keys the framework left DEAD are handled, and before
    // `ui` — `CodeEditor` matches these chords in an empty arm.
    let nossas = c
        .find("||self.tecla_do_editor_de_texto(cx,event){return;}")
        .expect(
            "`Ctrl+Left/Right`, `Ctrl+Home/End` and `Ctrl+Y` died again \
             in the framework's empty arm",
        );
    assert!(
        nossas < ao_ui,
        "editor navigation ended up AFTER `ui` sees the event, and the framework's \
         empty arm has already consumed the key when it responds"
    );
}

/// **Wire** — the structure tree REPAINTS when selection changes.
///
/// # The defect this gate locks down, and it looked like something else
///
/// The owner, with the checkmark one line below the clicked one: *"clicked the top one,
/// selected the bottom one... it's a zoom side effect, even at 100%"*.
///
/// It wasn't zoom, and the proof was on his screen: the status bar said
/// `edt_nome_ selected` — the RIGHT node, the one clicked. The model was
/// up to date; the tree DRAWING was frozen on the previous checkmark.
///
/// The cause has been recorded in this repo since Package Manager, in the doc of
/// `redraw_panel_list`: `self.ui.redraw(cx)` **does not reach** a
/// `PortalList` living inside a dock item. `select_node` only called
/// that.
///
/// # Why the gate checks `sincronizar`, and not `select_node`
///
/// Because selection changes through many ports — clicking on the canvas, `Del`,
/// `Ctrl+Z`, `Ctrl+A`, an action catalog effect. A line in each one
/// would be one to forget, and the forgotten one would produce the outdated checkmark
/// on a single port — which is the hardest form to notice.
#[test]
fn tree_repaints_when_selection_changes() {
    let sync = method_body("sincronizar");
    assert!(
        sync.contains("ifsel!=self.selecao_na_arvore{"),
        "`sincronizar` stopped comparing selection with what the tree \
         drew. Without comparison, either it never repaints — leaving the checkmark \
         on the previous row —, or repaints every frame and the window never sleeps"
    );
    assert!(
        sync.contains(
            "self.redraw_panel_list(cx,Panel::Components,id!(structure_tree.structure_list),)"
        ) || sync.contains(
            "self.redraw_panel_list(cx,Panel::Components,id!(structure_tree.structure_list))"
        ),
        "the tree went back to depending on `self.ui.redraw`, which DOES NOT reach a \
         `PortalList` inside a dock item — measured here, and documented \
         in `redraw_panel_list`"
    );
    assert!(
        sync.contains("self.selecao_na_arvore=sel.clone();"),
        "the comparison never stores what was drawn, so it is true \
         forever and the list redraws on every frame"
    );
}

/// **Wire** — the canvas REPAINTS when a node is selected or cleared.
///
/// Without `canvas.redraw(cx)`, `self.ui.redraw(cx)` does not descend to the canvas
/// living in the dock: clicking a node in Structure updates the checkmark in the tree but leaves
/// the designer frozen with the old handles.
#[test]
fn canvas_repaints_on_node_select_or_clear() {
    let sel = method_body("select_node");
    assert!(
        sel.contains("canvas.redraw(cx);"),
        "`select_node` does not call `canvas.redraw(cx)`. Without this, clicking on \
         a component in the tree (or root) updates the checkmark in the tree but leaves \
         the designer frozen on the previous frame without updating handles"
    );
    let clr = method_body("clear_selection");
    assert!(
        clr.contains("canvas.redraw(cx);"),
        "`clear_selection` does not call `canvas.redraw(cx)`"
    );
}

/// **Wire** — the editor EDITS, holds what was typed, and saves the file.
///
/// # What this gate locks down
///
/// The owner, on 2026-09-17: *"NOTHING works, Ctrl+Arrows, Enter, Typing,
/// as if readonly, where is the working Editor?"*. He was reading it
/// correctly — the footer explicitly said `Read only`, because `CodeView` is created as
/// `read_only: true` (`code_view.rs:22`).
///
/// Enabling typing is one line. The other three are what separates an editor from
/// a machine for losing work, and each is an assertion here:
///
/// 1. read-only mode is HANDED to the framework, and changes with the view;
/// 2. what was typed WINS over disk when the tab returns;
/// 3. text is READ BACK every frame — `CodeView` notifies no one;
/// 4. `Ctrl+S` saves the FILE, and stops there.
#[test]
fn editor_edits_holds_and_saves() {
    let c = compacto();

    // 1. The mode is handed over, not written once in the DSL.
    assert!(
        method_body("aplicar_somente_leitura").contains("read_only:(so_leitura)"),
        "read-only mode stopped being handed over to `CodeEditor`. Stuck \
         in the DSL, it applies to the form source AND the unit — and the \
         unit is the user's file"
    );
    assert!(
        method_body("aplicar_somente_leitura").contains("self.editavel.is_none()"),
        "the mode stopped coming from the `pode_editar` rule, making it a literal \
         that does not change with the view"
    );
    assert!(
        c.contains("self.aplicar_somente_leitura(cx);"),
        "the mode is computed and never handed over: the view stays with what the DSL \
         declared at the start"
    );

    // 2. The buffer wins over disk.
    assert!(
        c.contains("self.editavel.as_ref().and_then(|p|self.host.buffer(p))"),
        "switching tabs went back to rereading disk over what was typed. `CodeView`'s \
         `set_text` DISCARDS the session (`code_view.rs:82`): switching to \
         `Design` and back would erase work, without asking or warning"
    );

    // 3. Text is read back on every frame.
    assert!(
        c.contains("self.colher_do_editor(cx);"),
        "no one reads back what was typed. `CodeView` SWALLOWS editor \
         `TextDidChange` (`code_view.rs:67`), so from outside editing \
         is invisible — and what is not read is not saved"
    );

    // 4. `Ctrl+S` saves the file, and STOPS.
    let salvar = method_body("salvar");
    assert!(
        salvar.contains("ifself.save_editor_file(cx){return;}"),
        "`Ctrl+S` on an edited file went back to saving the FORM. The \
         `return` is half the fix: without it `Generate` runs behind it, \
         rewrites `.designer.rs`, and prunes handlers (ADR 0025) because of a \
         typed line"
    );
    // 5. AND `Generate` SAVES FIRST — the pre-pass READS the unit from disk, and the
    // generator overwrites it. With a line typed and unsaved, the
    // user's work would return erased.
    let gerar = method_body("gerar_projeto");
    let gravar_antes = method_body("save_before_build");
    assert!(
        gerar.contains("if!self.save_before_build(cx){return;}")
            && gravar_antes
                .contains("||(self.save_editor_file(cx)&&self.editor_sujo()){returnfalse;}")
            && gravar_antes.contains("(all&&!self.save_all(cx))"),
        "`Generate` ran over typed and unsaved changes again. Both \
         halves matter: save before, and DO NOT generate when \
         saving failed"
    );

    let gravar = method_body("save_editor_file");
    assert!(
        gravar.contains("self.encoding_na_vista.to_bytes(&texto)"),
        "saving went back to writing the raw String. The editor always stores \
         `\\n` and never the mark: a CRLF file with BOM would return to disk as LF and \
         without the mark, and a one-word edit would show up in review with all \
         lines replaced"
    );
}

/// Generate writes the `.crawform` before the code, so a new
/// project's form is never only in memory when the generated code is on disk.
#[test]
fn generate_saves_the_form_before_writing_code() {
    assert!(
        method_body("save_before_build").contains("self.save_form_file()"),
        "Generate stopped saving the form: the code lands on disk and the `.crawform` does not"
    );
    let body = method_body("gerar_projeto");
    let saved_at = body.find("self.save_before_build(cx)").expect(
        "Generate stopped saving the form: the code lands on disk and the `.crawform` does not",
    );
    let member_at = body
        .find("self.membro()")
        .expect("`self.member_at()` left Generate");
    assert!(
        saved_at < member_at,
        "the form must be saved BEFORE the generation runs"
    );
    assert!(
        method_body("save_before_build")
            .contains("is_form(p)&&(d.revision()!=self.rev_salva||!p.exists())"),
        "only a changed or never-saved `.crawform` is written — never `main.rs`"
    );
}

/// The MCP `run.start` generates first, like the IDE's Run: the
/// form IS the program, and the template demo must never run in its place.
#[test]
fn mcp_run_start_generates_before_cargo_run() {
    let source: String = include_str!("../../ide_app/mcp_host.rs")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let arm = source
        .find("ifname==\"run.start\"{")
        .map(|i| &source[i..])
        .expect("`run.start` left the MCP dispatch");
    let generated_at = arm
        .find("app.gerar_projeto(cx);")
        .expect("run.start stopped generating");
    // The extraction inlined `tool_run_start` into the arm, so the guard now looks for the
    // call it became. What it protects is unchanged: generating must come BEFORE running,
    // or `cargo run` builds the previous version of the form.
    let run_at = arm
        .find("self.running_process.start(")
        .expect("run.start stopped running");
    assert!(
        generated_at < run_at,
        "run.start must generate BEFORE `cargo run`"
    );
    assert!(
        arm[..run_at].contains("crawrad_project::generator::refused(&said)"),
        "a refused generation must not fall through to `cargo run`"
    );
}

/// Debugger, chat AI and audio pumps run on Makepad's `Event::Signal`
/// instead of polling on fast timers. No 0.05 / 0.08 / 0.25 poll timeouts remain.
#[test]
fn signal_wakes_debugger_chat_and_audio_without_polling_timers() {
    let shell = compacto();
    assert!(
        shell.contains("ifmatches!(event,Event::Signal){self.bombear_pacotes(cx);self.bombear_criacao(cx);self.bombear_build(cx);self.bombear_mcp(cx);self.pump_environment(cx);self.pump_disk_changes(cx);self.pump_debugging(cx);self.bombear_audio(cx);self.bombear_ia(cx);self.pump_doctest(cx);self.pump_dataset_columns(cx);}"),
        "debugger, audio and chat pumps must run inside Event::Signal"
    );

    let chat_source = include_str!("../../ide_app/chat.rs");
    let debug_source = include_str!("../../ide_app/debug.rs");
    let options_source = include_str!("../../ide_app/dialogs/options.rs");

    assert!(
        !chat_source.contains("start_timeout(0.05)"),
        "fast polling timer start_timeout(0.05) must not remain in chat.rs"
    );
    assert!(
        !chat_source.contains("start_timeout(0.08)"),
        "fast polling timer start_timeout(0.08) must not remain in chat.rs"
    );
    assert!(
        !options_source.contains("start_timeout(0.05)"),
        "fast polling timer start_timeout(0.05) must not remain in options.rs"
    );
    assert!(
        !debug_source.contains("start_timeout(Self::PULSO_DA_DEPURACAO)"),
        "debugger poll timer must not remain in debug.rs"
    );
    assert!(
        !shell.contains("pulso_da_depuracao"),
        "pulso_da_depuracao field must be removed from the shell"
    );
    assert!(
        !shell.contains("pulso_da_ia"),
        "pulso_da_ia field must be removed from the shell"
    );
    assert!(
        !shell.contains("pulso_do_audio"),
        "pulso_do_audio field must be removed from the shell"
    );
}

#[test]
fn chat_status_label_is_not_hardcoded() {
    let dsl = include_str!("../../ide_app/panels/chat.rs");
    let ready_status_decl = dsl
        .find("lbl_ready_status")
        .map(|idx| &dsl[idx..idx + 150])
        .expect("lbl_ready_status not found in panels/chat.rs");
    assert!(
        !ready_status_decl.contains("Connected"),
        "DSL default for lbl_ready_status must no longer contain 'Connected'"
    );
    assert!(
        ready_status_decl.contains("⚪ Checking the AI agent…"),
        "DSL default for lbl_ready_status should be '⚪ Checking the AI agent…'"
    );

    let chat_code = include_str!("../../ide_app/chat.rs");
    // The texts (lbl_ready_status included) come from crawrad-agent's hero_labels; the IDE only sets them.
    assert!(
        chat_code.contains("chat_layout::hero_labels(msgs, agent, connected)"),
        "refresh code must set the hero labels from hero_labels (real agent state)"
    );
}
