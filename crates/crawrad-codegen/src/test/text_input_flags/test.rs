//! **A password field never shows the password; a read-only one cannot be
//! edited**: `password_char` and `readonly` reach the Makepad
//! `TextInput` — in its DSL and, at run time, in the sync.

use crawrad_ir::Value;

/// One control on the test form: its class, its name and the props it gets.
type Control<'a> = (&'a str, &'a str, &'a [(&'a str, Value)]);

/// The generated designer of a form with these controls.
fn designer(controls: &[Control]) -> String {
    let r = crawrad_registry::Registry::builtin();
    let mut d = crawrad_ir::Document::new("form_main", "Form");
    d.node_mut("form_main")
        .expect("the root")
        .set_rect(crawrad_ir::Rect::new(0.0, 0.0, 640.0, 480.0));
    for (i, &(class, name, props)) in controls.iter().enumerate() {
        let id = format!("n{i}");
        let at = crawrad_ir::Rect::new(8.0, 8.0 + 40.0 * i as f32, 200.0, 32.0);
        let mut n = r
            .instantiate(class, &id, at)
            .expect("the catalogue's class");
        n.set_name(name);
        for (key, value) in props.iter() {
            n.props.insert(key.to_string(), value.clone());
        }
        d.add_child("form_main", n).unwrap();
    }
    d.validate().unwrap();
    let root = d.node("form_main").unwrap();
    let nodes: Vec<_> = (0..controls.len())
        .map(|i| d.node(&format!("n{i}")).unwrap())
        .collect();
    super::form_designer_rs(&d, &r, root, "FormMain", &nodes).unwrap()
}

/// The DSL block of widget `name` in the generated code.
fn block<'a>(code: &'a str, name: &str) -> &'a str {
    let start = code.find(&format!("{name} = <TextInput>")).expect(name);
    let end = start + code[start..].find("\n            }").expect("its end");
    &code[start..end]
}

#[test]
fn a_password_edit_is_masked_and_a_plain_one_is_not() {
    let code = designer(&[
        (
            "Edit",
            "edt_pass",
            &[("password_char", Value::Str("*".into()))],
        ),
        ("Edit", "edt_name", &[]),
    ]);
    assert!(
        block(&code, "edt_pass").contains("is_password: true"),
        "{code}"
    );
    assert!(!block(&code, "edt_name").contains("is_password"), "{code}");
    // at run time: a password_char set or cleared by the form's code follows
    assert!(code.contains("let masked = !self.form.edt_pass.password_char.is_empty();"));
    assert!(code.contains("ti.set_is_password(cx, masked);"));
}

#[test]
fn read_only_reaches_the_edit_and_the_memo() {
    let code = designer(&[
        ("Edit", "edt_ro", &[("readonly", Value::Bool(true))]),
        ("Memo", "mmo_ro", &[("readonly", Value::Bool(true))]),
        ("Memo", "mmo_rw", &[]),
    ]);
    assert!(
        block(&code, "edt_ro").contains("is_read_only: true"),
        "{code}"
    );
    assert!(
        block(&code, "mmo_ro").contains("is_read_only: true"),
        "{code}"
    );
    assert!(!block(&code, "mmo_rw").contains("is_read_only"), "{code}");
    // the Memo keeps `readonly` as a field, so the form's code can change it;
    // a disabled box takes no typing either
    for field in ["mmo_ro", "edt_ro"] {
        let rule =
            format!("let read_only = self.form.{field}.readonly || !self.form.{field}.enabled;");
        assert!(code.contains(&rule), "{rule}\n{code}");
    }
    assert!(code.contains("ti.set_is_read_only(cx, read_only);"));
    // a Memo has no password_char: its sync never asks for one
    assert!(!code.contains("self.form.mmo_ro.password_char"));
}

