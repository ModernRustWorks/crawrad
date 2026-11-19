//! The gate of the DOCK: the panel-to-tab mapping, the default arrangement,
//! and the file the arrangement is written to.
//!
//! # The three levels it charges
//!
//! 1. **The mapping** is pure and exhaustive — every panel of the catalogue
//!    has a tab and a template, and no two share one.
//! 2. **The file** is a compatibility surface from the first save on, and the
//!    rule that keeps it honest is *discard whole, never half* — measured here
//!    with a saved arrangement from a build that had another panel.
//! 3. **The wiring**, by the technique of `fios.rs`: the shell is read as
//!    TEXT. A perfect `aplicar_arranjo` nobody calls passes every unit test,
//!    and a `check_and_clear_need_save` nobody asks means an arrangement that
//!    never survives a restart — both of them silent.
//!
//! What it does **not** prove: that the panels come out where the arrangement
//! says. That is screen, and screen is checked by looking — and the trap
//! measured in this house is exactly there, because `area().rect()` answered
//! right while the window drew something else.

use std::collections::HashMap;

use crawrad_actions::Panel;
use makepad_widgets::splitter::{SplitterAlign, SplitterAxis};
use makepad_widgets::*;

use super::layout;

/// The Makepad shell — the whole file, as text.
use super::casca::CASCA;

/// The theme of the shell, as text: the dock measurements live there.
const TEMA: &str = super::tests::TEMA_COMPLETO;

/// `CASCA` with no whitespace at all — immune to whatever `rustfmt` picks.
fn compacto() -> String {
    CASCA.chars().filter(|c| !c.is_whitespace()).collect()
}

/// The value of a numeric token of the theme.
fn theme_number(token: &str) -> f64 {
    let agulha = format!("pub {token} = ");
    let i = TEMA
        .find(&agulha)
        .unwrap_or_else(|| panic!("token `{token}` vanished from src/ide_app/tema.rs"));
    let resto = &TEMA[i + agulha.len()..];
    let fim = resto
        .find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-'))
        .unwrap_or(resto.len());
    resto[..fim]
        .parse()
        .unwrap_or_else(|_| panic!("`{token}` is not a number"))
}

/// An arrangement of one tab in one group, for the persistence tests.
fn arranjo_minimo(aba: LiveId) -> HashMap<LiveId, DockItem> {
    HashMap::from([
        (
            live_id!(root),
            DockItem::Tabs {
                tabs: vec![aba],
                selected: 0,
                closable: true,
            },
        ),
        (
            aba,
            DockItem::Tab {
                name: "x".into(),
                template: layout::TAB_TEMPLATE,
                kind: live_id!(PanelComponents),
            },
        ),
    ])
}

// ------------------------------------------------------------- the mapping

#[test]
fn cada_painel_tem_uma_aba_so_e_nenhuma_se_repete() {
    let abas: std::collections::BTreeSet<u64> =
        Panel::ALL.iter().map(|p| layout::tab_of(*p).0).collect();
    assert_eq!(
        abas.len(),
        Panel::ALL.len(),
        "two panels share the same tab `LiveId`. The id is what the \
         saved arrangement stores: two identical ones would cause one panel to open in \
         place of the other, silently."
    );
}

#[test]
fn toda_aba_conhecida_volta_para_o_painel_dela() {
    for p in Panel::ALL {
        assert_eq!(layout::panel_of_tab(layout::tab_of(*p)), Some(*p));
    }
    assert_eq!(layout::panel_of_tab(live_id!(tab_que_nunca_existiu)), None);
}

#[test]
fn cada_painel_tem_molde_proprio() {
    // This test was the opposite: it checked that `Explorer` and `Packages`
    // SHARED a template — `PanelPending`, which wrote "no wire
    // yet" in place of both screens. Both gained screens in this cycle
    // and the rule became the opposite.
    //
    // The mutation it catches is the one that the shared template made impossible
    // to notice: two panels pointing to the same template. One of them would draw
    // the other, and whoever clicked on the first would see the second — without crashing.
    let moldes: std::collections::BTreeSet<u64> = Panel::ALL
        .iter()
        .map(|p| layout::template_of(*p).0)
        .collect();
    assert_eq!(
        moldes.len(),
        Panel::ALL.len(),
        "two dock panels share a template. One would draw the other."
    );
}

#[test]
fn todo_molde_citado_pelo_mapeamento_existe_na_dsl() {
    // The central trap of Makepad: a `kind` that matches no DSL template
    // causes `item_or_create` to return `None` and the tab is born EMPTY,
    // without crashing (`dock.rs:1076-1085`). Here this turns red in 0 s.
    let c = compacto();
    for p in Panel::ALL {
        let nome = p.template_name();
        assert!(
            c.contains(&format!("{nome}=<")),
            "template `{nome}`, requested by panel `{}`, is not declared in the \
             dock DSL",
            p.id()
        );
        assert_eq!(layout::template_of(*p), LiveId::from_str(&nome));
    }
}

