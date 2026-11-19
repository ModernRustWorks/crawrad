//! The GENERATED signature against the DECLARED signature, across the entire catalogue
//! and in both targets.
//!
//! # Why scan instead of writing one test per event
//!
//! The defect this file closes is **invisible to the gate**: a stub with
//! `&mut self` compiles because nothing calls it. It only appears when the
//! user writes the body and the parameter is missing — that is, right in front
//! of the user, not us. One test per event would cover today's eight and
//! none of those to come; the scan enforces the new event **the day it is
//! created**, which is why it asks the `Registry` instead of loading a
//! list written here.
//!
//! # The two questions, and the second was the missing one
//!
//! 1. does the generated handler have **the parameters declared by the catalogue**?
//! 2. do the **types** named by those parameters exist in the generated project?
//!
//! The second is what preserves the proper order. Reading the signature without emitting the
//! types generates code that does not compile — the handler would reference a type the project
//! lacks —, and that is probably why the Makepad target had a
//! hardcoded `&mut self`.

use super::*;
use crate::{Dono, Gerador};
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;

/// An empty form, sized after the `Blank form` template.
fn form(nome: &str) -> Document {
    let mut d = Document::new(nome, "Form");
    let raiz = d.node_mut(nome).expect("the newly-created root");
    raiz.set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
    raiz.set_name(nome);
    raiz.set_text("Form1");
    d
}

/// The document the IDE would have after double-clicking the event: a
/// component of the class, with the event wired to the default handler name.
fn com_o_evento_ligado(classe: &str, evento: &str) -> (Document, String) {
    let r = Registry::builtin();
    let mut d = form("form_main");
    if classe == "Form" {
        let handler = crate::handler_padrao("form_main", evento);
        d.node_mut("form_main")
            .expect("the root")
            .events
            .insert(evento.into(), handler.clone());
        d.validate().expect("valid");
        return (d, handler);
    }
    let (w, h) = r.get(classe).expect("catalogue class").default_size;
    let mut n = r
        .instantiate(classe, "sonda1", Rect::new(8.0, 8.0, w, h))
        .expect("the catalogue instantiates what it lists");
    n.set_name("sonda1");
    let handler = crate::handler_padrao("sonda1", evento);
    n.events.insert(evento.into(), handler.clone());
    d.add_child("form_main", n).expect("into the form");
    d.validate().expect("valid");
    (d, handler)
}

/// Both targets, by the name they present themselves with.
fn geradores() -> Vec<(&'static str, Box<dyn Gerador>)> {
    vec![
        ("egui", Box::new(crate::GeradorRxp) as Box<dyn Gerador>),
        ("makepad", Box::new(crate::GeradorMakepad)),
    ]
}

/// The user file and the designer file, in order, from what the target generated.
fn arquivos(g: &dyn Gerador, d: &Document) -> (String, String) {
    let arqs = g
        .gerar(d, "sonda", &crawrad_registry::Registry::builtin())
        .expect("generate");
    let usuario = arqs
        .iter()
        .find(|a| matches!(a.dono, Dono::Usuario))
        .expect("the user file");
    let designer = arqs
        .iter()
        .find(|a| {
            a.caminho
                .to_string_lossy()
                .ends_with(crate::SUFIXO_GERADO.trim_start_matches('.'))
        })
        .expect("the designer");
    (usuario.conteudo.clone(), designer.conteudo.clone())
}

/// The parameter list that the generated text actually declares for `handler`,
/// or `None` when the target wrote no handler at all.
fn parametros_gerados(fonte: &str, handler: &str) -> Option<String> {
    let agulha = format!("pub fn {handler}(");
    let i = fonte.find(&agulha)? + agulha.len();
    let mut profundidade = 1usize;
    for (k, c) in fonte[i..].char_indices() {
        match c {
            '(' => profundidade += 1,
            ')' => {
                profundidade -= 1;
                if profundidade == 0 {
                    return Some(fonte[i..i + k].to_string());
                }
            }
            _ => {}
        }
    }
    panic!("declaration of `{handler}` does not close parenthesis");
}

