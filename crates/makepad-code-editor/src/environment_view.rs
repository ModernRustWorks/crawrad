//! Environment check dialog view.
//!
//! Visual presentation of required and optional toolchains with real-time status indicators,
//! installation progress, and command execution controls.

use makepad_widgets::*;

/// Head parts: (heading, intro, install_all_label, is_installing); an empty text is hidden.
pub type EnvironmentHeadParts = (String, String, String, bool);

/// Row parts of a single tool: (icon_color, title, mono_tag, state_word, purpose, detail, progress_ratio, output_line, btn_label).
pub type EnvironmentRowParts = (
    [f32; 4],
    String,
    String,
    String,
    String,
    String,
    Option<f32>,
    String,
    Option<String>,
);

/// Actions emitted by [`EnvironmentCheckView`].
#[derive(Clone, Debug, DefaultNone, PartialEq)]
pub enum EnvironmentViewAction {
    None,
    Primary(usize),
    CheckAgain,
    InstallAll,
    StopInstall,
    Cancel,
}

live_design! {
    use link::shaders::*;
    use link::widgets::*;
    use link::theme::*;

    EnvText = <Label> {
        width: Fit, height: Fit,
        draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 9.75 } color: #e7e7ea }
        text: ""
    }
    EnvMuted = <EnvText> { draw_text: { color: #98989b } }
    EnvCode = <EnvText> { draw_text: { text_style: <THEME_FONT_CODE> { font_size: 9.0 } color: #b7b7ba } }

    EnvButton = <Button> {
        height: 26.0,
        margin: 0.0,
        padding: { left: 6.0, right: 6.0, top: 0.0, bottom: 0.0 },
        align: { x: 0.5, y: 0.5 },
        draw_bg: {
            border_size: 1.0,
            border_radius: 2.0,
            color: #2b2b2d,
            color_hover: #343437,
            color_down: #2c455d,
            border_color_1: #4d4f52,
            border_color_2: #4d4f52,
        }
        draw_text: {
            color: #e7e7ea,
            color_hover: #f5f5f8,
            color_down: #f5f5f8,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.75 }
        }
    }

    EnvPrimaryButton = <Button> {
        height: 26.0,
        margin: 0.0,
        padding: { left: 6.0, right: 6.0, top: 0.0, bottom: 0.0 },
        align: { x: 0.5, y: 0.5 },
        draw_bg: {
            border_size: 1.0,
            border_radius: 2.0,
            color: #de5a2b,
            color_hover: #ea6328,
            color_down: #c34316,
            border_color_1: #b83d16,
            border_color_2: #b83d16,
        }
        draw_text: {
            color: #ffffff,
            text_style: <THEME_FONT_BOLD> { font_size: 9.75 }
        }
    }

    EnvCloseButton = <Button> {
        width: 24.0, height: 24.0,
        padding: 0.0, margin: 0.0,
        draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 12.0 } color: #98989b color_hover: #ffffff }
        draw_bg: { border_size: 0.0, color: #00000000, color_hover: #c42b1c, color_down: #c42b1c }
        text: "×"
    }

    EnvToolRow = <View> {
        width: Fill, height: Fit, flow: Right, align: { y: 0.5 },
        padding: { left: 4.0, right: 4.0, top: 2.0, bottom: 2.0 },
        info = <View> {
            width: Fill, height: Fit, flow: Down, spacing: 2.0,
            header = <View> {
                width: Fill, height: Fit, flow: Right, align: { y: 0.5 }, spacing: 4.0,
                icon = <EnvText> { text: "●" }
                title = <EnvText> {
                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 9.75 } }
                    text: ""
                }
                tag = <EnvCode> { text: "" }
                state = <EnvText> { text: "" }
            }
            purpose = <EnvMuted> { width: Fill, height: Fit, text: "" }
            detail = <View> { visible: false, width: Fill, height: Fit, text = <EnvMuted> { width: Fill, height: Fit, text: "" } }
            progress_bar = <View> {
                visible: false, width: Fill, height: 4.0,
                show_bg: true,
                draw_bg: { color: #2b2b2d }
                fill = <View> {
                    width: 0.0, height: Fill,
                    show_bg: true,
                    draw_bg: { color: #de5a2b }
                }
            }
            output_line = <View> { visible: false, width: Fill, height: Fit, text = <EnvCode> { width: Fill, height: Fit, text: "" } }
        }
        btn = <EnvButton> { width: 170.0, height: 26.0, text: "" }
    }

    pub EnvironmentCheckView = {{EnvironmentCheckView}} {
        width: 820.0, height: Fit,
        flow: Down,

        // Title bar
        title_bar = <View> {
            width: Fill, height: 30.0,
            flow: Right, align: { y: 0.5 },
            padding: { left: 10.0, right: 4.0, top: 0.0, bottom: 0.0 },
            show_bg: true,
            draw_bg: { color: #161819 }
            env_title = <EnvText> {
                draw_text: {
                    color: #f5f5f8,
                    text_style: <THEME_FONT_BOLD> { font_size: 9.75 }
                }
                text: "Environment Check"
            }
            <View> { width: Fill, height: 1.0 }
            env_close = <EnvCloseButton> {}
        }

        // Body
        body = <View> {
            width: Fill, height: Fit,
            flow: Down,
            padding: 10.0,
            spacing: 6.0,

            env_heading = <EnvText> {
                draw_text: {
                    color: #f5f5f8,
                    text_style: <THEME_FONT_BOLD> { font_size: 16.5 }
                }
                text: "Environment Check"
            }
            env_subheading = <View> {
                width: Fill, height: Fit,
                text = <EnvMuted> { width: Fill, height: Fit, text: "" }
            }
            env_row_0 = <EnvToolRow> {}
            env_row_1 = <EnvToolRow> {}
            env_row_2 = <EnvToolRow> {}
            env_row_3 = <EnvToolRow> {}
            env_row_4 = <EnvToolRow> {}
        }

        // Footer
        footer = <View> {
            width: Fill, height: 36.0,
            flow: Right, align: { y: 0.5 },
            spacing: 4.0,
            padding: { left: 10.0, right: 10.0, top: 0.0, bottom: 10.0 },

            btn_check_again = <EnvButton> { width: 80.0, text: "Check again" }
            btn_stop = <EnvButton> { width: 80.0, visible: false, text: "Stop install" }
            <View> { width: Fill, height: 1.0 }
            btn_install_all = <EnvPrimaryButton> { width: 180.0, text: "" }
            btn_cancel = <EnvButton> { width: 80.0, text: "Cancel" }
        }
    }
}

const ROW_IDS: [LiveId; 5] = [
    live_id!(env_row_0),
    live_id!(env_row_1),
    live_id!(env_row_2),
    live_id!(env_row_3),
    live_id!(env_row_4),
];

/// The Environment Check dialog view.
#[derive(Live, LiveHook, Widget)]
pub struct EnvironmentCheckView {
    #[deref]
    view: View,
}

impl Widget for EnvironmentCheckView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));
        let act = self.action(&actions);
        if act != EnvironmentViewAction::None {
            cx.widget_action(self.widget_uid(), &scope.path, act);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl EnvironmentCheckView {
    /// Applies head parts and rows to the dialog controls.
    pub fn set_view(
        &mut self,
        cx: &mut Cx,
        head: EnvironmentHeadParts,
        rows: Vec<EnvironmentRowParts>,
    ) {
        let (heading, intro, install_all_label, is_installing) = head;
        self.view.label(id!(env_heading)).set_text(cx, &heading);
        self.view
            .label(id!(env_subheading.text))
            .set_text(cx, &intro);
        self.view
            .view(id!(env_subheading))
            .set_visible(cx, !intro.is_empty());
        self.view
            .button(id!(btn_stop))
            .set_visible(cx, is_installing);
        let btn_install = self.view.button(id!(btn_install_all));
        btn_install.set_text(cx, &install_all_label);
        btn_install.set_visible(cx, !is_installing && !install_all_label.is_empty());

        for (i, row_id) in ROW_IDS.iter().enumerate() {
            let row_view = self.view.view(&[*row_id]);
            if let Some((
                color,
                title,
                tag,
                state,
                purpose,
                detail,
                progress,
                output_line,
                btn_label,
            )) = rows.get(i)
            {
                let vcolor = vec4(color[0], color[1], color[2], color[3]);
                row_view
                    .label(id!(icon))
                    .apply_over(cx, live! { draw_text: { color: (vcolor) } });
                row_view.label(id!(title)).set_text(cx, title);
                row_view.label(id!(tag)).set_text(cx, tag);
                row_view.label(id!(state)).set_text(cx, state);
                row_view.label(id!(purpose)).set_text(cx, purpose);

                row_view.label(id!(detail.text)).set_text(cx, detail);
                row_view
                    .view(id!(detail))
                    .set_visible(cx, !detail.is_empty());

                let progress_bar = row_view.view(id!(progress_bar));
                let output_label = row_view.view(id!(output_line));
                row_view
                    .label(id!(output_line.text))
                    .set_text(cx, output_line);
                if let Some(ratio) = progress {
                    progress_bar.set_visible(cx, true);
                    let bar_width = 380.0 * ratio;
                    row_view
                        .view(id!(progress_bar.fill))
                        .apply_over(cx, live! { width: (bar_width) });
                    output_label.set_visible(cx, !output_line.is_empty());
                } else {
                    progress_bar.set_visible(cx, false);
                    output_label.set_visible(cx, false);
                }

                let btn = row_view.button(id!(btn));
                if let Some(label) = btn_label {
                    btn.set_visible(cx, true);
                    btn.set_text(cx, label);
                } else {
                    btn.set_visible(cx, false);
                }
            }
        }
        self.redraw(cx);
    }

    /// Evaluates user actions on the dialog.
    pub fn action(&mut self, actions: &Actions) -> EnvironmentViewAction {
        if self.view.button(id!(env_close)).clicked(actions)
            || self.view.button(id!(btn_cancel)).clicked(actions)
        {
            return EnvironmentViewAction::Cancel;
        }
        if self.view.button(id!(btn_stop)).clicked(actions) {
            return EnvironmentViewAction::StopInstall;
        }
        if self.view.button(id!(btn_check_again)).clicked(actions) {
            return EnvironmentViewAction::CheckAgain;
        }
        if self.view.button(id!(btn_install_all)).clicked(actions) {
            return EnvironmentViewAction::InstallAll;
        }

        for (i, row_id) in ROW_IDS.iter().enumerate() {
            if self.view.view(&[*row_id]).button(id!(btn)).clicked(actions) {
                return EnvironmentViewAction::Primary(i);
            }
        }

        EnvironmentViewAction::None
    }

    /// Serialized status for diagnostic and testing queries.
    pub fn rows_json(&self) -> String {
        serde_json::json!({
            "heading": self.view.label(id!(env_heading)).text(),
            "install_all": self.view.button(id!(btn_install_all)).text(),
        })
        .to_string()
    }
}

impl EnvironmentCheckViewRef {
    pub fn set_view(
        &self,
        cx: &mut Cx,
        head: EnvironmentHeadParts,
        rows: Vec<EnvironmentRowParts>,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_view(cx, head, rows);
        }
    }

    pub fn action(&self, actions: &Actions) -> EnvironmentViewAction {
        // The view captures its buttons' actions and re-emits one of its own (see
        // `handle_event`): a mouse click only ever reaches the caller as that action.
        let own = actions
            .find_widget_action(self.widget_uid())
            .cast::<EnvironmentViewAction>();
        if own != EnvironmentViewAction::None {
            return own;
        }
        self.borrow_mut()
            .map_or(EnvironmentViewAction::None, |mut inner| {
                inner.action(actions)
            })
    }

    pub fn rows_json(&self) -> String {
        self.borrow()
            .map_or_else(|| "{}".into(), |inner| inner.rows_json())
    }
}
