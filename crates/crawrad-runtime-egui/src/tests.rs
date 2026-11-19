//! The entire RXP is testable **without a window**: `egui::Context` runs a frame in
//! memory. So gesture collection and painting go through CI like everything else
//! in this repository — the same discipline that the mock backend brought to the
//! RCL (ADR 0002).

use super::*;
use crawrad_ir::Node;

fn documento() -> Document {
    let mut d = Document::new("form_main", "Form");
    d.nodes
        .get_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));

    let mut btn = Node::new("btn_ok", "Button");
    btn.set_text("&OK");
    btn.set_rect(Rect::new(24.0, 96.0, 96.0, 32.0));
    d.add_child("form_main", btn).unwrap();

    let mut edt = Node::new("edt_nome", "Edit");
    edt.set_rect(Rect::new(24.0, 48.0, 280.0, 24.0));
    edt.props
        .insert("win32.PasswordChar".into(), Value::Str("*".into()));
    d.add_child("form_main", edt).unwrap();

    let mut acl = Node::new("ActionList1", "ActionList");
    acl.set_rect(Rect::new(400.0, 16.0, 28.0, 28.0));
    d.add_child("form_main", acl).unwrap();

    d
}

/// An already opened runtime, with the surface starting at the origin so the
/// coordinate calculation remains readable in the test.
fn runtime(doc: &Document) -> EguiRuntime {
    let mut rt = EguiRuntime::new(egui::Context::default());
    rt.set_viewport(
        ViewportPx {
            x: 0,
            y: 0,
            w: 800,
            h: 600,
        },
        Scale(1.0),
    );
    rt.open(doc).unwrap();
    rt
}

/// Runs an actual egui frame, without a window, delivering raw events.
fn quadro(rt: &mut EguiRuntime, eventos: Vec<egui::Event>) {
    let ctx = rt.ctx.clone();
    let mut entrada = egui::RawInput {
        screen_rect: Some(ERect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0))),
        ..Default::default()
    };
    entrada.events = eventos;
    // egui 0.36 explicitly begins and ends the pass — it is the same
    // frame that eframe runs, without a window underneath.
    ctx.begin_pass(entrada);
    rt.frame();
    // `FullOutput` carries the font atlas texture delta, and egui
    // panics if it is dropped unused — it is its check
    // warning that nothing was actually painted. There is no GPU here, so
    // we acknowledge and discard it intentionally.
    let mut saida = ctx.end_pass();
    saida.textures_delta.clear();
}

/// Where, on SCREEN, a design point lands — with the calculation written HERE.
///
/// It intentionally repeats what `Transformacao` does: a test that called
/// `rt.ponto_na_tela` would pass with the wrong transformation, because it would be
/// measuring the same function on both sides. Written by hand, it fails the day
/// the ruler offsets the origin without warning, or when zoom stops
/// multiplying.
///
/// `sup` is the surface corner; the design origin is one ruler further.
fn na_tela_de(sup: (f32, f32), zoom: f32, x: f32, y: f32) -> Pos2 {
    Pos2::new(sup.0 + REGUA + x * zoom, sup.1 + REGUA + y * zoom)
}

/// The same, for the surface that [`runtime`] in this file sets up: corner at
/// the screen origin and 100% zoom.
fn na_tela(x: f32, y: f32) -> Pos2 {
    na_tela_de((0.0, 0.0), 1.0, x, y)
}

fn clique(pos: Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

#[test]
fn abrir_le_os_filhos_do_formulario() {
    let d = documento();
    let rt = runtime(&d);
    assert_eq!(rt.ordem, vec!["btn_ok", "edt_nome", "ActionList1"]);
    assert_eq!(rt.ctls["btn_ok"].text, "&OK");
    assert_eq!(rt.ctls["btn_ok"].rect, Rect::new(24.0, 96.0, 96.0, 32.0));
    assert!(rt.ctls["btn_ok"].visual);
    assert!(
        !rt.ctls["ActionList1"].visual,
        "ActionList does not become a window"
    );
}

#[test]
fn a_arvore_do_treeview_vem_do_documento() {
    // The items were HARDCODED in the Win32 runtime code — `Node1`, `SubNode1`,
    // `Node2` written by hand. It was WYSIWYG to the eye and a lie to everything
    // else: this runtime would draw something else. Now both read the same
    // document, and that is what is proven here.
    let mut d = documento();
    let mut tv = Node::new("tv1", "TreeView");
    tv.set_rect(Rect::new(20.0, 200.0, 160.0, 120.0));
    tv.props.insert(
        "items".into(),
        crawrad_ir::Item::list_to(&[
            crawrad_ir::Item::with_children("Customers", vec![crawrad_ir::Item::new("Active")]),
            crawrad_ir::Item::new("Products"),
        ]),
    );
    d.add_child("form_main", tv).unwrap();
    d.validate().unwrap();

    let mut rt = runtime(&d);
    let itens = &rt.ctls["tv1"].itens;
    assert_eq!(itens.len(), 2, "two nodes at the first level");
    assert_eq!(itens[0].text, "Customers");
    assert_eq!(itens[0].children[0].text, "Active");
    assert_eq!(crawrad_ir::Item::total(itens), 3);

    // And drawing the entire tree must not blow up.
    quadro(&mut rt, vec![]);
}

/// Two SIBLING trees, side by side, neither inside the other.
///
/// The nested case already had a test (`a_aba_de_fora_fechada_esconde_a_pagina_de_
/// dentro_inteira`); the sibling case had none, and that is precisely what
/// `Ui::id()` does not distinguish.
fn documento_com_duas_arvores_irmas() -> Document {
    let mut d = documento();
    for (id, raiz) in [("tvA", "Alpha"), ("tvB", "Beta")] {
        let mut tv = Node::new(id, "TreeView");
        tv.set_name(id);
        let x = if id == "tvA" { 20.0 } else { 220.0 };
        tv.set_rect(Rect::new(x, 200.0, 160.0, 120.0));
        tv.props.insert(
            "items".into(),
            crawrad_ir::Item::list_to(&[crawrad_ir::Item::new(raiz)]),
        );
        d.add_child("form_main", tv).unwrap();
    }
    d.validate().unwrap();
    d
}

#[test]
fn duas_arvores_irmas_nao_dividem_o_alvo_do_clique_no_preview() {
    // **The bug that cost a REJECT on the tabs PR, in its third
    // copy.** `Ui::id()` does NOT change from one sibling to another: a child `Ui`
    // is created with `pai.id().with("child")`, a FIXED word (`egui-0.36.1`,
    // `src/ui.rs:251`), and `Ui::id()` on line 344 returns that `stable_id`,
    // and not the `unique_id` that exists beside it. Because `arvore_viva` built the
    // target with `ui.id().with(("item", &caminho))`, both trees requested the
    // SAME `Id` for row 0 — and clicking on one responded for both.
    //
    // Proven by mutation: removing `nome` from both `interact` lines in
    // `arvore_viva`, this test fails on the second assertion — tree A
    // selects itself with the click that landed on B.
    let d = documento_com_duas_arvores_irmas();
    let mut rt = runtime(&d);
    rt.set_mode(Mode::Live);
    aquecer_o_desenho(&mut rt);

    // The first row of tree B. The `+2` and the half row come from
    // `arvore_viva`: row 0 starts at `r.min.y + 2` and is 18 pt high.
    let _ = clicar_em(&mut rt, Pos2::new(220.0 + 40.0, 200.0 + 2.0 + 9.0));

    assert_eq!(
        rt.selecao_viva.get("tvB"),
        Some(&vec![0]),
        "click landed on tree B and it should have been selected"
    );
    assert_eq!(
        rt.selecao_viva.get("tvA").cloned().unwrap_or_default(),
        Vec::<usize>::new(),
        "tree A selected itself from a click on B: both are \
         sharing the target, and `Ui::id()` is identical for every sibling"
    );
}

#[test]
fn clicking_the_tree_marker_collapses_without_selecting() {
    // the target for the whole row was registered AFTER the marker
    // target and covered it; egui delivers the click to the last one, so clicking
    // the `−` only selected the row and the tree never collapsed.
    //
    // Proven by mutation: restoring the row target to the whole `linha`,
    // this test fails on the first assertion.
    let mut d = documento();
    let mut tv = Node::new("tvA", "TreeView");
    tv.set_name("tvA");
    tv.set_rect(Rect::new(20.0, 200.0, 160.0, 120.0));
    let mut root = crawrad_ir::Item::new("Alpha");
    root.children.push(crawrad_ir::Item::new("Child"));
    tv.props
        .insert("items".into(), crawrad_ir::Item::list_to(&[root]));
    d.add_child("form_main", tv).unwrap();
    d.validate().unwrap();
    let mut rt = runtime(&d);
    rt.set_mode(Mode::Live);
    aquecer_o_desenho(&mut rt);

    // The level 0 marker is centered at `r.min.x + 3 + 5` and at half
    // row (`r.min.y + 2 + 9`), the calculations from `arvore_viva`.
    let _ = clicar_em(&mut rt, Pos2::new(20.0 + 8.0, 200.0 + 2.0 + 9.0));

    assert!(
        rt.fechados_vivos
            .get("tvA")
            .is_some_and(|closed| closed.contains(&vec![0])),
        "clicking the marker should have collapsed the item"
    );
    assert_eq!(
        rt.selecao_viva.get("tvA").cloned().unwrap_or_default(),
        Vec::<usize>::new(),
        "clicking the marker must not select the row"
    );

    // And the text still selects.
    let _ = clicar_em(&mut rt, Pos2::new(20.0 + 40.0, 200.0 + 2.0 + 9.0));
    assert_eq!(rt.selecao_viva.get("tvA"), Some(&vec![0]));
}

#[test]
fn propriedade_de_outro_alvo_nao_atravessa() {
    // The ADR 0002 rule seen from the other side: `win32.PasswordChar` exists in the
    // document and RXP must not see it. If it did, the contract would be
    // decorative.
    let d = documento();
    let rt = runtime(&d);
    assert_eq!(
        rt.ctls["edt_nome"].senha, None,
        "RXP must not read Win32 namespaced property"
    );
}

#[test]
fn a_legenda_do_botao_vem_da_acao() {
    // The TAction link (ADR 0006) could not stay only in the RCL: the same document
    // must say the same thing in both runtimes.
    let mut d = documento();
    let mut act = Node::new("EditCopy1", "Action");
    act.set_text("&Copy");
    act.props.insert("enabled".into(), Value::Bool(false));
    d.nodes.insert("EditCopy1".into(), act);
    d.node_mut("ActionList1")
        .unwrap()
        .children
        .push("EditCopy1".into());
    d.node_mut("btn_ok")
        .unwrap()
        .props
        .insert("action".into(), Value::Ref("EditCopy1".into()));
    d.validate().unwrap();

    let rt = runtime(&d);
    assert_eq!(
        rt.ctls["btn_ok"].text, "&Copy",
        "caption comes from the action"
    );
    assert!(
        !rt.ctls["btn_ok"].enabled,
        "disabled state also comes from the action"
    );
}

#[test]
fn atualizar_tira_da_tela_quem_saiu_do_documento() {
    let mut d = documento();
    let mut rt = runtime(&d);

    d.node_mut("form_main")
        .unwrap()
        .children
        .retain(|c| c != "edt_nome");
    d.nodes.remove("edt_nome");
    let tocados: BTreeSet<NodeId> = ["form_main".to_string()].into_iter().collect();
    rt.update(&d, &tocados).unwrap();

    assert!(!rt.ctls.contains_key("edt_nome"));
    assert_eq!(rt.ordem, vec!["btn_ok", "ActionList1"]);
}

#[test]
fn atualizar_antes_de_abrir_e_recusado() {
    let d = documento();
    let mut rt = EguiRuntime::new(egui::Context::default());
    let e = rt.update(&d, &BTreeSet::new()).unwrap_err();
    assert!(matches!(e, RuntimeError::NotStarted));
}

#[test]
fn no_modo_vivo_o_designer_nao_recebe_gesto() {
    // Running the form is using it: the click belongs to the application user, not the
    // designer. If a PointerDown leaked from here, dragging inside the live form
    // would move the component.
    let d = documento();
    let mut rt = runtime(&d);
    rt.set_mode(Mode::Live);

    // In LIVE mode there is no ruler, so the same screen point is a different design
    // point — but nothing bubbles up to the designer, which is what is measured here.
    let p = na_tela(40.0, 110.0);
    quadro(&mut rt, vec![egui::Event::PointerMoved(p), clique(p, true)]);
    assert!(
        rt.drain_events().is_empty(),
        "in live mode no event bubbles to the designer"
    );

    // And returning to design restores the gesture.
    rt.set_mode(Mode::Design);
    quadro(&mut rt, vec![egui::Event::PointerMoved(p), clique(p, true)]);
    assert_eq!(
        rt.drain_events(),
        vec![RuntimeEvent::PointerDown {
            x: 40.0,
            y: 110.0,
            ctrl: false
        }]
    );
}

#[test]
fn o_modo_vivo_semeia_os_campos_e_nao_toca_no_documento() {
    let mut d = documento();
    d.node_mut("edt_nome").unwrap().set_text("John Doe");
    let mut rt = runtime(&d);

    rt.set_mode(Mode::Live);
    assert_eq!(
        rt.texto_vivo.get("edt_nome").map(String::as_str),
        Some("John Doe"),
        "field is initialized with what the document states"
    );

    // The user types…
    rt.texto_vivo
        .insert("edt_nome".into(), "something else".into());
    quadro(&mut rt, vec![]);

    // …and exiting discards it. Running a form does not modify the .dfm.
    rt.set_mode(Mode::Design);
    assert!(rt.texto_vivo.is_empty());
    assert_eq!(
        d.node("edt_nome").unwrap().text(),
        Some("John Doe"),
        "document was not touched"
    );
}

#[test]
fn a_ordem_do_tab_vem_do_documento() {
    // `TabOrder` was a decorative property: nobody read it. In live mode it
    // determines the widget creation order, and egui makes Tab navigate
    // through it.
    let mut d = documento();
    d.node_mut("btn_ok")
        .unwrap()
        .props
        .insert("tab_order".into(), Value::Int(0));
    d.node_mut("edt_nome")
        .unwrap()
        .props
        .insert("tab_order".into(), Value::Int(1));
    let rt = runtime(&d);
    assert_eq!(rt.ctls["btn_ok"].tab_order, 0);
    assert_eq!(rt.ctls["edt_nome"].tab_order, 1);
}

#[test]
fn o_gesto_chega_em_unidade_de_design() {
    // The surface starts at (0,0), the form as well, and the ruler offsets the design
    // origin by 18 points. What is proven here is that the conversion exists and
    // that the event leaves the runtime already converted — the designer never sees pixels.
    let d = documento();
    let mut rt = runtime(&d);

    let p = na_tela(40.0, 110.0);
    quadro(&mut rt, vec![egui::Event::PointerMoved(p), clique(p, true)]);
    let evs = rt.drain_events();
    assert_eq!(
        evs,
        vec![RuntimeEvent::PointerDown {
            x: 40.0,
            y: 110.0,
            ctrl: false
        }]
    );
    assert!(
        rt.busy(),
        "with the button pressed a gesture is in progress"
    );

    let s = na_tela(60.0, 130.0);
    quadro(
        &mut rt,
        vec![egui::Event::PointerMoved(s), clique(s, false)],
    );
    let evs = rt.drain_events();
    assert_eq!(
        evs,
        vec![RuntimeEvent::PointerUp { x: 60.0, y: 130.0 }],
        "release ends the gesture"
    );
    assert!(!rt.busy());
}

#[test]
fn o_duplo_clique_abre_o_editor_como_no_win32() {
    // Win32 reports double-click on DOWN (WM_LBUTTONDBLCLK); egui only
    // recognizes it on UP, when the pair completes. If the runtime did not absorb this
    // difference, double-click simply would not exist in RXP — and that is what
    // happened: the ActionList and ImageList editors would not open.
    let d = documento();
    let mut rt = runtime(&d);
    let p = na_tela(40.0, 110.0);

    quadro(&mut rt, vec![egui::Event::PointerMoved(p), clique(p, true)]);
    quadro(&mut rt, vec![clique(p, false)]);
    quadro(&mut rt, vec![clique(p, true)]);
    quadro(&mut rt, vec![clique(p, false)]);

    let evs = rt.drain_events();
    assert!(
        evs.contains(&RuntimeEvent::PointerDblClick { x: 40.0, y: 110.0 }),
        "second click must become PointerDblClick; got {evs:?}"
    );
}

#[test]
fn o_viewport_desloca_a_conta() {
    // With the surface starting at (200,80), the same screen click becomes another
    // design coordinate. This is the offset that the side panel imposes.
    let d = documento();
    let mut rt = runtime(&d);
    rt.set_viewport(
        ViewportPx {
            x: 200,
            y: 80,
            w: 600,
            h: 520,
        },
        Scale(1.0),
    );
    let p = na_tela_de((200.0, 80.0), 1.0, 40.0, 30.0);
    quadro(&mut rt, vec![egui::Event::PointerMoved(p), clique(p, true)]);
    assert_eq!(
        rt.drain_events(),
        vec![RuntimeEvent::PointerDown {
            x: 40.0,
            y: 30.0,
            ctrl: false
        }]
    );
}

#[test]
fn clique_fora_da_superficie_nao_e_do_designer() {
    // A click on the toolbar must not become a drag gesture.
    let d = documento();
    let mut rt = runtime(&d);
    rt.set_viewport(
        ViewportPx {
            x: 200,
            y: 80,
            w: 600,
            h: 520,
        },
        Scale(1.0),
    );
    quadro(
        &mut rt,
        vec![
            egui::Event::PointerMoved(Pos2::new(10.0, 10.0)),
            clique(Pos2::new(10.0, 10.0), true),
        ],
    );
    assert!(rt.drain_events().is_empty());
    assert!(!rt.busy());
}

#[test]
fn the_classic_accelerator_is_read_as_on_win32() {
    assert_eq!(sem_acelerador("&OK"), "OK");
    assert_eq!(indice_do_acelerador("&OK"), Some(0));

    assert_eq!(sem_acelerador("Cu&t"), "Cut");
    assert_eq!(indice_do_acelerador("Cu&t"), Some(2));

    // `&&` is an actual `&`, and does not mark any accelerator.
    assert_eq!(sem_acelerador("Save && Exit"), "Save & Exit");
    assert_eq!(indice_do_acelerador("Save && Exit"), None);

    assert_eq!(sem_acelerador("Cancel"), "Cancel");
    assert_eq!(indice_do_acelerador("Cancel"), None);
}

#[test]
fn pintar_nao_entra_em_panico_com_o_documento_de_verdade() {
    // Painting does not have an easy assertion, but it has a worthwhile guarantee: running a
    // complete frame with selection, guides, marquee, and badges must not blow up.
    let d = documento();
    let mut rt = runtime(&d);
    rt.set_selection(&["btn_ok".to_string(), "ActionList1".to_string()]);
    rt.set_marquee(Some(Rect::new(10.0, 10.0, 120.0, 80.0)));
    rt.set_badges(&[("btn_ok".to_string(), "1".to_string())]);
    rt.set_grid(8.0);
    rt.set_guides(&[
        GuideLine {
            vertical: true,
            pos: 24.0,
            from: 0.0,
            to: 200.0,
            kind: GuideKind::Component,
            thickness: 0.0,
        },
        GuideLine {
            vertical: false,
            pos: 112.0,
            from: 0.0,
            to: 300.0,
            kind: GuideKind::Text,
            thickness: 0.0,
        },
        GuideLine {
            vertical: true,
            pos: 130.0,
            from: 90.0,
            to: 130.0,
            kind: GuideKind::Spacing,
            thickness: 8.0,
        },
    ]);
    quadro(&mut rt, vec![]);

    rt.close();
    assert!(rt.ctls.is_empty());
    quadro(&mut rt, vec![]);
}

#[test]
fn o_combo_le_a_colecao_e_a_escolha_do_documento() {
    let mut d = documento();
    let mut c = Node::new("cbo1", "ComboBox");
    c.set_name("cbo_uf");
    c.set_rect(Rect::new(8.0, 200.0, 145.0, 24.0));
    c.props.insert(
        "items".into(),
        crawrad_ir::Item::list_to(&[crawrad_ir::Item::new("SP"), crawrad_ir::Item::new("RJ")]),
    );
    c.props.insert("item_index".into(), Value::Int(1));
    d.add_child("form_main", c).unwrap();
    let rt = runtime(&d);

    let ctl = &rt.ctls["cbo1"];
    assert_eq!(ctl.itens.len(), 2);
    assert_eq!(ctl.itens[1].text, "RJ");
    assert_eq!(ctl.escolhido, Some(1));
    assert!(ctl.visual);
}

#[test]
fn item_index_menos_um_no_combo_e_nenhum() {
    // In a classic RAD IDE, `-1` means *none selected*, not an index.
    let mut d = documento();
    let mut c = Node::new("cbo1", "ComboBox");
    c.set_name("cbo_uf");
    c.set_rect(Rect::new(8.0, 200.0, 145.0, 24.0));
    c.props.insert("item_index".into(), Value::Int(-1));
    d.add_child("form_main", c).unwrap();
    let rt = runtime(&d);
    assert_eq!(rt.ctls["cbo1"].escolhido, None);
}

/// Runs a **live** frame and returns the painted rectangles.
///
/// It is the only honest size measurement: what the test sees is what egui
/// requested to draw, not what we requested.
fn retangulos_pintados(rt: &mut EguiRuntime) -> Vec<ERect> {
    let ctx = rt.ctx.clone();
    let entrada = egui::RawInput {
        screen_rect: Some(ERect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0))),
        ..Default::default()
    };
    ctx.begin_pass(entrada);
    rt.frame();
    let mut saida = ctx.end_pass();
    saida.textures_delta.clear();
    saida
        .shapes
        .iter()
        .filter_map(|c| match &c.shape {
            egui::Shape::Rect(r) => Some(r.rect),
            _ => None,
        })
        .collect()
}

