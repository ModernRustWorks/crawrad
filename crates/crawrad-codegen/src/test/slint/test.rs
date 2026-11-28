use crate::Gerador;
use crate::targets::slint::GeradorSlint;
use crate::targets::slint::layout::{position, text};
use crate::targets::slint::main_app::main_rs;
use crawrad_ir::{Document, Node, Rect, Value};

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

#[test]
fn test_slint_slice2_components() {
    use crawrad_ir::Value;

    let mut doc = Document::new("FormSlice2", "Form");
    {
        let form = doc.node_mut("FormSlice2").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        form.set_name("FormSlice2");
        form.set_text("Slice 2 Form");
    }

    // 1. ScrollBar
    let mut sb = Node::new("sb_pos", "ScrollBar");
    sb.set_name("sb_pos");
    sb.set_rect(Rect::new(10.0, 20.0, 120.0, 16.0));
    sb.props.insert("min".to_string(), Value::Float(0.0));
    sb.props.insert("max".to_string(), Value::Float(100.0));
    sb.props.insert("position".to_string(), Value::Float(25.0));
    sb.events
        .insert("on_change".to_string(), "sb_change".to_string());
    sb.events
        .insert("on_scroll".to_string(), "sb_scroll".to_string());
    let _ = doc.add_child("FormSlice2", sb);

    // 2. ScrollBox
    let mut sbx = Node::new("sbx_view", "ScrollBox");
    sbx.set_name("sbx_view");
    sbx.set_rect(Rect::new(10.0, 50.0, 200.0, 150.0));
    sbx.props.insert("border_width".to_string(), Value::Int(2));
    let _ = doc.add_child("FormSlice2", sbx);

    // 3. Splitter
    let mut spl = Node::new("spl_pane", "Splitter");
    spl.set_name("spl_pane");
    spl.set_rect(Rect::new(10.0, 210.0, 5.0, 100.0));
    spl.events
        .insert("on_moved".to_string(), "spl_moved".to_string());
    let _ = doc.add_child("FormSlice2", spl);

    // 4. Drawer
    let mut drw = Node::new("drw_menu", "Drawer");
    drw.set_name("drw_menu");
    drw.set_rect(Rect::new(0.0, 0.0, 240.0, 320.0));
    drw.set_text("Navigation");
    drw.props.insert("opened".to_string(), Value::Bool(true));
    drw.props
        .insert("show_header".to_string(), Value::Bool(true));
    drw.events
        .insert("on_close".to_string(), "drw_close".to_string());
    let _ = doc.add_child("FormSlice2", drw);

    let generator = GeradorSlint;
    let files = generator
        .gerar(&doc, "slice2_app", &crawrad_registry::Registry::builtin())
        .expect("must generate slice 2 successfully");

    let slint_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormSlice2.slint")
        .unwrap();
    let sc = &slint_file.conteudo;

    // === Assertions for ScrollBar ===
    assert!(
        sc.contains("sb_pos := Slider {"),
        "ScrollBar element missing"
    );
    assert!(sc.contains("x: 10.0px;"), "ScrollBar x missing");
    assert!(sc.contains("y: 20.0px;"), "ScrollBar y missing");
    assert!(sc.contains("width: 120.0px;"), "ScrollBar width missing");
    assert!(sc.contains("height: 16.0px;"), "ScrollBar height missing");
    assert!(sc.contains("minimum: 0.0;"), "ScrollBar minimum missing");
    assert!(sc.contains("maximum: 100.0;"), "ScrollBar maximum missing");
    assert!(sc.contains("value: 25.0;"), "ScrollBar value missing");
    assert!(
        sc.contains("changed(val) => { root.sb_pos_changed(val); }"),
        "ScrollBar change wiring missing"
    );
    assert!(
        sc.contains("callback sb_pos_changed(float);"),
        "ScrollBar callback declaration missing"
    );

    // === Assertions for ScrollBox ===
    assert!(
        sc.contains("sbx_view := Rectangle {"),
        "ScrollBox element missing"
    );
    assert!(sc.contains("x: 10.0px;"), "ScrollBox x missing");
    assert!(sc.contains("y: 50.0px;"), "ScrollBox y missing");
    assert!(sc.contains("width: 200.0px;"), "ScrollBox width missing");
    assert!(sc.contains("height: 150.0px;"), "ScrollBox height missing");
    assert!(
        sc.contains("background: #1e1e2e;"),
        "ScrollBox background missing"
    );
    assert!(
        sc.contains("border-width: 2px;"),
        "ScrollBox border-width missing"
    );
    assert!(
        sc.contains("border-color: #45475a;"),
        "ScrollBox border-color missing"
    );
    assert!(sc.contains("clip: true;"), "ScrollBox clip missing");

    // === Assertions for Splitter ===
    assert!(
        sc.contains("spl_pane := Rectangle {"),
        "Splitter element missing"
    );
    assert!(sc.contains("x: 10.0px;"), "Splitter x missing");
    assert!(sc.contains("y: 210.0px;"), "Splitter y missing");
    assert!(sc.contains("width: 5.0px;"), "Splitter width missing");
    assert!(sc.contains("height: 100.0px;"), "Splitter height missing");
    assert!(
        sc.contains("background: #252538;"),
        "Splitter background missing"
    );
    assert!(
        sc.contains("mouse-cursor: col-resize;"),
        "Splitter mouse-cursor missing"
    );
    assert!(
        sc.contains("moved => { root.spl_pane_moved(); }"),
        "Splitter moved wiring missing"
    );
    assert!(
        sc.contains("callback spl_pane_moved();"),
        "Splitter callback declaration missing"
    );

    // === Assertions for Drawer ===
    assert!(
        sc.contains("drw_menu := Rectangle {"),
        "Drawer element missing"
    );
    assert!(sc.contains("x: 0.0px;"), "Drawer x missing");
    assert!(sc.contains("y: 0.0px;"), "Drawer y missing");
    assert!(sc.contains("width: 240.0px;"), "Drawer width missing");
    assert!(sc.contains("height: 320.0px;"), "Drawer height missing");
    assert!(
        sc.contains("background: #2f2f45;"),
        "Drawer background missing"
    );
    assert!(
        sc.contains("border-color: #6e7290;"),
        "Drawer border-color missing"
    );
    assert!(
        sc.contains("visible: true && true;"),
        "Drawer visible binding missing"
    );
    assert!(sc.contains("height: 32px;"), "Drawer header height missing");
    assert!(sc.contains("text: \"Navigation\";"), "Drawer title missing");
    assert!(
        sc.contains("text: \"×\";"),
        "Drawer close button glyph missing"
    );
    assert!(
        sc.contains("clicked => { root.drw_menu_close_clicked(); }"),
        "Drawer close click wiring missing"
    );
    assert!(
        sc.contains("callback drw_menu_close_clicked();"),
        "Drawer callback declaration missing"
    );

    let designer_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormSlice2.designer.rs")
        .unwrap();
    let dc = &designer_file.conteudo;

    // Designer struct and field checks
    assert!(
        dc.contains("pub sb_pos: ScrollBar,"),
        "ScrollBar field missing in designer struct"
    );
    assert!(
        dc.contains("pub sbx_view: ScrollBox,"),
        "ScrollBox field missing in designer struct"
    );
    assert!(
        dc.contains("pub spl_pane: Splitter,"),
        "Splitter field missing in designer struct"
    );
    assert!(
        dc.contains("pub drw_menu: Drawer,"),
        "Drawer field missing in designer struct"
    );
    assert!(
        dc.contains("pub struct ScrollBar"),
        "ScrollBar struct definition missing"
    );
    assert!(
        dc.contains("pub struct ScrollBox"),
        "ScrollBox struct definition missing"
    );
    assert!(
        dc.contains("pub struct Splitter"),
        "Splitter struct definition missing"
    );
    assert!(
        dc.contains("pub struct Drawer"),
        "Drawer struct definition missing"
    );

    // Action wiring in designer
    assert!(
        dc.contains("window.on_sb_pos_changed"),
        "ScrollBar on_changed wiring missing"
    );
    assert!(
        dc.contains("f.form.sb_change();"),
        "ScrollBar on_change action missing"
    );
    assert!(
        dc.contains("f.form.sb_scroll();"),
        "ScrollBar on_scroll action missing"
    );
    assert!(
        dc.contains("window.on_spl_pane_moved"),
        "Splitter on_moved wiring missing"
    );
    assert!(
        dc.contains("f.form.spl_moved();"),
        "Splitter on_moved action missing"
    );
    assert!(
        dc.contains("window.on_drw_menu_close_clicked"),
        "Drawer on_close wiring missing"
    );
    assert!(
        dc.contains("f.form.drw_close();"),
        "Drawer on_close action missing"
    );
    assert!(
        dc.contains("f.form.drw_menu.opened = false;"),
        "Drawer state close missing"
    );
}

