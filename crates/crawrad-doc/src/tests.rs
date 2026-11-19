use crate::*;
use crawrad_ir::{Node, Rect, Value};

fn doc() -> Doc {
    let mut d = crawrad_ir::Document::new("form_main", "Form");
    let mut btn = Node::new("btn_ok", "Button");
    btn.set_text("&OK");
    btn.set_rect(Rect::new(24.0, 96.0, 96.0, 32.0));
    d.add_child("form_main", btn).unwrap();
    let mut edt = Node::new("edt_nome", "Edit");
    edt.set_rect(Rect::new(24.0, 48.0, 280.0, 24.0));
    d.add_child("form_main", edt).unwrap();
    Doc::new(d)
}

fn rect_of(d: &Doc, id: &str) -> Rect {
    d.ir().node(id).unwrap().rect().unwrap()
}

fn mover(node: &str, x: f32, y: f32) -> Command {
    Command::SetRect {
        node: node.into(),
        rect: Rect::new(x, y, 96.0, 32.0),
    }
}

#[test]
fn mover_e_desfazer_volta_ao_lugar() {
    let mut d = doc();
    let antes = rect_of(&d, "btn_ok");

    d.apply_one("move btn_ok", mover("btn_ok", 72.0, 120.0))
        .unwrap();
    assert_eq!(rect_of(&d, "btn_ok").x, 72.0);

    d.undo().unwrap();
    assert_eq!(rect_of(&d, "btn_ok"), antes);

    d.redo().unwrap();
    assert_eq!(rect_of(&d, "btn_ok").y, 120.0);
}

#[test]
fn undo_redo_alternado_nao_embaralha_as_pilhas() {
    let mut d = doc();
    for x in [10.0, 20.0, 30.0] {
        d.apply_one("move", mover("btn_ok", x, 0.0)).unwrap();
    }
    d.undo().unwrap();
    assert_eq!(rect_of(&d, "btn_ok").x, 20.0);
    d.undo().unwrap();
    assert_eq!(rect_of(&d, "btn_ok").x, 10.0);
    d.redo().unwrap();
    assert_eq!(rect_of(&d, "btn_ok").x, 20.0);
    d.redo().unwrap();
    assert_eq!(rect_of(&d, "btn_ok").x, 30.0);
    assert!(!d.can_redo());
}

#[test]
fn edicao_nova_limpa_o_refazer() {
    let mut d = doc();
    d.apply_one("a", mover("btn_ok", 1.0, 0.0)).unwrap();
    d.undo().unwrap();
    assert!(d.can_redo());
    d.apply_one("b", mover("btn_ok", 2.0, 0.0)).unwrap();
    assert!(
        !d.can_redo(),
        "editing after undo must truncate redo history"
    );
}

#[test]
fn lote_desfaz_como_um_so_ctrl_z() {
    // the guarantee that allows an erroneous agent to be undone with a shortcut
    let mut d = doc();
    let tx = Transaction::new("align selection", Author::Agent("claude".into()))
        .with(mover("btn_ok", 0.0, 0.0))
        .with(mover("edt_nome", 0.0, 40.0));
    d.apply(tx).unwrap();
    assert_eq!(rect_of(&d, "btn_ok").x, 0.0);
    assert_eq!(rect_of(&d, "edt_nome").x, 0.0);

    d.undo().unwrap();
    assert_eq!(rect_of(&d, "btn_ok").x, 24.0);
    assert_eq!(rect_of(&d, "edt_nome").y, 48.0);
    assert!(!d.can_undo(), "the batch is ONE history entry, not two");
}

#[test]
fn transacao_falha_no_meio_e_nada_entra() {
    let mut d = doc();
    let antes = rect_of(&d, "btn_ok");
    let tx = Transaction::user("half-baked")
        .with(mover("btn_ok", 9.0, 9.0))
        .with(mover("fantasma", 0.0, 0.0));
    assert!(d.apply(tx).is_err());
    assert_eq!(
        rect_of(&d, "btn_ok"),
        antes,
        "the first command should have been rolled back"
    );
    assert!(!d.can_undo(), "failed transaction does not enter history");
}

