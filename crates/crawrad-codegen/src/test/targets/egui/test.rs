use crate::Gerador;
use crate::targets::egui::GeradorRxp;
use crawrad_ir::{Document, Node, Rect};

#[test]
fn o_gerador_egui_emite_os_tres_arquivos() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
        form.set_text("Meu Form Egui");
    }

    let mut btn = Node::new("btn_ok", "Button");
    btn.set_name("btn_ok");
    btn.set_text("Confirmar");
    btn.events
        .insert("on_click".to_string(), "btn_ok_click".to_string());
    let _ = doc.add_child("Form1", btn);

    let gerador = GeradorRxp;
    let arquivos = gerador
        .gerar(&doc, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("should generate successfully");

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
    assert!(main_rs.conteudo.contains("eframe::run_ui_native"));
    assert!(main_rs.conteudo.contains("Form1"));

    let designer_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap();
    assert!(designer_rs.conteudo.contains("pub struct Form1"));
    assert!(designer_rs.conteudo.contains("btn_ok"));
    assert!(designer_rs.conteudo.contains("self.btn_ok_click();"));

    let user_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.rs")
        .unwrap();
    assert!(user_rs.conteudo.contains("impl Form1"));
    assert!(user_rs.conteudo.contains("pub fn btn_ok_click(&mut self)"));
}

#[test]
fn o_gerador_egui_emite_todos_os_componentes_e_itens() {
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
    b1.set_name("Button1");
    b1.set_text("Button1");
    b1.events.insert("on_click".into(), "Button1Click".into());
    doc.add_child("Form1", b1).unwrap();

    // ComboBox1
    let mut c1 = Node::new("c1", "ComboBox");
    c1.set_name("ComboBox1");
    c1.props.insert(
        "items".into(),
        Item::list_to(&[Item::new("Opcao 1"), Item::new("Opcao 2")]),
    );
    c1.events
        .insert("on_change".into(), "ComboBox1Change".into());
    doc.add_child("Form1", c1).unwrap();

    // CheckBox1
    let mut cb1 = Node::new("cb1", "CheckBox");
    cb1.set_name("CheckBox1");
    cb1.set_text("CheckBox1");
    doc.add_child("Form1", cb1).unwrap();

    // RadioButton1
    let mut rb1 = Node::new("rb1", "RadioButton");
    rb1.set_name("RadioButton1");
    rb1.set_text("RadioButton1");
    doc.add_child("Form1", rb1).unwrap();

    // Memo1
    let mut m1 = Node::new("m1", "Memo");
    m1.set_name("Memo1");
    m1.props.insert(
        "text".into(),
        Value::Str("Linha 1\nLinha 2\nLinha 3".into()),
    );
    m1.events.insert("on_change".into(), "Memo1Change".into());
    doc.add_child("Form1", m1).unwrap();

    // ListBox1
    let mut l1 = Node::new("l1", "ListBox");
    l1.set_name("ListBox1");
    l1.props.insert(
        "items".into(),
        Item::list_to(&[Item::new("Item1"), Item::new("Item2"), Item::new("Item3")]),
    );
    doc.add_child("Form1", l1).unwrap();

    // TreeView1
    let mut tv1 = Node::new("tv1", "TreeView");
    tv1.set_name("TreeView1");
    tv1.props.insert(
        "items".into(),
        Item::list_to(&[
            Item::with_children("Node1", vec![Item::new("SubNode1"), Item::new("SubNode2")]),
            Item::new("Item5"),
        ]),
    );
    doc.add_child("Form1", tv1).unwrap();

    // Edit1
    let mut e1 = Node::new("e1", "Edit");
    e1.set_name("Edit1");
    e1.set_text("Texto Inicial");
    doc.add_child("Form1", e1).unwrap();

    // Label1
    let mut lb1 = Node::new("lb1", "Label");
    lb1.set_name("Label1");
    lb1.set_text("Rotulo");
    doc.add_child("Form1", lb1).unwrap();

    // GroupBox1
    let mut gb1 = Node::new("gb1", "GroupBox");
    gb1.set_name("GroupBox1");
    gb1.set_text("Grupo");
    doc.add_child("Form1", gb1).unwrap();

    // Panel1
    let mut p1 = Node::new("p1", "Panel");
    p1.set_name("Panel1");
    doc.add_child("Form1", p1).unwrap();

    // ScrollBar1
    let mut sb1 = Node::new("sb1", "ScrollBar");
    sb1.set_name("ScrollBar1");
    doc.add_child("Form1", sb1).unwrap();

    // ScrollBox1
    let mut sbox1 = Node::new("sbox1", "ScrollBox");
    sbox1.set_name("ScrollBox1");
    doc.add_child("Form1", sbox1).unwrap();

    let gerador = GeradorRxp;
    let arquivos = gerador
        .gerar(
            &doc,
            "app_validacao",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("should generate successfully");

    let designer_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("Form1.designer.rs");

    // Validates that all fields are declared in the form struct
    assert!(designer_rs.conteudo.contains("pub Button1: Button,"));
    assert!(designer_rs.conteudo.contains("pub ComboBox1: ComboBox,"));
    assert!(designer_rs.conteudo.contains("pub CheckBox1: CheckBox,"));
    assert!(
        designer_rs
            .conteudo
            .contains("pub RadioButton1: RadioButton,")
    );
    assert!(designer_rs.conteudo.contains("pub Memo1: Memo,"));
    assert!(designer_rs.conteudo.contains("pub ListBox1: ListBox,"));
    assert!(designer_rs.conteudo.contains("pub TreeView1: TreeView,"));
    assert!(designer_rs.conteudo.contains("pub Edit1: Edit,"));
    assert!(designer_rs.conteudo.contains("pub Label1: Label,"));
    assert!(designer_rs.conteudo.contains("pub GroupBox1: GroupBox,"));
    assert!(designer_rs.conteudo.contains("pub Panel1: Panel,"));
    assert!(designer_rs.conteudo.contains("pub ScrollBar1: ScrollBar,"));
    assert!(designer_rs.conteudo.contains("pub ScrollBox1: ScrollBox,"));

    // Validates item constants
    assert!(designer_rs.conteudo.contains("COMBOBOX1_ITEMS"));
    assert!(designer_rs.conteudo.contains("\"Opcao 1\""));
    assert!(designer_rs.conteudo.contains("LISTBOX1_ITEMS"));
    assert!(designer_rs.conteudo.contains("\"Item1\""));
    assert!(!designer_rs.conteudo.contains("TREEVIEW1_ITEMS"));
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
    assert!(
        designer_rs
            .conteudo
            .contains("crawrad_runtime::tree::TreeNode::leaf(\"Item5\")")
    );

    // Validates event dispatch
    assert!(designer_rs.conteudo.contains("self.Button1Click();"));
    assert!(designer_rs.conteudo.contains("self.ComboBox1Change();"));
    assert!(designer_rs.conteudo.contains("self.Memo1Change();"));

    let user_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.rs")
        .expect("Form1.rs");
    assert!(user_rs.conteudo.contains("pub fn Button1Click(&mut self)"));
    assert!(
        user_rs
            .conteudo
            .contains("pub fn ComboBox1Change(&mut self)")
    );
    assert!(user_rs.conteudo.contains("pub fn Memo1Change(&mut self)"));
}

#[test]
fn o_formulario_egui_suporta_heranca_visual_com_composicao() {
    let mut doc = Document::new_inherited("FormEspecial", "Form", "form_cliente");
    {
        let form = doc.node_mut("FormEspecial").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        form.set_name("FormEspecial");
        form.set_text("Cliente Especial");
    }

    let mut btn = Node::new("btn_vip", "Button");
    btn.set_name("btn_vip");
    btn.set_text("VIP Action");
    doc.add_child("FormEspecial", btn).unwrap();

    let gerador = GeradorRxp;
    let arquivos = gerador
        .gerar(&doc, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("should generate successfully");

    let designer_rs = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormEspecial.designer.rs")
        .expect("designer");

    // 1. The form struct composes the base (HAS-A)
    assert!(
        designer_rs
            .conteudo
            .contains("use crate::form_cliente_designer::FormCliente;")
    );
    assert!(designer_rs.conteudo.contains("pub struct FormEspecial {"));
    assert!(designer_rs.conteudo.contains("pub base: FormCliente,"));
    assert!(designer_rs.conteudo.contains("pub btn_vip: Button,"));

    // 2. Default initializes the base
    assert!(
        designer_rs
            .conteudo
            .contains("base: FormCliente::default(),")
    );

    // 3. The show() method draws the base first
    assert!(designer_rs.conteudo.contains("self.base.show(ui);"));

    // 4. Implements Deref and DerefMut for ergonomic access to base members
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
}

/// The egui Image decodes its picture with what `eframe` already
/// brings, and honours Stretch / Proportional / Center at run time.
#[test]
fn the_egui_image_decodes_its_picture_with_eframe() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
    }
    let r = crawrad_registry::Registry::builtin();
    let mut img = r
        .instantiate("Image", "img_a", Rect::new(8.0, 8.0, 96.0, 96.0))
        .expect("the catalogue instantiates an Image");
    img.set_name("img_a");
    let _ = doc.add_child("Form1", img);
    let arquivos = GeradorRxp.gerar(&doc, "meu_app", &r).expect("generates");
    let designer = arquivos
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("the designer file");
    let code = &designer.conteudo;
    assert!(
        code.contains("eframe::icon_data::from_png_bytes(bytes)"),
        "{code}"
    );
    assert!(
        !code.contains("Image::from_bytes"),
        "no loader-dependent path left: {code}"
    );
    for field in ["stretch", "proportional", "center", "transparent"] {
        assert!(
            code.contains(&format!("self.img_a.{field}")),
            "`{field}` is read at run time"
        );
    }
}

