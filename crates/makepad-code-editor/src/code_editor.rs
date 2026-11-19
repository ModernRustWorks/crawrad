use {
    crate::{
        Line, Selection, Token,
        decoration::{Decoration, DecorationType},
        history::NewGroup,
        layout::{BlockElement, LENS_HEIGHT, WrappedElement},
        note_popup::NotePopupState,
        selection::Affinity,
        session::{CodeSession, SelectionMode},
        str::StrExt,
        text::Position,
        token::TokenKind,
    },
    makepad_widgets::*,
    std::{mem, slice::Iter},
};

live_design! {
    use link::shaders::*;
    use link::theme::*;
    use link::widgets::*;
    use crate::note_icon::*;

    TokenColors = {{TokenColors}} {
        whitespace: #6E6E6E,
        delimiter: #a,
        delimiter_highlight: #f,
        error_decoration: #f00,
        warning_decoration: #0f0,
        unnecessary_decoration: #1e1e1e99,
        deprecated_decoration: #c0c0c0,
        semantic_macro: #E5C07B,
        semantic_trait: #B8D7A3,
        semantic_mutable: #F07B46,
        semantic_parameter: #9CDCFE,
        semantic_field: #x8EC6E6,
        semantic_enum_member: #4FC1FF,
        semantic_unsafe: #f38ba8,

        unknown: #C0C0C0,
        branch_keyword: #C485BE,
        constant: #CC917B,
        identifier: #D4D4D4,
        loop_keyword: #FF8C00,
        number: #B6CEAA,
        other_keyword: #5B9BD3,
        punctuator: #D4D4D4,
        string: #CC917B,
        function: #fffcc9,
        typename: #56C9B1,
        comment: #638D54,
    }

    // A diagnostic's message and line number (text) and the band behind its
    // line (band), per severity; Tools > Options > Color Scheme overrides.
    DiagnosticColors = {{DiagnosticColors}} {
        error: #f38ba8,
        warning: #fab387,
        info: #89b4fa,
        error_band: #3a2230,
        warning_band: #3a2c22,
        info_band: #1d2236,
    }

    DrawDecoration = {{DrawDecoration}} {
        fn pixel(self) -> vec4 {
            // Under the squiggle, the text faded (Unnecessary) or
            // struck through (Deprecated), drawn over the glyphs.
            let size = self.rect_size;
            let tag = Sdf2d::viewport(self.pos * size);
            if self.faded > 0.5 {
                tag.rect(0.0, 0.0, size.x, size.y);
                tag.fill(self.fade_color);
            }
            if self.struck > 0.5 {
                tag.move_to(0.0, size.y * 0.55);
                tag.line_to(size.x, size.y * 0.55);
                tag.stroke(self.strike_color, 1.0);
            }
            let transformed_pos = vec2(self.pos.x, self.pos.y + 0.03 * sin(self.pos.x * self.rect_size.x));
            let cx = Sdf2d::viewport(transformed_pos * self.rect_size);
            cx.move_to(0.0, self.rect_size.y - 1.0);
            cx.line_to(self.rect_size.x, self.rect_size.y - 1.0);
            let squiggle = cx.stroke(self.color, 0.8);
            return squiggle + tag.result * (1.0 - squiggle.w);
        }
    }

    DrawSelection = {{DrawSelection}} {
        uniform gloopiness: 8.0
        uniform border_radius: 2.0
        uniform focus: 1.0
        fn vertex(self) -> vec4 {
            let clipped: vec2 = clamp(
                self.geom_pos * vec2(self.rect_size.x + 16., self.rect_size.y) + self.rect_pos - vec2(8., 0.),
                self.draw_clip.xy,
                self.draw_clip.zw
            );
            self.pos = (clipped - self.rect_pos) / self.rect_size;
            return self.camera_projection * (self.camera_view * (
                self.view_transform * vec4(clipped.x, clipped.y, self.draw_depth + self.draw_zbias, 1.)
            ));
        }

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.rect_pos + self.pos * self.rect_size);
            sdf.box(
                self.rect_pos.x,
                self.rect_pos.y,
                self.rect_size.x,
                self.rect_size.y,
                self.border_radius
            );
            if self.prev_w > 0.0 {
                sdf.box(
                    self.prev_x,
                    self.rect_pos.y - self.rect_size.y,
                    self.prev_w,
                    self.rect_size.y,
                    self.border_radius
                );
                sdf.gloop(self.gloopiness);
            }
            if self.next_w > 0.0 {
                sdf.box(
                    self.next_x,
                    self.rect_pos.y + self.rect_size.y,
                    self.next_w,
                    self.rect_size.y,
                    self.border_radius
                );
                sdf.gloop(self.gloopiness);
            }
            return sdf.fill(mix(THEME_COLOR_U_1 * 1.1, THEME_COLOR_U_3 * 0.8, self.focus));
        }
    }

    DrawCodeText = {{DrawCodeText}} { }

    pub CodeEditor = {{CodeEditor}} {
        height: Fill, width: Fill,
        margin: 0,
        pad_left_top: vec2(0.0, 0.0)
        scroll_bars: <ScrollBars> {}
        glyph_margin: 28.0,
        gutter_pad: 2,
        draw_bg: { color: (THEME_COLOR_BG_CONTAINER) }
        draw_gutter: {
            draw_depth: 1.0,
            text_style: <THEME_FONT_CODE> {},
            color: (THEME_COLOR_LABEL_OUTER),
        }
        // The caret's line number: bold, in the IDE's Rust orange (CRAWRAD_ACENTO).
        draw_gutter_current_line: {
            draw_depth: 1.0,
            text_style: <THEME_FONT_BOLD> {},
            color: #de5a2b,
        }
        draw_text: {
            draw_depth: 1.0,
            text_style: <THEME_FONT_CODE> {}
            fn get_brightness(self)->float{
                return 1.1
            }

            fn blend_color(self, incol: vec4) -> vec4 {
                if self.outline < 0.5 {
                    return incol
                }
                if self.pos.y < 0.12 {
                    return #f
                }
                return incol
            }
        }
        draw_indent_guide: {
            color: #404040
        }
        draw_scope_header: {
            color: #1a3c6e
        }
        draw_decoration: {
          //  draw_depth: 2.0,
        }
        draw_selection: {
           // draw_depth: 3.0,
        }

        draw_cursor: {
          //  draw_depth: 4.0,
            instance focus: 0.0
            instance blink: 1.0
            fn pixel(self) -> vec4 {
                let color = mix(THEME_COLOR_U_HIDDEN, mix(self.color, THEME_COLOR_U_HIDDEN, self.blink),self.focus);
                return vec4(color.rgb*color.a, color.a);
            }
            color: (THEME_COLOR_WHITE),
        }


        draw_cursor_bg: {
            instance focus: 0.0
            fn pixel(self) -> vec4 {
                let color = mix(THEME_COLOR_U_HIDDEN, THEME_COLOR_U_1, self.focus);
                return vec4(color.rgb * color.a, color.a);
            }
        }

        draw_landing_line: {
            color: #FF8C0099
        }

        // The caret's current line: a subtle background band across the text area.
        draw_current_line: {
            color: #ffffff0a
        }

        // The line the program stopped on: a band across the text.
        draw_execution_line: {
            color: #f5c5422a
        }

        // The yellow arrow of the line the program stopped on.
        draw_execution_point: {
            draw_depth: 1.2,
            color: #F5C542,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let w = self.rect_size.x;
                let h = self.rect_size.y;
                sdf.move_to(0.5, h * 0.3);
                sdf.line_to(w * 0.5, h * 0.3);
                sdf.line_to(w * 0.5, 0.5);
                sdf.line_to(w - 0.5, h * 0.5);
                sdf.line_to(w * 0.5, h - 0.5);
                sdf.line_to(w * 0.5, h * 0.7);
                sdf.line_to(0.5, h * 0.7);
                sdf.close_path();
                sdf.fill(self.color);
                return sdf.result;
            }
        }

        // A sidecar note's anchor line: the folded-corner page.
        draw_note: <NoteFileIcon> {
            draw_depth: 1.05,
        }

        // A documentation note's anchor: a solid rounded page.
        draw_note_doc: <NoteDocIcon> {
            draw_depth: 1.05,
        }

        // A comment note's anchor: a speech bubble drawn as an outline only.
        draw_note_comment: <NoteCommentIcon> {
            draw_depth: 1.05,
        }

        // An alert indicator on an anchor: an orange dot #F0883E with a 1.4 px ring
        // in the editor background colour (#1E2127).
        draw_note_alert: {
            draw_depth: 1.06,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let w = self.rect_size.x;
                let h = self.rect_size.y;
                let r = min(w, h) * 0.5;
                sdf.circle(w * 0.5, h * 0.5, r);
                sdf.fill(#1E2127);
                sdf.circle(w * 0.5, h * 0.5, r - 1.4);
                sdf.fill(#F0883E);
                return sdf.result;
            }
        }

        // An extension's line mark: a little list tree in the glyph
        // margin, in the mark's colour; a click is reported.
        draw_line_mark: {
            draw_depth: 1.05,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let w = self.rect_size.x;
                let h = self.rect_size.y;
                sdf.move_to(w * 0.22, h * 0.15);
                sdf.line_to(w * 0.22, h * 0.82);
                sdf.move_to(w * 0.22, h * 0.32);
                sdf.line_to(w * 0.85, h * 0.32);
                sdf.move_to(w * 0.22, h * 0.58);
                sdf.line_to(w * 0.85, h * 0.58);
                sdf.move_to(w * 0.22, h * 0.82);
                sdf.line_to(w * 0.85, h * 0.82);
                sdf.stroke(self.color, 1.4);
                return sdf.result;
            }
        }

        // A note's anchor tag, folded while the caret is off its line: a pill
        // over the tag's own cells, with the note's title.
        draw_note_chip: {
            draw_depth: 1.2,
            color: #2d2a22,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 1.0, self.rect_size.x - 1.0, self.rect_size.y - 2.0, 3.0);
                sdf.fill_keep(self.color);
                sdf.stroke(#d7ba7d66, 1.0);
                return sdf.result;
            }
        }
        draw_note_chip_text: {
            draw_depth: 1.25,
            color: #d7ba7d,
            text_style: <THEME_FONT_CODE> {}
        }

        // A diagnostic's message after the end of its line (as VS Code's
        // Error Lens): its colour is set per message, by severity.
        draw_message: {
            text_style: <THEME_FONT_CODE> {}
        }

        // The band behind a line that carries a diagnostic, end to end: its
        // colour is set per line, by severity.
        draw_message_line: {
            color: #0000
        }

        // Full-width background band for diff review lines (added / removed).
        draw_review_bg: {
            color: #0000
        }

        // 3 px bar at the left edge of the text area for diff review lines.
        draw_review_bar: {
            color: #0000
        }

        // Strike line through removed diff review lines.
        draw_review_strike: {
            color: #e05a5ab3
        }

        // Diff review: Accept button background (green #2f6b3a, 4 px radius).
        draw_review_btn_accept: {
            draw_depth: 2.0,
            color: #2f6b3a,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                sdf.fill(self.color);
                return sdf.result;
            }
        }

        // Diff review: Accept button text (#e9fbe9).
        draw_review_btn_accept_text: {
            draw_depth: 2.1,
            color: #e9fbe9,
            text_style: <THEME_FONT_REGULAR> {}
        }

        // Diff review: Accept button icon (check mark ✓ in text color #e9fbe9).
        draw_review_btn_accept_icon: {
            draw_depth: 2.1,
            color: #e9fbe9,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let w = self.rect_size.x;
                let h = self.rect_size.y;
                sdf.move_to(w * 0.15, h * 0.50);
                sdf.line_to(w * 0.42, h * 0.78);
                sdf.line_to(w * 0.85, h * 0.20);
                sdf.stroke(self.color, 2.0);
                return sdf.result;
            }
        }

        // Diff review: Reject button background (#3a2626, border #5a3434, 4 px radius).
        draw_review_btn_reject: {
            draw_depth: 2.0,
            color: #3a2626,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 4.0);
                sdf.fill_keep(self.color);
                sdf.stroke(#5a3434, 1.0);
                return sdf.result;
            }
        }

        // Diff review: Reject button text (#ffd6d6).
        draw_review_btn_reject_text: {
            draw_depth: 2.1,
            color: #ffd6d6,
            text_style: <THEME_FONT_REGULAR> {}
        }

        // Diff review: Reject button icon (X mark ✕ in text color #ffd6d6).
        draw_review_btn_reject_icon: {
            draw_depth: 2.1,
            color: #ffd6d6,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let w = self.rect_size.x;
                let h = self.rect_size.y;
                sdf.move_to(w * 0.20, h * 0.20);
                sdf.line_to(w * 0.80, h * 0.80);
                sdf.move_to(w * 0.80, h * 0.20);
                sdf.line_to(w * 0.20, h * 0.80);
                sdf.stroke(self.color, 2.0);
                return sdf.result;
            }
        }

        // The occurrences of the name under the caret, behind the text: a
        // read in grey, a write in blue — VS Code's dark theme colours.
        draw_occurrence: {
            color: #575757b8
        }
        draw_occurrence_write: {
            color: #004972b8
        }

        // Ctrl held over a name: the link's underline.
        draw_link: {
            color: #4fc1ff
        }

        // The change bars beside the numbers: a line changed and saved
        // in this session, and one changed and not saved yet.
        draw_change_saved: {
            draw_depth: 1.1,
            color: #4caf50
        }
        draw_change_unsaved: {
            draw_depth: 1.1,
            color: #e5c07b
        }

        // A numbered bookmark: a red flag with its digit, at the left of the
        // glyph margin.
        draw_bookmark: {
            draw_depth: 1.15,
            color: #c42b1c,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 1.5);
                sdf.fill(self.color);
                return sdf.result;
            }
        }
        draw_bookmark_digit: {
            draw_depth: 1.16,
            color: #ffffff,
            text_style: <THEME_FONT_BOLD> { font_size: 7.5 }
        }

        // The dot the mouse would add, faint, as VS Code shows it.
        draw_breakpoint_hint: {
            draw_depth: 1.1,
            color: #E5140066,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let r = min(self.rect_size.x, self.rect_size.y) * 0.5;
                sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, r - 0.5);
                sdf.fill(self.color);
                return sdf.result;
            }
        }

        // The tip beside the mouse over the glyph margin.
        draw_tip_bg: {
            draw_depth: 3.0,
            color: #252526,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 4.0);
                sdf.fill_keep(self.color);
                sdf.stroke(#454545, 1.0);
                return sdf.result;
            }
        }
        draw_tip_text: {
            draw_depth: 3.1,
            color: #cccccc,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }

        // The popup "Add documentation / Add note / Add comment" under the caret line.
        draw_note_popup_bg: {
            draw_depth: 4.0,
            color: #252A32,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 8.0);
                sdf.fill_keep(self.color);
                sdf.stroke(#3A404B, 1.0);
                return sdf.result;
            }
        }
        draw_note_popup_row_bg: {
            draw_depth: 4.05,
            color: #313844,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 6.0);
                sdf.fill(self.color);
                return sdf.result;
            }
        }
        draw_note_popup_header: {
            draw_depth: 4.1,
            color: #9BA1AB,
            text_style: <THEME_FONT_REGULAR> { font_size: 8.5 }
        }
        draw_note_popup_title: {
            draw_depth: 4.1,
            color: #F2F3F5,
            text_style: <THEME_FONT_BOLD> { font_size: 10.5 }
        }
        draw_note_popup_desc: {
            draw_depth: 4.1,
            color: #A9AFB8,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
        draw_note_popup_shortcut: {
            draw_depth: 4.1,
            color: #9BA1AB,
            text_style: <THEME_FONT_CODE> { font_size: 9.0 }
        }

        // The breakpoint dot in the glyph margin.
        draw_breakpoint: {
            draw_depth: 1.1,
            color: #E51400,
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let r = min(self.rect_size.x, self.rect_size.y) * 0.5;
                sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, r - 0.5);
                sdf.fill(self.color);
                return sdf.result;
            }
        }

        animator: {
            blink = {
                default: off
                off = {
                    from: {all: Forward {duration:0.05}}
                    apply: {
                        draw_cursor: {blink:0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.05}}
                    apply: {
                        draw_cursor: {blink:1.0}
                    }
                }
            }
            focus = {
                default: off
                off = {
                    from: {all: Forward {duration:0.05}}
                    apply: {
                        draw_cursor: {focus:0.0}
                        draw_cursor_bg: {focus:0.0}
                        draw_selection: {focus:0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.05}}
                    apply: {
                        draw_cursor: {focus:1.0}
                        draw_cursor_bg: {focus:1.0}
                        draw_selection: {focus:1.0}
                    }
                }
            }
        }
    }

}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
struct DrawCodeText {
    #[deref]
    draw_super: DrawText,
    #[live]
    outline: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LandingIndicator {
    pub line_index: usize,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Live, LiveRegister)]
