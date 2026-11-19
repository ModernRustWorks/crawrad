//! **A control linked to an Action runs it and takes its state**, from
//! the link and the `Bind` table — no code per class.

use crate::targets::makepad::action_link::action_links;
use crawrad_ir::{Document, Rect, Value};
use crawrad_registry::Registry;

/// A form with an Action `act_save` (with `on_execute`/`on_update`) and a
/// `class` named `ctl` linked to it.
fn linked(class: &str) -> Document {
    let r = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    let mut a = r
        .instantiate("Action", "act_save", Rect::new(0.0, 0.0, 24.0, 24.0))
        .expect("the catalogue instantiates an Action");
    a.set_name("act_save");
    a.props.insert("text".into(), Value::Str("Save".into()));
    a.events
        .insert("on_execute".into(), "act_save_execute".into());
    a.events
        .insert("on_update".into(), "act_save_update".into());
    d.add_child("form_main", a).expect("into the form");
    let mut c = r
        .instantiate(class, "ctl", Rect::new(8.0, 8.0, 80.0, 24.0))
        .expect("the catalogue instantiates the control");
    c.set_name("ctl");
    c.props
        .insert("action".into(), Value::Ref("act_save".into()));
    d.add_child("form_main", c).expect("into the form");
    d
}

fn links(class: &str) -> (String, String) {
    let d = linked(class);
    let nodes = [
        d.node("act_save").expect("the action"),
        d.node("ctl").expect("the control"),
    ];
    let l = action_links(&d, &nodes).expect("emitting");
    (l.dispatch, l.pre_sync)
}

#[test]
fn the_controls_click_runs_the_action_while_it_is_enabled() {
    for class in ["Button", "BitBtn", "SpeedButton"] {
        let (dispatch, _) = links(class);
        assert!(
            dispatch.contains("if self.form.act_save.enabled && "),
            "{class}: {dispatch}"
        );
        assert!(
            dispatch.contains("self.form.act_save_execute();"),
            "{class}: {dispatch}"
        );
    }
}

#[test]
fn on_update_runs_first_and_the_action_owns_the_state() {
    let (_, pre) = links("Button");
    let update = pre
        .find("self.form.act_save_update();")
        .expect("OnUpdate runs");
    let enabled = pre
        .find("self.form.ctl.enabled = self.form.act_save.enabled;")
        .expect("Enabled passes");
    assert!(
        update < enabled,
        "OnUpdate decides the state the control then takes:\n{pre}"
    );
    assert!(
        pre.contains("self.form.ctl.visible = self.form.act_save.visible;"),
        "{pre}"
    );
    assert!(
        pre.contains("self.form.ctl.text = self.form.act_save.text.clone();"),
        "{pre}"
    );
}

#[test]
fn on_egui_the_control_takes_the_actions_caption_too() {
    for class in ["Button", "BitBtn", "SpeedButton"] {
        let d = linked(class);
        let node = d.node("ctl").expect("the control");
        let code =
            crate::targets::egui::layout::caption_from_action(&d, node, "ctl").expect("emitting");
        assert!(
            code.contains("self.ctl.text = self.act_save.text.clone();"),
            "{class}: {code}"
        );
    }
}