#[test]
fn three_default_layout_tabs_in_dsl_have_same_names() {
    // The tab names are what the saved file stores. The DSL declares THREE
    // of them — those of the default arrangement —, and the names must match: a `tab_desing`
    // in the DSL would create a tab that `panel_of_tab` does not recognize, and the
    // entire arrangement would be discarded on the next launch.
    let c = compacto();
    for p in [Panel::Components, Panel::Design, Panel::Chat] {
        assert!(
            c.contains(&format!("{}=Tab{{", p.tab_name())),
            "tab `{}` of the default arrangement vanished from the DSL, or changed name",
            p.tab_name()
        );
    }
}

/// The saved arrangement must be READABLE, and that does not come for free.
///
/// Measured on 16/09/2026, and it was a silent defect: `SerRon` of `LiveId` only
/// prints the NAME of an id known to the framework table
/// (`makepad-micro-serde-1.0.0/src/serde_ron.rs:885`), and `LiveId::from_str`
/// does **not** register any name. The three tabs that do not appear in the DSL came out of the
/// file as 64-bit numbers — and, since `DeRon` reads the number back,
/// nothing broke and the defect would remain.
#[test]
fn o_arranjo_gravado_sai_com_o_nome_das_abas() {
    layout::register_tab_names();
    for p in Panel::ALL {
        let texto = layout::encode(&arranjo_minimo(layout::tab_of(*p)));
        assert!(
            texto.contains(&p.tab_name()),
            "the tab for `{}` came out of the saved arrangement as a number, not as a \
             name. A file no one reads is a file no one debugs — \
             and this is a compatibility surface.",
            p.id()
        );
    }
}

// ------------------------------------------------------------- the arrangement

#[test]
fn o_arranjo_padrao_e_o_da_dsl() {
    // `Reset Layout` returns the factory arrangement, and it exists TWICE:
    // in DSL (to be born) and in Rust (to restore). Diverging would yield a reset
    // that returns a screen that was never the initial one, without anything complaining — and
    // `live_design!` cannot be read from Rust (it is a string).
    let c = compacto();
    for id in [
        "root=Splitter{",
        "dock_rest=Splitter{",
        "dock_left=Tabs{",
        "dock_center=Tabs{",
        "dock_right=Tabs{",
        "tab_components=Tab{",
        "tab_design=Tab{",
        "tab_chat=Tab{",
    ] {
        assert!(
            c.contains(id),
            "`{id}` vanished from the default DSL arrangement, and Rust's `arranjo_padrao()` \
             still builds it — `Reset Layout` would return a different \
             screen"
        );
    }
    assert!(
        c.contains("align:FromA((CRAWRAD_DOCA_LARGURA))")
            && c.contains("align:FromB((CRAWRAD_CHAT_LARGURA))"),
        "the default arrangement splitters stopped using the theme measurements"
    );
}

#[test]
fn as_medidas_do_arranjo_padrao_batem_com_o_tema() {
    // `live_design!` does not compile Rust: a Rust constant does not enter the
    // DSL and a DSL token does not enter Rust. What remains is duplication
    // locked by tests — the same decision that `ide_app/tema.rs` already declares
    // at its top.
    assert_eq!(
        layout::DOCK_WIDTH,
        theme_number("CRAWRAD_DOCA_LARGURA"),
        "the left column width diverged between Rust and the DSL: \
         `Reset Layout` would return a column of a different width"
    );
    assert_eq!(
        layout::CHAT_WIDTH,
        theme_number("CRAWRAD_CHAT_LARGURA"),
        "the chat column width diverged between Rust and the DSL"
    );
}

// ------------------------------------------------------------- the file

#[test]
fn o_arranjo_gravado_volta_igual() {
    let antes = arranjo_minimo(layout::tab_of(Panel::Components));
    let depois = layout::decode(&layout::encode(&antes)).expect("arrangement did not round-trip");
    assert_eq!(antes.len(), depois.len());
    assert!(depois.contains_key(&live_id!(root)));
    assert!(depois.contains_key(&layout::tab_of(Panel::Components)));
}

#[test]
fn file_carries_format_version() {
    let texto = layout::encode(&arranjo_minimo(layout::tab_of(Panel::Design)));
    assert!(
        texto.starts_with(&format!(
            "crawrad-dock-layout {}",
            crawrad_runtime_makepad::dock::FORMAT_VERSION
        )),
        "the saved arrangement stopped carrying the format version. Without it, the \
         first file written by another format returns as garbage — and what \
         one sees is a crooked window, not an error."
    );
}

#[test]
fn versao_desconhecida_e_descartada_inteira() {
    let texto = layout::encode(&arranjo_minimo(layout::tab_of(Panel::Design)));
    let de_outro_formato = texto.replacen("crawrad-dock-layout 1", "crawrad-dock-layout 99", 1);
    assert!(
        layout::decode(&de_outro_formato).is_none(),
        "an arrangement from another format was accepted. The rule is to discard WHOLE: \
         half an arrangement leaves the window missing a panel without explanation."
    );
    assert!(layout::decode("sem marca nenhuma\n{}").is_none());
    assert!(layout::decode("").is_none());
}

