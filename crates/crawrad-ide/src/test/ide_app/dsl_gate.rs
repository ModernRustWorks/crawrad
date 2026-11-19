//! The headless DSL gate **for the shell** — the counterpart of
//! `crates/crawrad-runtime-makepad/tests/dsl_gate.rs`, with the same mechanism and the
//! same purpose.
//!
//! # Why it exists
//!
//! In Makepad 1.0.0, an error inside `live_design!` **does not break
//! compilation**: the DSL is a string that `rustc` never reads. The error only appears
//! at runtime, and even then as a log line — the widget
//! keeps instantiating (`makepad-platform-1.0.0/src/live_cx.rs:153-166`,
//! `error!` → `println!` in `log.rs:103`).
//!
//! The shell has ~500 lines of DSL and, until this round, **no** gate over
//! them: the tests in neighboring `test.rs` compare THEME tokens, and state in
//! writing that they do not catch wrong property names. This file catches them.
//!
//! # How it works
//!
//! Three stages, matching the adapter gate:
//!
//! 1. **expansion**, via the structured channel
//!    (`LiveRegistry::expand_all_documents(&mut errs)` —
//!    `makepad-live-compiler-1.0.0/src/live_registry.rs:945`), which returns
//!    `Vec<LiveError>` instead of a loose `println!`;
//! 2. **instantiation** of the entire `IdeApp` — and here there is no structured
//!    channel, only the log. Therefore, the probe runs in a **child process** and the
//!    parent reads its `stdout`: `libtest` diverts `println!` from the process
//!    itself into an inaccessible buffer;
//! 3. **instantiating templates** for the two `PortalList`s in the left panel
//!    — the Toolbox and the structure tree.
//!    Measured in the adapter gate: a block stored as a template is only applied
//!    when someone instantiates it (`portal_list.rs:563`) — an uninstantiated template
//!    is an unverified template.
//!
//! # What it does NOT catch
//!
//! A valid yet wrong color, a valid yet wrong layout, and a shader that compiles and
//! renders incorrectly pass green. It answers *"the DSL is well-formed"*, never
//! *"the screen is correct"* — and covers exactly what the probe instantiates.

use std::process::Command;

use makepad_widgets::*;

use super::IdeApp;

/// Variable that authorizes the probe to run (only the parent process defines it).
const VAR_SONDA: &str = "CRAWRAD_DSL_GATE_CASCA";

/// Mark printed by the probe when it reaches the end without panicking.
const MARCA_OK: &str = "<<<CRAWRAD-CASCA-GATE-OK>>>";

/// Mark with which the probe reports an already structured expansion error.
const MARCA_EXPANSAO: &str = "<<<CRAWRAD-CASCA-GATE-EXPANSAO>>>";

/// Every textual mark that Makepad uses to report a DSL error.
///
/// The list matches the adapter gate, and the locations are there; it is
/// repeated because `crawrad-ide` is a BINARY crate and there is no way
/// to share test code between the two.
const MARCAS_DE_ERRO: &[&str] = &[
    MARCA_EXPANSAO,
    "Apply error:",
    "Apply without file",
    // Covers `Error expanding live file`, `Error parsing live file` and the
    // `log!("Error {}", err)` of the shader compiler (`draw_vars.rs:312`).
    "- Error ",
    "Cannot find main file id",
    "Could not load resource",
];

/// The three widgets that a row with an icon MUST have.
///
/// Without this, a `nomee` in the DSL would return `WidgetRef::empty()` silently and the
/// Toolbox — or the tree — would appear with the right icon and a blank name. It is the
/// same family of defect that the Object Inspector checks below.
fn conferir_linha_com_icone(item: &WidgetRef, onde: &str) {
    assert!(
        item.button(id!(alvo)).borrow().is_some(),
        "the row for {onde} does not have the click target in `alvo` — without it, \
         clicking on the row emits no action"
    );
    assert!(
        !item.view(id!(face.icone)).is_empty(),
        "the row for {onde} does not have the icon in `face.icone`"
    );
    for campo in [id!(face.nome), id!(face.classe)] {
        assert!(
            item.label(campo).borrow().is_some(),
            "the row for {onde} does not have the label {campo:?}"
        );
    }
}

