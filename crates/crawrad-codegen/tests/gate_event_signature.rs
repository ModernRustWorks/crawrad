//! **The event signature gate.** The handler the IDE writes on double-click
//! is born with the parameter declared by the catalog — and the project
//! compiles with it.
//!
//! # Why this gate exists when two others already compile a project
//!
//! Because neither of them reaches this bug, and this was measured before
//! writing a line:
//!
//! - `gate_fase3` wires an `OnClick`, an `OnTimer`, an `OnCreate` and an
//!   `OnClose`. None of them has parameters;
//! - `gate_sem_avisos` scans the entire catalog, but only wires the event that
//!   the generator **calls** (`vira_chamada`) — and none of the eight events
//!   with parameters is called by generated code today. They are left out by
//!   construction;
//! - both compile **egui**. Up to this point, there was no gate that compiled
//!   a makepad project.
//!
//! The bug, therefore, was **invisible to the gate**: a stub with hardcoded
//! `&mut self` compiles because nobody calls it. It only shows up when the user
//! writes the body and the parameter is not there — in the user's face, not
//! ours.
//!
//! # What this gate does, and why the body is written like this
//!
//! It generates the form, **does not rewrite the signature** — it keeps what
//! the generator emitted — and splices into each handler body one line per
//! parameter:
//!
//! ```ignore
//! let _prova: KeyEvent = _e;
//! ```
//!
//! Such a line enforces three things at once, and that is why it is a
//! re-binding with an annotated type instead of `let _ = _e;`:
//!
//! 1. the parameter **exists** with the declared name;
//! 2. it has the declared **type**, not one that merely looks like it;
//! 3. this type is **in scope in the user file** — catching missing `use`
//!    statements, types the designer did not emit, and targets that emitted
//!    something else.
//!
//! # The cost, measured
//!
//! The makepad project pulls in `makepad-widgets`, and the initial compilation
//! is expensive. The draft stays in `target/` and is reused between runs under
//! the same rule as sibling gates — and for the same reason: a warm `target/`
//! makes the second run cheap.
//!
//! ```text
//! cargo test -p crawrad-codegen --test gate_event_signature -- --ignored --nocapture
//! ```

mod comum;

use comum::manifesto_intocado;
use crawrad_codegen::Gerador as _;
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;
use std::path::{Path, PathBuf};
use std::process::Command;

// **The self-destructing waiver, and the day it happened.**
//
// Here lived `CARGA_SEM_QUEM_CONSTRUA`: a list of five payload types that
// the designer emitted because a handler named them and that **no line of
// generated code constructed**. `ShiftState`, `MouseEvent`, `MouseMoveEvent`,
// `KeyEvent` and `MouseActivateEvent` were born `never constructed`, and the gate
// waived them — with the debt documented alongside, enforced in both directions.
//
// **17/09/2026: the debt was paid and the list removed.** The egui target began
// dispatching form events (`targets/egui/form_events.rs`), the five types
// started being constructed, and the reverse check itself —
// *"line whose warning does not appear → failure telling you to delete it"* —
// was what notified us. It worked exactly as designed.
//
// What replaces it is the standard without exception: **any** `never
// constructed` fails. A waiver without debt authorizes the next bug to be born
// silent, which is what the designer's `#![allow(dead_code)]` did for years.

/// The GROUP root, written before `cargo new` — as the IDE does (ADR 0018).
const RAIZ_DO_GRUPO: &str = r#"[workspace]
members = ["app"]
resolver = "2"
default-members = ["app"]
"#;

/// The two targets, with what each project needs in `Cargo.toml`.
struct Alvo {
    nome: &'static str,
    /// The crate that `cargo add` adds, by which the draft guard recognizes
    /// that the member was already set up.
    dependencia: &'static str,
}

const ALVOS: &[Alvo] = &[
    Alvo {
        nome: "egui",
        dependencia: "eframe@0.36",
    },
    Alvo {
        nome: "makepad",
        dependencia: "makepad-widgets@1.0",
    },
];

fn gerador(alvo: &str) -> Box<dyn crawrad_codegen::Gerador> {
    match alvo {
        "egui" => Box::new(crawrad_codegen::GeradorRxp),
        "makepad" => Box::new(crawrad_codegen::GeradorMakepad),
        outro => panic!("unknown target: {outro}"),
    }
}