#[test]
fn preview_nao_muda_nada() {
    let d = doc();
    let rev = d.revision();
    let tx =
        Transaction::new("agent proposes", Author::Agent("claude".into())).with(Command::SetProp {
            node: "btn_ok".into(),
            key: "text".into(),
            value: Some(Value::Str("Save".into())),
        });
    let p = d.preview(&tx).unwrap();
    assert_eq!(p.result.node("btn_ok").unwrap().text(), Some("Save"));
    assert_eq!(
        d.ir().node("btn_ok").unwrap().text(),
        Some("&OK"),
        "the real document must not have changed"
    );
    assert_eq!(d.revision(), rev);
    assert!(p.touched.contains("btn_ok"));
}

/// **The key that THIS crate can judge is blocked before dirtying the
/// document.**
///
/// # What it measured before, and who judges now
///
/// It inserted `PasswordChar` and asserted rejection — which came from the `COMMON`
/// list of forty names in `crawrad-ir`. Amendment 1 of ADR 0031 stripped that list of
/// its gatekeeping role, because it was a duplicate of what the component already
/// declares.
///
/// `crawrad-doc` does not know classes — the catalog is a `dev-dependency` of it, only
/// for gates —, so what it blocks is what can be judged without any catalog:
/// **unknown target and malformed key**. The question *"does the class declare
/// this?"* is asked in two places that have the catalog in hand:
/// `component.set_property` in `crawrad-actions` (the single mutation surface
/// published to agents) and `propriedade_da_classe::conferir` in
/// `crawrad-registry`.
#[test]
fn a_chave_que_este_crate_julga_e_barrada_na_porta() {
    let mut d = doc();
    let e = d.apply_one(
        "target this IDE does not know",
        Command::SetProp {
            node: "btn_ok".into(),
            key: "qt.Frame".into(),
            value: Some(Value::Bool(true)),
        },
    );
    assert!(e.is_err(), "SetProp blocks before dirtying the document");
    assert!(
        !d.ir()
            .node("btn_ok")
            .unwrap()
            .props
            .contains_key("qt.Frame")
    );

    // And the key that it NO LONGER judges passes through here — whatever rejects it is what
    // has the catalog.
    d.apply_one(
        "prefixless, and the class is what judges",
        Command::SetProp {
            node: "btn_ok".into(),
            key: "PasswordChar".into(),
            value: Some(Value::Str("*".into())),
        },
    )
    .expect("the contract without catalogue cannot judge this");
}

#[test]
fn propriedade_namespeada_passa() {
    let mut d = doc();
    d.apply_one(
        "password",
        Command::SetProp {
            node: "edt_nome".into(),
            key: "win32.PasswordChar".into(),
            value: Some(Value::Str("*".into())),
        },
    )
    .unwrap();
    assert!(d.ir().validate().is_ok());
}

#[test]
fn adicionar_e_desfazer_remove() {
    let mut d = doc();
    let novo = Node::new("btn_cancel", "Button");
    d.apply_one(
        "add",
        Command::AddNode {
            parent: "form_main".into(),
            node: novo,
            index: 1,
        },
    )
    .unwrap();
    assert_eq!(d.ir().node("form_main").unwrap().children[1], "btn_cancel");
    assert!(d.ir().validate().is_ok());

    d.undo().unwrap();
    assert!(d.ir().node("btn_cancel").is_none());
    assert!(d.ir().validate().is_ok());
}

#[test]
fn remover_e_desfazer_volta_na_mesma_posicao() {
    let mut d = doc();
    d.apply_one(
        "remove",
        Command::RemoveNode {
            node: "btn_ok".into(),
        },
    )
    .unwrap();
    assert!(d.ir().node("btn_ok").is_none());

    d.undo().unwrap();
    let filhos = &d.ir().node("form_main").unwrap().children;
    assert_eq!(
        filhos[0], "btn_ok",
        "must return to original position, not at the end"
    );
    assert_eq!(
        d.ir().node("btn_ok").unwrap().text(),
        Some("&OK"),
        "with properties intact"
    );
}