/// The path of a menu button on the bar.
///
/// Mirrors `IdeApp::menu_button`, which is private — and by design the
/// mirror is an exhaustive `match`: a new menu in the enum does not compile until it has a
/// path here as well.
fn menu_button_id(m: super::menu::Menu) -> &'static [LiveId] {
    match m {
        super::menu::Menu::File => id!(top_titlebar.menu_file),
        super::menu::Menu::Edit => id!(top_titlebar.menu_edit),
        super::menu::Menu::View => id!(top_titlebar.menu_view),
        super::menu::Menu::Go => id!(top_titlebar.menu_go),
        super::menu::Menu::Refactor => id!(top_titlebar.menu_refactor),
        super::menu::Menu::Project => id!(top_titlebar.menu_project),
        super::menu::Menu::Run => id!(top_titlebar.menu_run),
        super::menu::Menu::Component => id!(top_titlebar.menu_component),
        super::menu::Menu::Tools => id!(top_titlebar.menu_tools),
        super::menu::Menu::Help => id!(top_titlebar.menu_help),
    }
}

fn colher_erros(texto: &str) -> Vec<String> {
    texto
        .lines()
        .filter(|linha| MARCAS_DE_ERRO.iter().any(|marca| linha.contains(marca)))
        .map(|linha| linha.trim().to_string())
        .collect()
}

