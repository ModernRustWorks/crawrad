//! **A THIRD-PARTY COMPONENT EVENT REACHES THE HANDLER** — on both targets.
//!
//! # What was missing, and why it is the half that makes the rest worthwhile
//!
//! The third-party component was already turned into code and drawn. What it
//! did not do was **notify**: the user wired `OnClick` in the Object Inspector, the stub
//! was generated in their file, and the application never called it.
//!
//! Without this, the component suite is a set of pictures. With the click, the
//! handler can modify the component properties — and the declared drawing
//! follows in the next frame, because the verbs read the fields. That is what turns a
//! drawing into a component.
//!
//! # The table is ONE, and the vocabulary is the shortest that exists here
//!
//! A declared component is not a toolkit widget: its only surface is its
//! own rectangle. That is why the list of events it can respond to is
//! target-neutral and lives in `targets/compartilhado.rs` — and today it has one item.

use crawrad_ir::{Document, Rect, Value};
use crawrad_package::manifest::Manifest;
use crawrad_registry::Registry;

use crate::Gerador;
use crate::targets::compartilhado::{EVENTOS_DO_DECLARADO, ligacoes_do_declarado};
use crate::targets::makepad::binding;

/// The catalogue with the `Gauge` read from the **real** `manifest.json`.
fn catalogo() -> Registry {
    let texto = include_str!("../../../../../components/gauge-crawrad/manifest.json");
    let manifesto: Manifest =
        serde_json::from_str(texto).expect("the `manifest.json` is valid JSON");
    let mut r = Registry::builtin();
    for c in manifesto.to_components().expect("converts") {
        r.register_external(c).expect("registers");
    }
    r
}

/// A form with `quantos` gauges, the first with `OnClick` wired.
fn doc(reg: &Registry, quantos: usize) -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("the root");
        raiz.set_rect(Rect::new(0.0, 0.0, 420.0, 260.0));
        raiz.set_name("form_main");
    }
    for i in 0..quantos {
        let nome = format!("gau{i}");
        let mut n = reg
            .instantiate(
                "Gauge",
                nome.clone(),
                Rect::new(8.0, 8.0 + i as f32 * 56.0, 180.0, 36.0),
            )
            .expect("the catalogue instantiates");
        n.set_name(&nome);
        n.props.insert("value".to_string(), Value::Int(60));
        n.events
            .insert("on_click".to_string(), format!("{nome}_click"));
        d.add_child("form_main", n).expect("enters");
    }
    d.validate().expect("valid");
    d
}

fn arquivo(g: &dyn Gerador, reg: &Registry, d: &Document, agulha: &str) -> String {
    g.gerar(d, "app", reg)
        .expect("generating")
        .into_iter()
        .find(|a| a.caminho.to_string_lossy().contains(agulha))
        .unwrap_or_else(|| panic!("the file `{agulha}`"))
        .conteudo
}

// ---------------------------------------------------------------------------
// 1 — THE CLICK ARRIVES
// ---------------------------------------------------------------------------

/// **In egui**: the click area is the component's rectangle, and dispatch comes
/// after drawing.
#[test]
fn no_egui_o_clique_chama_o_handler() {
    let r = catalogo();
    let fonte = arquivo(&crate::GeradorRxp, &r, &doc(&r, 1), "designer");
    assert!(
        fonte.contains("let _resp = ui.interact("),
        "the component does not reserve any click area:\n{}",
        linhas_com(&fonte, "interact")
    );
    assert!(
        fonte.contains("if self.gau0.enabled && _resp.clicked() {")
            && fonte.contains("self.gau0_click();"),
        "the click does not turn into a call:\n{}",
        linhas_com(&fonte, "clicked")
    );
}

/// **DISABLED DOES NOT DISAPPEAR, IT DIMS.** It remains drawn and stops
/// responding — that is what a classic RAD IDE does, and what this target already does with built-in components.
#[test]
fn no_egui_o_desabilitado_nao_responde() {
    let r = catalogo();
    let fonte = arquivo(&crate::GeradorRxp, &r, &doc(&r, 1), "designer");
    let despacho = fonte
        .lines()
        .find(|l| l.contains("_resp.clicked()"))
        .expect("the dispatch line");
    assert!(
        despacho.contains("self.gau0.enabled"),
        "the dispatch does not check `enabled`: `{}`",
        despacho.trim()
    );
}