#[test]
fn test_slint_slice3_components() {
    use crawrad_ir::Value;

    let mut doc = Document::new("FormSlice3", "Form");
    {
        let form = doc.node_mut("FormSlice3").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        form.set_name("FormSlice3");
        form.set_text("Slice 3 Form");
    }

    // 1. ToolBar
    let mut tbr = Node::new("tbr_actions", "ToolBar");
    tbr.set_name("tbr_actions");
    tbr.set_rect(Rect::new(0.0, 0.0, 800.0, 32.0));
    tbr.props.insert("flat".to_string(), Value::Bool(true));
    let _ = doc.add_child("FormSlice3", tbr);

    // 2. Card
    let mut crd = Node::new("crd_box", "Card");
    crd.set_name("crd_box");
    crd.set_rect(Rect::new(10.0, 40.0, 220.0, 140.0));
    crd.set_text("Card Title");
    crd.props.insert(
        "subtitle".to_string(),
        Value::Str("Card Subtitle".to_string()),
    );
    crd.props.insert("corner_radius".to_string(), Value::Int(8));
    crd.props
        .insert("show_header".to_string(), Value::Bool(true));
    crd.events
        .insert("on_click".to_string(), "crd_click".to_string());
    let _ = doc.add_child("FormSlice3", crd);

    // 3. CategoryPanel
    let mut cat = Node::new("cat_group", "CategoryPanel");
    cat.set_name("cat_group");
    cat.set_rect(Rect::new(10.0, 190.0, 200.0, 160.0));
    cat.set_text("General");
    cat.props
        .insert("collapsed".to_string(), Value::Bool(false));
    cat.events
        .insert("on_collapsed".to_string(), "cat_collapse".to_string());
    cat.events
        .insert("on_expanded".to_string(), "cat_expand".to_string());
    let _ = doc.add_child("FormSlice3", cat);

    // 4. StatusBar
    let mut stb = Node::new("stb_foot", "StatusBar");
    stb.set_name("stb_foot");
    stb.set_rect(Rect::new(0.0, 580.0, 780.0, 20.0));
    stb.props
        .insert("simple_text".to_string(), Value::Str("Ready".to_string()));
    stb.events
        .insert("on_click".to_string(), "stb_click".to_string());
    let _ = doc.add_child("FormSlice3", stb);

    // 5. MainMenu and MenuItem child
    let mut mnu = Node::new("mnu_main", "MainMenu");
    mnu.set_name("mnu_main");
    mnu.props
        .insert("images".to_string(), Value::Str("iml_icons".to_string()));
    let mut mni = Node::new("mni_file", "MenuItem");
    mni.set_name("mni_file");
    mni.set_text("File");
    mni.events
        .insert("on_click".to_string(), "file_click".to_string());
    let _ = doc.add_child("FormSlice3", mnu);
    let _ = doc.add_child("mnu_main", mni);

    // 6. PopupMenu
    let mut pop = Node::new("pop_ctx", "PopupMenu");
    pop.set_name("pop_ctx");
    pop.props
        .insert("images".to_string(), Value::Str("iml_icons".to_string()));
    let _ = doc.add_child("FormSlice3", pop);

    let generator = GeradorSlint;
    let files = generator
        .gerar(&doc, "slice3_app", &crawrad_registry::Registry::builtin())
        .expect("must generate slice 3 successfully");

    let slint_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormSlice3.slint")
        .unwrap();
    let sc = &slint_file.conteudo;

    // === Assertions for ToolBar ===
    assert!(
        sc.contains("tbr_actions := Rectangle {\n            x: 0.0px; y: 0.0px;\n            width: 800.0px;\n            height: 32.0px;"),
        "ToolBar geometry with width 800.0px missing or incorrect"
    );
    assert!(
        sc.contains("tbr_actions := Rectangle {"),
        "ToolBar element missing"
    );
    assert!(sc.contains("x: 0.0px;"), "ToolBar x missing");
    assert!(sc.contains("y: 0.0px;"), "ToolBar y missing");
    assert!(sc.contains("width: 800.0px;"), "ToolBar width missing");
    assert!(sc.contains("height: 32.0px;"), "ToolBar height missing");
    assert!(
        sc.contains("background: #2d3139;"),
        "ToolBar background missing"
    );
    assert!(
        sc.contains("border-width: 1px;"),
        "ToolBar border-width missing"
    );
    assert!(
        sc.contains("border-color: #1e1e24;"),
        "ToolBar border-color missing"
    );

    // === Assertions for Card ===
    assert!(
        sc.contains("crd_box := Rectangle {"),
        "Card element missing"
    );
    assert!(sc.contains("x: 10.0px;"), "Card x missing");
    assert!(sc.contains("y: 40.0px;"), "Card y missing");
    assert!(sc.contains("width: 220.0px;"), "Card width missing");
    assert!(sc.contains("height: 140.0px;"), "Card height missing");
    assert!(
        sc.contains("border-radius: 8px;"),
        "Card border-radius missing"
    );
    assert!(
        sc.contains("background: #252538;"),
        "Card background missing"
    );
    assert!(
        sc.contains("border-color: #3e3e56;"),
        "Card border-color missing"
    );
    assert!(sc.contains("text: \"Card Title\";"), "Card title missing");
    assert!(
        sc.contains("text: \"Card Subtitle\";"),
        "Card subtitle missing"
    );
    assert!(
        sc.contains("clicked => { root.crd_box_clicked(); }"),
        "Card click wiring missing"
    );
    assert!(
        sc.contains("callback crd_box_clicked();"),
        "Card callback declaration missing"
    );

    // === Assertions for CategoryPanel ===
    assert!(
        sc.contains("cat_group := Rectangle {"),
        "CategoryPanel element missing"
    );
    assert!(sc.contains("x: 10.0px;"), "CategoryPanel x missing");
    assert!(sc.contains("y: 190.0px;"), "CategoryPanel y missing");
    assert!(
        sc.contains("width: 200.0px;"),
        "CategoryPanel width missing"
    );
    assert!(
        sc.contains("height: 160.0px;"),
        "CategoryPanel height missing"
    );
    assert!(
        sc.contains("text: \"- General\";"),
        "CategoryPanel header text missing"
    );
    assert!(
        sc.contains("clicked => { root.cat_group_header_clicked(); }"),
        "CategoryPanel click wiring missing"
    );
    assert!(
        sc.contains("callback cat_group_header_clicked();"),
        "CategoryPanel callback declaration missing"
    );

    // === Assertions for StatusBar ===
    assert!(
        sc.contains("stb_foot := Rectangle {\n            x: 0.0px; y: 580.0px;\n            width: 780.0px;\n            height: 20.0px;"),
        "StatusBar geometry with width 780.0px missing or incorrect"
    );
    assert!(
        sc.contains("stb_foot := Rectangle {"),
        "StatusBar element missing"
    );
    assert!(sc.contains("x: 0.0px;"), "StatusBar x missing");
    assert!(sc.contains("y: 580.0px;"), "StatusBar y missing");
    assert!(sc.contains("width: 780.0px;"), "StatusBar width missing");
    assert!(sc.contains("height: 20.0px;"), "StatusBar height missing");
    assert!(
        sc.contains("background: #252526;"),
        "StatusBar background missing"
    );
    assert!(
        sc.contains("text: \"Ready\";"),
        "StatusBar simple text missing"
    );
    assert!(
        sc.contains("clicked => { root.stb_foot_clicked(); }"),
        "StatusBar click wiring missing"
    );
    assert!(
        sc.contains("callback stb_foot_clicked();"),
        "StatusBar callback declaration missing"
    );

    let designer_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormSlice3.designer.rs")
        .unwrap();
    let dc = &designer_file.conteudo;

    // Designer struct and field checks
    assert!(
        dc.contains("pub tbr_actions: ToolBar,"),
        "ToolBar field missing in designer struct"
    );
    assert!(
        dc.contains("pub crd_box: Card,"),
        "Card field missing in designer struct"
    );
    assert!(
        dc.contains("pub cat_group: CategoryPanel,"),
        "CategoryPanel field missing in designer struct"
    );
    assert!(
        dc.contains("pub stb_foot: StatusBar,"),
        "StatusBar field missing in designer struct"
    );
    assert!(
        dc.contains("pub mnu_main: MainMenu,"),
        "MainMenu field missing in designer struct"
    );
    assert!(
        dc.contains("pub mni_file: MenuItem,"),
        "MenuItem field missing in designer struct"
    );
    assert!(
        dc.contains("pub pop_ctx: PopupMenu,"),
        "PopupMenu field missing in designer struct"
    );
    assert!(
        dc.contains("pub struct ToolBar"),
        "ToolBar struct definition missing"
    );
    assert!(
        dc.contains("pub struct Card"),
        "Card struct definition missing"
    );
    assert!(
        dc.contains("pub struct CategoryPanel"),
        "CategoryPanel struct definition missing"
    );
    assert!(
        dc.contains("pub struct StatusBar"),
        "StatusBar struct definition missing"
    );
    assert!(
        dc.contains("pub struct MainMenu"),
        "MainMenu struct definition missing"
    );
    assert!(
        dc.contains("pub struct MenuItem"),
        "MenuItem struct definition missing"
    );
    assert!(
        dc.contains("pub struct PopupMenu"),
        "PopupMenu struct definition missing"
    );
    assert!(
        dc.contains("mnu_main: MainMenu { images: \"iml_icons\".to_string() }"),
        "MainMenu default initialization missing"
    );
    assert!(
        dc.contains("pop_ctx: PopupMenu { images: \"iml_icons\".to_string() }"),
        "PopupMenu default initialization missing"
    );

    // Action wiring in designer
    assert!(
        dc.contains("window.on_crd_box_clicked"),
        "Card on_click wiring missing"
    );
    assert!(
        dc.contains("f.form.crd_click();"),
        "Card on_click action invocation missing"
    );
    assert!(
        dc.contains("window.on_cat_group_header_clicked"),
        "CategoryPanel on_click wiring missing"
    );
    assert!(
        dc.contains("f.form.cat_collapse();"),
        "CategoryPanel on_collapsed action missing"
    );
    assert!(
        dc.contains("f.form.cat_expand();"),
        "CategoryPanel on_expanded action missing"
    );
    assert!(
        dc.contains("window.on_stb_foot_clicked"),
        "StatusBar on_click wiring missing"
    );
    assert!(
        dc.contains("f.form.stb_click();"),
        "StatusBar on_click action invocation missing"
    );
    assert!(
        dc.contains("window.on_mni_file_clicked"),
        "MenuItem on_click wiring missing"
    );
    assert!(
        dc.contains("f.form.file_click();"),
        "MenuItem on_click action invocation missing"
    );
}

