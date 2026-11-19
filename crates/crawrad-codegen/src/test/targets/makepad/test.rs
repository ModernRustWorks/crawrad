use crate::Gerador;
use crate::targets::makepad::GeradorMakepad;
use crawrad_ir::{Document, Item, Node, Rect};

#[test]
fn o_gerador_makepad_emite_os_tres_arquivos() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
        form.set_text("My Makepad Form");
    }

    let mut btn = Node::new("btn_ok", "Button");
    btn.set_name("btn_ok");
    btn.set_text("Confirm");
    btn.events
        .insert("on_click".to_string(), "btn_ok_click".to_string());
    let _ = doc.add_child("Form1", btn);

    let gerador = GeradorMakepad;
    let arquivos = gerador
        .gerar(&doc, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("must generate successfully");

    assert_eq!(arquivos.len(), 3);
    let caminhos: Vec<_> = arquivos
        .iter()
        .map(|a| a.caminho.to_str().unwrap())
        .collect();
    assert!(caminhos.contains(&"src/main.rs"));
    assert!(caminhos.contains(&"src/Form1.designer.rs"));
    assert!(caminhos.contains(&"src/Form1.rs"));

    let main_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/main.rs")
        .unwrap();
    assert!(main_rs.conteudo.contains("Form1_designer::app_main();"));

    let dsl_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap();
    assert!(dsl_rs.conteudo.contains("pub struct Form1"));
    assert!(dsl_rs.conteudo.contains("live_design!"));
    assert!(dsl_rs.conteudo.contains("app_main!(App);"));
    assert!(dsl_rs.conteudo.contains("btn_ok = <Button>"));
    assert!(dsl_rs.conteudo.contains("id!(btn_ok)).clicked(actions)"));
    assert!(dsl_rs.conteudo.contains("self.form.btn_ok_click();"));

    let user_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.rs")
        .unwrap();
    assert!(user_rs.conteudo.contains("impl Form1"));
    assert!(user_rs.conteudo.contains("pub fn btn_ok_click(&mut self)"));
}

#[test]
fn o_gerador_makepad_emite_todos_os_componentes_e_itens() {
    use crawrad_ir::{Item, Value};

    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 760.0, 688.0));
        form.set_name("Form1");
        form.set_text("Form1");
    }

    // Button1
    let mut b1 = Node::new("b1", "Button");
    b1.set_name("button1");
    b1.set_text("Button1");
    b1.events.insert("on_click".into(), "Button1Click".into());
    doc.add_child("Form1", b1).unwrap();

    // ComboBox1
    let mut c1 = Node::new("c1", "ComboBox");
    c1.set_name("combo_box1");
    c1.props.insert(
        "items".into(),
        Item::list_to(&[Item::new("Option 1"), Item::new("Option 2")]),
    );
    c1.events
        .insert("on_change".into(), "ComboBox1Change".into());
    doc.add_child("Form1", c1).unwrap();

    // ListBox1
    let mut l1 = Node::new("l1", "ListBox");
    l1.set_name("list_box1");
    l1.props.insert(
        "items".into(),
        Item::list_to(&[Item::new("Item1"), Item::new("Item2"), Item::new("Item3")]),
    );
    doc.add_child("Form1", l1).unwrap();

    // Memo1
    let mut m1 = Node::new("m1", "Memo");
    m1.set_name("memo1");
    m1.props
        .insert("text".into(), Value::Str("Line 1\nLine 2\nLine 3".into()));
    m1.events.insert("on_change".into(), "Memo1Change".into());
    doc.add_child("Form1", m1).unwrap();

    // TreeView1
    let mut tv1 = Node::new("tv1", "TreeView");
    tv1.set_name("tree_view1");
    tv1.props.insert(
        "items".into(),
        Item::list_to(&[
            Item::with_children("Node1", vec![Item::new("SubNode1"), Item::new("SubNode2")]),
            Item::new("Item5"),
        ]),
    );
    doc.add_child("Form1", tv1).unwrap();

    let gerador = GeradorMakepad;
    let arquivos = gerador
        .gerar(
            &doc,
            "app_validacao",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("must generate successfully");

    let designer_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("Form1.designer.rs");

    // Validates that the Makepad ComboBox emits <CrawradComboBox> and syncs items
    assert!(
        designer_rs
            .conteudo
            .contains("combo_box1 = <CrawradComboBox>")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("combo_box.set_items(cx, self.form.combo_box1.items.as_slice());")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("combo_box.set_drop_down_count(cx, self.form.combo_box1.drop_down_count);")
    );
    // the ListBox's items live in the form's state (drawn at run time)
    assert!(
        designer_rs
            .conteudo
            .contains("items: Strings::from_slice(&[\"Item1\", \"Item2\"")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("text: \"Line 1\\nLine 2\\nLine 3\"")
    );
    // the nodes are run-time state, drawn by the list widget
    assert!(
        designer_rs
            .conteudo
            .contains("crawrad_runtime::tree::TreeNode::branch(\"Node1\"")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("crawrad_runtime::tree::TreeNode::leaf(\"SubNode1\")")
    );

    // Validates that the Makepad Form1 struct has typed fields in snake_case (idiomatic Rust)
    assert!(designer_rs.conteudo.contains("pub button1: Button,"));
    assert!(designer_rs.conteudo.contains("pub combo_box1: ComboBox,"));
    assert!(designer_rs.conteudo.contains("pub list_box1: ListBox,"));
    assert!(designer_rs.conteudo.contains("pub memo1: Memo,"));
    assert!(designer_rs.conteudo.contains("pub tree_view1: TreeView,"));
    assert!(!designer_rs.conteudo.contains("allow(non_snake_case)"));

    let main_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/main.rs")
        .expect("main.rs");
    assert!(main_rs.conteudo.contains("Form1_designer::app_main();"));
}

#[test]
fn stubs_gera_bloco_para_on_enter_e_on_exit() {
    let mut doc = Document::new("Form1", "Form");
    let mut b1 = Node::new("btn1", "Button");
    b1.set_name("button1");
    b1.events.insert("on_enter".into(), "Button1Enter".into());
    b1.events.insert("on_exit".into(), "Button1Exit".into());
    doc.add_child("Form1", b1).unwrap();

    let gerador = GeradorMakepad;
    let arquivos = gerador
        .gerar(&doc, "app_eventos", &crawrad_registry::Registry::builtin())
        .expect("must generate");

    let form_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.rs")
        .expect("Form1.rs");

    assert!(form_rs.conteudo.contains("pub fn Button1Enter(&mut self)"));
    assert!(form_rs.conteudo.contains("pub fn Button1Exit(&mut self)"));
    assert_eq!(form_rs.adicoes.len(), 2);
    assert!(
        form_rs
            .adicoes
            .iter()
            .any(|a| a.nome_da_fn == "Button1Enter")
    );
    assert!(
        form_rs
            .adicoes
            .iter()
            .any(|a| a.nome_da_fn == "Button1Exit")
    );
}

#[test]
fn page_control_emite_tabs_header_e_pages_body_sem_conflito_de_tabsheet() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
    }

    let mut pgc = Node::new("pgc1", "PageControl");
    pgc.set_name("page_control1");
    pgc.set_rect(Rect::new(40.0, 180.0, 320.0, 150.0));
    pgc.events
        .insert("on_change".into(), "PageControl1Change".into());
    doc.add_child("Form1", pgc).unwrap();

    let mut tab1 = Node::new("tab1", "TabSheet");
    tab1.set_name("tab_sheet1");
    tab1.set_text("First Tab");
    doc.add_child("pgc1", tab1).unwrap();

    let mut tab2 = Node::new("tab2", "TabSheet");
    tab2.set_name("tab_sheet2");
    tab2.set_text("Second Tab");
    doc.add_child("pgc1", tab2).unwrap();

    let mut btn = Node::new("btn1", "Button");
    btn.set_name("button1");
    btn.set_text("Button On Tab 1");
    doc.add_child("tab1", btn).unwrap();

    let gerador = GeradorMakepad;
    let arquivos = gerador
        .gerar(&doc, "app_pgc", &crawrad_registry::Registry::builtin())
        .expect("must generate");

    let designer_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer");

    // PageControl with tabs_header and child tabs
    assert!(designer_rs.conteudo.contains("page_control1 = <View>"));
    assert!(designer_rs.conteudo.contains("tabs_header = <View>"));
    assert!(designer_rs.conteudo.contains("text: \"First Tab\""));
    assert!(designer_rs.conteudo.contains("text: \"Second Tab\""));
    assert!(designer_rs.conteudo.contains("pages_body = <View>"));

    // TabSheet must not emit a stray <View> overlapping the form
    assert!(!designer_rs.conteudo.contains("tab_sheet1 = <View>"));
    assert!(!designer_rs.conteudo.contains("tab_sheet2 = <View>"));

    // The child button is emitted
    assert!(designer_rs.conteudo.contains("button1 = <Button>"));

    // Action dispatch for tab switching and on_change event
    assert!(
        designer_rs
            .conteudo
            .contains("self.ui.button(id!(page_control1.tabs_header.tab_0)).clicked(actions)")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("self.ui.button(id!(page_control1.tabs_header.tab_1)).clicked(actions)")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("self.form.PageControl1Change();")
    );
}

#[test]
fn tree_view_emite_itens_interativos_com_scrollbars_e_despacho() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
    }

    let mut tv = Node::new("tv1", "TreeView");
    tv.set_name("tree_view1");
    tv.set_rect(Rect::new(20.0, 20.0, 180.0, 200.0));
    tv.events
        .insert("on_change".into(), "TreeView1Change".into());
    tv.props.insert(
        "items".into(),
        Item::list_to(&[
            Item::with_children("Files", vec![Item::new("Doc1"), Item::new("Doc2")]),
            Item::new("Help"),
        ]),
    );
    doc.add_child("Form1", tv).unwrap();

    let gerador = GeradorMakepad;
    let arquivos = gerador
        .gerar(&doc, "app_tree", &crawrad_registry::Registry::builtin())
        .expect("must generate");

    let designer_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer");

    // a CrawradListBox drawn from the form's runtime tree nodes
    assert!(
        designer_rs
            .conteudo
            .contains("tree_view1 = <CrawradListBox>")
    );
    assert!(!designer_rs.conteudo.contains("item_0"));

    // Sync hands visible rows, row texts, indents, and selected
    assert!(designer_rs.conteudo.contains("visible_rows("));
    assert!(designer_rs.conteudo.contains("set_selected("));

    // Selection dispatch and on_change event
    assert!(
        designer_rs
            .conteudo
            .contains("self.ui.crawrad_list_box(id!(tree_view1)).selected(actions)")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("self.form.tree_view1.selected = Some(idx);")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("self.form.tree_view1.toggle_collapse(idx);")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("self.form.TreeView1Change();")
    );
}

#[test]
fn list_box_emite_itens_interativos_com_scrollbars_e_despacho() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
    }

    let mut lst = Node::new("lst1", "ListBox");
    lst.set_name("list_box1");
    lst.set_rect(Rect::new(20.0, 20.0, 140.0, 150.0));
    lst.events
        .insert("on_change".into(), "ListBox1Change".into());
    lst.props.insert(
        "items".into(),
        Item::list_to(&[Item::new("First"), Item::new("Second")]),
    );
    doc.add_child("Form1", lst).unwrap();

    let gerador = GeradorMakepad;
    let arquivos = gerador
        .gerar(&doc, "app_list", &crawrad_registry::Registry::builtin())
        .expect("must generate");

    let designer_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer");

    // a CrawradListBox (a scrolling PortalList) drawn from the form's items
    assert!(
        designer_rs
            .conteudo
            .contains("list_box1 = <CrawradListBox>")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("items: Strings::from_slice(&[\"First\", \"Second\"])")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("list_box.set_items(cx, &self.form.list_box1.items);")
    );

    // Selection dispatch and on_change event
    assert!(
        designer_rs
            .conteudo
            .contains("self.ui.crawrad_list_box(id!(list_box1)).selected(actions)")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("self.form.list_box1.selected = Some(_val);")
    );
    assert!(designer_rs.conteudo.contains("self.form.ListBox1Change();"));
}

