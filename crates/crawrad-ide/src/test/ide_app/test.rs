//! The shell's THEME gate — and what it no longer can test.
//!
//! # What it was, and why it changed
//!
//! Until 17/09/2026 there were TWO shells, and the theme was written twice: in
//! Rust (`src/tema.rs`, egui) and in DSL (`src/ide_app/tema.rs`, Makepad),
//! because `live_design!` does not compile Rust and does not interpolate any constants.
//! Three tests here read both files as text and compared them number by
//! number — it was the only thing preventing the duplicate code from diverging.
//!
//! ADR 0032
//! removed the egui shell, at the owner's request: *"the IDE IS ONLY GOING TO BE IN MAKEPAD"*. With
//! it went the other side of the comparison, and the three tests went along — **not
//! because they were in the way, but because there was nothing left to compare against**. The
//! duplication they guarded ceased to exist, and `ide_app/tema.rs`
//! became the single source of truth for the appearance of the IDE.
//!
//! # What remains here, and remains valid
//!
//! Two tests, and neither required egui:
//!
//! - **no hardcoded colour outside the canvas mockup** — chrome references tokens,
//!   not arbitrary hex values;
//! - **every token referenced by the shell exists in the theme** — an undeclared
//!   `CRAWRAD_ACENTU` does not break the Makepad build: it disappears from the screen and writes
//!   a log line.
//!
//! # What this gate does NOT catch, and is worth noting
//!
//! It does not check whether a token was used in the right place, and does not check DSL
//! PROPERTY names (`color:` mistyped as `colorr:`), which passes `cargo
//! check`, passes `live_expand`, instantiates the widget, and only shows up as a
//! log line (`makepad-live-compiler-1.0.0/src/live_cx.rs:153-166`). For
//! that, only visual inspection or log capturing helps.

/// **Where a Makepad shell template can be declared**, in DSL.
///
/// There are two files since 17/09/2026, and not out of preference: the eleven modals moved
/// from `ide_app.rs` to their own `link` when it exceeded ten thousand lines.
/// The theme defines how the shell LOOKS; modals are entire windows, and
/// stuffing them into the theme would bloat it with non-theme code.
///
/// Both are included here because this gate asks a single question — *"does the name
/// referenced by the shell exist somewhere?"* —, and a token declared in an adjacent
/// file is not a bug: it is organization.
pub(crate) const TEMA_COMPLETO: &str = concat!(
    include_str!("../../ide_app/theme.rs"),
    include_str!("../../ide_app/theme/tokens.rs"),
    include_str!("../../ide_app/theme/widgets.rs"),
    include_str!("../../ide_app/theme/class_icons.rs"),
    include_str!("../../ide_app/theme/workspace.rs"),
    include_str!("../../ide_app/theme/inspector_widgets.rs"),
    include_str!("../../ide_app/theme/dialog_templates.rs"),
    include_str!("../../ide_app/theme/explorer_dock.rs"),
);

/// The palette components the IDE itself uses: defined by the runtime the
/// generated apps share, not by the shell's theme.
const PALETTE: &str = concat!(
    include_str!("../../../../crawrad-runtime-makepad/src/list.rs"),
    include_str!("../../../../crawrad-runtime-makepad/src/card.rs"),
    include_str!("../../../../crawrad-runtime-makepad/src/breadcrumb.rs"),
    include_str!("../../../../crawrad-runtime-makepad/src/chips.rs"),
    include_str!("../../../../crawrad-runtime-makepad/src/progress_card.rs"),
    include_str!("../../../../crawrad-runtime-makepad/src/check_list.rs"),
    include_str!("../../../../crawrad-runtime-makepad/src/segment_list.rs"),
    include_str!("../../../../crawrad-runtime-makepad/src/history_panel.rs"),
    include_str!("../../../../crawrad-runtime-makepad/src/welcome.rs"),
    include_str!("../../../../crawrad-runtime-makepad/src/diff_view.rs"),
    include_str!("../../../../crawrad-runtime-makepad/src/column_splitter.rs"),
    include_str!("../../../../crawrad-runtime-makepad/src/color_picker.rs"),
    include_str!("../../../../crawrad-runtime-makepad/src/scheme_preview.rs"),
);