#[test]
fn o_combo_vivo_tem_a_altura_que_o_designer_deu() {
    // Found live, comparing with the Edit next to it: the combo height in
    // egui does NOT come from the rectangle — `button_frame` does
    // `outer_rect.set_height(at_least(interact_size.y))`, and the default is
    // lower than 24. The designer showed both as equal, while the application did not.
    let mut d = documento();
    let mut c = Node::new("cbo1", "ComboBox");
    c.set_name("cbo_uf");
    c.set_rect(Rect::new(300.0, 40.0, 145.0, 24.0));
    c.props.insert(
        "items".into(),
        crawrad_ir::Item::list_to(&[crawrad_ir::Item::new("SP")]),
    );
    c.props.insert("item_index".into(), Value::Int(0));
    d.add_child("form_main", c).unwrap();

    let mut rt = runtime(&d);
    rt.set_mode(Mode::Live);
    // The first egui frame does not have measured text size yet.
    let _ = retangulos_pintados(&mut rt);
    let pintados = retangulos_pintados(&mut rt);

    let esperado = ERect::from_min_size(Pos2::new(300.0, 40.0), Vec2::new(145.0, 24.0));
    let achou = pintados.iter().any(|r| {
        (r.width() - esperado.width()).abs() < 1.0
            && (r.height() - esperado.height()).abs() < 1.0
            && (r.min.x - esperado.min.x).abs() < 2.0
            && (r.min.y - esperado.min.y).abs() < 2.0
    });
    assert!(
        achou,
        "no painted rectangle matches the designer's ({esperado:?});\npainted: {pintados:#?}"
    );
}

#[test]
fn a_caixa_e_o_radio_leem_o_checked_do_documento() {
    let mut d = documento();
    let mut c = Node::new("chk1", "CheckBox");
    c.set_name("chk_ativo");
    c.set_rect(Rect::new(8.0, 200.0, 120.0, 20.0));
    c.props.insert("checked".into(), Value::Bool(true));
    d.add_child("form_main", c).unwrap();

    let mut r = Node::new("rb1", "RadioButton");
    r.set_name("rb_fisica");
    r.set_rect(Rect::new(8.0, 230.0, 120.0, 20.0));
    d.add_child("form_main", r).unwrap();

    let rt = runtime(&d);
    assert!(rt.ctls["chk1"].marcado, "Checked from document must arrive");
    assert!(!rt.ctls["rb1"].marcado, "and unchecked must stay unchecked");
}

#[test]
fn o_modo_vivo_semeia_o_estado_de_todos_os_tipos() {
    // Entering the live form must start from what the document says — if the
    // state started empty, running a form would seem to lose the design.
    let mut d = documento();
    let mut c = Node::new("chk1", "CheckBox");
    c.set_name("chk_ativo");
    c.set_rect(Rect::new(8.0, 200.0, 120.0, 20.0));
    c.props.insert("checked".into(), Value::Bool(true));
    d.add_child("form_main", c).unwrap();

    let mut m = Node::new("mmo1", "Memo");
    m.set_name("mmo_obs");
    m.set_rect(Rect::new(8.0, 230.0, 200.0, 80.0));
    m.set_text("line 1");
    d.add_child("form_main", m).unwrap();

    let mut l = Node::new("lst1", "ListBox");
    l.set_name("lst_uf");
    l.set_rect(Rect::new(8.0, 320.0, 145.0, 80.0));
    l.props.insert(
        "items".into(),
        crawrad_ir::Item::list_to(&[crawrad_ir::Item::new("SP"), crawrad_ir::Item::new("RJ")]),
    );
    l.props.insert("item_index".into(), Value::Int(1));
    d.add_child("form_main", l).unwrap();

    let mut rt = runtime(&d);
    rt.set_mode(Mode::Live);
    assert_eq!(rt.marcados_vivos.get("chk1"), Some(&true));
    assert_eq!(
        rt.texto_vivo.get("mmo1").map(String::as_str),
        Some("line 1")
    );
    assert_eq!(rt.selecao_viva.get("lst1"), Some(&vec![1]));
}

#[test]
fn a_alca_pede_a_seta_do_eixo_dela() {
    // The cursor over handles existed ONLY in the RCL — it was `SetCursor` inside the
    // Win32 subclass — and disappeared when it was removed (ADR 0013). Now it belongs
    // to the adapter drawing the handle, and the table must match `cantos`:
    // clockwise starting from the top-left corner.
    use crawrad_runtime::Axis;
    assert_eq!(
        EIXO_DAS_ALCAS,
        [
            Axis::Nwse, // ↖ top-left
            Axis::Ns,   // ↑ top
            Axis::Nesw, // ↗ top-right
            Axis::We,   // → right
            Axis::Nwse, // ↘ bottom-right
            Axis::Ns,   // ↓ bottom
            Axis::Nesw, // ↙ bottom-left
            Axis::We,   // ← left
        ]
    );
    // Opposite corners share the axis: that is what makes the arrow look right.
    // In DESIGN units: `cantos` adds `ALCA_FORA` before conversion
    // to screen, not after — see its doc.
    let r = Rect::new(10.0, 20.0, 100.0, 40.0);
    let c = cantos(r);
    // Handles stay OUTSIDE the rectangle, so they do not cover component
    // content — see `crawrad_runtime::HANDLE_OUTSET`. What this test protects here
    // is the ORDER, which is a contract with `Handle::ALL` and the table above.
    let f = crawrad_runtime::HANDLE_OUTSET;
    assert_eq!(c[0], (r.x - f, r.y - f), "0 is the top-left corner");
    assert_eq!(
        c[4],
        (r.x + r.w + f, r.y + r.h + f),
        "4 is the bottom-right corner"
    );
    assert_eq!(EIXO_DAS_ALCAS[0], EIXO_DAS_ALCAS[4]);
    assert_eq!(EIXO_DAS_ALCAS[2], EIXO_DAS_ALCAS[6]);
}

