use crate::Gerador;
use crate::targets::slint::GeradorSlint;
use crate::targets::slint::layout::{position, text};
use crate::targets::slint::main_app::main_rs;
use crawrad_ir::{Document, Node, Rect};

#[test]
fn slint_generator_emits_four_files() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
        form.set_text("My Slint Form");
    }

    let mut btn = Node::new("btn_ok", "Button");
    btn.set_name("btn_ok");
    btn.set_text("Confirm");
    btn.events
        .insert("on_click".to_string(), "btn_ok_click".to_string());
    let _ = doc.add_child("Form1", btn);

    let generator = GeradorSlint;
    let files = generator
        .gerar(&doc, "my_app", &crawrad_registry::Registry::builtin())
        .expect("must generate successfully");

    assert_eq!(files.len(), 4);
    let paths: Vec<_> = files.iter().map(|a| a.caminho.to_str().unwrap()).collect();
    assert!(paths.contains(&"src/main.rs"));
    assert!(paths.contains(&"src/Form1.designer.rs"));
    assert!(paths.contains(&"src/Form1.slint"));
    assert!(paths.contains(&"src/Form1.rs"));

    let main_rs_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/main.rs")
        .unwrap();
    assert!(
        main_rs_file
            .conteudo
            .contains("Form1_designer::app_main();")
    );

    let slint_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.slint")
        .unwrap();
    assert!(
        slint_file
            .conteudo
            .contains("export component Form1Window inherits Window")
    );
    assert!(slint_file.conteudo.contains("title: \"My Slint Form\";"));
    assert!(slint_file.conteudo.contains("width: 640.0px;"));
    assert!(slint_file.conteudo.contains("height: 480.0px;"));
    assert!(slint_file.conteudo.contains("btn_ok := Button"));
    // THE CAPTION AND THE WIRING, which the audit found unasserted on 27/11/2026:
    // the generator emitted both, and changing `text:` to a typo or `clicked` to
    // `pressed` left every test passing. A button with no label and no reaction
    // would have shipped.
    assert!(
        slint_file.conteudo.contains("text: \"Confirm\";"),
        "the button lost its caption"
    );
    assert!(
        slint_file
            .conteudo
            .contains("clicked => { root.btn_ok_clicked(); }"),
        "the button's click no longer reaches the callback it declares"
    );
    assert!(slint_file.conteudo.contains("callback btn_ok_clicked();"));

    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap();
    assert!(designer_rs.conteudo.contains("pub struct Form1"));
    assert!(designer_rs.conteudo.contains("slint::slint!"));
    assert!(designer_rs.conteudo.contains("pub fn app_main()"));
    assert!(designer_rs.conteudo.contains("window.on_btn_ok_clicked"));
    assert!(designer_rs.conteudo.contains("f.form.btn_ok_click();"));

    let user_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.rs")
        .unwrap();
    assert!(user_rs.conteudo.contains("impl Form1"));
    assert!(user_rs.conteudo.contains("pub fn btn_ok_click(&mut self)"));
}

#[test]
fn test_slint_main_rs_formatting() {
    let code = main_rs("FormMain", "FormMain");
    assert!(code.contains("mod FormMain;"));
    assert!(code.contains("FormMain_designer::app_main();"));
}

#[test]
fn test_slint_layout_position_and_text() {
    assert_eq!(position(10.0, 25.5), "x: 10.0px; y: 25.5px;");

    let mut node = Node::new("btn", "Button");
    node.set_text("&Save File");
    assert_eq!(text(&node), "Save File");
}