/// The probe: builds a headless `Cx`, expands the shell DSL, and instantiates the
/// entire `IdeApp` — window, docks, canvas, and tree templates.
#[test]
#[ignore = "gate probe; parent runs it in a child process"]
fn sonda_da_casca() {
    // The probe only makes sense inside the child process launched by the gate. With
    // `--include-ignored` it runs directly, and `#[ignore]` does NOT protect against the
    // flag that exists specifically to disable it. Exiting silently is what keeps the
    // gate green — failing here would leave
    // `cargo test --workspace -- --include-ignored` red forever.
    if std::env::var(VAR_SONDA).is_err() {
        return;
    }

    let mut cx = Cx::new(Box::new(|_, _| {}));

    // Without this, `expand_all_documents` panics on the `unwrap()` of `main_module`
    // (`live_registry.rs:952`).
    IdeApp::register_main_module(&mut cx);

    // A single call: the shell's `live_design!` calls `live_design_with` for
    // each type BEFORE registering its own body
    // (`makepad-derive-live-1.0.0/src/live_design_macro.rs:29-30`), and it is the
    // `live_design_with` of `IdeApp` that passes through `LiveRegister` — that is,
    // through `makepad_widgets`, the theme, and `crawrad-runtime-makepad`. If the
    // registration order is wrong, it is wrong here too.
    super::live_design(&mut cx);

    // ---- stage 1: expansion, with errors in hand ----
    let mut erros = Vec::new();
    {
        let registro = cx.live_registry.clone();
        let mut registro = registro.borrow_mut();
        registro.expand_all_documents(&mut erros);
        for erro in erros.drain(..) {
            println!(
                "{MARCA_EXPANSAO} {}",
                registro.live_error_to_live_file_error(erro)
            );
        }
    }

    // Fonts, without which any `text_style` panics headless
    // (`makepad-draw-1.0.0/src/shader/draw_text.rs:599`). Both methods are
    // public and this is the same order as the native backend
    // (`makepad-platform-1.0.0/src/os/windows/windows.rs:430-431`).
    cx.live_scan_dependencies();
    cx.native_load_dependencies();

    // ---- stage 2: instantiation ----
    let Some(app) = IdeApp::new_main(&mut cx) else {
        panic!("IdeApp was not found in the expanded DSL");
    };

    // The canvas from ADR 0027 must be there, and must be a real
    // `DesignCanvas`: if `use link::crawrad::*` disappears, `<DesignCanvas>` becomes an
    // unknown widget and what remains is a hole on the screen.
    let canvas = app.ui.widget(id!(designer_row.designer_canvas.canvas));
    assert!(
        canvas
            .borrow::<crawrad_runtime_makepad::DesignCanvas>()
            .is_some(),
        "the shell's `canvas` is not a DesignCanvas"
    );

    // ---- stage 2b: the FIVE dock panel templates ----
    //
    // With the dock (ADR 0014, Amendment 9), the panel is no longer in the window
    // DSL: it is a TEMPLATE that `Dock` instantiates when the tab exists. The
    // default layout opens three — `Components`, `Design`, and `Chat` —, and the
    // other three are created only when requested.
    //
    // An uninstantiated template is an UNVERIFIED template, and here that is literal:
    // without these lines, an error inside `PanelInspector` would pass the entire
    // gate silently, because the Inspector does not open by itself. It is also what
    // allows the `portal_list(id!(...))` below to find anything:
    // `Dock::find_widgets` only traverses items that EXIST (`dock.rs:185`).
    let doca = app.ui.dock(super::layout::DOCK);
    for p in crawrad_actions::Panel::ALL {
        let painel = doca.item_or_create(
            &mut cx,
            super::layout::tab_of(*p),
            super::layout::template_of(*p),
        );
        assert!(
            painel.is_some_and(|w| !w.is_empty()),
            "panel `{}` did not become a widget: either the template is not in the dock DSL, or `template_of` points to a non-existent name — and in both cases the tab would open EMPTY, without crashing",
            p.id()
        );
    }

    // ---- stage 3: templates of the two left panel lists ----
    //
    // An uninstantiated template is an UNVERIFIED template — the rule is at the top
    // of this file, and applies line by line: adding a template to the DSL without
    // adding the name here is frozen coverage.
    let arvore = app.ui.portal_list(id!(structure_tree.structure_list));
    // TWO TEMPLATES, and there were four until 2026-09-17.
    //
    // Selection no longer swaps the row template: what marks it now is
    // `visto`, a label inside `CrawradIconRow` itself. The two templates that
    // were removed — `LinhaSel` and `RaizSel` — were not forgotten here: they no
    // longer exist, and instantiating a name absent from the DSL returns an empty
    // widget, which is exactly what this probe exists to catch.
    for (i, molde) in [live_id!(Linha), live_id!(Raiz)].into_iter().enumerate() {
        let item = arvore.item(&mut cx, i, molde);
        assert!(
            !item.is_empty(),
            "tree template {i} did not become a widget"
        );
        conferir_linha_com_icone(&item, "the tree");
    }

    let toolbox = app.ui.portal_list(id!(toolbox_palette.toolbox_list));
    for (i, molde) in [
        live_id!(Secao),
        live_id!(SecaoPrimeira),
        live_id!(Item),
        live_id!(Vazia),
    ]
    .into_iter()
    .enumerate()
    {
        let item = toolbox.item(&mut cx, i, molde);
        assert!(
            !item.is_empty(),
            "Toolbox template {i} did not become a widget"
        );
    }
    conferir_linha_com_icone(&toolbox.item(&mut cx, 2, live_id!(Item)), "the Toolbox");

    // ---- stage 3b: the two MENU templates ----
    //
    // The menu panel is the fifth list of the shell, and the same rule applies: an
    // uninstantiated template is an unverified template. Both have the click target
    // underneath and two labels on top — if one of these `id`s disappears from the DSL,
    // the menu opens with blank rows and nothing crashes.
    let popup = app.ui.portal_list(id!(menu_popup.menu_list));
    for (i, molde) in [live_id!(Comando), live_id!(ComandoCinza)]
        .into_iter()
        .enumerate()
    {
        let item = popup.item(&mut cx, i, molde);
        assert!(
            !item.is_empty(),
            "menu template {i} did not become a widget"
        );
        assert!(
            item.button(id!(alvo)).borrow().is_some(),
            "menu row {i} does not have the click target in `alvo` — without it, \
             clicking the item emits no action, silently"
        );
        for campo in [id!(face.rotulo), id!(face.atalho)] {
            assert!(
                item.label(campo).borrow().is_some(),
                "menu row {i} does not have the label {campo:?}"
            );
        }
    }
    assert!(
        !app.ui.view(id!(menu_popup)).is_empty(),
        "the menu panel vanished from the DSL"
    );
    for m in super::menu::Menu::ALL {
        assert!(
            app.ui.button(menu_button_id(*m)).borrow().is_some(),
            "the menu button `{}` is not a Button on the bar",
            m.title()
        );
    }

    // ---- stage 4: Object Inspector templates ----
    //
    // These contain CONTROLS inside the row — text field, checkbox,
    // and combo —, and are therefore the ones with the most DSL property
    // names to get wrong: `CrawradPropCheck` alone writes twenty colors for
    // `CheckBox`'s `draw_bg`, and a missing `mark_color_activ` would pass
    // `cargo check`, pass expansion, and disappear from the screen.
    //
    // `CrawradPropDrop` pulls along the entire `PopupMenu`: `DropDown`'s
    // `after_apply` instantiates the menu when application comes from the document
    // (`drop_down.rs:646-659`), so the menu style is also checked
    // here.
    //
    // Each template is instantiated at its own INDEX: `PortalList` keeps one item per
    // index and swaps the widget when the template changes (`portal_list.rs:566-575`),
    // so reusing index zero would only verify the last item in the list.
    let inspetor = app.ui.portal_list(id!(inspector_panel.inspector_list));
    // THE THREE TEMPLATES NOT COMING FROM AN EDITOR — category, notice, and event
    // row. They are the only names written here.
    for (i, molde) in [live_id!(Category), live_id!(Notice), live_id!(RowEvent)]
        .into_iter()
        .enumerate()
    {
        let item = inspetor.item(&mut cx, i, molde);
        assert!(
            !item.is_empty(),
            "Object Inspector template {i} did not become a widget"
        );
    }

    // The EVENT row has a code field at `value` — it receives
    // the handler name and emits `event.bind`.
    let evento_row = inspetor.item(&mut cx, 2, live_id!(RowEvent));
    assert!(
        evento_row.text_input(id!(value)).borrow().is_some(),
        "the Inspector event row does not have a TextInput in `value`"
    );
    assert!(
        evento_row.label(id!(name)).borrow().is_some(),
        "the Inspector event row does not have the label `name`"
    );

    // AND THE REMAINING ONES COME FROM THE EDITORS THEMSELVES — the names are NOT written
    // here, they come from `PropEditor::template`, by the same rule as the Explorer
    // below. Written twice, this gate would stay green on a name
    // that the design no longer uses, which is frozen coverage.
    //
    // Each sample is instantiated at its own INDEX, away from the ones above:
    // `PortalList` keeps one item per index and swaps the widget when the template
    // changes (`portal_list.rs:566-575`), so reusing an index would verify only the
    // last template that passed through it.
    for (i, editor) in super::inspector::editors::samples().iter().enumerate() {
        let molde = LiveId::from_str(editor.template());
        let item = inspetor.item(&mut cx, 100 + i, molde);
        assert!(
            !item.is_empty(),
            "Object Inspector template `{}` did not become a widget — the row \
             would come out blank, without crashing",
            editor.template()
        );
        assert!(
            item.label(id!(name)).borrow().is_some(),
            "template `{}` does not have the label `name` — the row would come out without \
             the property name",
            editor.template()
        );
        // And each CELL declared by the editor must find a widget OF THE RIGHT
        // TYPE in the template. A `valorr` in the DSL would return `WidgetRef::empty()`
        // silently, and the Inspector would end up with an invisible control — which is
        // exactly the dead field this panel exists to prevent.
        for celula in editor.cells() {
            let caminho = super::slot_path(celula.slot());
            let achou = match &celula {
                super::inspector::Cell::Field { .. } => item.text_input(caminho).borrow().is_some(),
                super::inspector::Cell::Check { .. } => item.check_box(caminho).borrow().is_some(),
                super::inspector::Cell::Choice { .. } => item.drop_down(caminho).borrow().is_some(),
                super::inspector::Cell::Caption { .. } => item.label(caminho).borrow().is_some(),
                super::inspector::Cell::Press { .. } => item.button(caminho).borrow().is_some(),
                super::inspector::Cell::Swatch { .. } => !item.view(caminho).is_empty(),
            };
            assert!(
                achou,
                "template `{}` does not have the widget that cell {:?} requests in \
                 {caminho:?} — the control would not draw, and nothing would complain",
                editor.template(),
                celula.slot()
            );
        }
    }

    // ---- stage 4b: the TWO Inspector DIALOGS ----
    //
    // They are not created from templates: they are body views, hidden until
    // requested. A wrong `id` in one of the fields below returns `WidgetRef::empty()`
    // and the window opens without the field, without anything crashing.
    // The ITEMS EDITOR and LINES EDITOR, via the paths written in Rust.
    let items = app.ui.view(id!(items_dialog.dlg));
    for rotulo in [
        &id!(items_title)[..],
        &id!(items_owner)[..],
        &id!(items_label)[..],
        &id!(props_label)[..],
        &id!(text_label)[..],
        &id!(image_label)[..],
        &id!(props_note)[..],
    ] {
        assert!(
            items.label(rotulo).borrow().is_some(),
            "the Items Editor does not have the label {rotulo:?} — it would open without this word, and nothing would crash"
        );
    }
    assert!(
        items.text_input(id!(item_text)).borrow().is_some(),
        "the Items Editor does not have the item's `Text:` field"
    );
    assert!(
        items.text_input(id!(item_image)).borrow().is_some(),
        "the Items Editor does not have the item's `Image Index:` field"
    );
    assert!(
        items.check_box(id!(item_expanded)).borrow().is_some(),
        "the Items Editor does not have the item's `Expanded` field"
    );
    assert!(
        items.portal_list(id!(items_list)).borrow().is_some(),
        "the Items Editor does not have the tree list"
    );
    for botao in [
        &id!(items_ok)[..],
        &id!(items_cancel)[..],
        &id!(items_apply)[..],
    ] {
        assert!(
            items.button(botao).borrow().is_some(),
            "button {botao:?} in the Items Editor footer vanished from the DSL"
        );
    }
    // The FIVE middle commands, via the shell's TABLE paths — and not via
    // a list written here, which would be a second list that diverges.
    for (caminho, rotulo, _) in super::ITEMS_BOTOES {
        assert!(
            items.button(caminho()).borrow().is_some(),
            "button `{rotulo}` of the Items Editor vanished from the DSL"
        );
    }

    let lines = app.ui.view(id!(lines_dialog.dlg));
    for rotulo in [
        &id!(lines_title)[..],
        &id!(lines_owner)[..],
        &id!(lines_count)[..],
    ] {
        assert!(
            lines.label(rotulo).borrow().is_some(),
            "the Lines Editor does not have the label {rotulo:?}"
        );
    }
    assert!(
        lines.text_input(id!(lines_text)).borrow().is_some(),
        "the Lines Editor does not have the text field — the window would open with nowhere to type"
    );
    for botao in [
        &id!(lines_ok)[..],
        &id!(lines_cancel)[..],
        &id!(lines_apply)[..],
    ] {
        assert!(
            lines.button(botao).borrow().is_some(),
            "button {botao:?} in the Lines Editor footer vanished from the DSL"
        );
    }
    assert!(
        app.ui.button(id!(align_pad.zone_none)).borrow().is_some(),
        "button `None` of the `Align` selector vanished from the DSL"
    );
    // THE FIVE REGIONS of the `Align` selector, via the paths used on click:
    // they come from the shell's table, not from a list written here.
    for (caminho, nome) in super::ALIGN_ZONES {
        assert!(
            app.ui.button(caminho()).borrow().is_some(),
            "the region `{nome}` of the Align selector is not a Button — clicking \
             it would emit no action, silently"
        );
    }

    let popt = app.ui.view(id!(project_options_dialog.dlg));
    assert!(
        popt.label(id!(popt_title)).borrow().is_some(),
        "project_options_dialog lacks popt_title"
    );
    assert!(
        popt.text_input(id!(popt_title_input)).borrow().is_some(),
        "project_options_dialog lacks popt_title_input"
    );
    assert!(
        popt.button(id!(popt_load_btn)).borrow().is_some(),
        "project_options_dialog lacks popt_load_btn"
    );
    assert!(
        popt.button(id!(popt_default_btn)).borrow().is_some(),
        "project_options_dialog lacks popt_default_btn"
    );
    assert!(
        popt.button(id!(popt_ok)).borrow().is_some(),
        "project_options_dialog lacks popt_ok"
    );
    assert!(
        popt.button(id!(popt_cancel)).borrow().is_some(),
        "project_options_dialog lacks popt_cancel"
    );

    // ---- stage 5: the rail and tabs ----
    //
    // The rail button template, by the same rule as stage 3: an uninstantiated
    // template is an unverified template. There used to be TWO — the destination with a panel
    // and the blank one, without —, and became one when the two destinations without screens received
    // theirs.
    //
    // And the two Inspector tabs, which are actual `RadioButton`s: if one
    // of them becomes `WidgetRef::empty()`, `RadioButtonSet` returns `None`
    // forever and the `Events` tab becomes unreachable, silently.
    let trilho = app.ui.portal_list(id!(activity_rail.rail_list));
    let item = trilho.item(&mut cx, 0, live_id!(Destino));
    assert!(
        !item.is_empty(),
        "the rail button template did not become a widget"
    );
    for aba in [
        id!(inspector_tabs.tab_props),
        id!(inspector_tabs.tab_events),
    ] {
        assert!(
            app.ui.radio_button(aba).borrow().is_some(),
            "tab {aba:?} of the Object Inspector is not a RadioButton"
        );
    }
    assert!(
        app.ui.button(id!(rail_settings)).borrow().is_some(),
        "the gear at the foot of the rail vanished from the DSL"
    );
    assert!(
        doca.borrow().is_some(),
        "the shell's `dock` is not a Dock — the entire layout vanished from the DSL"
    );

    // ---- stage 6: the EXPLORER ----
    //
    // Template names are NOT written here: they come from
    // `explorer::Row::template`, which the design queries. Written twice,
    // this gate would stay green on a name the design no longer uses
    // — frozen coverage, which is the defect it exists to avoid.
    //
    // Each template is instantiated at its own INDEX: `PortalList` stores one item per
    // index and swaps the widget when the template changes (`portal_list.rs:566-575`).
    let explorador = doca
        .item(super::layout::tab_of(crawrad_actions::Panel::Explorer))
        .portal_list(id!(explorer_body.explorer_list));
    // THE ROW CONTRACT of this panel, verified template by template.
    //
    // Every Explorer template has `face`, and inside it `nome`; one that asks for
    // an icon has `face.icone`, one that folds has `face.marca` and `alvo`. A
    // `nomee` in the DSL would return `WidgetRef::empty()` silently — and it is
    // precisely because of this silence that the painter can be five calls
    // without a `match`: what guarantees the names exist is this gate, not the
    // compiler.
    for (i, linha) in super::explorer::sample_rows().iter().enumerate() {
        let molde = LiveId::from_str(linha.template());
        let item = explorador.item(&mut cx, i, molde);
        assert!(
            !item.is_empty(),
            "Explorer template `{}` did not become a widget — the row would come out \
             blank, without crashing",
            linha.template()
        );
        assert!(
            item.label(id!(face.nome)).borrow().is_some(),
            "Explorer template `{}` does not have `face.nome` — the row would come out with \
             blank text",
            linha.template()
        );
        if linha.icon().is_some() {
            assert!(
                item.label(id!(face.icone)).borrow().is_some(),
                "template `{}` requests an icon and does not have `face.icone`",
                linha.template()
            );
        }
        if linha.chevron().is_some() {
            assert!(
                item.label(id!(face.marca)).borrow().is_some(),
                "template `{}` requests a chevron and does not have `face.marca`",
                linha.template()
            );
        }
        if !matches!(linha.activation(), super::explorer::Activation::Inert) {
            assert!(
                item.button(id!(alvo)).borrow().is_some(),
                "template `{}` responds to clicks and does not have the target in `alvo` — \
                 without it, clicking the row emits no action",
                linha.template()
            );
        }
    }
    let com_selo = explorador.item(&mut cx, 0, live_id!(File));
    assert!(
        com_selo.label(id!(face.selo)).borrow().is_some(),
        "the Explorer file row does not have the label `face.selo`"
    );

    // ---- stage 7: the PACKAGE MANAGER ----
    let loja = doca.item(super::layout::tab_of(crawrad_actions::Panel::Packages));
    let lista = loja.portal_list(id!(packages_body.packages_list));
    for (i, linha) in super::packages::sample_rows().iter().enumerate() {
        let molde = LiveId::from_str(linha.template());
        let item = lista.item(&mut cx, i, molde);
        assert!(
            !item.is_empty(),
            "Package Manager template `{}` did not become a widget",
            linha.template()
        );
    }
    let cartao = lista.item(&mut cx, 0, live_id!(Found));
    assert!(
        cartao.button(id!(direita.acao)).borrow().is_some(),
        "the package card does not have the `Add` button in `direita.acao` — without it \
         there is no way to install anything, and the list becomes a showcase"
    );
    for campo in [
        &id!(esquerda.topo.nome)[..],
        &id!(esquerda.topo.versao)[..],
        &id!(esquerda.descricao.texto)[..],
    ] {
        assert!(
            cartao.label(campo).borrow().is_some(),
            "the package card does not have the label {campo:?}"
        );
    }
    // THE RIGHT COLUMN SLICES, from the list used by the PAINTER — and not from a
    // second list written here. It binds the number of DSL boxes to that of the
    // model, and a mismatched name returns an empty `WidgetRef`: `set_text`
    // does nothing and the stat disappears without complaint.
    for caixa in crate::ide_app::packages_app::PACKAGE_SLICES {
        assert!(
            !cartao.view(caixa).is_empty(),
            "the package card does not have the right-column box {caixa:?}"
        );
        // And the LABEL inside it, via the path derived by the painter. `Label`
        // does not hide (`widget.rs:44`), so the slice MUST be a `View`
        // with the text inside — and a wrong name here returns an empty widget, with
        // `set_text` doing nothing and the stat disappearing silently.
        let mut dentro = caixa.to_vec();
        dentro.push(live_id!(texto));
        assert!(
            cartao.label(&dentro).borrow().is_some(),
            "the slice {caixa:?} does not have the label `texto` inside it"
        );
    }
    let dependencia = lista.item(&mut cx, 1, live_id!(Dependency));
    assert!(
        dependencia.button(id!(acao)).borrow().is_some(),
        "the dependency row does not have the `Remove` button in `acao`"
    );
    for aba in [
        &id!(packages_tabs.tab_browse)[..],
        &id!(packages_tabs.tab_installed)[..],
    ] {
        assert!(
            loja.radio_button(aba).borrow().is_some(),
            "tab {aba:?} of the Package Manager is not a RadioButton — \
             `RadioButtonSet` would return `None` forever and the tab would remain \
             unreachable, silently"
        );
    }
    assert!(
        loja.text_input(id!(packages_search.packages_query))
            .borrow()
            .is_some(),
        "the Package Manager search field is not a TextInput"
    );
    for campo in [
        &id!(packages_header.packages_target)[..],
        &id!(packages_status.texto)[..],
    ] {
        assert!(
            loja.label(campo).borrow().is_some(),
            "the Package Manager does not have the label {campo:?}, which the shell writes \
             at runtime"
        );
    }
    let chat = doca.item(super::layout::tab_of(crawrad_actions::Panel::Chat));
    assert!(!chat.is_empty(), "chat panel empty in the dock");
    assert!(
        !chat
            .button(id!(chat_base.ai_input_bar.ai_tools_bar.btn_ai_model))
            .is_empty(),
        "btn_ai_model empty in the dock"
    );
    assert!(
        !chat
            .button(id!(chat_base.ai_input_bar.ai_tools_bar.btn_ai_effort))
            .is_empty(),
        "btn_ai_effort empty in the dock"
    );
    assert!(
        !chat
            .button(id!(chat_base.ai_input_bar.ai_tools_bar.btn_ai_attach))
            .is_empty(),
        "btn_ai_attach empty in the dock"
    );
    assert!(
        !chat.view(id!(ai_model_dropdown)).is_empty(),
        "ai_model_dropdown empty in the dock"
    );
    assert!(
        !chat
            .view(id!(ai_model_dropdown))
            .button(id!(opt_0))
            .is_empty(),
        "opt_0 in ai_model_dropdown empty in the dock"
    );
    println!("{MARCA_OK}");
}