#[test]
fn panel_groupbox_combobox_emitem_dsl_correto_e_acoes() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
    }

    let mut pnl = Node::new("pnl1", "Panel");
    pnl.set_name("panel1");
    pnl.set_text("Central Panel");
    pnl.set_rect(Rect::new(10.0, 10.0, 200.0, 100.0));
    pnl.events.insert("on_click".into(), "Panel1Click".into());
    doc.add_child("Form1", pnl).unwrap();

    let mut grp = Node::new("grp1", "GroupBox");
    grp.set_name("group_box1");
    grp.set_text("Personal Data");
    grp.set_rect(Rect::new(220.0, 10.0, 200.0, 100.0));
    grp.events
        .insert("on_click".into(), "GroupBox1Click".into());
    doc.add_child("Form1", grp).unwrap();

    let mut cb = Node::new("cb1", "ComboBox");
    cb.set_name("combo_box1");
    cb.set_rect(Rect::new(10.0, 120.0, 150.0, 30.0));
    cb.props.insert(
        "items".into(),
        Item::list_to(&[Item::new("Option A"), Item::new("Option B")]),
    );
    cb.props
        .insert("item_index".into(), crawrad_ir::Value::Int(1));
    cb.events
        .insert("on_change".into(), "ComboBox1Change".into());
    doc.add_child("Form1", cb).unwrap();

    let gerador = GeradorMakepad;
    let arquivos = gerador
        .gerar(&doc, "app_painel", &crawrad_registry::Registry::builtin())
        .expect("must generate");

    let designer_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer");

    // Styled Panel with paper and centered label
    assert!(designer_rs.conteudo.contains("panel1 = <View>"));
    assert!(designer_rs.conteudo.contains("draw_bg: { color: #252538 }"));
    assert!(designer_rs.conteudo.contains("text: \"Central Panel\""));

    // Styled GroupBox with header
    assert!(designer_rs.conteudo.contains("group_box1 = <View>"));
    assert!(designer_rs.conteudo.contains("draw_bg: { color: #1e1e2e }"));
    assert!(designer_rs.conteudo.contains("title = <Label>"));
    assert!(designer_rs.conteudo.contains("text: \"Personal Data\""));

    // the ComboBox is our own widget: items and selection come from the form
    assert!(
        designer_rs
            .conteudo
            .contains("combo_box1 = <CrawradComboBox>")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("items: Strings::from_slice(&[\"Option A\", \"Option B\"])")
    );
    assert!(designer_rs.conteudo.contains("selected: Some(1)"));

    // Panel and GroupBox on_click events via finger_up
    assert!(designer_rs.conteudo.contains(
        "if self.form.panel1.enabled && self.ui.view(id!(panel1)).finger_up(actions).is_some() {"
    ));
    assert!(designer_rs.conteudo.contains("self.form.Panel1Click();"));

    assert!(designer_rs
        .conteudo
        .contains("if self.form.group_box1.enabled && self.ui.view(id!(group_box1)).finger_up(actions).is_some() {"));
    assert!(designer_rs.conteudo.contains("self.form.GroupBox1Click();"));

    // ComboBox on_change event
    assert!(designer_rs.conteudo.contains(
        "let Some(_val) = self.ui.crawrad_combo_box(id!(combo_box1)).selected(actions) {"
    ));
    assert!(
        designer_rs
            .conteudo
            .contains("self.form.ComboBox1Change();")
    );
}

#[test]
fn scroll_bar_e_scroll_box_emitem_dsl_correto_e_acoes() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
    }

    let mut sb_h = Node::new("sb_h1", "ScrollBar");
    sb_h.set_name("scroll_bar_h");
    sb_h.set_rect(Rect::new(10.0, 10.0, 150.0, 20.0));
    sb_h.events
        .insert("on_change".into(), "ScrollBarHChange".into());
    doc.add_child("Form1", sb_h).unwrap();

    let mut sb_v = Node::new("sb_v1", "ScrollBar");
    sb_v.set_name("scroll_bar_v");
    sb_v.set_rect(Rect::new(180.0, 10.0, 20.0, 150.0));
    sb_v.events
        .insert("on_scroll".into(), "ScrollBarVScroll".into());
    doc.add_child("Form1", sb_v).unwrap();

    let mut sbox = Node::new("sbox1", "ScrollBox");
    sbox.set_name("scroll_box1");
    sbox.set_rect(Rect::new(10.0, 180.0, 200.0, 150.0));
    doc.add_child("Form1", sbox).unwrap();

    let gerador = GeradorMakepad;
    let arquivos = gerador
        .gerar(&doc, "app_scroll", &crawrad_registry::Registry::builtin())
        .expect("must generate");

    let designer_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer");

    // Horizontal ScrollBar (flow: Right, wide thumb)
    assert!(designer_rs.conteudo.contains("scroll_bar_h = <View>"));
    assert!(designer_rs.conteudo.contains("flow: Right"));
    assert!(designer_rs.conteudo.contains("thumb = <View>"));
    assert!(designer_rs.conteudo.contains("width: 32.0, height: Fill"));

    // Vertical ScrollBar (flow: Down, tall thumb)
    assert!(designer_rs.conteudo.contains("scroll_bar_v = <View>"));
    assert!(designer_rs.conteudo.contains("flow: Down"));
    assert!(designer_rs.conteudo.contains("width: Fill, height: 32.0"));

    // ScrollBox with native Makepad scroll_bars
    assert!(designer_rs.conteudo.contains("scroll_box1 = <RectView>"));
    assert!(designer_rs.conteudo.contains("scroll_bars: <ScrollBars> {"));
    assert!(designer_rs.conteudo.contains("show_scroll_x: true"));
    assert!(designer_rs.conteudo.contains("show_scroll_y: true"));

    // Action dispatch for ScrollBarH and ScrollBarV
    assert!(
        designer_rs
            .conteudo
            .contains("if self.ui.button(id!(scroll_bar_h.btn_prev)).clicked(actions) {")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("if self.ui.button(id!(scroll_bar_h.btn_next)).clicked(actions) {")
    );
    assert!(designer_rs.conteudo.contains(
        "if let Some(fd) = self.ui.view(id!(scroll_bar_h.track)).finger_down(actions) {"
    ));
    assert!(designer_rs.conteudo.contains(
        "if let Some(fm) = self.ui.view(id!(scroll_bar_h.track)).finger_move(actions) {"
    ));
    assert!(
        designer_rs
            .conteudo
            .contains("self.form.ScrollBarHChange();")
    );

    assert!(
        designer_rs
            .conteudo
            .contains("if self.ui.button(id!(scroll_bar_v.btn_prev)).clicked(actions) {")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("if self.ui.button(id!(scroll_bar_v.btn_next)).clicked(actions) {")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("self.form.ScrollBarVScroll();")
    );

    // sync_ui updates thumb margin
    assert!(
        designer_rs
            .conteudo
            .contains("self.ui.view(id!(scroll_bar_h.track.thumb)).apply_over(cx, live!{")
    );
    assert!(designer_rs.conteudo.contains("margin: { left: (offset) }"));
    assert!(designer_rs.conteudo.contains("margin: { top: (offset) }"));

    // Form1 struct with typed fields in snake_case
    assert!(
        designer_rs
            .conteudo
            .contains("pub scroll_bar_h: ScrollBar,")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("pub scroll_bar_v: ScrollBar,")
    );
    assert!(designer_rs.conteudo.contains("pub scroll_box1: ScrollBox,"));
}

#[test]
fn radio_button_emite_acao_com_find_widget_action_cast_e_desmarca_irmaos() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
    }

    let mut rb1 = Node::new("rb1", "RadioButton");
    rb1.set_name("radio_button1");
    rb1.set_text("Option 1");
    rb1.set_rect(Rect::new(10.0, 10.0, 120.0, 24.0));
    rb1.events
        .insert("on_click".into(), "RadioButton1Click".into());
    doc.add_child("Form1", rb1).unwrap();

    let mut rb2 = Node::new("rb2", "RadioButton");
    rb2.set_name("radio_button2");
    rb2.set_text("Option 2");
    rb2.set_rect(Rect::new(10.0, 40.0, 120.0, 24.0));
    doc.add_child("Form1", rb2).unwrap();

    let gerador = GeradorMakepad;
    let arquivos = gerador
        .gerar(&doc, "app_rb", &crawrad_registry::Registry::builtin())
        .expect("must generate");

    let designer_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer");

    // Validates that it uses find_widget_action_cast and RadioButtonAction::Clicked
    assert!(designer_rs.conteudo.contains(
        "let RadioButtonAction::Clicked = actions.find_widget_action_cast(self.ui.radio_button(id!(radio_button1)).widget_uid()) {"
    ));
    assert!(
        designer_rs
            .conteudo
            .contains("self.form.radio_button2.checked = false;")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("self.form.radio_button1.checked = true;")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("self.form.RadioButton1Click();")
    );

    // The second radio button also unchecks the first
    assert!(designer_rs.conteudo.contains(
        "let RadioButtonAction::Clicked = actions.find_widget_action_cast(self.ui.radio_button(id!(radio_button2)).widget_uid()) {"
    ));
    assert!(
        designer_rs
            .conteudo
            .contains("self.form.radio_button1.checked = false;")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("self.form.radio_button2.checked = true;")
    );
}

#[test]
fn sync_ui_sincroniza_visibilidade_e_texto_e_alterna_abas_de_page_control() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
    }

    let mut pgc = Node::new("pgc1", "PageControl");
    pgc.set_name("page_control1");
    pgc.set_rect(Rect::new(10.0, 10.0, 300.0, 200.0));
    doc.add_child("Form1", pgc).unwrap();

    let mut tab1 = Node::new("tab1", "TabSheet");
    tab1.set_name("tab_sheet1");
    tab1.set_text("Page 1");
    doc.add_child("pgc1", tab1).unwrap();

    let mut tab2 = Node::new("tab2", "TabSheet");
    tab2.set_name("tab_sheet2");
    tab2.set_text("Page 2");
    doc.add_child("pgc1", tab2).unwrap();

    let mut lst = Node::new("lst1", "ListBox");
    lst.set_name("list_box1");
    lst.set_rect(Rect::new(20.0, 40.0, 150.0, 100.0));
    doc.add_child("tab1", lst).unwrap();

    let mut memo = Node::new("memo1", "Memo");
    memo.set_name("memo1");
    memo.set_rect(Rect::new(10.0, 220.0, 200.0, 100.0));
    doc.add_child("Form1", memo).unwrap();

    let gerador = GeradorMakepad;
    let arquivos = gerador
        .gerar(&doc, "app_sync", &crawrad_registry::Registry::builtin())
        .expect("must generate");

    let designer_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer");

    // Validates that sync_ui is emitted and called after actions and on the first frame
    assert!(
        designer_rs
            .conteudo
            .contains("pub fn sync_ui(&mut self, cx: &mut Cx) {")
    );
    assert!(designer_rs.conteudo.contains("pub initial_sync: bool,"));
    assert!(designer_rs.conteudo.contains("if self.initial_sync {"));

    // Validates that the ListBox in TabSheet1 has its visibility tied to active_page == 0
    assert!(designer_rs.conteudo.contains(
        "self.ui.widget(id!(list_box1)).set_visible(cx, self.form.list_box1.visible && self.form.page_control1.active_page == 0);"
    ));

    // Validates that Memo1 has its visibility and text synchronized
    assert!(
        designer_rs
            .conteudo
            .contains("self.ui.widget(id!(memo1)).set_visible(cx, self.form.memo1.visible);")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("let ti = self.ui.text_input(id!(memo1));")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("ti.set_text(cx, &memo_text);")
    );

    // Validates that impl Default for Form1 initializes with designer values
    assert!(designer_rs.conteudo.contains("impl Default for Form1 {"));
    assert!(designer_rs
        .conteudo
        .contains("list_box1: ListBox { visible: true, enabled: true, selected: None, selection: Vec::new(), items: Strings::from_slice(&[]),"));

    // Validates tab highlighting and redraw
    assert!(designer_rs.conteudo.contains("self.ui.redraw(cx);"));
    assert!(designer_rs.conteudo.contains("color: #89b4fa"));
    assert!(designer_rs.conteudo.contains("color: #313244"));
}