/// **In Makepad**: the query comes from the TABLE, not from a handwritten `emit_actions`
/// — the declared component has no handwritten function to write.
#[test]
fn no_makepad_o_clique_chama_o_handler() {
    let r = catalogo();
    let fonte = arquivo(&crate::GeradorMakepad, &r, &doc(&r, 1), "designer");
    assert!(
        fonte.contains("if self.ui.view(id!(gau0)).finger_up(actions).is_some() {"),
        "the query to the framework was not emitted:\n{}",
        linhas_com(&fonte, "finger_up")
    );
    assert!(
        fonte.contains("self.form.gau0_click();"),
        "the handler is not called:\n{}",
        linhas_com(&fonte, "gau0_click")
    );
}

/// **THE CURSOR IS NOT DECORATION: WITHOUT IT THE CLICK NEVER ARRIVES.**
///
/// Measured in the framework source (`makepad-widgets-1.0.0/src/view.rs:701`):
///
/// ```text
/// if self.visible && self.cursor.is_some() || self.animator.live_ptr.is_some() {
///     match event.hits_with_capture_overload(cx, self.area(), …)
/// ```
///
/// A `<View>` only hit-tests when it has `cursor`. Without this line,
/// `handle_actions` checks for a `finger_up` that never happens — no error,
/// no warning, and with dispatch code right there, looking like a finished feature.
#[test]
fn no_makepad_o_view_ganha_o_cursor_que_o_hit_test_exige() {
    let r = catalogo();
    let fonte = arquivo(&crate::GeradorMakepad, &r, &doc(&r, 1), "designer");
    assert!(
        fonte.contains("cursor: Hand"),
        "without `cursor` Makepad's `<View>` does not hit-test, and the dispatch \
         below never runs:\n{}",
        linhas_com(&fonte, "gau0 = <View>")
    );
}

/// **And a component WITHOUT a handler gets no hand cursor.**
///
/// The twin of the test above, which makes the pair complete: a hand cursor over
/// a component that does not respond is an application making a promise it cannot keep.
#[test]
fn sem_handler_nao_ha_cursor_de_mao() {
    let r = catalogo();
    let mut d = doc(&r, 1);
    d.node_mut("gau0").expect("the node").events.clear();
    let fonte = arquivo(&crate::GeradorMakepad, &r, &d, "designer");
    assert!(
        !fonte.contains("cursor: Hand"),
        "the component does not respond to anything and yet promises a click"
    );
    let egui = arquivo(&crate::GeradorRxp, &r, &d, "designer");
    assert!(
        !egui.contains("ui.interact("),
        "the egui target reserved a click area for what has no handler:\n{}",
        linhas_com(&egui, "interact")
    );
}

// ---------------------------------------------------------------------------
// 2 — THE HANDLER EXISTS TO BE CALLED
// ---------------------------------------------------------------------------

/// **The stub is generated in the user's file.**
///
/// Calling a nonexistent method breaks compilation, and would be the
/// opposite of the old bug: previously the handler existed and no one called it.
#[test]
fn o_stub_do_handler_nasce_no_arquivo_do_usuario() {
    let r = catalogo();
    let d = doc(&r, 1);
    for (nome, g) in [
        ("egui", &crate::GeradorRxp as &dyn Gerador),
        ("makepad", &crate::GeradorMakepad as &dyn Gerador),
    ] {
        let arquivos = g.gerar(&d, "app", &r).expect("generating");
        let usuario = arquivos
            .iter()
            .find(|a| a.dono == crate::Dono::Usuario)
            .expect("the user file");
        let tem = usuario.conteudo.contains("fn gau0_click(")
            || usuario.adicoes.iter().any(|a| a.nome_da_fn == "gau0_click");
        assert!(tem, "[{nome}] handler stub was not created");
    }
}

// ---------------------------------------------------------------------------
// 3 — TWO INSTANCES DO NOT SHARE A CLICK TARGET
// ---------------------------------------------------------------------------