pub struct CodeEditor {
    #[walk]
    walk: Walk,
    #[live]
    scroll_bars: ScrollBars,
    #[live]
    draw_gutter: DrawText,
    #[live]
    draw_gutter_current_line: DrawText,
    #[live]
    draw_current_line: DrawColor,
    #[live]
    draw_text: DrawCodeText,
    #[live]
    token_colors: TokenColors,
    #[live]
    diagnostic_colors: DiagnosticColors,
    #[live]
    draw_indent_guide: DrawColor,
    #[live]
    draw_scope_header: DrawColor,
    #[live]
    draw_decoration: DrawDecoration,
    #[live]
    draw_selection: DrawSelection,
    #[live]
    draw_cursor: DrawColor,
    #[live]
    draw_cursor_bg: DrawColor,
    #[live]
    draw_bg: DrawColor,
    #[live]
    draw_landing_line: DrawColor,
    #[live]
    draw_breakpoint: DrawColor,
    #[live]
    draw_breakpoint_hint: DrawColor,
    #[live]
    draw_change_saved: DrawColor,
    #[live]
    draw_change_unsaved: DrawColor,
    #[live]
    draw_bookmark: DrawColor,
    #[live]
    draw_bookmark_digit: DrawText,
    #[live]
    draw_occurrence: DrawColor,
    #[live]
    draw_message: DrawText,
    #[live]
    draw_message_line: DrawColor,
    #[live]
    draw_review_bg: DrawColor,
    #[live]
    draw_review_bar: DrawColor,
    #[live]
    draw_review_strike: DrawColor,
    #[live]
    draw_review_btn_accept: DrawColor,
    #[live]
    draw_review_btn_accept_icon: DrawColor,
    #[live]
    draw_review_btn_accept_text: DrawText,
    #[live]
    draw_review_btn_reject: DrawColor,
    #[live]
    draw_review_btn_reject_icon: DrawColor,
    #[live]
    draw_review_btn_reject_text: DrawText,
    #[live]
    draw_note: DrawColor,
    #[live]
    draw_note_doc: DrawColor,
    #[live]
    draw_note_comment: DrawColor,
    #[live]
    draw_note_alert: DrawColor,
    #[live]
    draw_note_chip: DrawColor,
    #[live]
    draw_note_chip_text: DrawText,
    #[live]
    draw_line_mark: DrawColor,
    /// The marks extensions painted on this file, where each icon was
    /// drawn, and the one whose icon was clicked.
    #[rust]
    line_marks: Vec<crate::line_mark::LineMark>,
    /// Where the text pass drew deprecated names, struck through by
    /// the decoration pass over the glyphs.
    #[rust]
    strikes: Vec<Rect>,
    #[rust]
    line_mark_rects: Vec<(String, String, Rect)>,
    /// Where each code lens row was last drawn: (the line below it, rect).
    #[rust]
    lens_rects: Vec<(usize, Rect)>,
    #[rust]
    line_mark_clicked: Option<(String, String)>,
    /// The notes' previews the IDE handed over, by note id.
    #[rust]
    note_previews: NotePreviews,
    /// Note anchors in alert state (out of rustdoc rules or changed signature).
    #[rust]
    note_alerts: NoteAlerts,
    /// Note anchors of resolved comments (hidden from the gutter).
    #[rust]
    note_hidden: NoteHidden,
    /// Anchors drawn without a preview yet — the IDE fetches them: (id, 0-based line).
    #[rust]
    notes_missing: Vec<(String, usize)>,
    /// Where each note anchor's gutter icon was drawn: (note_id, gutter_line, rect).
    #[rust]
    note_icon_rects: Vec<(String, usize, Rect)>,
    /// The note whose anchor icon was clicked, waiting for the IDE to open it.
    #[rust]
    note_clicked: Option<String>,
    #[rust]
    ctrl_clicked: bool,
    #[live]
    draw_occurrence_write: DrawColor,
    #[live]
    draw_link: DrawColor,
    #[live]
    draw_tip_bg: DrawColor,
    #[live]
    draw_tip_text: DrawText,
    #[live]
    draw_note_popup_bg: DrawColor,
    #[live]
    draw_note_popup_row_bg: DrawColor,
    #[live]
    draw_note_popup_header: DrawText,
    #[live]
    draw_note_popup_title: DrawText,
    #[live]
    draw_note_popup_desc: DrawText,
    #[live]
    draw_note_popup_shortcut: DrawText,
    #[rust]
    pub note_popup: NotePopupState,
    #[rust]
    pub note_popup_choice: Option<usize>,
    #[rust]
    note_popup_ate_key: bool,
    /// The glyph-margin line under the mouse, and where the mouse is.
    #[rust]
    gutter_hover: Option<(usize, DVec2)>,
    /// Shows the tip once the mouse rested on the margin.
    #[rust]
    tip_timer: Timer,
    /// The name under the resting mouse: its span on a line, and whether
    /// Ctrl is held (the underline and the hand).
    #[rust]
    word_hover: Option<WordHover>,
    #[rust]
    hover_timer: Timer,
    /// The mouse rested on a name: where, once (the IDE asks for its hover).
    #[rust]
    hover_rest: Option<HoverRest>,
    /// The mouse went to another place in the text, once (the hover closes).
    #[rust]
    hover_moved: bool,
    #[rust]
    tip_shown: bool,
    #[live]
    draw_execution_point: DrawColor,
    #[live]
    draw_execution_line: DrawColor,
    #[rust]
    pub landing_indicator: Option<LandingIndicator>,
    #[rust(KeepCursorInView::Off)]
    pub keep_cursor_in_view: KeepCursorInView,
    #[rust]
    last_cursor_screen_pos: Option<DVec2>,
    #[live]
    pad_left_top: DVec2,
    #[rust]
    cell_size: DVec2,
    #[rust]
    cell_offset_y: f64,
    #[rust]
    gutter_rect: Rect,
    #[rust]
    gutter_chars: usize,
    #[rust]
    viewport_rect: Rect,
    #[rust]
    unscrolled_rect: Rect,
    #[rust]
    line_start: usize,
    #[rust]
    line_end: usize,
    #[rust(1.0)]
    pub height_scale: f64,
    #[live(true)]
    word_wrap: bool,
    #[live(false)]
    read_only: bool,
    #[live(false)]
    pub review_buttons: bool,
    #[rust]
    review_button_rects: Vec<(usize, bool, Rect)>,
    #[live(true)]
    show_gutter: bool,
    #[live(true)]
    pub highlight_current_line: bool,
    #[live(28.0)]
    pub glyph_margin: f64,
    #[live(2usize)]
    gutter_pad: usize,
    #[live(true)]
    empty_page_at_end: bool,

    #[live(0.5)]
    blink_speed: f64,

    #[animator]
    animator: Animator,

    #[rust]
    blink_timer: Timer,
}

/// The notes' previews by id. An alias: the `Live` derive takes no path type.
type NotePreviews = std::collections::HashMap<String, String>;

/// Anchor ids currently in alert. An alias: the `Live` derive takes no path type.
type NoteAlerts = std::collections::HashSet<String>;

/// Anchor ids of resolved comments that draw no gutter icon. An alias: the `Live` derive takes no path type.
type NoteHidden = std::collections::HashSet<String>;

const REVIEW_ADDED_BG: Vec4 = vec4(60.0 / 255.0, 160.0 / 255.0, 80.0 / 255.0, 48.0 / 255.0);
const REVIEW_REMOVED_BG: Vec4 = vec4(200.0 / 255.0, 70.0 / 255.0, 70.0 / 255.0, 48.0 / 255.0);
const REVIEW_ADDED_BAR: Vec4 = vec4(76.0 / 255.0, 191.0 / 255.0, 99.0 / 255.0, 1.0);
const REVIEW_REMOVED_BAR: Vec4 = vec4(224.0 / 255.0, 90.0 / 255.0, 90.0 / 255.0, 1.0);
const REVIEW_STRIKE: Vec4 = vec4(224.0 / 255.0, 90.0 / 255.0, 90.0 / 255.0, 0.7);

/// The text of a (zero-based) line, if the document has it.
fn line_text(session: &CodeSession, line: usize) -> Option<String> {
    session.document().as_text().as_lines().get(line).cloned()
}

/// A diagnostic's colours per severity: the message and line number (text)
/// and the band behind the line, end to end (band).
#[derive(Live, LiveHook, LiveRegister)]
pub struct DiagnosticColors {
    #[live]
    pub error: Vec4,
    #[live]
    pub warning: Vec4,
    #[live]
    pub info: Vec4,
    #[live]
    pub error_band: Vec4,
    #[live]
    pub warning_band: Vec4,
    #[live]
    pub info_band: Vec4,
}

impl DiagnosticColors {
    /// A severity's `(text, band)`: 0 an error, 1 a warning, anything else
    /// the rest.
    pub fn of(&self, severity: u8) -> (Vec4, Vec4) {
        match severity {
            0 => (self.error, self.error_band),
            1 => (self.warning, self.warning_band),
            _ => (self.info, self.info_band),
        }
    }
}

/// How many characters of a message are written after its line.
const MESSAGE_CHARS: usize = 100;

/// A message as it fits after a line: its lines joined with `: `, cut with
/// `…`. The compiler's footer (`` `#[warn(unused)]` on by default ``) is
/// left out; what an attribute's author wrote is kept: rustc puts the
/// `#[must_use = "…"]` text on the second line.
pub(crate) fn message_line(message: &str) -> String {
    let lines = message.lines().map(str::trim);
    let kept: Vec<&str> = lines
        .filter(|l| !l.is_empty() && !l.ends_with("on by default"))
        .collect();
    let joined = kept.join(": ");
    if joined.chars().count() <= MESSAGE_CHARS {
        return joined;
    }
    let cut: String = joined.chars().take(MESSAGE_CHARS - 1).collect();
    format!("{}…", cut.trim_end())
}

/// How long the mouse rests on the glyph margin before the tip shows.
const TIP_DELAY: f64 = 0.6;
/// How long the mouse rests on a name before its hover is asked.
const HOVER_DELAY: f64 = 0.5;

/// A name under the mouse: line, byte span, and Ctrl held.
#[derive(Clone, Copy, Debug, PartialEq)]
struct WordHover {
    line: usize,
    start: usize,
    end: usize,
    ctrl: bool,
}

/// Where the mouse rested: the text position under it, and the screen box
/// of the name (top-left and the line's height) to place a card beside.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HoverRest {
    pub line: usize,
    pub byte: usize,
    pub at: DVec2,
    pub line_height: f64,
}

/// The identifier span around `byte` of `line`, if `byte` is on one.
fn word_span(line: &str, byte: usize) -> Option<(usize, usize)> {
    let is_name = |c: char| c.is_alphanumeric() || c == '_';
    let byte = byte.min(line.len());
    let start = line[..byte]
        .char_indices()
        .rev()
        .take_while(|(_, c)| is_name(*c))
        .last()
        .map_or(byte, |(i, _)| i);
    let end = line[byte..]
        .find(|c| !is_name(c))
        .map_or(line.len(), |i| byte + i);
    (end > start).then_some((start, end))
}
const TIP_TEXT: &str = "Click to add a breakpoint";

/// The colour of text drawn after a line's end (an inlay): grey, so it never
/// reads as code.
const INLAY_COLOR: Vec4 = Vec4 {
    x: 0.55,
    y: 0.56,
    z: 0.6,
    w: 1.0,
};

pub enum KeepCursorInView {
    Once,
    Always(DVec2, NextFrame),
    LockStart,
    Locked(DVec2),
    LockedCenter(DVec2, Position, Affinity),
    FontResize(DVec2),
    JumpToPosition,
    /// Brings the cursor into view with room around it and keeps doing so
    /// until its place stops moving: a file just opened wraps its long lines
    /// only after the first frame, which pushed the line down out of view
    /// (measured on 30/09/2026: a debugger stop at line 683 left 629–667 on
    /// screen). See [`CodeEditor::reveal_cursor`].
    Reveal(Reveal),
    Off,
}

/// The state of a [`KeepCursorInView::Reveal`].
pub struct Reveal {
    last_y: Option<f64>,
    frames: u8,
    next: NextFrame,
}

impl Reveal {
    /// How many frames a reveal may take before it gives up.
    const MAX_FRAMES: u8 = 10;

    /// Whether the reveal is done now that the cursor stands at `y`: it stood
    /// at the same place a frame ago, or it ran out of frames.
    fn settled(&mut self, y: f64) -> bool {
        let done = self.last_y == Some(y) || self.frames >= Self::MAX_FRAMES;
        self.last_y = Some(y);
        self.frames += 1;
        done
    }
}

impl KeepCursorInView {
    fn is_once(&self) -> bool {
        matches!(self, Self::Once)
    }
    fn is_locked(&self) -> bool {
        matches!(
            self,
            Self::LockStart | Self::Locked(_) | Self::LockedCenter(_, _, _)
        )
    }
}
impl LiveHook for CodeEditor {}
/*
impl LiveHook for CodeEditor {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, CodeEditor)
    }
}

impl Widget for CodeEditor {
    fn redraw(&mut self, cx: &mut Cx) {
        self.scroll_bars.redraw(cx);
    }

    fn handle_event(
        &mut self,
        _cx: &mut Cx,
        _event: &Event,
        _scope:&mut WidgetScope,
        _dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionWrap),
    )->WidgetAction{

    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        self.walk
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, _scope:&mut WidgetScope, walk: Walk) -> WidgetDraw {
        if self.draw_state.begin(cx, walk) {
            return WidgetDraw::hook_above();
        }
        self.draw_state.end();
        WidgetDraw::done()
    }
}

#[derive(Clone, PartialEq, WidgetRef)]
pub struct CodeEditorRef(WidgetRef);
*/

pub use crate::scope_guides::{ScopeBlock, compute_scope_blocks};

