//! The gate for the Makepad shell KEYBOARD BRIDGE.
//!
//! # The defect this file exists to lock down
//!
//! The owner used the Makepad IDE, added components via the Toolbox, and **could
//! not delete any**. The `edit.delete` action was fully in the catalog
//! (`crates/crawrad-actions/src/lib.rs:860`), with `atalho: Some("Del")`, and
//! `grep -c KeyDown src/ide_app.rs` returned **0**: nothing listened to keys.
//! Being able to add and not remove is not a gap, it is a trap.
//!
//! # The two test families here, and what each proves
//!
//! 1. **Translation** — pure functions of [`crate::ide_app::teclado`]: catalog
//!    chord in, command out. The coverage proof is DERIVED from
//!    `crawrad_actions::catalogo()`, not from a list written here: if tomorrow
//!    a 19th action with a shortcut is created, this file fails if it is not
//!    reachable.
//! 2. **Wiring** — `src/ide_app.rs` read as TEXT, via the technique from
//!    `test/ide_app/fios.rs`. A perfect translation called by no one passes
//!    all of family 1, and that was exactly the state of the shell before this
//!    round.
//!
//! What family 2 does **not** prove remains what `fios.rs` already states:
//! that the call is written, not that it ran, let alone that the screen
//! was correct. That is the ceiling of text testing without a real window.

use super::teclado::{self, Comando};
use makepad_widgets::{KeyCode, KeyModifiers};

// ---------------------------------------------------------------------
// FAMILY 1 — translation
// ---------------------------------------------------------------------

/// No modifiers.
fn nada() -> KeyModifiers {
    KeyModifiers::default()
}

/// The actual modifiers described by a [`crawrad_runtime_makepad::keys::Chord`], **on this
/// platform**.
///
/// `primary` becomes `control` outside macOS and `logo` within it, which is the
/// same calculation that Makepad's `KeyModifiers::is_primary` performs in the
/// opposite direction (`makepad-platform-1.0.0/src/event/finger.rs:49-56`). Constructing the
/// event using the target's `cfg` is what makes this test valid on the three platforms
/// of ADR 0007 without a second table.
fn modificadores(a: crawrad_runtime_makepad::keys::Chord) -> KeyModifiers {
    KeyModifiers {
        shift: a.shift,
        alt: a.alt,
        control: a.primary && !cfg!(target_vendor = "apple"),
        logo: a.primary && cfg!(target_vendor = "apple"),
    }
}

/// The platform's primary modifier: `Cmd` (logo) on Apple, `Ctrl` elsewhere. Shortcuts
/// written `Ctrl+...` follow it, so a test that presses one must press the right key.
const APPLE: bool = cfg!(target_vendor = "apple");

/// **With the cursor in a file, EDITING keys belong to the editor — and IDE keys
/// remain with the IDE.**
///
/// # The defect this gate locks down, and it causes lost work
///
/// The `handle_key_down` of this shell receives **every** `KeyDown`, without focus
/// filtering. While the code view was read-only that was ugly; since it
/// edits (2026-09-17) it is severe: `Ctrl+Z` undid the last transaction of the
/// FORM, `Del` deleted the selected component, `Ctrl+C` copied the
/// component. All behind a screen nobody is looking at.
///
/// # The two halves, and the second is as important as the first
///
/// Muting too much breaks the product on the other side: `Ctrl+S` with the editor open is
/// precisely what is pressed most, and `F5`, `F12`, and `Ctrl+Shift+B` belong to the IDE.
///
/// The proof is DERIVED from the catalog — the list comes from `crawrad_actions::catalogo()` and
/// not from here —, because an id list written in this test would fall behind
/// the day the ninth `edit.*` was created.
#[test]
fn o_teclado_de_edicao_e_do_editor_e_o_da_ide_continua_da_ide() {
    let com_atalho: Vec<_> = crawrad_actions::catalogue()
        .into_iter()
        .filter(|a| a.shortcut.is_some())
        .collect();
    assert!(
        com_atalho.len() >= 18,
        "the catalog shrank: {} actions with chord",
        com_atalho.len()
    );

    let mut calados = 0;
    for a in &com_atalho {
        let calado = crawrad_actions::key_routing::e_do_editor_de_texto(a.id);
        assert_eq!(
            calado,
            a.group == crawrad_actions::key_routing::GRUPO_DO_TEXTO,
            "`{}` (group `{}`) disagrees with the group rule",
            a.id,
            a.group
        );
        calados += usize::from(calado);
    }
    assert!(
        calados >= 8,
        "only {calados} chords remained with the editor; the eight of `Ctrl+Z`, \
         `Ctrl+Y`, `Ctrl+C`, `Ctrl+X`, `Ctrl+V`, `Ctrl+A`, `Ctrl+D`, and `Del` \
         are the minimum"
    );

    // THE TWO HALVES, NAMED. They repeat what the loop above already checks, and it is
    // deliberate: if the group rule ever changes shape, these four
    // lines state what the INTENT was, instead of letting the loop adapt
    // silently to the new rule.
    for id in ["edit.undo", "edit.redo", "edit.delete", "edit.copy"] {
        assert!(
            crawrad_actions::key_routing::e_do_editor_de_texto(id),
            "`{id}` returned to controlling the designer with the cursor inside a file"
        );
    }
    for id in [
        "file.save",
        "project.run",
        "project.generate",
        "view.toggle_code",
    ] {
        assert!(
            !crawrad_actions::key_routing::e_do_editor_de_texto(id),
            "`{id}` belongs to the IDE, and the IDE does not cease to exist because the cursor is \
             in a file"
        );
    }

    // AND A NON-EXISTENT ID does not belong to the editor: the rule queries the catalog, and a
    // `None` answered with `true` would mute the entire IDE on a typo.
    assert!(!crawrad_actions::key_routing::e_do_editor_de_texto(
        "nao.existe"
    ));
}