#[test]
fn test_slint_generator_emits_inputs_and_toggles() {
    let mut doc = Document::new("FormMain", "Form");
    {
        let form = doc.node_mut("FormMain").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        form.set_name("FormMain");
        form.set_text("User Settings");
    }

    let mut edit = Node::new("txt_username", "Edit");
    edit.set_name("txt_username");
    edit.set_text("admin");
    edit.props.insert(
        "text_hint".to_string(),
        crawrad_ir::Value::Str("Enter username".to_string()),
    );
    edit.events
        .insert("on_change".to_string(), "txt_username_change".to_string());
    edit.events
        .insert("on_return".to_string(), "txt_username_enter".to_string());
    let _ = doc.add_child("FormMain", edit);

    let mut memo = Node::new("mem_bio", "Memo");
    memo.set_name("mem_bio");
    memo.set_text("Line 1\nLine 2");
    memo.events
        .insert("on_change".to_string(), "mem_bio_change".to_string());
    let _ = doc.add_child("FormMain", memo);

    let mut chk = Node::new("chk_notify", "CheckBox");
    chk.set_name("chk_notify");
    chk.set_text("Receive Notifications");
    chk.props
        .insert("checked".to_string(), crawrad_ir::Value::Bool(true));
    chk.events
        .insert("on_click".to_string(), "chk_notify_click".to_string());
    let _ = doc.add_child("FormMain", chk);

    let mut tgl = Node::new("tgl_dark", "Toggle");
    tgl.set_name("tgl_dark");
    tgl.set_text("Dark Mode");
    tgl.events
        .insert("on_change".to_string(), "tgl_dark_change".to_string());
    let _ = doc.add_child("FormMain", tgl);

    let generator = GeradorSlint;
    let files = generator
        .gerar(&doc, "my_app", &crawrad_registry::Registry::builtin())
        .expect("must generate inputs successfully");

    let slint_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormMain.slint")
        .unwrap();
    assert!(slint_file.conteudo.contains("txt_username := LineEdit"));
    // The CONTENT, not only the hint: the audit on 27/11/2026 found the hint
    // asserted and the text itself not, so a field could come up empty where the
    // form says it is filled.
    assert!(
        slint_file.conteudo.contains("text: \"admin\";"),
        "the edit lost the text the form gave it"
    );
    assert!(
        slint_file
            .conteudo
            .contains("placeholder-text: \"Enter username\";")
    );
    assert!(
        slint_file
            .conteudo
            .contains("callback txt_username_edited(string);")
    );
    assert!(
        slint_file
            .conteudo
            .contains("callback txt_username_accepted();")
    );

    assert!(slint_file.conteudo.contains("mem_bio := TextEdit"));
    assert!(
        slint_file
            .conteudo
            .contains("callback mem_bio_edited(string);")
    );

    assert!(slint_file.conteudo.contains("chk_notify := CheckBox"));
    assert!(
        slint_file
            .conteudo
            .contains("text: \"Receive Notifications\";")
    );
    assert!(slint_file.conteudo.contains("checked: true;"));
    assert!(
        slint_file
            .conteudo
            .contains("callback chk_notify_toggled(bool);")
    );

    assert!(slint_file.conteudo.contains("tgl_dark := Switch"));
    assert!(slint_file.conteudo.contains("text: \"Dark Mode\";"));
    assert!(
        slint_file
            .conteudo
            .contains("callback tgl_dark_toggled(bool);")
    );

    let designer_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormMain.designer.rs")
        .unwrap();
    assert!(designer_file.conteudo.contains("pub txt_username: Edit,"));
    assert!(designer_file.conteudo.contains("pub mem_bio: Memo,"));
    assert!(designer_file.conteudo.contains("pub chk_notify: CheckBox,"));
    assert!(designer_file.conteudo.contains("pub tgl_dark: Toggle,"));
    assert!(designer_file.conteudo.contains("pub struct Edit"));
    assert!(designer_file.conteudo.contains("pub struct Memo"));
    assert!(designer_file.conteudo.contains("pub struct CheckBox"));
    assert!(designer_file.conteudo.contains("pub struct Toggle"));
    assert!(
        designer_file
            .conteudo
            .contains("window.on_txt_username_edited")
    );
    assert!(
        designer_file
            .conteudo
            .contains("window.on_txt_username_accepted")
    );
    assert!(
        !designer_file
            .conteudo
            .contains("window.on_mem_notes_edited")
            && designer_file.conteudo.contains("window.on_mem_bio_edited")
    );
    assert!(
        designer_file
            .conteudo
            .contains("window.on_chk_notify_toggled")
    );
    assert!(
        designer_file
            .conteudo
            .contains("window.on_tgl_dark_toggled")
    );
}