#[test]
fn scroll_box_aninha_filhos_e_emite_scroll_bars() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
    }

    let mut sb = Node::new("sb1", "ScrollBox");
    sb.set_name("scroll_box1");
    sb.set_rect(Rect::new(40.0, 40.0, 200.0, 150.0));
    doc.add_child("Form1", sb).unwrap();

    let mut btn = Node::new("btn3", "Button");
    btn.set_name("button3");
    btn.set_text("Button3");
    btn.set_rect(Rect::new(150.0, 100.0, 80.0, 24.0));
    doc.add_child("sb1", btn).unwrap();

    let gerador = GeradorMakepad;
    let arquivos = gerador
        .gerar(&doc, "app_sb", &crawrad_registry::Registry::builtin())
        .expect("must generate successfully");

    let designer_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("Form1.designer.rs");

    // ScrollBox must contain View, scroll_bars, and the nested Button3
    assert!(designer_rs.conteudo.contains("scroll_box1 = <RectView> {"));
    assert!(designer_rs.conteudo.contains("scroll_bars: <ScrollBars> {"));
    assert!(designer_rs.conteudo.contains("show_scroll_x: true"));
    assert!(designer_rs.conteudo.contains("show_scroll_y: true"));
    assert!(designer_rs.conteudo.contains("button3 = <Button> {"));

    // Button3 must appear only ONCE in the DSL (nested inside ScrollBox, not in the root body)
    let occurrences = designer_rs.conteudo.matches("button3 = <Button>").count();
    assert_eq!(occurrences, 1, "button3 must be emitted exactly once");

    // Cascading visibility of the child with the ScrollBox
    assert!(designer_rs.conteudo.contains(
        "self.ui.widget(id!(button3)).set_visible(cx, self.form.button3.visible && self.form.scroll_box1.visible);"
    ));

    // Fields on the form struct
    assert!(designer_rs.conteudo.contains("pub scroll_box1: ScrollBox,"));
    assert!(designer_rs.conteudo.contains("pub button3: Button,"));
}

#[test]
fn todos_os_componentes_registrados_possuem_mapeamento_makepad() {
    let reg = crawrad_registry::Registry::builtin();
    let gerador = GeradorMakepad;
    let classes_gerador = gerador.classes();

    for desc in reg.all() {
        if desc.class != "Form" && desc.supports("makepad") && desc.emits_code {
            assert!(
                classes_gerador.contains(&desc.class),
                "Component `{}` is registered in crawrad-registry but is not listed in Makepad target classes",
                desc.class
            );
        }
    }
}

#[test]
fn componentes_definem_e_inicializam_o_proprio_estado_de_forma_autonoma() {
    use crate::targets::makepad::components::scroll_bar;

    let struct_def = scroll_bar::struct_definition();
    assert!(struct_def.contains("pub position: f32,"), "{struct_def}");
    assert!(struct_def.contains("pub min: f32,"), "{struct_def}");
    assert!(struct_def.contains("pub max: f32,"), "{struct_def}");
    assert!(
        struct_def.contains("pub small_change: f32,"),
        "{struct_def}"
    );
    assert!(
        struct_def.contains("pub large_change: f32,"),
        "{struct_def}"
    );

    let mut node = Node::new("sb_teste", "ScrollBar");
    let default_init = scroll_bar::emit_default(&node);
    assert!(default_init.contains("position: 0.0"), "{default_init}");
    assert!(default_init.contains("min: 0.0"), "{default_init}");
    assert!(default_init.contains("max: 100.0"), "{default_init}");

    // Value override by the designer
    node.props
        .insert("position".into(), crawrad_ir::Value::Float(45.5));
    let custom_init = scroll_bar::emit_default(&node);
    assert!(custom_init.contains("position: 45.5"), "{custom_init}");
}

#[test]
fn o_gerador_makepad_aninha_filhos_em_containers_e_sincroniza_sem_apply_over() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
    }

    // Panel with a child button
    let mut panel = Node::new("pnl1", "Panel");
    panel.set_name("panel1");
    panel.set_rect(Rect::new(10.0, 10.0, 200.0, 100.0));
    doc.add_child("Form1", panel).unwrap();

    let mut btn_inside_pnl = Node::new("btn_pnl", "Button");
    btn_inside_pnl.set_name("button_pnl");
    btn_inside_pnl.set_text("Inside Panel");
    btn_inside_pnl.set_rect(Rect::new(20.0, 20.0, 120.0, 30.0));
    doc.add_child("pnl1", btn_inside_pnl).unwrap();

    // GroupBox with a child label
    let mut gb = Node::new("gb1", "GroupBox");
    gb.set_name("group_box1");
    gb.set_text("Test Group");
    gb.set_rect(Rect::new(250.0, 10.0, 200.0, 100.0));
    doc.add_child("Form1", gb).unwrap();

    let mut lbl_inside_gb = Node::new("lbl_gb", "Label");
    lbl_inside_gb.set_name("label_gb");
    lbl_inside_gb.set_text("Inside Group");
    lbl_inside_gb.set_rect(Rect::new(10.0, 30.0, 100.0, 20.0));
    doc.add_child("gb1", lbl_inside_gb).unwrap();

    // TreeView
    let mut tv = Node::new("tv1", "TreeView");
    tv.set_name("tree_view1");
    tv.props.insert(
        "items".into(),
        Item::list_to(&[Item::with_children("Folder", vec![Item::new("File")])]),
    );
    doc.add_child("Form1", tv).unwrap();

    let gerador = GeradorMakepad;
    let arquivos = gerador
        .gerar(&doc, "app_nesting", &crawrad_registry::Registry::builtin())
        .expect("must generate");

    let designer_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer");

    // 1. The child button is nested inside the panel1 block in the Makepad DSL
    assert!(designer_rs.conteudo.contains("panel1 = <View>"));
    assert!(designer_rs.conteudo.contains("button_pnl = <Button>"));
    let pnl_idx = designer_rs.conteudo.find("panel1 = <View>").unwrap();
    let btn_idx = designer_rs.conteudo.find("button_pnl = <Button>").unwrap();
    assert!(btn_idx > pnl_idx, "button must be contained in the panel");

    // 2. The child label is nested inside the group_box1 block
    assert!(designer_rs.conteudo.contains("group_box1 = <View>"));
    // `<View>` and not `<Label>`: the label is WRAPPED, because Makepad's
    // `Label::draw_walk` consumes the walk twice and the margin is doubled
    // (see `components/label.rs`). The instance name belongs to the wrapper.
    assert!(designer_rs.conteudo.contains("label_gb = <View>"));
    let gb_idx = designer_rs.conteudo.find("group_box1 = <View>").unwrap();
    let lbl_idx = designer_rs.conteudo.find("label_gb = <View>").unwrap();
    assert!(lbl_idx > gb_idx, "label must be contained in the group_box");

    // 3. TreeView is emitted as CrawradListBox without per-item buttons
    assert!(!designer_rs.conteudo.contains("apply_over(cx, live!{"));
    assert!(
        designer_rs
            .conteudo
            .contains("tree_view1 = <CrawradListBox>")
    );
    assert!(!designer_rs.conteudo.contains("item_0"));
    assert!(
        designer_rs
            .conteudo
            .contains("self.ui.crawrad_list_box(id!(tree_view1)).selected(actions)")
    );
    assert!(designer_rs.conteudo.contains("visible_rows("));
    assert!(designer_rs.conteudo.contains("set_selected("));
    assert!(
        designer_rs
            .conteudo
            .contains("self.form.tree_view1.toggle_collapse(idx);")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("self.form.tree_view1.selected = Some(idx);")
    );
}

#[test]
fn o_formulario_makepad_herda_form_base_e_nao_injeta_codigo_de_teclado() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
        form.set_text("Registration");
    }

    let mut act_list = Node::new("al1", "ActionList");
    act_list.set_name("action_list1");
    doc.add_child("Form1", act_list).unwrap();

    let mut act = Node::new("act1", "Action");
    act.set_name("action_save");
    act.props
        .insert("text".into(), crawrad_ir::Value::Str("Save".into()));
    act.props
        .insert("shortcut".into(), crawrad_ir::Value::Str("Ctrl+S".into()));
    doc.add_child("al1", act).unwrap();

    let gerador = GeradorMakepad;
    let arquivos = gerador
        .gerar(&doc, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("must generate successfully");

    let designer_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer");

    // 1. The form imports the runtime prelude
    assert!(
        designer_rs
            .conteudo
            .contains("use crawrad_runtime::prelude::*;")
    );

    // 2. The form struct composes FormBase (analogous to class(TForm) in a classic RAD IDE)
    assert!(designer_rs.conteudo.contains("pub struct Form1 {"));
    assert!(designer_rs.conteudo.contains("pub base: FormBase,"));

    // 3. Implements Deref and Form trait
    assert!(
        designer_rs
            .conteudo
            .contains("impl std::ops::Deref for Form1")
    );
    assert!(designer_rs.conteudo.contains("impl Form for Form1"));

    // 4. Default initializes FormBase
    assert!(
        designer_rs
            .conteudo
            .contains("base: FormBase::new(\"Registration\", 640.0, 480.0)")
    );

    // 5. Does NOT inject raw text from shortcut.rs (KeyCode with match of hundreds of lines)
    assert!(
        !designer_rs
            .conteudo
            .contains("pub const fn as_str(self) -> &'static str")
    );
    assert!(!designer_rs.conteudo.contains("Self::ArrowDown => \"Down\""));
}