/// The form's Color, WindowState and Position reach the egui project.
#[test]
fn the_egui_form_takes_its_color_state_and_position() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
        form.props.insert(
            "color".into(),
            crawrad_ir::Value::Color(crawrad_ir::Color::rgb(0x20, 0x30, 0x40)),
        );
        form.props.insert(
            "window_state".into(),
            crawrad_ir::Value::Enum("wsMaximized".into()),
        );
        form.props.insert(
            "position".into(),
            crawrad_ir::Value::Enum("poScreenCenter".into()),
        );
    }
    let files = GeradorRxp
        .gerar(&doc, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let file = |p: &str| {
        files
            .iter()
            .find(|a| a.caminho.to_str().unwrap() == p)
            .unwrap()
            .conteudo
            .clone()
    };
    let main = file("src/main.rs");
    assert!(main.contains(".with_maximized(true)"), "{main}");
    assert!(main.contains("centered: true,"), "{main}");
    let designer = file("src/Form1.designer.rs");
    assert!(
        designer.contains("rect_filled(ui.max_rect(), 0.0, egui::Color32::from_rgba_unmultiplied(32, 48, 64, 255))"),
        "{designer}"
    );
}

/// The egui ScrollBar is the bar the designer draws, from the shared
/// `scroll_bar` section, and never an `egui::Slider`. The orientation is the
/// box's shape inside that section, and a move of the thumb calls
/// `OnScroll` as well as `OnChange`.
#[test]
fn the_egui_scroll_bar_is_vertical_when_tall_and_fires_on_scroll() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
    }
    let mut tall = Node::new("sb_v", "ScrollBar");
    tall.set_name("sb_v");
    tall.set_rect(Rect::new(10.0, 10.0, 16.0, 120.0));
    tall.events.insert("on_change".into(), "sb_v_change".into());
    tall.events.insert("on_scroll".into(), "sb_v_scroll".into());
    doc.add_child("Form1", tall).unwrap();
    let mut wide = Node::new("sb_h", "ScrollBar");
    wide.set_name("sb_h");
    wide.set_rect(Rect::new(40.0, 10.0, 120.0, 16.0));
    doc.add_child("Form1", wide).unwrap();

    let files = GeradorRxp
        .gerar(&doc, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap()
        .conteudo
        .clone();
    let body = |name: &str| {
        let start = designer
            .find(&format!("// {name} — ScrollBar"))
            .expect("its body");
        designer[start..].split("\n\n").next().unwrap().to_string()
    };
    for name in ["sb_v", "sb_h"] {
        assert!(
            body(name).contains(
                "scroll_bar(
"
            ),
            "{}",
            body(name)
        );
        assert!(!body(name).contains("Slider"), "{}", body(name));
    }
    assert!(!designer.contains("egui::Slider"), "{designer}");
    assert!(designer.contains("pub fn scroll_bar("), "{designer}");
    assert!(
        body("sb_v").contains("self.sb_v_change();")
            && body("sb_v").contains("self.sb_v_scroll();"),
        "{}",
        body("sb_v")
    );
}

/// Egui main captures run_ui_native result instead of returning a raw Result under windows_subsystem.
#[test]
fn bit_btn_kind_maps_to_standard_caption_in_egui() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
    }
    let mut bbtn = Node::new("bbtn_ok", "BitBtn");
    bbtn.set_name("bbtn_ok");
    bbtn.props
        .insert("kind".into(), crawrad_ir::Value::Enum("bkOK".into()));
    doc.add_child("Form1", bbtn).unwrap();

    let files = GeradorRxp
        .gerar(&doc, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap()
        .conteudo
        .clone();

    assert!(designer.contains("text: \"OK\".to_string()"), "{designer}");
}