#[test]
fn test_slint_generator_emits_lists_and_numerical_controls() {
    let mut doc = Document::new("FormLists", "Form");
    {
        let form = doc.node_mut("FormLists").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        form.set_name("FormLists");
        form.set_text("Lists and Numerical");
    }

    let mut cbo = Node::new("cbo_theme", "ComboBox");
    cbo.set_name("cbo_theme");
    cbo.props.insert(
        "items".to_string(),
        crawrad_ir::Value::List(vec![
            crawrad_ir::Value::Str("Light".to_string()),
            crawrad_ir::Value::Str("Dark".to_string()),
            crawrad_ir::Value::Str("Auto".to_string()),
        ]),
    );
    cbo.props
        .insert("item_index".to_string(), crawrad_ir::Value::Int(1));
    cbo.events
        .insert("on_select".to_string(), "cbo_theme_select".to_string());
    let _ = doc.add_child("FormLists", cbo);

    let mut lst = Node::new("lst_items", "ListBox");
    lst.set_name("lst_items");
    lst.props.insert(
        "items".to_string(),
        crawrad_ir::Value::List(vec![
            crawrad_ir::Value::Str("First".to_string()),
            crawrad_ir::Value::Str("Second".to_string()),
        ]),
    );
    lst.props
        .insert("item_index".to_string(), crawrad_ir::Value::Int(0));
    lst.events
        .insert("on_click".to_string(), "lst_items_click".to_string());
    let _ = doc.add_child("FormLists", lst);

    let mut pb = Node::new("pb_upload", "ProgressBar");
    pb.set_name("pb_upload");
    pb.props
        .insert("min".to_string(), crawrad_ir::Value::Int(0));
    pb.props
        .insert("max".to_string(), crawrad_ir::Value::Int(100));
    pb.props
        .insert("position".to_string(), crawrad_ir::Value::Int(45));
    let _ = doc.add_child("FormLists", pb);

    let mut tb = Node::new("tb_volume", "TrackBar");
    tb.set_name("tb_volume");
    tb.props
        .insert("min".to_string(), crawrad_ir::Value::Int(0));
    tb.props
        .insert("max".to_string(), crawrad_ir::Value::Int(10));
    tb.props
        .insert("position".to_string(), crawrad_ir::Value::Int(7));
    tb.events
        .insert("on_change".to_string(), "tb_volume_change".to_string());
    let _ = doc.add_child("FormLists", tb);

    let mut spn = Node::new("spn_count", "SpinEdit");
    spn.set_name("spn_count");
    spn.props
        .insert("min_value".to_string(), crawrad_ir::Value::Int(1));
    spn.props
        .insert("max_value".to_string(), crawrad_ir::Value::Int(50));
    spn.props
        .insert("value".to_string(), crawrad_ir::Value::Int(12));
    let _ = doc.add_child("FormLists", spn);

    let generator = GeradorSlint;
    let files = generator
        .gerar(&doc, "my_app", &crawrad_registry::Registry::builtin())
        .expect("must generate lists successfully");

    let slint_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormLists.slint")
        .unwrap();
    assert!(slint_file.conteudo.contains("cbo_theme := ComboBox"));
    assert!(
        slint_file
            .conteudo
            .contains("model: [\"Light\", \"Dark\", \"Auto\"];")
    );
    assert!(slint_file.conteudo.contains("current-index: 1;"));
    assert!(
        slint_file
            .conteudo
            .contains("callback cbo_theme_selected(int, string);")
    );

    assert!(
        slint_file
            .conteudo
            .contains("lst_items := StandardListView")
    );
    assert!(slint_file.conteudo.contains("current-item: 0;"));
    assert!(
        slint_file
            .conteudo
            .contains("callback lst_items_current_item_changed(int);")
    );

    assert!(slint_file.conteudo.contains("pb_upload := Rectangle"));
    assert!(slint_file.conteudo.contains("background: #2a82da;"));

    assert!(slint_file.conteudo.contains("tb_volume := Slider"));
    assert!(slint_file.conteudo.contains("minimum: 0.0;"));
    assert!(slint_file.conteudo.contains("maximum: 10.0;"));
    assert!(slint_file.conteudo.contains("value: 7.0;"));
    assert!(
        slint_file
            .conteudo
            .contains("callback tb_volume_changed(float);")
    );

    assert!(slint_file.conteudo.contains("spn_count := SpinBox"));
    assert!(slint_file.conteudo.contains("minimum: 1;"));
    assert!(slint_file.conteudo.contains("maximum: 50;"));
    assert!(slint_file.conteudo.contains("value: 12;"));

    let designer_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormLists.designer.rs")
        .unwrap();
    assert!(designer_file.conteudo.contains("pub cbo_theme: ComboBox,"));
    assert!(designer_file.conteudo.contains("pub lst_items: ListBox,"));
    assert!(
        designer_file
            .conteudo
            .contains("pub pb_upload: ProgressBar,")
    );
    assert!(designer_file.conteudo.contains("pub tb_volume: TrackBar,"));
    assert!(designer_file.conteudo.contains("pub spn_count: SpinEdit,"));
    assert!(designer_file.conteudo.contains("pub struct ComboBox"));
    assert!(designer_file.conteudo.contains("pub struct ListBox"));
    assert!(designer_file.conteudo.contains("pub struct ProgressBar"));
    assert!(designer_file.conteudo.contains("pub struct TrackBar"));
    assert!(designer_file.conteudo.contains("pub struct SpinEdit"));
    assert!(
        designer_file
            .conteudo
            .contains("window.on_cbo_theme_selected")
    );
    assert!(
        designer_file
            .conteudo
            .contains("window.on_lst_items_current_item_changed")
    );
    assert!(
        designer_file
            .conteudo
            .contains("window.on_tb_volume_changed")
    );
}