/// **The test the owner requested**: every shortcut declared in the catalog is
/// reachable through the bridge.
///
/// It is derived, not enumerated: the list comes from `crawrad_actions::catalogo()`,
/// the chord is read from `Action` itself, and the event is built from it. A
/// new action with `atalho: Some("Ctrl+K")` enters this count on its own — and
/// fails until `tecla_por_nome` knows what `K` is.
#[test]
fn todo_atalho_do_catalogo_e_alcancavel_pela_ponte() {
    let mut mudos = Vec::new();
    let mut vistos = 0;
    // THE TWO CHORDS, and the second for the same reason as the first:
    // `ATALHOS_PREFERIDOS` carries `Ctrl +`/`Ctrl -`/`Ctrl 0` for zoom, which
    // THIS shell delivers and the egui shell does not — there the host owns the key.
    // A preferred chord that the bridge did not reach would be the exact silent shortcut
    // this gate exists to catch, and the owner already encountered it once:
    // *"zoom with Ctrl+ and Ctrl- in the IDE does not work yet"*.
    let acordes: Vec<(&str, &str)> = crawrad_actions::catalogue()
        .iter()
        .flat_map(|a| {
            a.shortcut
                .into_iter()
                .chain(crawrad_actions::preferred_shortcuts(a.id).iter().copied())
                .map(|c| (a.id, c))
                .collect::<Vec<_>>()
        })
        .collect();
    for (id, acorde) in acordes {
        let acao = crawrad_actions::by_id(id).expect("id came from catalog");
        vistos += 1;
        let Some((tecla, pedido)) = crawrad_runtime_makepad::keys::parse_chord(acorde) else {
            mudos.push(format!("{id} ({acorde}): unknown key name"));
            continue;
        };
        let comandos = teclado::comandos(tecla, &modificadores(pedido), false);
        if !comandos.contains(&Comando::Acao(acao.id)) {
            mudos.push(format!("{id} ({acorde}): bridge did not return the action"));
        }
    }
    assert!(
        mudos.is_empty(),
        "catalog shortcut(s) that the keyboard bridge does NOT reach:\n  {}\n\
         A muted shortcut is a muted button: the action exists, the tooltip promises the key, \
         and the key does nothing. Teach the name to `tecla_por_nome` — do not \
         write a shortcut table in the shell.",
        mudos.join("\n  ")
    );
    assert!(
        vistos >= 25,
        "the catalog declared 25 chords — 18 `shortcut` plus 7 preferred — and \
         now declares {vistos}. If the count DROPPED, a chord vanished from \
         the catalog and this gate would pass measuring less than it used to."
    );
}

/// The `Del` from the report, by name.
///
/// It is separated from the derived test intentionally: if `edit.delete` leaves the
/// catalog, the test above stays green (it measures whatever is present) and this one
/// fails stating what was lost.
#[test]
fn del_apaga_o_componente_selecionado() {
    let comandos = teclado::comandos(KeyCode::Delete, &nada(), false);
    assert!(
        comandos.contains(&Comando::Acao("edit.delete")),
        "`Del` no longer reaches `edit.delete`. This was the report that opened \
         this round: adding via the Toolbox and being unable to remove."
    );
}