impl CodeEditor {
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.scroll_bars.redraw(cx);
    }

    pub fn set_visible(&mut self, _cx: &mut Cx, _visible: bool) {}

    pub fn visible(&self) -> bool {
        true
    }

    pub fn area(&self) -> Area {
        self.scroll_bars.area()
    }

    pub fn last_cursor_screen_pos(&self) -> Option<DVec2> {
        self.last_cursor_screen_pos
    }

    /// Brings the cursor into view and keeps it there while the file settles
    /// (see [`KeepCursorInView::Reveal`]).
    pub fn reveal_cursor(&mut self, cx: &mut Cx) {
        self.keep_cursor_in_view = KeepCursorInView::Reveal(Reveal {
            last_y: None,
            frames: 0,
            next: cx.new_next_frame(),
        });
        self.redraw(cx);
    }

    pub fn viewport_rect(&self) -> Rect {
        self.viewport_rect
    }

    pub fn walk(&self, _cx: &mut Cx) -> Walk {
        self.walk
    }

    pub fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    pub fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {}

    pub fn draw_empty_editor(&mut self, cx: &mut Cx2d, walk: Walk) {
        self.scroll_bars.begin(cx, walk, Layout::default());
        self.draw_bg.draw_abs(cx, cx.turtle().unscrolled_rect());
        self.scroll_bars.end(cx);
    }

    pub fn draw_walk_editor(&mut self, cx: &mut Cx2d, session: &mut CodeSession, walk: Walk) {
        // This needs to be called first to ensure the session is up to date.
        session.handle_changes();

        let doc = session.document().as_text();
        let doc_lines = doc.as_lines();
        let lines: Vec<&str> = doc_lines.iter().map(String::as_str).collect();
        let caret_lines: Vec<usize> = session
            .selections()
            .iter()
            .map(|s| s.cursor.position.line_index)
            .collect();
        let collapsed = crate::note_anchor::collapsed_lines(&lines, &caret_lines);
        session.set_collapsed_lines(&collapsed);

        let text = self
            .draw_text
            .layout(cx, 0.0, 0.0, None, false, Align::default(), "!");
        let first_row = text.rows.first().unwrap();
        let first_glyph = first_row.glyphs.first().unwrap();
        let width_in_lpxs = first_glyph.advance_in_lpxs();
        let height_in_lpxs = first_glyph.ascender_in_lpxs() - first_glyph.descender_in_lpxs();
        let line_spacing_in_lpxs = height_in_lpxs * self.draw_text.text_style.line_spacing;
        self.cell_size = dvec2(width_in_lpxs as f64, line_spacing_in_lpxs as f64);
        self.cell_offset_y = ((line_spacing_in_lpxs - height_in_lpxs) / 2.0) as f64;

        let last_added_selection = {
            let selections = session.selections();
            session
                .last_added_selection_index()
                .and_then(|idx| selections.get(idx).copied())
                .or_else(|| selections.first().copied())
                .unwrap_or_default()
        };
        let (cursor_x, cursor_y) = session.layout().logical_to_normalized_position(
            last_added_selection.cursor.position,
            last_added_selection.cursor.affinity,
        );
        let cursor_pos = dvec2(cursor_x, cursor_y) * self.cell_size;
        self.last_cursor_screen_pos = Some(cursor_pos - self.scroll_bars.get_scroll_pos());
        match self.keep_cursor_in_view {
            KeepCursorInView::Once | KeepCursorInView::Always(_, _) => {
                // make a cursor bounding box
                let pad_above = dvec2(self.cell_size.x * 8.0, self.cell_size.y);
                let pad_below = dvec2(self.cell_size.x * 8.0, self.cell_size.y * 2.0);
                let rect = Rect {
                    pos: cursor_pos - pad_above,
                    size: pad_above + pad_below,
                };
                // only scroll into
                self.scroll_bars.scroll_into_view(cx, rect);
                if self.keep_cursor_in_view.is_once() {
                    self.keep_cursor_in_view = KeepCursorInView::Off
                }
            }
            KeepCursorInView::LockStart => {
                // lets get the on screen position
                let screen_pos = cursor_pos - self.scroll_bars.get_scroll_pos();
                let rect = Rect {
                    pos: dvec2(0.0, 0.0),
                    size: self.viewport_rect.size,
                };
                if rect.contains(screen_pos) {
                    self.keep_cursor_in_view = KeepCursorInView::Locked(screen_pos);
                } else {
                    let center = rect.size * 0.5 + self.unscrolled_rect.pos;
                    let ((pos, aff), _) = self.pick(session, center);
                    let (cursor_x, cursor_y) =
                        session.layout().logical_to_normalized_position(pos, aff);
                    let screen_pos = dvec2(cursor_x, cursor_y) * self.cell_size
                        - self.scroll_bars.get_scroll_pos();
                    self.keep_cursor_in_view = KeepCursorInView::LockedCenter(screen_pos, pos, aff);
                }
            }
            KeepCursorInView::Locked(pos) => {
                // ok so we want to keep cursor pos at the same screen position
                let new_pos = cursor_pos - self.scroll_bars.get_scroll_pos();
                let delta = pos - new_pos;
                let new_pos = self.scroll_bars.get_scroll_pos() - dvec2(0.0, delta.y);
                self.scroll_bars.set_scroll_pos_no_clip(cx, new_pos);
                //self.keep_cursor_in_view = KeepCursorInView::Locked(cursor_pos);
            }
            KeepCursorInView::LockedCenter(screen_pos, pos, aff) => {
                let (cursor_x, cursor_y) =
                    session.layout().logical_to_normalized_position(pos, aff);
                let new_pos =
                    dvec2(cursor_x, cursor_y) * self.cell_size - self.scroll_bars.get_scroll_pos();
                let delta = screen_pos - new_pos;
                let new_pos = self.scroll_bars.get_scroll_pos() - dvec2(0.0, delta.y);
                self.scroll_bars.set_scroll_pos_no_clip(cx, new_pos);
                //self.keep_cursor_in_view = KeepCursorInView::Locked(cursor_pos);
            }
            KeepCursorInView::Reveal(ref mut reveal) => {
                // A third of the view above and below: the line lands inside
                // the view, not on its edge.
                let room = dvec2(
                    self.cell_size.x * 8.0,
                    (self.viewport_rect.size.y / 3.0).max(self.cell_size.y * 2.0),
                );
                let settled = reveal.settled(cursor_pos.y);
                self.scroll_bars.scroll_into_view(
                    cx,
                    Rect {
                        pos: cursor_pos - room,
                        size: 2.0 * room,
                    },
                );
                if settled {
                    self.keep_cursor_in_view = KeepCursorInView::Off;
                }
            }
            KeepCursorInView::JumpToPosition => {
                // alright so we need to make sure that cursor_pos
                // is in view.
                let padd = dvec2(self.cell_size.x * 10.0, self.cell_size.y * 10.0);
                self.scroll_bars.scroll_into_view(
                    cx,
                    Rect {
                        pos: cursor_pos - padd,
                        size: 2.0 * padd,
                    },
                );
                self.keep_cursor_in_view = KeepCursorInView::Off;
            }
            KeepCursorInView::FontResize(last_pos) => {
                let new_pos = cursor_pos - self.scroll_bars.get_scroll_pos();
                let delta = last_pos - new_pos;
                let new_pos = self.scroll_bars.get_scroll_pos() - dvec2(0.0, delta.y);
                self.scroll_bars.set_scroll_pos_no_clip(cx, new_pos);
                self.keep_cursor_in_view = KeepCursorInView::Off
            }
            KeepCursorInView::Off => {}
        }

        // if height = Fit, we need to compute it based on layout
        let height_is_fit = walk.height.is_fit();
        //if height_is_fit{
        //    walk.height = Size::Fixed(100000.0);
        // }
        const MAX_HEIGHT: f64 = 100_0000.0;
        self.scroll_bars.begin(cx, walk, Layout::default());

        let turtle_rect = cx.turtle().rect();

        let (gutter_width, gutter_chars) = crate::gutter::calculate_gutter_width(
            session,
            self.glyph_margin,
            self.gutter_pad,
            self.cell_size.x,
            self.show_gutter,
        );
        self.gutter_chars = gutter_chars;
        self.gutter_rect = Rect {
            pos: turtle_rect.pos,
            size: DVec2 {
                x: gutter_width,
                y: if height_is_fit {
                    MAX_HEIGHT
                } else {
                    turtle_rect.size.y
                },
            },
        };
        self.viewport_rect = Rect {
            pos: DVec2 {
                x: turtle_rect.pos.x + gutter_width,
                y: turtle_rect.pos.y,
            },
            size: DVec2 {
                x: turtle_rect.size.x - gutter_width,
                y: if height_is_fit {
                    MAX_HEIGHT
                } else {
                    turtle_rect.size.y
                },
            },
        };

        //let pad_left_top = dvec2(10., 10.);
        self.gutter_rect.pos += self.pad_left_top;
        self.gutter_rect.size -= self.pad_left_top;
        self.viewport_rect.pos += self.pad_left_top;
        self.viewport_rect.size -= self.pad_left_top;

        session.set_wrap_column(if self.word_wrap {
            Some((self.viewport_rect.size.x / self.cell_size.x) as usize)
        } else {
            None
        });

        let scroll_pos = self.scroll_bars.get_scroll_pos();

        self.line_start = session
            .layout()
            .find_first_line_ending_after_y(scroll_pos.y / self.cell_size.y - self.cell_size.y);
        self.line_end = session.layout().find_first_line_starting_after_y(
            (scroll_pos.y + self.viewport_rect.size.y) / self.cell_size.y,
        );
        self.unscrolled_rect = cx.turtle().unscrolled_rect();

        let bg_rect = Rect {
            pos: self.unscrolled_rect.pos,
            size: DVec2 {
                x: self.unscrolled_rect.size.x,
                y: if height_is_fit {
                    MAX_HEIGHT
                } else {
                    self.unscrolled_rect.size.y
                },
            },
        };
        self.draw_bg.draw_abs(cx, bg_rect);

        if self.show_gutter {
            self.draw_gutter(cx, session);
        }
        self.draw_current_line_layer(cx, session);
        self.draw_review_layer(cx, session);
        self.draw_message_line_layer(cx, session);
        self.draw_scope_header_layer(cx, session);
        self.draw_indent_guide_layer(cx, session);
        self.draw_occurrence_layer(cx, session);
        self.draw_link_underline(cx, session);
        self.draw_text_layer(cx, session);
        self.draw_note_layer(cx, session);
        self.draw_line_mark_layer(cx, session);
        self.draw_message_layer(cx, session);
        self.draw_decoration_layer(cx, session);
        self.draw_cursor.begin_many_instances(cx);
        self.draw_selection_layer(cx, session);
        self.draw_cursor.end_many_instances(cx);
        self.draw_landing_layer(cx, session);
        self.draw_review_buttons(cx, session);
        self.draw_tip(cx, session);
        self.draw_note_popup(cx, session);

        // Get the last added selection.
        // Get the normalized cursor position. To go from normalized to screen position, multiply by
        // the cell size, then shift by the viewport origin.

        cx.turtle_mut().set_used(
            session.layout().width() * self.cell_size.x + self.pad_left_top.x,
            self.height_scale * session.layout().height() * self.cell_size.y
                + if height_is_fit || !self.empty_page_at_end {
                    0.0
                } else {
                    self.viewport_rect.size.y
                }
                + self.pad_left_top.y * self.height_scale,
        );

        self.scroll_bars.end(cx);

        if session.update_folds() {
            cx.redraw_area_in_draw(self.scroll_bars.area());
        } else if self.keep_cursor_in_view.is_locked() {
            self.keep_cursor_in_view = KeepCursorInView::Off;
        }
    }

    pub fn get_scroll_pos(&self) -> DVec2 {
        self.scroll_bars.get_scroll_pos()
    }

    pub fn set_scroll_pos(&mut self, cx: &mut Cx, pos: DVec2) -> bool {
        self.scroll_bars.set_scroll_pos(cx, pos)
    }

    pub fn get_scroll_view_total(&mut self) -> DVec2 {
        self.scroll_bars.get_scroll_view_total()
    }

    pub fn get_scroll_view_visible(&mut self) -> DVec2 {
        self.scroll_bars.get_scroll_view_visible()
    }

    pub fn cell_size(&self) -> DVec2 {
        self.cell_size
    }

    pub fn line_start(&self) -> usize {
        self.line_start
    }

    pub fn line_end(&self) -> usize {
        self.line_end
    }

    pub fn set_key_focus(&mut self, cx: &mut Cx) {
        cx.set_key_focus(self.scroll_bars.area());
    }

    pub fn set_cursor_and_scroll(&mut self, cx: &mut Cx, pos: Position, session: &mut CodeSession) {
        session.set_selection(
            session.clamp_position(pos),
            Affinity::Before,
            SelectionMode::Simple,
            NewGroup::Yes,
        );
        self.keep_cursor_in_view = KeepCursorInView::JumpToPosition;
        self.redraw(cx);
    }

    pub fn set_selection_and_scroll(
        &mut self,
        cx: &mut Cx,
        start: Position,
        end: Position,
        session: &mut CodeSession,
    ) {
        session.set_selection(
            session.clamp_position(start),
            Affinity::Before,
            SelectionMode::Simple,
            NewGroup::Yes,
        );
        session.move_to(session.clamp_position(end), Affinity::Before, NewGroup::Yes);
        self.keep_cursor_in_view = KeepCursorInView::JumpToPosition;
        self.redraw(cx);
    }

    pub fn reset_font_size(&mut self) {
        self.draw_gutter.text_style.font_size = 9.0;
        self.draw_gutter_current_line.text_style.font_size = 9.0;
        self.draw_text.text_style.font_size = 9.0;
        if let Some(pos) = self.last_cursor_screen_pos {
            self.keep_cursor_in_view = KeepCursorInView::FontResize(pos);
        }
    }

    pub fn decrease_font_size(&mut self) {
        if self.draw_text.text_style.font_size > 3.0 {
            self.draw_text.text_style.font_size -= 1.0;
            self.draw_gutter.text_style.font_size = self.draw_text.text_style.font_size;
            self.draw_gutter_current_line.text_style.font_size =
                self.draw_text.text_style.font_size;
            if let Some(pos) = self.last_cursor_screen_pos {
                self.keep_cursor_in_view = KeepCursorInView::FontResize(pos);
            }
        }
    }

    pub fn increase_font_size(&mut self) {
        if self.draw_text.text_style.font_size < 20.0 {
            self.draw_text.text_style.font_size += 1.0;
            self.draw_gutter.text_style.font_size = self.draw_text.text_style.font_size;
            self.draw_gutter_current_line.text_style.font_size =
                self.draw_text.text_style.font_size;
            if let Some(pos) = self.last_cursor_screen_pos {
                self.keep_cursor_in_view = KeepCursorInView::FontResize(pos);
            }
        }
    }

    pub fn reset_cursor_blinker(&mut self, cx: &mut Cx) {
        if self.read_only {
            self.animator_cut(cx, id!(blink.off));
        } else {
            self.animator_cut(cx, id!(blink.off));
            cx.stop_timer(self.blink_timer);
            self.blink_timer = cx.start_timeout(self.blink_speed)
        }
    }

    pub fn handle_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        session: &mut CodeSession,
    ) -> Vec<CodeEditorAction> {
        let mut actions = Vec::new();

        self.animator_handle_event(cx, event);

        session.handle_changes();

        if !self.scroll_bars.handle_event(cx, event, scope).is_empty() {
            self.redraw(cx);
        };

        if self.hover_timer.is_event(event).is_some()
            && let Some(w) = self.word_hover
        {
            self.hover_rest = self.rest_of(Some(&*session), w);
        }
        if self.tip_timer.is_event(event).is_some() && self.gutter_hover.is_some() {
            self.tip_shown = true;
            self.redraw(cx);
        }
        if self.blink_timer.is_event(event).is_some() {
            if self.animator_in_state(cx, id!(blink.off)) {
                self.animator_play(cx, id!(blink.on));
            } else {
                self.animator_play(cx, id!(blink.off));
            }
            self.blink_timer = cx.start_timeout(self.blink_speed)
        }
        let mut keyboard_moved_cursor = false;
        // A key the popup takes must not also arrive as typed text: its
        // TextInput comes after the KeyDown, when the popup may be closed.
        if let Event::KeyDown(_) = event {
            self.note_popup_ate_key = self.note_popup.open;
        }
        match event.hits(cx, self.scroll_bars.area()) {
            Hit::TextInput(_) if std::mem::take(&mut self.note_popup_ate_key) => {
                return actions;
            }
            Hit::KeyFocusLost(_) => {
                self.animator_play(cx, id!(focus.off));
            }
            Hit::KeyFocus(_) => {
                self.animator_play(cx, id!(focus.on));
            }
            Hit::KeyDown(KeyEvent {
                key_code,
                modifiers,
                ..
            }) if self.note_popup.open => {
                match key_code {
                    KeyCode::Escape => {
                        self.note_popup.close();
                        self.redraw(cx);
                    }
                    KeyCode::ArrowUp => {
                        self.note_popup.move_up();
                        self.redraw(cx);
                    }
                    KeyCode::ArrowDown => {
                        self.note_popup.move_down();
                        self.redraw(cx);
                    }
                    KeyCode::ReturnKey | KeyCode::NumpadEnter => {
                        if let Some(ch) = self.note_popup.choose_selected() {
                            self.note_popup_choice = Some(ch);
                        }
                        self.redraw(cx);
                    }
                    KeyCode::KeyD | KeyCode::KeyN | KeyCode::KeyM
                        if !modifiers.control && !modifiers.alt && !modifiers.logo =>
                    {
                        let letter = match key_code {
                            KeyCode::KeyD => 'd',
                            KeyCode::KeyN => 'n',
                            _ => 'm',
                        };
                        if let Some(ch) = self.note_popup.handle_letter(letter) {
                            self.note_popup_choice = Some(ch);
                        }
                        self.redraw(cx);
                    }
                    _ => {}
                }
                return actions;
            }
            Hit::KeyUp(_) if self.note_popup.open => {
                return actions;
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Escape,
                is_repeat: false,
                ..
            }) => {
                session.fold();
                if !self.keep_cursor_in_view.is_locked() {
                    self.keep_cursor_in_view = KeepCursorInView::LockStart;
                }
                self.redraw(cx);
            }
            Hit::KeyUp(KeyEvent {
                key_code: KeyCode::Escape,
                ..
            }) => {
                session.unfold();
                if !self.keep_cursor_in_view.is_locked() {
                    self.keep_cursor_in_view = KeepCursorInView::LockStart;
                }
                self.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Minus,
                modifiers: KeyModifiers { control, logo, .. },
                ..
            }) if (control || logo) => {
                self.decrease_font_size();
                self.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Key0,
                modifiers: KeyModifiers { control, logo, .. },
                ..
            }) if (control || logo) => {
                self.reset_font_size();
                self.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Equals,
                modifiers: KeyModifiers { control, logo, .. },
                ..
            }) if (control || logo) => {
                self.increase_font_size();
                self.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::KeyW,
                modifiers: KeyModifiers { control, logo, .. },
                ..
            }) if (control || logo) => {
                self.word_wrap = !self.word_wrap;
                self.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::KeyA,
                modifiers: KeyModifiers { control, logo, .. },
                ..
            }) if (control || logo) => {
                self.select_all(cx, session);
            }

            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ArrowLeft,
                modifiers:
                    KeyModifiers {
                        shift,
                        control,
                        logo,
                        ..
                    },
                ..
            }) => {
                if control || logo {
                    //session.move_to_start_of_line(!shift);
                } else {
                    session.move_left(!shift);
                }
                keyboard_moved_cursor = true;
                self.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ArrowRight,
                modifiers:
                    KeyModifiers {
                        shift,
                        control,
                        logo,
                        ..
                    },
                ..
            }) => {
                if control || logo {
                    //session.move_to_end_of_line(!shift);
                } else {
                    session.move_right(!shift);
                }

                keyboard_moved_cursor = true;
                self.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ArrowUp,
                modifiers: KeyModifiers { shift, .. },
                ..
            }) => {
                session.move_up(!shift);
                keyboard_moved_cursor = true;
                self.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ArrowDown,
                modifiers: KeyModifiers { shift, .. },
                ..
            }) => {
                session.move_down(!shift);
                keyboard_moved_cursor = true;
                self.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Home,
                modifiers: KeyModifiers { shift, .. },
                ..
            }) => {
                session.home(!shift);
                keyboard_moved_cursor = true;
                self.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::End,
                modifiers: KeyModifiers { shift, .. },
                ..
            }) => {
                session.end(!shift);
                keyboard_moved_cursor = true;
                self.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::PageUp,
                modifiers: KeyModifiers { shift, .. },
                ..
            }) => {
                for _ in 0..self.line_end - self.line_start - 3 {
                    session.move_up(!shift);
                }
                keyboard_moved_cursor = true;
                self.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::PageDown,
                modifiers: KeyModifiers { shift, .. },
                ..
            }) => {
                for _ in 0..self.line_end - self.line_start - 3 {
                    session.move_down(!shift);
                }
                keyboard_moved_cursor = true;
                self.redraw(cx);
            }
            Hit::TextInput(TextInputEvent {
                ref input,
                was_paste: false,
                ..
            }) if !input.is_empty() && !self.read_only => {
                session.insert(input.into());
                self.redraw(cx);
                keyboard_moved_cursor = true;
                actions.push(CodeEditorAction::TextDidChange);
            }
            Hit::TextInput(TextInputEvent {
                ref input,
                was_paste: true,
                ..
            }) if !input.is_empty() && !self.read_only => {
                session.paste(input.into());
                self.redraw(cx);
                keyboard_moved_cursor = true;
                actions.push(CodeEditorAction::TextDidChange);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ReturnKey,
                ..
            }) => {
                session.enter();
                self.redraw(cx);
                keyboard_moved_cursor = true;
                actions.push(CodeEditorAction::TextDidChange);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Tab,
                modifiers: KeyModifiers { shift: false, .. },
                ..
            }) if !self.read_only => {
                session.indent();
                self.redraw(cx);
                keyboard_moved_cursor = true;
                actions.push(CodeEditorAction::TextDidChange);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Tab,
                modifiers: KeyModifiers { shift: true, .. },
                ..
            }) if !self.read_only => {
                session.outdent();
                self.redraw(cx);
                keyboard_moved_cursor = true;
                actions.push(CodeEditorAction::TextDidChange);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Delete,
                ..
            }) if !self.read_only => {
                session.delete();
                self.redraw(cx);
                keyboard_moved_cursor = true;
                actions.push(CodeEditorAction::TextDidChange);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Backspace,
                ..
            }) if !self.read_only => {
                session.backspace();
                self.redraw(cx);
                keyboard_moved_cursor = true;
                actions.push(CodeEditorAction::TextDidChange);
            }
            Hit::TextCopy(ce) => {
                *ce.response.borrow_mut() = Some(session.copy());
                keyboard_moved_cursor = true;
            }
            Hit::TextCut(ce) if !self.read_only => {
                *ce.response.borrow_mut() = Some(session.cut());
                keyboard_moved_cursor = true;
                self.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::KeyZ,
                modifiers:
                    KeyModifiers {
                        control,
                        logo,
                        shift,
                        ..
                    },
                ..
            }) if (control || logo) && !shift && !self.read_only && session.undo() => {
                cx.redraw_all();
                actions.push(CodeEditorAction::TextDidChange);
                keyboard_moved_cursor = true;
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::KeyZ,
                modifiers:
                    KeyModifiers {
                        control,
                        logo,
                        shift,
                        ..
                    },
                ..
            }) if (control || logo) && shift && !self.read_only && session.redo() => {
                self.redraw(cx);
                actions.push(CodeEditorAction::TextDidChange);
                keyboard_moved_cursor = true;
            }
            Hit::KeyDown(ke) => {
                self.landing_indicator = None;
                actions.push(CodeEditorAction::UnhandledKeyDown(ke))
            }
            Hit::FingerDown(fe)
                if self.review_buttons
                    && session.document().reviewing()
                    && self.hit_review_button(fe.abs).is_some() =>
            {
                if let Some((change, accept)) = self.hit_review_button(fe.abs) {
                    actions.push(CodeEditorAction::ReviewChoice { change, accept });
                }
                return actions;
            }
            Hit::FingerDown(FingerDownEvent { abs, .. })
                if self.cell_size.y > 0.0
                    && session
                        .lens_at_y((abs.y - self.viewport_rect.pos.y) / self.cell_size.y)
                        .is_some() =>
            {
                self.landing_indicator = None;
                cx.set_key_focus(self.scroll_bars.area());
                let norm_y = (abs.y - self.viewport_rect.pos.y) / self.cell_size.y;
                if let Some(line) = session.lens_at_y(norm_y)
                    && abs.x >= self.viewport_rect.pos.x
                {
                    actions.push(CodeEditorAction::LensClicked(line));
                }
            }
            Hit::FingerDown(fe) if self.note_popup.open => {
                match self.note_popup.handle_mouse_down(fe.abs) {
                    crate::note_popup::PopupMouseResult::Chosen(ch) => {
                        self.note_popup_choice = Some(ch);
                        self.redraw(cx);
                    }
                    crate::note_popup::PopupMouseResult::Closed => {
                        self.redraw(cx);
                    }
                    crate::note_popup::PopupMouseResult::Consumed => {}
                    crate::note_popup::PopupMouseResult::Ignored => {}
                }
                return actions;
            }
            // THE RIGHT BUTTON: the caret moves to the point unless it
            // falls inside the selection, which stays for the menu's Cut and
            // Copy; no drag starts. The host opens its context menu.
            Hit::FingerDown(fe) if fe.mouse_button().is_some_and(|b| b.is_secondary()) => {
                let ((position, affinity), _) = self.pick(session, fe.abs);
                let inside = session
                    .selections()
                    .iter()
                    .any(|s| !s.is_empty() && s.start() <= position && position <= s.end());
                if !inside {
                    session.set_selection(position, affinity, SelectionMode::Simple, NewGroup::Yes);
                    self.redraw(cx);
                }
                cx.set_key_focus(self.scroll_bars.area());
            }
            Hit::FingerDown(FingerDownEvent {
                abs,
                tap_count,
                modifiers:
                    KeyModifiers {
                        alt: false,
                        shift: false,
                        control,
                        logo,
                        ..
                    },
                ..
            }) => {
                self.landing_indicator = None;
                // Ctrl+Click: the caret goes to the name as on any click,
                // and the IDE is told to go to its definition.
                self.ctrl_clicked = (control || logo) && tap_count == 1;
                self.animator_play(cx, id!(focus.on));
                cx.set_key_focus(self.scroll_bars.area());
                let ((cursor, affinity), is_in_gutter) = self.pick(session, abs);
                // A click on the glyph margin toggles the line's breakpoint and
                // leaves the selection alone, as in classic RAD IDEs and VS Code.
                if is_in_gutter && self.is_in_glyph_margin(abs) {
                    if let Some((id, _, _)) = self
                        .note_icon_rects
                        .iter()
                        .find(|(_, line, rect)| *line == cursor.line_index || rect.contains(abs))
                    {
                        self.note_clicked = Some(id.clone());
                        return actions;
                    }
                    if let Some(id) = line_text(session, cursor.line_index)
                        .as_deref()
                        .and_then(crate::note_anchor::id_in)
                    {
                        self.note_clicked = Some(id.to_string());
                        return actions;
                    }
                    let line = cursor.line_index;
                    if let Some(m) = crate::line_mark::on_line(&self.line_marks, line).next() {
                        self.line_mark_clicked = Some((m.owner.clone(), m.id.clone()));
                        return actions;
                    }
                    session.document().toggle_gutter_mark(
                        crate::gutter_mark::GutterMarkKind::Breakpoint,
                        cursor.line_index,
                    );
                    self.redraw(cx);
                    return actions;
                }
                session.set_selection(
                    cursor,
                    affinity,
                    if is_in_gutter {
                        SelectionMode::Line
                    } else {
                        match tap_count {
                            1 => SelectionMode::Simple,
                            2 => SelectionMode::Word,
                            _ => SelectionMode::Line,
                            //_ => SelectionMode::All,
                        }
                    },
                    NewGroup::Yes,
                );
                self.reset_cursor_blinker(cx);
                self.keep_cursor_in_view = KeepCursorInView::Always(abs, cx.new_next_frame());
                self.redraw(cx);
            }
            Hit::FingerDown(FingerDownEvent {
                abs,
                tap_count,
                modifiers:
                    KeyModifiers {
                        alt: true,
                        shift: false,
                        ..
                    },
                ..
            }) => {
                self.animator_play(cx, id!(focus.on));
                cx.set_key_focus(self.scroll_bars.area());
                let ((cursor, affinity), is_in_gutter) = self.pick(session, abs);
                session.add_selection(
                    cursor,
                    affinity,
                    if is_in_gutter {
                        SelectionMode::Line
                    } else {
                        match tap_count {
                            1 => SelectionMode::Simple,
                            2 => SelectionMode::Word,
                            3 => SelectionMode::Line,
                            _ => SelectionMode::All,
                        }
                    },
                );
                self.reset_cursor_blinker(cx);
                self.keep_cursor_in_view = KeepCursorInView::Always(abs, cx.new_next_frame());
                self.redraw(cx);
            }
            Hit::FingerUp(_) => {
                if self.note_popup.open {
                    return actions;
                }
                self.reset_cursor_blinker(cx);
                self.keep_cursor_in_view = KeepCursorInView::Off;
            }
            Hit::FingerHoverIn(FingerHoverEvent { abs, modifiers, .. })
            | Hit::FingerHoverOver(FingerHoverEvent { abs, modifiers, .. }) => {
                if self.note_popup.open {
                    self.note_popup.handle_mouse_move(abs);
                    self.redraw(cx);
                    return actions;
                }
                self.hover_gutter(cx, session, abs);
                self.hover_word(cx, session, abs, modifiers.control || modifiers.logo);
            }
            Hit::FingerHoverOut(_) => {
                self.leave_gutter(cx);
                // Leaving for the hover card must not close it: no `moved`. The
                // word is kept, so coming back anywhere else does close it.
                if let Some(w) = self.word_hover.as_mut().filter(|w| w.ctrl) {
                    w.ctrl = false;
                    self.redraw(cx);
                }
                cx.stop_timer(self.hover_timer);
            }
            Hit::FingerDown(FingerDownEvent {
                abs,
                modifiers: KeyModifiers { shift: true, .. },
                ..
            })
            | Hit::FingerMove(FingerMoveEvent { abs, .. }) => {
                if self.note_popup.open {
                    return actions;
                }
                self.reset_cursor_blinker(cx);
                if let KeepCursorInView::Always(old_abs, _) = &mut self.keep_cursor_in_view {
                    *old_abs = abs;
                }
                cx.set_cursor(MouseCursor::Text);
                let ((cursor, affinity), _) = self.pick(session, abs);
                session.move_to(cursor, affinity, NewGroup::Yes);
                // alright how are we going to do scrolling
                self.redraw(cx);
            }
            _ => {}
        }
        if keyboard_moved_cursor {
            self.keep_cursor_in_view = KeepCursorInView::Once;
            self.reset_cursor_blinker(cx);
        }
        if let KeepCursorInView::Reveal(reveal) = &mut self.keep_cursor_in_view
            && reveal.next.is_event(event).is_some()
        {
            reveal.next = cx.new_next_frame();
            self.redraw(cx);
        }
        if let KeepCursorInView::Always(abs, next) = &mut self.keep_cursor_in_view
            && next.is_event(event).is_some()
        {
            *next = cx.new_next_frame();
            let abs = *abs;
            let ((cursor, affinity), _) = self.pick(session, abs);
            session.move_to(cursor, affinity, NewGroup::Yes);
            self.redraw(cx);
        }
        actions
    }

    fn draw_gutter(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        let highlighted = crate::current_line::highlighted_lines_from_selections(
            &session.selections(),
            self.highlight_current_line,
        );
        self.draw_gutter_current_line.text_style.font_size = self.draw_gutter.text_style.font_size;
        let message_numbers: Vec<(usize, Vec4)> = session
            .document()
            .line_messages()
            .iter()
            .map(|m| (m.line, self.diagnostic_colors.of(m.severity).0))
            .collect();
        let review_rows = session.document().review_rows();
        let review_numbers: Vec<Option<usize>> = if !review_rows.is_empty() {
            review_rows.iter().map(|r| r.number).collect()
        } else {
            Vec::new()
        };
        let review_slice = (!review_numbers.is_empty()).then_some(review_numbers.as_slice());
        let lines = crate::gutter::GutterLines {
            line_start: self.line_start,
            line_end: self.line_end,
            gutter_rect: self.gutter_rect,
            glyph_margin: self.glyph_margin,
            gutter_chars: self.gutter_chars,
            cell_size: self.cell_size,
            cell_offset_y: self.cell_offset_y,
            highlighted_lines: &highlighted,
            message_numbers: &message_numbers,
            review_numbers: review_slice,
        };
        crate::gutter::draw_gutter_layer(
            cx,
            session,
            lines,
            &mut self.draw_gutter,
            &mut self.draw_gutter_current_line,
        );
        crate::gutter::draw_gutter_marks(
            cx,
            session,
            self.line_start,
            self.line_end,
            self.gutter_rect,
            self.glyph_margin,
            self.cell_size,
            &mut self.draw_breakpoint,
            self.gutter_hover
                .filter(|(line, _)| {
                    !self.note_icon_rects.iter().any(|(_, l, _)| *l == *line)
                        && line_text(session, *line)
                            .as_deref()
                            .and_then(crate::note_anchor::id_in)
                            .is_none()
                })
                .map(|(line, _)| (line, &mut self.draw_breakpoint_hint)),
        );
        crate::gutter::draw_change_bars(
            cx,
            session,
            self.line_start,
            self.line_end,
            self.gutter_rect,
            self.cell_size,
            (&mut self.draw_change_saved, &mut self.draw_change_unsaved),
        );
        crate::gutter::draw_bookmarks(
            cx,
            session,
            self.line_start,
            self.line_end,
            self.gutter_rect,
            self.cell_size,
            (&mut self.draw_bookmark, &mut self.draw_bookmark_digit),
        );
        crate::gutter::draw_execution_point(
            cx,
            session,
            self.line_start,
            self.line_end,
            self.gutter_rect,
            self.glyph_margin,
            self.cell_size,
            self.viewport_rect.size.x,
            &mut self.draw_execution_point,
            &mut self.draw_execution_line,
        );
    }

    /// The mouse over the editor: an arrow on the gutter (it is not text),
    /// and on the glyph margin the faint dot of the line, then the tip.
    fn hover_gutter(&mut self, cx: &mut Cx, session: &CodeSession, abs: DVec2) {
        let on_gutter = self.show_gutter && abs.x < self.viewport_rect.pos.x;
        cx.set_cursor(if on_gutter {
            MouseCursor::Default
        } else {
            MouseCursor::Text
        });
        if !self.is_in_glyph_margin(abs) {
            return self.leave_gutter(cx);
        }
        let ((cursor, _), _) = self.pick(session, abs);
        let line = cursor.line_index;
        if self.gutter_hover.map(|(l, _)| l) != Some(line) {
            self.tip_shown = false;
            cx.stop_timer(self.tip_timer);
            self.tip_timer = cx.start_timeout(TIP_DELAY);
            self.redraw(cx);
        }
        self.gutter_hover = Some((line, abs));
    }

    fn leave_gutter(&mut self, cx: &mut Cx) {
        if self.gutter_hover.take().is_some() {
            self.tip_shown = false;
            cx.stop_timer(self.tip_timer);
            self.redraw(cx);
        }
    }

    /// "Click to add a breakpoint" beside the mouse, when the line has none.
    fn draw_tip(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        let Some((line, abs)) = self.gutter_hover.filter(|_| self.tip_shown) else {
            return;
        };
        let note = self
            .note_icon_rects
            .iter()
            .find(|(_, l, _)| *l == line)
            .map(|(id, _, _)| id.clone())
            .or_else(|| {
                line_text(session, line)
                    .as_deref()
                    .and_then(crate::note_anchor::id_in)
                    .map(str::to_string)
            });
        let lines: Vec<String> = match note {
            Some(id) => {
                let mut out: Vec<String> = self
                    .note_previews
                    .get(&id)
                    .map(|p| p.lines().take(4).map(str::to_string).collect())
                    .unwrap_or_default();
                if out.is_empty() {
                    out.push(format!("Note {id}"));
                }
                out.push("Click to open · Ctrl+Shift+N".to_string());
                out
            }
            None => {
                let has_one = session
                    .document()
                    .gutter_marks()
                    .lines(crate::gutter_mark::GutterMarkKind::Breakpoint)
                    .contains(&line);
                if has_one {
                    return;
                }
                vec![TIP_TEXT.to_string()]
            }
        };
        let pad = dvec2(8.0, 5.0);
        let mut width = 0.0f64;
        let mut row = 0.0f64;
        for l in &lines {
            let laid = self
                .draw_tip_text
                .layout(cx, 0.0, 0.0, None, false, Align::default(), l);
            width = width.max(laid.size_in_lpxs.width as f64);
            row = row.max(laid.size_in_lpxs.height as f64);
        }
        let size = dvec2(width, row * lines.len() as f64) + 2.0 * pad;
        let pos = abs + dvec2(14.0, 18.0);
        self.draw_tip_bg.draw_abs(cx, Rect { pos, size });
        for (i, l) in lines.iter().enumerate() {
            self.draw_tip_text
                .draw_abs(cx, pos + pad + dvec2(0.0, row * i as f64), l);
        }
    }

    fn draw_note_popup(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        if !self.note_popup.open {
            return;
        }

        let selections = session.selections();
        let last_sel = session.last_added_selection_index().unwrap_or(0);
        let cursor_pos = selections.get(last_sel).map_or(
            Position {
                line_index: 0,
                byte_index: 0,
            },
            |s| s.cursor.position,
        );

        let layout = session.layout();
        let (x, y) = layout.logical_to_normalized_position(cursor_pos, Affinity::Before);
        let line_height = layout.line(cursor_pos.line_index).height() * self.cell_size.y;
        drop(layout);

        let caret_screen_pos = dvec2(
            x * self.cell_size.x + self.viewport_rect.pos.x,
            y * self.cell_size.y + self.viewport_rect.pos.y,
        );

        let font_scale = (self.cell_size.y / 18.0).clamp(0.85, 2.0);
        self.draw_note_popup_header.font_scale = font_scale as f32;
        self.draw_note_popup_title.font_scale = font_scale as f32;
        self.draw_note_popup_desc.font_scale = font_scale as f32;
        self.draw_note_popup_shortcut.font_scale = font_scale as f32;
        let items = self.note_popup.items.clone();
        // The panel is as wide as its widest text: a cut row is not acceptable.
        let width_of = |draw: &mut DrawText, cx: &mut Cx2d, text: &str| {
            let laid = draw.layout(cx, 0.0, 0.0, None, false, Align::default(), text);
            laid.size_in_lpxs.width as f64
        };
        let mut content_w = 0.0f64;
        for item in &items {
            let title = width_of(&mut self.draw_note_popup_title, cx, &item.title)
                + 24.0 * font_scale
                + width_of(&mut self.draw_note_popup_shortcut, cx, item.shortcut);
            let desc = width_of(&mut self.draw_note_popup_desc, cx, &item.description);
            content_w = content_w.max(title).max(desc);
        }
        self.note_popup.compute_layout(
            caret_screen_pos,
            line_height,
            self.viewport_rect,
            font_scale,
            content_w,
        );

        let panel_rect = self.note_popup.panel_rect;
        self.draw_note_popup_bg.draw_abs(cx, panel_rect);

        let padding = 6.0 * font_scale;
        let header_pos = panel_rect.pos + dvec2(padding + 4.0, padding + 2.0);
        self.draw_note_popup_header
            .draw_abs(cx, header_pos, &self.note_popup.header);

        // The selection first: drawn inside the loop, its draw call would
        // land after the texts of the rows above it and cover the row.
        if let Some(rect) = self
            .note_popup
            .selected
            .and_then(|i| self.note_popup.row_rects.get(i))
        {
            self.draw_note_popup_row_bg.draw_abs(cx, *rect);
        }
        // The gutter drew the icon shaders before the panel: fresh draw calls
        // put the row icons on top of it instead of under it.
        self.draw_note_doc.new_draw_call(cx);
        self.draw_note.new_draw_call(cx);
        self.draw_note_comment.new_draw_call(cx);
        let icon_side = 20.0 * font_scale;
        for (i, item) in items.iter().enumerate() {
            let Some(row_rect) = self.note_popup.row_rects.get(i).copied() else {
                continue;
            };
            // A row that cannot be chosen is drawn at 40%.
            let opacity = if item.enabled { 1.0f32 } else { 0.4f32 };
            let icon_rect = Rect {
                pos: dvec2(
                    row_rect.pos.x + 10.0 * font_scale,
                    row_rect.pos.y + (row_rect.size.y - icon_side) * 0.5,
                ),
                size: dvec2(icon_side, icon_side),
            };
            self.draw_note_icon(cx, item.kind, icon_rect, opacity);

            let text_x = icon_rect.pos.x + icon_side + 10.0 * font_scale;
            let title_y = row_rect.pos.y + 7.0 * font_scale;
            let shortcut_w = width_of(&mut self.draw_note_popup_shortcut, cx, item.shortcut);
            let shortcut_x = row_rect.pos.x + row_rect.size.x - 10.0 * font_scale - shortcut_w;
            let texts: [(&mut DrawText, DVec2, &str); 3] = [
                (
                    &mut self.draw_note_popup_title,
                    dvec2(text_x, title_y),
                    &item.title,
                ),
                (
                    &mut self.draw_note_popup_desc,
                    dvec2(text_x, title_y + 17.0 * font_scale),
                    &item.description,
                ),
                (
                    &mut self.draw_note_popup_shortcut,
                    dvec2(shortcut_x, title_y + font_scale),
                    item.shortcut,
                ),
            ];
            for (draw, pos, text) in texts {
                let keep = draw.color;
                draw.color.w *= opacity;
                draw.draw_abs(cx, pos, text);
                draw.color = keep;
            }
        }
    }

    /// Opens the note popup at the caret.
    pub fn open_note_popup(&mut self, header: String, rows: Vec<crate::note_popup::PopupRowParts>) {
        let items: Vec<crate::note_popup::PopupItem> = rows
            .into_iter()
            .map(|(kind, title, description, shortcut, enabled)| {
                let kind = match kind {
                    0 => crate::note_anchor::AnchorKind::Doc,
                    1 => crate::note_anchor::AnchorKind::Note,
                    _ => crate::note_anchor::AnchorKind::Comment,
                };
                crate::note_popup::PopupItem {
                    kind,
                    title,
                    description,
                    shortcut,
                    enabled,
                }
            })
            .collect();
        self.note_popup.open(header, items);
        self.note_popup_choice = None;
    }

    /// The chosen popup row index, once.
    pub fn take_note_popup_choice(&mut self) -> Option<usize> {
        self.note_popup_choice.take()
    }

    /// Whether `abs` falls on the glyph margin — the column left of the line
    /// numbers where a click toggles a breakpoint.
    fn is_in_glyph_margin(&self, abs: DVec2) -> bool {
        self.show_gutter
            && abs.x >= self.gutter_rect.pos.x
            && abs.x < self.gutter_rect.pos.x + self.glyph_margin
    }

    fn draw_text_layer(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        self.lens_rects.clear();
        let total_lines = session.document().as_text().as_lines().len();
        if total_lines == 0 || self.line_start >= total_lines || self.line_start >= self.line_end {
            return;
        }
        let highlighted_delimiter_positions = session.highlighted_delimiter_positions();
        let document = session.document();
        let semantic = document.semantic();
        let mut semantic = crate::semantic::SemanticCursor::new(&semantic);
        let caret_lines: Vec<usize> = session
            .selections()
            .iter()
            .map(|s| s.cursor.position.line_index)
            .collect();
        self.strikes.clear();
        let mut line_index = self.line_start;
        let mut origin_y = session.layout().top_of(self.line_start);
        for element in session
            .layout()
            .block_elements(self.line_start, self.line_end)
        {
            match element {
                BlockElement::Line { line, .. } => {
                    if line.is_collapsed() {
                        line_index += 1;
                        origin_y += line.height();
                        continue;
                    }
                    let has_caret = caret_lines.contains(&line_index);
                    let hide_range = if has_caret {
                        None
                    } else {
                        crate::note_anchor::hidden_span(line.text())
                    };
                    self.draw_text.font_scale = line.scale() as f32;
                    let mut token_iter = line.tokens().iter().copied();
                    let mut token_slot = token_iter.next();
                    let mut row_index = 0;
                    let mut byte_index = 0;
                    let mut column_index = 0;
                    for element in line.wrapped_elements() {
                        match element {
                            WrappedElement::Text {
                                is_inlay: false,
                                mut text,
                            } => {
                                while !text.is_empty() {
                                    let token = match token_slot {
                                        Some(token) => {
                                            if text.len() < token.len {
                                                token_slot = Some(Token {
                                                    len: token.len - text.len(),
                                                    kind: token.kind,
                                                });
                                                Token {
                                                    len: text.len(),
                                                    kind: token.kind,
                                                }
                                            } else {
                                                token_slot = token_iter.next();
                                                token
                                            }
                                        }
                                        None => Token {
                                            len: text.len(),
                                            kind: TokenKind::Unknown,
                                        },
                                    };
                                    let (text_0, text_1) = text.split_at(token.len);
                                    text = text_1;
                                    self.draw_text.color = match token.kind {
                                        TokenKind::Unknown => self.token_colors.unknown,
                                        TokenKind::BranchKeyword => {
                                            self.token_colors.branch_keyword
                                        }
                                        TokenKind::Comment => self.token_colors.comment,
                                        TokenKind::Constant => self.token_colors.constant,
                                        TokenKind::Delimiter => self.token_colors.delimiter,
                                        TokenKind::Identifier => self.token_colors.identifier,
                                        TokenKind::LoopKeyword => self.token_colors.loop_keyword,
                                        TokenKind::Number => self.token_colors.number,
                                        TokenKind::OtherKeyword => self.token_colors.other_keyword,
                                        TokenKind::Punctuator => self.token_colors.punctuator,
                                        TokenKind::String => self.token_colors.string,
                                        TokenKind::Function => self.token_colors.function,
                                        TokenKind::Typename => self.token_colors.typename,
                                        TokenKind::Whitespace => self.token_colors.whitespace,
                                    };
                                    self.draw_text.outline = 0.0;
                                    if let TokenKind::Delimiter = token.kind
                                        && highlighted_delimiter_positions.contains(&Position {
                                            line_index,
                                            byte_index,
                                        })
                                    {
                                        self.draw_text.outline = 1.0;
                                        self.draw_text.color = self.token_colors.delimiter_highlight
                                    }
                                    let token_color = self.draw_text.color;
                                    for grapheme in text_0.graphemes() {
                                        let at = Position {
                                            line_index,
                                            byte_index,
                                        };
                                        let span = semantic.at(at);
                                        self.draw_text.color = match span.and_then(|s| s.kind) {
                                            Some(kind) if self.draw_text.outline == 0.0 => {
                                                self.token_colors.semantic(kind)
                                            }
                                            _ => token_color,
                                        };
                                        let (x, y) = line
                                            .grid_to_normalized_position(row_index, column_index);
                                        let in_hidden = hide_range.is_some_and(|(s, e)| {
                                            byte_index >= s && byte_index < e
                                        });
                                        // A tab has no glyph in the code font: it
                                        // took its column and drew an empty box.
                                        if !in_hidden && grapheme != "\t" {
                                            if span.is_some_and(|s| s.struck) {
                                                self.strike(
                                                    DVec2 { x, y: origin_y + y },
                                                    grapheme.column_count() as f64,
                                                    line.scale(),
                                                );
                                            }
                                            self.draw_text.draw_abs(
                                                cx,
                                                DVec2 { x, y: origin_y + y } * self.cell_size
                                                    + dvec2(
                                                        self.viewport_rect.pos.x,
                                                        self.viewport_rect.pos.y
                                                            + self.cell_offset_y,
                                                    ),
                                                grapheme,
                                            );
                                        }
                                        byte_index += grapheme.len();
                                        column_index += grapheme.column_count();
                                    }
                                }
                            }
                            WrappedElement::Text {
                                is_inlay: true,
                                text,
                            } => {
                                let (x, y) =
                                    line.grid_to_normalized_position(row_index, column_index);
                                // Grey, and on the same baseline as the code:
                                // the text pass adds `cell_offset_y`, and an
                                // inlay without it sat a few pixels too high.
                                let keep = self.draw_text.color;
                                self.draw_text.color = INLAY_COLOR;
                                self.draw_text.draw_abs(
                                    cx,
                                    DVec2 { x, y: origin_y + y } * self.cell_size
                                        + dvec2(
                                            self.viewport_rect.pos.x,
                                            self.viewport_rect.pos.y + self.cell_offset_y,
                                        ),
                                    text,
                                );
                                self.draw_text.color = keep;
                                column_index += text.column_count();
                            }
                            WrappedElement::Widget(widget) => {
                                column_index += widget.column_count;
                            }
                            WrappedElement::Wrap => {
                                column_index = line.wrap_indent_column_count();
                                row_index += 1;
                            }
                        }
                    }
                    line_index += 1;
                    origin_y += line.height();
                }
                BlockElement::Widget(widget) => {
                    origin_y += widget.height;
                }
                BlockElement::Lens { text } => {
                    let document = session.document();
                    let source = document.as_text();
                    let line_below = source.as_lines().get(line_index);
                    let indent_cols = line_below
                        .and_then(|l| l.indent())
                        .unwrap_or("")
                        .column_count();
                    let keep_color = self.draw_text.color;
                    let keep_scale = self.draw_text.font_scale;
                    self.draw_text.color = INLAY_COLOR;
                    self.draw_text.font_scale = 0.85;
                    let x = indent_cols as f64 * self.cell_size.x + self.viewport_rect.pos.x;
                    let y = origin_y * self.cell_size.y + self.viewport_rect.pos.y;
                    self.draw_text
                        .draw_abs(cx, dvec2(x, y + self.cell_offset_y), text);
                    let size = dvec2(
                        self.viewport_rect.pos.x + self.viewport_rect.size.x - x,
                        self.cell_size.y,
                    );
                    self.lens_rects.push((
                        line_index,
                        Rect {
                            pos: dvec2(x, y),
                            size,
                        },
                    ));
                    self.draw_text.color = keep_color;
                    self.draw_text.font_scale = keep_scale;
                    origin_y += LENS_HEIGHT;
                }
            }
        }
    }

    /// Draws a subtle background band for the caret line of each active selection,
    /// spanning the full width of the text area.
    fn draw_current_line_layer(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        if !self.highlight_current_line {
            return;
        }
        let total = session.document().as_text().as_lines().len();
        if total == 0 {
            return;
        }
        let lines = crate::current_line::highlighted_lines_from_selections(
            &session.selections(),
            self.highlight_current_line,
        );
        for line in lines {
            if line < self.line_start || line >= self.line_end || line >= total {
                continue;
            }
            let layout = session.layout();
            let (_, y) = layout.logical_to_normalized_position(
                Position {
                    line_index: line,
                    byte_index: 0,
                },
                Affinity::Before,
            );
            let height = layout.line(line).height() * self.cell_size.y;
            drop(layout);
            let top = y * self.cell_size.y + self.viewport_rect.pos.y;
            self.draw_current_line.draw_abs(
                cx,
                Rect {
                    pos: dvec2(self.viewport_rect.pos.x, top),
                    size: dvec2(self.viewport_rect.size.x, height),
                },
            );
        }
    }

    fn draw_scope_header_layer(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        crate::scope_guides::draw_scope_header(
            cx,
            session,
            self.line_start,
            self.line_end,
            self.viewport_rect,
            self.cell_size,
            &mut self.draw_scope_header,
        );
    }

    fn draw_indent_guide_layer(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        crate::scope_guides::draw_indent_guides(
            cx,
            session,
            self.line_start,
            self.line_end,
            self.viewport_rect,
            self.cell_size,
            &mut self.draw_indent_guide,
        );
    }

    /// Where a note anchor's gutter icon sits on `line`: (pos, size).
    fn note_icon_rect(&self, session: &CodeSession, line: usize) -> Rect {
        let layout = session.layout();
        let (_, y) = layout.logical_to_normalized_position(
            Position {
                line_index: line,
                byte_index: 0,
            },
            Affinity::Before,
        );
        let scale = layout.line(line).scale();
        drop(layout);
        let h = (self.cell_size.y * scale * 0.82).min(self.glyph_margin * 0.62);
        let w = h * 0.78;
        let top = y * self.cell_size.y + self.gutter_rect.pos.y;
        Rect {
            pos: dvec2(
                self.gutter_rect.pos.x + (self.glyph_margin - w) * 0.5,
                top + (self.cell_size.y * scale - h) * 0.5,
            ),
            size: dvec2(w, h),
        }
    }

    /// Draws a note icon of the given `kind` in `rect` with `opacity` (0.0..=1.0).
    pub fn draw_note_icon(
        &mut self,
        cx: &mut Cx2d,
        kind: crate::note_anchor::AnchorKind,
        rect: Rect,
        opacity: f32,
    ) {
        match kind {
            crate::note_anchor::AnchorKind::Doc => {
                let keep = self.draw_note_doc.color;
                self.draw_note_doc.color = vec4(keep.x, keep.y, keep.z, opacity);
                self.draw_note_doc.draw_abs(cx, rect);
                self.draw_note_doc.color = keep;
            }
            crate::note_anchor::AnchorKind::Note => {
                let keep = self.draw_note.color;
                self.draw_note.color = vec4(keep.x, keep.y, keep.z, opacity);
                self.draw_note.draw_abs(cx, rect);
                self.draw_note.color = keep;
            }
            crate::note_anchor::AnchorKind::Comment => {
                let keep = self.draw_note_comment.color;
                self.draw_note_comment.color = vec4(keep.x, keep.y, keep.z, opacity);
                self.draw_note_comment.draw_abs(cx, rect);
                self.draw_note_comment.color = keep;
            }
        }
    }

    /// The note icon on every visible anchor line; anchors without a preview
    /// are noted for the IDE to fetch.
    fn draw_note_layer(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        self.note_icon_rects.clear();
        let doc_text = session.document().as_text();
        let doc_lines = doc_text.as_lines();
        let total = doc_lines.len();
        if total == 0 {
            return;
        }
        let all_lines: Vec<&str> = doc_lines.iter().map(String::as_str).collect();

        let mut scan_start = self.line_start.min(total);
        while scan_start > 0 {
            let prev = scan_start - 1;
            let text = all_lines[prev].trim();
            if crate::note_anchor::is_link_only_line(text)
                || (text.starts_with("#[") && text.ends_with(']'))
                || text.starts_with("///")
            {
                scan_start = prev;
            } else {
                break;
            }
        }

        for line in scan_start..self.line_end.min(total) {
            let text = all_lines[line];
            let Some((kind, id)) = crate::note_anchor::anchor_in(text) else {
                continue;
            };
            let id = id.to_string();

            if !self.note_previews.contains_key(&id)
                && !self.notes_missing.iter().any(|(m, _)| m == &id)
            {
                self.notes_missing.push((id.clone(), line));
            }

            let target_line = if crate::note_anchor::is_link_only_line(text) {
                crate::note_anchor::icon_line(&all_lines, line)
            } else {
                line
            };

            if !self.show_gutter || target_line >= total || self.note_hidden.contains(&id) {
                continue;
            }

            let rect = self.note_icon_rect(session, target_line);
            self.draw_note_icon(cx, kind, rect, 1.0);
            if self.note_alerts.contains(&id) {
                let d = rect.size.y * 0.62;
                let dot_rect = Rect {
                    pos: dvec2(rect.pos.x + rect.size.x - d * 0.55, rect.pos.y - d * 0.3),
                    size: dvec2(d, d),
                };
                self.draw_note_alert.draw_abs(cx, dot_rect);
            }
            self.note_icon_rects.push((id, target_line, rect));
        }
    }

    /// The anchor's tag folded into a pill with the note's title, over the
    /// tag's own cells — the text stays as it is; the caret on the line
    /// unfolds it.
    /// The extensions' marks: the pill over its span while the caret
    /// is off the line, the icon in the glyph margin. A note's line keeps
    /// only its note.
    fn draw_line_mark_layer(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        self.line_mark_rects.clear();
        let total = session.document().as_text().as_lines().len();
        let carets: Vec<usize> = session
            .selections()
            .iter()
            .map(|s| s.cursor.position.line_index)
            .collect();
        let marks = std::mem::take(&mut self.line_marks);
        for line in self.line_start..self.line_end.min(total) {
            let Some(text) = line_text(session, line) else {
                continue;
            };
            if crate::note_anchor::id_in(&text).is_some() {
                continue;
            }
            for (k, m) in crate::line_mark::on_line(&marks, line).enumerate() {
                let (r, g, b) = crate::line_mark::kind_rgb(&m.kind);
                let end = m.end.min(text.len());
                if !carets.contains(&line)
                    && m.start < end
                    && text.is_char_boundary(m.start)
                    && text.is_char_boundary(end)
                {
                    self.draw_mark_pill(cx, session, m, end, vec4(r, g, b, 1.0));
                }
                if !self.show_gutter || k > 0 {
                    continue;
                }
                let layout = session.layout();
                let (_, y) = layout.logical_to_normalized_position(
                    Position {
                        line_index: line,
                        byte_index: 0,
                    },
                    Affinity::Before,
                );
                let scale = layout.line(line).scale();
                drop(layout);
                let side = (self.cell_size.y * scale * 0.8).min(self.glyph_margin * 0.66);
                let top = y * self.cell_size.y + self.gutter_rect.pos.y;
                let rect = Rect {
                    pos: dvec2(
                        self.gutter_rect.pos.x + (self.glyph_margin - side) * 0.5,
                        top + (self.cell_size.y * scale - side) * 0.5,
                    ),
                    size: dvec2(side, side),
                };
                self.draw_line_mark.color = vec4(r, g, b, 1.0);
                self.draw_line_mark.draw_abs(cx, rect);
                self.line_mark_rects
                    .push((m.owner.clone(), m.id.clone(), rect));
            }
        }
        self.line_marks = marks;
    }

    fn draw_mark_pill(
        &mut self,
        cx: &mut Cx2d,
        session: &CodeSession,
        m: &crate::line_mark::LineMark,
        end: usize,
        color: Vec4,
    ) {
        let layout = session.layout();
        let at = |byte_index, affinity| {
            layout.logical_to_normalized_position(
                Position {
                    line_index: m.line,
                    byte_index,
                },
                affinity,
            )
        };
        let ((x0, y), (x1, _)) = (at(m.start, Affinity::After), at(end, Affinity::Before));
        let scale = layout.line(m.line).scale();
        drop(layout);
        let rect = Rect {
            pos: DVec2 { x: x0, y } * self.cell_size + self.viewport_rect.pos,
            size: dvec2(
                (x1 - x0).max(0.0) * self.cell_size.x,
                scale * self.cell_size.y,
            ),
        };
        let room = (x1 - x0).max(0.0).round() as usize;
        let label = crate::note_anchor::fit_label(&m.label, room);
        self.draw_note_chip.color = vec4(color.x * 0.18, color.y * 0.18, color.z * 0.18, 1.0);
        self.draw_note_chip.draw_abs(cx, rect);
        self.draw_note_chip_text.font_scale = self.draw_text.font_scale;
        self.draw_note_chip_text.color = color;
        let pos = rect.pos + dvec2(self.cell_size.x * 0.5, self.cell_offset_y);
        self.draw_note_chip_text.draw_abs(cx, pos, &label);
        self.draw_note_chip.color = vec4(0.176, 0.165, 0.133, 1.0);
        self.draw_note_chip_text.color = vec4(0.843, 0.729, 0.49, 1.0);
    }

    /// The extensions' marks on this file.
    pub fn set_line_marks(&mut self, marks: Vec<crate::line_mark::LineMark>) {
        self.line_marks = marks;
    }

    /// Where each mark's icon was last drawn: (owner, id, rect).
    pub fn line_mark_rects(&self) -> &[(String, String, Rect)] {
        &self.line_mark_rects
    }

    /// Where each code lens row was last drawn: (the line below it, rect).
    pub fn lens_rects(&self) -> &[(usize, Rect)] {
        &self.lens_rects
    }

    /// The mark whose icon was clicked, once: (owner, id).
    pub fn take_line_mark_click(&mut self) -> Option<(String, String)> {
        self.line_mark_clicked.take()
    }

    /// The mouse rested on a name, once: where.
    pub fn take_hover_rest(&mut self) -> Option<HoverRest> {
        self.hover_rest.take()
    }

    /// The mouse moved to another place in the text, once.
    pub fn take_hover_moved(&mut self) -> bool {
        std::mem::take(&mut self.hover_moved)
    }

    /// The name under the mouse: a new one restarts the rest timer and tells
    /// the IDE the old place was left; Ctrl over it draws the link.
    fn hover_word(&mut self, cx: &mut Cx, session: &CodeSession, abs: DVec2, ctrl: bool) {
        let ((cursor, _), in_gutter) = self.pick(session, abs);
        let line = cursor.line_index;
        let span = (!in_gutter)
            .then(|| line_text(session, line))
            .flatten()
            .and_then(|t| word_span(&t, cursor.byte_index));
        let now = span.map(|(start, end)| WordHover {
            line,
            start,
            end,
            ctrl,
        });
        let same = now.map(|w| (w.line, w.start)) == self.word_hover.map(|w| (w.line, w.start));
        if !same {
            self.hover_moved = true;
            self.hover_rest = None;
            cx.stop_timer(self.hover_timer);
            if now.is_some() {
                self.hover_timer = cx.start_timeout(HOVER_DELAY);
            }
        }
        if now.map(|w| w.ctrl) != self.word_hover.map(|w| w.ctrl) || !same {
            self.redraw(cx);
        }
        if now.is_some_and(|w| w.ctrl) {
            cx.set_cursor(MouseCursor::Hand);
        }
        self.word_hover = now;
    }

    /// The screen box of `w`, for the card.
    fn rest_of(&self, session: Option<&CodeSession>, w: WordHover) -> Option<HoverRest> {
        self.anchor_at(session?, w.line, w.start)
    }

    /// Where text position (`line`, `byte`) is on screen, in window
    /// coordinates — the inverse of `pick`, so a card placed by the mouse
    /// and one placed by the caret land the same way.
    /// Does the editor take no typing?
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    /// Everything selected — `Ctrl+A` and the editor's Select All.
    pub fn select_all(&mut self, cx: &mut Cx, session: &CodeSession) {
        let ((cursor, affinity), _is_in_gutter) = self.pick(session, dvec2(0.0, 0.0));
        session.set_selection(cursor, affinity, SelectionMode::All, NewGroup::Yes);
        self.reset_cursor_blinker(cx);
        self.keep_cursor_in_view = KeepCursorInView::Off;
        self.redraw(cx);
    }

    pub fn anchor_at(&self, session: &CodeSession, line: usize, byte: usize) -> Option<HoverRest> {
        let w = WordHover {
            line,
            start: byte,
            end: byte,
            ctrl: false,
        };
        let layout = session.layout();
        let start = Position {
            line_index: w.line,
            byte_index: w.start,
        };
        let (x, y) = layout.logical_to_normalized_position(start, Affinity::After);
        let scale = layout.line(w.line).scale();
        Some(HoverRest {
            line: w.line,
            byte: w.start,
            at: DVec2 { x, y } * self.cell_size + self.viewport_rect.pos,
            line_height: scale * self.cell_size.y,
        })
    }

    /// Ctrl held over a name: a line under it, like a link.
    fn draw_link_underline(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        let Some(w) = self.word_hover.filter(|w| w.ctrl) else {
            return;
        };
        if w.line < self.line_start || w.line >= self.line_end {
            return;
        }
        let layout = session.layout();
        let p = |b| Position {
            line_index: w.line,
            byte_index: b,
        };
        let (x0, y) = layout.logical_to_normalized_position(p(w.start), Affinity::After);
        let (x1, _) = layout.logical_to_normalized_position(p(w.end), Affinity::Before);
        let scale = layout.line(w.line).scale();
        drop(layout);
        let top = DVec2 { x: x0, y } * self.cell_size + self.viewport_rect.pos;
        let rect = Rect {
            pos: top
                + DVec2 {
                    x: 0.0,
                    y: scale * self.cell_size.y - 1.5,
                },
            size: DVec2 {
                x: (x1 - x0).max(0.0) * self.cell_size.x,
                y: 1.5,
            },
        };
        self.draw_link.draw_abs(cx, rect);
    }

    /// The note the user clicked in the margin, once.
    pub fn take_note_click(&mut self) -> Option<String> {
        self.note_clicked.take()
    }

    /// Whether the last click was a Ctrl+Click on the text, once.
    pub fn take_ctrl_click(&mut self) -> bool {
        std::mem::take(&mut self.ctrl_clicked)
    }

    /// Anchors of visible lines that still lack a preview, once each: (id, 0-based line).
    pub fn take_missing_notes(&mut self) -> Vec<(String, usize)> {
        std::mem::take(&mut self.notes_missing)
    }

    /// A note's preview, for the tip over its anchor.
    pub fn set_note_preview(&mut self, id: &str, preview: &str) {
        self.note_previews
            .insert(id.to_string(), preview.to_string());
    }

    /// Whether an anchor icon carries the orange alert dot (doc out of rustdoc rule or signature changed).
    pub fn set_note_alert(&mut self, id: &str, on: bool) {
        if on {
            self.note_alerts.insert(id.to_string());
        } else {
            self.note_alerts.remove(id);
        }
    }

    /// Whether an anchor icon is hidden: a resolved comment draws no gutter icon.
    pub fn set_note_hidden(&mut self, id: &str, on: bool) {
        if on {
            self.note_hidden.insert(id.to_string());
        } else {
            self.note_hidden.remove(id);
        }
    }

    /// Clears the cached previews and missing lists so visible anchors refetch their preview and alert state.
    pub fn forget_note_previews(&mut self) {
        self.note_previews.clear();
        self.notes_missing.clear();
    }

    /// Full-width background, 3 px left bar, and strike lines for diff review rows.
    fn draw_review_layer(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        let review_rows = session.document().review_rows();
        if review_rows.is_empty() {
            return;
        }
        let total = session.document().as_text().as_lines().len();
        let end_line = self.line_end.min(total).min(review_rows.len());
        for line_index in self.line_start..end_line {
            let row = &review_rows[line_index];
            if row.kind == crate::review::ReviewRowKind::Same {
                continue;
            }
            let is_added = row.kind == crate::review::ReviewRowKind::Added;
            let (bg_color, bar_color) = if is_added {
                (REVIEW_ADDED_BG, REVIEW_ADDED_BAR)
            } else {
                (REVIEW_REMOVED_BG, REVIEW_REMOVED_BAR)
            };

            let layout = session.layout();
            let (_, y) = layout.logical_to_normalized_position(
                Position {
                    line_index,
                    byte_index: 0,
                },
                Affinity::Before,
            );
            let line = layout.line(line_index);
            let height = line.height() * self.cell_size.y;
            let top = y * self.cell_size.y + self.viewport_rect.pos.y;

            // Full-width background over the text viewport
            self.draw_review_bg.color = bg_color;
            self.draw_review_bg.draw_abs(
                cx,
                Rect {
                    pos: dvec2(self.viewport_rect.pos.x, top),
                    size: dvec2(self.viewport_rect.size.x, height),
                },
            );

            // 3 px bar at the left edge of the text area
            self.draw_review_bar.color = bar_color;
            self.draw_review_bar.draw_abs(
                cx,
                Rect {
                    pos: dvec2(self.viewport_rect.pos.x, top),
                    size: dvec2(3.0, height),
                },
            );

            // Removed only: 1 px strike line through the middle of the line
            if !is_added && let Some(start) = line.text.find(|c: char| !c.is_whitespace()) {
                let end = line.text.trim_end().len();
                if end > start {
                    let (r_start, c_start) = line.logical_to_grid_position(start, Affinity::Before);
                    let (x_start, y_start) = line.grid_to_normalized_position(r_start, c_start);

                    let (r_end, c_end) = line.logical_to_grid_position(end, Affinity::After);
                    let (x_end, _) = line.grid_to_normalized_position(r_end, c_end);

                    let norm_y = line.y() + y_start;
                    let pixel_x = x_start * self.cell_size.x + self.viewport_rect.pos.x;
                    let pixel_w = ((x_end - x_start) * self.cell_size.x).max(self.cell_size.x);
                    let pixel_y =
                        (norm_y + line.scale() * 0.5) * self.cell_size.y + self.viewport_rect.pos.y;

                    self.draw_review_strike.color = REVIEW_STRIKE;
                    self.draw_review_strike.draw_abs(
                        cx,
                        Rect {
                            pos: dvec2(pixel_x, pixel_y),
                            size: dvec2(pixel_w, 1.0),
                        },
                    );
                }
            }
        }
    }

    /// Checks whether `abs` falls inside any per-change review button.
    pub fn hit_review_button(&self, abs: DVec2) -> Option<(usize, bool)> {
        self.review_button_rects
            .iter()
            .find(|(_, _, r)| r.contains(abs))
            .map(|&(change, accept, _)| (change, accept))
    }

    /// Two small buttons on the first row of each diff review change:
    /// `Accept` and `Reject`.
    fn draw_review_buttons(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        self.review_button_rects.clear();
        if !self.review_buttons || !session.document().reviewing() {
            return;
        }

        let review_rows = session.document().review_rows();
        if review_rows.is_empty() {
            return;
        }

        // One pair per change, on its first ADDED row (the mockup: the struck
        // removed rows above carry none); a pure deletion uses its first row.
        use crate::review::ReviewRowKind::{Added, Same};
        let mut changes: Vec<(usize, usize)> = Vec::new();
        let mut previous = Same;
        for (row_idx, row) in review_rows.iter().enumerate() {
            if row.kind != Same && previous == Same {
                changes.push((changes.len(), row_idx));
            } else if row.kind == Added
                && previous != Added
                && previous != Same
                && let Some(last) = changes.last_mut()
            {
                last.1 = row_idx;
            }
            previous = row.kind;
        }

        let total_lines = session.document().as_text().as_lines().len();
        let right = self.viewport_rect.pos.x + self.viewport_rect.size.x - 14.0;
        for (k, row) in changes {
            if row >= total_lines || row < self.line_start || row >= self.line_end {
                continue;
            }
            let layout = session.layout();
            let at = Position {
                line_index: row,
                byte_index: 0,
            };
            let (_, y) = layout.logical_to_normalized_position(at, Affinity::Before);
            let middle = (y + layout.line(row).height() * 0.5) * self.cell_size.y;
            drop(layout);
            let middle = middle + self.viewport_rect.pos.y;
            // Right to left: Reject at the edge, Accept before it, 6 px apart.
            let reject = self.draw_review_button(cx, false, right, middle);
            let accept = self.draw_review_button(cx, true, reject.pos.x - 6.0, middle);
            self.review_button_rects
                .extend([(k, true, accept), (k, false, reject)]);
        }
    }

    /// One 22 px pill ending at `right`, centred on `middle`: `Accept` or
    /// `Reject` with icon. Returns its rect, for the click.
    fn draw_review_button(&mut self, cx: &mut Cx2d, accept: bool, right: f64, middle: f64) -> Rect {
        let (bg, icon, text, label) = match accept {
            true => (
                &mut self.draw_review_btn_accept,
                &mut self.draw_review_btn_accept_icon,
                &mut self.draw_review_btn_accept_text,
                "Accept",
            ),
            false => (
                &mut self.draw_review_btn_reject,
                &mut self.draw_review_btn_reject_icon,
                &mut self.draw_review_btn_reject_text,
                "Reject",
            ),
        };
        icon.color = text.color;
        let laid = text.layout(cx, 0.0, 0.0, None, false, Align::default(), label);
        let (w, h) = (
            laid.size_in_lpxs.width as f64,
            laid.size_in_lpxs.height as f64,
        );
        let icon_w = 10.0;
        let gap = 5.0;
        let total_w = w + 16.0 + icon_w + gap;
        let rect = Rect {
            pos: dvec2(right - total_w, middle - 11.0),
            size: dvec2(total_w, 22.0),
        };
        bg.draw_abs(cx, rect);
        let icon_rect = Rect {
            pos: dvec2(rect.pos.x + 8.0, middle - icon_w * 0.5),
            size: dvec2(icon_w, icon_w),
        };
        icon.draw_abs(cx, icon_rect);
        text.draw_abs(
            cx,
            dvec2(rect.pos.x + 8.0 + icon_w + gap, middle - h * 0.5),
            label,
        );
        rect
    }

    /// The band behind each line with a diagnostic message, across the whole
    /// text area, in the severity's tint (the squiggle still marks only the
    /// range the server gave).
    fn draw_message_line_layer(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        let messages: Vec<_> = session.document().line_messages().to_vec();
        for m in messages {
            if m.line < self.line_start || m.line >= self.line_end {
                continue;
            }
            let layout = session.layout();
            let (_, y) = layout.logical_to_normalized_position(
                Position {
                    line_index: m.line,
                    byte_index: 0,
                },
                Affinity::Before,
            );
            let height = layout.line(m.line).height() * self.cell_size.y;
            drop(layout);
            self.draw_message_line.color = self.diagnostic_colors.of(m.severity).1;
            self.draw_message_line.draw_abs(
                cx,
                Rect {
                    pos: dvec2(
                        self.viewport_rect.pos.x,
                        y * self.cell_size.y + self.viewport_rect.pos.y,
                    ),
                    size: dvec2(self.viewport_rect.size.x, height),
                },
            );
        }
    }

    /// Each line's diagnostic message, three cells after the end of the line,
    /// in the severity's colour; its first line only, cut to fit a glance.
    fn draw_message_layer(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        let messages: Vec<_> = session.document().line_messages().to_vec();
        if messages.is_empty() {
            return;
        }
        let text = session.document().as_text().as_lines().to_vec();
        for m in messages {
            if m.line < self.line_start || m.line >= self.line_end {
                continue;
            }
            let end = Position {
                line_index: m.line,
                byte_index: text.get(m.line).map_or(0, String::len),
            };
            let (x, y) = session
                .layout()
                .logical_to_normalized_position(end, Affinity::After);
            self.draw_message.color = self.diagnostic_colors.of(m.severity).0;
            self.draw_message.font_scale = self.draw_text.font_scale;
            let pos = DVec2 { x: x + 3.0, y } * self.cell_size
                + dvec2(
                    self.viewport_rect.pos.x,
                    self.viewport_rect.pos.y + self.cell_offset_y,
                );
            self.draw_message.draw_abs(cx, pos, &message_line(&m.text));
        }
    }

    /// The occurrences of the name under the caret, as boxes behind the text.
    fn draw_occurrence_layer(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        let found: Vec<_> = session.document().occurrences().to_vec();
        for o in found {
            let line = o.start.line_index;
            if line < self.line_start || line >= self.line_end {
                continue;
            }
            let layout = session.layout();
            let (x0, y) = layout.logical_to_normalized_position(o.start, Affinity::After);
            let (x1, _) = layout.logical_to_normalized_position(o.end, Affinity::Before);
            let scale = layout.line(line).scale();
            drop(layout);
            let rect = Rect {
                pos: DVec2 { x: x0, y } * self.cell_size + self.viewport_rect.pos,
                size: DVec2 {
                    x: (x1 - x0).max(0.0) * self.cell_size.x,
                    y: scale * self.cell_size.y,
                },
            };
            if o.write {
                self.draw_occurrence_write.draw_abs(cx, rect);
            } else {
                self.draw_occurrence.draw_abs(cx, rect);
            }
        }
    }

    /// One glyph of a deprecated name at `at` in cells: its rect,
    /// joined to the glyph drawn just before it on the same row.
    fn strike(&mut self, at: DVec2, columns: f64, scale: f64) {
        let rect = Rect {
            pos: at * self.cell_size + self.viewport_rect.pos,
            size: DVec2 {
                x: columns,
                y: scale,
            } * self.cell_size,
        };
        match self.strikes.last_mut() {
            Some(last)
                if last.pos.y == rect.pos.y
                    && (last.pos.x + last.size.x - rect.pos.x).abs() < 0.5 =>
            {
                last.size.x += rect.size.x
            }
            _ => self.strikes.push(rect),
        }
    }

    fn draw_decoration_layer(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        // The deprecated names first: struck through, with no squiggle.
        let draw = &mut self.draw_decoration;
        (draw.color, draw.strike_color) =
            (Vec4::default(), self.token_colors.deprecated_decoration);
        (draw.faded, draw.struck) = (0.0, 1.0);
        for rect in mem::take(&mut self.strikes) {
            self.draw_decoration.draw_abs(cx, rect);
        }
        let mut active_decoration = None;
        let decorations = session.document().decorations();
        let mut decorations = decorations.iter();
        while decorations
            .as_slice()
            .first()
            .is_some_and(|decoration| decoration.end().line_index < self.line_start)
        {
            decorations.next().unwrap();
        }
        if decorations
            .as_slice()
            .first()
            .is_some_and(|decoration| decoration.start().line_index < self.line_start)
        {
            active_decoration = Some(ActiveDecoration {
                decoration: *decorations.next().unwrap(),
                start_x: 0.0,
            });
        }
        DrawDecorationLayer {
            code_editor: self,
            active_decoration,
            decorations,
        }
        .draw_decoration_layer(cx, session)
    }

    fn draw_selection_layer(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        let mut active_selection = None;
        let selections = session.selections();
        let mut selections = selections.iter();
        while selections
            .as_slice()
            .first()
            .is_some_and(|selection| selection.end().line_index < self.line_start)
        {
            selections.next().unwrap();
        }
        if selections
            .as_slice()
            .first()
            .is_some_and(|selection| selection.start().line_index < self.line_start)
        {
            active_selection = Some(ActiveSelection {
                selection: *selections.next().unwrap(),
                start_x: 0.0,
            });
        }
        DrawSelectionLayer {
            code_editor: self,
            active_selection,
            selections,
        }
        .draw_selection_layer(cx, session)
    }

    fn draw_landing_layer(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        let Some(landing) = self.landing_indicator else {
            return;
        };
        let total_lines = session.document().as_text().as_lines().len();
        if landing.line_index >= total_lines
            || landing.line_index < self.line_start
            || landing.line_index >= self.line_end
        {
            return;
        }

        let layout = session.layout();
        let line = layout.line(landing.line_index);
        let text_len = line.text.len();
        let (safe_start, safe_end) = if landing.start_byte == 0 && landing.end_byte == 0 {
            let start = line.text.find(|c: char| !c.is_whitespace()).unwrap_or(0);
            let end = line.text.trim_end().len();
            (start, end.max(start))
        } else {
            let s = landing.start_byte.min(text_len);
            let e = landing.end_byte.min(text_len).max(s);
            (s, e)
        };

        let (r_start, c_start) = line.logical_to_grid_position(safe_start, Affinity::Before);
        let (x_start, y_start) = line.grid_to_normalized_position(r_start, c_start);

        let (r_end, c_end) = if safe_end > safe_start {
            line.logical_to_grid_position(safe_end, Affinity::After)
        } else {
            (r_start, c_start + 1)
        };
        let (x_end, _) = line.grid_to_normalized_position(r_end, c_end);

        let norm_y = line.y() + y_start;
        let pixel_x = x_start * self.cell_size.x + self.viewport_rect.pos.x;
        let pixel_w = ((x_end - x_start) * self.cell_size.x).max(self.cell_size.x);
        // A hairline: it marks where the caret landed, it must not shout.
        let pixel_y = (norm_y + line.scale()) * self.cell_size.y + self.viewport_rect.pos.y - 1.0;

        self.draw_landing_line.draw_abs(
            cx,
            Rect {
                pos: dvec2(pixel_x, pixel_y),
                size: dvec2(pixel_w, 1.0),
            },
        );
    }

    pub fn set_landing_line(&mut self, line_index: usize, start_byte: usize, end_byte: usize) {
        self.landing_indicator = Some(LandingIndicator {
            line_index,
            start_byte,
            end_byte,
        });
    }

    pub fn clear_landing_line(&mut self) {
        self.landing_indicator = None;
    }

    fn pick(&self, session: &CodeSession, position: DVec2) -> ((Position, Affinity), bool) {
        let position = (position - self.viewport_rect.pos) / self.cell_size;

        if position.y < 0.0 {
            let layout = session.layout();
            let first_line = (0..layout.as_text().as_lines().len())
                .find(|&idx| !layout.line(idx).is_collapsed())
                .unwrap_or(0);
            drop(layout);
            return (
                (
                    Position {
                        line_index: first_line,
                        byte_index: 0,
                    },
                    Affinity::Before,
                ),
                false,
            );
        }
        let layout = session.layout();
        if position.y > layout.height() {
            let lines = layout.as_text().as_lines();
            let last_line = (0..lines.len())
                .rfind(|&idx| !layout.line(idx).is_collapsed())
                .unwrap_or_else(|| lines.len().saturating_sub(1));
            return (
                (
                    Position {
                        line_index: last_line,
                        byte_index: lines.get(last_line).map_or(0, String::len),
                    },
                    Affinity::After,
                ),
                false,
            );
        }
        let total_lines = layout.as_text().as_lines().len();
        let mut line_index = layout.find_first_line_ending_after_y(position.y);
        let mut origin_y = layout.top_of(line_index);
        for block in layout.block_elements(line_index, total_lines) {
            match block {
                BlockElement::Line {
                    is_inlay: false,
                    line,
                } => {
                    if line.is_collapsed() {
                        line_index += 1;
                        origin_y += line.height();
                        continue;
                    }
                    let mut byte_index = 0;
                    let mut row_index = 0;
                    let mut column_index = 0;
                    for element in line.wrapped_elements() {
                        match element {
                            WrappedElement::Text {
                                is_inlay: false,
                                text,
                            } => {
                                for grapheme in text.graphemes() {
                                    let (start_x, y) =
                                        line.grid_to_normalized_position(row_index, column_index);
                                    let start_y = origin_y + y;
                                    let (end_x, _) = line.grid_to_normalized_position(
                                        row_index,
                                        column_index + grapheme.column_count(),
                                    );
                                    let end_y = start_y + line.scale();
                                    if (start_y..=end_y).contains(&position.y) {
                                        let mid_x = (start_x + end_x) / 2.0;
                                        if (start_x..=mid_x).contains(&position.x) {
                                            return (
                                                (
                                                    Position {
                                                        line_index,
                                                        byte_index,
                                                    },
                                                    Affinity::After,
                                                ),
                                                false,
                                            );
                                        }
                                        if (mid_x..=end_x).contains(&position.x) {
                                            return (
                                                (
                                                    Position {
                                                        line_index,
                                                        byte_index: byte_index + grapheme.len(),
                                                    },
                                                    Affinity::Before,
                                                ),
                                                false,
                                            );
                                        }
                                    }
                                    byte_index += grapheme.len();
                                    column_index += grapheme.column_count();
                                }
                            }
                            WrappedElement::Text {
                                is_inlay: true,
                                text,
                            } => {
                                let (start_x, y) =
                                    line.grid_to_normalized_position(row_index, column_index);
                                let start_y = origin_y + y;
                                let (end_x, _) = line.grid_to_normalized_position(
                                    row_index,
                                    column_index + text.column_count(),
                                );
                                let end_y = origin_y + line.scale();
                                if (start_y..=end_y).contains(&position.y)
                                    && (start_x..=end_x).contains(&position.x)
                                {
                                    return (
                                        (
                                            Position {
                                                line_index,
                                                byte_index,
                                            },
                                            Affinity::Before,
                                        ),
                                        false,
                                    );
                                }
                                column_index += text.column_count();
                            }
                            WrappedElement::Widget(widget) => {
                                column_index += widget.column_count;
                            }
                            WrappedElement::Wrap => {
                                let (_, y) =
                                    line.grid_to_normalized_position(row_index, column_index);
                                let start_y = origin_y + y;
                                let end_y = start_y + line.scale();
                                if (start_y..=end_y).contains(&position.y) {
                                    return if position.x < 0.0 {
                                        (
                                            (
                                                Position {
                                                    line_index,
                                                    byte_index: 0,
                                                },
                                                Affinity::Before,
                                            ),
                                            true,
                                        )
                                    } else {
                                        (
                                            (
                                                Position {
                                                    line_index,
                                                    byte_index,
                                                },
                                                Affinity::Before,
                                            ),
                                            false,
                                        )
                                    };
                                }
                                column_index = line.wrap_indent_column_count();
                                row_index += 1;
                            }
                        }
                    }
                    let (_, y) = line.grid_to_normalized_position(row_index, column_index);
                    let start_y = origin_y + y;
                    let end_y = start_y + line.scale();
                    if (start_y..=end_y).contains(&position.y) {
                        return if position.x < 0.0 {
                            (
                                (
                                    Position {
                                        line_index,
                                        byte_index: 0,
                                    },
                                    Affinity::Before,
                                ),
                                true,
                            )
                        } else {
                            (
                                (
                                    Position {
                                        line_index,
                                        byte_index,
                                    },
                                    Affinity::Before,
                                ),
                                false,
                            )
                        };
                    }
                    line_index += 1;
                    origin_y += line.height();
                }
                BlockElement::Line {
                    is_inlay: true,
                    line,
                } => {
                    let start_y = origin_y;
                    let end_y = start_y + line.height();
                    if (start_y..=end_y).contains(&position.y) {
                        return (
                            (
                                Position {
                                    line_index,
                                    byte_index: 0,
                                },
                                Affinity::Before,
                            ),
                            false,
                        );
                    }
                    origin_y += line.height();
                }
                BlockElement::Widget(widget) => {
                    origin_y += widget.height;
                }
                BlockElement::Lens { .. } => {
                    let start_y = origin_y;
                    let end_y = start_y + LENS_HEIGHT;
                    if (start_y..=end_y).contains(&position.y) {
                        return (
                            (
                                Position {
                                    line_index,
                                    byte_index: 0,
                                },
                                Affinity::Before,
                            ),
                            false,
                        );
                    }
                    origin_y += LENS_HEIGHT;
                }
            }
        }
        let lines = layout.as_text().as_lines();
        let last_line = (0..lines.len())
            .rfind(|&idx| !layout.line(idx).is_collapsed())
            .unwrap_or_else(|| lines.len().saturating_sub(1));
        (
            (
                Position {
                    line_index: last_line,
                    byte_index: lines.get(last_line).map_or(0, String::len),
                },
                Affinity::After,
            ),
            false,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, DefaultNone)]