/// **The test case form.** The root declares the eight events with parameters —
/// all of them —, and `Button` declares the seven that are not from the form.
///
/// Both sides are here on purpose: the ROOT event path and the COMPONENT
/// event path are different parts of the generator, and one has passed green
/// before while the other was broken.
fn documento() -> Document {
    let r = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("newly created root");
        raiz.set_rect(Rect::new(0.0, 0.0, 1000.0, 660.0));
        raiz.set_name("form_main");
        raiz.set_text("Form1");
        // The form's Color and Position (WindowState is proven by the
        // codegen tests; maximized here, it would move every proof window)
        raiz.props.insert(
            "color".into(),
            crawrad_ir::Value::Color(crawrad_ir::Color::rgb(0x1e, 0x2a, 0x3a)),
        );
        raiz.props.insert(
            "position".into(),
            crawrad_ir::Value::Enum("poScreenCenter".into()),
        );
    }
    for (classe, id) in [("Form", "form_main"), ("Button", "btn_prova")] {
        let desc = r.get(classe).expect("catalog class");
        let eventos: Vec<&'static str> = desc
            .events
            .iter()
            .filter(|e| e.method_params() != "&mut self")
            .map(|e| e.name)
            .collect();
        assert!(
            !eventos.is_empty(),
            "`{classe}` does not declare an event with parameters — test stopped measuring"
        );
        if classe != "Form" {
            let (w, h) = desc.default_size;
            let mut n = r
                .instantiate(classe, id, Rect::new(8.0, 8.0, w, h))
                .expect("catalog instantiates what it lists");
            n.set_name(id);
            d.add_child("form_main", n).expect("into the form");
        }
        for e in eventos {
            let handler = crawrad_codegen::handler_padrao(id, e);
            d.node_mut(id)
                .expect("newly placed node")
                .events
                .insert(e.into(), handler);
        }
    }
    // the ComboBox's pick: OnSelect and OnChange share makepad's action, and
    // only a build proves the emitted Makepad code
    {
        let (w, h) = r.get("ComboBox").expect("the class").default_size;
        let mut n = r
            .instantiate("ComboBox", "cbo_events", Rect::new(8.0, 600.0, w, h))
            .expect("instantiates");
        n.set_name("cbo_events");
        for e in ["on_change", "on_select"] {
            n.events
                .insert(e.into(), crawrad_codegen::handler_padrao("cbo_events", e));
        }
        d.add_child("form_main", n).expect("into the form");
    }
    // the ListView's row and header events: only a build proves the Makepad
    // calls match the handlers' parameters
    {
        let (w, h) = r.get("ListView").expect("the class").default_size;
        let mut n = r
            .instantiate("ListView", "lvw_events", Rect::new(8.0, 700.0, w, h))
            .expect("instantiates");
        n.set_name("lvw_events");
        for e in [
            "on_select_item",
            "on_column_click",
            "on_change",
            "on_dbl_click",
        ] {
            n.events
                .insert(e.into(), crawrad_codegen::handler_padrao("lvw_events", e));
        }
        d.add_child("form_main", n).expect("into the form");
    }
    // the TreeView's veto and fold events: only a build proves the Makepad
    // calls match the handlers' parameters
    {
        let (w, h) = r.get("TreeView").expect("the class").default_size;
        let mut n = r
            .instantiate("TreeView", "tv_events", Rect::new(260.0, 700.0, w, h))
            .expect("instantiates");
        n.set_name("tv_events");
        for e in ["on_changing", "on_change", "on_expanded", "on_collapsed"] {
            n.events
                .insert(e.into(), crawrad_codegen::handler_padrao("tv_events", e));
        }
        d.add_child("form_main", n).expect("into the form");
    }
    // the wheel and the resize of a Panel: only a build proves the Makepad
    // router and the size watch compile in a real project
    {
        let (w, h) = r.get("Panel").expect("the class").default_size;
        let mut n = r
            .instantiate("Panel", "pnl_events", Rect::new(480.0, 700.0, w, h))
            .expect("instantiates");
        n.set_name("pnl_events");
        for e in ["on_mouse_wheel", "on_resize"] {
            n.events
                .insert(e.into(), crawrad_codegen::handler_padrao("pnl_events", e));
        }
        d.add_child("form_main", n).expect("into the form");
    }
    // This is the gate that BUILDS a Makepad project, so the TextInput
    // flags (a password Edit, a read-only Memo) and their run time sync are
    // compiled here, on both targets
    for (classe, id, prop, valor, y, h) in [
        (
            "Edit",
            "edt_password",
            "password_char",
            crawrad_ir::Value::Str("*".into()),
            60.0,
            24.0,
        ),
        (
            "Memo",
            "mmo_read_only",
            "readonly",
            crawrad_ir::Value::Bool(true),
            96.0,
            60.0,
        ),
        // Disabled controls — the run time `enabled::apply` and the
        // guarded dispatch are compiled here too
        (
            "CheckBox",
            "chk_disabled",
            "enabled",
            crawrad_ir::Value::Bool(false),
            168.0,
            24.0,
        ),
        (
            "Edit",
            "edt_disabled",
            "enabled",
            crawrad_ir::Value::Bool(false),
            204.0,
            24.0,
        ),
        (
            "ProgressBar",
            "pb_load",
            "position",
            crawrad_ir::Value::Int(35),
            276.0,
            16.0,
        ),
        (
            "ProgressBar",
            "pb_up",
            "orientation",
            crawrad_ir::Value::Enum("Vertical".into()),
            300.0,
            48.0,
        ),
        (
            "RadioButton",
            "rad_disabled",
            "enabled",
            crawrad_ir::Value::Bool(false),
            240.0,
            24.0,
        ),
    ] {
        let mut n = r
            .instantiate(classe, id, Rect::new(8.0, y, 160.0, h))
            .expect("catalog instantiates");
        n.set_name(id);
        n.props.insert(prop.into(), valor);
        d.add_child("form_main", n).expect("into the form");
    }
    // A Timer that ticks every 200 ms into its OnTimer, and a Label for
    // the handler to count on — the App's Ticker and the tick are compiled here
    let mut tmr = r
        .instantiate("Timer", "tmr_tick", Rect::new(200.0, 8.0, 24.0, 24.0))
        .expect("catalog instantiates");
    tmr.set_name("tmr_tick");
    tmr.props
        .insert("interval".into(), crawrad_ir::Value::Int(200));
    tmr.events
        .insert("on_timer".into(), "tmr_tick_timer".into());
    d.add_child("form_main", tmr).expect("into the form");
    // An Image with a REAL picture (a 24x24 PNG), stretched in
    // proportion and centred — the picture literal and its decoding compile here
    let mut img = r
        .instantiate("Image", "img_logo", Rect::new(200.0, 100.0, 96.0, 96.0))
        .expect("catalog instantiates");
    img.set_name("img_logo");
    let png = include_bytes!("fixtures/img438.png").to_vec();
    let picture = crawrad_ir::Picture::from_bytes_with_format(png, crawrad_ir::ImageFormat::Png);
    img.props
        .insert("picture".into(), crawrad_ir::Value::Picture(picture));
    for k in ["stretch", "proportional", "center"] {
        img.props.insert(k.into(), crawrad_ir::Value::Bool(true));
    }
    d.add_child("form_main", img).expect("into the form");
    // A Glyph left of the caption, on a BitBtn and on a SpeedButton
    // (two glyphs in the picture: the first is shown); a ToolBar whose
    // SpeedButton hides its caption (ShowCaptions off) and draws flat
    let glyph = || {
        crawrad_ir::Value::Picture(crawrad_ir::Picture::from_bytes_with_format(
            include_bytes!("fixtures/img438.png").to_vec(),
            crawrad_ir::ImageFormat::Png,
        ))
    };
    for (class, id, y, w, h, caption, glyphs) in [
        ("BitBtn", "bbt_glyph", 30.0, 150.0, 32.0, "With glyph", 1),
        (
            "SpeedButton",
            "sbt_glyph",
            112.0,
            120.0,
            28.0,
            "Two glyphs",
            2,
        ),
    ] {
        let mut b = r
            .instantiate(class, id, Rect::new(830.0, y, w, h))
            .expect("catalog instantiates");
        b.set_name(id);
        b.set_text(caption);
        b.props.insert("glyph".into(), glyph());
        b.props
            .insert("num_glyphs".into(), crawrad_ir::Value::Int(glyphs));
        d.add_child("form_main", b).expect("into the form");
    }
    let mut bar = r
        .instantiate("ToolBar", "tbr_tools", Rect::new(830.0, 70.0, 150.0, 34.0))
        .expect("catalog instantiates");
    bar.set_name("tbr_tools");
    d.add_child("form_main", bar).expect("into the form");
    let mut tool = r
        .instantiate("SpeedButton", "sbt_tool", Rect::new(0.0, 5.0, 24.0, 24.0))
        .expect("catalog instantiates");
    tool.set_name("sbt_tool");
    tool.set_text("Hidden caption");
    tool.props.insert("glyph".into(), glyph());
    d.add_child("tbr_tools", tool).expect("into the toolbar");
    // A RadioGroup, four options in two columns, the second chosen;
    // a click picks another and OnChange runs
    let mut rgp = r
        .instantiate(
            "RadioGroup",
            "rgp_size",
            Rect::new(830.0, 150.0, 160.0, 90.0),
        )
        .expect("catalog instantiates");
    rgp.set_name("rgp_size");
    rgp.set_text("Size");
    rgp.props.insert(
        "items".into(),
        crawrad_ir::Item::list_to(
            &["Small", "Medium", "Large", "Huge"]
                .iter()
                .map(|t| crawrad_ir::Item::new(*t))
                .collect::<Vec<_>>(),
        ),
    );
    rgp.props
        .insert("item_index".into(), crawrad_ir::Value::Int(1));
    rgp.props
        .insert("columns".into(), crawrad_ir::Value::Int(2));
    rgp.events
        .insert("on_change".into(), "rgp_size_change".into());
    d.add_child("form_main", rgp).expect("into the form");
    // A SpinEdit, 5 in 0..10 by steps of 2; a step up runs OnChange
    let mut spn = r
        .instantiate(
            "SpinEdit",
            "spn_count",
            Rect::new(830.0, 250.0, 121.0, 24.0),
        )
        .expect("catalog instantiates");
    spn.set_name("spn_count");
    for (k, v) in [
        ("value", 5),
        ("min_value", 0),
        ("max_value", 10),
        ("increment", 2),
    ] {
        spn.props.insert(k.into(), crawrad_ir::Value::Int(v));
    }
    spn.events
        .insert("on_change".into(), "spn_count_change".into());
    d.add_child("form_main", spn).expect("into the form");
    // A Toggle, off; a click turns it on and OnChange runs
    let mut tgl = r
        .instantiate("Toggle", "tgl_dark", Rect::new(830.0, 285.0, 150.0, 24.0))
        .expect("catalog instantiates");
    tgl.set_name("tgl_dark");
    tgl.set_text("Dark mode");
    tgl.events
        .insert("on_change".into(), "tgl_dark_change".into());
    d.add_child("form_main", tgl).expect("into the form");
    // A ColorBox on Navy; choosing another colour runs OnChange
    let mut clb = r
        .instantiate(
            "ColorBox",
            "clb_theme",
            Rect::new(830.0, 385.0, 150.0, 24.0),
        )
        .expect("catalog instantiates");
    clb.set_name("clb_theme");
    clb.props.insert(
        "selected".into(),
        crawrad_ir::Value::Color(crawrad_ir::Color::rgb(0, 0, 128)),
    );
    clb.events
        .insert("on_change".into(), "clb_theme_change".into());
    d.add_child("form_main", clb).expect("into the form");
    // A LinkLabel to a web address; a click opens it and runs OnClick
    let mut lnk = r
        .instantiate(
            "LinkLabel",
            "lnk_site",
            Rect::new(830.0, 355.0, 150.0, 20.0),
        )
        .expect("catalog instantiates");
    lnk.set_name("lnk_site");
    lnk.set_text("Project site");
    lnk.props.insert(
        "url".into(),
        crawrad_ir::Value::Str("https://example.com/?a=1&b=2".into()),
    );
    lnk.events
        .insert("on_click".into(), "lnk_site_click".into());
    d.add_child("form_main", lnk).expect("into the form");
    // A Bevel framing the link and the colour box
    let mut bvl = r
        .instantiate("Bevel", "bvl_group", Rect::new(824.0, 350.0, 164.0, 66.0))
        .expect("catalog instantiates");
    bvl.set_name("bvl_group");
    bvl.props.insert(
        "bevel_shape".into(),
        crawrad_ir::Value::Enum("Frame".into()),
    );
    d.add_child("form_main", bvl).expect("into the form");
    // A ComboBox whose items the form changes at run time
    let mut cmb = r
        .instantiate(
            "ComboBox",
            "cmb_items",
            Rect::new(320.0, 100.0, 160.0, 24.0),
        )
        .expect("catalog instantiates");
    cmb.set_name("cmb_items");
    d.add_child("form_main", cmb).expect("into the form");
    // A ListBox whose rows the form adds at run time (a CrawradListBox)
    let mut lst = r
        .instantiate("ListBox", "lst_rows", Rect::new(320.0, 140.0, 160.0, 150.0))
        .expect("catalog instantiates");
    lst.set_name("lst_rows");
    d.add_child("form_main", lst).expect("into the form");
    // A TrackBar that is a real slider, its OnChange wired
    let mut tb = r
        .instantiate(
            "TrackBar",
            "tb_volume",
            Rect::new(320.0, 300.0, 200.0, 32.0),
        )
        .expect("catalog instantiates");
    tb.set_name("tb_volume");
    tb.props
        .insert("position".into(), crawrad_ir::Value::Int(3));
    tb.events
        .insert("on_change".into(), "tb_volume_change".into());
    tb.events
        .insert("on_enter".into(), "tb_volume_focus_in".into());
    d.add_child("form_main", tb).expect("into the form");
    // Labels that sit where they are told, break, fit their text, and
    // take a click and a double click through the pointer router
    for (id, text, props, y, h) in [
        (
            "lbl_center",
            "centred",
            vec![
                ("alignment", crawrad_ir::Value::Enum("taCenter".into())),
                ("layout", crawrad_ir::Value::Enum("tlCenter".into())),
            ],
            215.0,
            40.0,
        ),
        (
            "lbl_wrap",
            "a long caption that breaks at the edge",
            vec![("word_wrap", crawrad_ir::Value::Bool(true))],
            262.0,
            50.0,
        ),
        (
            "lbl_auto",
            "fits its own text, wider than the box",
            vec![("auto_size", crawrad_ir::Value::Bool(true))],
            320.0,
            20.0,
        ),
    ] {
        let mut l = r
            .instantiate("Label", id, Rect::new(650.0, y, 120.0, h))
            .expect("catalog instantiates");
        l.set_name(id);
        l.set_text(text);
        for (k, v) in props {
            l.props.insert(k.into(), v);
        }
        d.add_child("form_main", l).expect("into the form");
    }
    {
        let l = d.node_mut("lbl_center").expect("the label");
        l.events
            .insert("on_click".into(), "lbl_center_click".into());
        l.events
            .insert("on_dbl_click".into(), "lbl_center_dbl_click".into());
    } // A TreeView whose nodes with children announce OnExpanding and
    // OnCollapsing before they open or close
    let mut tv = r
        .instantiate("TreeView", "tv_nodes", Rect::new(650.0, 8.0, 160.0, 200.0))
        .expect("catalog instantiates");
    tv.set_name("tv_nodes");
    tv.events
        .insert("on_expanding".into(), "tv_nodes_expanding".into());
    tv.events
        .insert("on_collapsing".into(), "tv_nodes_collapsing".into());
    d.add_child("form_main", tv).expect("into the form"); // the router's double click (a StatusBar), and OnMouseActivate sharing the
    // press with OnMouseDown on btn_prova (one arm, both calls)
    // A Hint on the proof button, shown as a tooltip on both targets
    d.node_mut("btn_prova").expect("the button").props.insert(
        "hint".into(),
        crawrad_ir::Value::Str("Runs the proof".into()),
    );
    // A label with its own font and background (ParentFont and
    // ParentColor off); the others take the form's font size (11)
    {
        let l = d.node_mut("lbl_center").expect("the label");
        l.props.insert(
            "font".into(),
            crawrad_ir::Value::Font(crawrad_ir::Font {
                size: 16.0,
                // step 2: its own text colour too
                color: crawrad_ir::Color::rgb(0xff, 0xcc, 0x00),
                ..Default::default()
            }),
        );
        l.props
            .insert("parent_font".into(), crawrad_ir::Value::Bool(false));
    }
    // A button whose text is orange, at the form's size
    {
        let b = d.node_mut("btn_prova").expect("the button");
        b.props.insert(
            "font".into(),
            crawrad_ir::Value::Font(crawrad_ir::Font {
                size: 11.0,
                color: crawrad_ir::Color::rgb(0xff, 0x8c, 0x1a),
                ..Default::default()
            }),
        );
        b.props
            .insert("parent_font".into(), crawrad_ir::Value::Bool(false));
    }
    {
        let l = d.node_mut("lbl_center").expect("the label");
        l.props.insert(
            "color".into(),
            crawrad_ir::Value::Color(crawrad_ir::Color::rgb(0x3a, 0x5a, 0x8a)),
        );
        l.props
            .insert("parent_color".into(), crawrad_ir::Value::Bool(false));
    }
    d.node_mut("form_main").expect("the root").props.insert(
        "font".into(),
        crawrad_ir::Value::Font(crawrad_ir::Font {
            name: String::new(),
            size: 11.0,
            ..Default::default()
        }),
    );
    let mut sb = r
        .instantiate("StatusBar", "sb_main", Rect::new(0.0, 396.0, 640.0, 24.0))
        .expect("catalog instantiates");
    sb.set_name("sb_main");
    sb.events
        .insert("on_dbl_click".into(), "sb_main_dbl_click".into());
    d.add_child("form_main", sb).expect("into the form"); // A Panel and a GroupBox clicked through their own View (a cursor
    // makes it test the pointer); a click on the Button INSIDE the Panel is the
    // Button's, not the Panel's
    // The ListBox, the ComboBox and the StatusBar take a font
    // colour of their own; the list a background too
    for (id, rgb, bg) in [
        ("lst_rows", (0x5e, 0xe6, 0xc8), Some((0x14, 0x2e, 0x2a))),
        ("cmb_items", (0xff, 0x7a, 0xc8), None),
        ("sb_main", (0xff, 0xd8, 0x4a), None),
        // step 4
        ("tv_nodes", (0xc8, 0xa0, 0xff), Some((0x26, 0x1e, 0x3a))),
        // the egui Memo once painted its own box over its Color
        (
            "mmo_read_only",
            (0xf0, 0xd0, 0x90),
            Some((0x3a, 0x2a, 0x14)),
        ),
    ] {
        let n = d.node_mut(id).expect("the control");
        n.props.insert(
            "font".into(),
            crawrad_ir::Value::Font(crawrad_ir::Font {
                size: 11.0,
                color: crawrad_ir::Color::rgb(rgb.0, rgb.1, rgb.2),
                ..Default::default()
            }),
        );
        n.props
            .insert("parent_font".into(), crawrad_ir::Value::Bool(false));
        // something on screen to carry the colour
        match id {
            "cmb_items" => {
                n.props
                    .insert("item_index".into(), crawrad_ir::Value::Int(0));
            }
            "sb_main" => {
                n.props
                    .insert("simple_text".into(), crawrad_ir::Value::Str("Ready".into()));
            }
            _ => {}
        }
        if let Some((r, g, b)) = bg {
            n.props.insert(
                "color".into(),
                crawrad_ir::Value::Color(crawrad_ir::Color::rgb(r, g, b)),
            );
            n.props
                .insert("parent_color".into(), crawrad_ir::Value::Bool(false));
        }
    }
    // Two panels with a Splitter between them; dragging it moves the
    // edge, with OnMoved wired
    for (id, caption, x, w, rgb) in [
        ("pnl_split_a", "A", 330.0, 70.0, (0x3a, 0x5a, 0x8a)),
        ("pnl_split_b", "B", 405.0, 80.0, (0x5a, 0x3a, 0x6a)),
    ] {
        let mut p = r
            .instantiate("Panel", id, Rect::new(x, 8.0, w, 26.0))
            .expect("catalog instantiates");
        p.set_name(id);
        p.set_text(caption);
        p.props.insert(
            "color".into(),
            crawrad_ir::Value::Color(crawrad_ir::Color::rgb(rgb.0, rgb.1, rgb.2)),
        );
        p.props
            .insert("parent_color".into(), crawrad_ir::Value::Bool(false));
        d.add_child("form_main", p).expect("into the form");
    }
    let mut spl = r
        .instantiate("Splitter", "spl_main", Rect::new(400.0, 8.0, 5.0, 26.0))
        .expect("catalog instantiates");
    spl.set_name("spl_main");
    spl.props
        .insert("min_size".into(), crawrad_ir::Value::Int(20));
    spl.events
        .insert("on_moved".into(), "spl_main_moved".into());
    d.add_child("form_main", spl).expect("into the form");
    // Three SpeedButtons in one group (GroupIndex 1), the middle one
    // down; a click puts another down and lifts it
    for (i, id) in ["sbt_left", "sbt_mid", "sbt_right"].into_iter().enumerate() {
        let mut b = r
            .instantiate(
                "SpeedButton",
                id,
                Rect::new(500.0 + i as f32 * 34.0, 296.0, 32.0, 24.0),
            )
            .expect("catalog instantiates");
        b.set_name(id);
        b.set_text(["L", "M", "R"][i]);
        b.props
            .insert("group_index".into(), crawrad_ir::Value::Int(1));
        b.props
            .insert("down".into(), crawrad_ir::Value::Bool(id == "sbt_mid"));
        b.props
            .insert("flat".into(), crawrad_ir::Value::Bool(false));
        d.add_child("form_main", b).expect("into the form");
    }
    // The six Shape kinds, each its own brush, with a 2-point pen
    for (i, (kind, rgb)) in [
        ("Rectangle", (0xe0, 0x5a, 0x5a)),
        ("Square", (0xe0, 0x9a, 0x3a)),
        ("RoundRect", (0xd8, 0xd0, 0x40)),
        ("RoundSquare", (0x5a, 0xc0, 0x6a)),
        ("Ellipse", (0x4a, 0x9a, 0xe0)),
        ("Circle", (0xa0, 0x6a, 0xe0)),
    ]
    .into_iter()
    .enumerate()
    {
        let id = format!("shp_{i}");
        let mut shp = r
            .instantiate(
                "Shape",
                id.clone(),
                Rect::new(
                    200.0 + (i % 3) as f32 * 32.0,
                    34.0 + (i / 3) as f32 * 26.0,
                    28.0,
                    22.0,
                ),
            )
            .expect("catalog instantiates");
        shp.set_name(&id);
        shp.props
            .insert("shape".into(), crawrad_ir::Value::Enum(kind.into()));
        shp.props
            .insert("pen_width".into(), crawrad_ir::Value::Int(2));
        shp.props.insert(
            "brush_color".into(),
            crawrad_ir::Value::Color(crawrad_ir::Color::rgb(rgb.0, rgb.1, rgb.2)),
        );
        shp.props.insert(
            "pen_color".into(),
            crawrad_ir::Value::Color(crawrad_ir::Color::rgb(0xf0, 0xf0, 0xf0)),
        );
        d.add_child("form_main", shp).expect("into the form");
    }
    // The StatusBar shows three panels instead of its simple text
    d.node_mut("sb_main")
        .expect("the bar")
        .props
        .insert("simple_panel".into(), crawrad_ir::Value::Bool(false));
    for (id, text, width, alignment) in [
        ("stp_state", "Ready", 120, "taLeftJustify"),
        ("stp_pos", "Ln 12, Col 4", 140, "taCenter"),
        ("stp_mode", "INS", 60, "taRightJustify"),
    ] {
        let mut p = r
            .instantiate("StatusPanel", id, Rect::new(0.0, 0.0, 28.0, 28.0))
            .expect("catalog instantiates");
        p.set_name(id);
        p.set_text(text);
        p.props
            .insert("width".into(), crawrad_ir::Value::Int(width));
        p.props.insert(
            "alignment".into(),
            crawrad_ir::Value::Enum(alignment.into()),
        );
        d.add_child("sb_main", p).expect("into the bar");
    }
    // A ScrollBox with a 3-point border and a background of its own
    let mut sbx = r
        .instantiate(
            "ScrollBox",
            "sbx_frame",
            Rect::new(650.0, 345.0, 160.0, 45.0),
        )
        .expect("catalog instantiates");
    sbx.set_name("sbx_frame");
    sbx.props
        .insert("border_width".into(), crawrad_ir::Value::Int(3));
    sbx.props.insert(
        "color".into(),
        crawrad_ir::Value::Color(crawrad_ir::Color::rgb(0x2e, 0x24, 0x10)),
    );
    sbx.props
        .insert("parent_color".into(), crawrad_ir::Value::Bool(false));
    // Its horizontal axis is off, so only the vertical bar may show
    sbx.props
        .insert("horz_scroll_bar".into(), crawrad_ir::Value::Bool(false));
    d.add_child("form_main", sbx).expect("into the form");
    // A child that runs past the box on both axes
    let mut btn = r
        .instantiate("Button", "btn_sbx", Rect::new(6.0, 6.0, 220.0, 60.0))
        .expect("catalog instantiates");
    btn.set_name("btn_sbx");
    btn.set_text("Wider and taller than the box");
    d.add_child("sbx_frame", btn).expect("into the box");
    // A vertical ScrollBar (its box is tall), with OnScroll wired
    let mut sbv = r
        .instantiate(
            "ScrollBar",
            "sbv_side",
            Rect::new(176.0, 120.0, 16.0, 110.0),
        )
        .expect("catalog instantiates");
    sbv.set_name("sbv_side");
    sbv.props
        .insert("kind".into(), crawrad_ir::Value::Enum("sbVertical".into()));
    sbv.events
        .insert("on_scroll".into(), "sbv_side_scroll".into());
    d.add_child("form_main", sbv).expect("into the form");
    // A three-state CheckBox, starting grayed, with OnChange wired
    let mut chk = r
        .instantiate("CheckBox", "chk_tri", Rect::new(330.0, 40.0, 150.0, 20.0))
        .expect("catalog instantiates");
    chk.set_name("chk_tri");
    chk.set_text("chk_tri");
    chk.props
        .insert("allow_grayed".into(), crawrad_ir::Value::Bool(true));
    chk.props
        .insert("state".into(), crawrad_ir::Value::Enum("cbGrayed".into()));
    chk.events
        .insert("on_change".into(), "chk_tri_change".into());
    d.add_child("form_main", chk).expect("into the form");
    // Alignment left — the caption at the left edge, the box at the right
    let mut chk = r
        .instantiate("CheckBox", "chk_left", Rect::new(8.0, 366.0, 160.0, 20.0))
        .expect("catalog instantiates");
    chk.set_name("chk_left");
    chk.set_text("Caption left");
    chk.props
        .insert("checked".into(), crawrad_ir::Value::Bool(true));
    chk.props.insert(
        "alignment".into(),
        crawrad_ir::Value::Enum("taLeftJustify".into()),
    );
    d.add_child("form_main", chk).expect("into the form");
    let mut pnl = r
        .instantiate("Panel", "pnl_click", Rect::new(500.0, 200.0, 130.0, 90.0))
        .expect("catalog instantiates");
    pnl.set_name("pnl_click");
    pnl.events
        .insert("on_click".into(), "pnl_click_click".into());
    d.add_child("form_main", pnl).expect("into the form");
    let mut inner = r
        .instantiate("Button", "btn_inner", Rect::new(10.0, 50.0, 70.0, 24.0))
        .expect("catalog instantiates");
    inner.set_name("btn_inner");
    inner
        .events
        .insert("on_click".into(), "btn_inner_click".into());
    d.add_child("pnl_click", inner).expect("into the panel");
    let mut gbx = r
        .instantiate(
            "GroupBox",
            "gbx_click",
            Rect::new(200.0, 280.0, 110.0, 50.0),
        )
        .expect("catalog instantiates");
    gbx.set_name("gbx_click");
    gbx.events
        .insert("on_click".into(), "gbx_click_click".into());
    d.add_child("form_main", gbx).expect("into the form");
    // A CategoryPanel holding a button; its header folds both
    let mut cat = r
        .instantiate(
            "CategoryPanel",
            "cat_more",
            Rect::new(200.0, 336.0, 120.0, 54.0),
        )
        .expect("catalog instantiates");
    cat.set_name("cat_more");
    cat.set_text("More");
    cat.events
        .insert("on_collapsed".into(), "cat_more_collapsed".into());
    cat.events
        .insert("on_expanded".into(), "cat_more_expanded".into());
    d.add_child("form_main", cat).expect("into the form");
    let mut cat_btn = r
        .instantiate("Button", "btn_more", Rect::new(6.0, 27.0, 70.0, 22.0))
        .expect("catalog instantiates");
    cat_btn.set_name("btn_more");
    d.add_child("cat_more", cat_btn)
        .expect("into the CategoryPanel");
    // A Card holding a Label and a Button inside
    let mut crd = r
        .instantiate("Card", "crd_gate", Rect::new(300.0, 460.0, 220.0, 140.0))
        .expect("catalog instantiates");
    crd.set_name("crd_gate");
    crd.set_text("Profile");
    crd.props.insert(
        "subtitle".into(),
        crawrad_ir::Value::Str("Signed in".into()),
    );
    d.add_child("form_main", crd).expect("into the form");
    let mut crd_lbl = r
        .instantiate("Label", "lbl_gate_crd", Rect::new(12.0, 48.0, 100.0, 20.0))
        .expect("catalog instantiates");
    crd_lbl.set_name("lbl_gate_crd");
    crd_lbl.set_text("Status: Active");
    d.add_child("crd_gate", crd_lbl).expect("into the Card");
    let mut crd_btn = r
        .instantiate("Button", "btn_gate_crd", Rect::new(12.0, 76.0, 80.0, 24.0))
        .expect("catalog instantiates");
    crd_btn.set_name("btn_gate_crd");
    crd_btn.set_text("Edit");
    d.add_child("crd_gate", crd_btn).expect("into the Card");
    // A ListView with 2 columns and 3 items
    let mut lvw = r
        .instantiate("ListView", "lvw_gate", Rect::new(8.0, 420.0, 240.0, 120.0))
        .expect("catalog instantiates");
    lvw.set_name("lvw_gate");
    lvw.props.insert(
        "items".into(),
        crawrad_ir::Item::list_to(&[
            crawrad_ir::Item::with_children("Alpha", vec![crawrad_ir::Item::new("100 KB")]),
            crawrad_ir::Item::with_children("Beta", vec![crawrad_ir::Item::new("250 KB")]),
            crawrad_ir::Item::with_children("Gamma", vec![crawrad_ir::Item::new("500 KB")]),
        ]),
    );
    lvw.events
        .insert("on_change".into(), "lvw_gate_change".into());
    lvw.events
        .insert("on_click".into(), "lvw_gate_click".into());
    lvw.events
        .insert("on_dbl_click".into(), "lvw_gate_dbl_click".into());
    d.add_child("form_main", lvw).expect("into the form");

    let mut col1 = r
        .instantiate("ListColumn", "col_name", Rect::new(0.0, 0.0, 28.0, 28.0))
        .expect("catalog instantiates");
    col1.set_name("col_name");
    col1.set_text("Name");
    col1.props
        .insert("width".into(), crawrad_ir::Value::Int(120));
    col1.props.insert(
        "alignment".into(),
        crawrad_ir::Value::Enum("taLeftJustify".into()),
    );
    d.add_child("lvw_gate", col1).expect("into the ListView");

    let mut col2 = r
        .instantiate("ListColumn", "col_size", Rect::new(0.0, 0.0, 28.0, 28.0))
        .expect("catalog instantiates");
    col2.set_name("col_size");
    col2.set_text("Size");
    col2.props
        .insert("width".into(), crawrad_ir::Value::Int(80));
    col2.props.insert(
        "alignment".into(),
        crawrad_ir::Value::Enum("taRightJustify".into()),
    );
    d.add_child("lvw_gate", col2).expect("into the ListView");

    // A MessageDialog and a button whose OnClick calls execute()
    let mut dlg = r
        .instantiate(
            "MessageDialog",
            "dlg_msg_gate",
            Rect::new(0.0, 0.0, 28.0, 28.0),
        )
        .expect("catalog instantiates");
    dlg.set_name("dlg_msg_gate");
    dlg.set_text("Disable the Show Message button?");
    dlg.props.insert(
        "kind".into(),
        crawrad_ir::Value::Enum("mtConfirmation".into()),
    );
    dlg.props
        .insert("buttons".into(), crawrad_ir::Value::Enum("mbYesNo".into()));
    dlg.events
        .insert("on_close".into(), "dlg_msg_gate_close".into());
    d.add_child("form_main", dlg).expect("into the form");

    let mut btn_msg = r
        .instantiate("Button", "btn_show_msg", Rect::new(8.0, 550.0, 110.0, 24.0))
        .expect("catalog instantiates");
    btn_msg.set_name("btn_show_msg");
    btn_msg.set_text("Show Message");
    btn_msg
        .events
        .insert("on_click".into(), "btn_show_msg_click".into());
    d.add_child("form_main", btn_msg).expect("into the form");

    // Three Badges side by side in free space below Card
    for (id, caption, kind, outline, x) in [
        ("bdg_new", "New", "bkPrimary", false, 300.0),
        ("bdg_beta", "Beta", "bkInfo", true, 370.0),
        ("bdg_error", "Error", "bkDanger", false, 440.0),
    ] {
        let mut bdg = r
            .instantiate("Badge", id, Rect::new(x, 620.0, 56.0, 20.0))
            .expect("catalog instantiates");
        bdg.set_name(id);
        bdg.set_text(caption);
        bdg.props
            .insert("kind".into(), crawrad_ir::Value::Enum(kind.into()));
        bdg.props
            .insert("outline".into(), crawrad_ir::Value::Bool(outline));
        d.add_child("form_main", bdg).expect("into the form");
    }

    // An Inbox button and a notification Badge anchored to it
    let mut btn_inbox = r
        .instantiate("Button", "btn_inbox", Rect::new(600.0, 620.0, 80.0, 24.0))
        .expect("catalog instantiates");
    btn_inbox.set_name("btn_inbox");
    btn_inbox.set_text("Inbox");
    btn_inbox
        .events
        .insert("on_click".into(), "btn_inbox_click".into());
    d.add_child("form_main", btn_inbox).expect("into the form");

    let mut bdg_inbox = r
        .instantiate("Badge", "bdg_inbox", Rect::new(600.0, 620.0, 56.0, 20.0))
        .expect("catalog instantiates");
    bdg_inbox.set_name("bdg_inbox");
    bdg_inbox
        .props
        .insert("count".into(), crawrad_ir::Value::Int(3));
    bdg_inbox.props.insert(
        "associate".into(),
        crawrad_ir::Value::Ref("btn_inbox".into()),
    );
    d.add_child("form_main", bdg_inbox).expect("into the form");

    // A Chips control with 4 items, removable, first selected
    let mut chp = r
        .instantiate("Chips", "chp_tags", Rect::new(700.0, 470.0, 220.0, 56.0))
        .expect("catalog instantiates");
    chp.set_name("chp_tags");
    chp.props.insert(
        "items".into(),
        crawrad_ir::Item::list_to(
            &["Rust", "egui", "Makepad", "Slint"]
                .iter()
                .map(|t| crawrad_ir::Item::new(*t))
                .collect::<Vec<_>>(),
        ),
    );
    chp.props
        .insert("removable".into(), crawrad_ir::Value::Bool(true));
    chp.props
        .insert("item_index".into(), crawrad_ir::Value::Int(0));
    d.add_child("form_main", chp).expect("into the form");

    // A Breadcrumb control with 3 items and an OnChange handler
    let mut brc = r
        .instantiate(
            "Breadcrumb",
            "brc_path",
            Rect::new(700.0, 540.0, 260.0, 24.0),
        )
        .expect("catalog instantiates");
    brc.set_name("brc_path");
    brc.props.insert(
        "items".into(),
        crawrad_ir::Item::list_to(
            &["Home", "Projects", "CrawRAD"]
                .iter()
                .map(|t| crawrad_ir::Item::new(*t))
                .collect::<Vec<_>>(),
        ),
    );
    brc.events
        .insert("on_change".into(), "brc_path_change".into());
    d.add_child("form_main", brc).expect("into the form");

    // A Pagination control with 20 pages, index 9 and an OnChange handler
    let mut pag = r
        .instantiate(
            "Pagination",
            "pag_pages",
            Rect::new(700.0, 580.0, 280.0, 28.0),
        )
        .expect("catalog instantiates");
    pag.set_name("pag_pages");
    pag.props
        .insert("page_count".into(), crawrad_ir::Value::Int(20));
    pag.props
        .insert("page_index".into(), crawrad_ir::Value::Int(9));
    pag.events
        .insert("on_change".into(), "pag_pages_change".into());
    d.add_child("form_main", pag).expect("into the form");

    // A PageControl whose OnChanging can say no and whose OnChange
    // follows the switch
    let mut pgc = r
        .instantiate(
            "PageControl",
            "pgc_pages",
            Rect::new(330.0, 340.0, 300.0, 70.0),
        )
        .expect("catalog instantiates");
    pgc.set_name("pgc_pages");
    pgc.events
        .insert("on_changing".into(), "pgc_pages_changing".into());
    pgc.events
        .insert("on_change".into(), "pgc_pages_change".into());
    // The tabs under the pages
    pgc.props.insert(
        "tab_position".into(),
        crawrad_ir::Value::Enum("tpBottom".into()),
    );
    // The tabs' text in a colour of its own
    pgc.props.insert(
        "font".into(),
        crawrad_ir::Value::Font(crawrad_ir::Font {
            size: 11.0,
            color: crawrad_ir::Color::rgb(0x7c, 0xe0, 0x8a),
            ..Default::default()
        }),
    );
    pgc.props
        .insert("parent_font".into(), crawrad_ir::Value::Bool(false));
    d.add_child("form_main", pgc).expect("into the form");
    for (id, caption) in [("ts_one", "One"), ("ts_two", "Two")] {
        let mut ts = r
            .instantiate("TabSheet", id, Rect::new(0.0, 0.0, 300.0, 46.0))
            .expect("catalog instantiates");
        ts.set_name(id);
        ts.set_text(caption);
        d.add_child("pgc_pages", ts).expect("into the PageControl");
    }
    // A page whose tab is hidden — reached only by ActivePageIndex
    let mut ts = r
        .instantiate("TabSheet", "tab_hidden", Rect::new(0.0, 0.0, 300.0, 46.0))
        .expect("catalog instantiates");
    ts.set_name("tab_hidden");
    ts.set_text("Hidden");
    ts.props
        .insert("tab_visible".into(), crawrad_ir::Value::Bool(false));
    d.add_child("pgc_pages", ts).expect("into the PageControl"); // An Action with a caption, OnExecute and OnUpdate, and a Button
    // linked to it — the click runs it, the action owns the button's state
    let mut act = r
        .instantiate("Action", "act_count", Rect::new(240.0, 8.0, 24.0, 24.0))
        .expect("catalog instantiates");
    act.set_name("act_count");
    act.props
        .insert("text".into(), crawrad_ir::Value::Str("Count".into()));
    act.events
        .insert("on_execute".into(), "act_count_execute".into());
    act.events
        .insert("on_update".into(), "act_count_update".into());
    d.add_child("form_main", act).expect("into the form");
    let mut btn = r
        .instantiate("Button", "btn_count", Rect::new(110.0, 8.0, 80.0, 24.0))
        .expect("catalog instantiates");
    btn.set_name("btn_count");
    btn.props
        .insert("action".into(), crawrad_ir::Value::Ref("act_count".into()));
    d.add_child("form_main", btn).expect("into the form"); // A Memo whose Enter starts a line, whose Tab types a tab, that stops
    // at 40 characters and does not wrap — all read at run time
    let mut mmo = r
        .instantiate("Memo", "mmo_keys", Rect::new(200.0, 205.0, 110.0, 64.0))
        .expect("catalog instantiates");
    mmo.set_name("mmo_keys");
    mmo.props
        .insert("text".into(), crawrad_ir::Value::Str(String::new()));
    mmo.props
        .insert("word_wrap".into(), crawrad_ir::Value::Bool(false));
    mmo.props
        .insert("want_tabs".into(), crawrad_ir::Value::Bool(true));
    mmo.props
        .insert("max_length".into(), crawrad_ir::Value::Int(40));
    mmo.events
        .insert("on_change".into(), "mmo_keys_change".into());
    d.add_child("form_main", mmo).expect("into the form"); // The pointer events of ANY control, through the one router — a
    // PaintBox that sees the pointer press and move, a Button that sees it
    // come and go, a Shape that is clicked (each event as its class declares)
    let mut pbx = r
        .instantiate("PaintBox", "pbx_draw", Rect::new(500.0, 8.0, 120.0, 80.0))
        .expect("catalog instantiates");
    pbx.set_name("pbx_draw");
    for (e, h) in [
        ("on_mouse_down", "pbx_draw_down"),
        ("on_mouse_move", "pbx_draw_move"),
        // It draws
        ("on_paint", "pbx_draw_paint"),
    ] {
        pbx.events.insert(e.into(), h.into());
    }
    d.add_child("form_main", pbx).expect("into the form");
    for (e, h) in [
        ("on_mouse_enter", "btn_prova_enter"),
        ("on_mouse_leave", "btn_prova_leave"),
        // the focus: a press gives it, the keys follow it
        ("on_enter", "btn_prova_focus_in"),
        ("on_exit", "btn_prova_focus_out"),
    ] {
        d.node_mut("btn_prova")
            .expect("the button")
            .events
            .insert(e.into(), h.into());
    }
    let mut shp = r
        .instantiate("Shape", "shp_dot", Rect::new(500.0, 100.0, 60.0, 60.0))
        .expect("catalog instantiates");
    shp.set_name("shp_dot");
    shp.events.insert("on_click".into(), "shp_dot_click".into());
    d.add_child("form_main", shp).expect("into the form");
    let mut lbl = r
        .instantiate("Label", "lbl_ticks", Rect::new(200.0, 60.0, 160.0, 24.0))
        .expect("catalog instantiates");
    lbl.set_name("lbl_ticks");
    d.add_child("form_main", lbl).expect("into the form");
    // A MainMenu — File with a shortcut, a separator, a sub-menu and an
    // item that runs the Action; Edit with a checked item and a disabled one
    // (its bar covers the top row here: this gate builds, it does not click)
    // A PopupMenu with OnPopup, the Shape's secondary click opens it
    for (class, id, x) in [
        ("MainMenu", "mnu_main", 270.0),
        ("PopupMenu", "pop_shape", 300.0),
    ] {
        let mut m = r
            .instantiate(class, id, Rect::new(x, 140.0, 28.0, 28.0))
            .expect("catalog instantiates");
        m.set_name(id);
        if class == "PopupMenu" {
            m.events.insert("on_popup".into(), "pop_shape_popup".into());
        }
        d.add_child("form_main", m).expect("into the form");
    }
    for (owner, id, text, extra) in [
        ("mnu_main", "mni_file", "&File", ""),
        ("mni_file", "mni_open", "Open...", "Ctrl+O"),
        ("mni_file", "mni_sep", "-", ""),
        ("mni_file", "mni_recent", "Recent", ""),
        ("mni_recent", "mni_recent_a", "first.crawform", ""),
        ("mni_recent", "mni_recent_b", "second.crawform", ""),
        ("mni_file", "mni_count", "", "action"),
        ("mnu_main", "mni_edit", "&Edit", ""),
        ("mni_edit", "mni_wrap", "Word wrap", "checked"),
        ("mni_edit", "mni_undo", "Undo", "disabled"),
        ("pop_shape", "mni_grow", "Grow", ""),
        ("pop_shape", "mni_shrink", "Shrink", ""),
    ] {
        let mut it = r
            .instantiate("MenuItem", id, Rect::new(0.0, 0.0, 28.0, 28.0))
            .expect("catalog instantiates");
        it.set_name(id);
        it.set_text(text);
        match extra {
            "action" => {
                it.props
                    .insert("action".into(), crawrad_ir::Value::Ref("act_count".into()));
            }
            "checked" => {
                it.props
                    .insert("checked".into(), crawrad_ir::Value::Bool(true));
            }
            "disabled" => {
                it.props
                    .insert("enabled".into(), crawrad_ir::Value::Bool(false));
            }
            "" => {}
            shortcut => {
                it.props.insert(
                    "shortcut".into(),
                    crawrad_ir::Value::Shortcut(
                        shortcut
                            .parse::<crawrad_ir::Shortcut>()
                            .expect("a shortcut"),
                    ),
                );
            }
        }
        if matches!(id, "mni_open" | "mni_recent_a" | "mni_grow" | "mni_shrink") {
            it.events.insert("on_click".into(), format!("{id}_click"));
        }
        d.add_child(owner, it).expect("into the menu");
    }
    d.node_mut("shp_dot").expect("the shape").props.insert(
        "popup_menu".into(),
        crawrad_ir::Value::Ref("pop_shape".into()),
    );

    // A MultiSelect control and a summary Label with OnChange handler
    let mut lbl_stk = r
        .instantiate("Label", "lbl_stack", Rect::new(540.0, 405.0, 150.0, 20.0))
        .expect("catalog instantiates");
    lbl_stk.set_name("lbl_stack");
    lbl_stk.set_text("1 checked");
    d.add_child("form_main", lbl_stk).expect("into the form");

    let mut mse = r
        .instantiate(
            "MultiSelect",
            "mse_stack",
            Rect::new(540.0, 430.0, 150.0, 24.0),
        )
        .expect("catalog instantiates");
    mse.set_name("mse_stack");
    mse.props.insert(
        "items".into(),
        crawrad_ir::Item::list_to(
            &["Rust", "egui", "Makepad", "Slint", "Iced"]
                .iter()
                .map(|t| crawrad_ir::Item::new(*t))
                .collect::<Vec<_>>(),
        ),
    );
    mse.props.insert(
        "selected_items".into(),
        crawrad_ir::Item::list_to(
            &["Rust"]
                .iter()
                .map(|t| crawrad_ir::Item::new(*t))
                .collect::<Vec<_>>(),
        ),
    );
    mse.events
        .insert("on_change".into(), "mse_stack_change".into());
    d.add_child("form_main", mse).expect("into the form");

    let mut clb_box = r
        .instantiate(
            "CheckListBox",
            "clb_features",
            Rect::new(540.0, 460.0, 150.0, 80.0),
        )
        .expect("catalog instantiates");
    clb_box.set_name("clb_features");
    clb_box.props.insert(
        "items".into(),
        crawrad_ir::Item::list_to(
            &["Rust", "egui", "Makepad"]
                .iter()
                .map(|t| crawrad_ir::Item::new(*t))
                .collect::<Vec<_>>(),
        ),
    );
    clb_box.props.insert(
        "checked_items".into(),
        crawrad_ir::Item::list_to(
            &["Rust"]
                .iter()
                .map(|t| crawrad_ir::Item::new(*t))
                .collect::<Vec<_>>(),
        ),
    );
    clb_box
        .events
        .insert("on_change".into(), "clb_features_change".into());
    d.add_child("form_main", clb_box).expect("into the form");

    // A MaskEdit for a date, with its change, its keys and its Enter
    let mut msk = r
        .instantiate("MaskEdit", "msk_date", Rect::new(130.0, 590.0, 120.0, 24.0))
        .expect("catalog instantiates");
    msk.set_name("msk_date");
    msk.props.insert(
        "edit_mask".into(),
        crawrad_ir::Value::Str("!99/99/0000;1;_".into()),
    );
    msk.events
        .insert("on_change".into(), "msk_date_change".into());
    msk.events
        .insert("on_key_down".into(), "msk_date_key_down".into());
    msk.events
        .insert("on_return".into(), "msk_date_return".into());
    d.add_child("form_main", msk).expect("into the form");

    // A Button and a Drawer, with OnClick and OnClose handlers
    let mut btn_drw = r
        .instantiate("Button", "btn_drawer", Rect::new(8.0, 590.0, 110.0, 24.0))
        .expect("catalog instantiates");
    btn_drw.set_name("btn_drawer");
    btn_drw.set_text("Menu");
    btn_drw
        .events
        .insert("on_click".into(), "btn_drawer_click".into());
    d.add_child("form_main", btn_drw).expect("into the form");

    let mut drw = r
        .instantiate("Drawer", "drw_menu", Rect::new(760.0, 24.0, 240.0, 636.0))
        .expect("catalog instantiates");
    drw.set_name("drw_menu");
    drw.set_text("Menu");
    drw.props
        .insert("side".into(), crawrad_ir::Value::Enum("dsRight".into()));
    drw.props
        .insert("opened".into(), crawrad_ir::Value::Bool(false));
    drw.events
        .insert("on_close".into(), "drw_menu_close".into());
    d.add_child("form_main", drw).expect("into the form");

    let mut lbl_drw = r
        .instantiate("Label", "lbl_drawer", Rect::new(16.0, 48.0, 120.0, 20.0))
        .expect("catalog instantiates");
    lbl_drw.set_name("lbl_drawer");
    lbl_drw.set_text("Settings");
    d.add_child("drw_menu", lbl_drw).expect("into the Drawer");

    d.validate().expect("valid");
    d
}