/// It is the counterpart of [`retangulos_pintados`]: when the question is *what
/// appeared*, measuring the egui output is the only honest answer.
fn textos_pintados(rt: &mut EguiRuntime) -> Vec<String> {
    textos_pintados_com(rt, Vec::new())
}

/// The same thing, with **gesture**: when the question is *what the click changed on
/// screen*, measuring the egui output is the only honest answer — querying the runtime
/// field would only prove that the field changed, not that the drawing followed.
fn textos_pintados_com(rt: &mut EguiRuntime, eventos: Vec<egui::Event>) -> Vec<String> {
    let ctx = rt.ctx.clone();
    let entrada = egui::RawInput {
        screen_rect: Some(ERect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0))),
        events: eventos,
        ..Default::default()
    };
    ctx.begin_pass(entrada);
    rt.frame();
    let mut saida = ctx.end_pass();
    saida.textures_delta.clear();
    saida
        .shapes
        .iter()
        .filter_map(|c| match &c.shape {
            egui::Shape::Text(t) => Some(t.galley.text().to_string()),
            _ => None,
        })
        .collect()
}

/// The registry checks the DRAWING, not the list.
///
/// The first version of this test only checked whether the class name was
/// in [`CLASSES`] — and the reviewer rejected it: deleting BOTH arms of
/// `Memo` in the `match`, the entire suite remained green. The class fell into
/// the generic arm and became a labeled box, silently.
///
/// Now the proof is what was **painted**: the generic arm draws the class
/// name centered in the box, and that is the signature the test looks for.
/// If it appears, the class does not have its own drawing.
#[test]
fn toda_classe_do_registro_tem_desenho_de_verdade() {
    let reg = crawrad_registry::Registry::builtin();
    for classe in CLASSES {
        let d = reg.get(classe).expect("adapter class in registry");
        let _ = d;

        // A caption that is NOT the class name: if it were, the legitimate
        // label of `Label` would be confused with the generic arm's.
        let mut doc = Document::new("form", "Form");
        {
            let raiz = doc.node_mut("form").unwrap();
            raiz.set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
            raiz.set_name("form");
        }
        let mut n = Node::new("alvo", *classe);
        n.set_name("alvo");
        n.set_text("zzz");
        n.set_rect(Rect::new(20.0, 20.0, 160.0, 60.0));
        doc.add_child("form", n).unwrap();
        doc.validate().unwrap();

        let mut rt = runtime(&doc);
        let textos = textos_pintados(&mut rt);
        assert!(
            !textos.iter().any(|t| t == classe),
            "`{classe}` fell into the generic drawing arm: adapter painted \
             CLASS NAME in a box instead of drawing the component.\n\
             painted: {textos:?}"
        );
    }
}

/// The same in LIVE mode — that is where the leak first appeared.
#[test]
fn toda_classe_tem_widget_de_verdade_no_modo_vivo() {
    for classe in CLASSES {
        let mut doc = Document::new("form", "Form");
        {
            let raiz = doc.node_mut("form").unwrap();
            raiz.set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
            raiz.set_name("form");
        }
        let mut n = Node::new("alvo", *classe);
        n.set_name("alvo");
        n.set_text("zzz");
        n.set_rect(Rect::new(20.0, 20.0, 160.0, 60.0));
        doc.add_child("form", n).unwrap();
        doc.validate().unwrap();

        let mut rt = runtime(&doc);
        rt.set_mode(Mode::Live);
        // egui measures text on the first frame; the second one already paints.
        let _ = textos_pintados(&mut rt);
        let textos = textos_pintados(&mut rt);
        assert!(
            !textos.iter().any(|t| t == classe),
            "`{classe}` fell into the live mode generic arm: became a label \
             with the class name.\npainted: {textos:?}"
        );
    }
}

#[test]
fn componente_solto_dentro_de_um_painel_aparece_depois_do_update() {
    // Found live: the three buttons existed in the document and the structure
    // tree, and did not appear on screen. `open` traversed down the tree when
    // containers were introduced; `update` kept reading only root children and
    // rebuilt `self.ordem` on every transaction, discarding grandchildren.
    //
    // It is the worst kind of bug: the document is correct, the tree shows the
    // component, and the screen lies.
    let mut d = documento();
    let mut p = Node::new("pnl", "Panel");
    p.set_name("pnl_dados");
    p.set_rect(Rect::new(40.0, 200.0, 300.0, 120.0));
    d.add_child("form_main", p).unwrap();
    let mut rt = runtime(&d);
    assert!(rt.ctls.contains_key("pnl"), "panel is added on open");

    // The user drops a button INSIDE the panel — a transaction, as the shell does.
    let mut b = Node::new("btn_dentro", "Button");
    b.set_name("btn_dentro");
    b.set_rect(Rect::new(10.0, 10.0, 96.0, 32.0));
    d.add_child("pnl", b).unwrap();
    let mut mexidos = std::collections::BTreeSet::new();
    mexidos.insert("pnl".to_string());
    mexidos.insert("btn_dentro".to_string());
    rt.update(&d, &mexidos).unwrap();

    assert!(
        rt.ctls.contains_key("btn_dentro"),
        "panel child must exist after update"
    );
    assert!(
        rt.ordem.iter().any(|id| id == "btn_dentro"),
        "and must be in drawing order: without it, it is not painted"
    );
    // and in the right position: 40+10, 200+10
    assert_eq!(
        rt.ctls["btn_dentro"].rect,
        Rect::new(50.0, 210.0, 96.0, 32.0)
    );
}

#[test]
fn mover_o_painel_move_quem_esta_dentro() {
    // The child rectangle is relative to the parent: moving the panel changes the
    // SCREEN position of everything inside it, even without touching the child.
    let mut d = documento();
    let mut p = Node::new("pnl", "Panel");
    p.set_name("pnl_dados");
    p.set_rect(Rect::new(40.0, 200.0, 300.0, 120.0));
    d.add_child("form_main", p).unwrap();
    let mut b = Node::new("btn_dentro", "Button");
    b.set_name("btn_dentro");
    b.set_rect(Rect::new(10.0, 10.0, 96.0, 32.0));
    d.add_child("pnl", b).unwrap();
    let mut rt = runtime(&d);

    // only the PARENT is moved
    d.node_mut("pnl")
        .unwrap()
        .set_rect(Rect::new(140.0, 200.0, 300.0, 120.0));
    let mut mexidos = std::collections::BTreeSet::new();
    mexidos.insert("pnl".to_string());
    rt.update(&d, &mexidos).unwrap();

    assert_eq!(
        rt.ctls["btn_dentro"].rect,
        Rect::new(150.0, 210.0, 96.0, 32.0),
        "child must follow parent without being touched"
    );
}

/// A minimal `Ctl`, containing only what radio exclusion looks at.
fn ctl_de(class: &str, pai: &str) -> Ctl {
    Ctl {
        base: WindowControlBase {
            visual_base: crawrad_runtime::VisualControlBase {
                base: crawrad_runtime::ComponentBase::new("", "", class),
                rect: Rect::new(0.0, 0.0, 100.0, 20.0),
                visible: true,
                enabled: true,
                text: String::new(),
                visual: true,
                derived_geometry: false,
            },
            parent: pai.into(),
            children: Vec::new(),
            clip: None,
            tab_order: 0,
        },
        pagina_ativa: 0,
        senha: None,
        somente_leitura: false,
        itens: Vec::new(),
        escolhido: None,
        marcado: false,
        intervalo: 0,
        columns: 1,
        spin: 0,
        selected: crawrad_ir::Color::STANDARD[0].1,
        url: String::new(),
        borda: 1.0,
        scroll_bars: "ssNone".to_string(),
        posicao_scroll: 0.0,
        picture: None,
        col_count: 5,
        row_count: 5,
        fixed_cols: 0,
        fixed_rows: 1,
        default_col_width: 80.0,
        default_row_height: 24.0,
        grid_lines: true,
        shape_kind: "Rectangle".to_string(),
        bevel_shape: "Box".to_string(),
        bevel_style: "Lowered".to_string(),
        collapsed: false,
        beveled: true,
        columns_info: Vec::new(),
        show_column_headers: true,
        row_select: true,
        corner_radius: 8.0,
        elevation: 1,
        show_header: true,
        subtitle: String::new(),
        kind: String::new(),
        outline: false,
        associate: String::new(),
        count: 0,
        max_count: 99,
        removable: false,
        separator: "›".to_string(),
        style: "bsText".to_string(),
        segment_color: Color32::from_rgb(0x45, 0x46, 0x4b),
        current_color: Color32::from_rgb(0xe0, 0x61, 0x2b),
        page_count: 10,
        page_index: 0,
        max_buttons: 7,
        show_arrows: true,
        side: "dsLeft".to_string(),
        opened: false,
        overlay: true,
        selected_items: Vec::new(),
        checked_items: Vec::new(),
        text_hint: "Select…".to_string(),
        edit_mask: String::new(),
        date_time: String::new(),
        min_date: String::new(),
        max_date: String::new(),
    }
}

#[test]
fn o_radio_exclui_so_os_irmaos_do_mesmo_pai() {
    // `TurnSiblingsOff` from a classic RAD IDE. Two `GroupBox`/`Panel` side by side are
    // two independent groups because they are two PARENTS — not because of `WS_GROUP`, which
    // the classic library never used for this.
    //
    // This test exists because the code was previously wrong: the comment said
    // "no containers yet, parent is the form" and remained valid after
    // containers arrived (ADR 0012). Checking a radio button inside a
    // panel unchecked those outside.
    let ctls: Vec<(NodeId, Ctl)> = vec![
        ("r_form_1".to_string(), ctl_de("RadioButton", "form")),
        ("r_form_2".to_string(), ctl_de("RadioButton", "form")),
        ("r_pnl_1".to_string(), ctl_de("RadioButton", "pnl")),
        ("r_pnl_2".to_string(), ctl_de("RadioButton", "pnl")),
        ("r_pnl_3".to_string(), ctl_de("RadioButton", "pnl")),
        // A `CheckBox` under the same parent is not included: it is not mutually exclusive.
        ("chk".to_string(), ctl_de("CheckBox", "pnl")),
    ];
    let refs = || ctls.iter().map(|(i, c)| (i, c));

    let mut apagados = irmaos_a_desmarcar(refs(), &"r_pnl_1".to_string(), &"pnl".to_string());
    apagados.sort();
    assert_eq!(
        apagados,
        vec!["r_pnl_2".to_string(), "r_pnl_3".to_string()],
        "radio button in a panel must not reach the form or the CheckBox"
    );

    let mut apagados = irmaos_a_desmarcar(refs(), &"r_form_1".to_string(), &"form".to_string());
    apagados.sort();
    assert_eq!(
        apagados,
        vec!["r_form_2".to_string()],
        "form radio button must not reach inside the panel"
    );

    // And the clicked one never unchecks itself.
    assert!(
        !irmaos_a_desmarcar(refs(), &"r_pnl_1".to_string(), &"pnl".to_string())
            .contains(&"r_pnl_1".to_string())
    );
}

#[test]
fn o_groupbox_nao_pinta_fundo_e_mostra_a_legenda() {
    // The measurable difference between `Panel` and `GroupBox`: the panel paints a
    // background rectangle, the group box DOES NOT — the parent shows through it, like
    // `TGroupBox` in a classic RAD IDE. Painting a background would cause a group box inside
    // a panel to punch a hole of the wrong color.
    let mut d = documento();
    let mut gb = Node::new("gbx1", "GroupBox");
    gb.set_name("gbx_pagamento");
    gb.set_text("Payment");
    gb.set_rect(Rect::new(40.0, 150.0, 200.0, 100.0));
    d.add_child("form_main", gb).unwrap();

    let mut rt = runtime(&d);
    rt.set_mode(Mode::Live);
    let _ = retangulos_pintados(&mut rt);
    let pintados = retangulos_pintados(&mut rt);

    let caixa = ERect::from_min_size(Pos2::new(40.0, 150.0), Vec2::new(200.0, 100.0));
    let pintou_fundo = pintados.iter().any(|r| {
        (r.width() - caixa.width()).abs() < 2.0
            && (r.height() - caixa.height()).abs() < 2.0
            && (r.min.x - caixa.min.x).abs() < 2.0
            && (r.min.y - caixa.min.y).abs() < 2.0
    });
    assert!(
        !pintou_fundo,
        "the `GroupBox` painted a rectangle of its size: it has no background"
    );

    let textos = textos_pintados(&mut rt);
    assert!(
        textos.iter().any(|t| t == "Payment"),
        "the `GroupBox` caption did not appear: {textos:?}"
    );
}

#[test]
fn dois_groupbox_sao_dois_grupos_de_radio() {
    // The question that remained open when `RadioButton` was introduced: without a
    // group property, how do we know which radio button belongs to which set?
    // The answer in a classic RAD IDE is the PARENT, which is why `GroupBox` does not need
    // any new property — it is already the group by virtue of being a container.
    let ctls: Vec<(NodeId, Ctl)> = vec![
        ("a1".to_string(), ctl_de("RadioButton", "gbx_a")),
        ("a2".to_string(), ctl_de("RadioButton", "gbx_a")),
        ("b1".to_string(), ctl_de("RadioButton", "gbx_b")),
        ("b2".to_string(), ctl_de("RadioButton", "gbx_b")),
    ];
    let apagados = irmaos_a_desmarcar(
        ctls.iter().map(|(i, c)| (i, c)),
        &"a1".to_string(),
        &"gbx_a".to_string(),
    );
    assert_eq!(
        apagados,
        vec!["a2".to_string()],
        "a group box cannot reach radio buttons in neighboring group box"
    );
}