/// Edit component text input properties and events in egui.
#[test]
fn edit_component_properties_and_events_in_egui() {
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
    edt.events.insert("on_enter".into(), "edt_enter".into());
    edt.events.insert("on_exit".into(), "edt_exit".into());
    edt.events
        .insert("on_key_press".into(), "edt_key_press".into());
    doc.add_child("Form1", edt).unwrap();

    let files = GeradorRxp
        .gerar(&doc, "app_edit", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap()
        .conteudo
        .clone();

    assert!(
        designer.contains("pub password_char: String,"),
        "struct field password_char"
    );
    assert!(
        designer.contains("pub readonly: bool,"),
        "struct field readonly"
    );
    assert!(
        designer.contains("pub text_hint: String,"),
        "struct field text_hint"
    );
    assert!(
        designer.contains("pub numbers_only: bool,"),
        "struct field numbers_only"
    );
    assert!(
        designer.contains("pub max_length: i64,"),
        "struct field max_length"
    );
    assert!(
        designer.contains("pub alignment: String,"),
        "struct field alignment"
    );

    assert!(
        designer.contains("password_char: \"*\".to_string()"),
        "init password_char"
    );
    assert!(designer.contains("readonly: true"), "init readonly");
    assert!(
        designer.contains("text_hint: \"Enter password\".to_string()"),
        "init text_hint"
    );
    assert!(designer.contains("numbers_only: true"), "init numbers_only");
    assert!(designer.contains("max_length: 10"), "init max_length");
    assert!(
        designer.contains("alignment: \"taRightJustify\".to_string()"),
        "init alignment"
    );

    assert!(
        designer.contains(".password(!self.edt_input.password_char.is_empty())"),
        "password option"
    );
    assert!(
        designer.contains(".hint_text(&self.edt_input.text_hint)"),
        "hint_text option"
    );
    assert!(
        designer.contains(".char_limit(char_limit)"),
        "char_limit option"
    );
    assert!(
        designer.contains(".horizontal_align(halign)"),
        "horizontal_align option"
    );
    assert!(
        designer.contains(".interactive(!self.edt_input.readonly)"),
        "read-only: the field is not interactive"
    );
    assert!(
        designer.contains("if res.changed() && self.edt_input.numbers_only {"),
        "numbers filter"
    );
    assert!(
        designer.contains("self.edt_input.text.retain(|c| c.is_ascii_digit());"),
        "digits retain"
    );

    assert!(
        designer.contains("if res.changed() {\n                self.edt_change();"),
        "on_change dispatch"
    );
    // The focus pair goes through the router, which fires once per
    // change; the widget's own `lost_focus` comes on two passes running
    assert!(
        !designer.contains("if res.gained_focus()") && !designer.contains("if res.lost_focus() {"),
        "no focus dispatch of the Edit's own"
    );
    assert!(
        designer.contains("CrawradFocus::Enter") && designer.contains("self.edt_enter();"),
        "on_enter dispatch"
    );
    assert!(
        designer.contains("CrawradFocus::Exit") && designer.contains("self.edt_exit();"),
        "on_exit dispatch"
    );
    assert!(designer.contains("if res.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {\n                self.edt_return();"), "on_return dispatch");
    assert!(
        designer.contains("self.edt_key_press(ch);"),
        "on_key_press dispatch"
    );
}

/// Egui target generates OpenDialog and SaveDialog with execute() calling runtime helpers.
#[test]
fn egui_open_dialog_and_save_dialog_emit_execute_and_state() {
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
        crawrad_ir::Value::Str("Select Document".into()),
    );
    open_dlg.props.insert(
        "filter".into(),
        crawrad_ir::Value::Str("Text files (*.txt)|*.txt".into()),
    );
    open_dlg.props.insert(
        "initial_dir".into(),
        crawrad_ir::Value::Str("C:/docs".into()),
    );
    open_dlg
        .props
        .insert("default_ext".into(), crawrad_ir::Value::Str("txt".into()));
    doc.add_child("Form1", open_dlg).unwrap();

    let mut save_dlg = Node::new("dlg_save", "SaveDialog");
    save_dlg.set_name("dlg_save");
    save_dlg.props.insert(
        "title".into(),
        crawrad_ir::Value::Str("Save Document".into()),
    );
    save_dlg.props.insert(
        "filter".into(),
        crawrad_ir::Value::Str("All files (*.*)|*.*".into()),
    );
    save_dlg.props.insert(
        "initial_dir".into(),
        crawrad_ir::Value::Str("C:/out".into()),
    );
    save_dlg
        .props
        .insert("default_ext".into(), crawrad_ir::Value::Str("txt".into()));
    doc.add_child("Form1", save_dlg).unwrap();

    let files = GeradorRxp
        .gerar(&doc, "dialog_app", &crawrad_registry::Registry::builtin())
        .expect("generates successfully");

    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("Form1.designer.rs");

    // OpenDialog struct definition with files and execute() calling pick_open
    assert!(designer.conteudo.contains("pub struct OpenDialog {"));
    assert!(designer.conteudo.contains("pub files: Vec<String>,"));
    assert!(
        designer
            .conteudo
            .contains("pub fn execute(&mut self) -> bool")
    );
    assert!(
        designer
            .conteudo
            .contains("crawrad_runtime::file_dialog::pick_open(&req)")
    );

    // SaveDialog struct definition and execute() calling pick_save
    assert!(designer.conteudo.contains("pub struct SaveDialog {"));
    assert!(
        designer
            .conteudo
            .contains("crawrad_runtime::file_dialog::pick_save(&req)")
    );

    // Default initialization contains the props
    assert!(
        designer
            .conteudo
            .contains("title: \"Select Document\".to_string(),")
    );
    assert!(
        designer
            .conteudo
            .contains("initial_dir: \"C:/docs\".to_string(),")
    );
    assert!(
        designer
            .conteudo
            .contains("default_ext: \"txt\".to_string()")
    );
    assert!(designer.conteudo.contains("files: Vec::new()"));

    // Body must not contain pretended read statements
    assert!(!designer.conteudo.contains("let _ = (&self.dlg_open.title"));
    assert!(!designer.conteudo.contains("let _ = (&self.dlg_save.title"));
}