/// The types named in parameters — whatever starts with an uppercase letter, which is what
/// someone had to define somewhere.
fn tipos_citados(params: &str) -> Vec<String> {
    let mut tipos = Vec::new();
    let mut atual = String::new();
    for c in params.chars() {
        if c.is_alphanumeric() || c == '_' {
            atual.push(c);
            continue;
        }
        if !atual.is_empty() {
            tipos.push(std::mem::take(&mut atual));
        }
    }
    if !atual.is_empty() {
        tipos.push(atual);
    }
    tipos.retain(|t| t.chars().next().is_some_and(char::is_uppercase) && t != "Self");
    tipos.sort();
    tipos.dedup();
    tipos
}

/// Every (class, event) pair in the catalogue, with no duplicates.
fn todo_evento_do_catalogo() -> Vec<(&'static str, &'static str)> {
    let r = Registry::builtin();
    let mut pares = Vec::new();
    for desc in r.all() {
        // Components that don't generate code have no handlers to test.
        if !desc.emits_code {
            continue;
        }
        for ev in &desc.events {
            pares.push((desc.class, ev.name));
        }
    }
    pares
}

/// The declared signature for this pair.
fn declarada(classe: &str, evento: &str) -> String {
    method_params(&crawrad_registry::Registry::builtin(), classe, evento)
}

/// **The scan.** The generated handler declares what the catalogue declares — in
/// both targets, for every class and every event.
#[test]
fn a_assinatura_gerada_e_a_declarada_no_catalogo_nos_dois_alvos() {
    let mut medidos = 0usize;
    let mut com_parametro = 0usize;
    let mut sem_handler: Vec<String> = Vec::new();
    for (classe, evento) in todo_evento_do_catalogo() {
        let (doc, handler) = com_o_evento_ligado(classe, evento);
        let esperado = declarada(classe, evento);
        for (alvo, g) in geradores() {
            let (usuario, _) = arquivos(g.as_ref(), &doc);
            let Some(saiu) = parametros_gerados(&usuario, &handler) else {
                // Some classes have events that the target does not yet turn into any
                // handler — egui's `ActionList` is one of them. This is DEBT from
                // another topic, and this test does not measure it; what it will not
                // allow is for debt to become a hiding place: an event WITH parameters
                // that generates no handler would be this defect, silent.
                assert_eq!(
                    esperado, "&mut self",
                    "target `{alvo}` did not generate a handler for `{classe}.{evento}`, \
                     which has a declared parameter"
                );
                sem_handler.push(format!("{alvo}: {classe}.{evento}"));
                continue;
            };
            assert_eq!(
                saiu, esperado,
                "target `{alvo}` wrote a different signature for `{classe}.{evento}`"
            );
            if esperado != "&mut self" {
                com_parametro += 1;
            }
            medidos += 1;
        }
    }
    println!(
        "scanned {medidos} handlers in both targets, {com_parametro} with parameters; \
         {} pairs with no handler at all: {sem_handler:?}",
        sem_handler.len()
    );
    // Eight catalogue events have parameters and two targets write them: the floor
    // is 16. It exists for the day the scan stops scanning.
    assert!(
        medidos > 0 && com_parametro >= 16,
        "scan measured {medidos} handlers and only {com_parametro} with parameters — \
         the catalogue shrank or this test stopped measuring. Without handler: {sem_handler:?}"
    );
}

/// What preserves the ORDER: the type referenced in the signature exists in the target's
/// designer. Without this, reading the signature from the catalogue would generate a handler
/// that fails to compile.
#[test]
fn todo_tipo_citado_na_assinatura_e_definido_pelo_designer() {
    let mut cobrados = 0usize;
    for (classe, evento) in todo_evento_do_catalogo() {
        let esperado = declarada(classe, evento);
        let tipos = tipos_citados(&esperado);
        if tipos.is_empty() {
            continue;
        }
        let (doc, _) = com_o_evento_ligado(classe, evento);
        for (alvo, g) in geradores() {
            let (_, designer) = arquivos(g.as_ref(), &doc);
            for t in &tipos {
                assert!(
                    designer.contains(&format!("pub struct {t} "))
                        || designer.contains(&format!("pub struct {t}{{"))
                        || designer.contains(&format!("pub enum {t} ")),
                    "target `{alvo}` mentions `{t}` in the signature of `{classe}.{evento}` \
                     and does not define that type in the designer"
                );
                cobrados += 1;
            }
        }
    }
    assert!(
        cobrados > 0,
        "no catalogue event mentions any type — this test stopped measuring"
    );
}

