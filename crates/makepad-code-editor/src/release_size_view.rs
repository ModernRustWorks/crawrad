//! The Size page and Shrink dialog widgets.
//!
//! Visual presentation of the Release size breakdown and the `[profile.release]`
//! optimization tips dialog.

use makepad_widgets::*;

/// Plain data parts for populating [`ReleaseSizeView`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SizePageParts {
    pub header: String,
    pub size: String,
    pub change: Option<(String, u8)>, // 0 grew, 1 shrank, 2 same
    pub since: Option<String>,
    pub bars: Vec<f64>,
    pub crates: Vec<(String, String, String, String, bool)>, // name, size, share, change, own
    pub functions: Vec<(String, String)>,
    pub footnote: String,
}

impl
    From<(
        String,
        String,
        Option<(String, u8)>,
        Option<String>,
        Vec<f64>,
        Vec<(String, String, String, String, bool)>,
        Vec<(String, String)>,
        String,
    )> for SizePageParts
{
    fn from(
        tuple: (
            String,
            String,
            Option<(String, u8)>,
            Option<String>,
            Vec<f64>,
            Vec<(String, String, String, String, bool)>,
            Vec<(String, String)>,
            String,
        ),
    ) -> Self {
        Self {
            header: tuple.0,
            size: tuple.1,
            change: tuple.2,
            since: tuple.3,
            bars: tuple.4,
            crates: tuple.5,
            functions: tuple.6,
            footnote: tuple.7,
        }
    }
}

/// Actions emitted by [`ReleaseSizeView`].
#[derive(Clone, Debug, DefaultNone, PartialEq)]
pub enum ReleaseSizeAction {
    None,
    Shrink,
}

/// Actions emitted by [`ShrinkDialogView`].
#[derive(Clone, Debug, DefaultNone, PartialEq)]
pub enum ShrinkDialogAction {
    None,
    Toggled(Vec<usize>),
    Cancel,
    Apply(Vec<usize>),
    ApplyAndBuild(Vec<usize>),
}