#[test]
fn test_slint_generator_emits_containers_and_options() {
    let mut doc = Document::new("FormContainers", "Form");
    {
        let form = doc.node_mut("FormContainers").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        form.set_name("FormContainers");
        form.set_text("Containers Form");
    }

    let mut pnl = Node::new("pnl_header", "Panel");
    pnl.set_name("pnl_header");
    pnl.set_rect(Rect::new(10.0, 10.0, 780.0, 60.0));
    let _ = doc.add_child("FormContainers", pnl);

    let mut btn_child = Node::new("btn_inside", "Button");
    btn_child.set_name("btn_inside");
    btn_child.set_text("Inside Panel");
    btn_child.set_rect(Rect::new(10.0, 10.0, 120.0, 32.0));
    let _ = doc.add_child("pnl_header", btn_child);

    let mut grp = Node::new("grp_prefs", "GroupBox");
    grp.set_name("grp_prefs");
    grp.set_text("User Preferences");
    grp.set_rect(Rect::new(10.0, 80.0, 380.0, 200.0));
    let _ = doc.add_child("FormContainers", grp);

    let mut rb_child = Node::new("rb_dark", "RadioButton");
    rb_child.set_name("rb_dark");
    rb_child.set_text("Use Dark Theme");
    rb_child.set_rect(Rect::new(10.0, 20.0, 150.0, 24.0));
    let _ = doc.add_child("grp_prefs", rb_child);

    let mut rb_stand = Node::new("rb_mode", "RadioButton");
    rb_stand.set_name("rb_mode");
    rb_stand.set_text("Expert Mode");
    rb_stand
        .props
        .insert("checked".to_string(), crawrad_ir::Value::Bool(true));
    rb_stand
        .events
        .insert("on_click".to_string(), "rb_mode_click".to_string());
    let _ = doc.add_child("FormContainers", rb_stand);

    let mut rg = Node::new("rg_priority", "RadioGroup");
    rg.set_name("rg_priority");
    rg.set_text("Task Priority");
    rg.props.insert(
        "items".to_string(),
        crawrad_ir::Value::List(vec![
            crawrad_ir::Value::Str("Low".to_string()),
            crawrad_ir::Value::Str("High".to_string()),
        ]),
    );
    rg.props
        .insert("item_index".to_string(), crawrad_ir::Value::Int(1));
    let _ = doc.add_child("FormContainers", rg);

    let generator = GeradorSlint;
    let files = generator
        .gerar(&doc, "my_app", &crawrad_registry::Registry::builtin())
        .expect("must generate containers successfully");

    let slint_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormContainers.slint")
        .unwrap();

    assert!(slint_file.conteudo.contains("pnl_header := Rectangle"));
    assert!(slint_file.conteudo.contains("btn_inside := Button"));
    assert!(slint_file.conteudo.contains("grp_prefs := GroupBox"));
    assert!(slint_file.conteudo.contains("title: \"User Preferences\";"));
    assert!(slint_file.conteudo.contains("rb_dark := HorizontalLayout"));
    assert!(slint_file.conteudo.contains("rb_mode := HorizontalLayout"));
    assert!(slint_file.conteudo.contains("callback rb_mode_clicked();"));
    assert!(slint_file.conteudo.contains("rg_priority := GroupBox"));
    assert!(slint_file.conteudo.contains("title: \"Task Priority\";"));

    let designer_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormContainers.designer.rs")
        .unwrap();
    assert!(designer_file.conteudo.contains("pub pnl_header: Panel,"));
    assert!(designer_file.conteudo.contains("pub btn_inside: Button,"));
    assert!(designer_file.conteudo.contains("pub grp_prefs: GroupBox,"));
    assert!(designer_file.conteudo.contains("pub rb_dark: RadioButton,"));
    assert!(designer_file.conteudo.contains("pub rb_mode: RadioButton,"));
    assert!(
        designer_file
            .conteudo
            .contains("pub rg_priority: RadioGroup,")
    );
    assert!(designer_file.conteudo.contains("pub struct Panel"));
    assert!(designer_file.conteudo.contains("pub struct GroupBox"));
    assert!(designer_file.conteudo.contains("pub struct RadioButton"));
    assert!(designer_file.conteudo.contains("pub struct RadioGroup"));
    assert!(designer_file.conteudo.contains("window.on_rb_mode_clicked"));
}

#[test]
fn test_slint_generator_emits_page_control_and_tabs() {
    let mut doc = Document::new("FormTabs", "Form");
    {
        let form = doc.node_mut("FormTabs").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        form.set_name("FormTabs");
        form.set_text("Tabs Form");
    }

    let mut pgc = Node::new("pgc_main", "PageControl");
    pgc.set_name("pgc_main");
    pgc.set_rect(Rect::new(10.0, 10.0, 600.0, 400.0));
    pgc.props
        .insert("active_page".to_string(), crawrad_ir::Value::Int(0));
    pgc.events
        .insert("on_change".to_string(), "pgc_main_change".to_string());
    let _ = doc.add_child("FormTabs", pgc);

    let mut tab1 = Node::new("tab_general", "TabSheet");
    tab1.set_name("tab_general");
    tab1.set_text("General");
    let _ = doc.add_child("pgc_main", tab1);

    let mut btn_save = Node::new("btn_save", "Button");
    btn_save.set_name("btn_save");
    btn_save.set_text("Save Settings");
    let _ = doc.add_child("tab_general", btn_save);

    let mut tab2 = Node::new("tab_advanced", "TabSheet");
    tab2.set_name("tab_advanced");
    tab2.set_text("Advanced");
    let _ = doc.add_child("pgc_main", tab2);

    let generator = GeradorSlint;
    let files = generator
        .gerar(&doc, "my_app", &crawrad_registry::Registry::builtin())
        .expect("must generate page control successfully");

    let slint_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormTabs.slint")
        .unwrap();

    assert!(slint_file.conteudo.contains("pgc_main := Rectangle"));
    assert!(
        slint_file
            .conteudo
            .contains("callback pgc_main_tab_changed(int);")
    );
    assert!(slint_file.conteudo.contains("tab_general := Rectangle"));
    assert!(slint_file.conteudo.contains("btn_save := Button"));
    assert!(slint_file.conteudo.contains("tab_advanced := Rectangle"));

    let designer_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormTabs.designer.rs")
        .unwrap();
    assert!(
        designer_file
            .conteudo
            .contains("pub pgc_main: PageControl,")
    );
    assert!(
        designer_file
            .conteudo
            .contains("pub tab_general: TabSheet,")
    );
    assert!(
        designer_file
            .conteudo
            .contains("pub tab_advanced: TabSheet,")
    );
    assert!(designer_file.conteudo.contains("pub btn_save: Button,"));
    assert!(designer_file.conteudo.contains("pub struct PageControl"));
    assert!(designer_file.conteudo.contains("pub struct TabSheet"));
    assert!(
        designer_file
            .conteudo
            .contains("window.on_pgc_main_tab_changed")
    );
}