pub enum CodeEditorAction {
    TextDidChange,
    UnhandledKeyDown(KeyEvent),
    LensClicked(usize),
    ReviewChoice { change: usize, accept: bool },
    None,
}

struct DrawDecorationLayer<'a> {
    code_editor: &'a mut CodeEditor,
    active_decoration: Option<ActiveDecoration>,
    decorations: Iter<'a, Decoration>,
}

impl<'a> DrawDecorationLayer<'a> {
    fn draw_decoration_layer(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        let total_lines = session.document().as_text().as_lines().len();
        if total_lines == 0
            || self.code_editor.line_start >= total_lines
            || self.code_editor.line_start >= self.code_editor.line_end
        {
            return;
        }
        let mut line_index = self.code_editor.line_start;
        let mut origin_y = session.layout().top_of(self.code_editor.line_start);
        for block in session
            .layout()
            .block_elements(self.code_editor.line_start, self.code_editor.line_end)
        {
            match block {
                BlockElement::Line {
                    is_inlay: false,
                    line,
                } => {
                    let mut byte_index = 0;
                    let mut row_index = 0;
                    let mut column_index = 0;
                    self.handle_event(
                        cx,
                        line_index,
                        line,
                        byte_index,
                        Affinity::Before,
                        origin_y,
                        row_index,
                        column_index,
                    );
                    for element in line.wrapped_elements() {
                        match element {
                            WrappedElement::Text {
                                is_inlay: false,
                                text,
                            } => {
                                for grapheme in text.graphemes() {
                                    self.handle_event(
                                        cx,
                                        line_index,
                                        line,
                                        byte_index,
                                        Affinity::After,
                                        origin_y,
                                        row_index,
                                        column_index,
                                    );
                                    byte_index += grapheme.len();
                                    column_index += grapheme.column_count();
                                    self.handle_event(
                                        cx,
                                        line_index,
                                        line,
                                        byte_index,
                                        Affinity::Before,
                                        origin_y,
                                        row_index,
                                        column_index,
                                    );
                                }
                            }
                            WrappedElement::Text {
                                is_inlay: true,
                                text,
                            } => {
                                column_index += text.column_count();
                            }
                            WrappedElement::Widget(widget) => {
                                column_index += widget.column_count;
                            }
                            WrappedElement::Wrap => {
                                if self.active_decoration.is_some() {
                                    self.draw_decoration(
                                        cx,
                                        line,
                                        origin_y,
                                        row_index,
                                        column_index,
                                    );
                                }
                                column_index = line.wrap_indent_column_count();
                                row_index += 1;
                            }
                        }
                    }
                    self.handle_event(
                        cx,
                        line_index,
                        line,
                        byte_index,
                        Affinity::After,
                        origin_y,
                        row_index,
                        column_index,
                    );
                    if self.active_decoration.is_some() {
                        self.draw_decoration(cx, line, origin_y, row_index, column_index);
                    }
                    line_index += 1;
                    origin_y += line.height();
                }
                BlockElement::Line {
                    is_inlay: true,
                    line,
                } => {
                    origin_y += line.height();
                }
                BlockElement::Widget(widget) => {
                    origin_y += widget.height;
                }
                BlockElement::Lens { .. } => {
                    origin_y += LENS_HEIGHT;
                }
            }
        }
    }

