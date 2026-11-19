//! **Is `Bind` faithful to what the sixteen handlers do, and is its vocabulary
//! closed?**
//!
//! Three questions, and all three are measured against existing code:
//!
//! 1. the table [`binding::BINDINGS`] tells the truth about each handwritten
//!    `emit_actions`;
//! 2. the vocabulary is small — and the number stays visible, because a vocabulary
//!    that grows with every new class is not a vocabulary, it is a list;
//! 3. the two debts of `Bind` do not grow.
//!
//! # The two debts, and they point in opposite directions
//!
//! - **declared and unwired event**: `Button` declares twelve events and the
//!   generator wires one. The user sees all twelve in the Object Inspector, writes the
//!   handler, and eleven of them are never called;
//! - **wired and undeclared event**: `Panel`, `GroupBox`, and
//!   `PageControl` react to events that their descriptors do NOT declare — the
//!   Inspector never offers them, so that section is dead code masquerading
//!   as a feature.
//!
//! The second is the more treacherous: it looks like finished work.

use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;

use crate::targets::makepad::binding::{self, Binding};
use crate::targets::makepad::components;

/// A document with a node of the class, with the event handler wired.
fn no_com_evento(classe: &str, evento: &str) -> (Document, String) {
    let r = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").expect("the root");
        root.set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
        root.set_name("form_main");
        root.set_text("Form1");
    }
    let id = "alvo".to_string();
    let mut n = r
        .instantiate(classe, id.clone(), Rect::new(8.0, 8.0, 120.0, 32.0))
        .expect("the catalogue instantiates what it lists");
    n.set_name("alvo");
    n.events
        .insert(evento.to_string(), "handler_do_dono".to_string());
    d.add_child("form_main", n).expect("into the form");
    (d, id)
}

/// The same node, without any handler — the probe's baseline.
fn no_sem_evento(classe: &str) -> (Document, String) {
    let r = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").expect("the root");
        root.set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
        root.set_name("form_main");
        root.set_text("Form1");
    }
    let id = "alvo".to_string();
    let mut n = r
        .instantiate(classe, id.clone(), Rect::new(8.0, 8.0, 120.0, 32.0))
        .expect("the catalogue instantiates what it lists");
    n.set_name("alvo");
    d.add_child("form_main", n).expect("into the form");
    (d, id)
}

/// The text emitted by the handwritten handler, with the event wired or without
/// any.
fn acoes(classe: &str, evento: Option<&str>) -> String {
    let (doc, id) = match evento {
        Some(e) => no_com_evento(classe, e),
        None => no_sem_evento(classe),
    };
    let node = doc.node(&id).expect("the node we just put");
    components::for_class(classe)
        .expect("the class has a handler")
        .emit_actions(&doc, node, "alvo", &[])
        .expect("emitting")
}

/// The text emitted by the handler for that event.
fn acoes_escritas_a_mao(classe: &str, evento: &str) -> String {
    acoes(classe, Some(evento))
}

/// **FIDELITY.** The table describes what the code does — not what I
/// would like it to do.
///
/// An incorrectly descriptive table is worse than none: it becomes the source
/// someone consults instead of reading the code, propagating errors under the
/// guise of documentation.
#[test]
fn a_tabela_descreve_o_que_o_handler_faz() {
    let mut mentiras = Vec::new();
    for (classe, evento, b) in binding::BINDINGS {
        let texto = acoes_escritas_a_mao(classe, evento);
        // The shape decides HOW the framework is queried, and there is no fourth
        // handwritten shape beyond these.
        let esperado = match b.shape {
            binding::Shape::ActionCast => format!(
                "if let {} = actions.find_widget_action_cast(self.ui.{}(id!(alvo{})).widget_uid())",
                b.query, b.accessor, b.part
            ),
            _ => format!(
                "self.ui.{}(id!(alvo{})).{}(actions)",
                b.accessor, b.part, b.query
            ),
        };
        if !texto.contains(&esperado) {
            mentiras.push(format!(
                "{classe}.{evento}: the table says `{esperado}` and the handler does not emit it"
            ));
        }
        if let Some(w) = b.writeback {
            let dev = format!("self.form.alvo.{w}");
            if !texto.contains(&dev) {
                mentiras.push(format!(
                    "{classe}.{evento}: the table promises to return `{w}` to state and the handler does not return it"
                ));
            }
        }
    }
    assert!(
        mentiras.is_empty(),
        "THE `Bind` TABLE LIES ABOUT THE CODE:\n  {}\n\n\
         Either the handler changed and the table row grew stale, or the row was born \
         from a guess. Both are fixed by reading the class's `emit_actions`.",
        mentiras.join("\n  ")
    );
}