#[test]
fn o_formulario_makepad_suporta_heranca_visual_com_composicao() {
    let mut doc = Document::new_inherited("FormEspecial", "Form", "form_cliente");
    {
        let form = doc.node_mut("FormEspecial").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        form.set_name("FormEspecial");
        form.set_text("Special Customer");
    }

    let mut btn = Node::new("btn_vip", "Button");
    btn.set_name("btn_vip");
    btn.set_text("VIP Action");
    doc.add_child("FormEspecial", btn).unwrap();

    let gerador = GeradorMakepad;
    let arquivos = gerador
        .gerar(&doc, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("must generate successfully");

    let designer_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormEspecial.designer.rs")
        .expect("designer");

    // 1. The form struct composes base (HAS-A) with the corresponding type
    assert!(designer_rs.conteudo.contains("pub struct FormEspecial {"));
    assert!(designer_rs.conteudo.contains("pub base: FormCliente,"));
    assert!(designer_rs.conteudo.contains("pub btn_vip: Button,"));

    // 2. Implements Deref and DerefMut pointing to base
    assert!(
        designer_rs
            .conteudo
            .contains("impl std::ops::Deref for FormEspecial")
    );
    assert!(designer_rs.conteudo.contains("type Target = FormCliente;"));
    assert!(
        designer_rs
            .conteudo
            .contains("impl std::ops::DerefMut for FormEspecial")
    );

    // 3. Implements Form delegating to self.base.base()
    assert!(designer_rs.conteudo.contains("impl Form for FormEspecial"));
    assert!(designer_rs.conteudo.contains("self.base.base()"));
    assert!(designer_rs.conteudo.contains("self.base.base_mut()"));

    // 4. Default initializes base via FormCliente::default()
    assert!(
        designer_rs
            .conteudo
            .contains("base: FormCliente::default(),")
    );
}

/// The form's Color paints the window, and WindowState maximizes it once
/// the window id arrives with the first geometry event.
#[test]
fn the_makepad_form_takes_its_color_and_window_state() {
    let mut doc = Document::new("form_main", "Form");
    {
        let form = doc.node_mut("form_main").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("form_main");
        form.props.insert(
            "color".into(),
            crawrad_ir::Value::Color(crawrad_ir::Color::rgb(0x26, 0x26, 0x2E)),
        );
        form.props.insert(
            "window_state".into(),
            crawrad_ir::Value::Enum("wsMaximized".into()),
        );
    }
    let files = GeradorMakepad
        .gerar(&doc, "app", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let designer = &files
        .iter()
        .find(|a| a.caminho.to_str().unwrap().ends_with(".designer.rs"))
        .expect("the designer file")
        .conteudo;
    assert!(
        // `#26262E` would be lexed as a number with an empty exponent.
        designer.contains("draw_bg: { color: #x26262E }"),
        "{designer}"
    );
    assert!(
        designer.contains("cx.push_unique_platform_op(CxOsOp::MaximizeWindow(e.window_id));"),
        "{designer}"
    );
    assert!(
        designer.contains("pub crawrad_window_placed: bool,"),
        "{designer}"
    );
}

/// Edit and SpinEdit get the top padding that puts the capitals in
/// the middle: 24 px at the theme's 10 pt is 12 − 0.567 × 13.33. The ComboBox
/// is our own widget and centres them itself, by the same rule.
#[test]
fn single_line_fields_centre_their_capitals() {
    let mut doc = Document::new("Form1", "Form");
    let reg = crawrad_registry::Registry::builtin();
    for (id, class) in [("ed", "Edit"), ("sp", "SpinEdit"), ("cb", "ComboBox")] {
        let n = reg
            .instantiate(class, id, Rect::new(8.0, 8.0, 160.0, 24.0))
            .unwrap();
        let _ = doc.add_child("Form1", n);
    }
    let files = GeradorMakepad.gerar(&doc, "app", &reg).unwrap();
    let all: String = files.iter().map(|f| f.conteudo.as_str()).collect();
    let want = format!(
        "padding: {{ top: {:.1}, bottom: 0.0,",
        crawrad_component::text_line::cap_centred_top(
            24.0,
            crawrad_component::text_line::font_px(10.0)
        )
    );
    assert_eq!(all.matches(&want).count(), 2, "{want}");
}

/// The SpinEdit reads as one control: its square buttons sit inside the
/// field's frame (an overlay), never as rounded widgets beside it. Radius 0.5,
/// because a makepad 1.0 box of radius 0 paints no background at all.
#[test]
fn spin_edit_buttons_sit_inside_the_field() {
    let mut doc = Document::new("Form1", "Form");
    let reg = crawrad_registry::Registry::builtin();
    let n = reg
        .instantiate("SpinEdit", "sp", Rect::new(8.0, 8.0, 120.0, 24.0))
        .unwrap();
    let _ = doc.add_child("Form1", n);
    let files = GeradorMakepad.gerar(&doc, "app", &reg).unwrap();
    let all: String = files.iter().map(|f| f.conteudo.as_str()).collect();
    assert!(all.contains("flow: Overlay, align: { x: 1.0, y: 0.0 }"));
    assert_eq!(all.matches("draw_bg: { border_radius: 0.5 }").count(), 2);
    assert!(
        all.contains("right: 24.0 }"),
        "the number stops before the buttons"
    );
}

#[test]
fn makepad_form_on_create_runs_in_initial_sync_before_sync_ui() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
        form.events
            .insert("on_create".to_string(), "form_create".to_string());
    }

    let generator = GeradorMakepad;
    let files = generator
        .gerar(
            &doc,
            "app_on_create",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates successfully");

    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("Form1.designer.rs");

    // 1. The handler is called inside the initial_sync block, before sync_ui
    let expected_block = "        if self.initial_sync {\n            self.initial_sync = false;\n            #[cfg(target_arch = \"wasm32\")]\n            crawrad_runtime_makepad::look::light_page(cx, &self.ui);\n            self.form.form_create();\n            self.sync_ui(cx);\n        }";
    assert!(
        designer_rs.conteudo.contains(expected_block),
        "expected on_create inside initial_sync before sync_ui in:\n{}",
        designer_rs.conteudo
    );

    // 2. Called exactly once
    assert_eq!(
        designer_rs
            .conteudo
            .matches("self.form.form_create();")
            .count(),
        1,
        "handler call must be emitted exactly once"
    );

    // 3. No Default for App left
    assert!(
        !designer_rs.conteudo.contains("impl Default for App"),
        "impl Default for App should not be emitted"
    );
}

#[test]
fn makepad_form_without_on_create_emits_no_call_and_no_default_app() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
    }

    let generator = GeradorMakepad;
    let files = generator
        .gerar(
            &doc,
            "app_no_create",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates successfully");

    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("Form1.designer.rs");

    let expected_block = "        if self.initial_sync {\n            self.initial_sync = false;\n            #[cfg(target_arch = \"wasm32\")]\n            crawrad_runtime_makepad::look::light_page(cx, &self.ui);\n            self.sync_ui(cx);\n        }";
    assert!(
        designer_rs.conteudo.contains(expected_block),
        "expected initial_sync without on_create in:\n{}",
        designer_rs.conteudo
    );

    assert!(
        !designer_rs.conteudo.contains("self.form.form_create"),
        "no handler call should be emitted"
    );

    assert!(
        !designer_rs.conteudo.contains("impl Default for App"),
        "impl Default for App should not be emitted"
    );
}

/// BitBtn kind options map to standard caption in Makepad.
#[test]
fn bit_btn_kind_maps_to_standard_caption_in_makepad() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
    }
    let mut bbtn = Node::new("bbtn_close", "BitBtn");
    bbtn.set_name("bbtn_close");
    bbtn.props
        .insert("kind".into(), crawrad_ir::Value::Enum("bkClose".into()));
    doc.add_child("Form1", bbtn).unwrap();

    let generator = GeradorMakepad;
    let files = generator
        .gerar(&doc, "app_bbtn", &crawrad_registry::Registry::builtin())
        .expect("generates successfully");

    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("Form1.designer.rs");

    assert!(
        designer_rs.conteudo.contains("text: \"Close\""),
        "expected text: \"Close\" in:\n{}",
        designer_rs.conteudo
    );
}

/// Edit component text input properties and events in Makepad.
#[test]
fn edit_component_properties_and_events_in_makepad() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
    }
    let mut edt = Node::new("edt_input", "Edit");
    edt.set_name("edt_input");
    edt.set_text("secret");
    edt.props
        .insert("password_char".into(), crawrad_ir::Value::Str("*".into()));
    edt.props
        .insert("readonly".into(), crawrad_ir::Value::Bool(true));
    edt.props.insert(
        "text_hint".into(),
        crawrad_ir::Value::Str("Enter password".into()),
    );
    edt.props
        .insert("numbers_only".into(), crawrad_ir::Value::Bool(true));
    edt.props
        .insert("max_length".into(), crawrad_ir::Value::Int(10));
    edt.props.insert(
        "alignment".into(),
        crawrad_ir::Value::Enum("taRightJustify".into()),
    );
    edt.events.insert("on_change".into(), "edt_change".into());
    edt.events.insert("on_return".into(), "edt_return".into());
    doc.add_child("Form1", edt).unwrap();

    let generator = GeradorMakepad;
    let files = generator
        .gerar(&doc, "app_edit", &crawrad_registry::Registry::builtin())
        .expect("generates successfully");

    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("Form1.designer.rs");

    let content = &designer_rs.conteudo;
    assert!(
        content.contains("empty_text: \"Enter password\""),
        "empty_text in DSL"
    );
    assert!(content.contains("is_password: true"), "is_password in DSL");
    assert!(
        content.contains("is_read_only: true"),
        "is_read_only in DSL"
    );
    assert!(
        content.contains("is_numeric_only: true"),
        "is_numeric_only in DSL"
    );
    assert!(
        content.contains("label_align: { x: 1.0, y: 0.0 }"),
        "label_align in DSL"
    );

    assert!(
        content.contains("crawrad_runtime_makepad::edit::set_numbers_only"),
        "sync numbers_only"
    );
    assert!(
        content.contains("crawrad_runtime_makepad::edit::set_text_hint"),
        "sync text_hint"
    );
    assert!(
        content.contains("crawrad_runtime_makepad::edit::set_alignment"),
        "sync alignment"
    );

    assert!(content.contains("crawrad_runtime_makepad::memo::hold_to(cx, &self.ui.text_input(id!(edt_input)), &_val, self.form.edt_input.max_length)"), "hold_to for max_length");
    assert!(
        content.contains("self.form.edt_change();"),
        "on_change call"
    );
    assert!(
        content.contains("id!(edt_input)).returned(actions).is_some()"),
        "returned action check"
    );
    assert!(
        content.contains("self.form.edt_return();"),
        "on_return call"
    );
}

/// Makepad target generates OpenDialog and SaveDialog with execute() calling runtime helpers.
#[test]
fn makepad_open_dialog_and_save_dialog_emit_execute_and_state() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
    }

    let mut open_dlg = Node::new("dlg_open", "OpenDialog");
    open_dlg.set_name("dlg_open");
    open_dlg.props.insert(
        "title".into(),
        crawrad_ir::Value::Str("Open Project".into()),
    );
    open_dlg.props.insert(
        "filter".into(),
        crawrad_ir::Value::Str("Rust (*.rs)|*.rs".into()),
    );
    open_dlg
        .props
        .insert("initial_dir".into(), crawrad_ir::Value::Str("src".into()));
    open_dlg
        .props
        .insert("default_ext".into(), crawrad_ir::Value::Str("rs".into()));
    doc.add_child("Form1", open_dlg).unwrap();

    let mut save_dlg = Node::new("dlg_save", "SaveDialog");
    save_dlg.set_name("dlg_save");
    save_dlg.props.insert(
        "title".into(),
        crawrad_ir::Value::Str("Save Project".into()),
    );
    save_dlg.props.insert(
        "filter".into(),
        crawrad_ir::Value::Str("Rust (*.rs)|*.rs".into()),
    );
    save_dlg
        .props
        .insert("initial_dir".into(), crawrad_ir::Value::Str("src".into()));
    save_dlg
        .props
        .insert("default_ext".into(), crawrad_ir::Value::Str("rs".into()));
    doc.add_child("Form1", save_dlg).unwrap();

    let generator = GeradorMakepad;
    let files = generator
        .gerar(&doc, "app_dialogs", &crawrad_registry::Registry::builtin())
        .expect("generates successfully");

    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("Form1.designer.rs");

    // OpenDialog struct definition with files and execute() calling pick_open
    assert!(designer_rs.conteudo.contains("pub struct OpenDialog {"));
    assert!(designer_rs.conteudo.contains("pub files: Vec<String>,"));
    assert!(
        designer_rs
            .conteudo
            .contains("pub fn execute(&mut self) -> bool")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("crawrad_runtime::file_dialog::pick_open(&req)")
    );

    // SaveDialog struct definition and execute() calling pick_save
    assert!(designer_rs.conteudo.contains("pub struct SaveDialog {"));
    assert!(
        designer_rs
            .conteudo
            .contains("crawrad_runtime::file_dialog::pick_save(&req)")
    );

    // Default initialization contains the props
    assert!(
        designer_rs
            .conteudo
            .contains("title: \"Open Project\".to_string()")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("initial_dir: \"src\".to_string()")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("default_ext: \"rs\".to_string()")
    );
    assert!(designer_rs.conteudo.contains("files: Vec::new()"));

    // Sync should not emit dummy reads
    assert!(!designer_rs.conteudo.contains("&self.form.dlg_open.title"));
    assert!(!designer_rs.conteudo.contains("&self.form.dlg_save.title"));
}

#[test]
fn makepad_breadcrumb_emits_style_and_colors() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
    }

    let mut brc = Node::new("brc1", "Breadcrumb");
    brc.set_name("breadcrumb1");
    brc.props
        .insert("style".into(), crawrad_ir::Value::Enum("bsChevron".into()));
    brc.props.insert(
        "segment_color".into(),
        crawrad_ir::Value::Color(crawrad_ir::Color::rgb(0x33, 0x44, 0x55)),
    );
    brc.props.insert(
        "current_color".into(),
        crawrad_ir::Value::Color(crawrad_ir::Color::rgb(0xAA, 0xBB, 0xCC)),
    );
    doc.add_child("Form1", brc).unwrap();

    let generator = GeradorMakepad;
    let files = generator
        .gerar(
            &doc,
            "app_breadcrumb",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates successfully");

    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("Form1.designer.rs");

    assert!(
        designer_rs
            .conteudo
            .contains("breadcrumb1 = <CrawradBreadcrumb>")
    );
    assert!(designer_rs.conteudo.contains("style: \"bsChevron\""));
    assert!(designer_rs.conteudo.contains("segment_color: #x334455"));
    assert!(designer_rs.conteudo.contains("current_color: #AABBCC"));
    assert!(
        designer_rs
            .conteudo
            .contains("breadcrumb.set_style(cx, &self.form.breadcrumb1.style)")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("breadcrumb.set_segment_color(cx,")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("breadcrumb.set_current_color(cx,")
    );
}

