//! The gate that prevents the Toolbox from returning to having TWO maintenance points.
//!
//! The defect that these tests lock: the name of each component was
//! written **twice** — once in `live_design!`, as `item_btn =
//! <CrawradListRow> { text: "Button" }`, and again in a fourteen-entry table
//! in `handle_actions`, linking the row id back to the class. Adding a
//! component cost editing two places, and forgetting the second was
//! **silent**: the row appeared on screen and armed nothing. Three rows
//! were exactly like that (`ActionList`, `ImageList`, `Timer`).
//!
//! Two tests are TEXT tests, and deliberately so: what needs to stay locked is
//! the *shape* of the file, not just the in-memory result. A correct palette
//! rewritten by hand would pass the behavioral tests and bring the defect
//! back.

use crawrad_registry::Registry;

use super::palette::{self, Row};

/// The Makepad shell, as text.
use super::casca::CASCA;

/// The Makepad shell target — the same one `linhas_da_paleta` uses.
const ALVO: &str = crawrad_runtime_makepad::ALVO;

/// The DSL snippet that draws the Toolbox.
///
/// The boundaries are names declared by the DSL itself; if one disappears, the
/// test fails reporting that, instead of passing by inspecting the wrong file — the
/// same technique as `nenhuma_cor_a_mao_sobrou_fora_da_maquete_do_canvas`.
fn toolbox_snippet() -> &'static str {
    let ini = CASCA
        .find("toolbox_palette = ")
        .expect("`toolbox_palette` vanished from ide_app.rs; review the boundary of this test");
    let resto = &CASCA[ini..];
    let fim = resto
        .find("sep_left_mid = ")
        .expect("`sep_left_mid` vanished from ide_app.rs; review the boundary of this test");
    &resto[..fim]
}

/// The in-memory palette is exactly the catalog, group by group and in order.
#[test]
fn palette_rows_are_flattened_catalog() {
    let reg = Registry::builtin();
    let esperado = reg.palette(ALVO);
    assert!(
        !esperado.is_empty(),
        "the catalog returned no groups for target `{ALVO}`"
    );

    let mut linhas = palette::rows(&reg, ALVO, "", false).into_iter();
    for (grupo, itens) in &esperado {
        assert_eq!(
            linhas.next(),
            Some(Row::Section(grupo)),
            "group `{grupo}` did not open with its label"
        );
        for d in itens {
            assert_eq!(
                linhas.next(),
                Some(Row::Item {
                    class: d.class,
                    label: d.display,
                    blocked: None,
                }),
                "the row for `{}` came out out of order or with a different label",
                d.class
            );
        }
    }
    assert_eq!(
        linhas.next(),
        None,
        "the palette has a row that the catalog did not request"
    );
}

/// Every component row arms a class; every section label arms
/// nothing.
///
/// It is what separates a useful click from a dead click, and `handle_actions`
/// depends on this: it arms `Linha::classe()`.
#[test]
fn only_item_stores_a_class() {
    let reg = Registry::builtin();
    for linha in palette::rows(&reg, ALVO, "", false) {
        match linha {
            Row::Section(_) => {
                assert_eq!(linha.class(), None, "a section label started arming a tool")
            }
            Row::Empty => {
                panic!("the unfiltered palette reported no matches with a full catalog")
            }
            Row::Item { class, .. } => {
                assert_eq!(linha.class(), Some(class));
                assert!(
                    reg.get(class).is_some(),
                    "the palette offers `{class}`, which is not in the catalog"
                );
            }
        }
    }
}

/// The three rows that the manual table forgot now arm.
///
/// They were in the DSL and **outside** the `handle_actions` table: they appeared on
/// screen and clicking did nothing. This test is what prevents the hole from
/// returning via another route.
#[test]
fn rows_forgotten_by_manual_table_now_arm() {
    let reg = Registry::builtin();
    let armaveis: Vec<&'static str> = palette::rows(&reg, ALVO, "", false)
        .iter()
        .filter_map(Row::class)
        .collect();
    for esquecida in ["ActionList", "ImageList", "Timer"] {
        assert!(
            armaveis.contains(&esquecida),
            "`{esquecida}` became a row that arms nothing again"
        );
    }
}