    fn handle_event(
        &mut self,
        cx: &mut Cx2d,
        line_index: usize,
        line: Line<'_>,
        byte_index: usize,
        affinity: Affinity,
        origin_y: f64,
        row_index: usize,
        column_index: usize,
    ) {
        let position = Position {
            line_index,
            byte_index,
        };
        if self.active_decoration.as_ref().is_some_and(|decoration| {
            decoration.decoration.end() == position && affinity == Affinity::Before
        }) {
            self.draw_decoration(cx, line, origin_y, row_index, column_index);
            self.active_decoration = None;
        }
        if self
            .decorations
            .as_slice()
            .first()
            .is_some_and(|decoration| decoration.start() == position && affinity == Affinity::After)
        {
            let decoration = *self.decorations.next().unwrap();
            if !decoration.is_empty() {
                let (start_x, _) = line.grid_to_normalized_position(row_index, column_index);
                self.active_decoration = Some(ActiveDecoration {
                    decoration,
                    start_x,
                });
            }
        }
    }

    fn draw_decoration(
        &mut self,
        cx: &mut Cx2d,
        line: Line<'_>,
        origin_y: f64,
        row_index: usize,
        column_index: usize,
    ) {
        let start_x = mem::take(&mut self.active_decoration.as_mut().unwrap().start_x);
        let (x, y) = line.grid_to_normalized_position(row_index, column_index);
        let decoration = self.active_decoration.as_ref().unwrap().decoration;
        let colors = &self.code_editor.token_colors;
        let draw = &mut self.code_editor.draw_decoration;
        draw.color = match decoration.ty {
            DecorationType::Warning => colors.warning_decoration,
            DecorationType::Error => colors.error_decoration,
            DecorationType::Hint => Vec4::default(),
        };
        (draw.fade_color, draw.strike_color) =
            (colors.unnecessary_decoration, colors.deprecated_decoration);
        draw.faded = if decoration.faded { 1.0 } else { 0.0 };
        draw.struck = if decoration.struck { 1.0 } else { 0.0 };

        self.code_editor.draw_decoration.draw_abs(
            cx,
            Rect {
                pos: DVec2 {
                    x: start_x,
                    y: origin_y + y,
                } * self.code_editor.cell_size
                    + self.code_editor.viewport_rect.pos,
                size: DVec2 {
                    x: x - start_x,
                    y: line.scale(),
                } * self.code_editor.cell_size,
            },
        );
    }
}