/// **The vocabulary is closed, and the count is visible.**
///
/// Six accessors and four queries cover everything the target wires today. To
/// calibrate: `Brush`, the closed vocabulary this project already runs in production
/// for drawing, has twelve verbs.
///
/// If this test begins to fail whenever a new class is added, the conclusion
/// is not to loosen it — it is that `Bind` is not a vocabulary, and ADR 0036 is
/// wrong in this regard.
#[test]
fn o_vocabulario_do_bind_e_pequeno() {
    let acessores = binding::accessors();
    let consultas = binding::queries();
    assert!(
        acessores.len() <= 8,
        "`Bind` accessor vocabulary reached {}: {acessores:?}. \
         A vocabulary that grows with every new class is a list, not a vocabulary.",
        acessores.len()
    );
    assert!(
        consultas.len() <= 6,
        "`Bind` query vocabulary reached {}: {consultas:?}",
        consultas.len()
    );
}

/// **DEBT 1: event declared by the catalogue that the target does not wire.**
///
/// The user sees the event in the Object Inspector, writes the handler, and it is never
/// called.
const DECLARADO_E_NAO_LIGADO: &[(&str, &str)] = &[("PopupMenu", "on_popup")];

#[test]
fn a_divida_do_evento_sem_ligacao_so_encolhe() {
    let r = Registry::builtin();
    let mut medida: Vec<(String, String)> = Vec::new();
    for classe in components::classes() {
        let desc = r.get(classe).expect("class in the catalogue");
        for ev in &desc.events {
            // an event can also arrive through `handle_event` instead of an
            // action (a Timer's tick): wired when that code calls it
            let (doc, id) = no_com_evento(classe, ev.name);
            let node = doc.node(&id).expect("the node");
            let via_handle_event = components::app_state(&doc, node, "alvo")
                .expect("emitting")
                .is_some_and(|s| s.handle.contains("self.form.handler_do_dono()"));
            // or through the controls' pointer router, for any class
            let via_pointer =
                crate::targets::makepad::control_events::control_events(&r, &doc, &[node])
                    .expect("emitting")
                    .is_some_and(|c| c.app.handle.contains("self.form.handler_do_dono("));
            // or through a control linked to it (an Action runs on the
            // linked Button's click and updates before each sync)
            let via_link = {
                let mut doc = doc.clone();
                let mut b = Registry::builtin()
                    .instantiate(
                        "Button",
                        "cliente".to_string(),
                        Rect::new(8.0, 60.0, 80.0, 24.0),
                    )
                    .expect("the button");
                b.set_name("cliente");
                b.props
                    .insert("action".into(), crawrad_ir::Value::Ref(id.clone()));
                doc.add_child("form_main", b).expect("into the form");
                let nodes = [
                    doc.node(&id).expect("the node"),
                    doc.node("cliente").expect("the button"),
                ];
                let l = crate::targets::makepad::action_link::action_links(&doc, &nodes)
                    .expect("emitting");
                format!("{}{}", l.dispatch, l.pre_sync).contains("self.form.handler_do_dono(")
            };
            // or by the class's own actions (a PageControl switching pages)
            let via_actions = components::emit_actions_com(&r, &doc, node, "alvo", &[node])
                .expect("emitting")
                .contains("self.form.handler_do_dono(");
            // or by the class's own sync (a PaintBox runs OnPaint to
            // draw, every sync)
            let via_sync =
                components::emit_sync(&doc, node, "alvo").contains("self.form.handler_do_dono(");
            if binding::binding_of(classe, ev.name).is_none()
                && !via_sync
                && !via_handle_event
                && !via_pointer
                && !via_link
                && !via_actions
            {
                medida.push((classe.to_string(), ev.name.to_string()));
            }
        }
    }
    medida.sort();
    let registro: Vec<(String, String)> = DECLARADO_E_NAO_LIGADO
        .iter()
        .map(|(c, e)| (c.to_string(), e.to_string()))
        .collect();
    assert_eq!(
        medida, registro,
        "the unwired event debt changed. A NEW unwired event fails; \
         an event that became wired fails until the line leaves \
         `DECLARADO_E_NAO_LIGADO`. One today, zero in the end."
    );
}