#[test]
fn test_slint_slice4_components() {
    let mut doc = Document::new("FormInputs", "Form");
    if let Some(form) = doc.node_mut("FormInputs") {
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        form.set_name("FormInputs");
        form.set_text("Inputs Form");
    }

    // 1. ColorBox
    let mut clb = Node::new("clb_pick", "ColorBox");
    clb.set_name("clb_pick");
    clb.set_rect(Rect::new(10.0, 10.0, 145.0, 24.0));
    clb.props.insert(
        "selected".to_string(),
        crawrad_ir::Value::Color(crawrad_ir::Color::rgb(255, 0, 0)),
    );
    clb.events
        .insert("on_change".to_string(), "clb_change".to_string());
    let _ = doc.add_child("FormInputs", clb);

    // 2. DateTimePicker
    let mut dtp = Node::new("dtp_sched", "DateTimePicker");
    dtp.set_name("dtp_sched");
    dtp.set_rect(Rect::new(10.0, 45.0, 140.0, 24.0));
    dtp.props.insert(
        "kind".to_string(),
        crawrad_ir::Value::Str("dtkDate".to_string()),
    );
    dtp.props.insert(
        "date_time".to_string(),
        crawrad_ir::Value::Str("2026-11-27".to_string()),
    );
    dtp.events
        .insert("on_change".to_string(), "dtp_change".to_string());
    let _ = doc.add_child("FormInputs", dtp);

    // 3. MaskEdit
    let mut msk = Node::new("msk_phone", "MaskEdit");
    msk.set_name("msk_phone");
    msk.set_rect(Rect::new(10.0, 80.0, 120.0, 24.0));
    msk.props.insert(
        "text".to_string(),
        crawrad_ir::Value::Str("1199998888".to_string()),
    );
    msk.props.insert(
        "edit_mask".to_string(),
        crawrad_ir::Value::Str("(99) 9999-9999".to_string()),
    );
    msk.events
        .insert("on_change".to_string(), "msk_change".to_string());
    msk.events
        .insert("on_return".to_string(), "msk_return".to_string());
    let _ = doc.add_child("FormInputs", msk);

    // 4. MultiSelect
    let mut mse = Node::new("mse_tags", "MultiSelect");
    mse.set_name("mse_tags");
    mse.set_rect(Rect::new(10.0, 115.0, 180.0, 24.0));
    mse.props.insert(
        "items".to_string(),
        crawrad_ir::Value::List(vec![
            crawrad_ir::Value::Str("Rust".to_string()),
            crawrad_ir::Value::Str("Slint".to_string()),
            crawrad_ir::Value::Str("CrawRAD".to_string()),
        ]),
    );
    mse.props.insert(
        "selected_items".to_string(),
        crawrad_ir::Value::List(vec![
            crawrad_ir::Value::Str("Rust".to_string()),
            crawrad_ir::Value::Str("Slint".to_string()),
        ]),
    );
    mse.props.insert(
        "text_hint".to_string(),
        crawrad_ir::Value::Str("Choose tags...".to_string()),
    );
    mse.events
        .insert("on_change".to_string(), "mse_change".to_string());
    let _ = doc.add_child("FormInputs", mse);

    let generator = GeradorSlint;
    let files = generator
        .gerar(&doc, "my_app", &crawrad_registry::Registry::builtin())
        .expect("must generate Slice 4 components successfully");

    let slint_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormInputs.slint")
        .unwrap();
    let sc = &slint_file.conteudo;

    // ColorBox assertions
    assert!(
        sc.contains("clb_pick := Rectangle"),
        "ColorBox element missing"
    );
    assert!(sc.contains("width: 145.0px;"), "ColorBox width missing");
    assert!(sc.contains("height: 24.0px;"), "ColorBox height missing");
    assert!(
        sc.contains("clb_pick_swatch := Rectangle"),
        "ColorBox swatch missing"
    );
    assert!(
        sc.contains("background: #FF0000;"),
        "ColorBox swatch color missing"
    );
    assert!(
        sc.contains("border-radius: 2px;"),
        "ColorBox swatch border-radius missing"
    );
    assert!(
        sc.contains("clb_pick_combo := ComboBox"),
        "ColorBox combo missing"
    );
    assert!(
        sc.contains("current-value: \"Red\";"),
        "ColorBox combo current-value missing"
    );
    assert!(
        sc.contains("selected(val) => { root.clb_pick_changed(val); }"),
        "ColorBox selected callback missing"
    );
    assert!(
        sc.contains("callback clb_pick_changed(string);"),
        "ColorBox callback declaration missing"
    );

    // DateTimePicker assertions
    assert!(
        sc.contains("dtp_sched := Rectangle"),
        "DateTimePicker element missing"
    );
    assert!(
        sc.contains("width: 140.0px;"),
        "DateTimePicker width missing"
    );
    assert!(
        sc.contains("height: 24.0px;"),
        "DateTimePicker height missing"
    );
    assert!(
        sc.contains("dtp_sched_input := LineEdit"),
        "DateTimePicker LineEdit missing"
    );
    assert!(
        sc.contains("edited(val) => { root.dtp_sched_edited(val); }"),
        "DateTimePicker edited wiring missing"
    );
    assert!(
        sc.contains("text: \"📅\";"),
        "DateTimePicker date icon missing"
    );
    assert!(
        sc.contains("callback dtp_sched_edited(string);"),
        "DateTimePicker callback declaration missing"
    );

    // MaskEdit assertions
    assert!(
        sc.contains("msk_phone := LineEdit"),
        "MaskEdit element missing"
    );
    assert!(sc.contains("width: 120.0px;"), "MaskEdit width missing");
    assert!(sc.contains("height: 24.0px;"), "MaskEdit height missing");
    assert!(
        sc.contains("text: \"1199998888\";"),
        "MaskEdit text missing"
    );
    assert!(
        sc.contains("placeholder-text: \"(99) 9999-9999\";"),
        "MaskEdit placeholder-text missing"
    );
    assert!(
        sc.contains("edited(val) => { root.msk_phone_edited(val); }"),
        "MaskEdit edited wiring missing"
    );
    assert!(
        sc.contains("accepted => { root.msk_phone_accepted(); }"),
        "MaskEdit accepted wiring missing"
    );
    assert!(
        sc.contains("callback msk_phone_edited(string);"),
        "MaskEdit edited callback declaration missing"
    );
    assert!(
        sc.contains("callback msk_phone_accepted();"),
        "MaskEdit accepted callback declaration missing"
    );

    // MultiSelect assertions
    assert!(
        sc.contains("mse_tags := Rectangle"),
        "MultiSelect element missing"
    );
    assert!(sc.contains("width: 180.0px;"), "MultiSelect width missing");
    assert!(sc.contains("height: 24.0px;"), "MultiSelect height missing");
    assert!(
        sc.contains("mse_tags_summary := Text"),
        "MultiSelect summary text element missing"
    );
    assert!(
        sc.contains("text: \"Rust, Slint\";"),
        "MultiSelect selected items display missing"
    );
    assert!(
        sc.contains("color: #1c1b1f;"),
        "MultiSelect text color missing"
    );
    assert!(
        sc.contains("text: \"▼\";"),
        "MultiSelect dropdown arrow missing"
    );
    assert!(
        sc.contains("clicked => { root.mse_tags_clicked(); }"),
        "MultiSelect clicked wiring missing"
    );
    assert!(
        sc.contains("callback mse_tags_clicked();"),
        "MultiSelect callback declaration missing"
    );

    // Designer file assertions
    let designer_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormInputs.designer.rs")
        .unwrap();
    let dc = &designer_file.conteudo;

    assert!(
        dc.contains("pub clb_pick: ColorBox,"),
        "ColorBox field missing in designer struct"
    );
    assert!(
        dc.contains("pub dtp_sched: DateTimePicker,"),
        "DateTimePicker field missing in designer struct"
    );
    assert!(
        dc.contains("pub msk_phone: MaskEdit,"),
        "MaskEdit field missing in designer struct"
    );
    assert!(
        dc.contains("pub mse_tags: MultiSelect,"),
        "MultiSelect field missing in designer struct"
    );

    assert!(
        dc.contains("pub struct ColorBox"),
        "ColorBox struct definition missing"
    );
    assert!(
        dc.contains("pub selected: crawrad_ir::Color,"),
        "ColorBox selected field missing"
    );
    assert!(
        dc.contains("CRAWRAD_STANDARD_COLORS"),
        "CRAWRAD_STANDARD_COLORS constant missing"
    );
    assert!(
        dc.contains("pub struct DateTimePicker"),
        "DateTimePicker struct definition missing"
    );
    assert!(
        dc.contains("pub kind: String,"),
        "DateTimePicker kind field missing"
    );
    assert!(
        dc.contains("pub date_time: String,"),
        "DateTimePicker date_time field missing"
    );
    assert!(
        dc.contains("pub struct MaskEdit"),
        "MaskEdit struct definition missing"
    );
    assert!(
        dc.contains("pub edit_mask: String,"),
        "MaskEdit edit_mask field missing"
    );
    assert!(
        dc.contains("pub struct MultiSelect"),
        "MultiSelect struct definition missing"
    );
    assert!(
        dc.contains("pub selected_items: Strings,"),
        "MultiSelect selected_items field missing"
    );

    // Action wirings
    assert!(
        dc.contains("window.on_clb_pick_changed"),
        "ColorBox on_changed wiring missing"
    );
    assert!(
        dc.contains("f.form.clb_pick.selected = *c;"),
        "ColorBox selection update missing"
    );
    assert!(
        dc.contains("f.form.clb_change();"),
        "ColorBox on_change action invocation missing"
    );
    assert!(
        dc.contains("window.on_dtp_sched_edited"),
        "DateTimePicker on_edited wiring missing"
    );
    assert!(
        dc.contains("f.form.dtp_change();"),
        "DateTimePicker on_change action invocation missing"
    );
    assert!(
        dc.contains("window.on_msk_phone_edited"),
        "MaskEdit on_edited wiring missing"
    );
    assert!(
        dc.contains("f.form.msk_change();"),
        "MaskEdit on_change action invocation missing"
    );
    assert!(
        dc.contains("window.on_msk_phone_accepted"),
        "MaskEdit on_accepted wiring missing"
    );
    assert!(
        dc.contains("f.form.msk_return();"),
        "MaskEdit on_return action invocation missing"
    );
    assert!(
        dc.contains("window.on_mse_tags_clicked"),
        "MultiSelect on_clicked wiring missing"
    );
    assert!(
        dc.contains("f.form.mse_change();"),
        "MultiSelect on_change action invocation missing"
    );
}