/// The `(handler, declared parameters)` pairs requested by the document.
fn esperados() -> Vec<(String, String)> {
    let r = Registry::builtin();
    let mut fora = Vec::new();
    for (classe, id) in [("Form", "form_main"), ("Button", "btn_prova")] {
        for e in &r.get(classe).expect("catalog class").events {
            let params = e.method_params();
            if params == "&mut self" {
                continue;
            }
            fora.push((crawrad_codegen::handler_padrao(id, e.name), params));
        }
    }
    fora
}

/// What the proof PaintBox's `OnPaint` draws — a background, a broken
/// line, a circle and a caption — so both targets can be looked at.
fn com_o_desenho(fonte: &str) -> String {
    let agulha = "pub fn pbx_draw_paint(";
    let Some(i) = fonte.find(agulha) else {
        return fonte.to_string();
    };
    let abre = i + fonte[i..].find(") {").expect("handler opens body") + ") {".len();
    let drawing = "
        canvas.set_fill_style([40, 44, 60, 255]);
        canvas.fill_rect(0.0, 0.0, canvas.width(), canvas.height());
        canvas.set_stroke_style([240, 200, 80, 255]);
        canvas.set_line_width(2.0);
        canvas.move_to(8.0, 70.0);
        canvas.line_to(40.0, 32.0);
        canvas.line_to(70.0, 56.0);
        canvas.line_to(110.0, 14.0);
        canvas.set_fill_style([90, 170, 230, 255]);
        canvas.fill_circle(96.0, 60.0, 12.0);
        canvas.set_fill_style([235, 235, 235, 255]);
        canvas.fill_text(\"OnPaint\", 6.0, 4.0);";
    // the stub is born `_canvas` (an empty body warns about nothing); the
    // body uses it, so it gets its name, as the user would do
    let signature = fonte[i..abre].replacen("_canvas: &mut Canvas", "canvas: &mut Canvas", 1);
    format!("{}{signature}{drawing}{}", &fonte[..i], &fonte[abre..])
}

/// Wires `self.dlg_msg_gate.execute()` and reads `modal_result` on close,
/// and increments inbox badge count on Inbox button click.
fn com_o_dialogo(fonte: &str) -> String {
    let mut f = fonte.to_string();
    if let Some(i) = f.find("pub fn btn_show_msg_click(") {
        let abre = i + f[i..].find(") {").expect("btn_show_msg_click opens body") + ") {".len();
        f.insert_str(abre, "\n        self.dlg_msg_gate.execute();");
    }
    if let Some(i) = f.find("pub fn dlg_msg_gate_close(") {
        let abre = i + f[i..].find(") {").expect("dlg_msg_gate_close opens body") + ") {".len();
        f.insert_str(
            abre,
            "\n        if self.dlg_msg_gate.modal_result == ModalResult::Yes {\n            self.btn_show_msg.enabled = false;\n        }",
        );
    }
    if let Some(i) = f.find("pub fn btn_inbox_click(") {
        let abre = i + f[i..].find(") {").expect("btn_inbox_click opens body") + ") {".len();
        f.insert_str(abre, "\n        self.bdg_inbox.count += 1;");
    }
    // The classic breadcrumb navigation — the clicked segment becomes
    // the current page
    if let Some(i) = f.find("pub fn brc_path_change(") {
        let abre = i + f[i..].find(") {").expect("brc_path_change opens body") + ") {".len();
        f.insert_str(
            abre,
            "\n        self.brc_path.items.truncate(self.brc_path.item_index as usize + 1);",
        );
    }
    // The pagination page change
    if let Some(i) = f.find("pub fn pag_pages_change(") {
        let abre = i + f[i..].find(") {").expect("pag_pages_change opens body") + ") {".len();
        f.insert_str(abre, "\n        self.pag_pages.page_count += 1;");
    }
    // The multi-select selection change
    if let Some(i) = f.find("pub fn mse_stack_change(") {
        let abre = i + f[i..].find(") {").expect("mse_stack_change opens body") + ") {".len();
        f.insert_str(
            abre,
            "\n        self.lbl_stack.text = format!(\"{} checked\", self.mse_stack.selected_items.len());",
        );
    }
    if let Some(i) = f.find("pub fn clb_features_change(") {
        let abre = i + f[i..].find(") {").expect("clb_features_change opens body") + ") {".len();
        f.insert_str(
            abre,
            "\n        self.lbl_stack.text = format!(\"{} checked in clb\", self.clb_features.checked_items.len());",
        );
    }
    // Drawer open and close handlers
    if let Some(i) = f.find("pub fn btn_drawer_click(") {
        let abre = i + f[i..].find(") {").expect("btn_drawer_click opens body") + ") {".len();
        f.insert_str(abre, "\n        self.drw_menu.opened = true;");
    }
    if let Some(i) = f.find("pub fn drw_menu_close(") {
        let abre = i + f[i..].find(") {").expect("drw_menu_close opens body") + ") {".len();
        f.insert_str(
            abre,
            "\n        self.btn_drawer.text = \"Menu (closed)\".to_string();",
        );
    }
    f
}

/// Splices into the body of `handler` one proof per parameter, and returns the text.
///
/// The signature **is not touched**: it is what the generator wrote, and it is
/// what is being measured. Only the body is injected.
fn com_a_prova_no_corpo(fonte: &str, handler: &str, params: &str) -> String {
    let agulha = format!("pub fn {handler}(");
    let i = fonte
        .find(&agulha)
        .unwrap_or_else(|| panic!("generator did not write `{handler}`:\n{fonte}"));
    let abre = fonte[i..]
        .find(") {")
        .map(|p| i + p + ") {".len())
        .unwrap_or_else(|| panic!("declaration of `{handler}` does not open a body"));
    let mut corpo = String::new();
    for (k, p) in params.split(", ").skip(1).enumerate() {
        let (nome, tipo) = p
            .split_once(": ")
            .unwrap_or_else(|| panic!("parameter without type in `{handler}`: {p:?}"));
        corpo.push_str(&format!("\n        let _prova{k}: {tipo} = {nome};"));
    }
    format!("{}{corpo}{}", &fonte[..abre], &fonte[abre..])
}

/// A `cargo` invocation inside the generated project. Same as sibling gates,
/// and for the same reason: outer cargo environment variables would confuse the inner one.
fn cargo(destino: &Path, args: &[&str]) -> (bool, String) {
    let saida = Command::new(env!("CARGO"))
        .args(args)
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("RUSTFLAGS")
        .current_dir(destino)
        .output()
        .expect("calling cargo inside generated project");
    let texto = format!(
        "{}{}",
        String::from_utf8_lossy(&saida.stdout),
        String::from_utf8_lossy(&saida.stderr)
    );
    (saida.status.success(), texto)
}

/// Is the draft intact? Same rule as sibling gates: checks for **everything**
/// the setup writes, because a partial draft cannot be reused.
fn rascunho_intacto(grupo: &Path, destino: &Path, dependencia: &str) -> bool {
    if !manifesto_intocado(grupo, RAIZ_DO_GRUPO) || !destino.join("src").is_dir() {
        return false;
    }
    let crate_ = dependencia.split('@').next().expect("the crate name");
    std::fs::read_to_string(destino.join("Cargo.toml")).is_ok_and(|m| {
        m.contains(crate_)
            && (!dependencia.contains("makepad")
                || (m.contains("crawrad-runtime") && m.contains("crawrad-runtime-makepad")))
    })
}

fn preparar_projeto(destino: &Path, dependencia: &str) {
    let grupo = destino
        .parent()
        .expect("destination is a member of a group");
    if rascunho_intacto(grupo, destino, dependencia) {
        return;
    }
    if destino.exists() {
        std::fs::remove_dir_all(destino).expect("delete partial draft");
    }
    std::fs::create_dir_all(grupo).expect("create group root");
    std::fs::write(grupo.join("Cargo.toml"), RAIZ_DO_GRUPO).expect("group root");

    let nome = destino
        .file_name()
        .and_then(|n| n.to_str())
        .expect("the name");
    let (ok, texto) = cargo(
        grupo,
        &[
            "new",
            "--bin",
            "--vcs",
            "none",
            "--name",
            "gate_evento",
            nome,
        ],
    );
    assert!(ok, "`cargo new` failed:\n{texto}");

    let alvo = if dependencia.contains("makepad") {
        crawrad_codegen::AlvoGerador::Makepad
    } else {
        crawrad_codegen::AlvoGerador::Egui
    };

    for dep in crawrad_codegen::dependencias_do_projeto(alvo, true, false) {
        let args = dep.args_cargo_add();
        let mut cmd = vec!["add"];
        let refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        cmd.extend(refs);
        let (ok, out) = cargo(destino, &cmd);
        assert!(
            ok,
            "`cargo add` of unified dependency `{}` failed:\n{out}",
            dep.nome()
        );
    }
}

/// Deletes `src/` before each target: the user file only grows, and leaving
/// it between targets would measure a blend that no project has.
fn limpar_src(destino: &Path) {
    let src = destino.join("src");
    for e in std::fs::read_dir(&src).expect("read src") {
        let p = e.expect("an entry").path();
        if p.extension().is_some_and(|x| x == "rs") {
            std::fs::remove_file(&p).expect("delete old file");
        }
    }
}

/// **The gate.** In both targets, the handler is born with the declared parameter
/// and the project compiles using that parameter by type.
#[test]
#[ignore = "compiles a real project; run with --ignored"]
fn o_handler_nasce_com_o_parametro_declarado_e_o_projeto_compila() {
    let doc = documento();
    let esperados = esperados();
    assert!(
        esperados.len() >= 15,
        "case measures only {} handlers with parameters",
        esperados.len()
    );

    let mut falhas = Vec::new();
    for alvo in ALVOS {
        let destino: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/gate-assinatura")
            .join(alvo.nome)
            .join("app");
        preparar_projeto(&destino, alvo.dependencia);
        limpar_src(&destino);

        let arqs = gerador(alvo.nome)
            .gerar(&doc, "gate_evento", &crawrad_registry::Registry::builtin())
            .expect("generate");
        crawrad_codegen::escrever(&destino, &arqs).expect("write project");

        let arquivo = destino.join("src/form_main.rs");
        let mut fonte = std::fs::read_to_string(&arquivo).expect("the user file");
        for (handler, params) in &esperados {
            let declarado = format!("pub fn {handler}({params})");
            assert!(
                fonte.contains(&declarado),
                "target `{}` did not write `{declarado}`:\n{fonte}",
                alvo.nome
            );
            fonte = com_a_prova_no_corpo(&fonte, handler, params);
        }
        // The PaintBox's OnPaint draws something to look at
        fonte = com_o_desenho(&fonte);
        // MessageDialog execute and modal_result proof
        fonte = com_o_dialogo(&fonte);
        std::fs::write(&arquivo, &fonte).expect("save proof body");

        // No `allow` in the user file. It is INNER — it would silence the
        // entire file, including whatever the user writes in it. Since
        // 16/09/2026 handler names are `snake_case` and the warning no longer
        // exists, instead of being silenced (owner's decision).
        // The check looks for the ATTRIBUTE, not the word: a line that
        // *mentions* `#[allow(...)]` in a comment is harmless, which is the
        // same distinction `portao_do_lint_silenciado` makes using a full
        // lexer. Here a line check suffices — an attribute starts the line,
        // while a comment starts with `//`.
        let calados: Vec<&str> = fonte
            .lines()
            .map(str::trim)
            .filter(|l| l.starts_with("#[allow") || l.starts_with("#![allow"))
            .collect();
        assert!(
            calados.is_empty(),
            "target `{}` put {calados:?} in user file — it is INNER \
             when `#![...]` and would silence even what the user writes here",
            alvo.nome
        );

        let (ok, texto) = cargo(&destino, &["build"]);
        // The warning only appears in the output when the crate is actually
        // recompiled, and a gate that read zero due to caching would measure
        // nothing. Here it always recompiles: `limpar_src` wipes and code
        // generation rewrites the three `.rs` files before each run. Only
        // dependencies stay warm.
        //
        // The summary line (`... generated N warnings`) is excluded: counting
        // the summary alongside what it summarizes would double the count.
        //
        // **What IS waived, and why.** `never used` / `never read` is
        // `dead_code`, and its owner is known: of the eight events with
        // parameters, none is DISPATCHED by generated code yet — the handler
        // is born, compiles and nobody calls it. That debt belongs to event
        // dispatch, not the signature, and is the same reason why
        // `gate_sem_avisos` omits these events by construction
        // (`vira_chamada`). Demanding it here would measure another front's
        // work. What this gate enforces is what the REMOVAL of the two `allow`
        // directives could have brought back — `non_snake_case` and
        // `unused_imports` —, and that it enforces completely.
        //
        // **`never constructed` is the THIRD phrasing of the same debt**,
        // and it was missing here for a measured reason: when this line was
        // written, the designer still carried `#![allow(dead_code)]`, so
        // payload types nobody constructs never surfaced in the output — only
        // the user file's handler appeared, about which `rustc` reports `never
        // used`. Once `allow` was removed (the owner: *"rule, NO `allow()`"*),
        // that same debt spoke with the missing voice: `struct
        // \x60KeyEvent\x60 is never constructed`. It is the SAME fact — no
        // `OnKeyDown` is dispatched by generated code, so no `KeyEvent` is
        // constructed.
        //
        // It WAS waived by an expiring list, and the list was removed on
        // 17/09/2026 when the egui target began dispatching form events:
        // all five types started being constructed. Today `never constructed`
        // fails, without exception.
        //
        // Measured on 17/09/2026: egui = 1 waived (`never used`, the `Button`
        // handler, which is not yet dispatched); makepad = 0 overall.
        let linhas_de_aviso = || {
            texto
                .lines()
                .filter(|l| l.starts_with("warning:") && !l.contains("generated"))
        };
        let perdoado = |l: &str| l.contains("never used") || l.contains("never read");
        let avisos: Vec<&str> = linhas_de_aviso().filter(|l| !perdoado(l)).collect();
        let perdoados = linhas_de_aviso().filter(|l| perdoado(l)).count();
        println!(
            "{}: {} — {} warning(s) enforced, {perdoados} waived (dead_code)",
            alvo.nome,
            if ok { "compiles" } else { "BROKE" },
            avisos.len()
        );
        if !ok {
            falhas.push(format!("=== {} ===\n{texto}", alvo.nome));
        } else if !avisos.is_empty() {
            falhas.push(format!(
                "=== {} born with warnings ===\n{}",
                alvo.nome,
                avisos.join("\n")
            ));
        }
    }
    assert!(
        falhas.is_empty(),
        "generated project does not compile cleanly with event parameter:\n\n{}",
        falhas.join("\n")
    );

    // WHAT WAS HERE, and why it is no longer here.
    //
    // The reverse waiver check: every type in `CARGA_SEM_QUEM_CONSTRUA` that
    // NO LONGER appeared as `never constructed` failed the gate, requiring
    // that line to be deleted. On 17/09/2026 it failed all five at once — form
    // event dispatch arrived —, the list was deleted and its check removed
    // with it, because checking an empty list is noise.
}

/// **The gate's gate.** If the proof spliced into the body did not enforce the type,
/// this file would measure less than advertised.
#[test]
fn a_prova_enxertada_amarra_nome_e_tipo() {
    let fonte = "impl F {\n    /// `x.on_key_down`\n    pub fn xKeyDown(&mut self, _e: KeyEvent) {\n        \n    }\n}\n";
    let saida = com_a_prova_no_corpo(fonte, "xKeyDown", "&mut self, _e: KeyEvent");
    assert!(
        saida.contains("let _prova0: KeyEvent = _e;"),
        "proof did not bind type:\n{saida}"
    );
    assert!(
        saida.contains("pub fn xKeyDown(&mut self, _e: KeyEvent) {"),
        "signature was touched, and it is what is being measured:\n{saida}"
    );
}

/// Two parameters generate two proofs, with distinct names.
#[test]
fn cada_parametro_ganha_a_sua_prova() {
    let fonte = "impl F {\n    pub fn y(&mut self, _a: u8, _b: char) {\n        \n    }\n}\n";
    let saida = com_a_prova_no_corpo(fonte, "y", "&mut self, _a: u8, _b: char");
    assert!(saida.contains("let _prova0: u8 = _a;"), "{saida}");
    assert!(saida.contains("let _prova1: char = _b;"), "{saida}");
}