#[test]
fn egui_breadcrumb_emits_style_and_colors() {
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

    let files = GeradorRxp
        .gerar(&doc, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap()
        .conteudo
        .clone();

    assert!(designer.contains("style: \"bsChevron\".to_string()"));
    assert!(designer.contains("segment_color: Color { r: 51, g: 68, b: 85, a: 255 }"));
    assert!(designer.contains("current_color: Color { r: 170, g: 187, b: 204, a: 255 }"));
    assert!(designer.contains("paint_breadcrumb_chevron("));
}

/// The form strip is the emitter's declaration, not a class name in
/// `form.rs` — the MainMenu's bar goes out after the controls, a Button stays
/// a control.
#[test]
fn the_egui_form_strip_is_the_emitter_declaration() {
    use crate::targets::egui::componentes::form_strip;
    assert!(form_strip("MainMenu"));
    assert!(!form_strip("Button") && !form_strip("PopupMenu") && !form_strip("NoSuchClass"));

    let reg = crawrad_registry::Registry::builtin();
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
    }
    let mut menu = reg
        .instantiate("MainMenu", "menu1", Rect::new(0.0, 0.0, 28.0, 28.0))
        .expect("the menu");
    menu.set_name("menu1");
    doc.add_child("Form1", menu).unwrap();
    let mut item = reg
        .instantiate("MenuItem", "mi_file", Rect::new(0.0, 0.0, 28.0, 28.0))
        .expect("the item");
    item.set_name("mi_file");
    doc.add_child("menu1", item).unwrap();
    let mut button = Node::new("btn1", "Button");
    button.set_name("btn1");
    button.set_rect(Rect::new(10.0, 40.0, 80.0, 24.0));
    doc.add_child("Form1", button).unwrap();

    let files = GeradorRxp.gerar(&doc, "meu_app", &reg).expect("generates");
    let designer = &files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap()
        .conteudo;
    let bar = designer.find("egui::MenuBar::new()").expect("the bar");
    let btn = designer.find("// btn1").expect("the button");
    assert!(
        btn < bar,
        "the bar goes out after the controls:\n{designer}"
    );
}