#[test]
fn reordenar_muda_o_z_e_desfaz() {
    let mut d = doc();
    d.apply_one(
        "z",
        Command::Reorder {
            parent: "form_main".into(),
            from: 0,
            to: 1,
        },
    )
    .unwrap();
    assert_eq!(
        d.ir().node("form_main").unwrap().children,
        vec!["edt_nome", "btn_ok"]
    );
    d.undo().unwrap();
    assert_eq!(
        d.ir().node("form_main").unwrap().children,
        vec!["btn_ok", "edt_nome"]
    );
}

#[test]
fn renomear_nao_toca_na_identidade() {
    // The opposite of what this test required prior to ADR 0008. Renaming
    // changed the node key, and therefore needed to rewrite the parent's child
    // list and everyone's references. Now `id` is stable
    // and none of that happens: only the label changes.
    let mut d = doc();
    d.apply_one(
        "rename",
        Command::Rename {
            node: "btn_ok".into(),
            name: "btn_gravar".into(),
            handlers: true,
        },
    )
    .unwrap();

    let no = d.ir().node("btn_ok").expect("the id does NOT change");
    assert_eq!(no.name(), "btn_gravar", "what changes is the name");
    assert!(
        d.ir()
            .node("form_main")
            .unwrap()
            .children
            .contains(&"btn_ok".to_string()),
        "the parent's children list is not rewritten"
    );
    assert!(d.ir().node_by_name("btn_gravar").is_some());
    assert!(d.ir().validate().is_ok());

    d.undo().unwrap();
    assert_eq!(d.ir().node("btn_ok").unwrap().name(), "btn_ok");
    assert!(d.ir().validate().is_ok());
}

#[test]
fn dois_componentes_nao_podem_ter_o_mesmo_nome() {
    // Uniqueness did not disappear with ADR 0008 — it changed from structural (map
    // key) to a property rule. And remains mandatory because
    // codegen turns Name into a field.
    let mut d = doc();
    let e = d
        .apply_one(
            "rename",
            Command::Rename {
                node: "btn_ok".into(),
                name: "edt_nome".into(),
                handlers: true,
            },
        )
        .unwrap_err();
    assert!(
        matches!(e, DocError::Ir(crawrad_ir::IrError::DuplicateName(_))),
        "got {e:?}"
    );
    assert_eq!(d.ir().node("btn_ok").unwrap().name(), "btn_ok");
}

#[test]
fn nome_vazio_e_recusado() {
    let mut d = doc();
    let e = d
        .apply_one(
            "rename",
            Command::Rename {
                node: "btn_ok".into(),
                name: "   ".into(),
                handlers: true,
            },
        )
        .unwrap_err();
    assert!(matches!(e, DocError::Ir(crawrad_ir::IrError::EmptyName(_))));
}

#[test]
fn nome_so_de_sublinhado_e_recusado() {
    // `"___".trim()` is **not** empty, and so it passed the empty-name
    // guard right above. After that the generator derived an empty type, and
    // the rename migrator erased the word `FormMain` from inside the file
    // that the user had handwritten. Code loss, not an ugly file —
    // the full extent of the damage is in
    // `crawrad-codegen`, test `nome_so_de_sublinhado_nao_chega_a_tocar_o_arquivo_do_usuario`.
    //
    // The guard lives here because this is **the** write gateway: validation
    // in the shell could be bypassed by any other caller.
    for n in ["_", "__", "___"] {
        let mut d = doc();
        let e = d
            .apply_one(
                "rename",
                Command::Rename {
                    node: "btn_ok".into(),
                    name: n.into(),
                    handlers: true,
                },
            )
            .unwrap_err();
        assert!(
            matches!(&e, DocError::Ir(crawrad_ir::IrError::NameWithoutAlnum(x)) if x == n),
            "{n:?} got {e:?}"
        );
        // And the name did not move.
        assert_eq!(d.ir().node("btn_ok").unwrap().name(), "btn_ok");
    }
}