#[test]
fn panel_tab_unknown_to_build_discards_file() {
    // The real migration case: an arrangement saved by a version that had an
    // extra panel, or one with another name. Accepting the rest would yield a window without
    // that panel and without a word.
    let texto = layout::encode(&arranjo_minimo(live_id!(tab_de_um_painel_extinto)));
    assert!(
        layout::decode(&texto).is_none(),
        "an arrangement with an unknown tab was accepted"
    );
}

#[test]
fn arranjo_sem_raiz_e_descartado() {
    // Without `root` the `Dock` has no starting point to draw
    // (`dock.rs:1090`), and the screen comes up empty.
    let mut sem_raiz = arranjo_minimo(layout::tab_of(Panel::Chat));
    sem_raiz.remove(&live_id!(root));
    assert!(layout::decode(&layout::encode(&sem_raiz)).is_none());
}

#[test]
fn config_folder_is_same_across_shells() {
    // The arrangement's folder IS the rest of the IDE's (no copy to drift):
    // the same function, so the same folder.
    assert_eq!(
        crawrad_project::workspace::config_folder(),
        crawrad_project::workspace::config_folder()
    );
    let ours = include_str!("../../ide_app/layout.rs");
    assert!(ours.contains("dock_layout_file_path as file_path"));
    let desktop = include_str!("../../../../crawrad-project/src/desktop.rs");
    assert!(desktop.contains("workspace::config_folder()"));
}

// --------------------------------------------------------------- the wires

/// **Wire** — the layout effect reaches the dock.
///
/// The `effect_coverage_gate` of `crawrad-actions` requires that the shell **read** the
/// `layout` field; it does not require that reading it do anything. This one does.
#[test]
fn o_efeito_de_arranjo_e_aplicado() {
    let c = compacto();
    assert!(
        c.contains("ifletSome(comando)=efeito.layout{self.aplicar_arranjo(cx,comando);}"),
        "the `layout` field of `Efeito` stopped going to the dock. The action would run, \
         the effect would return correctly, and the window would not move — the \
         silent defect that `effect_coverage_gate` was born to catch."
    );
}

/// **Wire** — the arrangement is saved when the `Dock` says it changed.
#[test]
fn o_arranjo_e_gravado_quando_muda() {
    let c = compacto();
    assert!(
        c.contains("self.gravar_arranjo_se_mudou(cx);"),
        "the arrangement stopped being saved. Dragging a tab and closing the IDE \
         would lose everything again — and nothing would tell."
    );
    assert!(
        c.contains("self.dock().check_and_clear_need_save()"),
        "the shell stopped asking the `Dock` whether the arrangement changed \
         (`dock.rs:1336`), and began deciding on its own — either writing to disk on \
         every frame, or never writing"
    );
}

/// **Wire** — the saved arrangement is read at startup.
#[test]
fn o_arranjo_e_lido_no_arranque() {
    let c = compacto();
    assert!(
        c.contains("self.load_arrangement(cx);"),
        "the IDE stopped reading the saved arrangement: it would go to disk and never \
         return, which is the same as not persisting"
    );
    assert!(
        c.contains("layout::DISCARDED"),
        "the shell stopped reporting that it discarded a saved arrangement. An arrangement that \
         vanishes without a word is the window returning to default for no \
         apparent reason."
    );
}

/// **Wire** — `Reset Layout` exists, and it is the one from the action.
#[test]
fn o_reset_layout_e_uma_acao_e_esta_ligado() {
    assert!(
        crawrad_actions::by_id("layout.reset").is_some(),
        "action `layout.reset` vanished from the catalog"
    );
    let c = compacto();
    assert!(
        c.contains("Cmd::Reset=>{self.restaurar_arranjo(cx);"),
        "`Reset Layout` stopped restoring the arrangement. An arrangement that persists \
         with no way back is a trap (ADR 0014, Amendment 9)."
    );
}

/// **Wire** — moving and stacking go through the SAME function as dragging.
///
/// This is what makes the command test cover dragging: `drop_move` is the function that
/// `Drop` of `Dock` calls (`dock.rs:1258` → `handle_drop`). Splitter
/// arithmetic written here would be a second codebase to drift.
#[test]
fn mover_e_empilhar_usam_o_caminho_do_arrasto() {
    let c = compacto();
    assert_eq!(
        c.matches("drop_move(cx,").count(),
        2,
        "the number of calls to `drop_move` changed. There are two — move to a \
         border and stack into a tab —, and both exist so the command and \
         drag are the SAME code (ADR 0014, Amendment 10)."
    );
    assert!(
        !c.contains(
            "DockItem::Splitter{axis:SplitterAxis::Horizontal,align:SplitterAlign::Weighted"
        ),
        "the shell went back to hand-crafting splitters to move panels. What knows \
         how to do that is the framework's `handle_drop`."
    );
}