#[test]
fn test_slint_slice5_components() {
    let mut doc = Document::new("FormDataBound", "Form");
    if let Some(form) = doc.node_mut("FormDataBound") {
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        form.set_name("FormDataBound");
        form.set_text("Data Bound Form");
    }

    // 1. ActionList & Action
    let mut acl = Node::new("acl_main", "ActionList");
    acl.set_name("acl_main");
    acl.props.insert(
        "images".to_string(),
        crawrad_ir::Value::Ref("iml_icons".to_string()),
    );
    let _ = doc.add_child("FormDataBound", acl);

    let mut act = Node::new("act_open", "Action");
    act.set_name("act_open");
    act.props.insert(
        "text".to_string(),
        crawrad_ir::Value::Str("Open".to_string()),
    );
    act.props.insert(
        "category".to_string(),
        crawrad_ir::Value::Str("File".to_string()),
    );
    let _ = doc.add_child("acl_main", act);

    // 2. ImageList
    let mut iml = Node::new("iml_icons", "ImageList");
    iml.set_name("iml_icons");
    iml.props
        .insert("image_width".to_string(), crawrad_ir::Value::Int(24));
    iml.props
        .insert("image_height".to_string(), crawrad_ir::Value::Int(24));
    let mut icon_map = std::collections::BTreeMap::new();
    icon_map.insert(
        "name".to_string(),
        crawrad_ir::Value::Str("save".to_string()),
    );
    icon_map.insert(
        "symbol".to_string(),
        crawrad_ir::Value::Str("floppy".to_string()),
    );
    iml.props.insert(
        "images_data".to_string(),
        crawrad_ir::Value::List(vec![crawrad_ir::Value::Map(icon_map)]),
    );
    let _ = doc.add_child("FormDataBound", iml);

    // 3. CheckListBox
    let mut clb = Node::new("clb_tasks", "CheckListBox");
    clb.set_name("clb_tasks");
    clb.set_rect(Rect::new(10.0, 10.0, 150.0, 120.0));
    clb.props.insert(
        "items".to_string(),
        crawrad_ir::Value::List(vec![
            crawrad_ir::Value::Str("Setup".to_string()),
            crawrad_ir::Value::Str("Build".to_string()),
            crawrad_ir::Value::Str("Deploy".to_string()),
        ]),
    );
    clb.props.insert(
        "checked_items".to_string(),
        crawrad_ir::Value::List(vec![
            crawrad_ir::Value::Str("Setup".to_string()),
            crawrad_ir::Value::Str("Build".to_string()),
        ]),
    );
    clb.events
        .insert("on_change".to_string(), "clb_task_change".to_string());
    let _ = doc.add_child("FormDataBound", clb);

    // 4. ListView with ListColumn
    let mut lvw = Node::new("lvw_files", "ListView");
    lvw.set_name("lvw_files");
    lvw.set_rect(Rect::new(170.0, 10.0, 220.0, 150.0));
    lvw.props.insert(
        "items".to_string(),
        crawrad_ir::Value::List(vec![crawrad_ir::Value::Map(
            std::collections::BTreeMap::from([
                (
                    "text".to_string(),
                    crawrad_ir::Value::Str("main.rs".to_string()),
                ),
                (
                    "children".to_string(),
                    crawrad_ir::Value::List(vec![crawrad_ir::Value::Map(
                        std::collections::BTreeMap::from([(
                            "text".to_string(),
                            crawrad_ir::Value::Str("4 KB".to_string()),
                        )]),
                    )]),
                ),
            ]),
        )]),
    );
    lvw.events
        .insert("on_change".to_string(), "lvw_change".to_string());
    lvw.events
        .insert("on_click".to_string(), "lvw_click".to_string());
    lvw.events
        .insert("on_dbl_click".to_string(), "lvw_dbl_click".to_string());
    let _ = doc.add_child("FormDataBound", lvw);

    let mut col1 = Node::new("col_name", "ListColumn");
    col1.set_name("col_name");
    col1.props.insert(
        "text".to_string(),
        crawrad_ir::Value::Str("Name".to_string()),
    );
    col1.props
        .insert("width".to_string(), crawrad_ir::Value::Int(120));
    let _ = doc.add_child("lvw_files", col1);

    let mut col2 = Node::new("col_size", "ListColumn");
    col2.set_name("col_size");
    col2.props.insert(
        "text".to_string(),
        crawrad_ir::Value::Str("Size".to_string()),
    );
    col2.props
        .insert("width".to_string(), crawrad_ir::Value::Int(80));
    let _ = doc.add_child("lvw_files", col2);

    // 5. TreeView
    let mut trv = Node::new("trv_tree", "TreeView");
    trv.set_name("trv_tree");
    trv.set_rect(Rect::new(400.0, 10.0, 160.0, 140.0));
    trv.props.insert(
        "items".to_string(),
        crawrad_ir::Value::List(vec![crawrad_ir::Value::Map(
            std::collections::BTreeMap::from([
                (
                    "text".to_string(),
                    crawrad_ir::Value::Str("Root".to_string()),
                ),
                (
                    "children".to_string(),
                    crawrad_ir::Value::List(vec![crawrad_ir::Value::Map(
                        std::collections::BTreeMap::from([(
                            "text".to_string(),
                            crawrad_ir::Value::Str("Child".to_string()),
                        )]),
                    )]),
                ),
            ]),
        )]),
    );
    trv.events
        .insert("on_change".to_string(), "trv_change".to_string());
    trv.events
        .insert("on_click".to_string(), "trv_click".to_string());
    let _ = doc.add_child("FormDataBound", trv);

    // 6. Grid
    let mut grd = Node::new("grd_data", "Grid");
    grd.set_name("grd_data");
    grd.set_rect(Rect::new(10.0, 180.0, 320.0, 110.0));
    grd.props
        .insert("col_count".to_string(), crawrad_ir::Value::Int(4));
    grd.props
        .insert("row_count".to_string(), crawrad_ir::Value::Int(4));
    grd.props
        .insert("default_col_width".to_string(), crawrad_ir::Value::Int(80));
    grd.props
        .insert("default_row_height".to_string(), crawrad_ir::Value::Int(26));
    grd.events
        .insert("on_click".to_string(), "grd_click".to_string());
    let _ = doc.add_child("FormDataBound", grd);

    let generator = GeradorSlint;
    let files = generator
        .gerar(&doc, "my_app", &crawrad_registry::Registry::builtin())
        .expect("must generate Slice 5 components successfully");

    let slint_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormDataBound.slint")
        .unwrap();
    let sc = &slint_file.conteudo;

    // CheckListBox assertions
    assert!(
        sc.contains("clb_tasks := Rectangle"),
        "CheckListBox element missing"
    );
    assert!(sc.contains("width: 150.0px;"), "CheckListBox width missing");
    assert!(
        sc.contains("height: 120.0px;"),
        "CheckListBox height missing"
    );
    assert!(
        sc.contains("clb_tasks_chk_0 := CheckBox"),
        "CheckListBox checkbox element missing"
    );
    assert!(
        sc.contains("checked: true;"),
        "CheckListBox checkbox checked status missing"
    );
    assert!(
        sc.contains("text: \"Setup\";"),
        "CheckListBox item text missing"
    );
    assert!(
        sc.contains("toggled => { root.clb_tasks_toggled(0, self.checked); }"),
        "CheckListBox toggled wiring missing"
    );
    assert!(
        sc.contains("callback clb_tasks_toggled(int, bool);"),
        "CheckListBox callback declaration missing"
    );

    // ListView assertions
    assert!(
        sc.contains("lvw_files := Rectangle"),
        "ListView element missing"
    );
    assert!(sc.contains("width: 220.0px;"), "ListView width missing");
    assert!(sc.contains("height: 150.0px;"), "ListView height missing");
    assert!(
        sc.contains("lvw_files_header := Rectangle"),
        "ListView header missing"
    );
    assert!(
        sc.contains("text: \"Name\";"),
        "ListView column header Name missing"
    );
    assert!(
        sc.contains("text: \"Size\";"),
        "ListView column header Size missing"
    );
    assert!(
        sc.contains("width: 120px;"),
        "ListView column width 120 missing"
    );
    assert!(
        sc.contains("width: 80px;"),
        "ListView column width 80 missing"
    );
    assert!(
        sc.contains("text: \"main.rs\";"),
        "ListView item text main.rs missing"
    );
    assert!(
        sc.contains("text: \"4 KB\";"),
        "ListView sub-item text 4 KB missing"
    );
    assert!(
        sc.contains("clicked => { root.lvw_files_selected(0); }"),
        "ListView clicked selection missing"
    );
    assert!(
        sc.contains("double-clicked => { root.lvw_files_double_clicked(0); }"),
        "ListView double-clicked missing"
    );
    assert!(
        sc.contains("callback lvw_files_selected(int);"),
        "ListView selected callback declaration missing"
    );
    assert!(
        sc.contains("callback lvw_files_double_clicked(int);"),
        "ListView double_clicked callback declaration missing"
    );

    // TreeView assertions
    assert!(
        sc.contains("trv_tree := Rectangle"),
        "TreeView element missing"
    );
    assert!(sc.contains("width: 160.0px;"), "TreeView width missing");
    assert!(sc.contains("height: 140.0px;"), "TreeView height missing");
    assert!(
        sc.contains("trv_tree_node_0 := Rectangle"),
        "TreeView node row missing"
    );
    assert!(sc.contains("text: \"Root\";"), "TreeView Root text missing");
    assert!(
        sc.contains("clicked => { root.trv_tree_node_clicked(0); }"),
        "TreeView clicked wiring missing"
    );
    assert!(
        sc.contains("callback trv_tree_node_clicked(int);"),
        "TreeView callback declaration missing"
    );

    // Grid assertions
    assert!(sc.contains("grd_data := Rectangle"), "Grid element missing");
    assert!(sc.contains("width: 320.0px;"), "Grid width missing");
    assert!(sc.contains("height: 110.0px;"), "Grid height missing");
    assert!(
        sc.contains("grd_data_row_0 := Rectangle"),
        "Grid row missing"
    );
    assert!(sc.contains("height: 26px;"), "Grid row height missing");
    assert!(
        sc.contains("grd_data_cell_0_0 := Rectangle"),
        "Grid cell missing"
    );
    assert!(sc.contains("width: 80px;"), "Grid cell width missing");
    assert!(
        sc.contains("clicked => { root.grd_data_cell_selected(0, 0); }"),
        "Grid cell selected wiring missing"
    );
    assert!(
        sc.contains("callback grd_data_cell_selected(int, int);"),
        "Grid callback declaration missing"
    );

    // Designer file assertions
    let designer_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormDataBound.designer.rs")
        .unwrap();
    let dc = &designer_file.conteudo;

    assert!(
        dc.contains("pub acl_main: ActionList,"),
        "ActionList field missing in designer struct"
    );
    assert!(
        dc.contains("pub struct ActionList"),
        "ActionList struct definition missing"
    );
    assert!(
        dc.contains("pub act_open: Action,"),
        "Action field missing in designer struct"
    );
    assert!(
        dc.contains("pub struct Action"),
        "Action struct definition missing"
    );
    assert!(
        dc.contains("pub iml_icons: ImageList,"),
        "ImageList field missing in designer struct"
    );
    assert!(
        dc.contains("pub struct ImageList"),
        "ImageList struct definition missing"
    );
    assert!(
        dc.contains("pub struct ImageItem"),
        "ImageItem struct definition missing"
    );
    assert!(
        dc.contains("iml_icons: ImageList { image_width: 24, image_height: 24, images: vec![ImageItem { name: \"save\".to_string(), symbol: \"floppy\".to_string() }] }"),
        "ImageList default initialization missing"
    );
    assert!(
        dc.contains("pub clb_tasks: CheckListBox,"),
        "CheckListBox field missing in designer struct"
    );
    assert!(
        dc.contains("pub struct CheckListBox"),
        "CheckListBox struct definition missing"
    );
    assert!(
        dc.contains("pub lvw_files: ListView,"),
        "ListView field missing in designer struct"
    );
    assert!(
        dc.contains("pub struct ListView"),
        "ListView struct definition missing"
    );
    assert!(
        dc.contains("pub struct ListViewItem"),
        "ListViewItem struct definition missing"
    );
    assert!(
        dc.contains("pub col_name: ListColumn,"),
        "ListColumn field missing in designer struct"
    );
    assert!(
        dc.contains("pub struct ListColumn"),
        "ListColumn struct definition missing"
    );
    assert!(
        dc.contains("pub trv_tree: TreeView,"),
        "TreeView field missing in designer struct"
    );
    assert!(
        dc.contains("pub struct TreeView"),
        "TreeView struct definition missing"
    );
    assert!(
        dc.contains("pub grd_data: Grid,"),
        "Grid field missing in designer struct"
    );
    assert!(
        dc.contains("pub struct Grid"),
        "Grid struct definition missing"
    );

    // Action wirings in designer
    assert!(
        dc.contains("window.on_clb_tasks_toggled"),
        "CheckListBox on_toggled wiring missing"
    );
    assert!(
        dc.contains("f.form.clb_task_change();"),
        "CheckListBox on_change action invocation missing"
    );
    assert!(
        dc.contains("window.on_lvw_files_selected"),
        "ListView on_selected wiring missing"
    );
    assert!(
        dc.contains("f.form.lvw_change();"),
        "ListView on_change action invocation missing"
    );
    assert!(
        dc.contains("f.form.lvw_click();"),
        "ListView on_click action invocation missing"
    );
    assert!(
        dc.contains("window.on_lvw_files_double_clicked"),
        "ListView on_double_clicked wiring missing"
    );
    assert!(
        dc.contains("f.form.lvw_dbl_click();"),
        "ListView on_dbl_click action invocation missing"
    );
    assert!(
        dc.contains("window.on_trv_tree_node_clicked"),
        "TreeView on_node_clicked wiring missing"
    );
    assert!(
        dc.contains("f.form.trv_change();"),
        "TreeView on_change action invocation missing"
    );
    assert!(
        dc.contains("f.form.trv_click();"),
        "TreeView on_click action invocation missing"
    );
    assert!(
        dc.contains("window.on_grd_data_cell_selected"),
        "Grid on_cell_selected wiring missing"
    );
    assert!(
        dc.contains("f.form.grd_click();"),
        "Grid on_click action invocation missing"
    );
}