#[test]
fn a_mensagem_do_nome_sem_alfanumerico_diz_o_que_fazer() {
    // `EmptyName` would say "node 'btn_ok' has no name" to someone who just typed
    // three underscores in the Object Inspector — and the field on their screen has three
    // characters. That is why this has its own variant.
    let mut d = doc();
    let e = d
        .apply_one(
            "rename",
            Command::Rename {
                node: "btn_ok".into(),
                name: "___".into(),
                handlers: true,
            },
        )
        .unwrap_err();
    let msg = e.to_string();
    assert!(
        msg.contains("___") && msg.contains("letter or digit"),
        "{msg}"
    );
}

#[test]
fn os_vizinhos_do_nome_ruim_no_rename() {
    // What this guard catches and what it intentionally does **not** catch.
    //
    // The boundary changed on 2026-09-16. `1abc` and `botão` passed through here —
    // they were "a Rust rule, not a document rule", and what rejected them was
    // `crawrad-codegen::identificador` inside `gerar()`. With `snake_case`
    // enforced on input they fail here, and fail **with the correct form printed**:
    // the generator rejects without suggesting, and rejects when the user already forgot
    // what they typed.
    //
    // `fn` still passes, and the line is exactly this: a Rust reserved keyword
    // IS snake_case. What rejects it continues to be `identificador()`, in
    // `gerar()`. Is it worth bringing here too? Yes, and it is written in the report
    // from 2026-09-16 — not in this commit, which is about name translation.
    let casos: &[(&str, bool)] = &[
        ("   ", false), // empty after trim: `EmptyName`, was already caught
        ("___", false), // the defect that introduced `nome_tem_alfanumerico`
        ("_a", true),   // has substance
        ("a_", true),
        ("btn_ok2", true), // digit in the middle of snake is snake
        ("1abc", false),   // not an identifier, and the hint does not save it
        ("btnOK", false),  // name in classic style: rejected, with `btn_ok` suggested
        ("fn", true),      // reserved, but snake: codegen rejects before writing
        ("botão", false),  // non-ASCII
    ];
    for (nome, aceito) in casos {
        let mut d = doc();
        let r = d.apply_one(
            "rename",
            Command::Rename {
                node: "btn_ok".into(),
                name: (*nome).into(),
                handlers: true,
            },
        );
        assert_eq!(r.is_ok(), *aceito, "{nome:?} gave {r:?}");
    }
}

#[test]
fn evento_liga_e_desliga() {
    let mut d = doc();
    d.apply_one(
        "connect event",
        Command::SetEvent {
            node: "btn_ok".into(),
            event: "on_click".into(),
            handler: Some("btn_okClick".into()),
        },
    )
    .unwrap();
    assert_eq!(
        d.ir()
            .node("btn_ok")
            .unwrap()
            .events
            .get("on_click")
            .map(|s| s.as_str()),
        Some("btn_okClick")
    );
    d.undo().unwrap();
    assert!(d.ir().node("btn_ok").unwrap().events.is_empty());
}

#[test]
fn auditoria_registra_autor_e_revisao() {
    let mut d = doc();
    d.apply(Transaction::new("move", Author::User).with(mover("btn_ok", 1.0, 1.0)))
        .unwrap();
    d.apply(
        Transaction::new("text", Author::Agent("claude".into())).with(Command::SetProp {
            node: "btn_ok".into(),
            key: "text".into(),
            value: Some(Value::Str("x".into())),
        }),
    )
    .unwrap();
    d.undo().unwrap();

    let a = d.audit();
    assert_eq!(a.len(), 3);
    assert_eq!(a[0].author, Author::User);
    assert_eq!(a[1].author, Author::Agent("claude".into()));
    assert!(a[2].undo, "undo is also recorded");
    assert_eq!(a[1].author.to_string(), "agent:claude");
    assert_eq!(a[2].revision, 3);
}

#[test]
fn rotulo_do_undo_e_o_que_a_ide_mostra() {
    let mut d = doc();
    d.apply_one("move btn_ok", mover("btn_ok", 1.0, 1.0))
        .unwrap();
    assert_eq!(d.undo_label(), Some("move btn_ok"));
    d.undo().unwrap();
    assert_eq!(d.redo_label(), Some("move btn_ok"));
}