/// The `OnKeyDown` of a class that declares it: double-clicking the event
/// gives a handler with its parameter. (It used an `Edit`, which never
/// declared `on_key_down` — it passed only because nobody checked the event
/// against the catalog.)
#[test]
fn on_key_down_handler_gets_its_parameter_in_both_targets() {
    let (doc, handler) = com_o_evento_ligado("Button", "on_key_down");
    assert_eq!(handler, "sonda1_key_down");
    for (alvo, g) in geradores() {
        let (usuario, designer) = arquivos(g.as_ref(), &doc);
        assert!(
            usuario.contains("pub fn sonda1_key_down(&mut self, _e: KeyEvent)"),
            "target `{alvo}` did not write the parameter:\n{usuario}"
        );
        assert!(
            designer.contains("pub struct KeyEvent {"),
            "target `{alvo}` did not define the `KeyEvent` that the handler mentions"
        );
    }
}

/// Both targets emit the SAME type text — not two similar copies.
///
/// `OnKeyDown` on a `Button` pulls in three types: `KeyEvent` named by the
/// signature and the two it contains inside. Both targets must emit these three
/// **byte for byte** — it is the same `crate::events` for both, and the question here
/// is whether either of them rewrote the text along the way.
#[test]
fn os_dois_alvos_emitem_os_mesmos_tipos_de_evento() {
    let (doc, _) = com_o_evento_ligado("Button", "on_key_down");
    for tipo in ["KeyEvent", "Key", "ShiftState"] {
        let texto = crate::events::PAYLOAD_TABLE
            .iter()
            .find(|t| t.name == tipo)
            .expect("the type is in the table")
            .text;
        for (alvo, g) in geradores() {
            let (_, designer) = arquivos(g.as_ref(), &doc);
            assert!(
                designer.contains(texto),
                "target `{alvo}` did not emit the full `{tipo}`"
            );
        }
    }
}

/// What the form DOES NOT name does not enter.
///
/// This is the side hidden by the designer's `#![allow(dead_code)]`: a
/// `MouseEvent` in a form that only wires `OnKeyDown` is an item nobody
/// constructs, and in a binary `pub` saves no one — it is `dead_code`, and the
/// user cannot fix a file that is rewritten on the next generation.
#[test]
fn o_tipo_que_nenhum_handler_nomeia_nao_e_emitido() {
    let (doc, _) = com_o_evento_ligado("Button", "on_key_down");
    for (alvo, g) in geradores() {
        let (_, designer) = arquivos(g.as_ref(), &doc);
        for ausente in ["MouseEvent", "MouseMoveEvent", "MouseActivateEvent"] {
            assert!(
                !designer.contains(&format!("pub struct {ausente} {{")),
                "target `{alvo}` emitted `{ausente}`, which no handler in this \
                 form mentions"
            );
        }
    }
}

/// And a form with no events carries no payload type whatsoever.
#[test]
fn o_formulario_em_branco_nao_leva_tipo_de_carga() {
    let doc = form("form_main");
    for (alvo, g) in geradores() {
        let (_, designer) = arquivos(g.as_ref(), &doc);
        for t in crate::events::PAYLOAD_TABLE {
            assert!(
                !designer.contains(t.text),
                "target `{alvo}` emitted `{}` in a form without any events",
                t.name
            );
        }
    }
}

/// No type in the user contract speaks the language of a framework.
///
/// `KeyEvent` once had `key: egui::Key` inside, and that was the ONLY place where
/// a framework type crossed over to the user file. The same text
/// goes today to both targets, and in one of them `egui` does not exist.
#[test]
fn o_bloco_de_tipos_nao_cita_framework_nenhum() {
    for palavra in ["egui", "eframe", "makepad", "winit"] {
        for t in crate::events::PAYLOAD_TABLE {
            assert!(
                !t.text.contains(palavra),
                "`{}` mentions `{palavra}`",
                t.name
            );
        }
    }
}