/// A document with a two-page `PageControl` and a button on the second.
fn documento_com_paginas() -> Document {
    let mut d = documento();
    let mut pc = Node::new("pgc1", "PageControl");
    pc.set_name("pgc_cadastro");
    pc.set_rect(Rect::new(20.0, 150.0, 320.0, 200.0));
    pc.props.insert("active_page".into(), Value::Int(0));
    d.add_child("form_main", pc).unwrap();

    for (id, nome, legenda) in [
        ("tab1", "tab_dados", "Data"),
        ("tab2", "tab_notas", "Notes"),
    ] {
        let mut t = Node::new(id, "TabSheet");
        t.set_name(nome);
        t.set_text(legenda);
        t.set_rect(Rect::new(0.0, 24.0, 320.0, 176.0));
        d.add_child("pgc1", t).unwrap();
    }

    let mut b = Node::new("btn_p2", "Button");
    b.set_name("btn_na_pagina_dois");
    b.set_text("Second");
    b.set_rect(Rect::new(10.0, 10.0, 96.0, 32.0));
    d.add_child("tab2", b).unwrap();
    d
}

#[test]
fn so_a_pagina_aberta_aparece() {
    // The conditional visibility of the ADR 0012 amendment. The button on page 2
    // exists in the document and remains in the `Structure` tree — it is just not on
    // screen while page 1 is open.
    let mut d = documento_com_paginas();
    let mut rt = runtime(&d);
    rt.set_mode(Mode::Live);
    let _ = textos_pintados(&mut rt);
    let com_pagina_1 = textos_pintados(&mut rt);
    assert!(
        !com_pagina_1.iter().any(|t| t == "Second"),
        "closed page button was drawn: {com_pagina_1:?}"
    );
    // Both tabs always appear: it is the tab strip, not the content.
    assert!(com_pagina_1.iter().any(|t| t == "Data"));
    assert!(com_pagina_1.iter().any(|t| t == "Notes"));

    d.node_mut("pgc1")
        .unwrap()
        .props
        .insert("active_page".into(), Value::Int(1));
    let mut rt = runtime(&d);
    rt.set_mode(Mode::Live);
    let _ = textos_pintados(&mut rt);
    let com_pagina_2 = textos_pintados(&mut rt);
    assert!(
        com_pagina_2.iter().any(|t| t == "Second"),
        "opening the page did not bring its contents: {com_pagina_2:?}"
    );
}

/// Where the center of tab `i` of a `PageControl` lands, on the LIVE mode screen.
///
/// The calculation is written HERE intentionally, like `na_tela_de`: calling the
/// product calculation — today `page_control` in `desenho_comum.rs`, previously
/// `retangulos_das_abas` — would measure the same function on both sides and pass
/// when it is wrong. In live mode there is no ruler or zoom, so the design origin is
/// that of the surface — which in this file is `(0, 0)`.
fn centro_da_aba(pgc: Rect, i: usize) -> Pos2 {
    Pos2::new(pgc.x + i as f32 * TAB_W + TAB_W / 2.0, pgc.y + TAB_H / 2.0)
}

/// A full click on a point, and what the frame painted.
///
/// Down and up, because egui recognizes the click on **up** — Win32
/// reported on down, and that difference has already cost debugging here. `PointerMoved`
/// comes first because without it the pointer was never over the target.
///
/// What this frame paints is still the state BEFORE the click: the list of what
/// to draw is assembled at the start of the frame. To see the effect, call the
/// next [`textos_pintados`].
fn clicar_em(rt: &mut EguiRuntime, p: Pos2) -> Vec<String> {
    textos_pintados_com(
        rt,
        vec![
            egui::Event::PointerMoved(p),
            clique(p, true),
            clique(p, false),
        ],
    )
}

/// Two warmup frames: in the first, egui has not measured text yet, and
/// the click is resolved against targets registered by the PREVIOUS frame.
fn aquecer_o_desenho(rt: &mut EguiRuntime) {
    let _ = textos_pintados(rt);
    let _ = textos_pintados(rt);
}

#[test]
fn o_clique_na_aba_troca_a_pagina_no_preview() {
    // The defect: the Preview tab strip only PAINTED. `paginas_de` read the
    // document page, written once in `open`, and `viver` had no way
    // to change it — clicking the tab did nothing. Meanwhile, the GENERATED
    // application switched pages for real (`page_control`, in `crawrad-codegen`,
    // returns `changed` and mutates state). The Preview showed less than what the
    // application delivered, which is the opposite of ADR 0013.
    let d = documento_com_paginas();
    let mut rt = runtime(&d);
    rt.set_mode(Mode::Live);
    aquecer_o_desenho(&mut rt);
    let antes = textos_pintados(&mut rt);
    assert!(
        !antes.iter().any(|t| t == "Second"),
        "Preview should start on the page specified by document: {antes:?}"
    );

    let _ = clicar_em(
        &mut rt,
        centro_da_aba(Rect::new(20.0, 150.0, 320.0, 200.0), 1),
    );
    let depois = textos_pintados(&mut rt);
    assert!(
        depois.iter().any(|t| t == "Second"),
        "clicking tab 2 should bring its content: {depois:?}"
    );

    // And the control check, which is the difficult half: running the form DOES NOT change
    // the document. The new page lives in the live state; `.crawform` continues
    // stating what the designer selected.
    assert_eq!(
        d.node("pgc1").unwrap().props.get("active_page"),
        Some(&Value::Int(0)),
        "click in Preview wrote to document"
    );
    assert_eq!(
        rt.ctls["pgc1"].pagina_ativa, 0,
        "click in Preview modified document reading"
    );
    assert_eq!(
        rt.paginas_vivas.get("pgc1"),
        Some(&1),
        "opened page must live in live state"
    );
}

#[test]
fn sair_do_preview_esquece_a_aba_que_o_usuario_abriu() {
    // The same rule as typed text: what happened in Preview is discarded upon
    // returning to design. If the tab survived, running the form would change
    // what the designer sees — without ever modifying the document, which is the worst of
    // both worlds.
    let d = documento_com_paginas();
    let mut rt = runtime(&d);
    rt.set_mode(Mode::Live);
    aquecer_o_desenho(&mut rt);
    let _ = clicar_em(
        &mut rt,
        centro_da_aba(Rect::new(20.0, 150.0, 320.0, 200.0), 1),
    );
    assert_eq!(rt.paginas_vivas.get("pgc1"), Some(&1));
    // What the Preview SHOWS, not just what the map stores. Without this measurement the
    // test only asserted that the design did not change — and design never changes: with the
    // `paginas_de` bypass neutralized it passed, measuring the wrong side.
    // For there to be something to forget, the tab must have opened first.
    let vivo = textos_pintados(&mut rt);
    assert!(
        vivo.iter().any(|t| t == "Second"),
        "Preview did not open the page the user clicked: {vivo:?}"
    );

    rt.set_mode(Mode::Design);
    assert!(
        rt.paginas_vivas.is_empty(),
        "tab opened in Preview must die with it"
    );
    let _ = textos_pintados(&mut rt);
    let desenho = textos_pintados(&mut rt);
    assert!(
        !desenho.iter().any(|t| t == "Second"),
        "design returned showing the page the USER opened: {desenho:?}"
    );
}

#[test]
fn o_preview_abre_na_pagina_que_o_designer_deixou_aberta() {
    // Seeding in `set_modo`, and it needs a document that does NOT start
    // on the first page — with `active_page = 0`, seeding or not seeding produce the
    // same drawing, because `paginas_de` falls back to the document when the live map
    // has no answer. Measured: without this test, deleting the `PageControl`
    // `insert` in `set_modo` broke nothing.
    let mut d = documento_com_paginas();
    d.node_mut("pgc1")
        .unwrap()
        .props
        .insert("active_page".into(), Value::Int(1));
    let mut rt = runtime(&d);
    rt.set_mode(Mode::Live);
    assert_eq!(
        rt.paginas_vivas.get("pgc1"),
        Some(&1),
        "live state should start on the designer's page"
    );

    // And the half that the document alone would also explain holds up because of
    // the next one: with Preview open, the document no longer dictates.
    aquecer_o_desenho(&mut rt);
    let abertura = textos_pintados(&mut rt);
    assert!(
        abertura.iter().any(|t| t == "Second"),
        "Preview opened on first page instead of designer's: {abertura:?}"
    );

    // The designer modifies `ActivePageIndex` while Preview is running. The live
    // form **ignores** it, for the same reason it ignores the `Text` of an `Edit` in which
    // the user already typed: what governs live state is whoever is using the
    // form. The document takes control again on the next entry into Preview.
    d.node_mut("pgc1")
        .unwrap()
        .props
        .insert("active_page".into(), Value::Int(0));
    let tocados: BTreeSet<NodeId> = ["pgc1".to_string()].into_iter().collect();
    rt.update(&d, &tocados).unwrap();
    let depois = textos_pintados(&mut rt);
    assert!(
        depois.iter().any(|t| t == "Second"),
        "document dragged Preview to another page: {depois:?}"
    );
}

/// Three `PageControl` controls nested inside one another, with a button at the bottom.
///
/// `pgcA(A1, A2)` ‣ `pgcB(B1, B2)` inside `A1` ‣ `pgcC(C1, C2)` inside
/// `B1` ‣ the "Deep" button inside `C2`. It is the composite case that the
/// 07/09 amendment to ADR 0012 requires a gate to exercise: checking only the closest
/// page and stopping there passes with two levels and lies with three.
///
/// Measurements are chosen so the three tab strips fall into non-overlapping `y`
/// ranges — 150..174, 189..213, and 213..237 —, otherwise clicking a
/// tab would hit another level's tab and the test would measure something else.
fn documento_com_paginas_aninhadas() -> Document {
    let mut d = documento();

    let mut pgc_a = Node::new("pgcA", "PageControl");
    pgc_a.set_name("pgc_a");
    pgc_a.set_rect(Rect::new(20.0, 150.0, 320.0, 200.0));
    pgc_a.props.insert("active_page".into(), Value::Int(0));
    d.add_child("form_main", pgc_a).unwrap();
    for (id, legenda) in [("tabA1", "A1"), ("tabA2", "A2")] {
        let mut t = Node::new(id, "TabSheet");
        t.set_name(id);
        t.set_text(legenda);
        t.set_rect(Rect::new(0.0, 24.0, 320.0, 176.0));
        d.add_child("pgcA", t).unwrap();
    }

    // Inside page A1: rectangle relative to parent, as the document stores it.
    let mut pgc_b = Node::new("pgcB", "PageControl");
    pgc_b.set_name("pgc_b");
    pgc_b.set_rect(Rect::new(5.0, 15.0, 300.0, 150.0));
    pgc_b.props.insert("active_page".into(), Value::Int(0));
    d.add_child("tabA1", pgc_b).unwrap();
    for (id, legenda) in [("tabB1", "B1"), ("tabB2", "B2")] {
        let mut t = Node::new(id, "TabSheet");
        t.set_name(id);
        t.set_text(legenda);
        t.set_rect(Rect::new(0.0, 24.0, 300.0, 126.0));
        d.add_child("pgcB", t).unwrap();
    }

    let mut pgc_c = Node::new("pgcC", "PageControl");
    pgc_c.set_name("pgc_c");
    pgc_c.set_rect(Rect::new(10.0, 0.0, 270.0, 110.0));
    pgc_c.props.insert("active_page".into(), Value::Int(0));
    d.add_child("tabB1", pgc_c).unwrap();
    for (id, legenda) in [("tabC1", "C1"), ("tabC2", "C2")] {
        let mut t = Node::new(id, "TabSheet");
        t.set_name(id);
        t.set_text(legenda);
        t.set_rect(Rect::new(0.0, 24.0, 270.0, 86.0));
        d.add_child("pgcC", t).unwrap();
    }

    let mut b = Node::new("btn_fundo", "Button");
    b.set_name("btn_no_fundo");
    b.set_text("Deep");
    b.set_rect(Rect::new(10.0, 10.0, 96.0, 32.0));
    d.add_child("tabC2", b).unwrap();
    d
}

#[test]
fn a_aba_de_fora_fechada_esconde_a_pagina_de_dentro_inteira() {
    // The GATE that the 07/09 amendment to ADR 0012 requires on top of every copy
    // of the page rule: it must exercise the **composite case**, not the
    // class. `no_formulario_vivo` is the sixth copy of the rule, and the first
    // inside RXP; the bug it must not repeat is the one from the previous PR in the
    // generator, where traversal stopped at the closest page.
    //
    // Proven by mutation: replacing `return false` for `TabSheet` with a
    // `break` — stopping at the closest page, which is exactly the bug
    // fixed yesterday —, the frame paints ["OK", "", "A1", "A2", "C1", "C2",
    // "Deep"]: the inner strip and the bottom button on top of a CLOSED
    // outer page. The bottom two lines are the ones that fail.
    let d = documento_com_paginas_aninhadas();
    let mut rt = runtime(&d);
    rt.set_mode(Mode::Live);
    aquecer_o_desenho(&mut rt);

    // With everything open at each one's first level, all three strips appear.
    let inicio = textos_pintados(&mut rt);
    for aba in ["A1", "A2", "B1", "B2", "C1", "C2"] {
        assert!(
            inicio.iter().any(|t| t == aba),
            "strip {aba} should be on screen: {inicio:?}"
        );
    }
    assert!(
        !inicio.iter().any(|t| t == "Deep"),
        "button lives in C2, and open page is C1: {inicio:?}"
    );

    // Open C2, three levels deep: the button at the bottom appears.
    let _ = clicar_em(
        &mut rt,
        centro_da_aba(Rect::new(35.0, 213.0, 270.0, 110.0), 1),
    );
    let fundo = textos_pintados(&mut rt);
    assert!(
        fundo.iter().any(|t| t == "Deep"),
        "opening bottom page did not bring its contents: {fundo:?}"
    );

    // And now close the OUTER one. Nothing inside survives — neither the bottom
    // button nor the tab strips of intermediate `PageControl`s —, no matter how
    // open each underlying page is.
    let _ = clicar_em(
        &mut rt,
        centro_da_aba(Rect::new(20.0, 150.0, 320.0, 200.0), 1),
    );
    let fora = textos_pintados(&mut rt);
    assert!(
        !fora.iter().any(|t| t == "Deep"),
        "bottom button remained on screen with outer tab closed: {fora:?}"
    );
    for escondido in ["B1", "B2", "C1", "C2"] {
        assert!(
            !fora.iter().any(|t| t == escondido),
            "strip {escondido} remained on screen with outer tab closed: {fora:?}"
        );
    }
    // The outer strip remains: it, and not the content, always appears.
    assert!(fora.iter().any(|t| t == "A1"));
    assert!(fora.iter().any(|t| t == "A2"));
}