/// The Toolbox DSL does not hardcode any component name.
///
/// This is the test that locks the defect: with it, rewriting `item_btn =
/// <CrawradListRow> { text: "Button" }` fails the build. What the DSL may contain
/// is **style** — the `Secao`, `SecaoPrimeira`, and `Item` templates.
#[test]
fn toolbox_dsl_does_not_hardcode_component_names() {
    let trecho = toolbox_snippet();
    let reg = Registry::builtin();

    let mut escritos = Vec::new();
    for (grupo, itens) in reg.palette(ALVO) {
        // Quotes on purpose: comments may mention `Standard` in
        // backticks to explain the template; what cannot happen is the DSL writing the
        // text that goes to the screen.
        if trecho.contains(&format!("\"{grupo}\"")) {
            escritos.push(grupo.to_string());
        }
        for d in itens {
            if trecho.contains(&format!("\"{}\"", d.display)) {
                escritos.push(d.display.to_string());
            }
        }
    }

    assert!(
        escritos.is_empty(),
        "Toolbox went back to hardcoding component names in the DSL. The text \
         of each row comes from the `crawrad-registry` catalog, via `palette::rows`:\n  {}",
        escritos.join("\n  ")
    );
}

// ---------------------------------------------------------------------------
// THE FILTER
//
// The `Filter` field of the Makepad shell existed in the DSL and was dead UI: no one
// read what the user typed. The rule introduced is the SAME as the egui
// Toolbox (`crates/crawrad-ide/src/paleta.rs:470-480`), and these tests enforce
// precisely that — not semantics invented here.
// ---------------------------------------------------------------------------

/// The classes that the palette offers with a given filter.
fn classes_com(filtro: &str) -> Vec<&'static str> {
    let reg = Registry::builtin();
    palette::rows(&reg, ALVO, filtro, false)
        .iter()
        .filter_map(Row::class)
        .collect()
}

/// Blank filter returns the entire catalog — and whitespace is a blank
/// filter.
///
/// `trim` is not a detail: pasting a name with extra whitespace, or erasing what
/// was typed and leaving whitespace, would empty the entire Toolbox.
#[test]
fn blank_filter_returns_full_catalog() {
    let reg = Registry::builtin();
    let tudo = palette::rows(&reg, ALVO, "", false);
    for branco in ["", "   ", "\t", "\n "] {
        assert_eq!(
            palette::rows(&reg, ALVO, branco, false),
            tudo,
            "filter {branco:?} stopped counting as a blank filter"
        );
    }
}

/// Search is by SUBSTRING, at any position, and not by prefix.
///
/// It is what the egui Toolbox does (`contains`, `paleta.rs:478-479`), and it is what
/// lets `box` find `CheckBox`, `ComboBox`, and `ListBox` — the way one
/// searches when not remembering the beginning of the name.
#[test]
fn search_is_by_substring_not_prefix() {
    let achadas = classes_com("box");
    for esperada in ["CheckBox", "ComboBox", "GroupBox", "ListBox"] {
        assert!(
            achadas.contains(&esperada),
            "`box` did not find `{esperada}`; search reverted to prefix matching"
        );
    }
    assert!(
        !achadas.contains(&"Button"),
        "`box` returned `Button`, which does not contain the substring"
    );
}

/// Case of what is typed does not matter, on both sides.
#[test]
fn search_ignores_letter_case() {
    let referencia = classes_com("button");
    assert!(
        referencia.contains(&"Button"),
        "search did not find `Button` even in lowercase"
    );
    for grafia in ["BUTTON", "Button", "bUtToN"] {
        assert_eq!(
            classes_com(grafia),
            referencia,
            "`{grafia}` found something different than `button`"
        );
    }
}

/// What matches is the display name **or** the class.
///
/// Today both are identical across the entire catalog, so the test checks the rule where
/// it can be checked: every class found by a substring has that substring in
/// `display` or in `class`. It is the contract safeguarding the day a
/// label with spaces ("Check Box") is added and search by class must keep
/// working.
#[test]
fn search_matches_label_or_class() {
    let reg = Registry::builtin();
    let busca = "list";
    for linha in palette::rows(&reg, ALVO, busca, false) {
        let Row::Item { class, label, .. } = linha else {
            continue;
        };
        assert!(
            label.to_lowercase().contains(busca) || class.to_lowercase().contains(busca),
            "`{class}` entered the list without `{busca}` in the label or class"
        );
    }
}