/// **A disabled control is dimmed, takes no clicks and fires no event**.
#[test]
fn enabled_reaches_every_widget_and_guards_its_events() {
    let code = designer(&[
        ("Button", "btn_save", &[("enabled", Value::Bool(false))]),
        ("CheckBox", "chk_opt", &[]),
        ("Edit", "edt_name", &[]),
    ]);
    for field in ["btn_save", "chk_opt", "edt_name"] {
        let apply = format!(
            "crawrad_runtime_makepad::enabled::apply(cx, &self.ui, id!({field}), self.form.{field}.enabled);"
        );
        assert!(code.contains(&apply), "{apply}\n{code}");
    }
    // a disabled box shows its tick only when it is checked (Makepad draws it always)
    assert!(code.contains(
        "crawrad_runtime_makepad::enabled::disabled_mark(cx, &self.ui, id!(chk_opt_box), self.form.chk_opt.checked);"
    ));
    // the dispatch: the CheckBox's change only reaches the form while it is enabled
    let guard = code
        .find("if self.form.chk_opt.enabled && let Some(_val) = self.ui.check_box(")
        .expect("the guard");
    let change = code
        .find("check_box(id!(chk_opt_box)).changed(actions)")
        .expect("its change");
    assert!(
        guard < change,
        "the change is read outside the guard:\n{code}"
    );
}

/// **A Timer ticks on Makepad**: the `App` keeps its `Ticker`, the sync
/// keeps it in step with `enabled` / `interval_ms`, and its tick calls `OnTimer`.
#[test]
fn a_timer_keeps_a_ticker_and_its_tick_calls_on_timer() {
    let r = crawrad_registry::Registry::builtin();
    let mut d = crawrad_ir::Document::new("form_main", "Form");
    d.node_mut("form_main")
        .expect("the root")
        .set_rect(crawrad_ir::Rect::new(0.0, 0.0, 640.0, 480.0));
    let mut t = r
        .instantiate("Timer", "n0", crawrad_ir::Rect::new(8.0, 8.0, 24.0, 24.0))
        .expect("the catalogue's Timer");
    t.set_name("tmr_tick");
    t.events.insert("on_timer".into(), "tmr_tick_timer".into());
    d.add_child("form_main", t).unwrap();
    d.validate().unwrap();
    let root = d.node("form_main").unwrap();
    let code = super::form_designer_rs(&d, &r, root, "FormMain", &[d.node("n0").unwrap()]).unwrap();
    for want in [
        "use crawrad_runtime_makepad::timer::Ticker as CrawradTicker;",
        "pub tmr_tick_ticker: CrawradTicker,",
        "self.tmr_tick_ticker.sync(cx, self.form.tmr_tick.enabled, self.form.tmr_tick.interval_ms);",
        "if self.tmr_tick_ticker.fired(event) {",
        "self.form.tmr_tick_timer();",
    ] {
        assert!(code.contains(want), "{want}\n{code}");
    }
    // the tick is handled before the widgets see the event
    let handle = &code[code.find("fn handle_event(").expect("handle_event")..];
    assert!(handle.find("fired(event)") < handle.find("self.match_event(cx, event)"));
}

/// **A ProgressBar shows its Position**: the bar starts empty in the
/// DSL and the sync sizes it to the position's fraction of the control.
#[test]
fn a_progress_bar_is_sized_to_its_position() {
    let code = designer(&[("ProgressBar", "pb_load", &[("position", Value::Int(35))])]);
    let dsl = &code[code.find("pb_load = <View>").expect("its DSL")..];
    let dsl = &dsl[..dsl.find("\n            }").expect("its end")];
    assert!(
        dsl.contains("width: 0.0, height: Fill"),
        "never drawn full by the DSL:\n{dsl}"
    );
    for want in [
        "crawrad_runtime_makepad::progress::fraction(",
        "self.form.pb_load.position,",
        "let vertical = self.form.pb_load.orientation == \"Vertical\";",
        "crawrad_runtime_makepad::progress::apply(cx, &self.ui, id!(pb_load), fraction, vertical, 200.0, 32.0);",
    ] {
        assert!(code.contains(want), "{want}\n{code}");
    }
}