/// The neighboring chord does **not** trigger.
///
/// `Ctrl+Z` and `Ctrl+Shift+Z` are different keys, and undo/redo
/// would clash if modifier comparison used inclusion instead of
/// equality. It is the same rule as egui (`main.rs:5199-5217`).
#[test]
fn a_comparacao_de_modificadores_e_exata() {
    let ctrl = KeyModifiers {
        control: true,
        ..Default::default()
    };
    let ctrl_shift = KeyModifiers {
        control: true,
        shift: true,
        ..Default::default()
    };

    // Without the Apple platform `primary` is `control`; with it, `logo`. The
    // test is only valid where `control` IS primary, and that is where it runs.
    if !cfg!(target_vendor = "apple") {
        assert!(
            teclado::comandos(KeyCode::KeyZ, &ctrl, false).contains(&Comando::Acao("edit.undo")),
            "`Ctrl+Z` stopped undoing"
        );
        assert!(
            !teclado::comandos(KeyCode::KeyZ, &ctrl_shift, false)
                .contains(&Comando::Acao("edit.undo")),
            "`Ctrl+Shift+Z` triggered `edit.undo`: modifier comparison \
             became inclusion, and redo will fight with undo"
        );
        assert!(
            teclado::comandos(KeyCode::KeyS, &nada(), false).is_empty(),
            "the letter `S` alone became a shortcut. Typing in an unfocused field \
             would save the project."
        );
    }
}

/// **The four arrow chords**, and each with the MODE named by the owner.
///
/// The test queries for the MODE and never for a distance, and that is deliberate: the
/// distance belongs to `Designer::passo_da_seta`, which knows the grid. A number
/// written here would bind the bridge to a `Grid size` — and fail the day
/// someone changed the grid in the options, for an unrelated reason.
#[test]
fn cada_acorde_da_seta_pede_o_modo_certo() {
    use crawrad_designer::ArrowNudge;
    let shift = KeyModifiers {
        shift: true,
        ..Default::default()
    };
    let ctrl = KeyModifiers {
        control: true,
        ..Default::default()
    };
    let ctrl_shift = KeyModifiers {
        control: true,
        shift: true,
        ..Default::default()
    };
    for (tecla, dx, dy) in [
        (KeyCode::ArrowLeft, -1.0, 0.0),
        (KeyCode::ArrowRight, 1.0, 0.0),
        (KeyCode::ArrowUp, 0.0, -1.0),
        (KeyCode::ArrowDown, 0.0, 1.0),
    ] {
        for (m, modo, nome) in [
            (nada(), ArrowNudge::Move, "plain arrow"),
            (ctrl, ArrowNudge::MoveFine, "Ctrl+arrow"),
            (ctrl_shift, ArrowNudge::MoveFast, "Ctrl+Shift+arrow"),
            (shift, ArrowNudge::Resize, "Shift+arrow"),
        ] {
            assert!(
                teclado::comandos(tecla, &m, false).contains(&Comando::Seta { dx, dy, modo }),
                "`{nome}` with {tecla:?} does not ask for `{modo:?}`. That was the second                  report from the owner — \"arrows do not move components, only                  with mouse\" — and then the whole map that he dictated."
            );
        }
    }
}

/// `Esc` cancels the gesture — outside a field.
#[test]
fn esc_cancela_o_gesto() {
    assert!(teclado::comandos(KeyCode::Escape, &nada(), false).contains(&Comando::Cancelar));
}

/// **The text field guard**: typing never becomes a gesture.
///
/// An arrow inside `Filter` moves the text cursor, not the component. And `Del`
/// inside a field deletes a character — but note that `Del` never even
/// reaches here: Makepad's `TextInput` consumes it
/// (`makepad-widgets-1.0.0/src/text_input.rs:1602`) and never emits the
/// `KeyDownUnhandled` that feeds this path. The field decides.
#[test]
fn inside_field_arrows_and_esc_are_not_gestures() {
    for tecla in [
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::Escape,
    ] {
        assert!(
            teclado::comandos(tecla, &nada(), true).is_empty(),
            "{tecla:?} became a designer gesture coming from within a text \
             field. Typing in the Toolbox `Filter` would start nudging \
             components."
        );
    }
}

/// What the field REJECTED remains valid as a catalog shortcut.
///
/// It is the good half of the modern-editor rule: `Ctrl+S` with the cursor inside
/// `Filter` saves, because the field does nothing with it — and the field
/// itself indicates that via `KeyDownUnhandled`.
#[test]
fn unhandled_field_keys_remain_shortcuts() {
    if cfg!(target_vendor = "apple") {
        return;
    }
    let ctrl = KeyModifiers {
        control: true,
        ..Default::default()
    };
    assert!(
        teclado::comandos(KeyCode::KeyS, &ctrl, true).contains(&Comando::Acao("file.save")),
        "`Ctrl+S` stopped saving when coming from a field that rejected it. An \
         overly broad guard is the defect that the egui shell already had: inside \
         the editor, `F12` did not return to the design."
    );
}