// ------------------------------------------------- the panel context menu

#[test]
fn panel_menu_has_three_borders_and_two_more() {
    let reg = crawrad_registry::Registry::builtin();
    let area = crawrad_designer::clipboard::Clipboard::default();
    let ctx = crawrad_actions::Context {
        doc: None,
        registry: &reg,
        clipboard: &area,
        args: crawrad_actions::Args::none(),
        project: crawrad_actions::Project::default(),
        author: crawrad_actions::Author::User,
        editor_dirty: false,
        text: Default::default(),
    };
    let linhas = super::menu::panel_rows(Panel::Inspector, &ctx);

    let ids: Vec<&str> = linhas.iter().filter_map(|l| l.action.as_deref()).collect();
    assert_eq!(
        ids,
        vec![
            "layout.move_panel",
            "layout.move_panel",
            "layout.move_panel",
            "layout.hide_panel",
            "layout.float_panel",
        ],
        "the panel context menu changed shape. It is the THIRD door \
         to the arrangement, and the only one that works on all three platforms today: \
         dragging does not exist on Linux."
    );

    // EVERY row carries the panel. One without an argument would run the action against the \
    // refusal `panel is required`, and the menu would look broken.
    for l in &linhas {
        assert_eq!(
            l.args.as_text("panel"),
            Some(Panel::Inspector.id()),
            "panel menu row `{}` does not carry the panel",
            l.label
        );
        assert!(l.enabled, "row `{}` started disabled", l.label);
    }

    // And the first three carry the BORDER, one per `DockSide`.
    let bordas: Vec<Option<&str>> = linhas
        .iter()
        .take(3)
        .map(|l| l.args.as_text("side"))
        .collect();
    let esperado: Vec<Option<&str>> = crawrad_actions::DockSide::ALL
        .iter()
        .map(|s| Some(s.id()))
        .collect();
    assert_eq!(bordas, esperado);
}

#[test]
fn nenhum_rotulo_do_menu_do_painel_se_repete() {
    // Three rows with the same id (`layout.move_panel`) and three different
    // labels: without this the menu would show `Move panel to side` three times, and
    // none of them would be clickable intentionally.
    let reg = crawrad_registry::Registry::builtin();
    let area = crawrad_designer::clipboard::Clipboard::default();
    let ctx = crawrad_actions::Context {
        doc: None,
        registry: &reg,
        clipboard: &area,
        args: crawrad_actions::Args::none(),
        project: crawrad_actions::Project::default(),
        author: crawrad_actions::Author::User,
        editor_dirty: false,
        text: Default::default(),
    };
    let linhas = super::menu::panel_rows(Panel::Chat, &ctx);
    let rotulos: std::collections::BTreeSet<&str> =
        linhas.iter().map(|l| l.label.as_ref()).collect();
    assert_eq!(
        rotulos.len(),
        linhas.len(),
        "two identical labels in the menu"
    );
}

/// **Wire** — right-clicking on the rail opens the panel menu.
#[test]
fn right_click_on_rail_opens_panel_menu() {
    let c = compacto();
    assert!(
        c.contains("ifmd.button.is_secondary(){letalvo=self.painel_do_trilho_em(md.abs)"),
        "right-clicking the rail stopped opening the panel menu. Without it, \
         `Move To` has NO mouse entry point — tab dragging does not exist \
         on Linux, and the command only arrives via MCP."
    );
    assert!(
        c.contains("rail_rects.push((painel,item.area().rect(cx)));"),
        "the rail stopped storing each button's rect, and without it \
         right-click has nothing to hit: `PortalList` only yields \
         items inside the drawing loop"
    );
}

/// **Wire** — LEFT-clicking on the rail switches panels.
///
/// It became the **only** navigation path when the dock tab bar
/// gained zero height: before, anyone who did not find the icon still had the
/// tab written above the panel. Today it does not, and a mute rail would trap the
/// user in the opened panel — with nothing on screen explaining why.
///
/// The right button already had a wire (`o_botao_direito_no_trilho_abre_o_menu_do_painel`);
/// the left one, which is the common gesture, had none.
#[test]
fn o_clique_esquerdo_no_trilho_troca_de_painel() {
    let c = compacto();
    assert!(
        c.contains("escolhido=visible_rail.get(i).copied();"),
        "the rail stopped matching clicked items to panels by INDEX. \
         `rail::RAIL` is the drawn order and the read order — if either \
         changes alone, the wrench icon opens Explorer and nothing turns \
         red."
    );
    assert!(
        c.contains(
            "self.execute_with(cx,\"layout.toggle_panel\",crawrad_actions::Args::text(\"panel\",painel.id()),);"
        ),
        "rail clicks stopped going through the catalog. Without that, \
         navigation becomes mouse-only: an agent cannot switch panels, and \
         with the zero-height tab bar this is the ONLY remaining path."
    );
}