#[test]
fn o_formulario_nao_entra_no_mapa_de_controles() {
    // This absence terminates the traversal in `no_formulario_vivo`: it stops
    // when `ctls.get` returns `None`, and the only node that returns `None` is the
    // root. As long as this holds true traversal terminates; the day the
    // form enters the map — with its `parent` pointing to itself,
    // which is what `ctl_do_no` writes for parentless nodes — it loops
    // forever, and Preview freezes instead of failing visibly.
    //
    // `ordem_toda` is `em_ordem_de_desenho`, which pushes only CHILDREN, and
    // `ctls` is populated from it. Confusing `ctls`, `ordem`, and
    // `ordem_toda` has caused bugs in this file more than once.
    let d = documento_com_paginas_aninhadas();
    let rt = runtime(&d);
    assert!(
        !rt.ctls.contains_key(&d.root),
        "form entered `ctls`, and traversal in `no_formulario_vivo` became infinite"
    );
    assert!(
        !rt.ordem_toda.contains(&d.root),
        "root entered drawing order, and `ctls` comes from it"
    );
    // And the `parent` of the form's direct child INDEED points to the root: that
    // step exits the map and ends the traversal.
    assert_eq!(rt.ctls["pgcA"].parent, d.root);
}

#[test]
fn as_medidas_da_aba_batem_com_o_registro() {
    // The adapter does NOT know the catalog — `crawrad-registry` is a dev-dependency
    // here intentionally (ADR 0002/0013), so tab dimensions are written
    // on both sides.
    //
    // There were THREE copies: the catalog, this crate, and a string in `rxp.rs`.
    // Today there are two, and `TAB_H`/`TAB_W` here come from `desenho_comum.rs`, which
    // is the same text that the generator copies into the user's project —
    // meaning changing them moves designer, Preview, and application together.
    //
    // Unchecked duplication is a guaranteed divergence waiting to happen, and
    // it would be harsh: drawing would place the tab in one spot and clicking would look for it
    // in another. This test guards the remaining duplication, and can only exist
    // here because tests can see the catalog while production code cannot.
    assert_eq!(
        TAB_H,
        crawrad_registry::TAB_STRIP_HEIGHT,
        "tab strip height diverged between adapter and registry"
    );
    assert_eq!(
        TAB_W,
        crawrad_registry::TAB_WIDTH,
        "tab width diverged between adapter and registry"
    );
    assert_eq!(
        LARGURA_MAIS,
        crawrad_registry::PLUS_BUTTON_WIDTH,
        "\"+\" width diverged between adapter and registry"
    );
    assert_eq!(
        TAB_SCROLL_W,
        crawrad_registry::TAB_SCROLL_BUTTON_WIDTH,
        "tab scroll button width diverged between adapter and registry"
    );
}

#[test]
fn egui_tab_window_mirrors_registry_tab_window() {
    assert_eq!(TAB_SCROLL_W, crawrad_registry::TAB_SCROLL_BUTTON_WIDTH);
    for &w in &[60.0, 150.0, 320.0, 500.0] {
        for s in 0..=8 {
            for a in 0..=8 {
                let (first, count, arrows) = tab_window(w, s, a);
                let reg = crawrad_registry::tab_window(w, s, a, false);
                assert_eq!(first, reg.first, "first mismatch for w={w}, s={s}, a={a}");
                assert_eq!(count, reg.count, "count mismatch for w={w}, s={s}, a={a}");
                assert_eq!(
                    arrows, reg.arrows,
                    "arrows mismatch for w={w}, s={s}, a={a}"
                );
            }
        }
    }
}

// ------------------------------------------------------------------- Timer
//
// `Timer` is the first non-visual component that **runs**: it does not appear in the
// application and yet must fire. In a classic RAD IDE, Windows wakes it up
// with `WM_TIMER`. In egui, nothing wakes it up — the frame itself
// must request the next repaint.

/// A form with a `Timer`.
fn com_timer(intervalo: i64, ligado: bool) -> Document {
    let mut d = documento();
    let mut t = Node::new("tmr1", "Timer");
    t.set_rect(Rect::new(500.0, 16.0, 28.0, 28.0));
    t.props.insert("interval".into(), Value::Int(intervalo));
    t.props.insert("enabled".into(), Value::Bool(ligado));
    t.events.insert("on_timer".into(), "tmr1Timer".into());
    d.add_child("form_main", t).unwrap();
    d.validate().unwrap();
    d
}

/// A frame with manually set time.
///
/// `RawInput::time` is the clock returned by `InputState::time` — by setting it
/// here, the timer test waits zero seconds and does not depend on machine speed.
/// Returns the delay that the frame REQUESTED for the next repaint.
///
/// Be careful when reading this number in a freshly opened context: egui requests
/// immediate repaint on its own during early frames (font atlas, style
/// animation), and in those frames the timer request remains hidden under zero.
/// Measured: the first four frames return `0ns` with or without a timer. That is why
/// wait tests call [`aquecer`] before measuring.
fn quadro_em(rt: &mut EguiRuntime, tempo: f64) -> std::time::Duration {
    let ctx = rt.ctx.clone();
    let entrada = egui::RawInput {
        screen_rect: Some(ERect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0))),
        time: Some(tempo),
        ..Default::default()
    };
    ctx.begin_pass(entrada);
    rt.frame();
    let mut saida = ctx.end_pass();
    saida.textures_delta.clear();
    saida
        .viewport_output
        .get(&egui::ViewportId::ROOT)
        .map(|v| v.repaint_delay)
        .unwrap_or(std::time::Duration::MAX)
}

/// The first repaint request that means something, at the given instant.
///
/// egui requests immediate repaint (`0ns`) on its own while settling
/// — font atlas, style animation, a second pass for layout —,
/// and how many frames that takes is not fixed: measured, it varied between four and
/// five depending on the interval between them. While it requests zero, the
/// timer request is hidden underneath.
///
/// Four frames with time actually advancing, until egui stops requesting
/// immediate repaints on its own.
///
/// What holds it is not frame count, but **time**: egui style
/// animations last `animation_time` (1/12 s) and request repaint while
/// running. Measured: with a 2 ms step, sixteen consecutive frames return `0ns`;
/// with a 100 ms step, the third already returns the timer request.
fn aquecer(rt: &mut EguiRuntime, inicio: f64) {
    for i in 0..4 {
        quadro_em(rt, inicio + i as f64 * 0.1);
    }
}

/// Does the request match the remaining time, within one predicted frame?
///
/// egui subtracts one frame duration from what is requested, so repaint falls
/// **on** time instead of after it. That is why the comparison is a range, and the
/// range is one-sided: requesting more than what remains is the defect.
fn perto(pedido: std::time::Duration, falta_ms: u64) -> bool {
    let falta = std::time::Duration::from_millis(falta_ms);
    pedido <= falta && pedido + std::time::Duration::from_millis(20) >= falta
}

#[test]
fn o_primeiro_quadro_do_timer_so_comeca_a_contar() {
    // Firing in the frame where the timer appears would be a free tick, and
    // Windows `SetTimer` does not grant that either: it schedules the FIRST tick
    // `Interval` from now.
    let mut prazo = None;
    assert!(
        !timer_due(10.0, 500, &mut prazo),
        "does not fire on creation"
    );
    assert_eq!(prazo, Some(10.5), "and starts counting from now");
}

#[test]
fn o_timer_dispara_quando_o_prazo_vence() {
    let mut prazo = Some(10.5);
    assert!(
        !timer_due(10.4, 500, &mut prazo),
        "before deadline, nothing"
    );
    assert_eq!(prazo, Some(10.5), "and deadline does not move");
    assert!(timer_due(10.5, 500, &mut prazo), "at exact deadline, fires");
}

#[test]
fn o_tique_atrasado_e_descartado_e_nao_enfileirado() {
    // **The accumulation rule.** The application went 3 seconds without drawing and the
    // interval is 500 ms: the handler runs ONCE, not six times.
    //
    // This is the classic RAD IDE behavior, originating from Windows: `WM_TIMER` does not accumulate
    // in the queue — there is one message per timer, and stopping message pumping LOSES ticks
    // instead of queueing them. The alternative would be calling the handler six times
    // within the same frame, with zero elapsed time between calls.
    let mut prazo = Some(1.0);
    let mut disparos = 0;
    // A single frame, three seconds later.
    if timer_due(4.0, 500, &mut prazo) {
        disparos += 1;
    }
    assert_eq!(disparos, 1, "six missed ticks become one, not six");
    // And the new deadline counts from NOW. Were it `prazo + intervalo`, it would fall at
    // 1.5 s — already expired —, and the five delayed ticks would return in subsequent
    // frames: accumulation sneaking in through the back door.
    assert_eq!(prazo, Some(4.5), "deadline is recounted from now");
    assert!(
        !timer_due(4.1, 500, &mut prazo),
        "and following frame pays no delayed tick"
    );
}

#[test]
fn a_espera_pedida_e_o_que_falta_e_nunca_negativa() {
    assert_eq!(timer_wait(10.0, Some(10.5)).as_millis(), 500);
    // An already expired deadline requests an immediate frame, not a negative duration —
    // `from_secs_f64` of a negative number panics.
    assert_eq!(timer_wait(11.0, Some(10.5)), std::time::Duration::ZERO);
    assert_eq!(timer_wait(11.0, None), std::time::Duration::ZERO);
}

#[test]
fn o_timer_anda_no_preview_e_conta_os_tiques() {
    // A timer that does not fire in Preview is a timer that can only be checked
    // by generating the project. There is no user handler here to produce an effect
    // on screen — the count is proof that it ticked.
    let d = com_timer(500, true);
    let mut rt = runtime(&d);
    rt.set_mode(Mode::Live);

    quadro_em(&mut rt, 100.0);
    assert_eq!(rt.tiques(&"tmr1".to_string()), 0, "the first only counts");
    quadro_em(&mut rt, 100.4);
    assert_eq!(
        rt.tiques(&"tmr1".to_string()),
        0,
        "before deadline, nothing"
    );
    quadro_em(&mut rt, 100.5);
    assert_eq!(rt.tiques(&"tmr1".to_string()), 1);
    quadro_em(&mut rt, 101.0);
    assert_eq!(rt.tiques(&"tmr1".to_string()), 2);

    // And exiting Preview discards what ran, like all other live
    // state: the document does not change because it was executed.
    rt.set_mode(Mode::Design);
    assert_eq!(rt.tiques(&"tmr1".to_string()), 0);
}

#[test]
fn o_preview_pede_o_proximo_quadro_por_causa_do_timer() {
    // **The point that does not exist in a classic RAD IDE.** In immediate mode a frame only
    // occurs if requested; without this request the application sits idle and the
    // timer never fires. We measure the actual request in egui output.
    let d = com_timer(500, true);
    let mut rt = runtime(&d);
    rt.set_mode(Mode::Live);

    // The deadline is created in the first frame: 100.0 + 0.5 = 100.5. Warmup
    // stops at 100.3, with nothing expired.
    aquecer(&mut rt, 100.0);

    let pedido = quadro_em(&mut rt, 100.35);
    assert!(
        perto(pedido, 150),
        "with 150 ms remaining to deadline, frame requested {pedido:?}"
    );
    // And the request SHRINKS as the deadline approaches: a frame that always requested
    // the full interval would push the trigger forward forever.
    let mais_perto = quadro_em(&mut rt, 100.45);
    assert!(
        perto(mais_perto, 50) && mais_perto < pedido,
        "with 50 ms remaining request should shrink: {pedido:?} → {mais_perto:?}"
    );
}

#[test]
fn timer_parado_nao_dispara_nem_pede_quadro() {
    // The two ways to stop a `TTimer`, and both come from a classic RAD IDE:
    // `Enabled = False`, and `Interval = 0` — which is not a short interval.
    for (intervalo, ligado, porque) in [(500, false, "Enabled = False"), (0, true, "Interval = 0")]
    {
        let d = com_timer(intervalo, ligado);
        let mut rt = runtime(&d);
        rt.set_mode(Mode::Live);
        // Warmup, and then one hundred seconds all at once — plenty of time for
        // any interval.
        aquecer(&mut rt, 100.0);
        let pedido = quadro_em(&mut rt, 200.0);
        assert_eq!(rt.tiques(&"tmr1".to_string()), 0, "{porque}");
        assert!(
            pedido >= std::time::Duration::from_secs(1),
            "{porque}: stopped timer must not keep app awake, and requested {pedido:?}"
        );
    }
}