#[test]
fn makepad_page_control_emits_fixed_tab_widths_scroll_buttons_and_sync_window() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
    }

    let mut pc = Node::new("pc1", "PageControl");
    pc.set_name("page_control1");
    pc.set_rect(Rect::new(10.0, 10.0, 320.0, 200.0));
    doc.add_child("Form1", pc).unwrap();

    let mut t1 = Node::new("t1", "TabSheet");
    t1.set_name("tab1");
    t1.set_text("Page 1");
    doc.add_child("pc1", t1).unwrap();

    let mut t2 = Node::new("t2", "TabSheet");
    t2.set_name("tab2");
    t2.set_text("Page 2");
    doc.add_child("pc1", t2).unwrap();

    let generator = GeradorMakepad;
    let files = generator
        .gerar(
            &doc,
            "app_page_control",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates successfully");

    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("Form1.designer.rs");

    // Fixed tab button and cell widths
    assert!(designer_rs.conteudo.contains("tab_0_cell = <View> {"));
    assert!(designer_rs.conteudo.contains("width: 92.0, height: 24.0"));
    assert!(designer_rs.conteudo.contains("tab_0 = <Button> {"));
    assert!(
        !designer_rs
            .conteudo
            .contains("margin: {left: 0.0, top: 0.0, right: 2.0, bottom: 0.0}")
    );

    // Scroll buttons in DSL
    assert!(
        designer_rs
            .conteudo
            .contains("tabs_fill = <View> { width: Fill, height: 24.0 }")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("tab_scroll_left = <Button> { visible: false, width: 16.0, height: 24.0")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("tab_scroll_right = <Button> { visible: false, width: 16.0, height: 24.0")
    );

    // Sync window
    assert!(designer_rs.conteudo.contains("let window = crawrad_runtime_makepad::tabs::window(320.0, &[self.form.tab1.tab_visible, self.form.tab2.tab_visible], self.form.page_control1.active_page);"));
    assert!(designer_rs.conteudo.contains("if self.ui.widget(id!(page_control1.tabs_header.tab_scroll_left)).visible() != window.arrows"));
    assert!(
        designer_rs
            .conteudo
            .contains("let shown = self.form.tab1.tab_visible && window.contains(0);")
    );

    // Actions for scroll buttons
    assert!(designer_rs.conteudo.contains(
        "if self.ui.button(id!(page_control1.tabs_header.tab_scroll_left)).clicked(actions)"
    ));
    assert!(designer_rs.conteudo.contains(
        "if self.ui.button(id!(page_control1.tabs_header.tab_scroll_right)).clicked(actions)"
    ));
    assert!(designer_rs.conteudo.contains(
        "crawrad_runtime_makepad::tabs::prev_page(self.form.page_control1.active_page, vis)"
    ));
    assert!(designer_rs.conteudo.contains(
        "crawrad_runtime_makepad::tabs::next_page(self.form.page_control1.active_page, vis)"
    ));
}

/// `AutoScroll` off closes both axes of the Makepad app too.
#[test]
fn the_makepad_scroll_box_without_auto_scroll_shows_no_bar() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
    let mut sb = Node::new("sb1", "ScrollBox");
    sb.set_name("scroll_box1");
    sb.set_rect(Rect::new(40.0, 40.0, 200.0, 150.0));
    sb.props
        .insert("auto_scroll".into(), crawrad_ir::Value::Bool(false));
    doc.add_child("Form1", sb).unwrap();
    let mut btn = Node::new("btn3", "Button");
    btn.set_name("button3");
    btn.set_rect(Rect::new(150.0, 100.0, 80.0, 124.0));
    doc.add_child("sb1", btn).unwrap();
    let files = GeradorMakepad
        .gerar(&doc, "app_sb", &crawrad_registry::Registry::builtin())
        .expect("it generates");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("Form1.designer.rs");
    assert!(designer_rs.conteudo.contains("show_scroll_x: false"));
    assert!(designer_rs.conteudo.contains("show_scroll_y: false"));
}

#[test]
fn component_structs_derives_items_and_strings_for_third_party_classes() {
    use crate::targets::makepad::components::component_structs;
    use crawrad_ir::Value;
    use crawrad_registry::{Category, ComponentDesc, PropDesc, PropKind, Props, Registry};

    let mut reg = Registry::builtin();

    // 1. Third-party class with strings-serving property
    let third_party_picker = ComponentDesc {
        inherits: None,
        class: "ThirdPartyPicker",
        display: "ThirdPartyPicker",
        base_name: "third_party_picker",
        palette: "Custom",
        prefix: "tpp",
        default_size: (120.0, 30.0),
        props: vec![
            Props::name(),
            PropDesc::new_typed(
                "options",
                "Options",
                Value::List(Vec::new()),
                Category::Appearance,
                PropKind::Collection {
                    hierarchical: false,
                    item_class: None,
                },
            ),
        ],
        events: Vec::new(),
        targets: crawrad_registry::UNIVERSAL_TARGETS,
        children: crawrad_registry::Children::NoChildren,
        derived_geometry: false,
        visual: true,
        emits_code: true,
        private_state: false,
        provides: &[],
        text_line: None,
        requires: Vec::new(),
        crate_module: "",
        desenho: Vec::new(),
        icon: Vec::new(),
    };
    reg.register(third_party_picker);

    // 2. Third-party collection container with typed item class
    let third_party_container = ComponentDesc {
        inherits: None,
        class: "ThirdPartyContainer",
        display: "ThirdPartyContainer",
        base_name: "third_party_container",
        palette: "Custom",
        prefix: "tpc",
        default_size: (100.0, 100.0),
        props: vec![Props::name()],
        events: Vec::new(),
        targets: crawrad_registry::UNIVERSAL_TARGETS,
        children: crawrad_registry::Children::Collection {
            item: "ThirdPartyItem",
        },
        derived_geometry: false,
        visual: false,
        emits_code: true,
        private_state: false,
        provides: &[],
        text_line: None,
        requires: Vec::new(),
        crate_module: "",
        desenho: Vec::new(),
        icon: Vec::new(),
    };
    reg.register(third_party_container);

    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));

    let mut picker_node = Node::new("picker1", "ThirdPartyPicker");
    picker_node.set_name("picker1");
    doc.add_child("Form1", picker_node).unwrap();

    let mut container_node = Node::new("cont1", "ThirdPartyContainer");
    container_node.set_name("cont1");
    doc.add_child("Form1", container_node).unwrap();

    let structs = component_structs(&doc, &reg);

    // STRINGS_TYPE_DEF is emitted because ThirdPartyPicker declares a PropKind::Collection property
    assert!(
        structs.contains("pub struct Strings"),
        "STRINGS_TYPE_DEF must be emitted for third-party class with Collection property"
    );

    // Collection item class derivation: ActionList without any child nodes in doc
    // derives Action from the catalogue descriptor
    let mut doc_al = Document::new("Form1", "Form");
    doc_al
        .node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
    let mut al_node = Node::new("acl1", "ActionList");
    al_node.set_name("action_list1");
    doc_al.add_child("Form1", al_node).unwrap();

    let structs_al = component_structs(&doc_al, &reg);
    assert!(
        structs_al.contains("pub struct Action {"),
        "Action struct must be emitted when ActionList is used, derived from collection item class"
    );
}

#[test]
fn tree_view_items_are_runtime_nodes() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
    }

    let mut tv = Node::new("tv1", "TreeView");
    tv.set_name("tree_view1");
    tv.set_rect(Rect::new(20.0, 20.0, 180.0, 200.0));
    tv.props.insert(
        "items".into(),
        Item::list_to(&[Item::with_children("Folder", vec![Item::new("File")])]),
    );
    doc.add_child("Form1", tv).unwrap();

    let generator = GeradorMakepad;
    let files = generator
        .gerar(
            &doc,
            "app_tree_nodes",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("should generate");

    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer");

    assert!(
        designer_rs
            .conteudo
            .contains("pub items: Vec<crawrad_runtime::tree::TreeNode>,")
    );
    assert!(designer_rs.conteudo.contains("TreeNode::branch("));
    assert!(designer_rs.conteudo.contains("TreeNode::leaf("));
}

#[test]
fn tree_view_collapsed_initialiser_in_makepad() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
    }

    let mut branch = Item::with_children("Folder", vec![Item::new("File")]);
    branch.expanded = false;

    let mut tv = Node::new("tv1", "TreeView");
    tv.set_name("tree_view1");
    tv.set_rect(Rect::new(20.0, 20.0, 180.0, 200.0));
    tv.props.insert("items".into(), Item::list_to(&[branch]));
    doc.add_child("Form1", tv).unwrap();

    let generator = GeradorMakepad;
    let files = generator
        .gerar(
            &doc,
            "app_tree_collapsed",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("should generate");

    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer");

    assert!(
        designer_rs
            .conteudo
            .contains("collapsed: std::collections::BTreeSet::from([0])"),
        "expected BTreeSet::from([0]) in designer_rs, got:\n{}",
        designer_rs.conteudo
    );
}

#[test]
fn tree_view_with_linked_image_list_emits_symbol() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
    }

    let mut il = Node::new("il1", "ImageList");
    il.set_name("il1");
    let mut item0 = std::collections::BTreeMap::new();
    item0.insert("name".to_string(), crawrad_ir::Value::Str("first".into()));
    item0.insert("symbol".to_string(), crawrad_ir::Value::Str("📁".into()));
    let mut item1 = std::collections::BTreeMap::new();
    item1.insert("name".to_string(), crawrad_ir::Value::Str("second".into()));
    item1.insert("symbol".to_string(), crawrad_ir::Value::Str("📄".into()));
    il.props.insert(
        "images_data".into(),
        crawrad_ir::Value::List(vec![
            crawrad_ir::Value::Map(item0),
            crawrad_ir::Value::Map(item1),
        ]),
    );
    doc.add_child("Form1", il).unwrap();

    let mut tv_item = Item::new("Node1");
    tv_item.image_index = 1;

    let mut tv = Node::new("tv1", "TreeView");
    tv.set_name("tv1");
    tv.props.insert("items".into(), Item::list_to(&[tv_item]));
    tv.props
        .insert("images".into(), crawrad_ir::Value::Ref("il1".into()));
    doc.add_child("Form1", tv).unwrap();

    let generator = GeradorMakepad;
    let files = generator
        .gerar(
            &doc,
            "tree_icon_app",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("should generate");

    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer");

    assert!(
        designer_rs.conteudo.contains(".with_symbol(\"📄\")"),
        "expected with_symbol(\"📄\") in designer_rs, got:\n{}",
        designer_rs.conteudo
    );

    // With images empty, emits no with_symbol
    let tv_no_images = doc.node_mut("tv1").unwrap();
    tv_no_images
        .props
        .insert("images".into(), crawrad_ir::Value::Ref("".into()));

    let files2 = generator
        .gerar(
            &doc,
            "tree_no_icon_app",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("should generate");
    let designer2 = files2
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer");

    assert!(
        !designer2.conteudo.contains(".with_symbol("),
        "expected NO with_symbol when images is empty, got:\n{}",
        designer2.conteudo
    );
}

#[test]
fn an_unbound_grid_is_a_string_grid_in_makepad() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));

    let mut grid = Node::new("grid1", "Grid");
    grid.set_name("grid1");
    grid.set_rect(Rect::new(10.0, 10.0, 300.0, 200.0));
    grid.events.insert(
        "on_select_cell".to_string(),
        "grid_cell_selected".to_string(),
    );
    doc.add_child("Form1", grid).unwrap();

    let generator = GeradorMakepad;
    let files = generator
        .gerar(&doc, "grid_app", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer")
        .conteudo
        .as_str();

    assert!(designer_rs.contains("grid1 = <CrawradStringGrid>"));
    assert!(designer_rs.contains("set_grid("));
    assert!(designer_rs.contains("pub cells: crawrad_runtime::string_grid::Cells,"));
    assert!(designer_rs.contains("cells: Default::default()"));
    assert!(designer_rs.contains("self.form.grid_cell_selected(col, row);"));
    assert!(!designer_rs.contains("<CrawradDataGrid>"));
    assert!(!designer_rs.contains(".clicked("));
    assert!(!designer_rs.contains(".double_clicked("));
}

#[test]
fn unbound_grid_in_makepad_initializes_cells_from_cell_text() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));

    let mut grid = Node::new("grid1", "Grid");
    grid.set_name("grid1");
    grid.set_rect(Rect::new(10.0, 10.0, 300.0, 200.0));
    grid.props.insert(
        "cell_text".to_string(),
        crawrad_ir::Value::Str("Name\tAge\nAna\t30".to_string()),
    );
    doc.add_child("Form1", grid).unwrap();

    let generator = GeradorMakepad;
    let files = generator
        .gerar(&doc, "grid_app", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer")
        .conteudo
        .as_str();

    assert!(designer_rs.contains("Cells::from_text(\"Name\\tAge\\nAna\\t30\")"));
}

#[test]
fn unbound_grid_in_makepad_handles_click_and_double_click() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));

    let mut grid = Node::new("grid1", "Grid");
    grid.set_name("grid1");
    grid.set_rect(Rect::new(10.0, 10.0, 300.0, 200.0));
    grid.events
        .insert("on_click".to_string(), "grid1_click".to_string());
    grid.events
        .insert("on_dbl_click".to_string(), "grid1_dbl_click".to_string());
    doc.add_child("Form1", grid).unwrap();

    let generator = GeradorMakepad;
    let files = generator
        .gerar(&doc, "grid_app", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer")
        .conteudo
        .as_str();

    assert!(designer_rs.contains(
        "if self.ui.crawrad_string_grid(id!(grid1)).clicked(actions) {\n                self.form.grid1_click();"
    ));
    assert!(designer_rs.contains(
        "if self.ui.crawrad_string_grid(id!(grid1)).double_clicked(actions) {\n                self.form.grid1_dbl_click();"
    ));
}