#[test]
fn test_slint_generator_emits_button_variants_and_link_label() {
    let mut doc = Document::new("FormButtons", "Form");
    {
        let form = doc.node_mut("FormButtons").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        form.set_name("FormButtons");
        form.set_text("Buttons Form");
    }

    let mut sbtn = Node::new("sbtn_play", "SpeedButton");
    sbtn.set_name("sbtn_play");
    sbtn.set_text("Play");
    sbtn.set_rect(Rect::new(10.0, 10.0, 80.0, 32.0));
    sbtn.events
        .insert("on_click".to_string(), "sbtn_play_click".to_string());
    let _ = doc.add_child("FormButtons", sbtn);

    let mut bbtn = Node::new("bbtn_ok", "BitBtn");
    bbtn.set_name("bbtn_ok");
    bbtn.set_text("OK");
    bbtn.set_rect(Rect::new(100.0, 10.0, 96.0, 32.0));
    bbtn.events
        .insert("on_click".to_string(), "bbtn_ok_click".to_string());
    let _ = doc.add_child("FormButtons", bbtn);

    let mut lnk = Node::new("lnk_docs", "LinkLabel");
    lnk.set_name("lnk_docs");
    lnk.set_text("Documentation");
    lnk.set_rect(Rect::new(10.0, 60.0, 140.0, 24.0));
    lnk.events
        .insert("on_click".to_string(), "lnk_docs_click".to_string());
    let _ = doc.add_child("FormButtons", lnk);

    let generator = GeradorSlint;
    let files = generator
        .gerar(&doc, "my_app", &crawrad_registry::Registry::builtin())
        .expect("must generate button variants successfully");

    let slint_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormButtons.slint")
        .unwrap();

    assert!(slint_file.conteudo.contains("sbtn_play := Button"));
    assert!(slint_file.conteudo.contains("text: \"Play\";"));
    assert!(
        slint_file
            .conteudo
            .contains("callback sbtn_play_clicked();")
    );
    assert!(slint_file.conteudo.contains("bbtn_ok := Button"));
    assert!(slint_file.conteudo.contains("text: \"OK\";"));
    assert!(slint_file.conteudo.contains("callback bbtn_ok_clicked();"));
    assert!(slint_file.conteudo.contains("lnk_docs := TouchArea"));
    assert!(slint_file.conteudo.contains("text: \"Documentation\";"));
    assert!(slint_file.conteudo.contains("color: #0066cc;"));
    assert!(slint_file.conteudo.contains("callback lnk_docs_clicked();"));

    let designer_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormButtons.designer.rs")
        .unwrap();
    assert!(
        designer_file
            .conteudo
            .contains("pub sbtn_play: SpeedButton,")
    );
    assert!(designer_file.conteudo.contains("pub bbtn_ok: BitBtn,"));
    assert!(designer_file.conteudo.contains("pub lnk_docs: LinkLabel,"));
    assert!(designer_file.conteudo.contains("pub struct SpeedButton"));
    assert!(designer_file.conteudo.contains("pub struct BitBtn"));
    assert!(designer_file.conteudo.contains("pub struct LinkLabel"));
    assert!(
        designer_file
            .conteudo
            .contains("window.on_sbtn_play_clicked")
    );
    assert!(designer_file.conteudo.contains("window.on_bbtn_ok_clicked"));
    assert!(
        designer_file
            .conteudo
            .contains("window.on_lnk_docs_clicked")
    );
}

#[test]
fn test_slint_generator_emits_graphic_components() {
    let mut doc = Document::new("FormGraphics", "Form");
    {
        let form = doc.node_mut("FormGraphics").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        form.set_name("FormGraphics");
        form.set_text("Graphics Form");
    }

    let mut img = Node::new("img_logo", "Image");
    img.set_name("img_logo");
    img.set_rect(Rect::new(10.0, 10.0, 120.0, 80.0));
    let _ = doc.add_child("FormGraphics", img);

    let mut shp = Node::new("shp_box", "Shape");
    shp.set_name("shp_box");
    shp.set_rect(Rect::new(140.0, 10.0, 100.0, 100.0));
    let _ = doc.add_child("FormGraphics", shp);

    let mut pbx = Node::new("pbx_canvas", "PaintBox");
    pbx.set_name("pbx_canvas");
    pbx.set_rect(Rect::new(10.0, 120.0, 200.0, 150.0));
    let _ = doc.add_child("FormGraphics", pbx);

    let mut bvl = Node::new("bvl_separator", "Bevel");
    bvl.set_name("bvl_separator");
    bvl.set_rect(Rect::new(10.0, 280.0, 300.0, 4.0));
    let _ = doc.add_child("FormGraphics", bvl);

    let generator = GeradorSlint;
    let files = generator
        .gerar(&doc, "my_app", &crawrad_registry::Registry::builtin())
        .expect("must generate graphic components successfully");

    let slint_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormGraphics.slint")
        .unwrap();

    assert!(slint_file.conteudo.contains("img_logo := Image"));
    assert!(slint_file.conteudo.contains("image-fit: contain;"));
    assert!(slint_file.conteudo.contains("shp_box := Rectangle"));
    assert!(slint_file.conteudo.contains("border-color: #000000;"));
    assert!(slint_file.conteudo.contains("pbx_canvas := Rectangle"));
    assert!(slint_file.conteudo.contains("bvl_separator := Rectangle"));
    assert!(slint_file.conteudo.contains("border-color: #a0a0a0;"));

    let designer_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormGraphics.designer.rs")
        .unwrap();
    assert!(designer_file.conteudo.contains("pub img_logo: Image,"));
    assert!(designer_file.conteudo.contains("pub shp_box: Shape,"));
    assert!(designer_file.conteudo.contains("pub pbx_canvas: PaintBox,"));
    assert!(designer_file.conteudo.contains("pub bvl_separator: Bevel,"));
    assert!(designer_file.conteudo.contains("pub struct Image"));
    assert!(designer_file.conteudo.contains("pub struct Shape"));
    assert!(designer_file.conteudo.contains("pub struct PaintBox"));
    assert!(designer_file.conteudo.contains("pub struct Bevel"));
}