#[test]
fn test_slint_border_radius_property() {
    let mut doc = Document::new("FormRadius", "Form");
    {
        let form = doc.node_mut("FormRadius").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        form.set_name("FormRadius");
    }

    let mut grd = Node::new("grd_custom", "Grid");
    grd.set_name("grd_custom");
    grd.set_rect(Rect::new(10.0, 10.0, 300.0, 200.0));
    grd.props
        .insert("slint.border-radius".to_string(), Value::Int(10));
    let _ = doc.add_child("FormRadius", grd);

    let mut grd_default = Node::new("grd_default", "Grid");
    grd_default.set_name("grd_default");
    grd_default.set_rect(Rect::new(10.0, 220.0, 300.0, 200.0));
    let _ = doc.add_child("FormRadius", grd_default);

    let generator = GeradorSlint;
    let files = generator
        .gerar(&doc, "radius_app", &crawrad_registry::Registry::builtin())
        .expect("must generate form with border-radius property");

    let slint_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/FormRadius.slint")
        .unwrap();
    let sc = &slint_file.conteudo;

    assert!(
        sc.contains("border-radius: 10.0px;"),
        "Custom slint.border-radius = 10 must emit border-radius: 10.0px;"
    );
    assert!(
        sc.contains("border-radius: 4px;"),
        "Default node without slint.border-radius must fallback to border-radius: 4px;"
    );
}