/// **`Shift+arrow` resizes, and never moves.**
///
/// Here once stood the opposite: a test that kept the CHOICE of `Shift+arrow`
/// doing nothing, because *"this function does not exist in the brain"* —
/// `Designer::nudge` only moved. It stated, in writing, that it would fail the day
/// resizing arrived, requesting the wiring.
///
/// It arrived on 2026-09-17, and it failed. This is the same test from the other side:
/// the chord requests `Resize` and must **not** request `Move`, because moving
/// when resizing was requested is answering the wrong question — which was the
/// original argument, and still is.
#[test]
fn shift_com_seta_redimensiona_e_nao_move() {
    use crawrad_designer::ArrowNudge;
    let shift = KeyModifiers {
        shift: true,
        ..Default::default()
    };
    for tecla in [
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
    ] {
        let saida = teclado::comandos(tecla, &shift, false);
        assert!(
            saida.iter().any(|c| matches!(
                c,
                Comando::Seta {
                    modo: ArrowNudge::Resize,
                    ..
                }
            )),
            "`Shift+{tecla:?}` stopped resizing"
        );
        assert!(
            !saida.iter().any(|c| matches!(
                c,
                Comando::Seta {
                    modo: ArrowNudge::Move | ArrowNudge::MoveFine | ArrowNudge::MoveFast,
                    ..
                }
            )),
            "`Shift+{tecla:?}` MOVED the component. In a classic RAD IDE this chord              resizes; moving instead is answering the wrong question."
        );
    }
}

// ---------------------------------------------------------------------
// FAMILY 2 — wirings, and the prohibition of manual tables
// ---------------------------------------------------------------------

/// The Makepad shell, as text.
use super::casca::CASCA;

/// The bridge, as text.
const PONTE: &str = include_str!("../../ide_app/keyboard.rs");
const ROTEADOR: &str = include_str!("../../../../crawrad-actions/src/key_routing.rs");

/// `CASCA` without any whitespace — immune to `rustfmt` line breaks.
fn compacto() -> String {
    CASCA.chars().filter(|c| !c.is_whitespace()).collect()
}

/// The body of an `impl IdeApp` method, compacted. Same technique as
/// `test/ide_app/fios.rs`.
fn method_body(nome: &str) -> String {
    let agulha = format!("fn {nome}(");
    let i = CASCA
        .find(&agulha)
        .unwrap_or_else(|| panic!("method `{nome}` vanished from src/ide_app.rs"));
    let resto = &CASCA[i + agulha.len()..];
    let f = resto.find("\n    fn ").unwrap_or(resto.len());
    resto[..f].chars().filter(|c| !c.is_whitespace()).collect()
}

/// **THE LAW OF THE HOUSE**: the source of shortcuts is the catalog, never a custom table.
///
/// The proof is derived from the catalog itself: no action id may appear
/// written in the bridge. If someone translates `Del` to `"edit.delete"` in a `match`
/// here, this test reports the id that appeared. Two lists diverge, and the one that
/// diverges silently is the one that becomes wrong (ADR 0011).
#[test]
fn bridge_has_no_shortcut_table() {
    let mut escritos = Vec::new();
    for acao in crawrad_actions::catalogue() {
        if PONTE.contains(&format!("\"{}\"", acao.id)) {
            escritos.push(acao.id);
        }
    }
    assert!(
        escritos.is_empty(),
        "action id(s) handwritten in the keyboard bridge: {escritos:?}. \
         The shortcut comes from `Action::shortcut` in the catalog — a table here is the \
         second maintenance point that ADR 0011 forbids."
    );
    // The bridge hands the key to `key_routing`, and THAT reads the catalog.
    assert!(
        PONTE.contains("key_routing::commands(")
            && ROTEADOR.contains("crate::catalogue_with_scheme("),
        "the bridge stopped querying the catalog. If it does not read \
         `Action::shortcut`, it can only be reading its own table."
    );
}

/// Nor does the shell: no shortcut key written in `ide_app.rs`.
///
/// `handle_key_down` forwards to `teclado::comandos` and that is all. A
/// `KeyCode::KeyS => ...` in the middle of dispatch would be the table returning through
/// the back door.
#[test]
fn a_casca_nao_escreve_tecla_de_atalho() {
    for proibida in ["KeyCode::Key", "KeyCode::F1", "KeyCode::F5", "KeyCode::F12"] {
        assert!(
            !CASCA.contains(proibida),
            "`{proibida}` appeared in src/ide_app.rs. Key translation lives \
             in `ide_app/teclado.rs`, and dispatch only queries the catalog."
        );
    }
}