/// **Lock** — the dock tab bar takes up no height.
///
/// Owner order, repeated several times: the icon rail **already is** the
/// navigation (ADR 0014, modern-editor model), and a tab bar above it is
/// duplicated navigation — with one destination per panel, each tab was a
/// solitary tab stating the name of the panel that was already there.
///
/// **And this does NOT need a fork**, contrary to what an earlier measurement
/// of ours claimed: the `Dock`'s `tab_bar` is `#[live] Option<LivePtr>`
/// (`dock.rs:158`), so the DSL overrides the entire widget; `TabBar` has
/// `#[walk] walk: Walk` (`tab_bar.rs:193`); and drawing asks for the walk itself
/// (`dock.rs:1145`).
#[test]
fn a_tira_de_abas_da_doca_nao_ocupa_altura() {
    let tema: String = TEMA.chars().filter(|c| !c.is_whitespace()).collect();
    assert!(
        tema.contains("tab_bar:<TabBar>{height:0.0,"),
        "the dock tab bar took up height again. The owner asked to \
         remove it multiple times, and its reappearance brings back duplicate \
         navigation — plus the pixels it eats from the footer."
    );
}

/// **Wire** — the row argument is what runs.
#[test]
fn a_linha_do_menu_executa_com_o_proprio_argumento() {
    let c = compacto();
    assert!(
        c.contains("Some(id)=>self.execute_with(cx,id,linha.args.clone()),"),
        "the menu item ran without arguments again. The five rows of the \
         panel menu REQUIRE `panel`, and without it the action refuses — a menu that opens, \
         responds, and does nothing."
    );
}

// ------------------------------------ panel content does not come via window

/// The ids declared INSIDE a panel template, read from the DSL itself.
///
/// Derived, not a hand-written list: a new panel, or a new `id` inside
/// a panel, enters verification automatically. It is the same decision that turned the rail
/// into a `PortalList` instead of four buttons in the DSL.
fn ids_dentro_do_painel(nome_do_molde: &str) -> Vec<String> {
    // IT FOLLOWS ONE LEVEL OF INDIRECTION, and the reason has been in the DSL since 17/09/2026.
    //
    // The six panels moved from `ide_app.rs` to `ide_app/paineis.rs`
    // when that file exceeded six thousand lines, and what remained inside the
    // dock is the INSTANCE, in one line:
    //
    //     PanelExplorer = <CrawradPanelExplorer> {}
    //
    // Searching only for `PanelExplorer = <`, this reader found an EMPTY body
    // and concluded that the panel declares no id at all. Following the template is what
    // `Dock` itself does to draw the tab — and that is why the reader must
    // do the same, otherwise it measures the shell from before the split.
    let (base, corpo) = molde(nome_do_molde)
        .unwrap_or_else(|| panic!("template `{nome_do_molde}` vanished from the DSL"));
    // Only follows when there is NOTHING here: a panel that still declares ids in
    // its own place continues to be read where it is.
    let corpo = if ids_do_corpo(&corpo).is_empty() {
        molde(&base).map(|(_, c)| c).unwrap_or(corpo)
    } else {
        corpo
    };
    let ids = ids_do_corpo(&corpo);
    assert!(
        !ids.is_empty(),
        "template `{nome_do_molde}` declares no id — either it changed \
         shape, or this reader stopped seeing the DSL"
    );
    ids
}

/// The DSL template `nome`: the type it inherits from, and its body.
///
/// The body goes from the opening `{` to the closing `}`, counting braces — and
/// counting is what allows the read to survive a panel that has another block
/// inside, which all of them do.
fn molde(nome: &str) -> Option<(String, String)> {
    let abre = format!("{nome} = <");
    let i = CASCA.find(&abre)?;
    let depois = &CASCA[i + abre.len()..];
    let base = depois.split('>').next()?.trim().to_string();

    let resto = &CASCA[i..];
    let mut nivel = 0i32;
    let mut fim = resto.len();
    for (k, c) in resto.char_indices() {
        match c {
            '{' => nivel += 1,
            '}' => {
                nivel -= 1;
                if nivel == 0 {
                    fim = k;
                    break;
                }
            }
            _ => {}
        }
    }
    Some((base, resto[..fim].to_string()))
}

/// The ids that a DSL body declares — `nome = <Molde> {`.
///
/// Only lowercase ones: `Uppercase = <` is a TEMPLATE, and a template is not a widget
/// id. It is the same rule that the Makepad DSL uses to distinguish the two.
fn ids_do_corpo(corpo: &str) -> Vec<String> {
    let mut ids = Vec::new();
    for linha in corpo.lines() {
        let linha = linha.trim();
        if linha.starts_with("//") {
            continue;
        }
        if let Some((nome, _)) = linha.split_once(" = <") {
            let nome = nome.trim();
            if !nome.is_empty()
                && nome
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
            {
                ids.push(nome.to_string());
            }
        }
    }
    ids
}