/// **Fixed text in both files cannot contain handler-like names** —
/// and this is not stylistic nitpicking.
///
/// A handler's name is `<component><Event>`, and ADR 0025 pruning
/// looks for that name on disk to decide whether an empty handler can be removed. A
/// concrete example written in a COMMENT in the prelude — which is what happened
/// here, with `btn_ok` followed by the click suffix — goes literally into
/// every user's file, survives pruning inside the comment, and the ADR 0025
/// gate (residing in `crawrad-ide`) fails the entire tree.
///
/// That gate caught the defect; this test catches it **in the crate that
/// writes it**, where it is cheap to fix.
///
/// **The criterion is the full name, not the suffix.** I measured both: enforcing only
/// the suffix rejects legitimate text — `Key::Enter` in the type block and
/// `OnKeyDown` in prose would fail, and a guard that must be
/// relaxed to pass is no guard. What is required here is the full
/// name — `<component><Event>` — for the component names that an
/// example in comments would actually use, against **every** event in the
/// catalogue. New events enter the check the day they are created.
#[test]
fn o_preludio_dos_arquivos_gerados_nao_cita_nome_de_handler() {
    let eventos: Vec<&'static str> = Registry::builtin()
        .all()
        .flat_map(|d| d.events.iter().map(|e| e.name))
        .collect();
    // The names that a hand-written example would use — what went wrong this
    // time was precisely one of them.
    let componentes = [
        "btn_ok",
        "btn1",
        "form_main",
        "Button1",
        "Edit1",
        "edt_nome",
    ];
    // A form WITHOUT any events: everything emitted in both files is
    // our fixed text.
    let d = form("form_main");
    for (alvo, g) in geradores() {
        let (usuario, designer) = arquivos(g.as_ref(), &d);
        for (qual, fonte) in [("user file", &usuario), ("designer file", &designer)] {
            for c in componentes {
                for e in &eventos {
                    let h = crate::handler_padrao(c, e);
                    assert!(
                        !fonte.contains(&h),
                        "target `{alvo}` wrote `{h}` in the fixed text of the {qual} — \
                         pruning looks for handler names on disk, and a comment \
                         example passes as a real name"
                    );
                }
            }
        }
    }
}

/// The question about parameters is ONE, and it is the catalogue's.
#[test]
fn a_classe_que_nao_declara_o_evento_ainda_recebe_a_assinatura_certa() {
    // `Button` does not declare `on_key_down`; scanning the catalogue is what
    // prevents the handler from starting with a bare `&mut self` if it is ever
    // wired there.
    assert_eq!(
        method_params(
            &crawrad_registry::Registry::builtin(),
            "Button",
            "on_key_down"
        ),
        "&mut self, _e: KeyEvent"
    );
    // An event that does not exist anywhere falls back to the catch-all, and the
    // user file keeps compiling.
    assert_eq!(
        method_params(
            &crawrad_registry::Registry::builtin(),
            "Button",
            "on_nada_disso"
        ),
        "&mut self"
    );
}

/// **An Edit's own `OnKeyDown` reaches its handler in both targets** (owner,
/// 21/10): egui dispatches the keys of the frame while the Edit has the
/// focus, through the same `crawrad_key`/`crawrad_shift` conversions the form
/// uses; Makepad routes it through the focus router like any control that
/// takes the focus.
#[test]
fn an_edit_on_key_down_is_dispatched_in_both_targets() {
    let (doc, handler) = com_o_evento_ligado("Edit", "on_key_down");
    assert_eq!(handler, "sonda1_key_down");
    for (alvo, g) in geradores() {
        let (usuario, designer) = arquivos(g.as_ref(), &doc);
        assert!(
            usuario.contains("pub fn sonda1_key_down(&mut self, _e: KeyEvent)"),
            "target `{alvo}` did not write the handler:\n{usuario}"
        );
        assert!(
            designer.contains("sonda1_key_down("),
            "target `{alvo}` does not call the handler:\n{designer}"
        );
    }
    let (_, egui) = arquivos(geradores()[0].1.as_ref(), &doc);
    assert!(
        egui.contains("res.has_focus() || res.lost_focus()"),
        "{egui}"
    );
    assert!(
        egui.contains("fn crawrad_key(") && egui.contains("fn crawrad_shift("),
        "{egui}"
    );
}