#[test]
fn test_slint_generator_emits_timer_and_dialogs() {
    let mut doc = Document::new("FormSystem", "Form");
    {
        let form = doc.node_mut("FormSystem").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        form.set_name("FormSystem");
        form.set_text("System Components");
    }

    let mut tmr = Node::new("tmr_tick", "Timer");
    tmr.set_name("tmr_tick");
    tmr.props
        .insert("interval".to_string(), crawrad_ir::Value::Int(500));
    tmr.events
        .insert("on_timer".to_string(), "tmr_tick_timer".to_string());
    let _ = doc.add_child("FormSystem", tmr);

    let mut dlg_open = Node::new("dlg_open", "OpenDialog");
    dlg_open.set_name("dlg_open");
    let _ = doc.add_child("FormSystem", dlg_open);

    let mut dlg_save = Node::new("dlg_save", "SaveDialog");
    dlg_save.set_name("dlg_save");
    let _ = doc.add_child("FormSystem", dlg_save);

    let mut dlg_msg = Node::new("dlg_msg", "MessageDialog");
    dlg_msg.set_name("dlg_msg");
    let _ = doc.add_child("FormSystem", dlg_msg);

    let generator = GeradorSlint;
    let files = generator
        .gerar(&doc, "my_app", &crawrad_registry::Registry::builtin())
        .expect("must generate timer and dialogs successfully");

    let designer_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormSystem.designer.rs")
        .unwrap();
    assert!(designer_file.conteudo.contains("pub tmr_tick: Timer,"));
    assert!(designer_file.conteudo.contains("pub dlg_open: OpenDialog,"));
    assert!(designer_file.conteudo.contains("pub dlg_save: SaveDialog,"));
    assert!(
        designer_file
            .conteudo
            .contains("pub dlg_msg: MessageDialog,")
    );
    assert!(designer_file.conteudo.contains("pub struct Timer"));
    assert!(designer_file.conteudo.contains("pub struct OpenDialog"));
    assert!(designer_file.conteudo.contains("pub struct SaveDialog"));
    assert!(designer_file.conteudo.contains("pub struct MessageDialog"));
    assert!(designer_file.conteudo.contains("f.form.tmr_tick_timer();"));
}

#[test]
fn test_slint_target_manifest_and_dependencies() {
    use crate::AlvoGerador;
    use crate::dependencies::{dependencias_do_projeto, gerar_cargo_toml};

    // 1. Unified dependencies check: Slint gets slint and not makepad-widgets
    let deps = dependencias_do_projeto(AlvoGerador::Slint, true, false);
    assert!(
        deps.iter().any(|d| d.nome() == "slint"),
        "Slint project must have 'slint' dependency"
    );
    assert!(
        !deps.iter().any(|d| d.nome() == "makepad-widgets"),
        "Slint project must NOT have 'makepad-widgets' dependency"
    );

    // 2. Cargo.toml generation check: includes slint, excludes makepad-widgets and eframe
    let toml = gerar_cargo_toml("slint_sample_app", AlvoGerador::Slint, true, false, &[]);
    assert!(
        toml.contains("slint = \"1.9\""),
        "Generated Cargo.toml must contain slint = \"1.9\" dependency:\n{toml}"
    );
    assert!(
        !toml.contains("makepad-widgets"),
        "Generated Cargo.toml must NOT contain makepad-widgets:\n{toml}"
    );
    assert!(
        !toml.contains("eframe"),
        "Generated Cargo.toml must NOT contain eframe:\n{toml}"
    );
}