/// **The gate for the 16/09/2026 regression**, and it had three faces on the owner's
/// screen: the canvas with no component, the empty Object Inspector with a selected
/// node, and both appearing only on the second interaction.
///
/// # The cause, measured by pointer
///
/// `self.ui.widget(path)` traverses down the tree and `View` **caches** the
/// `WidgetRef`. `Dock` recreates the widget of an item on each `load_state`
/// (`dock.rs:989-993`) and when a closed tab returns — and the shell retains the
/// pointer to the DEAD instance:
///
/// ```text
/// 1ª chamada:   doca=0x2c8a5a9f2b0   janela=0x2c8a5a9f2b0   iguais=true
/// 2ª em diante: doca=0x2c8a5b59560   janela=0x2c8a5a9f2b0   iguais=FALSE
/// ```
///
/// # What this test enforces, and what it does NOT prove
///
/// It enforces that **no id declared inside a panel template is
/// queried via `self.ui`**. It is a TEXT check: it proves that the right door is
/// written, not that the screen is right. That the panel PAINTS remains screen
/// proof — and that was how the defect appeared and how the fix was
/// verified.
#[test]
fn no_panel_content_is_queried_by_window() {
    let c = compacto();
    // The ids that belong to a panel, and which panel each one belongs to.
    let mut de_painel: Vec<(String, &str)> = Vec::new();
    for p in Panel::ALL {
        for id in ids_dentro_do_painel(&p.template_name()) {
            de_painel.push((id, p.id()));
        }
    }

    // The scan starts from `self.ui.`, and not from the id: thus what is read is
    // exactly the expression performing the search, and an `item.label(id!(nome))`
    // inside a list row does not get counted by proximity.
    let mut achados = Vec::new();
    let mut busca = 0;
    while let Some(rel) = c[busca..].find("self.ui.") {
        let pos = busca + rel;
        busca = pos + "self.ui.".len();
        let resto = &c[busca..];
        let Some(k) = resto.find("(id!(") else {
            continue;
        };
        // Only counts if `id!(` is the argument of THIS call: between `.` and the
        // parenthesis there can only be the method name.
        if !resto[..k]
            .chars()
            .all(|ch| ch.is_alphanumeric() || ch == '_')
        {
            continue;
        }
        let raiz: String = resto[k + "(id!(".len()..]
            .chars()
            .take_while(|ch| ch.is_alphanumeric() || *ch == '_')
            .collect();
        if let Some((_, dono)) = de_painel.iter().find(|(id, _)| *id == raiz) {
            achados.push(format!("{dono} <- self.ui.…(id!({raiz}…"));
        }
    }
    achados.sort();
    achados.dedup();
    assert!(
        achados.is_empty(),
        "panel content queried via the CACHED window path, instead of \
         via the dock (`IdeApp::painel`). The `WidgetRef` in `self.ui` points to the \
         instance that `Dock` already discarded, and the on-screen result is a \
         panel that does not respond to the first event — or never responds:\n  {}",
        achados.join("\n  ")
    );
}

/// **Wire** — the model returns inside every panel that the dock recreated.
///
/// The dock's `load_state` is called in ONE place, the shell's own
/// `load_state`, and that one fills the panels again and keeps the editor from
/// being read until the open file is back in it.
#[test]
fn o_load_state_e_sempre_seguido_de_repovoar() {
    let c = compacto();
    assert_eq!(
        c.matches(".load_state(cx,").count(),
        c.matches("self.load_state(cx,").count() + 1,
        "the dock's `load_state` is called outside the shell's own `load_state`: \
         it drops EVERY panel widget (`dock.rs:989-993`), and without \
         `repovoar_paineis` the form leaves the screen and nothing tells"
    );
    let i = c
        .find("fnload_state(")
        .expect("the shell's `load_state` is gone");
    let corpo = &c[i..(i + 400).min(c.len())];
    for passo in [
        "self.remake_editor=self.host.center().is_text();",
        "self.repovoar_paineis(cx);",
    ] {
        assert!(corpo.contains(passo), "`load_state` lost `{passo}`");
    }
    assert!(
        c.contains("self.editavel.clone().filter(|_|!self.remake_editor)"),
        "`colher_do_editor` reads the editor the dock just remade: its empty \
         text becomes the open file's buffer, and Reset layout + Save wipes it"
    );
}