/// **Wire 1** — the key reaches the bridge.
///
/// The mutation: deleting the body of `handle_key_down`. The 18 shortcuts, the arrows, and
/// `Esc` return to doing nothing, **silently** — which is the state in which the
/// owner found the IDE.
#[test]
fn a_tecla_chega_ao_despacho() {
    let corpo = method_body("handle_key_down");
    assert!(
        corpo.contains("self.tecla(cx,ke,false)"),
        "`handle_key_down` stopped passing the key to dispatch. \
         `grep -c KeyDown` goes back to 0 in practice."
    );
    assert!(
        corpo.contains("cx.has_key_focus(campo.area())"),
        "the text field guard vanished from `handle_key_down`: typing `s` in \
         the Toolbox `Filter` would save the project, and `Del` in the AI panel \
         field would delete the selected component."
    );

    let despacho = method_body("tecla");
    assert!(
        despacho.contains("teclado::comandos(ke.key_code,&ke.modifiers,do_campo)"),
        "dispatch stopped asking the bridge which matches the key"
    );
    assert!(
        despacho.contains("self.execute_action(cx,id)"),
        "the catalog command no longer reaches `execute_action`: the shortcut \
         would trigger and no action would run"
    );
}

/// **Wire 2** — the text field is heard, and it decides.
#[test]
fn text_field_decides_shortcut_handling() {
    let c = compacto();
    assert!(
        c.contains("TextInputAction::KeyFocus=>self.campo_focado=wa.widgets.first().cloned()"),
        "the shell stopped tracking text field focus. Without this, either the \
         IDE keyboard goes deaf, or it steals keys from the typist."
    );
    assert!(
        c.contains("TextInputAction::KeyFocusLost=>{self.campo_focado=None;"),
        "a field that LOSES focus no longer returns the keyboard to the IDE: after \
         clicking once in `Filter`, `Del` would never work again."
    );
    // The SAME hook commits what was typed in the Object Inspector — leaving the
    // field and clicking on the form is the second of the two moments when a
    // classic RAD IDE's Object Inspector commits (the other being `Enter`). Without this line,
    // typing the caption and clicking outside would lose it silently.
    assert!(
        c.contains(
            "TextInputAction::KeyFocusLost=>{self.campo_focado=None;self.confirmar_edicao(cx);self.confirm_event(cx);}"
        ),
        "losing focus stopped confirming the Object Inspector draft: \
         typing the caption (or handler name, in the `Events` tab) and clicking \
         outside would silently discard what was typed."
    );
    assert!(
        c.contains("TextInputAction::KeyDownUnhandled(ke)=>self.tecla(cx,&ke,true)"),
        "a key that the field REJECTED stopped reaching dispatch: `Ctrl+S` with \
         the cursor in `Filter` would stop saving."
    );
}

/// **Wire 3** — the arrow reaches `crawrad-designer`, and the STEP comes from there.
///
/// The mutation: `fn seta(&mut self, ..) {}`. The arrows return to the state of the
/// report — "only with mouse" — and nothing on the screen says why.
///
/// Checking the STEP is what was introduced on 2026-09-17, with the four chords:
/// the shell might be tempted to multiply by its own number. The step belongs to the
/// designer, which knows the grid — a literal here would mean the `Grid size` from
/// options stops applying to the keyboard, and only to the keyboard.
#[test]
fn a_seta_chega_ao_designer() {
    let corpo = method_body("seta");
    assert!(
        corpo.contains("self.designer.nudge(d.ir(),&sel,dx*passo,dy*passo)"),
        "`seta` no longer calls `Designer::nudge`. Either arrows died, \
         or — worse — someone wrote the movement calculation in the shell, which is \
         exactly what ADR 0027 forbids."
    );
    assert!(
        corpo.contains(".nudge_size(d.ir(),&sel,dx*passo,dy*passo)"),
        "`Shift+arrow` no longer calls `Designer::nudge_size`: either it stopped \
         resizing, or went back to moving — and moving when resizing was requested \
         is answering the wrong question"
    );
    assert!(
        corpo.contains("self.designer.arrow_step(modo)"),
        "the shell stopped ASKING the designer for the step. A hardcoded number \
         here means the grid from options applies to the mouse and not the keyboard, \
         and the two shells disagreeing the day one of them is changed"
    );
    assert!(
        corpo.contains("dm.apply(tx)"),
        "the arrow transaction no longer reaches the document: arrow calculates and \
         throws away"
    );
    let cancelar = method_body("cancelar_gesto");
    assert!(
        cancelar.contains("self.designer.cancel()"),
        "`Esc` no longer calls `Designer::cancel`"
    );
    assert!(
        cancelar.contains("rt.set_marquee(None)"),
        "`Esc` no longer clears the selection marquee from the screen: the gesture \
         would cancel internally and remain drawn"
    );
}