#[test]
fn o_timer_e_icone_no_designer_e_nao_uma_caixa_rotulada() {
    // The icon is 28 points on each side and does not stretch — if `Timer` had
    // been left off the non-visual list, it would become the "box with class name"
    // that RXP paints for what it cannot draw, sized to the document rectangle.
    let d = com_timer(500, true);
    let mut rt = runtime(&d);
    let _ = retangulos_pintados(&mut rt);
    let pintados = retangulos_pintados(&mut rt);
    let icone = ERect::from_min_size(na_tela(500.0, 16.0), Vec2::splat(ICONE));
    assert!(
        pintados
            .iter()
            .any(|r| (r.min - icone.min).length() < 0.5 && (r.size() - icone.size()).length() < 0.5),
        "Timer icon was not painted at {icone:?};\npainted: {pintados:#?}"
    );
    // And it DOES NOT become a widget in Preview.
    assert!(!rt.ctls["tmr1"].visual);
}

#[test]
fn todo_nao_visual_do_registro_e_icone_e_nao_widget() {
    // The adapter does not know the catalog, so the non-visual list is
    // written by hand here. This test prevents the list from falling
    // behind — and it ALREADY matches the answer `ler_no` uses, so forgetting
    // a new component surfaces on both sides at once.
    let reg = crawrad_registry::Registry::builtin();
    for d in reg.all() {
        if !d.supports("egui") {
            continue;
        }
        assert_eq!(
            e_visual_no_canvas(d.class),
            d.visual,
            "`{}` is non-visual in catalog and RXP treats it as widget (or vice versa)",
            d.class
        );
    }
}

#[test]
fn a_espessura_da_moldura_do_painel_vem_do_documento() {
    // `border_width` was declared in the catalog and no one read it: RXP
    // drew a fixed `Stroke::new(1.0, …)`. Worse, the key was neither in the
    // common core nor in a namespace, so EVERY panel from the palette
    // failed `validate()` — and `Doc::preview`, which is the RTA path,
    // rejected any document with a panel.
    //
    // What was PAINTED is measured, not what was requested.
    for largura in [1, 4] {
        let mut d = documento();
        let mut pnl = Node::new("pnl1", "Panel");
        pnl.set_rect(Rect::new(200.0, 200.0, 160.0, 80.0));
        pnl.props.insert("border_width".into(), Value::Int(largura));
        d.add_child("form_main", pnl).unwrap();
        d.validate().unwrap();

        let mut rt = runtime(&d);
        let ctx = rt.ctx.clone();
        let entrada = egui::RawInput {
            screen_rect: Some(ERect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0))),
            ..Default::default()
        };
        ctx.begin_pass(entrada);
        rt.frame();
        let mut saida = ctx.end_pass();
        saida.textures_delta.clear();

        let alvo = ERect::from_min_size(na_tela(200.0, 200.0), Vec2::new(160.0, 80.0));
        let achou = saida.shapes.iter().any(|c| match &c.shape {
            egui::Shape::Rect(r) => {
                (r.rect.min - alvo.min).length() < 0.5
                    && (r.rect.size() - alvo.size()).length() < 0.5
                    && (r.stroke.width - largura as f32).abs() < 0.01
            }
            _ => false,
        });
        assert!(achou, "no {largura} pt frame in panel rectangle");
    }
}

#[test]
fn a_alca_do_filho_e_recortada_pelo_pai() {
    // Dragging a child outside its container clips the COMPONENT — this is
    // Win32 `WS_CLIPCHILDREN`, which the adapter handles manually here. Selection did
    // not follow the same rule: all eight handles were painted across the full rectangle,
    // and stuck out on the outside, over nothing. Eight blue squares where
    // there is no component to grab.
    //
    // What this test enforces is a SINGLE clip rect. It measures the painter, not the
    // pixel: `pintor_do_filho` is the function both paths call, and if
    // someone gives one clip rect to drawing and another to selection, the intersection ceases
    // to match.
    let mut d = documento();
    let mut pnl = Node::new("pnl1", "Panel");
    pnl.set_name("pnl_caixa");
    pnl.set_rect(Rect::new(40.0, 200.0, 120.0, 80.0));
    d.add_child("form_main", pnl).unwrap();

    // The button starts inside and CROSSES the right border of the panel: 100 + 96
    // exceeds the width of 120.
    let mut b = Node::new("btn_fora", "Button");
    b.set_name("btn_fora");
    b.set_rect(Rect::new(100.0, 10.0, 96.0, 32.0));
    d.add_child("pnl1", b).unwrap();

    let rt = runtime(&d);
    let c = rt.ctls.get("btn_fora").expect("the child");
    let clip = c.clip.expect("container child has clip");
    assert_eq!(
        clip,
        Rect::new(40.0, 200.0, 120.0, 80.0),
        "clip is PARENT's rectangle, in form coordinates"
    );
    // And the component indeed crosses the border: without this the test proves nothing.
    let dele = c.rect;
    assert!(
        dele.x + dele.w > clip.x + clip.w,
        "child had to cross parent border for clip to matter"
    );
}

#[test]
fn o_chao_nao_se_mexe_debaixo_do_gesto() {
    // The defect, measured: screen→form conversion subtracts the surface
    // origin, and that origin changes when a panel opens, closes, or changes
    // width. Changing MID-drag, the gesture `delta` absorbs the
    // entire displacement — the component jumps the distance the surface
    // moved. It really happened: switching the Components panel (190px) to the
    // Object Inspector (320px) sent the dragged component to `x = -116`,
    // outside the form, underneath the inspector.
    //
    // The rule is `SetCapture` carried to completion: a captured gesture owns the
    // interaction **and the frame of reference** until release.
    let mut rt = runtime(&documento());
    rt.set_viewport(
        crawrad_runtime::ViewportPx {
            x: 200,
            y: 0,
            w: 800,
            h: 600,
        },
        crawrad_runtime::Scale(1.0),
    );
    // The DESIGN origin is the corner of the drawing area, which begins after the
    // ruler: with the surface at x=200 and an 18 pt ruler, the form zero
    // lands at (218, 18) on screen.
    let antes = rt.em_design(Pos2::new(300.0, 100.0));
    assert_eq!(
        antes,
        (82.0, 82.0),
        "without gesture, origin is current origin"
    );

    // Gesture begins: the frame of reference freezes.
    rt.capturando = true;
    rt.gesto = Some(rt.transformacao());

    // The surface moves 130px to the right — the panel changed width.
    rt.set_viewport(
        crawrad_runtime::ViewportPx {
            x: 330,
            y: 0,
            w: 670,
            h: 600,
        },
        crawrad_runtime::Scale(1.0),
    );
    assert_eq!(
        rt.em_design(Pos2::new(300.0, 100.0)),
        antes,
        "same SCREEN point must yield same FORM point \
         while gesture lasts — otherwise component jumps surface displacement"
    );

    // On release, the frame of reference reverts to the current one.
    rt.capturando = false;
    rt.gesto = None;
    assert_eq!(
        rt.em_design(na_tela_de((330.0, 0.0), 1.0, 0.0, 100.0)),
        (0.0, 100.0),
        "gesture ended, conversion reverts to actual surface"
    );
}

// ---------------------------------------------------------------------------
// Zoom and ruler — ADR 0019
// ---------------------------------------------------------------------------

/// A form created as the user creates it: through the palette, via `instantiate`.
///
/// Study rule 9. `Node::new` does not load catalog properties, and
/// a manually assembled conversion test would measure the model instead of the path
/// the IDE takes — that was how `Panel.border_width` lived for months beneath
/// a green suite.
fn como_a_paleta_faz() -> Document {
    let reg = crawrad_registry::Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    d.nodes
        .get_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
    for (classe, id, r) in [
        ("Button", "Button1", Rect::new(24.0, 96.0, 96.0, 32.0)),
        ("Panel", "Panel1", Rect::new(200.0, 200.0, 160.0, 80.0)),
        (
            "ActionList",
            "ActionList1",
            Rect::new(500.0, 16.0, 28.0, 28.0),
        ),
    ] {
        let n = reg.instantiate(classe, id, r).expect("class in catalog");
        d.add_child("form_main", n).unwrap();
    }
    d.validate().unwrap();
    d
}

#[test]
fn a_conversao_vai_e_volta_com_zoom_diferente_de_um() {
    // The test requested by ADR 0019, and format matters: it does NOT compare number
    // to number — it converts from design to screen and back, asserting that the
    // point returned to its position.
    //
    // This format catches a conversion that missed the scale factor, which is how this
    // actually breaks: at 1.0 zoom, forward without multiplication and reverse without
    // division produce the same result, and the bug goes undetected.
    let d = como_a_paleta_faz();
    let mut rt = runtime(&d);

    for zoom in crawrad_runtime::ZOOMS {
        rt.set_zoom(zoom);
        for (x, y) in [(0.0, 0.0), (24.0, 96.0), (200.0, 200.0), (639.0, 419.0)] {
            let (vx, vy) = rt.em_design(rt.ponto_na_tela(x, y));
            assert!(
                (vx - x).abs() < 0.01 && (vy - y).abs() < 0.01,
                "zoom {zoom}: ({x},{y}) returned as ({vx},{vy})"
            );
        }
        // And the full rectangle, which is what the component uses: the corner
        // returns to position AND size was multiplied. Checking only origin
        // would allow a `na_tela` that converts position but forgets
        // size — leaving the component in the right place with the wrong body.
        let r = Rect::new(24.0, 96.0, 96.0, 32.0);
        let na_tela = rt.na_tela(r);
        let (vx, vy) = rt.em_design(na_tela.min);
        assert!(
            (vx - r.x).abs() < 0.01 && (vy - r.y).abs() < 0.01,
            "zoom {zoom}: rectangle corner did not return"
        );
        assert!(
            (na_tela.width() - r.w * zoom).abs() < 0.01
                && (na_tela.height() - r.h * zoom).abs() < 0.01,
            "zoom {zoom}: SIZE did not track factor"
        );
    }
}

#[test]
fn a_alca_desenhada_e_a_alca_que_o_designer_procura() {
    // The trap named by ADR 0019: handle position written in two
    // places. Here output is measured — what was PAINTED — against the calculation
    // `Designer::hit_handle` performs on the other side, which is `crawrad_runtime::alcas`
    // on the DESIGN rectangle.
    //
    // At 1.0 zoom both calculations agree even when the adapter adds
    // `ALCA_FORA` after converting. That is why zoom here is 2.0: it
    // distinguishes "add 3.5 on screen" from "add 3.5 in design and convert".
    let d = como_a_paleta_faz();
    let mut rt = runtime(&d);
    rt.set_zoom(2.0);
    rt.set_selection(&["Button1".to_string()]);

    let r = d.node("Button1").unwrap().rect().unwrap();
    let _ = retangulos_pintados(&mut rt);
    let pintados = retangulos_pintados(&mut rt);

    let lado = crawrad_runtime::HANDLE_SIDE * 2.0;
    for (ax, ay) in crawrad_runtime::handles(r.x, r.y, r.w, r.h) {
        // The screen calculation written HERE, not queried from the runtime.
        let centro = na_tela_de((0.0, 0.0), 2.0, ax, ay);
        assert!(
            pintados.iter().any(|q| {
                (q.center() - centro).length() < 0.5 && (q.width() - lado).abs() < 0.5
            }),
            "no {lado}pt handle centered at {centro:?};\npainted: {pintados:#?}"
        );
    }
}

#[test]
fn dar_zoom_no_meio_do_arrasto_nao_move_o_componente() {
    // The consequence of ADR 0019 on the frozen reference frame: `origem` AND
    // `escala`. Without freezing scale, the same screen pixel represents a
    // different design distance mid-gesture, and `delta` swallows the
    // difference — the component jumps, jumping further with larger steps.
    let d = como_a_paleta_faz();
    let mut rt = runtime(&d);

    let p = na_tela(100.0, 100.0);
    quadro(&mut rt, vec![egui::Event::PointerMoved(p), clique(p, true)]);
    assert_eq!(
        rt.drain_events(),
        vec![RuntimeEvent::PointerDown {
            x: 100.0,
            y: 100.0,
            ctrl: false
        }]
    );

    // Mid-drag, the zoom doubles.
    rt.set_zoom(2.0);
    let q = na_tela(110.0, 100.0);
    quadro(&mut rt, vec![egui::Event::PointerMoved(q)]);
    assert_eq!(
        rt.drain_events(),
        vec![RuntimeEvent::PointerMove {
            x: 110.0,
            y: 100.0,
            free: false
        }],
        "gesture continues in initial reference frame"
    );

    // On button release, the new zoom takes effect.
    quadro(&mut rt, vec![clique(q, false)]);
    let _ = rt.drain_events();
    let r = na_tela_de((0.0, 0.0), 2.0, 50.0, 50.0);
    quadro(&mut rt, vec![egui::Event::PointerMoved(r), clique(r, true)]);
    assert_eq!(
        rt.drain_events(),
        vec![RuntimeEvent::PointerDown {
            x: 50.0,
            y: 50.0,
            ctrl: false
        }],
        "gesture ended, new zoom takes effect"
    );
}

/// The cursor requested by the runtime in this frame.
///
/// Measures OUTPUT — egui's `platform_output` —, not intent: it is the only
/// honest way to ask "did the arrow appear?" without calling back into the same
/// function that decides it.
fn cursor_no_quadro(rt: &mut EguiRuntime, ponteiro: Pos2) -> egui::CursorIcon {
    let ctx = rt.ctx.clone();
    let entrada = egui::RawInput {
        screen_rect: Some(ERect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0))),
        events: vec![egui::Event::PointerMoved(ponteiro)],
        ..Default::default()
    };
    ctx.begin_pass(entrada);
    rt.frame();
    let mut saida = ctx.end_pass();
    saida.textures_delta.clear();
    saida.platform_output.cursor_icon
}