/// **Wire** — right-clicking on the BODY of a panel also opens the menu.
///
/// It is defect D of 16/09/2026. With the door only on the rail, `Design` and `Chat`
/// had no mouse gesture at all for `Move To`, `Hide`, and `Float`:
/// `rail::RAIL` is a subset of `Panel::ALL` and both are outside it.
/// The owner looked for `Move To` and could not find it.
#[test]
fn right_click_on_panel_body_opens_menu() {
    let c = compacto();
    assert!(
        c.contains(".or_else(||self.painel_no_corpo_em(cx,&self.arranjo(),md.abs));"),
        "right-clicking the BODY of a panel stopped opening the menu, and with \
         that `Design` and `Chat` once again have no mouse entry point for `Move To`"
    );
    // The body rectangle comes from the DOCK, never from the window: `self.ui.widget`
    // returns the DEAD instance whenever the layout changes, and the menu would refer
    // to the wrong panel. See `nenhum_conteudo_de_painel_e_buscado_pela_janela`.
    assert!(
        c.contains(".find(|p|self.painel(*p).area().rect(cx).contains(ponto))"),
        "the body hit-test stopped querying the dock"
    );
    // Only VISIBLE panels are counted: a tab behind another in the same
    // group keeps the rectangle of the last frame in which it appeared.
    assert!(
        c.contains(".filter(|p|self.painel_a_vista(arranjo,*p))"),
        "the body hit-test stopped filtering by visible panels, and the click \
         can hit the tab BEHIND the one that is visible"
    );
}

/// **Wire** — the context menu opens AT THE POINTER, and does not leak outside the window.
///
/// Anchoring it to the rail button was half of defect D: the branch of the panel without
/// a button hid the menu instead of opening it.
#[test]
fn panel_menu_opens_at_pointer_and_fits_window() {
    let c = compacto();
    assert!(
        c.contains("Aberto::Painel{em,..}=>(em.x,em.y),"),
        "the context menu was anchored to something other than the pointer again"
    );
    assert!(
        c.contains("letjanela=self.ui.area().rect(cx);"),
        "the context menu stopped asking the WINDOW where it ends, and a \
         menu opened near the footer draws rows off-screen"
    );
}

/// Every text field centers INK in the box — and no one needs to remember.
///
/// # The defect this locks, and it happened twice in the same day
///
/// `label_align: { y: 0.5 }` of `TextInput` centers the LINE box and not the
/// glyph drawing: line spacing slack remains below the ink and the text moves up. The indent
/// that corrects this is a measured number (`CRAWRAD_TEXTO_NO_CAMPO`), and it lives in
/// the `padding` of each field template.
///
/// The problem is that **in Makepad, redeclaring `padding` on a derived template
/// replaces the entire parent's padding**, not just the mentioned edge. A new template that
/// writes `padding: { left: …, right: … }` zeroes out the indent **in silence** —
/// nothing in the build complains, and it only shows up on screen.
///
/// It happened twice in this cycle: the `Name` field of the Object Inspector
/// was centered and the `Rect` numbers were not, because `CrawradPropQuarter` and
/// `CrawradPropDrop` redeclared `padding`. The owner saw it in both screenshots, and said
/// what was missing:
///
/// > *"You need to define something standard for height and positions of this, because
/// > if we have to validate and fix every edit we are screwed."*
///
/// That is right. This test is the standard: it reads the theme and requires the token in every
/// `padding` declaration of a text field template.
#[test]
fn every_text_field_centers_ink() {
    // The templates that ARE text fields: the base and those derived from it. Derived
    // from the source itself, not from a list written here — a new template enters
    // the count automatically.
    let campos: Vec<&str> = TEMA
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            let nome = l.strip_prefix("pub Crawrad")?.split(' ').next()?;
            let deriva = l.split('<').nth(1)?.split('>').next()?;
            (deriva.contains("TextInput") || deriva == "DropDown").then_some(nome)
        })
        .collect();
    assert!(
        campos.len() >= 4,
        "the scan of field templates found {} — it stopped reading the theme",
        campos.len()
    );

    // And every `padding` INSIDE one of these templates carries the measured indent.
    let mut faltando = Vec::new();
    let mut atual: Option<&str> = None;
    for linha in TEMA.lines() {
        let t = linha.trim();
        if let Some(nome) = t
            .strip_prefix("pub Crawrad")
            .and_then(|r| r.split(' ').next())
        {
            atual = campos.iter().find(|c| **c == nome).copied();
        }
        if let Some(nome) = atual {
            // ONLY the `padding` of the template ITSELF, and not that of a child inside it.
            //
            // Indentation level is the criterion: the body of a `pub Crawrad…` lives at
            // eight spaces, and what is deeper is another widget with its own
            // lifecycle. Without this, the combo popup `menu_item` — which is a MENU
            // row and not a field — failed along with it.
            if !linha.starts_with("        padding: {") {
                continue;
            }
            // The INDENT applies to SINGLE-LINE fields, which is what needs to
            // center ink. A MULTI-LINE field starts text at the top
            // by design, and what it wants is equal breathing room above and below.
            //
            // The criterion is structural and not an exception list: SYMMETRICAL
            // vertical padding says *"I do not center a line, I frame a
            // block"*. That is how `CrawradPadText` — the `Memo` window —
            // entered the count without becoming a name written here.
            let vertical_simetrico = t
                .split("top:")
                .nth(1)
                .and_then(|r| r.split(',').next())
                .zip(t.split("bottom:").nth(1).and_then(|r| r.split('}').next()))
                .is_some_and(|(a, b)| a.trim() == b.trim());
            if !t.contains("CRAWRAD_TEXTO_NO_CAMPO") && !vertical_simetrico {
                faltando.push(format!("{nome}: {t}"));
            }
        }
    }
    assert!(
        faltando.is_empty(),
        "text field template redeclared `padding` WITHOUT the measured indent. In \
         Makepad that replaces the entire parent's padding, and text shifts up \
         in the box again — without anything complaining in the build:\n  {}",
        faltando.join("\n  ")
    );
}