#[test]
fn unbound_grid_in_makepad_supports_cell_editing() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));

    let mut grid = Node::new("grid1", "Grid");
    grid.set_name("grid1");
    grid.set_rect(Rect::new(10.0, 10.0, 300.0, 200.0));
    doc.add_child("Form1", grid).unwrap();

    let generator = GeradorMakepad;
    let files = generator
        .gerar(&doc, "grid_app", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer")
        .conteudo
        .as_str();

    assert!(designer_rs.contains("self.form.grid1.editing"));
    assert!(designer_rs.contains(
        "if let Some((col, row, text)) = self.ui.crawrad_string_grid(id!(grid1)).cell_edited(actions) {\n                self.form.grid1.cells.set(col, row, text);\n            }"
    ));
}

#[test]
fn combo_box_sync_emits_sorted_check_before_handover() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut cb = Node::new("cb1", "ComboBox");
    cb.set_name("combo_box1");
    cb.set_rect(Rect::new(10.0, 10.0, 150.0, 24.0));
    doc.add_child("Form1", cb).unwrap();

    let generator = GeradorMakepad;
    let files = generator
        .gerar(
            &doc,
            "app_combo_sorted",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer")
        .conteudo
        .as_str();

    let expected_sort = "        if self.form.combo_box1.sorted {\n            crawrad_runtime::list_selection::sort(&mut self.form.combo_box1.items, &mut self.form.combo_box1.selected, &mut []);\n        }";
    assert!(
        designer_rs.contains(expected_sort),
        "expected sorted block in designer_rs, got:\n{designer_rs}"
    );

    let sort_pos = designer_rs
        .find("crawrad_runtime::list_selection::sort(&mut self.form.combo_box1.items, &mut self.form.combo_box1.selected, &mut []);")
        .expect("sort call exists");
    let handover_pos = designer_rs
        .find("combo_box.set_items(cx, self.form.combo_box1.items.as_slice());")
        .expect("the items handed over to the combo box");
    assert!(
        sort_pos < handover_pos,
        "sort must come BEFORE handing items over to drop down"
    );
}