use crate::targets::slint::CLASSES;
use slint_interpreter::Compiler;
use std::future::Future;
use std::path::PathBuf;
use std::pin::pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut cx = Context::from_waker(&waker);
    let mut future = pin!(future);
    loop {
        if let Poll::Ready(value) = future.as_mut().poll(&mut cx) {
            return value;
        }
    }
}

fn diagnostics(source: &str) -> Vec<String> {
    let compiler = Compiler::default();
    let result =
        block_on(compiler.build_from_source(source.to_string(), PathBuf::from("form.slint")));
    result.diagnostics().map(|d| format!("{d}")).collect()
}

fn valid_form_doc() -> Document {
    let mut doc = Document::new("Form1", "Form");
    let form = doc.node_mut("Form1").unwrap();
    form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
    form.set_name("Form1");
    form.set_text("Form");
    doc
}

fn add_valid_node(doc: &mut Document, class: &str, index: usize) {
    let id = format!("c{index}");
    let mut node = Node::new(&id, class);
    node.set_name(&id);
    let reg = crawrad_registry::Registry::builtin();
    if let Some(desc) = reg.get(class)
        && desc.props.iter().any(|p| p.key == "text")
    {
        node.set_text(class);
    }
    node.set_rect(Rect::new(10.0, 10.0 + 30.0 * index as f32, 120.0, 24.0));
    let _ = doc.add_child("Form1", node);
}