/// Runs the probe in a child process and returns everything it printed.
fn rodar_sonda(nome: &str) -> (String, bool) {
    let exe = std::env::current_exe().expect("path of the test binary");
    let saida = Command::new(exe)
        .args([
            "--exact",
            nome,
            "--ignored",
            "--nocapture",
            "--test-threads",
            "1",
        ])
        .env(VAR_SONDA, "1")
        .output()
        .expect("run probe in a child process");

    let texto = format!(
        "{}{}",
        String::from_utf8_lossy(&saida.stdout),
        String::from_utf8_lossy(&saida.stderr)
    );
    (texto, saida.status.success())
}

/// The gate for the Makepad shell DSL.
#[test]
fn a_dsl_da_casca_makepad_esta_integra() {
    let nome = format!("{}::sonda_da_casca", module_path!());
    // `module_path!` puts the crate name in front (`crawrad_ide::…`), and the
    // libtest filter does not use it: the path starts at the binary's root module.
    let nome = nome.split_once("::").map(|(_, r)| r).unwrap_or(&nome);
    let (texto, sucesso) = rodar_sonda(nome);
    let achados = colher_erros(&texto);

    assert!(
        achados.is_empty(),
        "the probe found {} DSL error(s) that Makepad only reports via log:\n{}\n\
         --- full probe output ---\n{texto}",
        achados.len(),
        achados.join("\n")
    );
    assert!(
        sucesso,
        "the probe did not terminate successfully (headless panic?):\n{texto}"
    );
    assert!(
        texto.contains(MARCA_OK),
        "the probe did not reach the end; without the success mark the gate is worthless:\n{texto}"
    );
}