/// **The property that the `ui.id()` gate measures structurally, measured directly.**
///
/// The entire form draws in a single `Ui`, so `ui.id()` is literally the
/// same value for all components. Two gauges with the same word
/// would share the click target, and clicking one would trigger the other.
#[test]
fn dois_medidores_no_mesmo_formulario_tem_ids_diferentes() {
    let r = catalogo();
    let fonte = arquivo(&crate::GeradorRxp, &r, &doc(&r, 2), "designer");
    let ids: Vec<&str> = fonte
        .lines()
        .filter(|l| l.contains("ui.id().with((\"crawrad_declared\""))
        .map(str::trim)
        .collect();
    assert_eq!(ids.len(), 2, "expected two declared component `id`s");
    assert_ne!(
        ids[0], ids[1],
        "both gauges share the same click target: {ids:?}"
    );
}

// ---------------------------------------------------------------------------
// 4 — THE NEUTRAL LIST AND THE TARGET TABLE CANNOT DIVERGE
// ---------------------------------------------------------------------------

/// **Every event the neutral list promises, the Makepad target delivers.**
///
/// The list is what the Object Inspector ends up offering; the table is what becomes
/// a framework query. Diverging in either direction represents the two bugs that
/// the `verbo_bind` gate pursues in built-in components — here they are impossible by
/// construction, and this test ensures they stay that way.
#[test]
fn a_lista_neutra_e_a_tabela_do_makepad_dizem_a_mesma_coisa() {
    let da_tabela: Vec<&str> = binding::BINDINGS_DECLARADOS
        .iter()
        .map(|(e, _)| *e)
        .collect();
    assert_eq!(
        EVENTOS_DO_DECLARADO.to_vec(),
        da_tabela,
        "the neutral list and the Makepad table diverged. An event in the list and outside \
         the table appears in the Inspector and is never called; the opposite is emitted \
         code that nothing triggers."
    );
}

/// **And the egui target delivers the same.**
///
/// It has no table: the dispatch `match` is its table. An event that
/// entered the list without a match arm would exit with `continue` — silence, which
/// is precisely what cannot happen.
#[test]
fn o_alvo_egui_entrega_todo_evento_da_lista() {
    let r = catalogo();
    for evento in EVENTOS_DO_DECLARADO {
        let mut d = doc(&r, 1);
        {
            let n = d.node_mut("gau0").expect("the node");
            n.events.clear();
            n.events
                .insert((*evento).to_string(), "gau0_handler".to_string());
        }
        let fonte = arquivo(&crate::GeradorRxp, &r, &d, "designer");
        assert!(
            fonte.contains("self.gau0_handler();"),
            "[egui] event `{evento}` is in the list and does not turn into any call"
        );
    }
}

/// **An event the class does not declare is refused, not delivered**.
///
/// The `Gauge` declares only `on_click`. Earlier an `on_key_down` wired on
/// it just stayed out of the generated code; now generation stops with the
/// class and the event named — the document is wrong, and saying so is better
/// than generating around it.
#[test]
fn undeclared_event_is_refused_by_both_targets() {
    let r = catalogo();
    let mut d = doc(&r, 1);
    {
        let n = d.node_mut("gau0").expect("the node");
        n.events.clear();
        n.events
            .insert("on_key_down".to_string(), "gau0_key".to_string());
    }
    assert!(
        ligacoes_do_declarado(d.node("gau0").expect("the node"))
            .expect("reading")
            .is_empty(),
        "an event the targets do not deliver entered the binding list"
    );
    for (nome, g) in [
        ("egui", &crate::GeradorRxp as &dyn Gerador),
        ("makepad", &crate::GeradorMakepad as &dyn Gerador),
    ] {
        let err = g
            .gerar(&d, "app", &r)
            .expect_err("an undeclared event must stop generation");
        assert!(
            matches!(
                &err,
                crate::Erro::Catalog(
                    crawrad_registry::class_property::ClassError::UndeclaredEvent { event, .. }
                ) if event == "on_key_down"
            ),
            "[{nome}] wrong refusal: {err}"
        );
    }
}

fn linhas_com(fonte: &str, agulha: &str) -> String {
    fonte
        .lines()
        .filter(|l| l.contains(agulha))
        .collect::<Vec<_>>()
        .join("\n")
}