/// A section emptied by the filter does not become an orphan header — and none remains
/// without an item underneath.
///
/// It was the most visible defect of a filter implemented during drawing: the three group
/// labels would remain on screen, stacked, with only one row under one
/// of them.
#[test]
fn empty_filtered_section_does_not_appear() {
    let reg = Registry::builtin();
    // `treeview` only exists in one group; the others must disappear entirely.
    let linhas = palette::rows(&reg, ALVO, "treeview", false);
    assert!(
        linhas.iter().any(|l| l.class() == Some("TreeView")),
        "`treeview` filter did not find `TreeView`"
    );

    let mut anterior_era_secao = false;
    for (i, linha) in linhas.iter().enumerate() {
        let e_secao = matches!(linha, Row::Section(_));
        assert!(
            !(anterior_era_secao && e_secao),
            "two consecutive sections at position {i}: the first was left with no items"
        );
        anterior_era_secao = e_secao;
    }
    assert!(
        !anterior_era_secao,
        "the filtered list ends with a section label having no items"
    );
}

/// A filter that matches nothing does not leave the list blank: it says so.
///
/// And the phrase is the SAME across both shells — `NADA_CASOU` is read from the module, not
/// rewritten here, otherwise the test would become a second copy of the text.
#[test]
fn filter_with_no_results_shows_empty_list_row() {
    let reg = Registry::builtin();
    let linhas = palette::rows(&reg, ALVO, "zzqq", false);
    assert_eq!(
        linhas,
        vec![Row::Empty],
        "a filter with no results returned something other than the empty list row"
    );
    assert_eq!(linhas[0].text(), palette::NO_MATCH);
    assert_eq!(
        linhas[0].class(),
        None,
        "the empty list row started arming a tool"
    );
}

/// The Toolbox `Filter` field is READ by something.
///
/// This is the defect test: until this cycle, `grep filter_input` returned
/// **one** line — that of the DSL itself. The field was on screen, accepted text, and
/// had no reader. A correct list wired to a field that no one reads passes
/// every behavioral test, so what needs to stay locked is the
/// wiring.
///
/// The search is in the file with no whitespace at all so that whatever line wrapping
/// `rustfmt` picks does not break the test.
#[test]
fn toolbox_filter_field_is_not_dead_ui() {
    let compacto: String = CASCA.chars().filter(|c| !c.is_whitespace()).collect();
    assert!(
        compacto.contains("text_input(id!(toolbox_filter.filter_input))"),
        "nothing reads `filter_input` from the DSL; the `Filter` field reverted to dead UI"
    );
    assert!(
        compacto.contains(".changed(actions)"),
        "new text in the field is not read by `TextInputRef::changed` \
          (`makepad-widgets-1.0.0/src/text_input.rs:1843`), which is the only channel"
    );
    assert!(
        compacto.contains("self.filtro=texto"),
        "field text does not reach `IdeApp` state, and drawing the list \
         has nowhere to get it from"
    );
    assert!(
        compacto.contains(
            "palette::rows(r,crawrad_project::generator::target_key(),&self.filtro,web,)"
        ),
        "Toolbox list stopped being built with the stored filter"
    );
}

/// No Toolbox row has its own id in the DSL.
///
/// The id was the other half of duplication: `item_btn` appeared in the DSL and again
/// in the Rust table. With the list originating from the catalog, the `PortalList` index
/// identifies the row, and there is no id to repeat.
#[test]
fn no_toolbox_row_has_own_id() {
    let achados: Vec<String> = CASCA
        .lines()
        // The filter is `item_btn`/`item_nome`/`item_icone` — the ids that the Toolbox
        // had. `starts_with("item_")` alone was too broad: it rejected
        // `item_text` from the Items Editor, which is the `Text:` field of a
        // COLLECTION item and has nothing to do with the Toolbox (measured on 17/09/2026).
        .filter(|l| {
            l.contains("toolbox_palette.item_")
                || l.trim_start().starts_with("item_btn")
                || l.trim_start().starts_with("item_nome")
                || l.trim_start().starts_with("item_icone")
                || l.trim_start().starts_with("item_classe")
        })
        .map(|l| l.trim().to_string())
        .collect();
    assert!(
        achados.is_empty(),
        "Toolbox row ids returned — one id per component is the second \
         list to forget to update:\n  {}",
        achados.join("\n  ")
    );
}