/// **DEBT 2: event that the target wires and the catalogue does NOT declare.**
///
/// It is the more treacherous of the two, because it looks like finished work: there is code
/// emitting the check, it compiles, and it never runs — the Object Inspector does not
/// offer the event, so the node never carries it.
/// **And the first two lines are worse than they appeared, measured on
/// 2026-09-18.** `Panel` and `GroupBox` query
/// `self.ui.view(id!(campo)).finger_up(actions)`, and a Makepad `<View>` **only
/// hit-tests when it has `cursor`** — `makepad-widgets-1.0.0/src/view.rs:701`:
///
/// ```text
/// if self.visible && self.cursor.is_some() || self.animator.live_ptr.is_some() {
///     match event.hits_with_capture_overload(cx, self.area(), …)
/// ```
///
/// The `emit_dsl` of neither writes any `cursor`. Thus that code does not
/// run for TWO independent reasons: the Inspector does not offer the event, and the
/// framework would not produce it even if offered. Anyone removing the line here
/// must fix both.
///
/// **Empty.** `PageControl` began declaring
/// `on_change`. `Panel` and `GroupBox` declare `on_click`, and their `emit_dsl`
/// writes `cursor: Default` when the handler exists: both reasons
/// were fixed together.
const LIGADO_E_NAO_DECLARADO: &[(&str, &str)] = &[];

#[test]
fn a_ligacao_sem_evento_declarado_so_encolhe() {
    let r = Registry::builtin();
    let mut medida: Vec<(String, String)> = Vec::new();
    for classe in components::classes() {
        let desc = r.get(classe).expect("class in the catalogue");
        // **The probe compares WITH and WITHOUT the event**, and does not check whether text is
        // empty. The first version of this measurement checked emptiness, and measured incorrectly:
        // `CheckBox` emits its block either way — the widget `if`
        // always exists, and only the handler CALL is conditional. With the emptiness
        // ruler, nine classes appeared as debt when only two were.
        //
        // A guard that measures incorrectly is worse than none, and this one almost made it in.
        let base = acoes(classe, None);
        for candidato in ["on_click", "on_change"] {
            if desc.events.iter().any(|e| e.name == candidato) {
                continue;
            }
            if acoes_escritas_a_mao(classe, candidato) != base {
                medida.push((classe.to_string(), candidato.to_string()));
            }
        }
    }
    medida.sort();
    let registro: Vec<(String, String)> = LIGADO_E_NAO_DECLARADO
        .iter()
        .map(|(c, e)| (c.to_string(), e.to_string()))
        .collect();
    assert_eq!(
        medida, registro,
        "the binding without declared event changed. Either a class started emitting \
         code for an event it does not declare — dead code masquerading as a \
         feature —, or one that emitted stopped, and then the line leaves the registry."
    );
}

/// The report, which does not fail: it shows the size of the gap.
#[test]
fn quanto_o_bind_ja_cobre() {
    let r = Registry::builtin();
    let mut ligados = 0usize;
    let mut declarados = 0usize;
    let mut linhas = Vec::new();
    for classe in components::classes() {
        let desc = r.get(classe).expect("class in the catalogue");
        if desc.events.is_empty() {
            continue;
        }
        let mut ok = Vec::new();
        let mut falta = Vec::new();
        for ev in &desc.events {
            declarados += 1;
            match binding::binding_of(classe, ev.name) {
                Some(Binding {
                    accessor, query, ..
                }) => {
                    ligados += 1;
                    ok.push(format!("{} -> {accessor}.{query}", ev.name));
                }
                None => falta.push(ev.name.to_string()),
            }
        }
        linhas.push(format!(
            "{classe:<14} wires [{}] | MISSING [{}]",
            ok.join(", "),
            falta.join(", ")
        ));
    }
    println!(
        "\n=== THE `Bind` VERB ===\n{}\n\n{ligados} of {declarados} catalogue events are wired\n\
         accessors: {:?}\nqueries: {:?}\n",
        linhas.join("\n"),
        binding::accessors(),
        binding::queries()
    );
    assert!(ligados > 0, "no event wired — the table is empty");
}