#[test]
fn egui_state_struct_of_derives_custom_component_and_timer_valves() {
    use crate::targets::egui::state::struct_of;
    use crawrad_ir::Value;
    use crawrad_registry::{Category, ComponentDesc, PropDesc, PropKind, Props, Registry};

    let mut reg = Registry::builtin();
    let custom_desc = ComponentDesc {
        inherits: None,
        class: "CustomWidget",
        display: "CustomWidget",
        base_name: "custom_widget",
        palette: "Custom",
        prefix: "cw",
        default_size: (120.0, 30.0),
        props: vec![
            Props::name(),
            PropDesc::new_typed(
                "threshold",
                "Threshold",
                Value::Int(10),
                Category::Behavior,
                PropKind::Int { min: 0, max: 100 },
            ),
            PropDesc::new_typed(
                "active",
                "Active",
                Value::Bool(true),
                Category::Behavior,
                PropKind::Bool,
            ),
        ],
        events: Vec::new(),
        targets: crawrad_registry::UNIVERSAL_TARGETS,
        children: crawrad_registry::Children::NoChildren,
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
    reg.register(custom_desc);

    let desc = reg
        .get("CustomWidget")
        .expect("CustomWidget must be registered");
    let code = struct_of(desc);
    assert!(
        code.contains("pub threshold: i64,"),
        "expected pub threshold: i64, got:\n{code}"
    );
    assert!(
        code.contains("pub active: bool,"),
        "expected pub active: bool, got:\n{code}"
    );

    let timer_desc = reg.get("Timer").expect("Timer must be registered");
    let timer_code = struct_of(timer_desc);
    assert!(
        timer_code.contains("pub interval: u64,"),
        "Timer state must contain pub interval: u64, got:\n{timer_code}"
    );
    assert!(
        timer_code.contains("pub next_due: Option<f64>,"),
        "Timer state must contain pub next_due: Option<f64>, got:\n{timer_code}"
    );
    assert!(
        !timer_code.contains("interval_ms"),
        "Timer state must not contain interval_ms, got:\n{timer_code}"
    );

    // a field keeps its doc comment in the generated struct
    let edit_code = struct_of(reg.get("Edit").expect("Edit is built in"));
    assert!(
        edit_code.contains("    /// The case the typed text is held to (`CharCase`, #667).\n    pub char_case: String,"),
        "{edit_code}"
    );
}

#[test]
fn egui_state_init_of_derives_custom_component_and_timer() {
    use crate::targets::egui::state::init_of;
    use crawrad_ir::Value;
    use crawrad_registry::{Category, ComponentDesc, PropDesc, PropKind, Props, Registry};

    let mut reg = Registry::builtin();
    let custom_desc = ComponentDesc {
        inherits: None,
        class: "CustomWidget",
        display: "CustomWidget",
        base_name: "custom_widget",
        palette: "Custom",
        prefix: "cw",
        default_size: (120.0, 30.0),
        props: vec![
            Props::name(),
            PropDesc::new_typed(
                "threshold",
                "Threshold",
                Value::Int(10),
                Category::Behavior,
                PropKind::Int { min: 0, max: 100 },
            ),
            PropDesc::new_typed(
                "active",
                "Active",
                Value::Bool(false),
                Category::Behavior,
                PropKind::Bool,
            ),
        ],
        events: Vec::new(),
        targets: crawrad_registry::UNIVERSAL_TARGETS,
        children: crawrad_registry::Children::NoChildren,
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
    reg.register(custom_desc);

    let desc = reg
        .get("CustomWidget")
        .expect("CustomWidget must be registered");
    let mut custom_node = Node::new("cw1", "CustomWidget");
    custom_node
        .props
        .insert("threshold".to_string(), Value::Int(7));
    custom_node
        .props
        .insert("active".to_string(), Value::Bool(true));
    let doc = Document::new("Form1", "Form");

    let init = init_of(&custom_node, desc, &doc);
    assert_eq!(init, "CustomWidget { threshold: 7, active: true }");

    let timer_desc = reg.get("Timer").expect("Timer must be registered");
    let mut timer_node = Node::new("timer1", "Timer");
    timer_node
        .props
        .insert("interval".to_string(), Value::Int(500));
    let timer_init = init_of(&timer_node, timer_desc, &doc);
    assert!(
        timer_init.contains("interval: 500"),
        "expected interval: 500 in timer init, got:\n{timer_init}"
    );
    assert!(
        timer_init.contains("next_due: None"),
        "expected next_due: None in timer init, got:\n{timer_init}"
    );
}

#[test]
fn tree_view_items_are_runtime_nodes_in_egui() {
    use crawrad_ir::Item;

    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
    }

    let mut tv = Node::new("tv1", "TreeView");
    tv.set_name("tv1");
    tv.props.insert(
        "items".into(),
        Item::list_to(&[Item::with_children(
            "BranchNode",
            vec![Item::new("LeafNode")],
        )]),
    );
    doc.add_child("Form1", tv).unwrap();

    let files = GeradorRxp
        .gerar(&doc, "tree_app", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer file")
        .conteudo
        .as_str();

    assert!(
        designer.contains("pub items: Vec<crawrad_runtime::tree::TreeNode>,"),
        "TreeView struct must have items field in state:\n{designer}"
    );
    assert!(
        designer.contains("TreeNode::branch("),
        "initialiser must contain TreeNode::branch(:\n{designer}"
    );
    assert!(
        designer.contains("TreeNode::leaf("),
        "initialiser must contain TreeNode::leaf(:\n{designer}"
    );
    assert!(
        designer.contains("tree(ui, \"tv1\", r, &self.tv1.items,"),
        "tree(ui, must be called with &self.tv1.items:\n{designer}"
    );
    assert!(
        !designer.contains("_ITEMS: &[Item]"),
        "must not contain _ITEMS: &[Item]:\n{designer}"
    );
}

#[test]
fn tree_view_collapsed_initialiser_in_egui() {
    use crawrad_ir::Item;

    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
    }

    let mut branch = Item::with_children("Branch", vec![Item::new("Leaf")]);
    branch.expanded = false;

    let mut tv = Node::new("tv1", "TreeView");
    tv.set_name("tv1");
    tv.props.insert("items".into(), Item::list_to(&[branch]));
    doc.add_child("Form1", tv).unwrap();

    let files = GeradorRxp
        .gerar(&doc, "tree_app", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer file")
        .conteudo
        .as_str();

    assert!(
        designer.contains("collapsed: vec![vec![0]]"),
        "initialiser must initialise collapsed with path [0]:\n{designer}"
    );
}

#[test]
fn tree_view_with_linked_image_list_emits_symbol() {
    use crawrad_ir::Item;

    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
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

    let files = GeradorRxp
        .gerar(
            &doc,
            "tree_icon_app",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer file")
        .conteudo
        .as_str();

    assert!(
        designer.contains(".with_symbol(\"📄\")"),
        "expected with_symbol(\"📄\") in designer, got:\n{designer}"
    );

    // With images empty, emits no with_symbol
    let tv_no_images = doc.node_mut("tv1").unwrap();
    tv_no_images
        .props
        .insert("images".into(), crawrad_ir::Value::Ref("".into()));

    let files2 = GeradorRxp
        .gerar(
            &doc,
            "tree_no_icon_app",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates");
    let designer2 = files2
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer file")
        .conteudo
        .as_str();

    assert!(
        !designer2.contains(".with_symbol("),
        "expected NO with_symbol when images is empty, got:\n{designer2}"
    );
}

#[test]
fn unbound_grid_in_egui_uses_selectable_label_and_calls_handler() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut grid = Node::new("grid1", "Grid");
    grid.set_name("grid1");
    grid.set_rect(Rect::new(10.0, 10.0, 300.0, 200.0));
    grid.events
        .insert("on_select_cell".to_string(), "on_cell_selected".to_string());
    doc.add_child("Form1", grid).unwrap();

    let files = GeradorRxp
        .gerar(
            &doc,
            "grid_egui_app",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer file")
        .conteudo
        .as_str();

    assert!(designer.contains("ui.selectable_label("));
    assert!(designer.contains("cells: Default::default()"));
    assert!(designer.contains("self.on_cell_selected(col, row);"));
    assert!(!designer.contains("R{}C{}"));
    assert!(designer.contains("let _ = clicked;"));
    assert!(designer.contains("let _ = double;"));
}

#[test]
fn unbound_grid_in_egui_initializes_cells_from_cell_text() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut grid = Node::new("grid1", "Grid");
    grid.set_name("grid1");
    grid.set_rect(Rect::new(10.0, 10.0, 300.0, 200.0));
    grid.props.insert(
        "cell_text".to_string(),
        crawrad_ir::Value::Str("Name\tAge\nAna\t30".to_string()),
    );
    doc.add_child("Form1", grid).unwrap();

    let files = GeradorRxp
        .gerar(
            &doc,
            "grid_egui_app",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer file")
        .conteudo
        .as_str();

    assert!(designer.contains("Cells::from_text(\"Name\\tAge\\nAna\\t30\")"));
}

#[test]
fn unbound_grid_in_egui_handles_click_and_double_click() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut grid = Node::new("grid1", "Grid");
    grid.set_name("grid1");
    grid.set_rect(Rect::new(10.0, 10.0, 300.0, 200.0));
    grid.events
        .insert("on_click".to_string(), "grid1_click".to_string());
    grid.events
        .insert("on_dbl_click".to_string(), "grid1_dbl_click".to_string());
    doc.add_child("Form1", grid).unwrap();

    let files = GeradorRxp
        .gerar(
            &doc,
            "grid_egui_app",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer file")
        .conteudo
        .as_str();

    assert!(designer.contains("if clicked {"));
    assert!(designer.contains("self.grid1_click();"));
    assert!(designer.contains("if double {"));
    assert!(designer.contains("self.grid1_dbl_click();"));
    // fixed and selectable cells both feed the two flags
    assert_eq!(
        designer.matches("clicked |= response.clicked();").count(),
        2
    );
    assert_eq!(
        designer
            .matches("double |= response.double_clicked();")
            .count(),
        2
    );
}

#[test]
fn unbound_grid_in_egui_supports_cell_editing() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut grid = Node::new("grid1", "Grid");
    grid.set_name("grid1");
    grid.set_rect(Rect::new(10.0, 10.0, 300.0, 200.0));
    doc.add_child("Form1", grid).unwrap();

    let files = GeradorRxp
        .gerar(
            &doc,
            "grid_egui_app",
            &crawrad_registry::Registry::builtin(),
        )
        .expect("generates");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer file")
        .conteudo
        .as_str();

    assert!(designer.contains("egui::TextEdit::singleline"));
    // only a grid with Editing on opens the editor on a double click
    let lines: Vec<&str> = designer.lines().map(str::trim).collect();
    assert!(lines.windows(2).any(|pair| pair
        == [
            "if self.grid1.editing && response.double_clicked() {",
            "self.grid1.editor = Some(self.grid1.cells.get(col, row).to_string());",
        ]));
    assert!(designer.contains("editor: None"));
    // leaving the editor commits, unless it was left with Escape
    assert!(designer.contains("if !ui.input(|i| i.key_pressed(egui::Key::Escape)) {"));
    assert!(designer.contains("self.grid1.cells.set(col, row, buffer);"));
}

#[test]
fn combo_box_emits_sorted_check_before_drawing() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut cb = Node::new("cb1", "ComboBox");
    cb.set_name("combo_box1");
    cb.set_rect(Rect::new(10.0, 10.0, 150.0, 24.0));
    doc.add_child("Form1", cb).unwrap();

    let files = GeradorRxp
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

    let expected_sort = "        if self.combo_box1.sorted {\n            crawrad_runtime::list_selection::sort(&mut self.combo_box1.items, &mut self.combo_box1.selected, &mut []);\n        }";
    assert!(
        designer_rs.contains(expected_sort),
        "expected sorted block in designer_rs, got:\n{designer_rs}"
    );
}

/// The egui target honours `ScrollBox.HorzScrollBar` / `VertScrollBar`.
#[test]
fn egui_scroll_box_honours_horz_and_vert_scroll_bar() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut sb = Node::new("sb1", "ScrollBox");
    sb.set_name("scroll_box1");
    sb.set_rect(Rect::new(10.0, 10.0, 200.0, 150.0));
    sb.props
        .insert("horz_scroll_bar".into(), crawrad_ir::Value::Bool(false));
    doc.add_child("Form1", sb).unwrap();

    let files = GeradorRxp
        .gerar(&doc, "app_sb", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer")
        .conteudo
        .as_str();

    assert!(
        designer_rs.contains("horz_scroll_bar: false"),
        "expected horz_scroll_bar: false in state literal, got:\n{designer_rs}"
    );
    assert!(
        designer_rs.contains(
            "egui::ScrollArea::new([self.scroll_box1.horz_scroll_bar, self.scroll_box1.vert_scroll_bar])"
        ),
        "expected egui::ScrollArea::new([self.scroll_box1.horz_scroll_bar, self.scroll_box1.vert_scroll_bar]) in body, got:\n{designer_rs}"
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

    let files = GeradorRxp
        .gerar(&doc, "app_hint", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer")
        .conteudo
        .as_str();

    assert!(designer_rs.contains("const CRAWRAD_FORM_SHOW_HINT: bool = false;"));
    assert!(designer_rs.contains("if CRAWRAD_FORM_SHOW_HINT && self.btn1.visible && self.btn1.show_hint && !self.btn1.hint.is_empty()"));

    // without the root's show_hint set, generates = true;
    doc.node_mut("Form1").unwrap().props.remove("show_hint");
    let files_true = GeradorRxp
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
    assert!(designer_true.contains("if CRAWRAD_FORM_SHOW_HINT && self.btn1.visible && self.btn1.show_hint && !self.btn1.hint.is_empty()"));
}

/// A Panel with an Edit inside generates a Tab stop line for the Edit
/// and none for the Panel itself.
#[test]
fn egui_panel_with_edit_generates_tab_stop_for_edit_and_none_for_panel() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let reg = crawrad_registry::Registry::builtin();
    let mut panel = reg
        .instantiate("Panel", "panel1", Rect::new(10.0, 10.0, 200.0, 100.0))
        .expect("Panel");
    panel.set_name("panel1");
    doc.add_child("Form1", panel).unwrap();

    let mut edit = reg
        .instantiate("Edit", "edit1", Rect::new(10.0, 10.0, 100.0, 24.0))
        .expect("Edit");
    edit.set_name("edit1");
    doc.add_child("panel1", edit).unwrap();

    let files = GeradorRxp
        .gerar(&doc, "app_panel_tab", &reg)
        .expect("generates");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer")
        .conteudo
        .as_str();

    let edit_stop = "        if self.edit1.visible && self.edit1.enabled {\n            crawrad_tab.push((&[0, 0], r, false, \"edit1\"));\n        }";
    assert!(
        designer_rs.contains(edit_stop),
        "expected tab stop line for edit1 in designer_rs, got:\n{designer_rs}"
    );

    let panel_stop_prefix =
        "if self.panel1.visible && self.panel1.enabled {\n            crawrad_tab.push(";
    assert!(
        !designer_rs.contains(panel_stop_prefix),
        "panel1 itself must not generate a tab stop line in designer_rs, got:\n{designer_rs}"
    );
}

/// A TabSheet's Hint is read in the PageControl's hints array.
#[test]
fn tab_sheet_hint_emitted_in_egui_page_control() {
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

    let files = GeradorRxp
        .gerar(&doc, "app_tab_hint", &crawrad_registry::Registry::builtin())
        .expect("generates");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .expect("designer file")
        .conteudo
        .as_str();

    assert!(
        designer.contains("pub hint: String,"),
        "TabSheet struct must have hint field in state:\n{designer}"
    );
    assert!(
        designer.contains("pub show_hint: bool,"),
        "TabSheet struct must have show_hint field in state:\n{designer}"
    );
    assert!(
        designer.contains("let captions: [(&str, &str); 2] = [(self.tab1.text.as_str(), if CRAWRAD_FORM_SHOW_HINT && self.tab1.show_hint { self.tab1.hint.as_str() } else { \"\" }), (self.tab2.text.as_str(), if CRAWRAD_FORM_SHOW_HINT && self.tab2.show_hint { self.tab2.hint.as_str() } else { \"\" })];"),
        "expected (caption, hint) pairs in designer, got:\n{designer}"
    );
    assert!(
        designer.contains(
            "page_control(ui, \"page_control1\", r, &captions, &shown, bottom, &mut page)"
        ),
        "page_control call changed, got:\n{designer}"
    );
}

#[test]
fn egui_main_rs_honours_form_theme() {
    let mut doc_dark = Document::new("Form1", "Form");
    doc_dark
        .node_mut("Form1")
        .unwrap()
        .props
        .insert("theme".into(), crawrad_ir::Value::Enum("thDark".into()));
    let generator = GeradorRxp;
    let files_dark = generator
        .gerar(&doc_dark, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("should generate successfully");
    let main_rs_dark = files_dark
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/main.rs")
        .unwrap();
    assert!(
        main_rs_dark
            .conteudo
            .contains("ui.ctx().set_theme(eframe::egui::ThemePreference::Dark);")
    );
    assert_eq!(
        main_rs_dark
            .conteudo
            .matches("ThemePreference::Dark")
            .count(),
        1
    );

    // thSystem emits no set_theme
    let mut doc_sys = Document::new("Form1", "Form");
    doc_sys
        .node_mut("Form1")
        .unwrap()
        .props
        .insert("theme".into(), crawrad_ir::Value::Enum("thSystem".into()));
    let files_sys = generator
        .gerar(&doc_sys, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("should generate successfully");
    let main_rs_sys = files_sys
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/main.rs")
        .unwrap();
    assert!(!main_rs_sys.conteudo.contains("set_theme"));
}

#[test]
fn egui_inherited_event_not_overridden_calls_base_dispatch() {
    let reg = crawrad_registry::Registry::builtin();
    let mut base_doc = Document::new("BaseForm", "Form");
    let mut btn = Node::new("btn_save", "Button");
    btn.set_name("btn_save");
    btn.events
        .insert("on_click".into(), "btn_save_click".into());
    base_doc.add_child("BaseForm", btn).unwrap();

    let mut derived_doc = Document::new("DerivedForm", "Form");
    derived_doc.set_inherits(Some("BaseForm"));

    let files = GeradorRxp
        .generate_with_base(&derived_doc, "app", &reg, Some(&base_doc), false)
        .expect("generates successfully");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/DerivedForm.designer.rs")
        .unwrap();

    assert!(designer.conteudo.contains("self.base.dispatch(fired)"));
}

#[test]
fn egui_overridden_inherited_event_calls_derived_handler_and_node_not_emitted() {
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

    let files = GeradorRxp
        .generate_with_base(&derived_doc, "app", &reg, Some(&base_doc), false)
        .expect("generates successfully");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/DerivedForm.designer.rs")
        .unwrap();

    assert!(designer.conteudo.contains("self.btn_derived_click();"));
    assert!(!designer.conteudo.contains("pub btn_ok:"));
    assert!(!designer.conteudo.contains("btn_ok: Button::default()"));
}

#[test]
fn egui_overridden_event_stub_body_calls_base_handler() {
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

    let files = GeradorRxp
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
fn egui_base_form_reports_and_dispatches() {
    let reg = crawrad_registry::Registry::builtin();
    let mut base_doc = Document::new("FormBase", "Form");
    let mut btn = Node::new("btn_ok", "Button");
    btn.set_name("btn_ok");
    btn.events.insert("on_click".into(), "btn_ok_click".into());
    base_doc.add_child("FormBase", btn).unwrap();

    let files = GeradorRxp
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
            .contains("pub fn show_reporting(&mut self, ui: &mut egui::Ui) -> Vec<FormBaseFired>")
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
    assert!(designer.conteudo.contains("ui.ctx().request_repaint();"));
}

#[test]
fn egui_base_form_with_is_base_false_has_no_reporting() {
    let reg = crawrad_registry::Registry::builtin();
    let mut base_doc = Document::new("FormBase", "Form");
    let mut btn = Node::new("btn_ok", "Button");
    btn.set_name("btn_ok");
    btn.events.insert("on_click".into(), "btn_ok_click".into());
    base_doc.add_child("FormBase", btn).unwrap();

    let files = GeradorRxp
        .generate_with_base(&base_doc, "app", &reg, None, false)
        .expect("generates successfully");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormBase.designer.rs")
        .unwrap();

    assert!(!designer.conteudo.contains("FormBaseFired"));
    assert!(!designer.conteudo.contains("show_reporting"));
    assert!(!designer.conteudo.contains("pub fn dispatch"));
    assert!(designer.conteudo.contains("self.btn_ok_click();"));
}

#[test]
fn egui_mut_arg_or_if_call_is_not_rewritten() {
    let reg = crawrad_registry::Registry::builtin();
    let mut base_doc = Document::new("FormBase", "Form");
    let mut tv = Node::new("tv1", "TreeView");
    tv.set_name("tv1");
    tv.events.insert("on_changing".into(), "tv_changing".into());
    base_doc.add_child("FormBase", tv).unwrap();

    let files = GeradorRxp
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
            .contains("self.tv_changing(&mut allow_change);")
    );
}

#[test]
fn egui_derived_override_and_non_override() {
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

    let files = GeradorRxp
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
    assert!(designer.conteudo.contains("self.btn_derived_click();"));
    assert!(designer.conteudo.contains("fired => {"));
    assert!(designer.conteudo.contains("self.base.dispatch(fired);"));
    assert!(designer.conteudo.contains("ui.ctx().request_repaint();"));
}

#[test]
fn egui_form_with_no_wired_events_generates_no_fired_enum_and_output_unchanged() {
    let reg = crawrad_registry::Registry::builtin();
    let mut doc = Document::new("FormSimple", "Form");
    let mut btn = Node::new("btn_simple", "Button");
    btn.set_name("btn_simple");
    doc.add_child("FormSimple", btn).unwrap();

    let files = GeradorRxp
        .gerar(&doc, "app", &reg)
        .expect("generates successfully");
    let designer = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormSimple.designer.rs")
        .unwrap();

    assert!(!designer.conteudo.contains("FormSimpleFired"));
    assert!(!designer.conteudo.contains("show_reporting"));
    assert!(!designer.conteudo.contains("pub fn dispatch"));
}

#[test]
fn egui_reported_events_returns_only_rewritten_events() {
    let reg = crawrad_registry::Registry::builtin();
    let mut base_doc = Document::new("FormBase", "Form");
    let mut btn = Node::new("btn_ok", "Button");
    btn.set_name("btn_ok");
    btn.events.insert("on_click".into(), "btn_ok_click".into());
    base_doc.add_child("FormBase", btn).unwrap();

    let evs = crate::targets::egui::form::reported_events(&reg, &base_doc, &[]).unwrap();
    assert_eq!(evs.len(), 1);
    assert_eq!(evs[0].variant, "BtnOkClick");
}

#[test]
fn egui_combobox_emits_height_for_drop_down_count() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut cbo = Node::new("cbo1", "ComboBox");
    cbo.set_name("ComboBox1");
    cbo.props
        .insert("drop_down_count".into(), crawrad_ir::Value::Int(8));
    doc.add_child("Form1", cbo).unwrap();

    let files = GeradorRxp
        .gerar(&doc, "app_cbo", &crawrad_registry::Registry::builtin())
        .expect("generates successfully");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap();

    assert!(designer_rs.conteudo.contains(".height(drop_h)"));
    assert!(
        designer_rs
            .conteudo
            .contains("self.ComboBox1.drop_down_count == 0")
    );
    assert!(designer_rs.conteudo.contains("f32::INFINITY"));
}

#[test]
fn egui_combobox_emits_dropdown_style_and_text_edit() {
    let mut doc = Document::new("Form1", "Form");
    doc.node_mut("Form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));

    let mut cbo = Node::new("cbo1", "ComboBox");
    cbo.set_name("ComboBox1");
    cbo.props
        .insert("style".into(), crawrad_ir::Value::Enum("DropDown".into()));
    cbo.props
        .insert("text".into(), crawrad_ir::Value::Str("Sample".into()));
    doc.add_child("Form1", cbo).unwrap();

    let files = GeradorRxp
        .gerar(&doc, "app_cbo", &crawrad_registry::Registry::builtin())
        .expect("generates successfully");
    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap();

    assert!(
        designer_rs
            .conteudo
            .contains("self.ComboBox1.style == \"DropDown\"")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("egui::TextEdit::singleline(&mut self.ComboBox1.text)")
    );
    assert!(
        designer_rs
            .conteudo
            .contains("egui::Popup::from_response(&text_res)")
    );
}

#[test]
fn the_window_icon_goes_into_the_egui_viewport_once() {
    let main =
        crate::targets::egui::main_rs("form_main", "FormMain", &Node::new("form_main", "Form"));
    assert!(
        !main.contains(".with_icon("),
        "no icon unless the project has one"
    );
    let with = crate::targets::egui::with_window_icon(&main);
    assert!(with.contains("include_bytes!(\"../icon.png\")"), "{with}");
    assert_eq!(
        crate::targets::egui::with_window_icon(&with),
        with,
        "applied once"
    );
    let makepad = "fn main() { app_main() }";
    assert_eq!(crate::targets::egui::with_window_icon(makepad), makepad);
}

#[test]
fn apply_language_emits_translatable_properties_when_flag_on() {
    let mut doc = Document::new("form_main", "Form");
    {
        let form = doc.node_mut("form_main").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("form_main");
        form.props
            .insert("text".into(), crawrad_ir::Value::Str("Window Title".into()));
    }

    let mut btn = Node::new("btn_ok", "Button");
    btn.set_name("btn_ok");
    btn.props
        .insert("text".into(), crawrad_ir::Value::Str("&OK".into()));
    doc.add_child("form_main", btn).unwrap();

    let mut lbl = Node::new("lbl_help", "Label");
    lbl.set_name("lbl_help");
    lbl.props
        .insert("hint".into(), crawrad_ir::Value::Str("Help hint".into()));
    doc.add_child("form_main", lbl).unwrap();

    let mut empty_btn = Node::new("btn_empty", "Button");
    empty_btn.set_name("btn_empty");
    empty_btn
        .props
        .insert("text".into(), crawrad_ir::Value::Str("".into()));
    doc.add_child("form_main", empty_btn).unwrap();

    let reg = crawrad_registry::Registry::builtin();

    // 1. Flag off: no apply_language emitted
    crate::targets::egui::set_has_locales(false);
    let files_off = GeradorRxp.gerar(&doc, "my_app", &reg).unwrap();
    let designer_off = files_off
        .iter()
        .find(|f| f.caminho.to_str().unwrap() == "src/form_main.designer.rs")
        .unwrap();
    assert!(!designer_off.conteudo.contains("apply_language"));
    assert!(!designer_off.conteudo.contains("crate::locales"));

    // 2. Flag on: apply_language emitted with expected lines
    crate::targets::egui::set_has_locales(true);
    let files_on = GeradorRxp.gerar(&doc, "my_app", &reg).unwrap();
    crate::targets::egui::set_has_locales(false);

    let designer_on = files_on
        .iter()
        .find(|f| f.caminho.to_str().unwrap() == "src/form_main.designer.rs")
        .unwrap();
    assert!(
        designer_on
            .conteudo
            .contains("pub fn apply_language(&mut self, lang: &str)")
    );
    assert!(
        designer_on
            .conteudo
            .contains("pub fn set_language(&mut self, lang: &str)")
    );
    assert!(
        designer_on
            .conteudo
            .contains(r#"self.btn_ok.text = crate::locales::text(lang, "form_main.btn_ok.text").unwrap_or("&OK").to_string();"#)
    );
    assert!(
        designer_on
            .conteudo
            .contains(r#"self.lbl_help.hint = crate::locales::text(lang, "form_main.lbl_help.hint").unwrap_or("Help hint").to_string();"#)
    );
    // Empty text was skipped: only the body of apply_language is checked, the drawing code
    // names every field.
    let body = designer_on
        .conteudo
        .split("pub fn apply_language(&mut self, lang: &str)")
        .nth(1)
        .and_then(|rest| {
            rest.split(
                "
    }
",
            )
            .next()
        })
        .expect("apply_language body");
    assert!(!body.contains("self.btn_empty.text"));

    let main_on = files_on
        .iter()
        .find(|f| f.caminho.to_str().unwrap() == "src/main.rs")
        .unwrap();
    assert!(main_on.conteudo.contains("mod locales;"));
    assert!(
        main_on
            .conteudo
            .contains("let language = crate::locales::language();")
    );
    assert!(main_on.conteudo.contains("form.apply_language(&language);"));
    // The window opens titled in the chosen language, English when it has no translation.
    assert!(main_on.conteudo.contains(
        r#".with_title(crate::locales::text(&language, "form_main.form_main.text").unwrap_or("#
    ));
}

#[test]
fn price_edit_inheriting_spin_edit_generates_egui() {
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

    let files_price = GeradorRxp
        .gerar(&doc_price, "app_price", &reg)
        .expect("generates PriceEdit app");
    let designer_price = files_price
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap();

    // 1. Base SpinEdit widget code
    assert!(
        designer_price.conteudo.contains("if self.ped1.visible {"),
        "missing ped1 visible check in:\n{}",
        designer_price.conteudo
    );
    assert!(
        designer_price
            .conteudo
            .contains("egui::TextEdit::singleline(&mut text)"),
        "missing ped1 TextEdit in:\n{}",
        designer_price.conteudo
    );
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
            .contains("self.ped1_ext.display(self.ped1.value)"),
        "missing self.ped1_ext.display call in:\n{}",
        designer_price.conteudo
    );
    assert!(
        designer_price
            .conteudo
            .contains("self.ped1_ext.parse(&text)"),
        "missing self.ped1_ext.parse call in:\n{}",
        designer_price.conteudo
    );
    let changed_pos = designer_price
        .conteudo
        .find("self.ped1_ext.changed(self.ped1.value);")
        .expect("missing self.ped1_ext.changed call");
    let handler_pos = designer_price
        .conteudo
        .find("self.ped1_change();")
        .expect("missing ped1_change call");
    assert!(
        changed_pos < handler_pos,
        "changed hook must run before user on_change handler"
    );
}

#[test]
fn plain_spin_edit_generates_byte_identical_egui() {
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

    let files_a = GeradorRxp.gerar(&doc, "app_spin", &reg_builtin).unwrap();
    let files_b = GeradorRxp.gerar(&doc, "app_spin", &reg_with_price).unwrap();

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
    assert!(!designer_spin.conteudo.contains(".parse("));
    assert!(!designer_spin.conteudo.contains("_ext.changed("));
}

#[test]
fn external_event_with_signature_produces_typed_stub_egui() {
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

    let files = GeradorRxp.gerar(&doc, "app_price", &reg).unwrap();
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