/// **Wire 4** — the zoom from `Efeito` reaches the canvas.
///
/// This wire was born with a known defect: `grep -n zoom src/ide_app.rs`
/// returned **empty**. Connecting only the bridge would make `Ctrl+Shift+.` execute the action,
/// receive the effect, and **not change a pixel** — the same silent defect that
/// this shell previously had with `Historico` and `Arquivo`.
#[test]
fn o_zoom_do_efeito_chega_ao_canvas() {
    let corpo = method_body("apply_effect");
    assert!(
        corpo.contains("efeito.zoom"),
        "`apply_effect` ignored zoom from `Efeito` again: the shortcut \
         triggers, the action executes, and the screen remains unchanged, without explanation."
    );
    assert!(
        corpo.contains("crawrad_runtime::adjacent_zoom(self.zoom,true)"),
        "zoom steps stopped coming from `crawrad-runtime`. A table of \
         steps in the shell would disagree with egui on the first change."
    );
    let aplicar = method_body("aplicar_zoom");
    assert!(
        aplicar.contains("set_zoom(self.zoom)"),
        "zoom no longer reaches the canvas runtime"
    );
}

/// **Wire 5** — copied content survives until `Ctrl+V`.
///
/// The clipboard was newly created on each call
/// (`Clipboard::default()` inside `execute_with`), so `Ctrl+C` wrote
/// to an area that died on the same line and `Ctrl+V` answered
/// `clipboard is empty` forever.
#[test]
fn a_area_de_transferencia_sobrevive_entre_as_acoes() {
    let corpo = method_body("execute_with");
    assert!(
        corpo.contains("clipboard:&self.transferencia"),
        "`Contexto` received a freshly born clipboard again: \
         `Ctrl+V` would answer `clipboard is empty` right after a `Ctrl+C`."
    );
    let efeito = method_body("apply_effect");
    assert!(
        efeito.contains("efeito.copy_text"),
        "copied text is dropped on the floor again: neither IDE area nor \
         system."
    );
    assert!(
        efeito.contains("cx.copy_to_clipboard(&texto)"),
        "the clip stopped going to the system clipboard \
         (`makepad-platform-1.0.0/src/cx_api.rs:307`)"
    );
}

/// **Wire 6** — `F12` toggles the view, and what lacks wiring STATES that it lacks it.
///
/// The `Vista` and `ModoDoDesigner` families of `Efeito` were dropped on the floor along with
/// zoom. With the bridge connected that becomes `F12` executing the action and the screen remaining
/// unchanged — the usual silent defect.
#[test]
fn a_vista_e_a_trava_do_efeito_chegam_a_tela() {
    let corpo = method_body("apply_effect");
    assert!(
        corpo.contains("Some(crawrad_actions::ViewEffect::ToggleDesignCode)=>self.toggle_view(cx)"),
        "`F12` stopped switching between Design and Code again: the action executes, the \
         effect returns, and the screen remains unchanged."
    );
    // The last view with no wire got one: `ToggleProblems` shows or hides the
    // Problems panel, and a toggle that only spoke would be the old defect back.
    assert!(
        corpo.contains("Some(crawrad_actions::ViewEffect::ToggleProblems)=>{ifself.painel_a_vista(&self.arranjo(),Panel::Problems){self.esconder_painel(cx,Panel::Problems);}else{self.show_panel(cx,Panel::Problems);}}"),
        "`view.problems` stopped toggling the Problems panel"
    );
    assert!(
        corpo.contains("Some(crawrad_actions::ViewEffect::ToggleWelcome)=>self.show_welcome(cx)"),
        "`help.welcome` stopped opening the Welcome page"
    );
    assert!(
        corpo.contains("self.designer.locked=!self.designer.locked"),
        "`Ctrl+Shift+L` no longer toggles control lock"
    );
    assert!(
        corpo.contains("controlslocked"),
        "the lock returned to being a command without visible feedback: turning on and \
         off produce the same nothing on screen"
    );
}