const MAKEPAD: &str = concat!(
    include_str!("../../ide_app/theme.rs"),
    include_str!("../../ide_app/theme/tokens.rs"),
    include_str!("../../ide_app/theme/widgets.rs"),
    include_str!("../../ide_app/theme/class_icons.rs"),
    include_str!("../../ide_app/theme/workspace.rs"),
    include_str!("../../ide_app/theme/inspector_widgets.rs"),
    include_str!("../../ide_app/theme/dialog_templates.rs"),
    include_str!("../../ide_app/theme/explorer_dock.rs"),
    include_str!("../../ide_app/dialogs/dsl.rs"),
    include_str!("../../ide_app/dialogs/dsl/editor_dialogs.rs"),
    include_str!("../../ide_app/dialogs/dsl/options_dialog.rs"),
    include_str!("../../ide_app/dialogs/dsl/project_dialogs.rs"),
    include_str!("../../ide_app/dialogs/dsl/tools_dialogs.rs"),
    include_str!("../../ide_app/panels.rs"),
    include_str!("../../ide_app/panels/side_panels.rs"),
    include_str!("../../ide_app/panels/design.rs"),
    include_str!("../../ide_app/panels/chat.rs"),
    include_str!("../../ide_app/panels/terminal.rs"),
    include_str!("../../../../makepad-code-editor/src/minimap.rs"),
    include_str!("../../ide_app/dsl_main.rs"),
);
/// The Makepad shell, which consumes the tokens.
use super::casca::CASCA;

/// The `#RRGGBB` of a Makepad DSL token, as `u32`.
fn makepad_color(token: &str) -> u32 {
    let agulha = format!("\n    pub {token} = #");
    let pos = agulha_unica(MAKEPAD, &agulha, token) + agulha.len();
    u32::from_str_radix(&MAKEPAD[pos..pos + 6], 16)
        .unwrap_or_else(|_| panic!("`{token}` does not have six hexadecimal digits"))
}

fn agulha_unica(fonte: &str, agulha: &str, token: &str) -> usize {
    let pos = fonte
        .find(agulha)
        .unwrap_or_else(|| panic!("token `{token}` was not declared in the Makepad theme"));
    assert!(
        fonte[pos + agulha.len()..].find(agulha).is_none(),
        "token `{token}` was declared twice"
    );
    pos
}

/// Each color role of the Makepad shell and the `Paleta::ESCURA` field that
/// defines it. The token name is the field name in uppercase, on purpose:
/// reading one side reveals the other without a lookup table.
const CORES: &[&str] = &[
    "GROUND",
    "PAINEL",
    "ELEVADO",
    "FLUTUANTE",
    "CONTROLE",
    "LISTRA",
    "DIVISOR",
    "BORDA_CONTROLE",
    "BORDA_FOCO",
    "TEXTO_DESABILITADO",
    "TEXTO_TERCIARIO",
    "TEXTO_SECUNDARIO",
    "TEXTO_PRIMARIO",
    "TEXTO_ENFASE",
    "SELECAO",
    "SELECAO_FORTE",
    "ACENTO_BORDA",
    "ACENTO",
    "ACENTO_TEXTO",
    "ACENTO_FOCO",
    "ERRO",
    "AVISO",
];

/// **Every color role exists in the theme, and only once.**
///
/// # What it replaced, and why
///
/// Until ADR 0032 this list served as a key to compare Makepad's DSL
/// with the Rust `Paleta::ESCURA` of the egui shell. The shell was removed; the list remained,
/// because it is the thing that was **not** specific to egui: it is the set of ROLES that the
/// IDE has — ground, panel, elevated, stripe, accent, error, warning — and this
/// set belongs to the product.
///
/// What it catches now are the two defects that survived the removal:
///
/// - **a role deleted from the theme.** The DSL does not complain about missing tokens:
///   `live_expand` continues, the widget is instantiated and disappears from the screen, with a
///   log line (`makepad-live-compiler-1.0.0/src/live_cx.rs:153-166`);
/// - **a role declared TWICE**, which is what `agulha_unica` catches — with two
///   values, reading order wins, and the screen depends on
///   where the line was pasted.
#[test]
fn every_color_role_exists_in_theme_once() {
    for papel in CORES {
        // `cor_do_makepad` panics with the appropriate message when the token
        // does not exist or repeats — reading the value IS the test.
        let cor = makepad_color(&format!("CRAWRAD_{papel}"));
        assert!(
            cor <= 0xFFFFFF,
            "`CRAWRAD_{papel}` is not a #RRGGBB: {cor:#x}"
        );
    }
}

/// **No color role draws the same shade as another.**
///
/// Two roles with the same value are two names for one decision, and the second
/// can no longer change independently — which is exactly what tokens exist to
/// allow. The declared exception is the accent/border pair, which the theme chose
/// to keep close intentionally.
#[test]
fn no_color_role_duplicates_another() {
    let mut vistos: Vec<(&str, u32)> = Vec::new();
    for papel in CORES {
        let cor = makepad_color(&format!("CRAWRAD_{papel}"));
        if let Some((outro, _)) = vistos.iter().find(|(_, c)| *c == cor) {
            panic!(
                "`CRAWRAD_{papel}` and `CRAWRAD_{outro}` are the same color ({cor:#08x}).                  Two names for one decision: the second can no longer change                  alone, which is what a token is for"
            );
        }
        vistos.push((papel, cor));
    }
}

