//! The gate over the editor's navigation keys.
//!
//! Every rule here is arithmetic over text, so it is measured with no window —
//! which is the point. The half that needs a window is one line at the call
//! site (`set_selection` vs `move_to`), and it has a wire gate of its own.

use makepad_widgets::{KeyCode, KeyModifiers};

use super::{Command, LineDirection, alvo, comando, command_with_scheme, navegacao};
use crawrad_host::editor_chords::Motion;
use crawrad_host::word_motion::{
    word_end_right as fim_da_palavra_a_direita, word_start_left as inicio_da_palavra_a_esquerda,
};

/// A line of code this IDE actually writes — the one the owner had on screen.
const LINHA: &str = "        self.edt_nome_.text.clear();";

fn ctrl() -> KeyModifiers {
    KeyModifiers {
        control: true,
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// THE CHORD
// ---------------------------------------------------------------------------

/// **The four chords are read, and `Shift` is the difference between moving and
/// selecting.**
#[test]
fn os_quatro_acordes_sao_lidos() {
    let esperado = [
        (KeyCode::ArrowLeft, Motion::WordLeft),
        (KeyCode::ArrowRight, Motion::WordRight),
        (KeyCode::Home, Motion::FileStart),
        (KeyCode::End, Motion::FileEnd),
    ];
    for (tecla, movimento) in esperado {
        let n = navegacao(tecla, &ctrl()).unwrap_or_else(|| panic!("{tecla:?} was not read"));
        assert_eq!(n.motion, movimento);
        assert!(!n.extend, "without Shift the selection collapses");

        let com_shift = KeyModifiers {
            shift: true,
            ..ctrl()
        };
        assert!(
            navegacao(tecla, &com_shift)
                .expect("with Shift is also ours")
                .extend,
            "with Shift the anchor must follow — otherwise `Ctrl+Shift+Left` \
             moves cursor and loses selection"
        );
    }
}

/// **Without `Ctrl`, the key is NOT ours — it belongs to the framework.**
///
/// The defect this prevents: the plain arrow, `Home`, and `End` already work
/// (`code_editor.rs:800-820`). Stealing these chords would mean two responses to
/// the same key, and ours is unaware of `preferred_column_index` or folding.
#[test]
fn sem_ctrl_a_tecla_nao_e_nossa() {
    let nu = KeyModifiers::default();
    for tecla in [
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
        KeyCode::Home,
        KeyCode::End,
    ] {
        assert!(
            navegacao(tecla, &nu).is_none(),
            "{tecla:?} alone belongs to framework"
        );
    }
    // Nor `Alt`: in VS Code `Alt+Left` is *navigate back* in
    // position history, which is an entirely different feature.
    let alt = KeyModifiers {
        alt: true,
        ..ctrl()
    };
    assert!(navegacao(KeyCode::ArrowLeft, &alt).is_none());
}

/// **Catalog `Ctrl+Y` does not become an editor command — this is the owner's decision.**
///
/// # It was wired for one iteration
///
/// It redid text, under the chord assigned by the catalog to `edit.redo`. The owner tested and
/// decided:
///
/// > *"Ctrl+Y doesn't do it, but the VS Code standard doesn't either, that's only in [classic] which
/// > deletes the line. So Ctrl+Y should only work when we have keymap switching
/// > and select [classic]."*
///
/// So inside the editor the key is **silenced intentionally** until keymap
/// schemes exist — the feature carried by ADR 0011 since 31/08/2026.
///
/// # Silenced is not dangerous, and that is what this test checks
///
/// The chord is in the catalog's `Edit` group, so with the cursor in a file
/// it is held by [`e_do_editor_de_texto`], and it does not reach the FORM's
/// history. Both halves — it does not become a command, and does not leak to the
/// form — prove that the key is inert, not free-floating.
///
/// [`e_do_editor_de_texto`]: crawrad_actions::key_routing::e_do_editor_de_texto
#[test]
fn ctrl_y_belongs_to_classic_scheme() {
    let acorde = crawrad_actions::by_id("edit.redo")
        .and_then(|a| a.shortcut)
        .expect("`edit.redo` must have a chord in catalog");
    let (tecla, pedido) =
        crawrad_runtime_makepad::keys::parse_chord(acorde).expect("bridge recognizes this chord");
    let m = KeyModifiers {
        shift: pedido.shift,
        alt: pedido.alt,
        control: pedido.primary && !cfg!(target_vendor = "apple"),
        logo: pedido.primary && cfg!(target_vendor = "apple"),
    };
    assert_eq!(
        comando(tecla, &m),
        None,
        "`Ctrl+Y` started doing something inside editor again, and owner ordered \
         leaving it for classic scheme"
    );
    assert!(
        crawrad_actions::key_routing::e_do_editor_de_texto("edit.redo"),
        "`Ctrl+Y` is silenced in editor AND free for form: with \
         cursor in a file it would redo last design transaction"
    );
}

/// **`comando` is the single entry point, and navigation goes through it.**
///
/// Two entry points would mean two chord lists diverging — which is the same
/// reason why catalog shortcuts are written in one place only (ADR 0011).
#[test]
fn a_navegacao_passa_pela_porta_unica() {
    for tecla in [
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
        KeyCode::Home,
        KeyCode::End,
    ] {
        let n = navegacao(tecla, &ctrl()).expect("already tested above");
        assert_eq!(comando(tecla, &ctrl()), Some(Command::Move(n)));
    }
}

/// **A key not in the list does not become a motion.**
#[test]
fn tecla_de_fora_nao_vira_movimento() {
    for tecla in [
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::KeyA,
        KeyCode::Tab,
    ] {
        assert!(
            navegacao(tecla, &ctrl()).is_none(),
            "{tecla:?} is not our navigation"
        );
    }
}

// ---------------------------------------------------------------------------
// THE WORD
// ---------------------------------------------------------------------------

/// **`Ctrl+Left` stops at the START of the word, and symbols are stops.**
///
/// This is why three classes exist rather than two. On the line above,
/// coming from the end, the stops are: `(`, `clear`, `.`, `text`, `.`, `edt_nome_`,
/// `.`, `self`, and the indentation.
#[test]
fn ctrl_esquerda_para_no_comeco_da_palavra() {
    let fim = LINHA.len();
    let mut paradas = Vec::new();
    let mut b = fim;
    while let Some(n) = inicio_da_palavra_a_esquerda(LINHA, b) {
        if n == b {
            panic!("walk got stuck at {b}");
        }
        b = n;
        paradas.push(&LINHA[b..]);
        if paradas.len() > 40 {
            panic!("walk does not terminate");
        }
    }
    assert_eq!(b, 0, "walk must end at line start");
    assert!(
        paradas.contains(&"clear();"),
        "did not stop at start of `clear`: {paradas:?}"
    );
    assert!(
        paradas.contains(&"text.clear();"),
        "did not stop at start of `text`: {paradas:?}"
    );
    assert!(
        paradas.contains(&"edt_nome_.text.clear();"),
        "`edt_nome_` is ONE word — `_` does not split identifier: {paradas:?}"
    );
    assert!(
        paradas.contains(&"self.edt_nome_.text.clear();"),
        "did not stop at start of `self`: {paradas:?}"
    );
}

/// **The entire indentation is ONE step.**
///
/// Without this, leaving `self` on a line indented by eight spaces would cost
/// eight keypresses — and this project writes indented code.
#[test]
fn a_indentacao_e_um_passo_so() {
    let comeco_do_self = LINHA.find("self").expect("setup of this test");
    assert_eq!(
        inicio_da_palavra_a_esquerda(LINHA, comeco_do_self),
        Some(0),
        "the eight spaces must be crossed in a single press"
    );
}

/// **Space to the left of the cursor is consumed BEFORE measuring the word
/// begins — not in place of it.**
///
/// # The mutation this test exists to catch
///
/// It originated from a measurement, not from zeal: deleting the loop that skips space in
/// [`inicio_da_palavra_a_esquerda`] kept the indentation test **green**.
/// On a line with only whitespace to the left both loops yield the same `0` — the
/// second consumes the space by accident, classifying it as its "word".
///
/// The case separating them is the cursor right after A SINGLE space in the middle of
/// the line. With the skip, `Ctrl+Left` reaches `=`; without it, it stops at the space — half
/// a keypress, prompting the user to press it again.
#[test]
fn o_espaco_a_esquerda_e_pulado_antes_da_palavra() {
    let linha = "let x = 1;";
    let um = linha.find('1').expect("setup of this test");
    assert_eq!(&linha[um - 1..um], " ", "setup lost the space");
    assert_eq!(
        inicio_da_palavra_a_esquerda(linha, um),
        Some(linha.find('=').expect("setup of this test")),
        "walk stopped AT the space instead of crossing it"
    );
}

/// **`Ctrl+Right` stops at the END of the word — VS Code's asymmetry.**
///
/// It is not a whim: it is what makes `Ctrl+Shift+Right` select the entire
/// word starting from its beginning, rather than selecting up to the start of the
/// next and dragging the whitespace along.
#[test]
fn ctrl_direita_para_no_fim_da_palavra() {
    let comeco_do_self = LINHA.find("self").expect("setup of this test");
    let n = fim_da_palavra_a_direita(LINHA, comeco_do_self).expect("there is a word to the right");
    assert_eq!(
        &LINHA[comeco_do_self..n],
        "self",
        "did not select entire `self`"
    );

    // And from the start of the line, the first step skips indentation AND the
    // word: `        self` in one press.
    let n = fim_da_palavra_a_direita(LINHA, 0).expect("there is a word to the right");
    assert_eq!(&LINHA[..n], "        self");
}

/// **Extremes return `None`, which is how lines are crossed.**
#[test]
fn os_extremos_pedem_para_atravessar() {
    assert_eq!(inicio_da_palavra_a_esquerda(LINHA, 0), None);
    assert_eq!(fim_da_palavra_a_direita(LINHA, LINHA.len()), None);
    // Empty line: nowhere to go in either direction.
    assert_eq!(inicio_da_palavra_a_esquerda("", 0), None);
    assert_eq!(fim_da_palavra_a_direita("", 0), None);
}

/// **A line of ONLY whitespace walks from end to end without hanging.**
///
/// This is the case of an indented blank line, which every Rust file has inside
/// an `impl`. Walking left consumes everything and returns `0`; walking
/// right returns the end.
#[test]
fn a_linha_so_de_espaco_nao_trava() {
    let branca = "    ";
    assert_eq!(inicio_da_palavra_a_esquerda(branca, branca.len()), Some(0));
    assert_eq!(fim_da_palavra_a_direita(branca, 0), Some(branca.len()));
}

/// **A byte in the MIDDLE of a character does not panic.**
///
/// The first line of every unit written by this generator has an accented character, and
/// slicing a `&str` outside a boundary crashes the process.
#[test]
fn um_byte_dentro_de_um_caractere_nao_derruba() {
    let linha = "// Este arquivo é seu";
    let byte_do_acento = linha.find('é').expect("the accent of this line");
    assert!(linha.len() > linha.chars().count(), "setup lost the accent");
    // The byte in the MIDDLE of `é`.
    let dentro = byte_do_acento + 1;
    let e = inicio_da_palavra_a_esquerda(linha, dentro).expect("there is a word to the left");
    let d = fim_da_palavra_a_direita(linha, dentro).expect("there is a word to the right");
    assert!(linha.is_char_boundary(e), "{e} is not a character boundary");
    assert!(linha.is_char_boundary(d), "{d} is not a character boundary");
}

// ---------------------------------------------------------------------------
// THE TARGET, ACROSS LINES
// ---------------------------------------------------------------------------

fn arquivo() -> Vec<String> {
    vec![
        "fn main() {".to_string(),
        "    let x = 1;".to_string(),
        "".to_string(),
        "}".to_string(),
    ]
}

/// **At the start of the line, `Ctrl+Left` moves up to the END of the line above.**
///
/// Without this the cursor would get stuck at the start of each line, and the user would
/// have to press the plain arrow once to unstick it — which is exactly the complaint
/// that "the key does nothing".
#[test]
fn no_comeco_da_linha_ctrl_esquerda_sobe() {
    let l = arquivo();
    assert_eq!(alvo(&l, (1, 0), Motion::WordLeft), (0, l[0].len()));
    // And from the end of a line, `Ctrl+Right` moves down to the start of the line below.
    assert_eq!(alvo(&l, (0, l[0].len()), Motion::WordRight), (1, 0));
}

/// **FILE extrema stop instead of wrapping around.**
#[test]
fn file_extrema_stop_at_ends() {
    let l = arquivo();
    assert_eq!(alvo(&l, (0, 0), Motion::WordLeft), (0, 0));
    let ultima = l.len() - 1;
    assert_eq!(
        alvo(&l, (ultima, l[ultima].len()), Motion::WordRight),
        (ultima, l[ultima].len())
    );
}

/// **`Ctrl+Home` and `Ctrl+End` navigate the entire file, not the line.**
///
/// This is the distinction the framework fails to make: its `Home` calls `session.home`
/// with the modifier ignored (`code_editor.rs:800-810`), so `Ctrl+Home`
/// returned the start of the LINE.
#[test]
fn ctrl_home_and_ctrl_end_navigate_file() {
    let l = arquivo();
    assert_eq!(alvo(&l, (2, 0), Motion::FileStart), (0, 0));
    let ultima = l.len() - 1;
    assert_eq!(alvo(&l, (1, 3), Motion::FileEnd), (ultima, l[ultima].len()));
}

/// **An empty file, and a nonexistent line, respond instead of crashing.**
///
/// The position comes from the editor session and text comes from the same session, but a
/// frame between tab switching and drawing can leave the two out of sync.
#[test]
fn uma_posicao_impossivel_nao_derruba() {
    assert_eq!(alvo(&[], (7, 9), Motion::WordLeft), (0, 0));
    let l = arquivo();
    let ultima = l.len() - 1;
    assert_eq!(
        alvo(&l, (99, 0), Motion::FileEnd),
        (ultima, l[ultima].len())
    );
}

// ---------------------------------------------------------------------------
// CODE MANIPULATION TESTS (LAYER 1 - VS CODE)
// ---------------------------------------------------------------------------

#[test]
fn acordes_de_manipulacao_sao_reconhecidos() {
    let ctrl = KeyModifiers {
        control: true,
        ..Default::default()
    };
    let alt = KeyModifiers {
        alt: true,
        ..Default::default()
    };
    let shift_alt = KeyModifiers {
        alt: true,
        shift: true,
        ..Default::default()
    };
    let ctrl_shift = KeyModifiers {
        control: true,
        shift: true,
        ..Default::default()
    };

    assert_eq!(comando(KeyCode::Slash, &ctrl), Some(Command::ToggleComment));
    assert_eq!(
        comando(KeyCode::Semicolon, &ctrl),
        Some(Command::ToggleComment)
    );
    assert_eq!(
        comando(KeyCode::ArrowUp, &alt),
        Some(Command::MoveLines(LineDirection::Up))
    );
    assert_eq!(
        comando(KeyCode::ArrowDown, &alt),
        Some(Command::MoveLines(LineDirection::Down))
    );
    assert_eq!(
        comando(KeyCode::ArrowUp, &shift_alt),
        Some(Command::DuplicateLines(LineDirection::Up))
    );
    assert_eq!(
        comando(KeyCode::ArrowDown, &shift_alt),
        Some(Command::DuplicateLines(LineDirection::Down))
    );
    assert_eq!(
        comando(KeyCode::KeyK, &ctrl_shift),
        Some(Command::DeleteLine)
    );
    assert_eq!(comando(KeyCode::KeyD, &ctrl), Some(Command::NextOccurrence));
    assert_eq!(
        comando(KeyCode::KeyF, &ctrl),
        Some(Command::OpenFind(false))
    );
    assert_eq!(comando(KeyCode::KeyH, &ctrl), Some(Command::OpenFind(true)));
    assert_eq!(comando(KeyCode::KeyG, &ctrl), Some(Command::OpenGoto));
    assert_eq!(
        comando(KeyCode::KeyP, &ctrl),
        Some(Command::RunAction("file.quick_open"))
    );
    assert_eq!(
        comando(KeyCode::F3, &KeyModifiers::default()),
        Some(Command::FindNext)
    );
    assert_eq!(
        comando(
            KeyCode::F3,
            &KeyModifiers {
                shift: true,
                ..Default::default()
            }
        ),
        Some(Command::FindPrevious)
    );
}

#[test]
fn comando_reconhece_quick_open_goto_e_abrir_nota() {
    let ctrl_p = KeyModifiers {
        control: true,
        ..Default::default()
    };
    assert_eq!(
        comando(KeyCode::KeyP, &ctrl_p),
        Some(Command::RunAction("file.quick_open"))
    );

    let ctrl_shift_p = KeyModifiers {
        control: true,
        shift: true,
        ..Default::default()
    };
    assert_eq!(
        comando(KeyCode::KeyP, &ctrl_shift_p),
        Some(Command::RunAction("view.command_palette"))
    );

    let ctrl_g = KeyModifiers {
        control: true,
        ..Default::default()
    };
    assert_eq!(comando(KeyCode::KeyG, &ctrl_g), Some(Command::OpenGoto));

    let ctrl_shift_n = KeyModifiers {
        control: true,
        shift: true,
        ..Default::default()
    };
    assert_eq!(
        comando(KeyCode::KeyN, &ctrl_shift_n),
        Some(Command::RunAction("tools.sidecar_note"))
    );
    // The three kinds of the note popup reach their actions from the editor.
    let ctrl_alt = KeyModifiers {
        control: true,
        alt: true,
        ..Default::default()
    };
    for (key, id) in [
        (KeyCode::KeyD, "editor.add_documentation"),
        (KeyCode::KeyN, "editor.add_note"),
        (KeyCode::KeyM, "editor.add_comment"),
    ] {
        assert_eq!(comando(key, &ctrl_alt), Some(Command::RunAction(id)));
    }

    let ctrl_shift_lbracket = KeyModifiers {
        control: true,
        shift: true,
        ..Default::default()
    };
    assert_eq!(
        comando(KeyCode::LBracket, &ctrl_shift_lbracket),
        Some(Command::Fold)
    );

    let ctrl_shift_rbracket = KeyModifiers {
        control: true,
        shift: true,
        ..Default::default()
    };
    assert_eq!(
        comando(KeyCode::RBracket, &ctrl_shift_rbracket),
        Some(Command::Unfold)
    );

    let ctrl_space = KeyModifiers {
        control: true,
        ..Default::default()
    };
    assert_eq!(
        comando(KeyCode::Space, &ctrl_space),
        Some(Command::Complete)
    );
    // The code templates.
    assert_eq!(
        comando(KeyCode::KeyJ, &ctrl_space),
        Some(Command::RunAction("editor.templates"))
    );
}

#[test]
fn command_with_scheme_respects_classic_and_vscode() {
    let ctrl = KeyModifiers {
        control: true,
        ..Default::default()
    };
    let alt = KeyModifiers {
        alt: true,
        ..Default::default()
    };
    let ctrl_shift = KeyModifiers {
        control: true,
        shift: true,
        ..Default::default()
    };

    // In VS Code scheme:
    // - Ctrl+Y is NOT DeleteLine (remains reserved for Redo / catalog)
    // - Ctrl+Shift+K is DeleteLine
    // - Ctrl+H is Replace
    // - Ctrl+G is Go to line
    // - Ctrl+P is Quick Open
    // - Ctrl+/ is ToggleComment
    assert_eq!(
        command_with_scheme(KeyCode::KeyY, &ctrl, crawrad_actions::KeymapScheme::VsCode),
        None
    );
    assert_eq!(
        command_with_scheme(
            KeyCode::KeyK,
            &ctrl_shift,
            crawrad_actions::KeymapScheme::VsCode
        ),
        Some(Command::DeleteLine)
    );
    assert_eq!(
        command_with_scheme(KeyCode::KeyH, &ctrl, crawrad_actions::KeymapScheme::VsCode),
        Some(Command::OpenFind(true))
    );
    assert_eq!(
        command_with_scheme(KeyCode::KeyG, &ctrl, crawrad_actions::KeymapScheme::VsCode),
        Some(Command::OpenGoto)
    );
    assert_eq!(
        command_with_scheme(KeyCode::KeyP, &ctrl, crawrad_actions::KeymapScheme::VsCode),
        Some(Command::RunAction("file.quick_open"))
    );
    assert_eq!(
        command_with_scheme(KeyCode::Slash, &ctrl, crawrad_actions::KeymapScheme::VsCode),
        Some(Command::ToggleComment)
    );

    // In classic scheme:
    // - Ctrl+Y IS DeleteLine (classic in RAD IDEs)
    // - Ctrl+Shift+K is NOT DeleteLine
    // - Ctrl+R is Replace
    // - Alt+G is Go to line
    // - Ctrl+O is Quick Open
    // - Ctrl+Shift+C is ToggleComment
    assert_eq!(
        command_with_scheme(KeyCode::KeyY, &ctrl, crawrad_actions::KeymapScheme::Classic),
        Some(Command::DeleteLine)
    );
    assert_eq!(
        command_with_scheme(
            KeyCode::KeyK,
            &ctrl_shift,
            crawrad_actions::KeymapScheme::Classic
        ),
        None
    );
    assert_eq!(
        command_with_scheme(KeyCode::KeyR, &ctrl, crawrad_actions::KeymapScheme::Classic),
        Some(Command::OpenFind(true))
    );
    assert_eq!(
        command_with_scheme(KeyCode::KeyG, &alt, crawrad_actions::KeymapScheme::Classic),
        Some(Command::OpenGoto)
    );
    assert_eq!(
        command_with_scheme(KeyCode::KeyO, &ctrl, crawrad_actions::KeymapScheme::Classic),
        Some(Command::RunAction("file.quick_open"))
    );
    assert_eq!(
        command_with_scheme(
            KeyCode::KeyC,
            &ctrl_shift,
            crawrad_actions::KeymapScheme::Classic
        ),
        Some(Command::ToggleComment)
    );
}

/// **Numbered bookmarks: `Ctrl+Shift+N` sets, `Ctrl+N` goes**, in both
/// schemes; `Ctrl+0` is no bookmark.
#[test]
fn digits_set_and_go_to_numbered_bookmarks() {
    let shift = KeyModifiers {
        shift: true,
        ..ctrl()
    };
    for &scheme in crawrad_actions::KeymapScheme::ALL {
        assert_eq!(
            command_with_scheme(KeyCode::Key1, &shift, scheme),
            Some(Command::Bookmark {
                number: 1,
                set: true
            })
        );
        assert_eq!(
            command_with_scheme(KeyCode::Key3, &ctrl(), scheme),
            Some(Command::Bookmark {
                number: 3,
                set: false
            })
        );
        assert_ne!(
            command_with_scheme(KeyCode::Key0, &ctrl(), scheme),
            Some(Command::Bookmark {
                number: 0,
                set: false
            })
        );
    }
}

/// The scheme that is not VS Code's, named without its vendor (owner's rule).
fn classic() -> crawrad_actions::KeymapScheme {
    *crawrad_actions::KeymapScheme::ALL
        .iter()
        .find(|s| **s != crawrad_actions::KeymapScheme::VsCode)
        .unwrap()
}

/// **Classic scheme: `Alt+Up` is the vendor's Find Declaration, so
/// the editor leaves it to the catalog and moves lines on `Ctrl+Alt+Up`.**
/// VS Code's `Alt+Up` still moves lines.
#[test]
fn alt_up_moves_lines_only_in_vs_code() {
    use crawrad_actions::KeymapScheme::VsCode;
    let alt = KeyModifiers {
        alt: true,
        ..Default::default()
    };
    let ctrl_alt = KeyModifiers {
        alt: true,
        control: true,
        ..Default::default()
    };
    let up = KeyCode::ArrowUp;
    assert_eq!(
        command_with_scheme(up, &alt, VsCode),
        Some(Command::MoveLines(LineDirection::Up))
    );
    assert_eq!(command_with_scheme(up, &alt, classic()), None);
    assert_eq!(
        command_with_scheme(up, &ctrl_alt, classic()),
        Some(Command::MoveLines(LineDirection::Up))
    );
    assert_eq!(
        command_with_scheme(KeyCode::ArrowDown, &ctrl_alt, classic()),
        Some(Command::MoveLines(LineDirection::Down))
    );
}

/// **Classic `Ctrl+D` is the vendor's Format Source**: the editor
/// leaves it to the catalog and finds the next occurrence on `Ctrl+Shift+D`.
/// VS Code's `Ctrl+D` is still the next occurrence.
#[test]
fn ctrl_d_formats_only_in_the_classic_scheme() {
    use crawrad_actions::KeymapScheme::VsCode;
    let ctrl = KeyModifiers {
        control: true,
        ..Default::default()
    };
    let ctrl_shift = KeyModifiers {
        control: true,
        shift: true,
        ..Default::default()
    };
    let d = KeyCode::KeyD;
    assert_eq!(
        command_with_scheme(d, &ctrl, VsCode),
        Some(Command::NextOccurrence)
    );
    assert_eq!(command_with_scheme(d, &ctrl, classic()), None);
    assert_eq!(
        command_with_scheme(d, &ctrl_shift, classic()),
        Some(Command::NextOccurrence)
    );
}

/// **VS Code's F2 renames the symbol only from the code editor**:
/// the editor takes it before the catalog, so in the Explorer F2 still
/// renames a file. The classic scheme leaves F2 alone in the editor.
#[test]
fn f2_in_the_code_editor_is_rename_symbol_only_in_vs_code() {
    let none = KeyModifiers::default();
    let vs = crawrad_actions::KeymapScheme::VsCode;
    let classic = *crawrad_actions::KeymapScheme::ALL
        .iter()
        .find(|s| **s != vs)
        .unwrap();
    assert!(matches!(
        command_with_scheme(KeyCode::F2, &none, vs),
        Some(Command::RenameSymbol)
    ));
    assert!(command_with_scheme(KeyCode::F2, &none, classic).is_none());
    let shift = KeyModifiers {
        shift: true,
        ..Default::default()
    };
    assert!(command_with_scheme(KeyCode::F2, &shift, vs).is_none());
}