#[test]
fn test_slint_slice1_components() {
    use crawrad_ir::Value;

    let mut doc = Document::new("FormSlice1", "Form");
    {
        let form = doc.node_mut("FormSlice1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        form.set_name("FormSlice1");
        form.set_text("Slice 1 Form");
    }

    // 1. Badge
    let mut badge = Node::new("bdg_count", "Badge");
    badge.set_name("bdg_count");
    badge.set_rect(Rect::new(10.0, 20.0, 56.0, 20.0));
    badge.set_text("New");
    badge.props.insert("count".to_string(), Value::Int(5));
    badge.props.insert("max_count".to_string(), Value::Int(99));
    badge
        .props
        .insert("kind".to_string(), Value::Str("bkPrimary".to_string()));
    badge
        .props
        .insert("outline".to_string(), Value::Bool(false));
    badge
        .events
        .insert("on_click".to_string(), "bdg_click".to_string());
    let _ = doc.add_child("FormSlice1", badge);

    // 2. Chips
    let mut chips = Node::new("chp_tags", "Chips");
    chips.set_name("chp_tags");
    chips.set_rect(Rect::new(10.0, 50.0, 220.0, 56.0));
    chips.props.insert(
        "items".to_string(),
        Value::List(vec![
            Value::Str("Rust".to_string()),
            Value::Str("egui".to_string()),
            Value::Str("Makepad".to_string()),
        ]),
    );
    chips.props.insert("item_index".to_string(), Value::Int(1));
    chips
        .props
        .insert("removable".to_string(), Value::Bool(true));
    chips
        .events
        .insert("on_click".to_string(), "chp_click".to_string());
    chips
        .events
        .insert("on_change".to_string(), "chp_change".to_string());
    let _ = doc.add_child("FormSlice1", chips);

    // 3. Breadcrumb
    let mut breadcrumb = Node::new("brc_path", "Breadcrumb");
    breadcrumb.set_name("brc_path");
    breadcrumb.set_rect(Rect::new(10.0, 120.0, 260.0, 24.0));
    breadcrumb.props.insert(
        "items".to_string(),
        Value::List(vec![
            Value::Str("Home".to_string()),
            Value::Str("Projects".to_string()),
            Value::Str("CrawRAD".to_string()),
        ]),
    );
    breadcrumb
        .props
        .insert("separator".to_string(), Value::Str("›".to_string()));
    breadcrumb
        .props
        .insert("item_index".to_string(), Value::Int(2));
    breadcrumb.props.insert(
        "segment_color".to_string(),
        Value::Color(crawrad_ir::Color::rgb(0x45, 0x46, 0x4B)),
    );
    breadcrumb.props.insert(
        "current_color".to_string(),
        Value::Color(crawrad_ir::Color::rgb(0xE0, 0x61, 0x2B)),
    );
    breadcrumb
        .events
        .insert("on_click".to_string(), "brc_click".to_string());
    let _ = doc.add_child("FormSlice1", breadcrumb);

    // 4. Pagination
    let mut pagination = Node::new("pag_nav", "Pagination");
    pagination.set_name("pag_nav");
    pagination.set_rect(Rect::new(10.0, 160.0, 280.0, 28.0));
    pagination
        .props
        .insert("page_count".to_string(), Value::Int(5));
    pagination
        .props
        .insert("page_index".to_string(), Value::Int(2));
    pagination
        .props
        .insert("max_buttons".to_string(), Value::Int(5));
    pagination
        .props
        .insert("show_arrows".to_string(), Value::Bool(true));
    pagination
        .events
        .insert("on_change".to_string(), "pag_change".to_string());
    let _ = doc.add_child("FormSlice1", pagination);

    let generator = GeradorSlint;
    let files = generator
        .gerar(&doc, "slice1_app", &crawrad_registry::Registry::builtin())
        .expect("must generate slice 1 successfully");

    let slint_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormSlice1.slint")
        .unwrap();
    let sc = &slint_file.conteudo;

    // === Assertions for Badge ===
    assert!(
        sc.contains("bdg_count := Rectangle {"),
        "Badge element missing"
    );
    assert!(sc.contains("x: 10.0px;"), "Badge x missing");
    assert!(sc.contains("y: 20.0px;"), "Badge y missing");
    assert!(sc.contains("width: 56.0px;"), "Badge width missing");
    assert!(sc.contains("height: 20.0px;"), "Badge height missing");
    assert!(
        sc.contains("border-radius: 10.0px;"),
        "Badge border-radius missing"
    );
    assert!(
        sc.contains("background: #cc3838;"),
        "Badge background missing"
    );
    assert!(
        sc.contains("border-color: #cc3838;"),
        "Badge border-color missing"
    );
    assert!(
        sc.contains("border-width: 1px;"),
        "Badge border-width missing"
    );
    assert!(sc.contains("visible: true;"), "Badge visible missing");
    assert!(
        sc.contains("text: \"5\";"),
        "Badge effective count text missing"
    );
    assert!(sc.contains("color: #f2f2fa;"), "Badge text color missing");
    assert!(
        sc.contains("clicked => { root.bdg_count_clicked(); }"),
        "Badge click wiring missing"
    );
    assert!(
        sc.contains("callback bdg_count_clicked();"),
        "Badge callback declaration missing"
    );

    // === Assertions for Chips ===
    assert!(
        sc.contains("chp_tags := HorizontalLayout {"),
        "Chips element missing"
    );
    assert!(sc.contains("x: 10.0px;"), "Chips x missing");
    assert!(sc.contains("y: 50.0px;"), "Chips y missing");
    assert!(sc.contains("width: 220.0px;"), "Chips width missing");
    assert!(sc.contains("height: 56.0px;"), "Chips height missing");
    assert!(sc.contains("spacing: 6px;"), "Chips spacing missing");
    assert!(sc.contains("visible: true;"), "Chips visible missing");
    assert!(sc.contains("enabled: true;"), "Chips enabled missing");
    assert!(
        sc.contains("for item[index] in [\"Rust\", \"egui\", \"Makepad\"]: Rectangle {"),
        "Chips repeater missing"
    );
    assert!(
        sc.contains("border-radius: 12px;"),
        "Chips border-radius missing"
    );
    assert!(
        sc.contains("background: index == 1 ? #3a598c : #e0e0e0;"),
        "Chips background binding missing"
    );
    assert!(
        sc.contains("border-width: 1px;"),
        "Chips border-width missing"
    );
    assert!(
        sc.contains("border-color: #cccccc;"),
        "Chips border-color missing"
    );
    assert!(
        sc.contains("color: index == 1 ? #ffffff : #222222;"),
        "Chips text color binding missing"
    );
    assert!(sc.contains("text: \"×\";"), "Chips removable cross missing");
    assert!(
        sc.contains("clicked => { root.chp_tags_selected(index); }"),
        "Chips click wiring missing"
    );
    assert!(
        sc.contains("callback chp_tags_selected(int);"),
        "Chips selected callback declaration missing"
    );
    assert!(
        sc.contains("callback chp_tags_removed(int);"),
        "Chips removed callback declaration missing"
    );

    // === Assertions for Breadcrumb ===
    assert!(
        sc.contains("brc_path := HorizontalLayout {"),
        "Breadcrumb element missing"
    );
    assert!(sc.contains("x: 10.0px;"), "Breadcrumb x missing");
    assert!(sc.contains("y: 120.0px;"), "Breadcrumb y missing");
    assert!(sc.contains("width: 260.0px;"), "Breadcrumb width missing");
    assert!(sc.contains("height: 24.0px;"), "Breadcrumb height missing");
    assert!(
        sc.contains("for item[index] in [\"Home\", \"Projects\", \"CrawRAD\"]: HorizontalLayout {"),
        "Breadcrumb repeater missing"
    );
    assert!(
        sc.contains("color: index == 2 ? #E0612B : #45464B;"),
        "Breadcrumb current color binding missing"
    );
    assert!(
        sc.contains("clicked => { root.brc_path_clicked(index); }"),
        "Breadcrumb click wiring missing"
    );
    assert!(
        sc.contains("if index < 2: Text {"),
        "Breadcrumb separator condition missing"
    );
    assert!(
        sc.contains("text: \"›\";"),
        "Breadcrumb separator char missing"
    );
    assert!(
        sc.contains("callback brc_path_clicked(int);"),
        "Breadcrumb callback declaration missing"
    );

    // === Assertions for Pagination ===
    assert!(
        sc.contains("pag_nav := HorizontalLayout {"),
        "Pagination element missing"
    );
    assert!(sc.contains("x: 10.0px;"), "Pagination x missing");
    assert!(sc.contains("y: 160.0px;"), "Pagination y missing");
    assert!(sc.contains("width: 280.0px;"), "Pagination width missing");
    assert!(sc.contains("height: 28.0px;"), "Pagination height missing");
    assert!(
        sc.contains("Text { text: \"‹\";"),
        "Pagination prev arrow missing"
    );
    assert!(
        sc.contains("clicked => { root.pag_nav_page_changed(1); }"),
        "Pagination prev wiring missing"
    );
    assert!(
        sc.contains("Text { text: \"3\"; color: #ffffff;"),
        "Pagination active page button missing"
    );
    assert!(
        sc.contains("Text { text: \"1\"; color: #333333;"),
        "Pagination inactive page button missing"
    );
    assert!(
        sc.contains("Text { text: \"›\";"),
        "Pagination next arrow missing"
    );
    assert!(
        sc.contains("clicked => { root.pag_nav_page_changed(3); }"),
        "Pagination next wiring missing"
    );
    assert!(
        sc.contains("callback pag_nav_page_changed(int);"),
        "Pagination callback declaration missing"
    );

    let designer_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormSlice1.designer.rs")
        .unwrap();
    let dc = &designer_file.conteudo;

    // Designer struct and field checks
    assert!(
        dc.contains("pub bdg_count: Badge,"),
        "Badge field missing in designer struct"
    );
    assert!(
        dc.contains("pub chp_tags: Chips,"),
        "Chips field missing in designer struct"
    );
    assert!(
        dc.contains("pub brc_path: Breadcrumb,"),
        "Breadcrumb field missing in designer struct"
    );
    assert!(
        dc.contains("pub pag_nav: Pagination,"),
        "Pagination field missing in designer struct"
    );
    assert!(
        dc.contains("pub struct Badge"),
        "Badge struct definition missing"
    );
    assert!(
        dc.contains("pub struct Chips"),
        "Chips struct definition missing"
    );
    assert!(
        dc.contains("pub struct Breadcrumb"),
        "Breadcrumb struct definition missing"
    );
    assert!(
        dc.contains("pub struct Pagination"),
        "Pagination struct definition missing"
    );

    // Action wiring in designer
    assert!(
        dc.contains("window.on_bdg_count_clicked"),
        "Badge on_click wiring missing"
    );
    assert!(
        dc.contains("f.form.bdg_click();"),
        "Badge action invocation missing"
    );
    assert!(
        dc.contains("window.on_chp_tags_selected"),
        "Chips on_selected wiring missing"
    );
    assert!(
        dc.contains("f.form.chp_click();"),
        "Chips on_click invocation missing"
    );
    assert!(
        dc.contains("f.form.chp_change();"),
        "Chips on_change invocation missing"
    );
    assert!(
        dc.contains("window.on_chp_tags_removed"),
        "Chips on_removed wiring missing"
    );
    assert!(
        dc.contains("window.on_brc_path_clicked"),
        "Breadcrumb on_click wiring missing"
    );
    assert!(
        dc.contains("f.form.brc_click();"),
        "Breadcrumb action invocation missing"
    );
    assert!(
        dc.contains("window.on_pag_nav_page_changed"),
        "Pagination on_change wiring missing"
    );
    assert!(
        dc.contains("f.form.pag_change();"),
        "Pagination action invocation missing"
    );
}