#[test]
fn as_tres_alcas_do_formulario_saem_da_mesma_conta_do_designer() {
    // FORM handles — E, S, and SE — were drawn at `form.max.x` and
    // `form.center().y`, without `ALCA_FORA`, whereas
    // `Designer::hit_form_handle` always looked for them at `Handle::E.anchor()`,
    // which is `x + w + 3.5`. Both calculations agreed by chance within the
    // slack of 7, and at 400% the divergence becomes 14 screen points.
    //
    // It was fixed without a guard initially, and the reviewer showed the cost:
    // mutating back to the old calculation left all 522 green. This
    // test is the guard — which is why zoom is 2.0: at 1.0 the 3.5 points
    // difference still fits within any loose tolerance.
    let mut d = como_a_paleta_faz();
    // Small form intentionally: at 200% all three handles remain inside
    // the canvas, so the test measures what a user would see.
    d.nodes
        .get_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 200.0, 150.0));
    d.validate().unwrap();

    let mut rt = runtime(&d);
    rt.set_zoom(2.0);
    let _ = retangulos_pintados(&mut rt);
    let pintados = retangulos_pintados(&mut rt);

    let f = d.node("form_main").unwrap().rect().unwrap();
    // The DESIGNER calculation, called here as it calls it — and the indices are the
    // `alcas` contract: 3 = E, 5 = S, 4 = SE.
    let a = crawrad_runtime::handles(f.x, f.y, f.w, f.h);
    let lado = crawrad_runtime::HANDLE_SIDE * 2.0;
    for (nome, (ax, ay)) in [("E", a[3]), ("S", a[5]), ("SE", a[4])] {
        // Screen conversion written HERE, not queried from the runtime.
        let centro = na_tela_de((0.0, 0.0), 2.0, ax, ay);
        assert!(
            pintados
                .iter()
                .any(|q| (q.center() - centro).length() < 0.5 && (q.width() - lado).abs() < 0.5),
            "form handle {nome} was not painted at {centro:?} — \
             that is where `hit_form_handle` looks for it;\npainted: {pintados:#?}"
        );
    }
}

#[test]
fn a_seta_de_redimensionar_cobre_a_mesma_folga_que_o_arrasto() {
    // The resize cursor appeared in an annulus of `ALCA_LADO/2 + 1.0` = 4.5 points, whereas
    // `Designer::hit_handle` accepts dragging within `ALCA_ALVO/2` = 7.
    // Between 4.5 and 7 the pointer stayed a default arrow over a GRABBABLE area —
    // the interface promising less than it delivers, which no one reports and everyone
    // notices. Mutating back to 4.5 left all 522 green.
    //
    // Zoom 2.0 again, killing two birds with one stone: besides the old annulus,
    // it catches comparisons in SCREEN points — 6 design units become 12
    // points, and 12 does not fit in any tolerance.
    let d = como_a_paleta_faz();
    let mut rt = runtime(&d);
    rt.set_zoom(2.0);
    rt.set_selection(&["Button1".to_string()]);

    let r = d.node("Button1").unwrap().rect().unwrap();
    let (ax, ay) = crawrad_runtime::handles(r.x, r.y, r.w, r.h)[4]; // SE
    let folga = crawrad_runtime::HANDLE_TARGET / 2.0;
    assert_eq!(folga, 7.0, "drag margin is 7 design units");

    // One unit INSIDE the drag tolerance: the arrow must be there.
    let perto = na_tela_de((0.0, 0.0), 2.0, ax + folga - 1.0, ay);
    let _ = cursor_no_quadro(&mut rt, perto);
    assert_eq!(
        cursor_no_quadro(&mut rt, perto),
        egui::CursorIcon::ResizeNwSe,
        "at {folga} design units handle GRABS, so arrow must appear"
    );

    // One unit OUTSIDE: the arrow must not promise what dragging rejects.
    let longe = na_tela_de((0.0, 0.0), 2.0, ax + folga + 1.0, ay);
    let _ = cursor_no_quadro(&mut rt, longe);
    assert_eq!(
        cursor_no_quadro(&mut rt, longe),
        egui::CursorIcon::Default,
        "outside margin drag rejects, and arrow must not invite"
    );
}

#[test]
fn zoom_que_nao_e_numero_nao_apaga_o_canvas() {
    // Rust's `clamp` returns **NaN** when input is NaN — it compares,
    // and every comparison with NaN is false. `set_zoom` documentation states that this
    // clamp is the boundary against the agent across MCP (ADR 0005), and
    // it had a loophole: `0.0` was covered, `NaN` was not. With NaN scale,
    // `na_tela` returns a NaN rectangle, egui paints nothing, and `em_design`
    // returns NaN — the canvas disappears and no clicks hit anything anymore.
    //
    // Ignore and keep the previous zoom, instead of falling back to minimum: sending
    // garbage was not choosing 25%, and changing framing due to an
    // invalid value would obey an unissued command.
    let d = como_a_paleta_faz();
    let mut rt = runtime(&d);
    rt.set_zoom(2.0);
    for lixo in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        rt.set_zoom(lixo);
        assert_eq!(
            rt.transformacao().escala,
            2.0,
            "{lixo} is not zoom: previous framing remains"
        );
    }
}

#[test]
fn o_clique_na_regua_nao_comeca_gesto() {
    // The ruler is a frame, not the drawing board. Without this guard, clicking it became
    // a marquee drag starting from a negative design coordinate — dragging over
    // nothing, from a place where no form exists.
    let d = como_a_paleta_faz();
    let mut rt = runtime(&d);

    // Inside the horizontal ruler band, above the drawing area.
    let p = Pos2::new(120.0, REGUA / 2.0);
    quadro(&mut rt, vec![egui::Event::PointerMoved(p), clique(p, true)]);
    assert!(
        rt.drain_events().is_empty(),
        "click on ruler is not a designer gesture"
    );
    assert!(!rt.busy());

    // One point further, now on the drawing board, starts normally.
    let q = na_tela(0.0, 0.0);
    quadro(&mut rt, vec![egui::Event::PointerMoved(q), clique(q, true)]);
    assert_eq!(
        rt.drain_events(),
        vec![RuntimeEvent::PointerDown {
            x: 0.0,
            y: 0.0,
            ctrl: false
        }]
    );
}

#[test]
fn a_regua_marca_onde_a_transformacao_diz() {
    // A ruler with its own arithmetic is the same rule written in two places
    // (ADR 0019). What is measured here is OUTPUT: the numbered tick must land
    // exactly where conversion places that design coordinate — and the screen
    // calculation is written in this file, not requested from the runtime.
    let d = como_a_paleta_faz();
    let mut rt = runtime(&d);
    rt.set_zoom(2.0);

    let ctx = rt.ctx.clone();
    let entrada = egui::RawInput {
        screen_rect: Some(ERect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0))),
        ..Default::default()
    };
    ctx.begin_pass(entrada);
    rt.frame();
    let mut saida = ctx.end_pass();
    saida.textures_delta.clear();

    // Line segments falling INSIDE the top ruler band.
    let tracos: Vec<f32> = saida
        .shapes
        .iter()
        .filter_map(|c| match &c.shape {
            egui::Shape::LineSegment { points, .. } => {
                let (a, b) = (points[0], points[1]);
                // Vertical and inside the top band: it is a horizontal ruler
                // tick, not its border or the pointer marker
                // (which does not exist: this frame has no pointer).
                (a.y <= REGUA && b.y <= REGUA && (a.x - b.x).abs() < 0.01).then_some(a.x)
            }
            _ => None,
        })
        .collect();
    assert!(!tracos.is_empty(), "ruler did not draw any tick");

    // At 200%, the chosen step is the finest that still leaves 40 points between
    // two numbers: 25 design units yield 50 screen points.
    let passo = passo_da_regua(2.0);
    assert_eq!(passo, 25.0, "step must track zoom");
    for n in 0..5 {
        let v = n as f32 * passo;
        let x = na_tela_de((0.0, 0.0), 2.0, v, 0.0).x;
        assert!(
            tracos.iter().any(|t| (t - x).abs() < 0.5),
            "no ruler tick at {v} units, which is x={x} on screen"
        );
    }
}

#[test]
fn no_preview_nao_ha_regua_nem_zoom() {
    // Preview shows the APPLICATION. Zooming there would lie twice: widgets
    // belong to egui and their font would not track the rectangle, and what
    // is being measured is precisely the size the user will see.
    let d = como_a_paleta_faz();
    let mut rt = runtime(&d);
    rt.set_zoom(4.0);
    rt.set_mode(Mode::Live);

    let t = rt.transformacao();
    assert_eq!(t.escala, 1.0, "in live mode scale is 1");
    assert_eq!(
        t.origem,
        rt.superficie().min,
        "in live mode origin is surface corner: there is no ruler"
    );

    // And returning to design restores the stored zoom — exiting
    // Preview does not lose framing.
    rt.set_mode(Mode::Design);
    assert_eq!(rt.transformacao().escala, 4.0);
}

#[test]
fn o_zoom_nao_aceita_valor_que_colapsaria_o_canvas() {
    // The catalog is also the MCP surface (ADR 0005): an agent may be on the other
    // side of this value. Zero would divide by zero on reverse
    // conversion — the form would collapse to a point and no clicks would find
    // anything.
    let d = como_a_paleta_faz();
    let mut rt = runtime(&d);

    rt.set_zoom(0.0);
    assert_eq!(rt.transformacao().escala, crawrad_runtime::ZOOMS[0]);
    rt.set_zoom(1000.0);
    assert_eq!(
        rt.transformacao().escala,
        crawrad_runtime::ZOOMS[crawrad_runtime::ZOOMS.len() - 1]
    );
}

// ---------------------------------------------------------------------------
// Palette tile
// ---------------------------------------------------------------------------

/// What `pintar_amostra` requested to draw, without a window.
///
/// Two frames because egui's first frame has not measured text yet — the same
/// reason as `retangulos_pintados` above.
fn formas_da_amostra(doc: &Document, area: ERect) -> Vec<egui::Shape> {
    let ctx = egui::Context::default();
    let mut colhido = Vec::new();
    for _ in 0..2 {
        let entrada = egui::RawInput {
            screen_rect: Some(ERect::from_min_size(Pos2::ZERO, Vec2::new(400.0, 400.0))),
            ..Default::default()
        };
        ctx.begin_pass(entrada);
        let p = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Background,
            egui::Id::new("amostra"),
        ));
        pintar_amostra(&p, area, doc);
        let mut saida = ctx.end_pass();
        saida.textures_delta.clear();
        colhido = saida.shapes.into_iter().map(|c| c.shape).collect();
    }
    colhido
}

/// A SINGLE component document, created as the user creates it — via
/// catalog `instantiate` (rule 9).
fn amostra_de(classe: &str) -> Document {
    let reg = crawrad_registry::Registry::builtin();
    let (w, h) = reg.get(classe).expect("class in catalog").default_size;
    let mut d = Document::new("form", "Form");
    d.nodes
        .get_mut("form")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, w, h));
    let n = reg
        .instantiate(classe, classe, Rect::new(0.0, 0.0, w, h))
        .expect("class in catalog");
    d.add_child("form", n).unwrap();
    d.validate().unwrap();
    d
}

#[test]
fn o_ladrilho_desenha_o_botao_pelo_desenho_do_botao() {
    // Proof that the palette DOES NOT have a second drawing implementation: a `Button`
    // tile comes from the same function the canvas uses, so it must show
    // the button border and its caption. Deleting either one in the
    // `"Button"` arm of `desenhar_controle` breaks this test **and** the
    // canvas test — which is precisely the point of having a single drawing implementation.
    let d = amostra_de("Button");
    let area = ERect::from_min_size(Pos2::new(10.0, 10.0), Vec2::new(72.0, 42.0));
    let formas = formas_da_amostra(&d, area);

    // The `Button` defaults to 96x32: fitted into 72x42 the scale is 72/96.
    //
    // The FACE and the OUTLINE, not "a rectangle of the right size": the Windows 11
    // button is nearly white, and the outline indicates where it is — WCAG
    // requires 3:1 for it for this very reason. Checking only one of the two allowed
    // deleting `rect_stroke` with the test remaining green, as verified by mutation.
    let escala = 72.0 / 96.0;
    let esperado = Vec2::new(96.0 * escala, 32.0 * escala);
    let do_tamanho_do_botao =
        |r: &ERect| (r.width() - esperado.x).abs() < 0.5 && (r.height() - esperado.y).abs() < 0.5;
    let molduras: Vec<(Vec2, Color32, Color32)> = formas
        .iter()
        .filter_map(|s| match s {
            egui::Shape::Rect(r) => Some((r.rect.size(), r.fill, r.stroke.color)),
            _ => None,
        })
        .collect();
    let (mut face, mut contorno) = (false, false);
    for s in &formas {
        let egui::Shape::Rect(r) = s else { continue };
        if !do_tamanho_do_botao(&r.rect) {
            continue;
        }
        face |= r.fill.a() > 0;
        contorno |= r.stroke.color.a() > 0 && r.stroke.width > 0.0;
    }
    assert!(
        face,
        "tile did not paint button face ({esperado:?}); painted {molduras:?}"
    );
    assert!(
        contorno,
        "tile did not paint button outline ({esperado:?}); painted {molduras:?}"
    );

    let escrito: Vec<String> = formas
        .iter()
        .filter_map(|s| match s {
            egui::Shape::Text(t) => Some(t.galley.text().to_string()),
            _ => None,
        })
        .collect();
    assert!(
        escrito.iter().any(|t| t == "Button"),
        "button caption was not painted; written: {escrito:?}"
    );
}