/// Proves that the harvester is not decorative — without this, a `colher_erros` that
/// always returned empty would leave the gate green forever.
#[test]
fn o_colhedor_reconhece_cada_marca_do_makepad() {
    for marca in MARCAS_DE_ERRO {
        let linha = format!("src/algum_arquivo.rs:10:4 {marca} blá blá");
        assert_eq!(
            colher_erros(&linha).len(),
            1,
            "the harvester missed the mark {marca}"
        );
    }
    assert!(colher_erros("uma linha inocente qualquer").is_empty());
}

// Every dialog sits in the modal veil (`ModalVeil`): no click reaches the window behind it.
const DIALOG_TEMPLATES_RS: &str = include_str!("../../ide_app/theme/dialog_templates.rs");
const DSL_MAIN_RS: &str = include_str!("../../ide_app/dsl_main.rs");

#[test]
fn dialog_veil_template_uses_modal_veil() {
    assert!(
        DIALOG_TEMPLATES_RS.contains("pub CrawradDialogVeil = <ModalVeil>"),
        "`CrawradDialogVeil` must be defined using `<ModalVeil>` as its base widget"
    );
}

#[test]
fn shrink_dialog_is_wrapped_in_modal_veil() {
    assert!(
        DSL_MAIN_RS.contains("shrink_dialog = <CrawradDialogVeil>"),
        "`shrink_dialog` must be wrapped inside `<CrawradDialogVeil>`"
    );
}