/// An Edit's `OnKeyUp` alone: the release is dispatched, the press is not.
#[test]
fn an_edit_on_key_up_alone_dispatches_only_the_release() {
    let (doc, _) = com_o_evento_ligado("Edit", "on_key_up");
    let (_, egui) = arquivos(geradores()[0].1.as_ref(), &doc);
    assert!(egui.contains("if !pressed {"), "{egui}");
    assert!(egui.contains("self.sonda1_key_up(e)"), "{egui}");
}

/// **The events a target still does not call**, as `(target, class, event)`.
/// The list only shrinks: the gate below fails both when one of these starts
/// being called (take it out) and when a new one goes mute.
///
/// - `Form.on_hide`: the generated app has one form and never hides it.
/// - `PopupMenu.on_popup`: the menu opens and says nothing yet (the same
///   debt as `verb_bind`'s `DECLARADO_E_NAO_LIGADO`).
const STILL_MUTE: &[(&str, &str, &str)] = &[
    ("egui", "Form", "on_hide"),
    ("makepad", "Form", "on_hide"),
    ("egui", "PopupMenu", "on_popup"),
    ("makepad", "PopupMenu", "on_popup"),
];

/// The form with `class`'s `event` wired, where the event can happen: an
/// `Action` runs through the button linked to it, a `MenuItem` lives
/// in its menu.
fn wired_where_it_happens(class: &str, event: &str) -> (Document, String) {
    let (mut d, handler) = com_o_evento_ligado(class, event);
    let r = Registry::builtin();
    match class {
        "Action" => {
            let mut b = r
                .instantiate("Button", "linked1", Rect::new(8.0, 60.0, 80.0, 24.0))
                .expect("the button");
            b.set_name("linked1");
            b.props
                .insert("action".into(), crawrad_ir::Value::Ref("sonda1".into()));
            d.add_child("form_main", b).expect("into the form");
        }
        "MenuItem" => {
            d = form("form_main");
            let mut menu = r
                .instantiate("MainMenu", "menu1", Rect::new(0.0, 0.0, 28.0, 28.0))
                .expect("the menu");
            menu.set_name("menu1");
            d.add_child("form_main", menu).expect("into the form");
            let mut item = r
                .instantiate("MenuItem", "sonda1", Rect::new(0.0, 0.0, 28.0, 28.0))
                .expect("the item");
            item.set_name("sonda1");
            item.events.insert(event.into(), handler.clone());
            d.add_child("menu1", item).expect("into the menu");
        }
        _ => {}
    }
    d.validate().expect("valid");
    (d, handler)
}

/// **Every event the catalogue declares is CALLED in both targets**, but the
/// ones in [`STILL_MUTE`]. A class that declares an event promises it: a
/// handler the generated app never calls is a lie in the Object Inspector.
#[test]
fn every_declared_event_is_called_in_both_targets() {
    let mut mute = Vec::new();
    for (class, event) in todo_evento_do_catalogo() {
        let (doc, handler) = wired_where_it_happens(class, event);
        for (target, g) in geradores() {
            let (_, designer) = arquivos(g.as_ref(), &doc);
            if !designer.contains(&format!("{handler}(")) {
                mute.push((target, class, event));
            }
        }
    }
    let known: Vec<_> = STILL_MUTE.to_vec();
    assert_eq!(
        mute, known,
        "the events no target calls changed: shrink `STILL_MUTE`, never grow it"
    );
}

/// The egui tree folds: a click on a node with children unfolds a folded one
/// (`OnExpanding` first) and folds an open one (`OnCollapsing` first).
#[test]
fn an_egui_tree_announces_the_fold_before_it_happens() {
    for (event, when, then) in [
        ("on_expanding", "if at.is_some() {", ".collapsed.remove(i)"),
        (
            "on_collapsing",
            "if at.is_none() {",
            ".collapsed.push(path)",
        ),
    ] {
        let (doc, handler) = com_o_evento_ligado("TreeView", event);
        let (_, egui) = arquivos(geradores()[0].1.as_ref(), &doc);
        let announce = egui.find(when).expect(when);
        let call = egui.find(&format!("self.{handler}()")).expect("the call");
        let fold = egui.find(then).expect(then);
        assert!(
            announce < call && call < fold,
            "`{event}` must be told before the tree changes:\n{egui}"
        );
    }
}