// ------------------------------------------------------------ the lost root

fn splitter(a: LiveId, b: LiveId) -> DockItem {
    DockItem::Splitter {
        axis: SplitterAxis::Horizontal,
        align: SplitterAlign::Weighted(0.5),
        a,
        b,
    }
}

fn grupo(aba: LiveId) -> DockItem {
    DockItem::Tabs {
        tabs: vec![aba],
        selected: 0,
        closable: true,
    }
}

fn aba(p: Panel) -> DockItem {
    DockItem::Tab {
        name: String::new(),
        template: layout::TAB_TEMPLATE,
        kind: layout::template_of(p),
    }
}

/// What the `Dock` leaves behind when the left column is emptied: `root` is
/// gone and `dock_rest` is pointed at by nobody.
fn sem_a_coluna_esquerda() -> HashMap<LiveId, DockItem> {
    HashMap::from([
        (
            live_id!(dock_rest),
            splitter(layout::CENTER_GROUP, layout::RIGHT_GROUP),
        ),
        (layout::CENTER_GROUP, grupo(layout::tab_of(Panel::Design))),
        (layout::RIGHT_GROUP, grupo(layout::tab_of(Panel::Chat))),
        (layout::tab_of(Panel::Design), aba(Panel::Design)),
        (layout::tab_of(Panel::Chat), aba(Panel::Chat)),
    ])
}

#[test]
fn a_coluna_esvaziada_devolve_o_root_ao_irmao_que_sobrou() {
    let reparado = layout::with_root_restored(&sem_a_coluna_esquerda())
        .expect("the arrangement lost `root` and must get it back");
    match reparado.get(&live_id!(root)) {
        Some(DockItem::Splitter { a, b, .. }) => {
            assert_eq!((*a, *b), (layout::CENTER_GROUP, layout::RIGHT_GROUP));
        }
        outro => panic!("`root` must be the surviving splitter, got {outro:?}"),
    }
    assert!(
        !reparado.contains_key(&live_id!(dock_rest)),
        "the orphan is renamed, not copied"
    );
    assert_eq!(reparado.len(), sem_a_coluna_esquerda().len());
}

#[test]
fn com_as_duas_colunas_fora_o_centro_vira_o_root() {
    let so_o_centro = HashMap::from([
        (layout::CENTER_GROUP, grupo(layout::tab_of(Panel::Design))),
        (layout::tab_of(Panel::Design), aba(Panel::Design)),
    ]);
    let reparado = layout::with_root_restored(&so_o_centro).expect("root is back");
    assert!(matches!(
        reparado.get(&live_id!(root)),
        Some(DockItem::Tabs { .. })
    ));
}

#[test]
fn arranjo_com_root_nao_e_tocado() {
    let mut inteiro = sem_a_coluna_esquerda();
    let resto = inteiro.remove(&live_id!(dock_rest)).expect("dock_rest");
    inteiro.insert(live_id!(root), resto);
    assert!(layout::with_root_restored(&inteiro).is_none());
    assert!(layout::with_root_restored(&HashMap::new()).is_none());
}

#[test]
fn dois_orfaos_nao_se_adivinha_qual_e_o_root() {
    let mut ambiguo = sem_a_coluna_esquerda();
    ambiguo.insert(
        live_id!(dock_left),
        grupo(layout::tab_of(Panel::Components)),
    );
    assert!(layout::with_root_restored(&ambiguo).is_none());
}

/// The wiring: both doors call the repair — the command door and the net
/// every event crosses (the drag goes around the first).
#[test]
fn as_duas_portas_chamam_o_reparo_do_root() {
    let arranjo = include_str!("../../ide_app/arrangement.rs");
    let depois_do_comando = arranjo
        .split("fn aplicar_arranjo_e_depois")
        .nth(1)
        .and_then(|s| s.split("\n    }\n").next())
        .expect("aplicar_arranjo_e_depois");
    assert!(depois_do_comando.contains("self.restore_root_if_lost(cx);"));
    let gravar = arranjo
        .split("fn gravar_arranjo_se_mudou")
        .nth(1)
        .expect("gravar_arranjo_se_mudou");
    assert!(
        gravar
            .split("\n    }\n")
            .next()
            .unwrap_or("")
            .contains("self.restore_root_if_lost(cx);")
    );
}