#[test]
fn selecao_ignora_no_inexistente() {
    let mut d = doc();
    d.select(vec!["btn_ok".to_string(), "fantasma".to_string()]);
    assert_eq!(d.selection(), ["btn_ok"]);
    d.toggle("edt_nome");
    assert_eq!(d.selection().len(), 2);
    d.toggle("btn_ok");
    assert_eq!(d.selection(), ["edt_nome"]);
    d.clear_selection();
    assert!(d.selection().is_empty());
}

#[test]
fn raiz_nao_pode_ser_removida() {
    let mut d = doc();
    assert!(
        d.apply_one(
            "remove root",
            Command::RemoveNode {
                node: "form_main".into()
            }
        )
        .is_err()
    );
}

#[test]
fn documento_segue_valido_depois_de_muita_edicao() {
    let mut d = doc();
    for i in 0..30 {
        let alvo = if i % 2 == 0 { "btn_ok" } else { "edt_nome" };
        d.apply_one("move", mover(alvo, i as f32, i as f32))
            .unwrap();
    }
    for _ in 0..15 {
        d.undo().unwrap();
    }
    for _ in 0..10 {
        d.redo().unwrap();
    }
    assert!(d.ir().validate().is_ok());
}

#[test]
fn renomear_a_acao_nao_desliga_quem_aponta_para_ela() {
    // Prior to ADR 0008 this test was named `..._religa_...`: renaming
    // changed identity, so the `Value::Ref` of each button had to be
    // rewritten, and forgetting that would disconnect buttons silently.
    //
    // Now there is nothing to reconnect — the reference points to `id`, and `id`
    // does not change. The entire mechanism is no longer needed, which is what
    // is proven here.
    let mut d = Document::new("form_main", "Form");
    d.nodes
        .insert("Action1".into(), crawrad_ir::Node::new("Action1", "Action"));
    let mut acl = crawrad_ir::Node::new("ActionList1", "ActionList");
    acl.children.push("Action1".into());
    d.add_child("form_main", acl).unwrap();
    let mut b = crawrad_ir::Node::new("Button1", "Button");
    b.props
        .insert("action".into(), crawrad_ir::Value::Ref("Action1".into()));
    d.add_child("form_main", b).unwrap();

    let mut doc = Doc::new(d);
    doc.apply(Transaction::user("rename").with(Command::Rename {
        node: "Action1".into(),
        name: "act_salvar".into(),
        handlers: true,
    }))
    .unwrap();

    assert_eq!(
        doc.ir().node("Button1").unwrap().props.get("action"),
        Some(&crawrad_ir::Value::Ref("Action1".into())),
        "the reference points to the id, and the id did not change"
    );
    assert_eq!(doc.ir().node("Action1").unwrap().name(), "act_salvar");
    assert!(doc.ir().validate().is_ok());

    doc.undo().unwrap();
    assert_eq!(doc.ir().node("Action1").unwrap().name(), "Action1");
}

#[test]
fn mexer_na_acao_marca_quem_a_referencia_como_tocado() {
    // This is what makes the button repaint with the new caption without the IDE knowing the rule.
    let mut d = Document::new("form_main", "Form");
    d.nodes
        .insert("Action1".into(), crawrad_ir::Node::new("Action1", "Action"));
    let mut acl = crawrad_ir::Node::new("ActionList1", "ActionList");
    acl.children.push("Action1".into());
    d.add_child("form_main", acl).unwrap();
    let mut b = crawrad_ir::Node::new("Button1", "Button");
    b.props
        .insert("action".into(), crawrad_ir::Value::Ref("Action1".into()));
    d.add_child("form_main", b).unwrap();

    let mut doc = Doc::new(d);
    let ap = doc
        .apply(Transaction::user("caption").with(Command::SetProp {
            node: "Action1".into(),
            key: "text".into(),
            value: Some(crawrad_ir::Value::Str("&Save".into())),
        }))
        .unwrap();

    assert!(
        ap.touched.contains("Button1"),
        "whatever points to the action changed along with it: {:?}",
        ap.touched
    );
}