/// **An Image draws its Picture**: a makepad `<Image>` with the fit the
/// classic RAD switches ask for, the bytes handed over by the sync, and what is
/// shown remembered in the `App`.
#[test]
fn an_image_draws_its_picture_with_the_fit_its_switches_ask_for() {
    let code = designer(&[
        ("Image", "img_plain", &[]),
        ("Image", "img_fill", &[("stretch", Value::Bool(true))]),
        (
            "Image",
            "img_fit",
            &[
                ("proportional", Value::Bool(true)),
                ("center", Value::Bool(true)),
            ],
        ),
    ]);
    let block = |name: &str| {
        let s = &code[code.find(&format!("{name} = <View>")).expect(name)..];
        s[..s.find("\n            }").expect("its end")].to_string()
    };
    assert!(block("img_plain").contains("img = <Image> { width: Fit, height: Fit, fit: Size }"));
    assert!(block("img_fill").contains("fit: Stretch"));
    assert!(
        block("img_fit").contains("fit: Smallest")
            && block("img_fit").contains("align: { x: 0.5, y: 0.5 }")
    );
    for want in [
        "use crawrad_runtime_makepad::picture::Shown as CrawradPictureShown;",
        "pub img_fit_shown: CrawradPictureShown,",
        "crawrad_runtime_makepad::picture::show(cx, &self.ui, id!(img_fit), self.form.img_fit.picture.bytes(), &mut self.img_fit_shown);",
    ] {
        assert!(code.contains(want), "{want}\n{code}");
    }
}

/// **A ComboBox shows the form's items at run time**: the sync hands
/// them over before the selection and the row limit, to our own widget.
#[test]
fn a_combo_box_hands_its_run_time_items_over_before_the_selection() {
    let code = designer(&[("ComboBox", "cmb_city", &[])]);
    let items = code
        .find("combo_box.set_items(cx, self.form.cmb_city.items.as_slice());")
        .expect("the items handed over");
    let selection = code
        .find("combo_box.set_selected(cx, self.form.cmb_city.selected);")
        .expect("the selection");
    assert!(
        items < selection,
        "the items must go first:
{code}"
    );
    assert!(
        code.contains("combo_box.set_drop_down_count(cx, self.form.cmb_city.drop_down_count);")
    );
    assert!(code.contains("cmb_city = <CrawradComboBox>"), "{code}");
}

/// **A TrackBar is a real slider**: makepad's `Slider` with the range,
/// whole steps and the orientation; the sync moves it, dragging writes the
/// position back.
#[test]
fn a_track_bar_is_a_slider_that_moves_and_reports() {
    let code = designer(&[
        ("TrackBar", "tb_h", &[("position", Value::Int(4))]),
        (
            "TrackBar",
            "tb_v",
            &[("orientation", Value::Enum("Vertical".into()))],
        ),
    ]);
    let block = |name: &str| {
        let s = &code[code.find(&format!("{name} = <SliderMinimal>")).expect(name)..];
        s[..s.find("\n            }").expect("its end")].to_string()
    };
    assert!(
        block("tb_h").contains("min: 0.0, max: 10.0, step: 1.0, default: 4.0"),
        "{}",
        block("tb_h")
    );
    assert!(block("tb_h").contains("axis: Horizontal"));
    assert!(block("tb_v").contains("axis: Vertical"));
    for want in [
        "self.ui.slider(id!(tb_h)).set_value(cx, self.form.tb_h.position as f64);",
        "let Some(_val) = self.ui.slider(id!(tb_h)).slided(actions) {",
        "self.form.tb_h.position = _val.round() as i64;",
    ] {
        assert!(code.contains(want), "{want}\n{code}");
    }
}