#[test]
fn form_show_hint_controls_hint_visibility() {
    use crawrad_ir::Value;

    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
    doc.node_mut("Form1")
        .unwrap()
        .props
        .insert("show_hint".into(), Value::Bool(false));

    let mut btn = Node::new("btn1", "Button");
    btn.set_name("btn1");
    btn.props
        .insert("hint".into(), Value::Str("click me".into()));
    doc.add_child("Form1", btn).unwrap();

    let files = GeradorMakepad
        .gerar(&doc, "app_hint", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer")
        .conteudo
        .as_str();

    assert!(designer_rs.contains("const CRAWRAD_FORM_SHOW_HINT: bool = false;"));
    assert!(designer_rs.contains(
        "if CRAWRAD_FORM_SHOW_HINT && self.form.btn1.show_hint && !self.form.btn1.hint.is_empty()"
    ));

    // without the root's show_hint set, generates = true;
    doc.node_mut("Form1").unwrap().props.remove("show_hint");
    let files_true = GeradorMakepad
        .gerar(
            &doc,
            "app_hint_true",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates");
    let designer_true = files_true
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer")
        .conteudo
        .as_str();

    assert!(designer_true.contains("const CRAWRAD_FORM_SHOW_HINT: bool = true;"));
    assert!(designer_true.contains(
        "if CRAWRAD_FORM_SHOW_HINT && self.form.btn1.show_hint && !self.form.btn1.hint.is_empty()"
    ));
}

#[test]
fn memo_scroll_bars_ss_none_and_unset_emits_bare_text_input() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut mmo1 = Node::new("mmo1", "Memo");
    mmo1.set_name("memo_plain");
    mmo1.set_rect(Rect::new(10.0, 10.0, 200.0, 90.0));
    doc.add_child("Form1", mmo1).unwrap();

    let mut mmo2 = Node::new("mmo2", "Memo");
    mmo2.set_name("memo_none");
    mmo2.set_rect(Rect::new(10.0, 120.0, 200.0, 90.0));
    mmo2.props.insert(
        "scroll_bars".into(),
        crawrad_ir::Value::Enum("ssNone".into()),
    );
    doc.add_child("Form1", mmo2).unwrap();

    let files = GeradorMakepad
        .gerar(
            &doc,
            "app_memo_none",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer")
        .conteudo
        .as_str();

    // ssNone and unset emit bare <TextInput>
    assert!(designer_rs.contains("memo_plain = <TextInput> {"));
    assert!(designer_rs.contains("memo_none = <TextInput> {"));
    assert!(!designer_rs.contains("memo_plain = <RoundedView> {"));
    assert!(!designer_rs.contains("memo_none = <RoundedView> {"));
    assert!(designer_rs.contains("let ti = self.ui.text_input(id!(memo_plain));"));
    assert!(designer_rs.contains("let ti = self.ui.text_input(id!(memo_none));"));
}

#[test]
fn memo_scroll_bars_ss_vertical_emits_view_wrapper_and_inner_text_input() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut mmo = Node::new("mmo1", "Memo");
    mmo.set_name("memo_vert");
    mmo.set_rect(Rect::new(10.0, 10.0, 200.0, 90.0));
    mmo.props.insert(
        "scroll_bars".into(),
        crawrad_ir::Value::Enum("ssVertical".into()),
    );
    doc.add_child("Form1", mmo).unwrap();

    let files = GeradorMakepad
        .gerar(
            &doc,
            "app_memo_vert",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer")
        .conteudo
        .as_str();

    // ssVertical emits View wrapper with show_scroll_x: false, show_scroll_y: true
    assert!(designer_rs.contains("memo_vert = <RoundedView> {"));
    assert!(designer_rs.contains("show_scroll_x: false"));
    assert!(designer_rs.contains("show_scroll_y: true"));
    assert!(designer_rs.contains("memo_vert_text = <TextInput> {"));
    assert!(designer_rs.contains("width: Fill, height: Fit"));
    assert!(designer_rs.contains("let ti = self.ui.text_input(id!(memo_vert.memo_vert_text));"));
}

/// With scroll bars, the Memo's keyboard-and-text widget is the inner
/// TextInput: the Tab sequence, the key router and the font all address it,
/// and none addresses the scrolling box as if it were the text input.
#[test]
fn memo_with_scroll_bars_is_reached_through_its_inner_text_input() {
    let reg = crawrad_registry::Registry::builtin();
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
    let mut mmo = reg
        .instantiate("Memo", "mmo1", Rect::new(10.0, 10.0, 200.0, 90.0))
        .expect("Memo");
    mmo.set_name("memo_vert");
    mmo.props.insert(
        "scroll_bars".into(),
        crawrad_ir::Value::Enum("ssVertical".into()),
    );
    doc.add_child("Form1", mmo).unwrap();
    let mut edt = reg
        .instantiate("Edit", "edt1", Rect::new(10.0, 120.0, 120.0, 24.0))
        .expect("Edit");
    edt.set_name("edit1");
    doc.add_child("Form1", edt).unwrap();

    let files = GeradorMakepad
        .gerar(&doc, "app_memo_paths", &reg)
        .expect("generates");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer")
        .conteudo
        .as_str();

    let inner = "self.ui.widget(id!(memo_vert.memo_vert_text))";
    assert!(
        designer_rs.matches(inner).count() >= 2,
        "the Tab list and the text look must address the inner TextInput:\n{designer_rs}"
    );
    assert!(
        !designer_rs.contains("self.ui.widget(id!(memo_vert))),"),
        "the Tab list addresses the scrolling box:\n{designer_rs}"
    );
}

#[test]
fn memo_scroll_bars_ss_both_emits_both_axes() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut mmo = Node::new("mmo1", "Memo");
    mmo.set_name("memo_both");
    mmo.set_rect(Rect::new(10.0, 10.0, 200.0, 90.0));
    mmo.props.insert(
        "scroll_bars".into(),
        crawrad_ir::Value::Enum("ssBoth".into()),
    );
    doc.add_child("Form1", mmo).unwrap();

    let files = GeradorMakepad
        .gerar(
            &doc,
            "app_memo_both",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer")
        .conteudo
        .as_str();

    // ssBoth emits show_scroll_x: true, show_scroll_y: true
    assert!(designer_rs.contains("memo_both = <RoundedView> {"));
    assert!(designer_rs.contains("show_scroll_x: true"));
    assert!(designer_rs.contains("show_scroll_y: true"));
    assert!(designer_rs.contains("memo_both_text = <TextInput> {"));
    assert!(designer_rs.contains("height: Fit"));
    assert!(designer_rs.contains("let ti = self.ui.text_input(id!(memo_both.memo_both_text));"));
}

#[test]
fn memo_scroll_bars_word_wrap_off_with_horizontal_uses_fit_width() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut mmo = Node::new("mmo1", "Memo");
    mmo.set_name("memo_nowrap");
    mmo.set_rect(Rect::new(10.0, 10.0, 200.0, 90.0));
    mmo.props.insert(
        "scroll_bars".into(),
        crawrad_ir::Value::Enum("ssBoth".into()),
    );
    mmo.props
        .insert("word_wrap".into(), crawrad_ir::Value::Bool(false));
    doc.add_child("Form1", mmo).unwrap();

    let files = GeradorMakepad
        .gerar(
            &doc,
            "app_memo_nowrap",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer")
        .conteudo
        .as_str();

    assert!(designer_rs.contains("memo_nowrap_text = <TextInput> {"));
    assert!(designer_rs.contains("width: Fit, height: Fit"));
    assert!(designer_rs.contains("flow: Right"));
}

/// A TabSheet's Hint shows over its tab on Makepad.
#[test]
fn tab_sheet_hint_shows_over_tab_in_makepad() {
    use crawrad_ir::Value;

    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut pc = Node::new("pc1", "PageControl");
    pc.set_name("page_control1");
    pc.set_rect(Rect::new(10.0, 10.0, 320.0, 200.0));
    doc.add_child("Form1", pc).unwrap();

    let mut t1 = Node::new("t1", "TabSheet");
    t1.set_name("tab1");
    t1.set_text("Page 1");
    t1.props.insert("hint".into(), Value::Str("first".into()));
    doc.add_child("pc1", t1).unwrap();

    let mut t2 = Node::new("t2", "TabSheet");
    t2.set_name("tab2");
    t2.set_text("Page 2");
    doc.add_child("pc1", t2).unwrap();

    let generator = GeradorMakepad;
    let files = generator
        .gerar(&doc, "app_tab_hint", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer file");

    let content = &designer_rs.conteudo;
    assert!(
        content.contains("pub hint: String,"),
        "TabSheet struct must have hint field in state:\n{content}"
    );
    assert!(
        content.contains("pub show_hint: bool,"),
        "TabSheet struct must have show_hint field in state:\n{content}"
    );
    assert!(
        content.contains("hint: \"first\".to_string()"),
        "TabSheet default init must have hint:\n{content}"
    );
    assert!(
        content.contains(
            "self.ui.widget(id!(page_control1.tabs_header.tab_0)).area().rect(cx).contains(e.abs)"
        ),
        "expected hover check on tab_0, got:\n{content}"
    );
    assert!(
        content.contains("self.form.tab1.hint"),
        "expected self.form.tab1.hint in tab hover check, got:\n{content}"
    );
}

#[test]
fn makepad_live_register_honours_form_theme() {
    let mut doc_light = Document::new("Form1", "Form");
    doc_light
        .node_mut("Form1")
        .unwrap()
        .props
        .insert("theme".into(), crawrad_ir::Value::Enum("thLight".into()));
    let generator = GeradorMakepad;
    let files_light = generator
        .gerar(
            &doc_light,
            "meu_app",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("must generate successfully");
    let designer_light = files_light
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap();
    assert!(
        designer_light
            .conteudo
            .contains("cx.link(live_id!(theme), live_id!(crawrad_theme_light));"),
        "thLight must emit the link for crawrad_theme_light"
    );

    // thDark links CrawRAD's dark copy, never makepad's own (its code font is GPL)
    let mut doc_dark = Document::new("Form1", "Form");
    doc_dark
        .node_mut("Form1")
        .unwrap()
        .props
        .insert("theme".into(), crawrad_ir::Value::Enum("thDark".into()));
    let files_dark = generator
        .gerar(&doc_dark, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("must generate successfully");
    let designer_dark = files_dark
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap();
    assert!(
        !designer_dark.conteudo.contains("crawrad_theme_light"),
        "thDark must not emit the link for crawrad_theme_light"
    );
    assert!(
        designer_dark
            .conteudo
            .contains("cx.link(live_id!(theme), live_id!(crawrad_theme_dark));"),
        "thDark must link crawrad_theme_dark"
    );
    assert!(!designer_dark.conteudo.contains("look::light_page"));
    assert!(
        designer_light
            .conteudo
            .contains("draw_bg: { color: #ffffff }"),
        "thLight is a white page"
    );
}

/// The form's default `thSystem` keeps the desktop dark and links the
/// light theme, with a white page, only in the WebAssembly build.
#[test]
fn makepad_system_theme_is_a_white_page_only_in_the_browser() {
    let doc = Document::new("Form1", "Form");
    let files = GeradorMakepad
        .gerar(&doc, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("must generate successfully");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap();
    let code = &designer.conteudo;
    assert!(code.contains(
        "        #[cfg(target_arch = \"wasm32\")]\n        cx.link(live_id!(theme), live_id!(crawrad_theme_light));\n        #[cfg(not(target_arch = \"wasm32\"))]\n        cx.link(live_id!(theme), live_id!(crawrad_theme_dark));\n"
    ));
    assert!(code.contains(
        "            #[cfg(target_arch = \"wasm32\")]\n            crawrad_runtime_makepad::look::light_page(cx, &self.ui);\n"
    ));
    assert!(
        code.contains("draw_bg: { color: #242424 }"),
        "the desktop stays dark"
    );
}

#[test]
fn makepad_page_control_honours_tab_stop_and_arrow_keys() {
    use crawrad_ir::Value;

    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut edit = Node::new("edt1", "Edit");
    edit.set_name("edit1");
    edit.props.insert("tab_order".into(), Value::Int(0));
    doc.add_child("Form1", edit).unwrap();

    let mut pc = Node::new("pc1", "PageControl");
    pc.set_name("page_control1");
    pc.props.insert("tab_order".into(), Value::Int(1));
    doc.add_child("Form1", pc).unwrap();

    let mut t1 = Node::new("t1", "TabSheet");
    t1.set_name("tab1");
    t1.set_text("First");
    doc.add_child("pc1", t1).unwrap();

    let mut t2 = Node::new("t2", "TabSheet");
    t2.set_name("tab2");
    t2.set_text("Second");
    doc.add_child("pc1", t2).unwrap();

    let files = GeradorMakepad
        .gerar(&doc, "app_tab_stop", &crawrad_registry::Registry::builtin())
        .expect("generates successfully");

    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("Form1.designer.rs");

    // The Tab list contains the PageControl's stop addressing its active tab, after the Edit
    let at_edit = designer_rs
        .conteudo
        .find("id!(edit1)")
        .expect("edit in Tab list");
    let at_pc = designer_rs
        .conteudo
        .find("self.form.page_control1.active_page")
        .expect("page control in Tab list");
    assert!(at_edit < at_pc, "PageControl tab stop is after Edit");
    assert!(
        designer_rs
            .conteudo
            .contains("page_control1.tabs_header.tab_0")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("page_control1.tabs_header.tab_1")
    );

    // The generated key handling for Left/Right references the page control and its active_page
    assert!(designer_rs.conteudo.contains("KeyCode::ArrowLeft"));
    assert!(designer_rs.conteudo.contains("KeyCode::ArrowRight"));
    assert!(
        designer_rs.conteudo.contains(
            "crawrad_runtime_makepad::tabs::prev_page(self.form.page_control1.active_page"
        )
    );
    assert!(
        designer_rs.conteudo.contains(
            "crawrad_runtime_makepad::tabs::next_page(self.form.page_control1.active_page"
        )
    );

    // Focus cue in sync passes is_focused to tabs::look
    assert!(
        designer_rs
            .conteudo
            .contains("crawrad_runtime_makepad::tabs::look(cx, &self.ui.widget(id!(page_control1.tabs_header.tab_0_mark)), &self.ui.widget(id!(page_control1.tabs_header.tab_0_ring)), is_active, is_focused);")
    );
    // the focus lands a pass after it is asked for: the ring follows it then,
    // and a tab not drawn yet (no area) holds no focus
    assert!(
        designer_rs
            .conteudo
            .contains("if let Event::KeyFocus(_) | Event::KeyFocusLost(_) = event {")
    );
    assert!(designer_rs.conteudo.contains(
        "crawrad_runtime_makepad::tabs::focused(cx, &self.ui.widget(id!(page_control1.tabs_header.tab_0)))"
    ));

    // With tab_stop false: no stop for page control
    doc.node_mut("pc1")
        .unwrap()
        .props
        .insert("tab_stop".into(), Value::Bool(false));
    let files_no_stop = GeradorMakepad
        .gerar(
            &doc,
            "app_no_tab_stop",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates successfully");
    let designer_no_stop = files_no_stop
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("Form1.designer.rs");
    let tab_list_section = designer_no_stop
        .conteudo
        .split("let crawrad_tab = [")
        .nth(1)
        .and_then(|s| s.split("];").next())
        .unwrap_or("");
    assert!(
        !tab_list_section.contains("page_control1"),
        "PageControl has no stop when tab_stop is false"
    );
}

#[test]
fn makepad_inherited_event_not_overridden_calls_base_dispatch() {
    let reg = crawrad_registry::Registry::builtin();
    let mut base_doc = Document::new("BaseForm", "Form");
    let mut btn = Node::new("btn_save", "Button");
    btn.set_name("btn_save");
    btn.events
        .insert("on_click".into(), "btn_save_click".into());
    base_doc.add_child("BaseForm", btn).unwrap();

    let mut derived_doc = Document::new("DerivedForm", "Form");
    derived_doc.set_inherits(Some("BaseForm"));

    let files = GeradorMakepad
        .generate_with_base(&derived_doc, "app", &reg, Some(&base_doc), false)
        .expect("generates successfully");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/DerivedForm.designer.rs")
        .unwrap();

    assert!(designer.conteudo.contains("self.form.base.dispatch(fired)"));
}

#[test]
fn makepad_overridden_inherited_event_calls_derived_handler_and_node_not_emitted() {
    let reg = crawrad_registry::Registry::builtin();
    let mut base_doc = Document::new("BaseForm", "Form");
    let mut btn = Node::new("btn_ok", "Button");
    btn.set_name("btn_ok");
    btn.events.insert("on_click".into(), "btn_ok_click".into());
    base_doc.add_child("BaseForm", btn).unwrap();

    let mut derived_doc = Document::new("DerivedForm", "Form");
    derived_doc.set_inherits(Some("BaseForm"));
    let mut override_btn = Node::new("btn_ok", "Button");
    override_btn.set_name("btn_ok");
    override_btn
        .events
        .insert("on_click".into(), "btn_derived_click".into());
    derived_doc.add_child("DerivedForm", override_btn).unwrap();

    let files = GeradorMakepad
        .generate_with_base(&derived_doc, "app", &reg, Some(&base_doc), false)
        .expect("generates successfully");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/DerivedForm.designer.rs")
        .unwrap();

    assert!(designer.conteudo.contains("self.form.btn_derived_click();"));
    assert!(!designer.conteudo.contains("pub btn_ok:"));
    // the field is the base's, but the widget is drawn here, once
    assert_eq!(designer.conteudo.matches("btn_ok = <Button>").count(), 1);
}

#[test]
fn makepad_derived_form_draws_and_syncs_the_base_components() {
    let reg = crawrad_registry::Registry::builtin();
    let mut base_doc = Document::new("BaseForm", "Form");
    let mut btn = Node::new("btn_save", "Button");
    btn.set_name("btn_save");
    btn.events
        .insert("on_click".into(), "btn_save_click".into());
    base_doc.add_child("BaseForm", btn).unwrap();
    let mut label = Node::new("lbl_status", "Label");
    label.set_name("lbl_status");
    base_doc.add_child("BaseForm", label).unwrap();

    let mut derived_doc = Document::new("DerivedForm", "Form");
    derived_doc.set_inherits(Some("BaseForm"));
    let mut own = Node::new("btn_extra", "Button");
    own.set_name("btn_extra");
    derived_doc.add_child("DerivedForm", own).unwrap();

    let files = GeradorMakepad
        .generate_with_base(&derived_doc, "app", &reg, Some(&base_doc), false)
        .expect("generates successfully");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/DerivedForm.designer.rs")
        .unwrap();
    let code = &designer.conteudo;

    for widget in [
        "btn_save = <Button>",
        "lbl_status = <View>",
        "btn_extra = <Button>",
    ] {
        assert_eq!(code.matches(widget).count(), 1, "{widget} drawn once");
    }
    // the base's components are drawn first, under the derived ones
    assert!(code.find("btn_save = <Button>") < code.find("btn_extra = <Button>"));
    // their state is the base's, reached through Deref
    assert!(code.contains("self.ui.widget(id!(btn_save)).set_visible("));
    assert!(code.contains("self.form.btn_save.enabled"));
    assert!(!code.contains("pub btn_save:"));
    assert!(code.contains("pub btn_extra:"));
}

#[test]
fn makepad_overridden_event_stub_body_calls_base_handler() {
    let reg = crawrad_registry::Registry::builtin();
    let mut base_doc = Document::new("BaseForm", "Form");
    let mut btn = Node::new("btn_ok", "Button");
    btn.set_name("btn_ok");
    btn.events.insert("on_click".into(), "btn_ok_click".into());
    base_doc.add_child("BaseForm", btn).unwrap();

    let mut derived_doc = Document::new("DerivedForm", "Form");
    derived_doc.set_inherits(Some("BaseForm"));
    let mut override_btn = Node::new("btn_ok", "Button");
    override_btn.set_name("btn_ok");
    override_btn
        .events
        .insert("on_click".into(), "btn_derived_click".into());
    derived_doc.add_child("DerivedForm", override_btn).unwrap();

    let files = GeradorMakepad
        .generate_with_base(&derived_doc, "app", &reg, Some(&base_doc), false)
        .expect("generates successfully");
    let user_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/DerivedForm.rs")
        .unwrap();

    assert!(
        user_rs
            .conteudo
            .contains("pub fn btn_derived_click(&mut self)")
    );
    assert!(user_rs.conteudo.contains("self.base.btn_ok_click();"));
}

#[test]
fn makepad_base_form_reports_and_dispatches() {
    let reg = crawrad_registry::Registry::builtin();
    let mut base_doc = Document::new("FormBase", "Form");
    let mut btn = Node::new("btn_ok", "Button");
    btn.set_name("btn_ok");
    btn.events.insert("on_click".into(), "btn_ok_click".into());
    base_doc.add_child("FormBase", btn).unwrap();

    let files = GeradorMakepad
        .generate_with_base(&base_doc, "app", &reg, None, true)
        .expect("generates successfully");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormBase.designer.rs")
        .unwrap();

    assert!(designer.conteudo.contains("pub enum FormBaseFired {"));
    assert!(designer.conteudo.contains("BtnOkClick,"));
    assert!(
        designer
            .conteudo
            .contains("pub fn handle_actions_reporting")
    );
    assert!(
        designer
            .conteudo
            .contains("pub fn dispatch(&mut self, fired: FormBaseFired)")
    );
    assert!(
        designer
            .conteudo
            .contains("crawrad_fired.push(FormBaseFired::BtnOkClick);")
    );
}

#[test]
fn makepad_base_form_with_is_base_false_has_no_reporting() {
    let reg = crawrad_registry::Registry::builtin();
    let mut base_doc = Document::new("FormBase", "Form");
    let mut btn = Node::new("btn_ok", "Button");
    btn.set_name("btn_ok");
    btn.events.insert("on_click".into(), "btn_ok_click".into());
    base_doc.add_child("FormBase", btn).unwrap();

    let files = GeradorMakepad
        .generate_with_base(&base_doc, "app", &reg, None, false)
        .expect("generates successfully");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormBase.designer.rs")
        .unwrap();

    assert!(!designer.conteudo.contains("FormBaseFired"));
    assert!(!designer.conteudo.contains("handle_actions_reporting"));
    assert!(!designer.conteudo.contains("pub fn dispatch"));
    assert!(designer.conteudo.contains("self.form.btn_ok_click();"));
}

#[test]
fn makepad_mut_arg_or_if_call_is_not_rewritten() {
    let reg = crawrad_registry::Registry::builtin();
    let mut base_doc = Document::new("FormBase", "Form");
    let mut tv = Node::new("tv1", "TreeView");
    tv.set_name("tv1");
    tv.events.insert("on_changing".into(), "tv_changing".into());
    base_doc.add_child("FormBase", tv).unwrap();

    let files = GeradorMakepad
        .generate_with_base(&base_doc, "app", &reg, None, true)
        .expect("generates successfully");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormBase.designer.rs")
        .unwrap();

    assert!(!designer.conteudo.contains("TvChanging"));
    assert!(
        designer
            .conteudo
            .contains("self.form.tv_changing(&mut allow_change);")
    );
}

#[test]
fn makepad_derived_override_and_non_override() {
    let reg = crawrad_registry::Registry::builtin();
    let mut base_doc = Document::new("BaseForm", "Form");
    let mut btn1 = Node::new("btn_ok", "Button");
    btn1.set_name("btn_ok");
    btn1.events.insert("on_click".into(), "btn_ok_click".into());
    base_doc.add_child("BaseForm", btn1).unwrap();
    let mut btn2 = Node::new("btn_cancel", "Button");
    btn2.set_name("btn_cancel");
    btn2.events
        .insert("on_click".into(), "btn_cancel_click".into());
    base_doc.add_child("BaseForm", btn2).unwrap();

    let mut derived_doc = Document::new("DerivedForm", "Form");
    derived_doc.set_inherits(Some("BaseForm"));
    let mut override_btn = Node::new("btn_ok", "Button");
    override_btn.set_name("btn_ok");
    override_btn
        .events
        .insert("on_click".into(), "btn_derived_click".into());
    derived_doc.add_child("DerivedForm", override_btn).unwrap();

    let files = GeradorMakepad
        .generate_with_base(&derived_doc, "app", &reg, Some(&base_doc), false)
        .expect("generates successfully");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/DerivedForm.designer.rs")
        .unwrap();

    assert!(
        designer
            .conteudo
            .contains("use crate::BaseForm_designer::{BaseForm, BaseFormFired};")
    );
    assert!(designer.conteudo.contains("BaseFormFired::BtnOkClick => {"));
    assert!(designer.conteudo.contains("self.form.btn_derived_click();"));
    assert!(designer.conteudo.contains("fired => {"));
    assert!(
        designer
            .conteudo
            .contains("self.form.base.dispatch(fired);")
    );
}

#[test]
fn makepad_form_with_no_wired_events_generates_no_fired_enum_and_output_unchanged() {
    let reg = crawrad_registry::Registry::builtin();
    let mut doc = Document::new("FormSimple", "Form");
    let mut btn = Node::new("btn_simple", "Button");
    btn.set_name("btn_simple");
    doc.add_child("FormSimple", btn).unwrap();

    let files = GeradorMakepad
        .gerar(&doc, "app", &reg)
        .expect("generates successfully");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormSimple.designer.rs")
        .unwrap();

    assert!(!designer.conteudo.contains("FormSimpleFired"));
    assert!(!designer.conteudo.contains("handle_actions_reporting"));
    assert!(!designer.conteudo.contains("pub fn dispatch"));
}

#[test]
fn makepad_reported_events_returns_only_rewritten_events() {
    let reg = crawrad_registry::Registry::builtin();
    let mut base_doc = Document::new("FormBase", "Form");
    let mut btn = Node::new("btn_ok", "Button");
    btn.set_name("btn_ok");
    btn.events.insert("on_click".into(), "btn_ok_click".into());
    base_doc.add_child("FormBase", btn).unwrap();

    let evs = crate::targets::makepad::form::reported_events(&reg, &base_doc, &[]).unwrap();
    assert_eq!(evs.len(), 1);
    assert_eq!(evs[0].variant, "BtnOkClick");
}

#[test]
fn makepad_combobox_emits_crawrad_combo_box_and_drop_down_count() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut cbo = Node::new("cbo1", "ComboBox");
    cbo.set_name("combo_box1");
    cbo.props
        .insert("drop_down_count".into(), crawrad_ir::Value::Int(12));
    cbo.props.insert(
        "items".into(),
        Item::list_to(&[Item::new("OptionA"), Item::new("OptionB")]),
    );
    doc.add_child("Form1", cbo).unwrap();

    let files = GeradorMakepad
        .gerar(&doc, "app_cbo", &crawrad_registry::Registry::builtin())
        .expect("generates successfully");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap();

    assert!(
        designer_rs
            .conteudo
            .contains("combo_box1 = <CrawradComboBox>")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("combo_box.set_drop_down_count(cx, self.form.combo_box1.drop_down_count);")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("use crawrad_runtime_makepad::combo_box::CrawradComboBoxWidgetRefExt;")
    );
}

#[test]
fn makepad_combobox_emits_editable_and_text_sync() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut cbo = Node::new("cbo1", "ComboBox");
    cbo.set_name("combo_box1");
    cbo.props
        .insert("style".into(), crawrad_ir::Value::Enum("DropDown".into()));
    cbo.props
        .insert("text".into(), crawrad_ir::Value::Str("Sample".into()));
    cbo.events
        .insert("on_change".into(), "ComboBox1Change".into());
    doc.add_child("Form1", cbo).unwrap();

    let files = GeradorMakepad
        .gerar(&doc, "app_cbo", &crawrad_registry::Registry::builtin())
        .expect("generates successfully");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap();

    assert!(
        designer_rs
            .conteudo
            .contains("combo_box.set_editable(cx, self.form.combo_box1.style == \"DropDown\");")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("combo_box.set_text(cx, &self.form.combo_box1.text);")
    );
    let text_arm = designer_rs
        .conteudo
        .split("text_changed(actions)")
        .nth(1)
        .expect("a text_changed arm");
    assert!(text_arm.contains("self.form.combo_box1.text = _val.clone();"));
    assert!(text_arm.contains("self.form.ComboBox1Change();"));
}

#[test]
fn price_edit_inheriting_spin_edit_generates_makepad() {
    let mut reg = crawrad_registry::Registry::builtin();
    reg.register_external(crawrad_registry::sample_price_edit())
        .expect("PriceEdit registers");

    let mut doc_price = Document::new("Form1", "Form");
    {
        let form = doc_price.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
    }
    let mut ped1 = reg
        .instantiate(
            "PriceEdit",
            "ped1".to_string(),
            Rect::new(10.0, 20.0, 120.0, 24.0),
        )
        .expect("instantiate PriceEdit");
    ped1.set_name("ped1");
    ped1.events
        .insert("on_change".to_string(), "ped1_change".to_string());
    doc_price.add_child("Form1", ped1).unwrap();

    let files_price = GeradorMakepad
        .gerar(&doc_price, "app_price", &reg)
        .expect("generates PriceEdit app");
    let designer_price = files_price
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap();

    // 1. The node is generated as its base SpinEdit widget
    assert!(
        designer_price.conteudo.contains("ped1 = <View> {"),
        "missing ped1 View DSL in:\n{}",
        designer_price.conteudo
    );
    assert!(
        designer_price
            .conteudo
            .contains("ped1_edit = <TextInput> {"),
        "missing ped1_edit TextInput DSL in:\n{}",
        designer_price.conteudo
    );
    assert!(
        designer_price.conteudo.contains("ped1_steps = <View> {"),
        "missing ped1_steps View DSL in:\n{}",
        designer_price.conteudo
    );
    assert!(
        designer_price.conteudo.contains("ped1_up = <Button> {"),
        "missing ped1_up Button DSL in:\n{}",
        designer_price.conteudo
    );
    assert!(
        designer_price.conteudo.contains("ped1_down = <Button> {"),
        "missing ped1_down Button DSL in:\n{}",
        designer_price.conteudo
    );
    // Base widget field on form struct
    assert!(
        designer_price.conteudo.contains("pub ped1: SpinEdit,"),
        "missing pub ped1: SpinEdit in:\n{}",
        designer_price.conteudo
    );
    // 2. Component's own part
    assert!(
        designer_price
            .conteudo
            .contains("pub ped1_ext: finance_crawrad::PriceEdit,"),
        "missing pub ped1_ext in:\n{}",
        designer_price.conteudo
    );
    assert!(
        designer_price.conteudo.contains(
            "ped1_ext: finance_crawrad::PriceEdit { currency: \"USD\".to_string(), decimals: 2 },"
        ),
        "missing ped1_ext initializer in:\n{}",
        designer_price.conteudo
    );
    // 3. Behaviour hooks & base events (on_change)
    assert!(
        designer_price
            .conteudo
            .contains("use crawrad_runtime::SpinEditHooks;"),
        "missing SpinEditHooks import in:\n{}",
        designer_price.conteudo
    );
    assert!(
        designer_price
            .conteudo
            .contains("self.form.ped1_ext.display(self.form.ped1.value)"),
        "missing ped1_ext.display call in:\n{}",
        designer_price.conteudo
    );
    assert!(
        designer_price
            .conteudo
            .contains("self.form.ped1_ext.parse(&typed)"),
        "missing ped1_ext.parse call in:\n{}",
        designer_price.conteudo
    );
    let changed_pos = designer_price
        .conteudo
        .find("self.form.ped1_ext.changed(self.form.ped1.value);")
        .expect("missing ped1_ext.changed call");
    let handler_pos = designer_price
        .conteudo
        .find("self.form.ped1_change();")
        .expect("missing ped1_change call");
    assert!(
        changed_pos < handler_pos,
        "changed hook must run before user on_change handler"
    );
}

#[test]
fn plain_spin_edit_generates_byte_identical_makepad() {
    let reg_builtin = crawrad_registry::Registry::builtin();
    let mut reg_with_price = crawrad_registry::Registry::builtin();
    reg_with_price
        .register_external(crawrad_registry::sample_price_edit())
        .expect("PriceEdit registers");

    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
    }
    let mut spn1 = reg_builtin
        .instantiate(
            "SpinEdit",
            "spn1".to_string(),
            Rect::new(10.0, 20.0, 120.0, 24.0),
        )
        .expect("instantiate SpinEdit");
    spn1.set_name("spn1");
    spn1.events
        .insert("on_change".to_string(), "spn1_change".to_string());
    doc.add_child("Form1", spn1).unwrap();

    let files_a = GeradorMakepad
        .gerar(&doc, "app_spin", &reg_builtin)
        .unwrap();
    let files_b = GeradorMakepad
        .gerar(&doc, "app_spin", &reg_with_price)
        .unwrap();

    assert_eq!(files_a.len(), files_b.len());
    for (fa, fb) in files_a.iter().zip(files_b.iter()) {
        assert_eq!(fa.caminho, fb.caminho);
        assert_eq!(fa.conteudo, fb.conteudo);
    }

    let designer_spin = files_a
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap();
    assert!(!designer_spin.conteudo.contains("spn1_ext"));
    assert!(!designer_spin.conteudo.contains(".display("));
    assert!(!designer_spin.conteudo.contains("_ext.parse("));
    assert!(!designer_spin.conteudo.contains("_ext.changed("));
}

#[test]
fn external_event_with_signature_produces_typed_stub_makepad() {
    let mut reg = crawrad_registry::Registry::builtin();
    let mut price = crawrad_registry::sample_price_edit();
    price
        .events
        .push(crawrad_registry::external::ExternalEvent {
            name: "on_limit".to_string(),
            display: "OnLimit".to_string(),
            signature: "value: i64".to_string(),
        });
    reg.register_external(price).expect("PriceEdit registers");

    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
    }
    let mut ped1 = reg
        .instantiate(
            "PriceEdit",
            "ped1".to_string(),
            Rect::new(10.0, 20.0, 120.0, 24.0),
        )
        .unwrap();
    ped1.set_name("ped1");
    ped1.events
        .insert("on_limit".to_string(), "ped1_limit".to_string());
    doc.add_child("Form1", ped1).unwrap();

    let files = GeradorMakepad.gerar(&doc, "app_price", &reg).unwrap();
    let user_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.rs")
        .unwrap();
    assert!(
        user_file.conteudo.contains("value: i64"),
        "expected typed stub with `value: i64` in:\n{}",
        user_file.conteudo
    );
}