#[test]
fn o_ladrilho_encaixa_o_componente_e_nunca_o_amplia() {
    // Two fitting rules, measured from painted output.
    //
    // First: the component fits. A 200x90 `Memo` drawn at natural
    // size inside a 72-point tile would be clipped in half,
    // and half a Memo does not convey what a Memo is.
    //
    // Second: nothing is ENLARGED. A non-visual badge is 28 units, and
    // stretching it to fill the tile would show a component at a size
    // it never has in the form — the palette lying about the component.
    let area = ERect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(72.0, 42.0));
    for classe in ["Memo", "PageControl", "Timer", "CheckBox"] {
        let d = amostra_de(classe);
        for s in formas_da_amostra(&d, area) {
            let egui::Shape::Rect(r) = s else { continue };
            let r = r.rect;
            assert!(
                r.width() <= area.width() + 0.5 && r.height() <= area.height() + 0.5,
                "{classe}: painted {r:?}, larger than tile {area:?}"
            );
        }
    }
    // And the `Timer` badge, which could fit enlarged, appears at design size.
    let selos: Vec<Vec2> = formas_da_amostra(&amostra_de("Timer"), area)
        .iter()
        .filter_map(|s| match s {
            egui::Shape::Rect(r) => Some(r.rect.size()),
            _ => None,
        })
        .collect();
    assert!(
        selos
            .iter()
            .any(|s| (s.x - ICONE).abs() < 0.5 && (s.y - ICONE).abs() < 0.5),
        "Timer badge should have {ICONE} side; got {selos:?}"
    );
}

// ---------------------------------------------------------------------------
// Template preview — ADR 0021
// ---------------------------------------------------------------------------

/// Paints a sample in an actual frame and returns what was produced.
///
/// Two passes, as in the palette tile test: the first is still
/// building the font atlas, and its text is output without a galley.
fn amostra_pintada(doc: &Document, area: ERect, do_formulario: bool) -> Vec<egui::Shape> {
    let ctx = egui::Context::default();
    let mut fora = Vec::new();
    for _ in 0..2 {
        ctx.begin_pass(egui::RawInput {
            screen_rect: Some(ERect::from_min_size(Pos2::ZERO, Vec2::new(400.0, 400.0))),
            ..Default::default()
        });
        let p = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Background,
            egui::Id::new("amostra"),
        ));
        if do_formulario {
            pintar_amostra_do_formulario(&p, area, doc);
        } else {
            pintar_amostra(&p, area, doc);
        }
        let mut saida = ctx.end_pass();
        saida.textures_delta.clear();
        fora = saida.shapes.into_iter().map(|c| c.shape).collect();
    }
    fora
}

/// Painted paper — the background rectangle of the form.
fn papel_de(formas: &[egui::Shape]) -> Option<ERect> {
    formas.iter().find_map(|f| match f {
        egui::Shape::Rect(r) if r.fill == PAPEL => Some(r.rect),
        _ => None,
    })
}

#[test]
fn a_previa_encaixa_pelo_formulario_e_o_ladrilho_pelo_desenho() {
    // The difference between the two samples is THIS, and nothing else: the palette
    // tile fits by the component bounding box (the form behind is incidental), and
    // the template preview fits by the form — because in a template empty space
    // conveys information. Two buttons in a corner with remaining space empty must appear
    // like that, not inflated to fill the screen.
    let d = documento();
    // A SQUARE area against a 640x420 form: fitting by the
    // form leaves padding on top and bottom, which is what is measured. An area of the
    // same aspect ratio as the form would not distinguish anything.
    let area = ERect::from_min_size(Pos2::new(10.0, 10.0), Vec2::splat(300.0));

    let ladrilho = papel_de(&amostra_pintada(&d, area, false)).expect("tile paints paper");
    assert_eq!(
        ladrilho, area,
        "tile fills area: it is a PIECE of form, clipped by palette frame"
    );

    let previa = papel_de(&amostra_pintada(&d, area, true)).expect("preview paints paper");
    assert!(
        area.contains_rect(previa),
        "preview cannot exceed received area"
    );
    // 640x420 fitted in a square: width governs, and height follows proportionally.
    // Compared against the document, not against a hardcoded number
    // here — hardcoded numbers end up measuring the test instead of the drawing.
    let forma = d
        .node("form_main")
        .and_then(Node::rect)
        .expect("form has rect");
    assert!(
        (previa.width() - area.width()).abs() < 0.5,
        "fitted in a square, a landscape form occupies full width: {:?}",
        previa
    );
    assert!(
        (previa.height() / previa.width() - forma.h / forma.w).abs() < 0.01,
        "preview must match form aspect ratio — {:?} against {}x{}",
        previa,
        forma.w,
        forma.h
    );
    assert!(
        previa.height() < area.height() - 1.0,
        "extra margin top and bottom, otherwise fit was by area and not form"
    );
}

#[test]
fn a_previa_desenha_os_componentes_do_modelo() {
    // The preview is the RUNTIME drawing, not an image (ADR 0020/0021). What
    // proves this is the component text appearing painted: an embedded image
    // would pass all geometry tests and fail here.
    let d = documento();
    let area = ERect::from_min_size(Pos2::ZERO, Vec2::new(320.0, 240.0));
    let formas = amostra_pintada(&d, area, true);

    let textos: Vec<String> = formas
        .iter()
        .filter_map(|f| match f {
            egui::Shape::Text(t) => Some(t.galley.text().to_string()),
            _ => None,
        })
        .collect();
    // `documento()` has a Button with `&OK` — `&` is an accelerator and is omitted in the
    // galley, as `texto_centrado` does on canvas.
    assert!(
        textos.iter().any(|t| t.contains("OK")),
        "button caption was not painted; got {textos:?}"
    );
}

#[test]
fn a_previa_de_formulario_vazio_ainda_desenha_o_papel() {
    // `Blank form` is an actual template, and its preview is an empty
    // form — not a blank panel. Without this guarantee, fitting "by the
    // component bounding box" (which gives up when empty) would also apply
    // here, and the first item in the list would draw nothing.
    let mut d = Document::new("form_main", "Form");
    d.nodes
        .get_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
    let area = ERect::from_min_size(Pos2::ZERO, Vec2::new(320.0, 240.0));

    assert!(
        papel_de(&amostra_pintada(&d, area, true)).is_some(),
        "form without any components still has paper"
    );
    assert!(
        papel_de(&amostra_pintada(&d, area, false)).is_none(),
        "TILE gives up without components — behavior preview must not inherit"
    );
}

mod canvas_neutro {
    use super::*;

    /// WCAG contrast ratio between two opaque colors.
    fn contraste(a: Color32, b: Color32) -> f32 {
        fn lum(c: Color32) -> f32 {
            fn canal(v: u8) -> f32 {
                let v = v as f32 / 255.0;
                if v <= 0.04045 {
                    v / 12.92
                } else {
                    ((v + 0.055) / 1.055).powf(2.4)
                }
            }
            0.2126 * canal(c.r()) + 0.7152 * canal(c.g()) + 0.0722 * canal(c.b())
        }
        let (x, y) = (lum(a), lum(b));
        let (claro, escuro) = if x > y { (x, y) } else { (y, x) };
        (claro + 0.05) / (escuro + 0.05)
    }

    #[test]
    fn o_texto_do_formulario_se_le_sobre_o_papel() {
        // The bug this test exists to prevent already happened: the canvas
        // inherited `Visuals` from the IDE, and with dark chrome the form
        // label was output at **1.08:1** against the paper — invisible.
        for (rotulo, fundo) in [("paper", PAPEL), ("field", CTL_FUNDO), ("face", CTL_FACE)] {
            let r = contraste(CTL_TEXTO, fundo);
            assert!(r >= 4.5, "form text over {rotulo}: {r:.2}:1");
        }
        // Disabled text is fainter intentionally, but must still be readable.
        let r = contraste(CTL_TEXTO_FRACO, PAPEL);
        assert!(r >= 4.5, "disabled text over paper: {r:.2}:1");
    }

    #[test]
    fn o_contorno_diz_onde_o_controle_esta() {
        // WCAG requires 3:1 for the outline identifying a control — and here
        // it is the only thing separating a white field from near-white
        // paper. The same issue that caused `Panel` to disappear with `faint_bg_color`.
        for (rotulo, fundo) in [("paper", PAPEL), ("field", CTL_FUNDO), ("face", CTL_FACE)] {
            let r = contraste(CTL_BORDA, fundo);
            assert!(r >= 3.0, "control border over {rotulo}: {r:.2}:1");
        }
        // And `Panel`, which has its own constants since it disappeared into the paper.
        assert!(contraste(PAINEL_BORDA, PAINEL) >= 2.0);
        assert!(contraste(PAINEL, PAPEL) > 1.0);
    }

    #[test]
    fn o_estilo_do_papel_nao_vem_do_tema_da_casca() {
        // The rule: canvas shows the USER's form and belongs to no
        // design system. `estilo_do_papel` is a function without `Context` —
        // it cannot read the IDE theme, and that signature guards the
        // boundary. If someone passes a `Context` to it, this test will not even compile.
        let e = estilo_do_papel();
        assert_eq!(e.visuals.extreme_bg_color, CTL_FUNDO);
        assert_eq!(e.visuals.panel_fill, PAPEL);
        assert_eq!(e.visuals.text_color(), CTL_TEXTO);
        // And the paper governs the theme: light paper, light widgets.
        assert_eq!(e.visuals.dark_mode, PAPEL.r() < 0x80);
    }
}

mod conformidade {
    use super::*;

    /// The registry checks the adapter.
    ///
    /// A component entering the registry declaring this target without drawing
    /// support here does not disappear: it becomes a rectangle with the name inside, and the
    /// defect only shows when someone drags it onto the screen. With thirty
    /// components ahead and TWO adapters for each, forgetting is
    /// a matter of time — this test keeps track.
    #[test]
    fn toda_classe_do_registro_com_alvo_egui_tem_desenho() {
        let reg = crawrad_registry::Registry::builtin();
        for d in reg.all() {
            // `Form` is the surface, not a child; non-visual components become
            // icons in the designer and nothing in the application.
            if !d.visual || d.class == "Form" {
                continue;
            }
            if !d.supports("egui") {
                continue;
            }
            assert!(
                CLASSES.contains(&d.class),
                "`{}` declares `egui` target and RXP cannot draw it",
                d.class
            );
        }
    }

    /// And the adapter checks the registry: a class drawn here that does not exist
    /// there is leftover from a renamed or removed component.
    #[test]
    fn nao_desenhamos_classe_que_o_registro_nao_conhece() {
        let reg = crawrad_registry::Registry::builtin();
        for c in CLASSES {
            let d = reg
                .get(c)
                .unwrap_or_else(|| panic!("`{c}` is not in registry"));
            assert!(
                d.supports("egui"),
                "`{}` is drawn here but does not support `egui` target",
                d.class
            );
        }
    }

    /// **This target can namespace properties** — its name is in the
    /// contract.
    ///
    /// The sibling of this test lives in the Makepad adapter, and the full rationale is there:
    /// a target outside `contract::TARGETS` has any `<target>.x` property
    /// rejected by `validate()`, and target-specific properties get forced into
    /// the common core — breaking the rule of ADR 0002 while the suite remains green.
    ///
    /// Each adapter holds its own name, avoiding a manually maintained central
    /// target list that could be forgotten.
    #[test]
    fn o_nome_deste_alvo_esta_no_contrato() {
        assert!(
            crawrad_ir::contract::TARGETS.contains(&crate::ALVO),
            "`{}` is not in `contract::TARGETS` ({:?})",
            crate::ALVO,
            crawrad_ir::contract::TARGETS
        );
    }
}

#[test]
fn pages_of_is_empty_for_nodes_that_are_not_page_controls() {
    let mut doc = documento();
    let mut pc = Node::new("pc1", "PageControl");
    pc.set_rect(Rect::new(10.0, 10.0, 200.0, 100.0));
    doc.add_child("form_main", pc).unwrap();

    let mut ts1 = Node::new("ts1", "TabSheet");
    ts1.set_text("Tab1");
    doc.add_child("pc1", ts1).unwrap();

    let mut ts2 = Node::new("ts2", "TabSheet");
    ts2.set_text("Tab2");
    doc.add_child("pc1", ts2).unwrap();

    let rt = runtime(&doc);

    // For PageControl, returns tabs
    let (pages_pc, ativa_pc) = rt.paginas_de(&"pc1".to_string());
    assert_eq!(pages_pc, vec!["Tab1", "Tab2"]);
    assert_eq!(ativa_pc, 0);

    // For any other control, returns empty immediately without scanning
    let (pages_btn, ativa_btn) = rt.paginas_de(&"btn_ok".to_string());
    assert!(pages_btn.is_empty());
    assert_eq!(ativa_btn, 0);

    let (pages_form, ativa_form) = rt.paginas_de(&"form_main".to_string());
    assert!(pages_form.is_empty());
    assert_eq!(ativa_form, 0);
}

#[test]
fn a_bar_in_blocks_fills_from_its_start_and_stops_at_the_fraction() {
    let r = ERect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(150.0, 15.0));
    assert!(progress_blocks(r, 0.0, false).is_empty());
    let half = progress_blocks(r, 0.5, false);
    // 10 pt blocks with a 2 pt gap: the last block starts before 75
    assert_eq!(half[0].min.x, 0.0);
    assert_eq!(half[0].width(), 10.0);
    assert!(half.last().unwrap().min.x < 75.0);
    assert!(half.iter().all(|b| b.max.x <= 150.0));
    // vertical: from the bottom up
    let v = ERect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(15.0, 150.0));
    let up = progress_blocks(v, 0.5, true);
    assert_eq!(up[0].max.y, 150.0);
    assert!(up.last().unwrap().max.y > 75.0);
}

/// The Preview draws with the app's theme, and the paper stays the
/// drawing board's.
#[test]
fn the_preview_style_follows_the_form_theme() {
    let ctx = egui::Context::default();
    assert!(!app_style_for(&ctx, "thLight").visuals.dark_mode);
    assert!(app_style_for(&ctx, "thDark").visuals.dark_mode);
    // no system answer (no window): dark, as eframe does
    assert!(app_style_for(&ctx, "thSystem").visuals.dark_mode);
    assert_eq!(estilo_do_papel().visuals.panel_fill, PAPEL);
}