struct ActiveDecoration {
    decoration: Decoration,
    start_x: f64,
}

struct DrawSelectionLayer<'a> {
    code_editor: &'a mut CodeEditor,
    active_selection: Option<ActiveSelection>,
    selections: Iter<'a, Selection>,
}

impl<'a> DrawSelectionLayer<'a> {
    fn draw_selection_layer(&mut self, cx: &mut Cx2d, session: &CodeSession) {
        let total_lines = session.document().as_text().as_lines().len();
        if total_lines == 0
            || self.code_editor.line_start >= total_lines
            || self.code_editor.line_start >= self.code_editor.line_end
        {
            return;
        }
        let mut line_index = self.code_editor.line_start;
        let mut origin_y = session.layout().top_of(self.code_editor.line_start);
        for block in session
            .layout()
            .block_elements(self.code_editor.line_start, self.code_editor.line_end)
        {
            match block {
                BlockElement::Line {
                    is_inlay: false,
                    line,
                } => {
                    let mut byte_index = 0;
                    let mut row_index = 0;
                    let mut column_index = 0;
                    self.draw_selection_event(
                        cx,
                        line_index,
                        line,
                        byte_index,
                        Affinity::Before,
                        origin_y,
                        row_index,
                        column_index,
                    );
                    for element in line.wrapped_elements() {
                        match element {
                            WrappedElement::Text {
                                is_inlay: false,
                                text,
                            } => {
                                for grapheme in text.graphemes() {
                                    self.draw_selection_event(
                                        cx,
                                        line_index,
                                        line,
                                        byte_index,
                                        Affinity::After,
                                        origin_y,
                                        row_index,
                                        column_index,
                                    );
                                    byte_index += grapheme.len();
                                    column_index += grapheme.column_count();
                                    self.draw_selection_event(
                                        cx,
                                        line_index,
                                        line,
                                        byte_index,
                                        Affinity::Before,
                                        origin_y,
                                        row_index,
                                        column_index,
                                    );
                                }
                            }
                            WrappedElement::Text {
                                is_inlay: true,
                                text,
                            } => {
                                column_index += text.column_count();
                            }
                            WrappedElement::Widget(widget) => {
                                column_index += widget.column_count;
                            }
                            WrappedElement::Wrap => {
                                if self.active_selection.is_some() {
                                    self.draw_selection(
                                        cx,
                                        line,
                                        origin_y,
                                        row_index,
                                        column_index,
                                    );
                                }
                                column_index = line.wrap_indent_column_count();
                                row_index += 1;
                            }
                        }
                    }
                    self.draw_selection_event(
                        cx,
                        line_index,
                        line,
                        byte_index,
                        Affinity::After,
                        origin_y,
                        row_index,
                        column_index,
                    );
                    column_index += 1;
                    if self.active_selection.is_some() {
                        self.draw_selection(cx, line, origin_y, row_index, column_index);
                    }
                    line_index += 1;
                    origin_y += line.height();
                }
                BlockElement::Line {
                    is_inlay: true,
                    line,
                } => {
                    origin_y += line.height();
                }
                BlockElement::Widget(widget) => {
                    origin_y += widget.height;
                }
                BlockElement::Lens { .. } => {
                    origin_y += LENS_HEIGHT;
                }
            }
        }
        if self.active_selection.is_some() {
            self.code_editor.draw_selection.end(cx);
        }
    }