#[test]
fn keyboard_schemes_toggle_build_and_run_commands() {
    let f5 = KeyCode::F5;
    let f9 = KeyCode::F9;
    let ctrl_shift_b = KeyModifiers {
        control: !APPLE,
        logo: APPLE,
        shift: true,
        ..Default::default()
    };
    let shift_f9 = KeyModifiers {
        shift: true,
        ..Default::default()
    };
    let ctrl_f9 = KeyModifiers {
        control: !APPLE,
        logo: APPLE,
        ..Default::default()
    };

    // Test directly via commands_with_scheme
    let run_modern =
        teclado::commands_with_scheme(f5, &nada(), false, crawrad_actions::KeymapScheme::Modern);
    assert!(run_modern.contains(&Comando::Acao("project.run")));

    let build_modern = teclado::commands_with_scheme(
        KeyCode::KeyB,
        &ctrl_shift_b,
        false,
        crawrad_actions::KeymapScheme::Modern,
    );
    assert!(build_modern.contains(&Comando::Acao("project.build_manager")));

    let run_classic =
        teclado::commands_with_scheme(f9, &nada(), false, crawrad_actions::KeymapScheme::Classic);
    assert!(run_classic.contains(&Comando::Acao("project.run")));

    let build_classic =
        teclado::commands_with_scheme(f9, &shift_f9, false, crawrad_actions::KeymapScheme::Classic);
    assert!(build_classic.contains(&Comando::Acao("project.build_manager")));

    let gen_classic =
        teclado::commands_with_scheme(f9, &ctrl_f9, false, crawrad_actions::KeymapScheme::Classic);
    assert!(gen_classic.contains(&Comando::Acao("project.generate")));

    // Test dynamic switching of active scheme
    teclado::set_active_scheme(crawrad_actions::KeymapScheme::Modern);
    assert_eq!(
        teclado::active_scheme(),
        crawrad_actions::KeymapScheme::Modern
    );
    let cmds = teclado::comandos(f5, &nada(), false);
    assert!(cmds.contains(&Comando::Acao("project.run")));

    teclado::set_active_scheme(crawrad_actions::KeymapScheme::Classic);
    assert_eq!(
        teclado::active_scheme(),
        crawrad_actions::KeymapScheme::Classic
    );
    let cmds_classic = teclado::comandos(f9, &nada(), false);
    assert!(cmds_classic.contains(&Comando::Acao("project.run")));

    // Restore to default Modern
    teclado::set_active_scheme(crawrad_actions::KeymapScheme::Modern);
}

/// **Go to Definition and Back / Forward reach the catalog by key**:
/// `Ctrl+F12` and `Alt+Left/Right` in the Modern scheme; the vendor's
/// `Alt+Up` and `Alt+Left/Right` in the classic one.
#[test]
fn definition_and_back_forward_keys_reach_the_catalog() {
    let classic = *crawrad_actions::KeymapScheme::ALL
        .iter()
        .find(|s| **s != crawrad_actions::KeymapScheme::Modern)
        .unwrap();
    let m = |control: bool, alt: bool| KeyModifiers {
        control: control && !APPLE,
        logo: control && APPLE,
        alt,
        ..Default::default()
    };
    let hit =
        |key, mods: &KeyModifiers, scheme| teclado::commands_with_scheme(key, mods, false, scheme);
    let vs = crawrad_actions::KeymapScheme::Modern;
    let go = |id: &'static str| vec![Comando::Acao(id)];
    assert_eq!(
        hit(KeyCode::F12, &m(true, false), vs),
        go("editor.goto_definition")
    );
    assert_eq!(
        hit(KeyCode::ArrowUp, &m(false, true), classic),
        go("editor.goto_definition")
    );
    for scheme in [vs, classic] {
        assert_eq!(
            hit(KeyCode::ArrowLeft, &m(false, true), scheme),
            go("editor.navigate_back")
        );
        assert_eq!(
            hit(KeyCode::ArrowRight, &m(false, true), scheme),
            go("editor.navigate_forward")
        );
    }
    // F12 alone is still Design / Code (the house's identity).
    assert_eq!(
        hit(KeyCode::F12, &m(false, false), vs),
        go("view.toggle_code")
    );
}

/// **Format Document reaches the catalog by key**: a modern editor's
/// `Shift+Alt+F`; the vendor's `Ctrl+D` in the classic scheme, where the
/// designer's duplicate moved to `Ctrl+Shift+D`.
#[test]
fn format_document_keys_reach_the_catalog() {
    let classic = *crawrad_actions::KeymapScheme::ALL
        .iter()
        .find(|s| **s != crawrad_actions::KeymapScheme::Modern)
        .unwrap();
    let vs = crawrad_actions::KeymapScheme::Modern;
    let m = |control: bool, shift: bool, alt: bool| KeyModifiers {
        control: control && !APPLE,
        logo: control && APPLE,
        shift,
        alt,
    };
    let hit =
        |key, mods: &KeyModifiers, scheme| teclado::commands_with_scheme(key, mods, false, scheme);
    let go = |id: &'static str| vec![Comando::Acao(id)];
    assert_eq!(
        hit(KeyCode::KeyF, &m(false, true, true), vs),
        go("editor.format_document")
    );
    assert_eq!(
        hit(KeyCode::KeyD, &m(true, false, false), classic),
        go("editor.format_document")
    );
    assert_eq!(
        hit(KeyCode::KeyD, &m(true, true, false), classic),
        go("edit.duplicate")
    );
    assert_eq!(
        hit(KeyCode::KeyD, &m(true, false, false), vs),
        go("edit.duplicate")
    );
}