// ---------------------------------------------------------------------------
// THE GESTURE — and the framework defect that it cost
// ---------------------------------------------------------------------------

/// **The palette ARMS on `pressed`, and never on `clicked`.**
///
/// # The `clicked` of a Toolbox row DOES NOT HAPPEN, and this was measured
///
/// The same row that arms also starts the drag (`cx.start_dragging`), and on
/// Windows `CxOsOp::StartDragging` calls **synchronous** `DoDragDrop`
/// (`makepad-platform-1.0.0/src/os/windows/win32_app.rs:450`): an OLE modal loop
/// that holds the mouse button until released. `FingerUp` never returns to
/// `Button`, so `ButtonAction::Clicked` is never emitted.
///
/// While arming depended on `clicked`, **clicking the palette armed
/// nothing** — with both native and third-party components alike. The Toolbox said
/// *"click a component, then click the form"* and the click did
/// nothing at all. It was the central gesture of a classic RAD IDE, and it was dead.
///
/// This test is a TEXT test because what needs to stay locked is the choice: a
/// `clicked` returning here compiles, passes every behavioral test that
/// does not open a window, and brings the entire defect back.
#[test]
fn palette_arms_on_pressed_because_clicked_never_fires() {
    // Without comments, for the same reason as sibling tests: a commented line is not
    // code, and `contains` on the raw block cannot tell the difference.
    let trecho: String = toolbox_gesture_snippet()
        .lines()
        .map(str::trim)
        .filter(|l| !l.starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        trecho.contains("if alvo.pressed(actions) {"),
        "Toolbox stopped arming on `pressed`; `clicked` on a row that \
         triggers `start_dragging` never occurs on Windows"
    );
    assert!(
        !trecho.contains("alvo.clicked(actions)"),
        "`alvo.clicked(actions)` returned to Toolbox. It is NEVER emitted \
         on that line — Windows `DoDragDrop` is synchronous and swallows the button — and \
         the effect is clicking the palette arms nothing, silently."
    );
}

/// **And drop DISARMS.**
///
/// The twin of the test above, and it completes the pair. Because both gestures
/// originate from the same `FingerDown`, dragging also arms: without disarming, dropping on
/// the form would create the component **and leave the tool armed** — the next
/// click would create a second one that nobody asked for.
#[test]
fn releasing_drag_disarms_tool() {
    let d = CASCA
        .find("DragHit::Drop(f) => {")
        .expect("drop arm in the canvas");
    let ate = CASCA[d..]
        .find("\n            _ => {}")
        .map(|i| d + i)
        .unwrap_or(CASCA.len());
    let braco = &CASCA[d..ate];
    // **Lines are read WITHOUT COMMENTS.**
    //
    // `contains` on the raw block answered yes to a
    // `// self.tool_armed = None;` — measured by mutation: commenting the line
    // left the guard GREEN. A guard that measures wrongly is worse than none.
    let codigo: Vec<&str> = braco
        .lines()
        .map(str::trim)
        .filter(|l| !l.starts_with("//"))
        .collect();
    assert!(
        codigo
            .iter()
            .any(|l| l.contains("self.colocar(cx, &classe, x, y);")),
        "drop stopped creating the component"
    );
    assert!(
        codigo.iter().any(|l| l.contains("self.tool_armed = None;")),
        "drop creates without disarming: the next click on the form would create a \
         second component that no one requested"
    );
}

/// The `handle_actions` block handling the Toolbox, as text.
///
/// Sibling of [`trecho_da_toolbox`], which clips the DSL: this clips the GESTURE.
fn toolbox_gesture_snippet() -> &'static str {
    let i = CASCA
        .find("if toolbox.any_items_with_actions(actions) {")
        .expect("Toolbox block in `handle_actions`");
    let j = CASCA[i..]
        .find("// O OBJECT INSPECTOR:")
        .map(|k| i + k)
        .unwrap_or(CASCA.len());
    &CASCA[i..j]
}