    fn draw_selection_event(
        &mut self,
        cx: &mut Cx2d,
        line_index: usize,
        line: Line<'_>,
        byte_index: usize,
        affinity: Affinity,
        origin_y: f64,
        row_index: usize,
        column_index: usize,
    ) {
        let position = Position {
            line_index,
            byte_index,
        };
        if self.active_selection.as_ref().is_some_and(|selection| {
            selection.selection.end() == position && selection.selection.end_affinity() == affinity
        }) {
            self.draw_selection(cx, line, origin_y, row_index, column_index);
            self.code_editor.draw_selection.end(cx);
            let selection = self.active_selection.take().unwrap().selection;
            if selection.cursor.position == position && selection.cursor.affinity == affinity {
                self.draw_cursor(cx, line, origin_y, row_index, column_index);
            }
        }
        if self.selections.as_slice().first().is_some_and(|selection| {
            selection.start() == position && selection.start_affinity() == affinity
        }) {
            let selection = *self.selections.next().unwrap();
            if selection.cursor.position == position && selection.cursor.affinity == affinity {
                self.draw_cursor_bg(cx, line, origin_y, row_index, column_index);
                self.draw_cursor(cx, line, origin_y, row_index, column_index);
            }
            if !selection.is_empty() {
                let (start_x, _) = line.grid_to_normalized_position(row_index, column_index);
                self.active_selection = Some(ActiveSelection { selection, start_x });
            }
            self.code_editor.draw_selection.begin();
        }
    }