/// A ComboBox's `OnSelect` fires on every pick, the same item or not, and
/// `OnChange` only when the choice is a new one — in both targets.
#[test]
fn a_combo_box_selects_on_every_pick_and_changes_only_on_a_new_one() {
    let (mut doc, select) = com_o_evento_ligado("ComboBox", "on_select");
    let change = crate::handler_padrao("sonda1", "on_change");
    doc.node_mut("sonda1")
        .expect("the combo")
        .events
        .insert("on_change".into(), change.clone());
    let (_, egui) = arquivos(geradores()[0].1.as_ref(), &doc);
    assert!(egui.contains("picked = true;"), "{egui}");
    let picked = egui.find("if picked {").expect("the pick");
    let call = egui[picked..]
        .find(&format!("self.{select}()"))
        .expect("the select call");
    let changed = egui[picked..].find("if changed {").expect("the change");
    assert!(call < changed, "{egui}");
    let (_, makepad) = arquivos(geradores()[1].1.as_ref(), &doc);
    assert!(
        makepad.contains("let changed = self.form.sonda1.selected != Some(_val) || "),
        "{makepad}"
    );
    let at = makepad
        .find(&format!("self.form.{select}()"))
        .expect("select");
    let ch = makepad
        .find(&format!("self.form.{change}()"))
        .expect("change");
    assert!(at < ch, "{makepad}");
}

#[test]
fn a_list_view_announces_the_row_that_leaves_before_the_one_that_arrives() {
    let (mut doc, select) = com_o_evento_ligado("ListView", "on_select_item");
    let column = crate::handler_padrao("sonda1", "on_column_click");
    doc.node_mut("sonda1")
        .expect("the list")
        .events
        .insert("on_column_click".into(), column.clone());
    let (_, egui) = arquivos(geradores()[0].1.as_ref(), &doc);
    let leaves = egui
        .find(&format!("self.{select}(before_item as usize, false)"))
        .expect("the row that leaves");
    let arrives = egui
        .find(&format!(
            "self.{select}(self.sonda1.item_index as usize, true)"
        ))
        .expect("the row that arrives");
    assert!(leaves < arrives, "{egui}");
    assert!(egui.contains("column_clicked = Some(idx);"), "{egui}");
    assert!(egui.contains(&format!("self.{column}(column)")), "{egui}");
    let (_, makepad) = arquivos(geradores()[1].1.as_ref(), &doc);
    let leaves = makepad
        .find(&format!("self.form.{select}(before_item as usize, false)"))
        .expect("the row that leaves");
    let arrives = makepad
        .find(&format!("self.form.{select}(_val, true)"))
        .expect("the row that arrives");
    assert!(leaves < arrives, "{makepad}");
    assert!(
        makepad.contains(&format!("self.form.{column}(_col)")),
        "{makepad}"
    );
}

#[test]
fn a_tree_view_asks_before_it_selects_and_tells_after_it_folds() {
    let (mut doc, changing) = com_o_evento_ligado("TreeView", "on_changing");
    for e in ["on_expanded", "on_collapsed", "on_change"] {
        let h = crate::handler_padrao("sonda1", e);
        doc.node_mut("sonda1")
            .expect("the tree")
            .events
            .insert(e.into(), h);
    }
    let expanded = crate::handler_padrao("sonda1", "on_expanded");
    let collapsed = crate::handler_padrao("sonda1", "on_collapsed");
    let (_, egui) = arquivos(geradores()[0].1.as_ref(), &doc);
    let ask = egui
        .find(&format!("self.{changing}(&mut allow_change);"))
        .expect("the question");
    let take = egui
        .find("self.sonda1.selection = path;")
        .expect("the pick");
    assert!(ask < take, "{egui}");
    let open = egui
        .find("self.sonda1.collapsed.remove(i);")
        .expect("the unfold");
    let told = egui.find(&format!("self.{expanded}();")).expect("expanded");
    assert!(open < told, "{egui}");
    let fold = egui
        .find("self.sonda1.collapsed.push(path);")
        .expect("the fold");
    let told = egui
        .find(&format!("self.{collapsed}();"))
        .expect("collapsed");
    assert!(fold < told, "{egui}");
    let (_, makepad) = arquivos(geradores()[1].1.as_ref(), &doc);
    let ask = makepad
        .find(&format!("self.form.{changing}(&mut allow_change);"))
        .expect("the question");
    let take = makepad
        .find("self.form.sonda1.selected = Some(")
        .expect("the pick");
    assert!(ask < take, "{makepad}");
    let toggle = makepad
        .find("self.form.sonda1.toggle_collapse(")
        .expect("the toggle");
    let told = makepad
        .find(&format!("self.form.{expanded}();"))
        .expect("expanded");
    assert!(toggle < told, "{makepad}");
    assert!(makepad.contains("if allow_change {"), "{makepad}");
}