/// **Find All References reaches the catalog by key**: a modern editor's
/// `Shift+Alt+F12`; the vendor's *Find References*, `Shift+Ctrl+Enter`, in
/// the classic scheme — and neither key does it in the other scheme.
#[test]
fn find_references_keys_reach_the_catalog() {
    let classic = *crawrad_actions::KeymapScheme::ALL
        .iter()
        .find(|s| **s != crawrad_actions::KeymapScheme::Modern)
        .unwrap();
    let vs = crawrad_actions::KeymapScheme::Modern;
    let m = |control: bool, shift: bool, alt: bool| KeyModifiers {
        control: control && !APPLE,
        logo: control && APPLE,
        shift,
        alt,
    };
    let hit =
        |key, mods: &KeyModifiers, scheme| teclado::commands_with_scheme(key, mods, false, scheme);
    let go = |id: &'static str| vec![Comando::Acao(id)];
    assert_eq!(
        hit(KeyCode::F12, &m(false, true, true), vs),
        go("editor.find_references")
    );
    assert_eq!(
        hit(KeyCode::ReturnKey, &m(true, true, false), classic),
        go("editor.find_references")
    );
    assert!(
        !hit(KeyCode::ReturnKey, &m(true, true, false), vs)
            .contains(&Comando::Acao("editor.find_references"))
    );
    assert!(
        !hit(KeyCode::F12, &m(false, true, true), classic)
            .contains(&Comando::Acao("editor.find_references"))
    );
}

/// **Save All reaches the catalog by the classic scheme's key**: the
/// vendor's `Shift+Ctrl+S`. A modern editor's is the chord `Ctrl+K S`, which this
/// shell does not take, so there `Ctrl+Shift+S` does not save all.
#[test]
fn save_all_key_reaches_the_catalog_in_the_classic_scheme() {
    let classic = *crawrad_actions::KeymapScheme::ALL
        .iter()
        .find(|s| **s != crawrad_actions::KeymapScheme::Modern)
        .unwrap();
    let vs = crawrad_actions::KeymapScheme::Modern;
    let ctrl_shift = KeyModifiers {
        control: !APPLE,
        logo: APPLE,
        shift: true,
        ..Default::default()
    };
    let hit = |scheme| teclado::commands_with_scheme(KeyCode::KeyS, &ctrl_shift, false, scheme);
    assert_eq!(hit(classic), vec![Comando::Acao("file.save_all")]);
    assert!(!hit(vs).contains(&Comando::Acao("file.save_all")));
}

/// **Rename Symbol reaches the catalog by the classic scheme's key**: the
/// vendor's *Refactor > Rename*, `Shift+Ctrl+E`. In the Modern scheme its
/// key is F2 in the code editor (`editor_keys`), so the catalog has none.
#[test]
fn rename_symbol_key_reaches_the_catalog_in_the_classic_scheme() {
    let classic = *crawrad_actions::KeymapScheme::ALL
        .iter()
        .find(|s| **s != crawrad_actions::KeymapScheme::Modern)
        .unwrap();
    let vs = crawrad_actions::KeymapScheme::Modern;
    let ctrl_shift = KeyModifiers {
        control: !APPLE,
        logo: APPLE,
        shift: true,
        ..Default::default()
    };
    let hit = |scheme| teclado::commands_with_scheme(KeyCode::KeyE, &ctrl_shift, false, scheme);
    assert_eq!(hit(classic), vec![Comando::Acao("editor.rename_symbol")]);
    assert!(!hit(vs).contains(&Comando::Acao("editor.rename_symbol")));
}

/// **A key with two owners runs the one of the view in front**:
/// Ctrl+C is the components' copy in the designer and the text's with the
/// code in front; with the editor focused, its editing keys belong to the
/// system's clipboard events (never twice); an action of any view always runs.
#[test]
fn a_key_with_two_owners_runs_the_view_in_front() {
    // the designer in front, nothing focused in the editor
    assert!(!teclado::not_here("edit.copy", false, false));
    assert!(teclado::not_here("editor.copy", false, false));
    // the code in front, the editor not focused (a panel has the keys)
    assert!(teclado::not_here("edit.copy", false, true));
    assert!(!teclado::not_here("editor.copy", false, true));
    // the editor focused: the system's events copy, the catalog does not
    assert!(teclado::not_here("editor.copy", true, true));
    // any view
    assert!(
        !teclado::not_here("file.save", false, true)
            && !teclado::not_here("file.save", false, false)
    );
}

#[test]
fn ctrl_alt_d_n_m_open_the_three_note_kinds() {
    let m = KeyModifiers {
        control: !APPLE,
        logo: APPLE,
        alt: true,
        ..Default::default()
    };
    for (key, id) in [
        (KeyCode::KeyD, "editor.add_documentation"),
        (KeyCode::KeyN, "editor.add_note"),
        (KeyCode::KeyM, "editor.add_comment"),
    ] {
        let fired = teclado::comandos(key, &m, false);
        assert!(
            fired.contains(&Comando::Acao(id)),
            "{key:?} fired {fired:?}"
        );
    }
}