#[test]
fn command_label_produces_expected_english_strings() {
    let set_prop = Command::SetProp {
        node: "btn_ok".into(),
        key: "text".into(),
        value: Some(crawrad_ir::Value::Str("OK".into())),
    };
    assert_eq!(set_prop.label(), "btn_ok.text");

    let set_rect = Command::SetRect {
        node: "btn_ok".into(),
        rect: Rect::new(0.0, 0.0, 10.0, 10.0),
    };
    assert_eq!(set_rect.label(), "move/resize btn_ok");

    let add_node = Command::AddNode {
        parent: "form_main".into(),
        node: Node::new("btn_ok", "Button"),
        index: 0,
    };
    assert_eq!(add_node.label(), "add Button");

    let remove_node = Command::RemoveNode {
        node: "btn_ok".into(),
    };
    assert_eq!(remove_node.label(), "remove btn_ok");

    let reorder = Command::Reorder {
        parent: "form_main".into(),
        from: 0,
        to: 1,
    };
    assert_eq!(reorder.label(), "reorder in form_main");

    let reparent = Command::Reparent {
        node: "btn_ok".into(),
        parent: "panel1".into(),
        index: 0,
        rect: Rect::new(0.0, 0.0, 10.0, 10.0),
    };
    assert_eq!(reparent.label(), "move btn_ok into panel1");

    let set_event = Command::SetEvent {
        node: "btn_ok".into(),
        event: "on_click".into(),
        handler: Some("btn_ok_click".into()),
    };
    assert_eq!(set_event.label(), "btn_ok.on_click");

    let set_binding = Command::SetBinding {
        node: "btn_ok".into(),
        prop: "enabled".into(),
        binding: None,
    };
    assert_eq!(set_binding.label(), "bind btn_ok.enabled");

    let rename = Command::Rename {
        node: "btn_ok".into(),
        name: "btn_submit".into(),
        handlers: true,
    };
    assert_eq!(rename.label(), "rename btn_ok to btn_submit");
}

fn inheriting_doc() -> Doc {
    let mut base = Document::new("form_a", "Form");
    let mut save = Node::new("btn_save", "Button");
    save.events
        .insert("on_click".into(), "btn_save_click".into());
    base.add_child("form_a", save).unwrap();
    let own = Document::new_inherited("form_b", "Form", "form_a");
    Doc::inheriting(&own, base).unwrap()
}

#[test]
fn an_inherited_component_cannot_be_removed_renamed_or_moved() {
    let mut d = inheriting_doc();
    let refused = [
        Command::RemoveNode {
            node: "btn_save".into(),
        },
        Command::Rename {
            node: "btn_save".into(),
            name: "btn_other".into(),
            handlers: true,
        },
        Command::Reorder {
            parent: "form_b".into(),
            from: 0,
            to: 0,
        },
    ];
    for c in refused {
        let err = d.apply_one("try", c).unwrap_err();
        assert!(
            matches!(err, DocError::InheritedNode(ref n) if n == "btn_save"),
            "{err}"
        );
    }
    assert_eq!(d.revision(), 0);
}

#[test]
fn an_inherited_event_overridden_and_reverted_saves_only_the_override() {
    let mut d = inheriting_doc();
    let set = |h: &str| Command::SetEvent {
        node: "btn_save".into(),
        event: "on_click".into(),
        handler: Some(h.into()),
    };
    d.apply_one("override", set("btn_save_click_override"))
        .unwrap();
    let own = d.own_ir().into_owned();
    assert_eq!(
        own.node("btn_save").unwrap().events["on_click"],
        "btn_save_click_override"
    );
    d.apply_one("revert", set("btn_save_click")).unwrap();
    assert!(d.own_ir().node("btn_save").is_none());
    let plain = Doc::new(Document::new("form_c", "Form"));
    assert!(!plain.is_inherited("form_c") && plain.base().is_none());
}