#[test]
fn a_panel_hears_the_wheel_and_its_own_resize_in_both_targets() {
    let (mut doc, wheel) = com_o_evento_ligado("Panel", "on_mouse_wheel");
    let resize = crate::handler_padrao("sonda1", "on_resize");
    doc.node_mut("sonda1")
        .expect("the panel")
        .events
        .insert("on_resize".into(), resize.clone());
    let (_, egui) = arquivos(geradores()[0].1.as_ref(), &doc);
    assert!(egui.contains("CrawradPointer::Wheel"), "{egui}");
    assert!(egui.contains(&format!("self.{wheel}()")), "{egui}");
    assert!(
        egui.contains("crawrad_sizes.push((0, r.size()));"),
        "{egui}"
    );
    assert!(
        egui.contains("self.crawrad_sizes.resized(&crawrad_sizes)"),
        "{egui}"
    );
    assert!(egui.contains(&format!("self.{resize}()")), "{egui}");
    let (_, makepad) = arquivos(geradores()[1].1.as_ref(), &doc);
    assert!(makepad.contains("CrawradPointer::Wheel"), "{makepad}");
    assert!(
        makepad.contains("self.crawrad_sizes.resized(&sizes)"),
        "{makepad}"
    );
    assert!(makepad.contains("cx.new_next_frame();"), "{makepad}");
    assert!(
        makepad.contains(&format!("self.form.{resize}()")),
        "{makepad}"
    );
}

#[test]
fn no_resize_handler_no_size_watch_in_the_app() {
    let (doc, _) = com_o_evento_ligado("Panel", "on_mouse_wheel");
    let (_, egui) = arquivos(geradores()[0].1.as_ref(), &doc);
    assert!(!egui.contains("CrawradSizeWatch"), "{egui}");
    let (_, makepad) = arquivos(geradores()[1].1.as_ref(), &doc);
    assert!(!makepad.contains("CrawradSizeWatch"), "{makepad}");
}

/// Only a handler with no argument, or one owned argument, can be reported
/// to a derived form: two arguments or a borrowed one stay direct calls.
#[test]
fn only_owned_single_argument_handlers_are_reportable() {
    use super::handler_args;
    assert!(handler_args("&mut self").is_empty());
    assert_eq!(handler_args("&mut self, e: KeyEvent"), ["e: KeyEvent"]);
    assert_eq!(
        handler_args("&mut self, m: HashMap<String, Vec<u8>>, n: u8"),
        ["m: HashMap<String, Vec<u8>>", "n: u8"]
    );
    assert_eq!(
        handler_args("&mut self, f: fn(u8, u8) -> std::cmp::Ordering, n: u8"),
        ["f: fn(u8, u8) -> std::cmp::Ordering", "n: u8"]
    );
    let reg = crawrad_registry::Registry::builtin();
    assert!(super::is_reportable(&reg, "Button", "on_click"));
    let two = reg
        .all()
        .flat_map(|d| {
            d.events
                .iter()
                .map(move |e| (d.class.to_string(), e.name.to_string()))
        })
        .find(|(c, e)| super::method_params(&reg, c, e).matches(':').count() >= 2);
    if let Some((class, event)) = two {
        assert!(
            !super::is_reportable(&reg, &class, &event),
            "{class}.{event}"
        );
    }
}