live_design! {
    use link::shaders::*;
    use link::widgets::*;
    use link::theme::*;

    SizeText = <Label> {
        width: Fit, height: Fit,
        draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } color: #e7e7ea }
        text: ""
    }
    SizeMuted = <SizeText> { draw_text: { color: #98989b } }
    SizeCode = <SizeText> { draw_text: { text_style: <THEME_FONT_CODE> { font_size: 9.0 } } }

    ShrinkRow = <View> {
        width: Fill, height: Fit,
        flow: Right, align: { y: 0.5 },
        spacing: 10.0,

        check = <CheckBox> { width: Fit, height: Fit, text: "" }
        <View> {
            width: Fill, height: Fit,
            flow: Down, spacing: 2.0,
            <View> {
                width: Fill, height: Fit,
                flow: Right, align: { y: 0.5 }, spacing: 6.0,
                key = <SizeCode> {
                    draw_text: { color: #e7e7ea }
                    text: ""
                }
                label = <SizeText> {
                    width: Fill, height: Fit,
                    draw_text: { color: #e7e7ea }
                    text: ""
                }
            }
            cost = <SizeMuted> {
                width: Fill, height: Fit,
                text: ""
            }
        }
        <View> {
            width: 140.0, height: Fit,
            flow: Right, align: { x: 1.0, y: 0.5 },
            diff = <SizeCode> {
                draw_text: { color: #98989b }
                text: ""
            }
        }
    }

    SizeButton = <Button> {
        width: Fit, height: 24.0,
        padding: { left: 9.0, right: 9.0 },
        draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 8.5 } color: #e7e7ea color_hover: #ffffff }
        draw_bg: { border_radius: 4.0, border_size: 1.0, color: #2b2b2d, color_hover: #343437, color_down: #3a3a3d, border_color_1: #4d4f52, border_color_2: #4d4f52 }
    }

    CrateRow = <View> {
        width: Fill, height: 22.0,
        flow: Right, align: { y: 0.5 },
        padding: { left: 8.0, right: 8.0 },
        name = <SizeText> { width: Fill, height: Fit, text: "" }
        size = <SizeText> { width: 90.0, height: Fit, text: "" }
        share = <SizeText> { width: 70.0, height: Fit, text: "" }
        change = <SizeText> { width: 100.0, height: Fit, text: "" }
    }

    CrateRowOwn = <View> {
        width: Fill, height: 22.0,
        flow: Right, align: { y: 0.5 },
        padding: { left: 8.0, right: 8.0 },
        name = <SizeText> {
            width: Fill, height: Fit,
            draw_text: { color: #c9b6ff }
            text: ""
        }
        size = <SizeText> { width: 90.0, height: Fit, text: "" }
        share = <SizeText> { width: 70.0, height: Fit, text: "" }
        change = <SizeText> { width: 100.0, height: Fit, text: "" }
    }

    FuncRow = <View> {
        width: Fill, height: 22.0,
        flow: Right, align: { y: 0.5 },
        padding: { left: 8.0, right: 8.0 },
        name = <SizeText> { width: Fill, height: Fit, text: "" }
        size = <SizeText> { width: 90.0, height: Fit, text: "" }
    }

    pub ReleaseSizeView = {{ReleaseSizeView}} {
        width: Fill, height: Fill,
        flow: Overlay,

        bld_size_content = <View> {
            width: Fill, height: Fill,
            flow: Down,
            spacing: 8.0,

            // 1. HEADER
            <View> {
                width: Fill, height: Fit,
                flow: Down,
                spacing: 3.0,

                bld_size_header = <SizeMuted> { text: "" }
                <View> {
                    width: Fill, height: Fit,
                    flow: Right, align: { y: 0.5 },
                    spacing: 12.0,

                    bld_size_big = <SizeText> {
                        draw_text: {
                            text_style: <THEME_FONT_BOLD> { font_size: 18.0 }
                            color: #f5f5f8
                        }
                        text: ""
                    }
                    bld_size_change = <SizeText> {
                        draw_text: {
                            text_style: <THEME_FONT_BOLD> { font_size: 12.0 }
                        }
                        text: ""
                    }
                }
                bld_size_since = <SizeMuted> { text: "" }
            }

            // 2. LAST 6 RELEASE BUILDS
            <View> {
                width: Fill, height: Fit,
                flow: Down,
                spacing: 3.0,

                <SizeMuted> { text: "Last 6 Release builds" }
                <View> {
                    width: Fit, height: 32.0,
                    flow: Right, align: { y: 1.0 },
                    spacing: 5.0,

                    bld_bar_0 = <View> { show_bg: true, width: 12.0, height: 2.0, draw_bg: { color: #4d4f52 } }
                    bld_bar_1 = <View> { show_bg: true, width: 12.0, height: 2.0, draw_bg: { color: #4d4f52 } }
                    bld_bar_2 = <View> { show_bg: true, width: 12.0, height: 2.0, draw_bg: { color: #4d4f52 } }
                    bld_bar_3 = <View> { show_bg: true, width: 12.0, height: 2.0, draw_bg: { color: #4d4f52 } }
                    bld_bar_4 = <View> { show_bg: true, width: 12.0, height: 2.0, draw_bg: { color: #4d4f52 } }
                    bld_bar_5 = <View> { show_bg: true, width: 12.0, height: 2.0, draw_bg: { color: #de5a2b } }
                }
            }

            // 3. TABS (Crates | Functions) & SHRINK BUTTON
            <View> {
                width: Fill, height: 26.0,
                flow: Right, align: { y: 0.5 },
                spacing: 6.0,

                bld_tab_crates = <SizeButton> { width: Fit, height: 24.0, text: "Crates" }
                bld_tab_functions = <SizeButton> { width: Fit, height: 24.0, text: "Functions" }
                <View> { width: Fill, height: 1.0 }
                bld_btn_shrink = <SizeButton> { width: Fit, height: 24.0, text: "Shrink…" }
            }

            // 4. TABLE
            <RoundedView> {
                width: Fill, height: Fill,
                flow: Down,
                padding: 6.0,
                spacing: 4.0,
                show_bg: true,
                draw_bg: { border_radius: 2.0, border_size: 1.0, color: #161819, border_color: #4d4f52 }

                bld_crates_header = <View> {
                    width: Fill, height: 20.0,
                    flow: Right, align: { y: 0.5 },
                    padding: { left: 8.0, right: 8.0 },
                    <SizeMuted> { width: Fill, height: Fit, text: "Crate" }
                    <SizeMuted> { width: 90.0, height: Fit, text: "Size" }
                    <SizeMuted> { width: 70.0, height: Fit, text: "Share" }
                    <SizeMuted> { width: 100.0, height: Fit, text: "Change" }
                }
                bld_funcs_header = <View> {
                    visible: false,
                    width: Fill, height: 20.0,
                    flow: Right, align: { y: 0.5 },
                    padding: { left: 8.0, right: 8.0 },
                    <SizeMuted> { width: Fill, height: Fit, text: "Function" }
                    <SizeMuted> { width: 90.0, height: Fit, text: "Size" }
                }

                bld_size_list = <PortalList> {
                    width: Fill, height: Fill,
                    scroll_bar: <ScrollBar> {}
                    CrateRow = <CrateRow> {}
                    CrateRowOwn = <CrateRowOwn> {}
                    FuncRow = <FuncRow> {}
                }
            }

            // 5. FOOTNOTE
            bld_size_footnote = <SizeMuted> {
                width: Fill, height: Fit,
                text: ""
            }
        }

        bld_size_empty = <View> {
            width: Fill, height: Fill,
            visible: false,
            flow: Down,
            align: { x: 0.5, y: 0.5 },
            padding: 24.0,

            <SizeMuted> {
                width: 480.0, height: Fit,
                draw_text: {
                    wrap: Word,
                    text_style: <THEME_FONT_REGULAR> { font_size: 10.0 }
                }
                text: "No Release build yet. Choose Release in Profile and click Build: the size of the executable and what weighs in it will show here."
            }
        }
    }

    pub ShrinkDialogView = {{ShrinkDialogView}} {
        visible: false,
        abs_pos: vec2(0.0, 0.0),
        width: Fill, height: Fill,
        flow: Overlay,
        align: { x: 0.5, y: 0.5 },
        show_bg: true,
        draw_bg: { color: #00000088 }

        dlg = <RoundedView> {
            width: 640.0, height: Fit,
            flow: Down,
            show_bg: true,
            draw_bg: { border_radius: 3.0, border_size: 1.0, color: #161819, border_color: #4d4f52 }

            // Title bar
            <View> {
                width: Fill, height: 38.0,
                flow: Right, align: { y: 0.5 },
                padding: { left: 16.0, right: 12.0 },
                show_bg: true,
                draw_bg: { color: #212324 }

                shrink_title = <SizeText> {
                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 10.5 } color: #f5f5f8 }
                    text: "Shrink the Release build"
                }
                <View> { width: Fill, height: 1.0 }
                shrink_close = <Button> {
                    width: 24.0, height: 24.0,
                    draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 12.0 } color: #98989b color_hover: #ffffff }
                    draw_bg: { border_size: 0.0, color: #00000000, color_hover: #343437 }
                    text: "×"
                }
            }

            // Body
            <View> {
                width: Fill, height: Fit,
                flow: Down,
                padding: 16.0,
                spacing: 12.0,

                shrink_sub = <SizeMuted> {
                    width: Fill, height: Fit,
                    text: "[profile.release] in Cargo.toml"
                }

                // Five recommendation rows
                shrink_row_0 = <ShrinkRow> {}

                shrink_row_1 = <ShrinkRow> {}

                shrink_row_2 = <ShrinkRow> {}

                shrink_row_3 = <ShrinkRow> {}

                shrink_row_4 = <ShrinkRow> {}

                // Preview box
                <View> {
                    width: Fill, height: Fit,
                    flow: Down, spacing: 4.0,

                    shrink_preview_title = <SizeMuted> {
                        width: Fill, height: Fit,
                        text: "What Cargo.toml will get"
                    }

                    <RoundedView> {
                        width: Fill, height: Fit,
                        flow: Down,
                        padding: 8.0,
                        spacing: 3.0,
                        show_bg: true,
                        draw_bg: { border_radius: 2.0, border_size: 1.0, color: #101214, border_color: #4d4f52 }

                        shrink_preview_hdr = <SizeCode> {
                            draw_text: { color: #7a7a7d }
                            text: "[profile.release]"
                        }
                        shrink_preview_empty = <View> {
                            width: Fill, height: Fit, visible: false,
                            line = <SizeCode> {
                                draw_text: { color: #606063 }
                                text: "No change."
                            }
                        }
                        shrink_prev_0 = <View> {
                            width: Fill, height: Fit, flow: Down,
                            shrink_add_0 = <View> {
                                width: Fill, height: Fit, visible: false,
                                line = <SizeCode> {
                                    draw_text: { color: #7cb46b }
                                    text: ""
                                }
                            }
                            shrink_rem_0 = <View> {
                                width: Fill, height: Fit, visible: false,
                                line = <SizeCode> {
                                    draw_text: { color: #e2726b }
                                    text: ""
                                }
                            }
                        }
                        shrink_prev_1 = <View> {
                            width: Fill, height: Fit, flow: Down,
                            shrink_add_1 = <View> {
                                width: Fill, height: Fit, visible: false,
                                line = <SizeCode> {
                                    draw_text: { color: #7cb46b }
                                    text: ""
                                }
                            }
                            shrink_rem_1 = <View> {
                                width: Fill, height: Fit, visible: false,
                                line = <SizeCode> {
                                    draw_text: { color: #e2726b }
                                    text: ""
                                }
                            }
                        }
                        shrink_prev_2 = <View> {
                            width: Fill, height: Fit, flow: Down,
                            shrink_add_2 = <View> {
                                width: Fill, height: Fit, visible: false,
                                line = <SizeCode> {
                                    draw_text: { color: #7cb46b }
                                    text: ""
                                }
                            }
                            shrink_rem_2 = <View> {
                                width: Fill, height: Fit, visible: false,
                                line = <SizeCode> {
                                    draw_text: { color: #e2726b }
                                    text: ""
                                }
                            }
                        }
                        shrink_prev_3 = <View> {
                            width: Fill, height: Fit, flow: Down,
                            shrink_add_3 = <View> {
                                width: Fill, height: Fit, visible: false,
                                line = <SizeCode> {
                                    draw_text: { color: #7cb46b }
                                    text: ""
                                }
                            }
                            shrink_rem_3 = <View> {
                                width: Fill, height: Fit, visible: false,
                                line = <SizeCode> {
                                    draw_text: { color: #e2726b }
                                    text: ""
                                }
                            }
                        }
                        shrink_prev_4 = <View> {
                            width: Fill, height: Fit, flow: Down,
                            shrink_add_4 = <View> {
                                width: Fill, height: Fit, visible: false,
                                line = <SizeCode> {
                                    draw_text: { color: #7cb46b }
                                    text: ""
                                }
                            }
                            shrink_rem_4 = <View> {
                                width: Fill, height: Fit, visible: false,
                                line = <SizeCode> {
                                    draw_text: { color: #e2726b }
                                    text: ""
                                }
                            }
                        }
                    }
                }

                // Footnote
                shrink_footnote = <SizeMuted> {
                    width: Fill, height: Fit,
                    text: "The gain is measured by the next Release build and shown on the Size page — never guessed."
                }
            }

            // Footer
            <View> {
                width: Fill, height: 44.0,
                flow: Right, align: { y: 0.5 },
                padding: { left: 16.0, right: 16.0 },
                spacing: 8.0,
                show_bg: true,
                draw_bg: { color: #161819 }

                <View> { width: Fill, height: 1.0 }
                shrink_cancel = <SizeButton> { text: "Cancel" }
                shrink_apply = <SizeButton> { text: "Apply" }
                shrink_apply_build = <SizeButton> {
                    text: "Apply and build Release"
                    draw_bg: { color: #de5a2b, color_hover: #e86d40, color_down: #c94e22, border_color_1: #de5a2b, border_color_2: #de5a2b }
                    draw_text: { color: #ffffff }
                }
            }
        }
    }
}

/// The Size page view: executable size breakdown and last 6 builds chart.
#[derive(Live, LiveHook, Widget)]
pub struct ReleaseSizeView {
    #[deref]
    view: View,
    #[rust(true)]
    is_crates: bool,
    #[rust]
    crates: Vec<(String, String, String, String, bool)>,
    #[rust]
    functions: Vec<(String, String)>,
}

impl Widget for ReleaseSizeView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));
        if self.view.button(id!(bld_tab_crates)).clicked(&actions) {
            self.set_tab(cx, true);
        } else if self.view.button(id!(bld_tab_functions)).clicked(&actions) {
            self.set_tab(cx, false);
        } else if self.view.button(id!(bld_btn_shrink)).clicked(&actions) {
            cx.widget_action(self.widget_uid(), &scope.path, ReleaseSizeAction::Shrink);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
            let list = step.as_portal_list();
            let Some(mut list) = list.borrow_mut() else {
                continue;
            };
            if self.is_crates {
                list.set_item_range(cx, 0, self.crates.len());
                while let Some(i) = list.next_visible_item(cx) {
                    let Some((name, size, share, change, own)) = self.crates.get(i) else {
                        continue;
                    };
                    let template = if *own {
                        live_id!(CrateRowOwn)
                    } else {
                        live_id!(CrateRow)
                    };
                    let item = list.item(cx, i, template);
                    item.label(id!(name)).set_text(cx, name);
                    item.label(id!(size)).set_text(cx, size);
                    item.label(id!(share)).set_text(cx, share);
                    item.label(id!(change)).set_text(cx, change);
                    item.draw_all(cx, scope);
                }
            } else {
                list.set_item_range(cx, 0, self.functions.len());
                while let Some(i) = list.next_visible_item(cx) {
                    let Some((name, size)) = self.functions.get(i) else {
                        continue;
                    };
                    let item = list.item(cx, i, live_id!(FuncRow));
                    item.label(id!(name)).set_text(cx, name);
                    item.label(id!(size)).set_text(cx, size);
                    item.draw_all(cx, scope);
                }
            }
        }
        DrawStep::done()
    }
}

impl ReleaseSizeView {
    /// Switches the table tab (true = Crates, false = Functions).
    pub fn set_tab(&mut self, cx: &mut Cx, is_crates: bool) {
        self.is_crates = is_crates;
        self.update_tab_look(cx);
        self.view.portal_list(id!(bld_size_list)).redraw(cx);
        self.redraw(cx);
    }

    fn update_tab_look(&self, cx: &mut Cx) {
        let (crates_btn, funcs_btn) = (
            self.view.button(id!(bld_tab_crates)),
            self.view.button(id!(bld_tab_functions)),
        );
        let active_bg = vec4(0.871, 0.353, 0.169, 1.0);
        let active_bdr = vec4(0.722, 0.239, 0.086, 1.0);
        let active_txt = vec4(1.0, 1.0, 1.0, 1.0);
        let inactive_bg = vec4(0.169, 0.169, 0.176, 1.0);
        let inactive_bdr = vec4(0.302, 0.310, 0.322, 1.0);
        let inactive_txt = vec4(0.906, 0.906, 0.918, 1.0);

        if self.is_crates {
            crates_btn.apply_over(cx, live! {
                draw_bg: { color: (active_bg), border_color_1: (active_bdr), border_color_2: (active_bdr) }
                draw_text: { color: (active_txt) }
            });
            funcs_btn.apply_over(cx, live! {
                draw_bg: { color: (inactive_bg), border_color_1: (inactive_bdr), border_color_2: (inactive_bdr) }
                draw_text: { color: (inactive_txt) }
            });
        } else {
            crates_btn.apply_over(cx, live! {
                draw_bg: { color: (inactive_bg), border_color_1: (inactive_bdr), border_color_2: (inactive_bdr) }
                draw_text: { color: (inactive_txt) }
            });
            funcs_btn.apply_over(cx, live! {
                draw_bg: { color: (active_bg), border_color_1: (active_bdr), border_color_2: (active_bdr) }
                draw_text: { color: (active_txt) }
            });
        }
        self.view
            .view(id!(bld_crates_header))
            .set_visible(cx, self.is_crates);
        self.view
            .view(id!(bld_funcs_header))
            .set_visible(cx, !self.is_crates);
    }

    /// Sets the structured data parts for the Size page.
    pub fn set_page(&mut self, cx: &mut Cx, parts: SizePageParts) {
        self.view.view(id!(bld_size_content)).set_visible(cx, true);
        self.view.view(id!(bld_size_empty)).set_visible(cx, false);
        self.view
            .label(id!(bld_size_header))
            .set_text(cx, &parts.header);
        self.view.label(id!(bld_size_big)).set_text(cx, &parts.size);

        let change_lbl = self.view.label(id!(bld_size_change));
        if let Some((ref change_str, trend)) = parts.change {
            change_lbl.set_text(cx, change_str);
            let color = match trend {
                0 => vec4(0.871, 0.353, 0.169, 1.0),
                1 => vec4(0.486, 0.706, 0.420, 1.0),
                _ => vec4(0.596, 0.596, 0.608, 1.0),
            };
            change_lbl.apply_over(cx, live! { draw_text: { color: (color) } });
        } else {
            change_lbl.set_text(cx, "");
        }

        let since_lbl = self.view.label(id!(bld_size_since));
        if let Some(ref since_str) = parts.since {
            since_lbl.set_text(cx, since_str);
        } else {
            since_lbl.set_text(cx, "");
        }

        let bar_ids = [
            id!(bld_bar_0),
            id!(bld_bar_1),
            id!(bld_bar_2),
            id!(bld_bar_3),
            id!(bld_bar_4),
            id!(bld_bar_5),
        ];
        let bar_count = parts.bars.len();
        for (idx, bar_id) in bar_ids.iter().enumerate() {
            let bar_view = self.view.view(*bar_id);
            if idx < bar_count {
                bar_view.set_visible(cx, true);
                let fraction = parts.bars.get(idx).copied().unwrap_or(0.0);
                let height = (fraction * 28.0).max(2.0);
                let is_newest = idx.saturating_add(1) == bar_count;
                let color = if is_newest {
                    vec4(0.871, 0.353, 0.169, 1.0)
                } else {
                    vec4(0.302, 0.310, 0.322, 1.0)
                };
                bar_view.apply_over(cx, live! { height: (height), draw_bg: { color: (color) } });
            } else {
                bar_view.set_visible(cx, false);
            }
        }

        self.crates = parts.crates;
        self.functions = parts.functions;

        self.update_tab_look(cx);
        self.view
            .label(id!(bld_size_footnote))
            .set_text(cx, &parts.footnote);
        self.view.portal_list(id!(bld_size_list)).redraw(cx);
        self.redraw(cx);
    }

    /// Shows the empty state when no release build exists yet.
    pub fn set_empty(&mut self, cx: &mut Cx) {
        self.view.view(id!(bld_size_content)).set_visible(cx, false);
        self.view.view(id!(bld_size_empty)).set_visible(cx, true);
        self.redraw(cx);
    }

    /// Evaluates actions performed on this view this frame.
    pub fn action(&mut self, actions: &Actions) -> ReleaseSizeAction {
        if self.view.button(id!(bld_btn_shrink)).clicked(actions) {
            return ReleaseSizeAction::Shrink;
        }
        let uid = self.widget_uid();
        if actions.iter().any(|a| {
            a.downcast_ref::<WidgetAction>().is_some_and(|w| {
                w.widget_uid == uid
                    && w.action.downcast_ref::<ReleaseSizeAction>()
                        == Some(&ReleaseSizeAction::Shrink)
            })
        }) {
            ReleaseSizeAction::Shrink
        } else {
            ReleaseSizeAction::None
        }
    }
}

impl ReleaseSizeViewRef {
    pub fn set_page(&self, cx: &mut Cx, parts: SizePageParts) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_page(cx, parts);
        }
    }

    pub fn set_empty(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_empty(cx);
        }
    }

    pub fn action(&self, actions: &Actions) -> ReleaseSizeAction {
        self.borrow_mut()
            .map_or(ReleaseSizeAction::None, |mut inner| inner.action(actions))
    }
}

/// The "Shrink the Release build" dialog view.
#[derive(Live, LiveHook, Widget)]
pub struct ShrinkDialogView {
    #[deref]
    view: View,
    #[rust]
    checked: Vec<usize>,
}

impl Widget for ShrinkDialogView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));
        let act = self.action(&actions);
        if act != ShrinkDialogAction::None {
            cx.widget_action(self.widget_uid(), &scope.path, act);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

/// The five rows of the dialog, one per `[profile.release]` key, in `ShrinkKey::ALL` order.
const SHRINK_ROWS: [LiveId; 5] = [
    live_id!(shrink_row_0),
    live_id!(shrink_row_1),
    live_id!(shrink_row_2),
    live_id!(shrink_row_3),
    live_id!(shrink_row_4),
];

const SHRINK_ADD_IDS: [LiveId; 5] = [
    live_id!(shrink_add_0),
    live_id!(shrink_add_1),
    live_id!(shrink_add_2),
    live_id!(shrink_add_3),
    live_id!(shrink_add_4),
];
const SHRINK_REM_IDS: [LiveId; 5] = [
    live_id!(shrink_rem_0),
    live_id!(shrink_rem_1),
    live_id!(shrink_rem_2),
    live_id!(shrink_rem_3),
    live_id!(shrink_rem_4),
];

impl ShrinkDialogView {
    /// Populates the five recommendation rows with labels and checked state.
    pub fn set_tips(
        &mut self,
        cx: &mut Cx,
        rows: &[(String, String, String, String)],
        checked: &[usize],
    ) {
        self.checked = checked.to_vec();
        for (i, (row, (key, label, cost, diff))) in SHRINK_ROWS.iter().zip(rows).enumerate() {
            self.view
                .check_box(&[*row, live_id!(check)])
                .set_active(cx, self.checked.contains(&i));
            self.view.label(&[*row, live_id!(key)]).set_text(cx, key);
            self.view
                .label(&[*row, live_id!(label)])
                .set_text(cx, label);
            self.view.label(&[*row, live_id!(cost)]).set_text(cx, cost);
            self.view.label(&[*row, live_id!(diff)]).set_text(cx, diff);
        }
        self.redraw(cx);
    }

    /// Sets the diff preview lines in the preview box and updates button states.
    pub fn set_preview(&mut self, cx: &mut Cx, lines: &[String]) {
        let is_empty = lines.is_empty();
        self.view
            .view(id!(dlg.shrink_preview_empty))
            .set_visible(cx, is_empty);

        for (i, (add_id, rem_id)) in SHRINK_ADD_IDS.iter().zip(SHRINK_REM_IDS.iter()).enumerate() {
            let line = lines.get(i);
            let is_add = line.is_some_and(|l| l.starts_with('+'));
            let is_rem = line.is_some() && !is_add;
            let text = line.map_or("", String::as_str);
            for (id, shown) in [(*add_id, is_add), (*rem_id, is_rem)] {
                self.view.view(&[id]).set_visible(cx, shown);
                self.view
                    .label(&[id, live_id!(line)])
                    .set_text(cx, if shown { text } else { "" });
            }
        }

        let apply_btn = self.view.button(id!(dlg.shrink_apply));
        apply_btn.as_button().set_enabled(cx, !is_empty);
        apply_btn.set_disabled(cx, is_empty);

        let apply_build_btn = self.view.button(id!(dlg.shrink_apply_build));
        apply_build_btn.as_button().set_enabled(cx, !is_empty);
        apply_build_btn.set_disabled(cx, is_empty);
    }

    /// Evaluates user actions in the shrink dialog this frame.
    pub fn action(&mut self, actions: &Actions) -> ShrinkDialogAction {
        if self.view.button(id!(dlg.shrink_close)).clicked(actions)
            || self.view.button(id!(dlg.shrink_cancel)).clicked(actions)
        {
            return ShrinkDialogAction::Cancel;
        }
        if self.view.button(id!(dlg.shrink_apply)).clicked(actions) {
            return ShrinkDialogAction::Apply(self.checked.clone());
        }
        if self
            .view
            .button(id!(dlg.shrink_apply_build))
            .clicked(actions)
        {
            return ShrinkDialogAction::ApplyAndBuild(self.checked.clone());
        }
        let mut toggled = false;
        for (i, row) in SHRINK_ROWS.iter().enumerate() {
            if let Some(on) = self
                .view
                .check_box(&[*row, live_id!(check)])
                .changed(actions)
            {
                if on && !self.checked.contains(&i) {
                    self.checked.push(i);
                    toggled = true;
                } else if !on && self.checked.contains(&i) {
                    self.checked.retain(|&x| x != i);
                    toggled = true;
                }
            }
        }
        if toggled {
            return ShrinkDialogAction::Toggled(self.checked.clone());
        }
        let uid = self.widget_uid();
        for a in actions {
            if let Some(w) = a.downcast_ref::<WidgetAction>()
                && w.widget_uid == uid
                && let Some(act) = w.action.downcast_ref::<ShrinkDialogAction>()
            {
                return act.clone();
            }
        }
        ShrinkDialogAction::None
    }
}

impl ShrinkDialogViewRef {
    pub fn set_tips(
        &self,
        cx: &mut Cx,
        rows: &[(String, String, String, String)],
        checked: &[usize],
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_tips(cx, rows, checked);
        }
    }

    pub fn set_preview(&self, cx: &mut Cx, lines: &[String]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_preview(cx, lines);
        }
    }

    pub fn action(&self, actions: &Actions) -> ShrinkDialogAction {
        self.borrow_mut()
            .map_or(ShrinkDialogAction::None, |mut inner| inner.action(actions))
    }
}