/// When the mockup is removed, the count drops to zero on its own and the test stays green —
/// it does not require a canvas to exist, it requires that chrome does not go back to hardcoded
/// colors.
#[test]
fn no_hardcoded_color_remains_outside_canvas_mockup() {
    let ini = CASCA
        .find("designer_canvas = ")
        .expect("`designer_canvas` disappeared from ide_app.rs; review boundary of this test");
    let fim = CASCA
        .find("code_container = ")
        .expect("`code_container` disappeared from ide_app.rs; review boundary of this test");
    assert!(ini < fim, "mockup stopped coming before code editor");

    let mut fora = Vec::new();
    let mut busca = 0;
    while let Some(rel) = CASCA[busca..].find(": #") {
        let pos = busca + rel;
        busca = pos + 3;
        let digitos = CASCA[pos + 3..]
            .chars()
            .take_while(|c| c.is_ascii_hexdigit())
            .count();
        if digitos < 3 {
            continue;
        }
        if pos < ini || pos > fim {
            let linha = CASCA[..pos].lines().count();
            fora.push(format!("line {linha}: {}", &CASCA[pos..pos + 3 + digitos]));
        }
    }

    assert!(
        fora.is_empty(),
        "shell chrome has hand-written color again, outside canvas \
         mockup. All chrome color comes from src/ide_app/tema.rs:\n  {}",
        fora.join("\n  ")
    );
}

/// Every token and template name referenced by the shell must exist in the theme.
///
/// This is the only test here that tackles Makepad's core trap: a
/// misspelled name in the DSL **does not break the build**. `(CRAWRAD_PAINELL)` compiles, expands,
/// and instantiates; the property disappears from the screen and leaves a log line. Here it
/// becomes a test failure in 0 s.
#[test]
fn every_token_referenced_by_shell_exists_in_theme() {
    let mut vistos: Vec<String> = Vec::new();
    let bytes: Vec<char> = CASCA.chars().collect();
    let mut i = 0;
    while i < bytes.len() {
        // An identifier starts where the preceding character is not an
        // identifier character — otherwise `MEU_CRAWRAD_X` would match `CRAWRAD_X`.
        let inicio_de_palavra = i == 0 || !(bytes[i - 1].is_alphanumeric() || bytes[i - 1] == '_');
        if inicio_de_palavra {
            let resto: String = bytes[i..].iter().take(64).collect();
            if resto.starts_with("CRAWRAD_") || resto.starts_with("Crawrad") {
                let nome: String = resto
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                if !vistos.contains(&nome) {
                    vistos.push(nome);
                }
            }
        }
        i += 1;
    }

    assert!(
        !vistos.is_empty(),
        "shell does not cite a single theme token — either it has loose \
         colors again, or this test stopped seeing the file"
    );

    for nome in &vistos {
        assert!(
            MAKEPAD.contains(&format!("pub {nome} = "))
                || PALETTE.contains(&format!("pub {nome} = "))
                // an environment variable, not a theme token
                || nome == "CRAWRAD_INSTANCE"
                // a Rust accessor trait `derive(Widget)` generates, not a DSL name
                || (nome.ends_with("WidgetRefExt") && PALETTE.contains("derive(Live, LiveHook, Widget)")),
            "shell cites `{nome}`, which does not exist in `ide_app/tema.rs` \
             or `ide_app/dialogos/dsl.rs`. In Makepad DSL this does NOT \
             break the build: value is ignored and leaves a log line."
        );
    }
}

/// On screen a check box IS its border colour (the fill does not paint
/// over it at that size), so a focus border in the accent read as "checked"
/// on an unchecked box. Focus takes the hover colour; only `active` is accent.
#[test]
fn a_focused_check_box_does_not_look_checked() {
    let src = include_str!("../../ide_app/theme/inspector_widgets.rs");
    let start = src.find("pub CrawradPropCheck").expect("CrawradPropCheck");
    let block = &src[start..start + src[start..].find("text: \"\"").expect("end")];
    for side in ["border_color_1_focus", "border_color_2_focus"] {
        let line = block.lines().find(|l| l.contains(side)).expect(side);
        assert!(
            !line.contains("ACENTO"),
            "{side} must not be the accent: {line}"
        );
    }
}
