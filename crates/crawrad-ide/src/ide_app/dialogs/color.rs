use super::super::*;
use crawrad_actions::color_dialog::{Channel, ColorDialog, ColorTarget};
use crawrad_ir::Color;
use crawrad_note::note;
use crawrad_runtime_makepad::{CrawradColorSquareWidgetRefExt, CrawradHueStripWidgetRefExt};

#[note("ide-app-dialogs-color-fields")]
const FIELDS: [(&[LiveId], Option<Channel>); 5] = [
    (id!(color_hex), None),
    (id!(color_red), Some(Channel::Red)),
    (id!(color_green), Some(Channel::Green)),
    (id!(color_blue), Some(Channel::Blue)),
    (id!(color_alpha), Some(Channel::Alpha)),
];

fn rgba(c: Color) -> Vec4 {
    vec4(c.r as f32, c.g as f32, c.b as f32, c.a as f32) / 255.0
}

#[note("ide-app-dialogs-color-patches")]
fn patches(recent: &[Color]) -> impl Iterator<Item = (LiveId, Option<Color>)> + '_ {
    let basic = ColorDialog::basic().into_iter().enumerate();
    let basic = basic.map(|(i, (_, c))| (format!("color_basic_{i}"), Some(c)));
    let recent = (0..8).map(|i| (format!("color_recent_{i}"), recent.get(i).copied()));
    basic.chain(recent).map(|(n, c)| (LiveId::from_str(&n), c))
}

#[note("ide-app-dialogs-color-write-fields")]
fn write_fields(cx: &mut Cx, dlg: &ViewRef, model: &ColorDialog) {
    let f = model.fields();
    for ((path, _), text) in FIELDS.iter().zip([f.hex, f.red, f.green, f.blue, f.alpha]) {
        dlg.text_input(path).set_text(cx, &text);
    }
}

impl IdeApp {
    #[note("ide-app-dialogs-color-ideapp-open-color")]
    pub(in super::super) fn open_color(
        &mut self,
        cx: &mut Cx,
        title: String,
        current: Color,
        target: ColorTarget,
    ) {
        let pref = crawrad_project::workspace::read_preferences();
        let recent = pref
            .color_recent
            .iter()
            .filter_map(|h| Color::from_hex(h))
            .collect();
        let model = ColorDialog::new(title, current, recent);
        let dlg = self.ui.view(id!(color_dialog.dlg));
        dlg.label(id!(color_error)).set_text(cx, "");
        write_fields(cx, &dlg, &model);
        self.color = Some((model, target));
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-color-ideapp-open-color-for-property")]
    pub(in super::super) fn open_color_for_property(
        &mut self,
        cx: &mut Cx,
        node: NodeId,
        key: &str,
    ) {
        let current = self
            .linhas_do_inspector()
            .into_iter()
            .find_map(|row| match row {
                inspector::Row::Prop(p) if p.key == key => p.editor.window_color(),
                _ => None,
            });
        let Some(current) = current else { return };
        let target = ColorTarget::Property {
            node: node.to_string(),
            key: key.to_string(),
        };
        self.open_color(cx, format!("Color · {key}"), current, target);
    }

    #[note("ide-app-dialogs-color-ideapp-draw-color")]
    pub(in super::super) fn draw_color(&mut self, cx: &mut Cx) {
        self.show_veil(cx, id!(color_dialog), self.color.is_some());
        let Some((model, _)) = self.color.as_ref() else {
            return;
        };
        let dlg = self.ui.view(id!(color_dialog.dlg));
        dlg.label(id!(color_title)).set_text(cx, &model.title);
        let (s, v) = model.saturation_value();
        dlg.crawrad_color_square(id!(color_square))
            .set(cx, model.hue(), s, v);
        dlg.crawrad_hue_strip(id!(color_hue))
            .set(cx, model.hue() / 360.0);
        for (name, c) in [
            ("color_current", model.current),
            ("color_new", model.chosen),
        ] {
            let c = rgba(c);
            let patch = dlg.view(&[LiveId::from_str(name)]);
            patch.apply_over(cx, live! { draw_bg: { color: (c) } });
            patch.redraw(cx);
        }
        for (name, c) in patches(&model.recent) {
            let patch = dlg.view(&[name]);
            patch.set_visible(cx, c.is_some());
            if let Some(c) = c.map(rgba) {
                patch.apply_over(cx, live! { draw_bg: { color: (c) } });
                patch.redraw(cx);
            }
        }
    }

    #[note("ide-app-dialogs-color-ideapp-handle-color")]
    pub(in super::super) fn handle_color(&mut self, cx: &mut Cx, actions: &Actions) {
        let Some((model, _)) = self.color.as_mut() else {
            return;
        };
        let dlg = self.ui.view(id!(color_dialog.dlg));
        let (mut said, before) = (None, model.chosen);
        if let Some((s, v)) = dlg.crawrad_color_square(id!(color_square)).picked(actions) {
            model.pick_square(s, v);
        } else if let Some(f) = dlg.crawrad_hue_strip(id!(color_hue)).picked(actions) {
            model.pick_hue(f);
        }
        let clicked = patches(&model.recent)
            .find(|(n, _)| dlg.view(&[*n]).button(id!(pick)).clicked(actions));
        if let Some((_, Some(c))) = clicked {
            model.pick(c);
        }
        let moved = model.chosen != before || said.is_some();
        for (path, channel) in FIELDS {
            if let Some((text, _)) = dlg.text_input(path).returned(actions) {
                let set = match channel {
                    None => model.set_hex(&text),
                    Some(ch) => model.set_channel(ch, &text),
                };
                said = Some(set.err().unwrap_or_default());
            }
        }
        if moved || said.is_some() {
            write_fields(cx, &dlg, model);
        }
        if let Some(said) = &said {
            dlg.label(id!(color_error)).set_text(cx, said);
        }
        let escaped = FIELDS
            .iter()
            .any(|(p, _)| dlg.text_input(p).escaped(actions));
        if dlg.button(id!(color_ok)).clicked(actions) {
            if let Some((model, target)) = self.color.take() {
                self.accept_color(cx, model, target);
            }
        } else if escaped || dlg.button(id!(color_cancel)).clicked(actions) {
            self.color = None;
        } else if !moved && said.is_none() {
            return;
        }
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-color-ideapp-accept-color")]
    fn accept_color(&mut self, cx: &mut Cx, model: ColorDialog, target: ColorTarget) {
        let (chosen, recent) = model.accept();
        let recent = recent.into_iter().map(Color::to_hex).collect();
        crawrad_project::workspace::update_preferences(|p| p.color_recent = recent);
        match target {
            ColorTarget::Property { node, key } => {
                let set = inspector::Outcome::Set(crawrad_ir::Value::Color(chosen));
                self.aplicar_resultado(cx, set, Some(&NodeId::from(node)), &key);
            }
            ColorTarget::SchemeField(i) => {
                let dlg = self.ui.view(id!(options_dialog.dlg));
                let input = dlg.text_input(&super::options::scheme(i, "val"));
                input.set_text(cx, &chosen.to_hex());
                self.apply_scheme_fields(cx);
            }
        }
    }
}