    fn draw_selection(
        &mut self,
        cx: &mut Cx2d,
        line: Line<'_>,
        origin_y: f64,
        row_index: usize,
        column_index: usize,
    ) {
        let start_x = mem::take(&mut self.active_selection.as_mut().unwrap().start_x);
        let (x, y) = line.grid_to_normalized_position(row_index, column_index);
        if start_x == x {
            return;
        }
        self.code_editor.draw_selection.draw(
            cx,
            Rect {
                pos: DVec2 {
                    x: start_x,
                    y: origin_y + y,
                } * self.code_editor.cell_size
                    + self.code_editor.viewport_rect.pos,
                size: DVec2 {
                    x: x - start_x,
                    y: line.scale(),
                } * self.code_editor.cell_size,
            },
        );
    }

    fn draw_cursor(
        &mut self,
        cx: &mut Cx2d,
        line: Line,
        origin_y: f64,
        row_index: usize,
        column_index: usize,
    ) {
        let (x, y) = line.grid_to_normalized_position(row_index, column_index);

        self.code_editor.draw_cursor.draw_abs(
            cx,
            Rect {
                pos: DVec2 { x, y: origin_y + y } * self.code_editor.cell_size
                    + self.code_editor.viewport_rect.pos,
                size: DVec2 {
                    x: 2.0,
                    y: line.scale() * self.code_editor.cell_size.y,
                },
            },
        );
    }

    fn draw_cursor_bg(
        &mut self,
        cx: &mut Cx2d,
        line: Line<'_>,
        origin_y: f64,
        row_index: usize,
        column_index: usize,
    ) {
        let (_x, y) = line.grid_to_normalized_position(row_index, column_index);

        self.code_editor.draw_cursor_bg.draw_abs(
            cx,
            Rect {
                pos: DVec2 {
                    x: self.code_editor.unscrolled_rect.pos.x,
                    y: (origin_y + y) * self.code_editor.cell_size.y
                        + self.code_editor.viewport_rect.pos.y,
                },
                size: DVec2 {
                    x: self.code_editor.unscrolled_rect.size.x,
                    y: line.scale() * self.code_editor.cell_size.y,
                },
            },
        );
    }
}

struct ActiveSelection {
    selection: Selection,
    start_x: f64,
}

#[derive(Live, LiveHook, LiveRegister)]
struct TokenColors {
    #[live]
    unknown: Vec4,
    #[live]
    branch_keyword: Vec4,
    #[live]
    comment: Vec4,
    #[live]
    constant: Vec4,
    #[live]
    delimiter: Vec4,
    #[live]
    delimiter_highlight: Vec4,
    #[live]
    identifier: Vec4,
    #[live]
    loop_keyword: Vec4,
    #[live]
    number: Vec4,
    #[live]
    other_keyword: Vec4,
    #[live]
    function: Vec4,
    #[live]
    punctuator: Vec4,
    #[live]
    string: Vec4,
    #[live]
    typename: Vec4,
    #[live]
    whitespace: Vec4,
    #[live]
    error_decoration: Vec4,
    #[live]
    warning_decoration: Vec4,
    #[live]
    unnecessary_decoration: Vec4,
    #[live]
    deprecated_decoration: Vec4,
    #[live]
    semantic_macro: Vec4,
    #[live]
    semantic_trait: Vec4,
    #[live]
    semantic_mutable: Vec4,
    #[live]
    semantic_parameter: Vec4,
    #[live]
    semantic_field: Vec4,
    #[live]
    semantic_enum_member: Vec4,
    #[live]
    semantic_unsafe: Vec4,
}

impl TokenColors {
    /// The colour of a span the language server classified.
    fn semantic(&self, kind: crate::semantic::SemanticKind) -> Vec4 {
        use crate::semantic::SemanticKind as K;
        match kind {
            K::Macro => self.semantic_macro,
            K::Trait => self.semantic_trait,
            K::Mutable => self.semantic_mutable,
            K::Parameter => self.semantic_parameter,
            K::Field => self.semantic_field,
            K::EnumMember => self.semantic_enum_member,
            K::Unsafe => self.semantic_unsafe,
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
struct DrawDecoration {
    #[deref]
    draw_super: DrawQuad,
    #[live]
    color: Vec4,
    #[live]
    fade_color: Vec4,
    #[live]
    strike_color: Vec4,
    #[live]
    faded: f32,
    #[live]
    struck: f32,
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
struct DrawSelection {
    #[deref]
    draw_super: DrawQuad,
    #[live]
    prev_x: f32,
    #[live]
    prev_w: f32,
    #[live]
    next_x: f32,
    #[live]
    next_w: f32,
    #[rust]
    prev_prev_rect: Option<Rect>,
    #[rust]
    prev_rect: Option<Rect>,
}

impl DrawSelection {
    fn begin(&mut self) {
        debug_assert!(self.prev_rect.is_none());
    }

    fn end(&mut self, cx: &mut Cx2d) {
        self.draw_rect_internal(cx, None);
        self.prev_prev_rect = None;
        self.prev_rect = None;
    }

    fn draw(&mut self, cx: &mut Cx2d, rect: Rect) {
        self.draw_rect_internal(cx, Some(rect));
        self.prev_prev_rect = self.prev_rect;
        self.prev_rect = Some(rect);
    }

    fn draw_rect_internal(&mut self, cx: &mut Cx2d, rect: Option<Rect>) {
        if let Some(prev_rect) = self.prev_rect {
            if let Some(prev_prev_rect) = self.prev_prev_rect {
                self.prev_x = prev_prev_rect.pos.x as f32;
                self.prev_w = prev_prev_rect.size.x as f32;
            } else {
                self.prev_x = 0.0;
                self.prev_w = 0.0;
            }
            if let Some(rect) = rect {
                self.next_x = rect.pos.x as f32;
                self.next_w = rect.size.x as f32;
            } else {
                self.next_x = 0.0;
                self.next_w = 0.0;
            }
            self.draw_abs(cx, prev_rect);
        }
    }
}

#[cfg(test)]
mod reveal_tests {
    use super::Reveal;
    use makepad_widgets::NextFrame;

    fn reveal() -> Reveal {
        Reveal {
            last_y: None,
            frames: 0,
            next: NextFrame::default(),
        }
    }

    #[test]
    fn a_reveal_goes_on_while_the_line_moves_and_stops_when_it_stays() {
        // Frame 1 before the long lines wrap, frame 2 after: the line moved
        // down, so the reveal must scroll again; frame 3 finds it still.
        let mut r = reveal();
        assert!(!r.settled(100.0), "the first frame never settles");
        assert!(!r.settled(420.0), "the line moved: scroll again");
        assert!(r.settled(420.0), "the line stayed: done");
    }

    #[test]
    fn a_reveal_gives_up_after_its_frames() {
        let mut r = reveal();
        let mut y = 0.0;
        let mut frames = 0;
        while !r.settled(y) {
            y += 10.0;
            frames += 1;
            assert!(frames <= Reveal::MAX_FRAMES as usize + 1, "never gave up");
        }
    }
}