fn valid_markup(doc: &Document) -> String {
    let reg = crawrad_registry::Registry::builtin();
    let files: Vec<crate::Arquivo> = GeradorSlint.gerar(doc, "app", &reg).expect("generate");
    let slint = files
        .iter()
        .find(|f| f.caminho.extension().is_some_and(|e| e == "slint"))
        .expect("a .slint file");
    slint.conteudo.clone()
}

#[test]
fn every_class_generates_markup_the_slint_compiler_accepts() {
    let mut failures = Vec::new();
    for (i, class) in CLASSES.iter().enumerate() {
        let mut doc = valid_form_doc();
        add_valid_node(&mut doc, class, i);
        let source = valid_markup(&doc);
        let diags = diagnostics(&source);
        if !diags.is_empty() {
            failures.push(format!(
                "== {class} ==\n{}\n-- markup --\n{source}",
                diags.join("\n")
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

#[test]
fn a_form_carrying_every_class_compiles_together() {
    let mut doc = valid_form_doc();
    for (i, class) in CLASSES.iter().enumerate() {
        add_valid_node(&mut doc, class, i);
    }
    let source = valid_markup(&doc);
    let diags = diagnostics(&source);
    assert!(
        diags.is_empty(),
        "{}\n-- markup --\n{source}",
        diags.join("\n")
    );
}

#[test]
fn special_characters_in_captions_compile_cleanly() {
    for (name, caption) in [
        ("newline", "Line1\nLine2"),
        ("tab", "Col1\tCol2"),
        ("cr", "Carriage\rReturn"),
        ("crlf", "Windows\r\nNewline"),
        ("brace", "Contains {brace}"),
        ("quote", "Contains \"quoted\" text"),
        ("backslash", "Contains \\ backslash"),
    ] {
        let mut doc = valid_form_doc();
        let mut btn = Node::new(name, "Button");
        btn.set_name(name);
        btn.set_text(caption);
        btn.set_rect(Rect::new(10.0, 10.0, 120.0, 24.0));
        let _ = doc.add_child("Form1", btn);
        let source = valid_markup(&doc);
        let diags = diagnostics(&source);
        assert!(
            diags.is_empty(),
            "Failed for {name}: {}\n-- markup --\n{source}",
            diags.join("\n")
        );
    }
}
