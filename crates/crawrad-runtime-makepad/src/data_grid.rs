//! Visual grid widget rendering large datasets with virtualized rows and off-UI-thread query loading.

use crawrad_note::note;
use crawrad_runtime::grid::{
    CellRenderer, ColumnAlign, CurrentRow, EditKey, EditOutcome, FilterInput, GridColumn,
    GridState, ViewLine, checkbox_state, effective_editor, effective_filter_kind, line_for_item,
    parse_hex_color, progress_fraction,
};
use datagrid::LiveRows;
use makepad_widgets::*;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use crate::borderless::*;

    pub CrawradDataGridBase = {{CrawradDataGrid}} {}

    FilterTextInput = <TextInput> {
        width: Fill, height: 22.0,
        padding: { top: 3.0, bottom: 3.0, left: 6.0, right: 6.0 },
        align: { y: 0.5 },
        empty_text: ""
        draw_bg: {
            color: #18181f,
            color_hover: #18181f,
            color_focus: #18181f,
            color_down: #18181f,
            color_empty: #18181f,
            border_radius: 3.0,
            border_color_1: #4a4e5a,
            border_color_1_hover: #4a4e5a,
            border_color_1_focus: #7aa7ff,
            border_color_1_down: #4a4e5a,
            border_color_1_empty: #4a4e5a,
            border_color_2: #4a4e5a,
            border_color_2_hover: #4a4e5a,
            border_color_2_focus: #7aa7ff,
            border_color_2_down: #4a4e5a,
            border_color_2_empty: #4a4e5a,
        }
        draw_text: {
            color: #dfe1e5,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
    }

    FilterButton = <Button> {
        width: Fill, height: 22.0,
        draw_bg: {
            color: #18181f,
            color_hover: #26282b,
            color_down: #2e436b,
            color_focus: #18181f,
            border_color_1: #4a4e5a,
            border_color_2: #4a4e5a,
            border_color_1_hover: #4a4e5a,
            border_color_2_hover: #4a4e5a,
            border_color_1_down: #4a4e5a,
            border_color_2_down: #4a4e5a,
            border_color_1_focus: #4a4e5a,
            border_color_2_focus: #4a4e5a,
            border_radius: 3.0
        }
        draw_text: {
            color: #dfe1e5,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
        text: "All ▼"
    }

    PickButton = <Button> {
        visible: false,
        width: Fill, height: 22.0,
        draw_bg: {
            color: #1e1e2e,
            color_hover: #2e436b,
            color_down: #24324a,
            color_focus: #1e1e2e,
            border_color_1: #45475a,
            border_color_2: #45475a,
            border_color_1_hover: #7aa7ff,
            border_color_2_hover: #7aa7ff,
            border_color_1_down: #7aa7ff,
            border_color_2_down: #7aa7ff,
            border_color_1_focus: #45475a,
            border_color_2_focus: #45475a,
            border_radius: 2.0
        }
        draw_text: {
            color: #cdd6f4,
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }
        text: ""
    }

    // One row of an edLookup list: the display columns side by side, in a
    // fixed-width font so they line up. The row's highlight is the View's background — a
    // Button's `color` is a uniform, shared by every button, so it cannot mark one row.
    LookupRow = <View> {
        visible: false,
        width: Fill, height: 22.0,
        show_bg: true,
        draw_bg: { color: #1e1e2e }
        btn = <Button> {
            width: Fill, height: Fill,
            margin: 0.0,
            padding: { left: 6.0, right: 6.0, top: 0.0, bottom: 0.0 },
            align: { x: 0.0, y: 0.5 },
            draw_bg: {
                color: #0000,
                color_hover: #ffffff14,
                color_down: #ffffff20,
                color_focus: #0000,
                border_color_1: #0000,
                border_color_2: #0000,
                border_color_1_hover: #0000,
                border_color_2_hover: #0000,
                border_color_1_down: #0000,
                border_color_2_down: #0000,
                border_color_1_focus: #0000,
                border_color_2_focus: #0000,
                border_radius: 0.0
            }
            draw_text: {
                color: #cdd6f4,
                text_style: <THEME_FONT_CODE> { font_size: 9.0 }
            }
            text: ""
        }
    }

    GridCell = <View> {
        draw_bg: {
            instance line_color: #393b40
            fn pixel(self) -> vec4 {
                let px = self.pos * self.rect_size;
                if px.x > self.rect_size.x - 1.0 || px.y > self.rect_size.y - 1.0 {
                    return mix(self.color, vec4(self.line_color.xyz, 1.0), self.line_color.w);
                }
                return self.color;
            }
        }
    }

    // A filled box with a 1 px border (renderers: check box, bar track, swatch).
    BoxedView = <View> {
        show_bg: true,
        draw_bg: {
            instance border_color: #6e7290
            fn pixel(self) -> vec4 {
                let px = self.pos * self.rect_size;
                if px.x < 1.0 || px.y < 1.0 || px.x > self.rect_size.x - 1.0 || px.y > self.rect_size.y - 1.0 {
                    return self.border_color;
                }
                return self.color;
            }
        }
    }

    GridDataCell = <GridCell> {
        visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 },
        // The background is ALWAYS on (transparent when nothing colours the cell):
        // turning show_bg on after the first draw puts it over the text.
        // No cell lines in the data area, as before: the line is the header's.
        show_bg: true, draw_bg: { color: #0000, line_color: #0000 },
        flow: Overlay,

        lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" }
        // The same label in the faces a style rule / OnDrawCell can ask for: the
        // font family cannot change through apply_over, so the cell writes in one of
        // these and leaves the others empty (a Label has no `visible`).
        lbl_b = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" }
        lbl_i = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_ITALIC> { font_size: 9.0 } }, text: "" }
        lbl_bi = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD_ITALIC> { font_size: 9.0 } }, text: "" }

        cb_box = <BoxedView> {
            visible: false,
            width: 14.0, height: 14.0,
            flow: Overlay,
            align: { x: 0.5, y: 0.5 },
            draw_bg: { color: #181825 }
            cb_check = <View> {
                visible: false,
                width: 8.0, height: 8.0,
                align: { x: 0.5, y: 0.5 },
                show_bg: true,
                draw_bg: { color: #89b4fa }
            }
        }

        pb_track = <BoxedView> {
            visible: false,
            width: Fill, height: 16.0,
            flow: Overlay,
            align: { x: 0.5, y: 0.5 },
            draw_bg: { color: #181825 }
            // The fill grows from the LEFT edge (the track's centre align is for the label).
            pb_fill_row = <View> {
                width: Fill, height: Fill,
                align: { x: 0.0, y: 0.5 },
                pb_fill = <View> {
                    width: 0.0, height: Fill,
                    show_bg: true,
                    draw_bg: { color: #89b4fa }
                }
            }
            pb_lbl = <Label> {
                width: Fit, height: Fit,
                align: { x: 0.5, y: 0.5 },
                draw_text: {
                    color: #cdd6f4,
                    text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                }
                text: ""
            }
        }

        swatch_row = <View> {
            visible: false,
            width: Fill, height: Fill,
            flow: Right,
            align: { y: 0.5 },
            swatch_box = <BoxedView> {
                width: 12.0, height: 12.0,
                margin: { right: 6.0 },
                draw_bg: { color: #0000 }
            }
            swatch_lbl = <Label> {
                width: Fit, height: Fit,
                draw_text: {
                    color: #dfe1e5,
                    text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                }
                text: ""
            }
        }

        // Link colour + hand cursor. No underline here: a Fit view inside this
        // Overlay cell takes the whole cell, so a drawn line lands on the row edge.
        link_view = <View> {
            visible: false,
            width: Fit, height: Fit,
            link_lbl = <Label> {
                width: Fit, height: Fit,
                draw_text: {
                    color: #89b4fa,
                    text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                }
                text: ""
            }
        }
    }

    GroupChip = <View> {
        visible: false,
        width: Fit, height: 26.0,
        flow: Right,
        align: { y: 0.5 },
        padding: { left: 8.0, right: 4.0 },
        show_bg: true,
        draw_bg: { color: #2f4a7a }

        lbl = <Label> {
            width: Fit, height: Fit,
            draw_text: {
                color: #e6eeff,
                text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
            }
            text: ""
        }
        btn_close = <CrawradBorderlessButton> {
            width: 20.0, height: 20.0, margin: { left: 4.0 },
            draw_bg: {
                color: #0000, color_hover: #385896, color_down: #2e436b, color_focus: #0000,
            }
            draw_text: {
                color: #e6eeff, color_hover: #ffffff, color_focus: #e6eeff,
                text_style: <THEME_FONT_BOLD> { font_size: 11.0 }
            }
            text: "×"
        }
    }

    // One template per row state (even, odd, selected): the row colour is
    // NEVER changed with `apply_over` per frame — inside a PortalList that
    // breaks the draw order and the backgrounds of later rows cover the texts.
    // The gutter cell of the header, filter and summary bars: no mark, just the width.
    BlankGutter = <View> {
        visible: false, width: 22.0, height: Fill,
        show_bg: true, draw_bg: { color: #2b2d30 }
    }

    GroupHeaderRow = <View> {
        visible: false,
        width: Fill, height: 30.0,
        flow: Right,
        align: { y: 0.5 },
        padding: { left: 8.0, right: 8.0 },
        show_bg: true,
        draw_bg: { color: #24324a }
        lbl_title = <Label> { width: Fit, height: Fit, draw_text: { color: #cfdcf5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" }
        <View> { width: 12.0 }
        lbl_count = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" }
        <View> { width: 12.0 }
        lbl_totals = <Label> { width: Fit, height: Fit, draw_text: { color: #cfdcf5, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "" }
    }

    GridItem = <View> {
        width: Fill, height: Fit,
        flow: Down,

        h0 = <GroupHeaderRow> {}
        h1 = <GroupHeaderRow> {}
        h2 = <GroupHeaderRow> {}
        h3 = <GroupHeaderRow> {}

        row = <View> {
            width: Fill, height: 30.0, margin: 0.0,
            flow: Overlay,
            btn = <CrawradBorderlessButton> {
                width: Fill, height: Fill, margin: 0.0,
                draw_bg: {
                    color: #1e1f22, color_hover: #26282b, color_down: #2e436b, color_focus: #1e1f22,
                }
                draw_text: { text_style: { font_size: 0.0 } }
                text: ""
            }
            // The gutter and the cells sit side by side: showing the gutter pushes every
            // cell right on every row alike (no per-row margin to forget).
            content = <View> {
                width: Fill, height: Fill,
                flow: Right,
            gutter = <View> {
                visible: false, width: 22.0, height: Fill,
                align: { x: 0.5, y: 0.5 },
                show_bg: true, draw_bg: { color: #212225 }
                // The icon font has no blank glyph: an unmarked row hides the box instead.
                mark_box = <View> { width: Fit, height: Fit,
                mark = <Label> {
                    width: Fit, height: Fit,
                    draw_text: {
                        color: #dfe1e5,
                        text_style: <THEME_FONT_ICONS> { font_size: 8.0 }
                    }
                    text: ""
                } }
            }
            cols = <View> {
                width: Fill, height: Fill,
                flow: Right,
                c0 = <GridDataCell> {}
                c1 = <GridDataCell> {}
                c2 = <GridDataCell> {}
                c3 = <GridDataCell> {}
                c4 = <GridDataCell> {}
                c5 = <GridDataCell> {}
                c6 = <GridDataCell> {}
                c7 = <GridDataCell> {}
                c8 = <GridDataCell> {}
                c9 = <GridDataCell> {}
                c10 = <GridDataCell> {}
                c11 = <GridDataCell> {}
                c12 = <GridDataCell> {}
                c13 = <GridDataCell> {}
                c14 = <GridDataCell> {}
                c15 = <GridDataCell> {}
            }
            }
        }
    }

    pub CrawradDataGrid = <CrawradDataGridBase> {
        width: Fill, height: Fill,
        flow: Overlay,
        // The cell editor's list may run past the grid's edges, like a combo's popup.
        clip_x: false, clip_y: false,
        show_bg: true,
        draw_bg: { color: #1e1f22 }
        draw_measure: {
            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
        }

        main = <View> {
            width: Fill, height: Fill,
            flow: Down,
            group_bar = <View> {
                visible: false,
                width: Fill, height: 40.0, margin: 0.0,
                flow: Right,
                align: { y: 0.5 },
                padding: { left: 12.0, right: 12.0 },
                spacing: 8.0,
                show_bg: true,
                draw_bg: {
                    instance line_color: #393b40
                    color: #26282b
                    fn pixel(self) -> vec4 {
                        let px = self.pos * self.rect_size;
                        if px.y > self.rect_size.y - 1.0 {
                            return self.line_color;
                        }
                        return self.color;
                    }
                }

                lbl_title = <Label> {
                    width: Fit, height: Fit,
                    draw_text: {
                        color: #9da0a8,
                        text_style: <THEME_FONT_BOLD> { font_size: 9.0 }
                    }
                    text: "Group by:"
                }

                chip0 = <GroupChip> {}
                chip1 = <GroupChip> {}
                chip2 = <GroupChip> {}
                chip3 = <GroupChip> {}
                chip4 = <GroupChip> {}
                chip5 = <GroupChip> {}
                chip6 = <GroupChip> {}
                chip7 = <GroupChip> {}

                lbl_hint = <Label> {
                    width: Fit, height: Fit,
                    draw_text: {
                        color: #9da0a8,
                        text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                    }
                    text: "drag a column header here to group"
                }
            }

            outer_frame = <View> {
                width: Fill, height: Fill,
                flow: Down,
                padding: 1.0,
                show_bg: true,
                draw_bg: {
                    instance border_color: #393b40
                    color: #0000
                    fn pixel(self) -> vec4 {
                        let px = self.pos * self.rect_size;
                        if px.x < 1.0 || px.y < 1.0 || px.x > self.rect_size.x - 1.0 || px.y > self.rect_size.y - 1.0 {
                            return self.border_color;
                        }
                        return self.color;
                    }
                }

                header = <View> {
                    visible: true,
                    width: Fill, height: 34.0, margin: 0.0,
                    flow: Right,
                    show_bg: true,
                    draw_bg: { color: #2b2d30 }

                    gutter = <BlankGutter> {}

                c0 = <GridCell> { visible: false, cursor: Hand, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c1 = <GridCell> { visible: false, cursor: Hand, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c2 = <GridCell> { visible: false, cursor: Hand, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c3 = <GridCell> { visible: false, cursor: Hand, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c4 = <GridCell> { visible: false, cursor: Hand, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c5 = <GridCell> { visible: false, cursor: Hand, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c6 = <GridCell> { visible: false, cursor: Hand, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c7 = <GridCell> { visible: false, cursor: Hand, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c8 = <GridCell> { visible: false, cursor: Hand, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c9 = <GridCell> { visible: false, cursor: Hand, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c10 = <GridCell> { visible: false, cursor: Hand, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c11 = <GridCell> { visible: false, cursor: Hand, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c12 = <GridCell> { visible: false, cursor: Hand, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c13 = <GridCell> { visible: false, cursor: Hand, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c14 = <GridCell> { visible: false, cursor: Hand, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c15 = <GridCell> { visible: false, cursor: Hand, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
            }

            filter_bar = <View> {
                visible: false,
                width: Fill, height: 30.0, margin: 0.0,
                flow: Right,
                show_bg: true,
                draw_bg: { color: #2b2d30 }

                gutter = <BlankGutter> {}

                c0 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 4.0, right: 4.0 }, align: { y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, flow: Overlay, txt_box = <View> { visible: false, width: Fill, height: Fill, align: { y: 0.5 }, txt = <FilterTextInput> { empty_text: "" } }, range_box = <View> { visible: false, width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, spacing: 2.0, lo = <FilterTextInput> { width: Fill, empty_text: "min" }, dash = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "–" }, hi = <FilterTextInput> { width: Fill, empty_text: "max" } }, pick = <FilterButton> { visible: false, text: "All ▼" } }
                c1 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 4.0, right: 4.0 }, align: { y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, flow: Overlay, txt_box = <View> { visible: false, width: Fill, height: Fill, align: { y: 0.5 }, txt = <FilterTextInput> { empty_text: "" } }, range_box = <View> { visible: false, width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, spacing: 2.0, lo = <FilterTextInput> { width: Fill, empty_text: "min" }, dash = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "–" }, hi = <FilterTextInput> { width: Fill, empty_text: "max" } }, pick = <FilterButton> { visible: false, text: "All ▼" } }
                c2 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 4.0, right: 4.0 }, align: { y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, flow: Overlay, txt_box = <View> { visible: false, width: Fill, height: Fill, align: { y: 0.5 }, txt = <FilterTextInput> { empty_text: "" } }, range_box = <View> { visible: false, width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, spacing: 2.0, lo = <FilterTextInput> { width: Fill, empty_text: "min" }, dash = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "–" }, hi = <FilterTextInput> { width: Fill, empty_text: "max" } }, pick = <FilterButton> { visible: false, text: "All ▼" } }
                c3 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 4.0, right: 4.0 }, align: { y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, flow: Overlay, txt_box = <View> { visible: false, width: Fill, height: Fill, align: { y: 0.5 }, txt = <FilterTextInput> { empty_text: "" } }, range_box = <View> { visible: false, width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, spacing: 2.0, lo = <FilterTextInput> { width: Fill, empty_text: "min" }, dash = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "–" }, hi = <FilterTextInput> { width: Fill, empty_text: "max" } }, pick = <FilterButton> { visible: false, text: "All ▼" } }
                c4 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 4.0, right: 4.0 }, align: { y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, flow: Overlay, txt_box = <View> { visible: false, width: Fill, height: Fill, align: { y: 0.5 }, txt = <FilterTextInput> { empty_text: "" } }, range_box = <View> { visible: false, width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, spacing: 2.0, lo = <FilterTextInput> { width: Fill, empty_text: "min" }, dash = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "–" }, hi = <FilterTextInput> { width: Fill, empty_text: "max" } }, pick = <FilterButton> { visible: false, text: "All ▼" } }
                c5 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 4.0, right: 4.0 }, align: { y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, flow: Overlay, txt_box = <View> { visible: false, width: Fill, height: Fill, align: { y: 0.5 }, txt = <FilterTextInput> { empty_text: "" } }, range_box = <View> { visible: false, width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, spacing: 2.0, lo = <FilterTextInput> { width: Fill, empty_text: "min" }, dash = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "–" }, hi = <FilterTextInput> { width: Fill, empty_text: "max" } }, pick = <FilterButton> { visible: false, text: "All ▼" } }
                c6 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 4.0, right: 4.0 }, align: { y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, flow: Overlay, txt_box = <View> { visible: false, width: Fill, height: Fill, align: { y: 0.5 }, txt = <FilterTextInput> { empty_text: "" } }, range_box = <View> { visible: false, width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, spacing: 2.0, lo = <FilterTextInput> { width: Fill, empty_text: "min" }, dash = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "–" }, hi = <FilterTextInput> { width: Fill, empty_text: "max" } }, pick = <FilterButton> { visible: false, text: "All ▼" } }
                c7 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 4.0, right: 4.0 }, align: { y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, flow: Overlay, txt_box = <View> { visible: false, width: Fill, height: Fill, align: { y: 0.5 }, txt = <FilterTextInput> { empty_text: "" } }, range_box = <View> { visible: false, width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, spacing: 2.0, lo = <FilterTextInput> { width: Fill, empty_text: "min" }, dash = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "–" }, hi = <FilterTextInput> { width: Fill, empty_text: "max" } }, pick = <FilterButton> { visible: false, text: "All ▼" } }
                c8 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 4.0, right: 4.0 }, align: { y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, flow: Overlay, txt_box = <View> { visible: false, width: Fill, height: Fill, align: { y: 0.5 }, txt = <FilterTextInput> { empty_text: "" } }, range_box = <View> { visible: false, width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, spacing: 2.0, lo = <FilterTextInput> { width: Fill, empty_text: "min" }, dash = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "–" }, hi = <FilterTextInput> { width: Fill, empty_text: "max" } }, pick = <FilterButton> { visible: false, text: "All ▼" } }
                c9 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 4.0, right: 4.0 }, align: { y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, flow: Overlay, txt_box = <View> { visible: false, width: Fill, height: Fill, align: { y: 0.5 }, txt = <FilterTextInput> { empty_text: "" } }, range_box = <View> { visible: false, width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, spacing: 2.0, lo = <FilterTextInput> { width: Fill, empty_text: "min" }, dash = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "–" }, hi = <FilterTextInput> { width: Fill, empty_text: "max" } }, pick = <FilterButton> { visible: false, text: "All ▼" } }
                c10 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 4.0, right: 4.0 }, align: { y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, flow: Overlay, txt_box = <View> { visible: false, width: Fill, height: Fill, align: { y: 0.5 }, txt = <FilterTextInput> { empty_text: "" } }, range_box = <View> { visible: false, width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, spacing: 2.0, lo = <FilterTextInput> { width: Fill, empty_text: "min" }, dash = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "–" }, hi = <FilterTextInput> { width: Fill, empty_text: "max" } }, pick = <FilterButton> { visible: false, text: "All ▼" } }
                c11 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 4.0, right: 4.0 }, align: { y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, flow: Overlay, txt_box = <View> { visible: false, width: Fill, height: Fill, align: { y: 0.5 }, txt = <FilterTextInput> { empty_text: "" } }, range_box = <View> { visible: false, width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, spacing: 2.0, lo = <FilterTextInput> { width: Fill, empty_text: "min" }, dash = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "–" }, hi = <FilterTextInput> { width: Fill, empty_text: "max" } }, pick = <FilterButton> { visible: false, text: "All ▼" } }
                c12 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 4.0, right: 4.0 }, align: { y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, flow: Overlay, txt_box = <View> { visible: false, width: Fill, height: Fill, align: { y: 0.5 }, txt = <FilterTextInput> { empty_text: "" } }, range_box = <View> { visible: false, width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, spacing: 2.0, lo = <FilterTextInput> { width: Fill, empty_text: "min" }, dash = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "–" }, hi = <FilterTextInput> { width: Fill, empty_text: "max" } }, pick = <FilterButton> { visible: false, text: "All ▼" } }
                c13 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 4.0, right: 4.0 }, align: { y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, flow: Overlay, txt_box = <View> { visible: false, width: Fill, height: Fill, align: { y: 0.5 }, txt = <FilterTextInput> { empty_text: "" } }, range_box = <View> { visible: false, width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, spacing: 2.0, lo = <FilterTextInput> { width: Fill, empty_text: "min" }, dash = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "–" }, hi = <FilterTextInput> { width: Fill, empty_text: "max" } }, pick = <FilterButton> { visible: false, text: "All ▼" } }
                c14 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 4.0, right: 4.0 }, align: { y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, flow: Overlay, txt_box = <View> { visible: false, width: Fill, height: Fill, align: { y: 0.5 }, txt = <FilterTextInput> { empty_text: "" } }, range_box = <View> { visible: false, width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, spacing: 2.0, lo = <FilterTextInput> { width: Fill, empty_text: "min" }, dash = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "–" }, hi = <FilterTextInput> { width: Fill, empty_text: "max" } }, pick = <FilterButton> { visible: false, text: "All ▼" } }
                c15 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 4.0, right: 4.0 }, align: { y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, flow: Overlay, txt_box = <View> { visible: false, width: Fill, height: Fill, align: { y: 0.5 }, txt = <FilterTextInput> { empty_text: "" } }, range_box = <View> { visible: false, width: Fill, height: Fill, flow: Right, align: { y: 0.5 }, spacing: 2.0, lo = <FilterTextInput> { width: Fill, empty_text: "min" }, dash = <Label> { width: Fit, height: Fit, draw_text: { color: #9da0a8, text_style: <THEME_FONT_REGULAR> { font_size: 9.0 } }, text: "–" }, hi = <FilterTextInput> { width: Fill, empty_text: "max" } }, pick = <FilterButton> { visible: false, text: "All ▼" } }
            }

            body = <View> {
                width: Fill, height: Fill,
                flow: Overlay,

                list = <PortalList> {
                    width: Fill, height: Fill,
                    item = <GridItem> {}
                    item_odd = <GridItem> {
                        row = { btn = { draw_bg: { color: #212225, color_focus: #212225 } } }
                    }
                    item_selected = <GridItem> {
                        row = { btn = { draw_bg: { color: #2e436b, color_hover: #2e436b, color_focus: #2e436b } } }
                    }
                }

                edit_error_frame = <BoxedView> {
                    visible: false,
                    width: 100.0, height: 30.0,
                    draw_bg: {
                        border_color: #e06c75,
                        color: #0000
                    }
                }

                edit_error = <BoxedView> {
                    visible: false,
                    width: Fit, height: Fit,
                    padding: { left: 6.0, right: 6.0, top: 4.0, bottom: 4.0 },
                    draw_bg: {
                        border_color: #e06c75,
                        color: #5a1d1d
                    }
                    lbl_msg = <Label> {
                        width: Fit, height: Fit,
                        draw_text: {
                            color: #ffd7d7,
                            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                        }
                        text: ""
                    }
                }

                // The old rows dim under ONE veil in the grid's own background
                // colour (alpha = 1 - rows_alpha), not by recolouring every cell:
                // one change per frame, and the rows keep the colours of the DSL.
                veil = <View> {
                    visible: false,
                    width: Fill, height: Fill,
                    show_bg: true,
                    draw_bg: { color: #1e1f2200 }
                }

                loading_overlay = <View> {
                    visible: false,
                    width: Fill, height: Fill,
                    flow: Overlay,
                    align: { x: 0.5, y: 0.5 },

                    // As approved: the spinner on the left, the two lines on the right;
                    // the card is as big as its text (Fit), never a fixed box.
                    card = <View> {
                        width: Fit, height: Fit,
                        flow: Right,
                        align: { y: 0.5 },
                        padding: { left: 18.0, right: 18.0, top: 12.0, bottom: 12.0 },
                        spacing: 12.0,
                        show_bg: true,
                        draw_bg: {
                            instance border_color: #4e5157
                            color: #2b2d30
                            fn pixel(self) -> vec4 {
                                let px = self.pos * self.rect_size;
                                if px.x < 1.0 || px.y < 1.0 || px.x > self.rect_size.x - 1.0 || px.y > self.rect_size.y - 1.0 {
                                    return self.border_color;
                                }
                                return self.color;
                            }
                        }

                        spinner = <View> {
                            width: 20.0, height: 20.0,
                            show_bg: true,
                            draw_bg: {
                                instance spin: 0.0
                                fn pixel(self) -> vec4 {
                                    let p = self.pos - vec2(0.5, 0.5);
                                    let r = length(p);
                                    let ring = smoothstep(0.50, 0.46, r) * smoothstep(0.30, 0.34, r);
                                    let angle = atan(p.y, p.x) + self.spin * 6.2831853;
                                    let arc = fract(angle / 6.2831853);
                                    let arc_mask = smoothstep(0.15, 0.95, arc);
                                    let col = #7aa7ff;
                                    return vec4(col.xyz * ring * arc_mask, ring * arc_mask);
                                }
                            }
                        }

                        texts = <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            spacing: 2.0,

                            lbl_title = <Label> {
                                width: Fit, height: Fit,
                                draw_text: {
                                    color: #dfe1e5,
                                    text_style: <THEME_FONT_REGULAR> { font_size: 10.0 }
                                }
                                text: ""
                            }

                            lbl_detail = <Label> {
                                width: Fit, height: Fit,
                                draw_text: {
                                    color: #9da0a8,
                                    text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                                }
                                text: ""
                            }
                        }
                    }
                }

                filter_popup = <BoxedView> {
                    visible: false,
                    width: 160.0, height: 200.0,
                    flow: Overlay,
                    draw_bg: { border_color: #45475a, color: #1e1e2e }

                    lbl_loading = <Label> {
                        width: Fill, height: Fit,
                        padding: 8.0,
                        draw_text: {
                            color: #cdd6f4,
                            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                        }
                        text: ""
                    }

                    distinct_list = <PortalList> {
                        width: Fill, height: Fill,
                        row = <CheckBox> {
                            width: Fill, height: 22.0, margin: { left: 6.0 },
                            align: { x: 0.0, y: 0.5 },
                            draw_text: {
                                color: #cdd6f4,
                                text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                            }
                        }
                    }
                }
            }

            summary_bar = <View> {
                visible: false,
                width: Fill, height: 30.0, margin: 0.0,
                flow: Right,
                show_bg: true,
                draw_bg: { color: #2b2d30 }

                gutter = <BlankGutter> {}

                c0 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c1 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c2 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c3 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c4 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c5 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c6 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c7 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c8 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c9 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c10 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c11 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c12 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c13 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c14 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
                c15 = <GridCell> { visible: false, width: 120.0, height: Fill, padding: { left: 8.0, right: 8.0 }, align: { x: 0.0, y: 0.5 }, show_bg: true, draw_bg: { color: #2b2d30, line_color: #393b40 }, lbl = <Label> { width: Fit, height: Fit, draw_text: { color: #dfe1e5, text_style: <THEME_FONT_BOLD> { font_size: 9.0 } }, text: "" } }
            }
        }

            footer = <View> {
                width: Fill, height: 28.0, margin: 0.0,
                flow: Right,
                align: { y: 0.5 },
                padding: { left: 12.0, right: 12.0 },
                spacing: 16.0,
                show_bg: true,
                draw_bg: {
                    instance line_color: #393b40
                    color: #2b2d30
                    fn pixel(self) -> vec4 {
                        let px = self.pos * self.rect_size;
                        if px.y < 1.0 {
                            return self.line_color;
                        }
                        return self.color;
                    }
                }

                lbl_position = <Label> {
                    width: Fit, height: Fit,
                    draw_text: {
                        color: #dfe1e5,
                        text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                    }
                    text: ""
                }

                lbl_state = <Label> {
                    width: Fit, height: Fit,
                    draw_text: {
                        color: #7aa7ff,
                        text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                    }
                    text: ""
                }

                spacer = <View> { width: Fill }

                lbl_order = <Label> {
                    width: Fit, height: Fit,
                    draw_text: {
                        color: #9da0a8,
                        text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                    }
                    text: ""
                }

                lbl_timing = <Label> {
                    width: Fit, height: Fit,
                    draw_text: {
                        color: #9da0a8,
                        text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                    }
                    text: ""
                }

            }

            // A source error must be SEEN, and WHOLE: without it a refused query
            // only looks like rows that did not change, and cut off at the
            // footer's edge it hid the very path the user had to check. Its own
            // line, the full width, wrapping; hidden while there is no error.
            error_line = <View> {
                visible: false,
                width: Fill, height: Fit,
                padding: { left: 12.0, right: 12.0, top: 2.0, bottom: 6.0 },
                show_bg: true,
                draw_bg: { color: #2b2d30 }
                lbl_error = <Label> {
                    width: Fill, height: Fit,
                    draw_text: {
                        wrap: Word,
                        color: #f2a65a,
                        text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                    }
                    text: ""
                }
            }
        }

        // Above everything the grid draws, unclipped: the cell editor and its list.
        popup_layer = <View> {
            width: Fill, height: Fill,
            flow: Overlay,
            clip_x: false, clip_y: false,
            // Overlay: the edLookup list sits under the text box, or above it when the
            // window has no room below — each part is placed by its own margin.
            cell_editor = <View> {
                visible: false,
                width: 100.0, height: 30.0,
                flow: Overlay,
                clip_x: false, clip_y: false,
                // A TextInput has no `visible`: its box hides it under an edList.
                // Opaque, so the cell's own text never shows around the editor.
                // ONE square frame around the text and its button: they read as one control.
                ed_box = <BoxedView> {
                    width: Fill, height: 30.0,
                    flow: Right,
                    padding: 1.0,
                    draw_bg: { color: #18181f, border_color: #7aa7ff }
                    ed = <TextInput> {
                        width: Fill, height: Fill,
                        align: { y: 0.5 },
                        empty_text: ""
                        draw_bg: {
                            color: #18181f,
                            color_hover: #18181f,
                            color_focus: #18181f,
                            color_down: #18181f,
                            color_empty: #18181f,
                            border_radius: 0.0,
                            border_color_1: #0000,
                            border_color_1_hover: #0000,
                            border_color_1_focus: #0000,
                            border_color_1_down: #0000,
                            border_color_1_empty: #0000,
                            border_color_2: #0000,
                            border_color_2_hover: #0000,
                            border_color_2_focus: #0000,
                            border_color_2_down: #0000,
                            border_color_2_empty: #0000,
                        }
                        draw_text: {
                            color: #dfe1e5,
                            text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }
                        }
                    }
                    // The thin line between the text and the button, inside the one frame.
                    ed_sep = <View> {
                        visible: false,
                        width: 1.0, height: Fill,
                        show_bg: true,
                        draw_bg: { color: #3d5a8a }
                    }
                    btn_dots = <Button> {
                        visible: false,
                        width: 26.0, height: Fill,
                        draw_bg: {
                            color: #18181f,
                            color_hover: #33353a,
                            color_down: #383a40,
                            color_focus: #18181f,
                            border_color_1: #0000,
                            border_color_2: #0000,
                            border_radius: 0.0
                        }
                        draw_text: {
                            color: #dfe1e5,
                            text_style: <THEME_FONT_REGULAR> { font_size: 10.0 }
                        }
                        text: "…"
                    }
                    btn_arrow = <Button> {
                        visible: false,
                        width: 20.0, height: Fill,
                        draw_bg: {
                            color: #18181f,
                            color_hover: #26282b,
                            color_down: #2e436b,
                            color_focus: #18181f,
                            border_color_1: #0000,
                            border_color_2: #0000,
                            border_radius: 0.0
                        }
                        draw_text: {
                            color: #9da0a8,
                            text_style: <THEME_FONT_REGULAR> { font_size: 8.0 }
                        }
                        text: "▼"
                    }
                }
                pick_box = <View> {
                    width: Fill, height: Fit,
                    flow: Down,
                    pk0 = <PickButton> {}
                    pk1 = <PickButton> {}
                    pk2 = <PickButton> {}
                    pk3 = <PickButton> {}
                    pk4 = <PickButton> {}
                    pk5 = <PickButton> {}
                    pk6 = <PickButton> {}
                    pk7 = <PickButton> {}
                    pk8 = <PickButton> {}
                    pk9 = <PickButton> {}
                    pk10 = <PickButton> {}
                    pk11 = <PickButton> {}
                }
                // The edLookup list: header, rows, footer on a solid box. A Label
                // has no `visible`: the box hides it all.
                lookup_box = <BoxedView> { visible: false, width: Fill, height: Fit, flow: Down,
                    padding: 1.0,
                    draw_bg: { color: #1e1e2e, border_color: #4a5470 }
                    lookup_header = <Label> {
                        // 16 + the 4 of padding = LOOKUP_LINE_HEIGHT: padding adds to a fixed height.
                        width: Fill, height: 16.0,
                        padding: { left: 6.0, right: 6.0, top: 4.0, bottom: 0.0 },
                        draw_text: {
                            color: #9da0a8,
                            text_style: <THEME_FONT_CODE> { font_size: 8.0 }
                        }
                        text: ""
                    }
                    // The rows, and the scroll bar beside them when there are more than fit.
                    lk_body = <View> {
                        width: Fill, height: Fit,
                        flow: Right,
                        lk_rows = <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            lk0 = <LookupRow> {}
                            lk1 = <LookupRow> {}
                            lk2 = <LookupRow> {}
                            lk3 = <LookupRow> {}
                            lk4 = <LookupRow> {}
                            lk5 = <LookupRow> {}
                            lk6 = <LookupRow> {}
                            lk7 = <LookupRow> {}
                            lk8 = <LookupRow> {}
                            lk9 = <LookupRow> {}
                            lk10 = <LookupRow> {}
                            lk11 = <LookupRow> {}
                            lk12 = <LookupRow> {}
                            lk13 = <LookupRow> {}
                            lk14 = <LookupRow> {}
                            lk15 = <LookupRow> {}
                        }
                        lk_track = <View> {
                            visible: false,
                            width: 8.0, height: 0.0,
                            flow: Down,
                            padding: { left: 2.0, right: 2.0 },
                            show_bg: true,
                            draw_bg: { color: #262838 }
                            lk_thumb = <View> {
                                width: Fill, height: 20.0,
                                show_bg: true,
                                draw_bg: { color: #5a6280 }
                            }
                        }
                    }
                    lbl_lookup_footer = <Label> {
                        // 16 + the 4 of padding = LOOKUP_LINE_HEIGHT: padding adds to a fixed height.
                        width: Fill, height: 16.0,
                        padding: { left: 6.0, right: 6.0, top: 4.0, bottom: 0.0 },
                        draw_text: {
                            color: #9da0a8,
                            text_style: <THEME_FONT_REGULAR> { font_size: 8.0 }
                        }
                        text: ""
                    }
                }
            }
        }
    }
}

#[note("data-grid-max-columns")]
pub const MAX_COLUMNS: usize = 16;

const CELL_IDS: [LiveId; MAX_COLUMNS] = [
    live_id!(c0),
    live_id!(c1),
    live_id!(c2),
    live_id!(c3),
    live_id!(c4),
    live_id!(c5),
    live_id!(c6),
    live_id!(c7),
    live_id!(c8),
    live_id!(c9),
    live_id!(c10),
    live_id!(c11),
    live_id!(c12),
    live_id!(c13),
    live_id!(c14),
    live_id!(c15),
];

pub(crate) fn cell_id(idx: usize) -> LiveId {
    CELL_IDS[idx.min(MAX_COLUMNS - 1)]
}

#[note("data-grid-column-widths")]
pub fn column_widths(available: f64, columns: &[GridColumn]) -> Vec<f64> {
    let asked: Vec<Option<f64>> = columns.iter().map(|c| c.width.map(f64::from)).collect();
    let bounds: Vec<(f64, Option<f64>)> = columns
        .iter()
        .map(|c| {
            let min = (c.min_width as f64).max(MIN_COLUMN_WIDTH);
            (min, c.max_width.map(f64::from))
        })
        .collect();
    crawrad_runtime::grid::share_widths_clamped(available, &asked, &bounds)
}

/// The Makepad walk of one cell: a free last column takes the rest (`Fill`).
fn apply_cell_walk(
    cx: &mut Cx,
    cell: &ViewRef,
    width: f64,
    fill: bool,
    align: ColumnAlign,
    valign: Option<crawrad_runtime::grid::ColumnVAlign>,
) {
    let align_x: f64 = match align {
        ColumnAlign::Left => 0.0,
        ColumnAlign::Center => 0.5,
        ColumnAlign::Right => 1.0,
    };
    let align_y: f64 = match valign {
        Some(crawrad_runtime::grid::ColumnVAlign::Top) => 0.0,
        Some(crawrad_runtime::grid::ColumnVAlign::Bottom) => 1.0,
        _ => 0.5,
    };
    if fill {
        cell.apply_over(
            cx,
            live! { width: Fill, align: { x: (align_x), y: (align_y) } },
        );
    } else {
        cell.apply_over(
            cx,
            live! { width: (width), align: { x: (align_x), y: (align_y) } },
        );
    }
}

#[note("data-grid-min-column-width")]
pub const MIN_COLUMN_WIDTH: f64 = 60.0;

/// Rows asked of the source at a time, from the top visible one.
const PAGE_ROWS: usize = 200;

/// The gutter marks, in the icon font (Font Awesome solid): neither IBM Plex nor Noto
/// Sans has a pencil.
const MAKEPAD_EDITED_ROW: &str = "\u{f303}";
const MAKEPAD_NEW_ROW: &str = "\u{f069}";

const CELL_BG_ENABLED: Vec4 = Vec4 {
    x: 0.169,
    y: 0.176,
    z: 0.188,
    w: 1.0,
};
const CELL_BG_DIMMED: Vec4 = Vec4 {
    x: 0.125,
    y: 0.13,
    z: 0.14,
    w: 1.0,
};

/// Synchronizes one bar row (filter bar or summary bar) with column widths and visibility.
fn sync_bar_cells(
    cx: &mut Cx,
    bar: &ViewRef,
    num_cols: usize,
    ordered_cols: &[GridColumn],
    widths: &[f64],
    fills: impl Fn(usize) -> bool,
    mut update_cell: impl FnMut(&mut Cx, usize, &ViewRef, &GridColumn),
) {
    for idx in 0..MAX_COLUMNS {
        let cell = bar.view(&[cell_id(idx)]);
        if idx < num_cols
            && let Some(col_info) = ordered_cols.get(idx)
        {
            cell.set_visible(cx, true);
            let w = widths.get(idx).copied().unwrap_or(MIN_COLUMN_WIDTH);
            apply_cell_walk(cx, &cell, w, fills(idx), col_info.align, None);
            update_cell(cx, idx, &cell, col_info);
        } else {
            cell.set_visible(cx, false);
        }
    }
}

fn chip_id(idx: usize) -> LiveId {
    match idx {
        0 => live_id!(chip0),
        1 => live_id!(chip1),
        2 => live_id!(chip2),
        3 => live_id!(chip3),
        4 => live_id!(chip4),
        5 => live_id!(chip5),
        6 => live_id!(chip6),
        _ => live_id!(chip7),
    }
}

fn pick_id(idx: usize) -> LiveId {
    match idx {
        0 => live_id!(pk0),
        1 => live_id!(pk1),
        2 => live_id!(pk2),
        3 => live_id!(pk3),
        4 => live_id!(pk4),
        5 => live_id!(pk5),
        6 => live_id!(pk6),
        7 => live_id!(pk7),
        8 => live_id!(pk8),
        9 => live_id!(pk9),
        10 => live_id!(pk10),
        _ => live_id!(pk11),
    }
}

/// The rows of the edLookup list.
fn lookup_row_id(idx: usize) -> LiveId {
    match idx {
        0 => live_id!(lk0),
        1 => live_id!(lk1),
        2 => live_id!(lk2),
        3 => live_id!(lk3),
        4 => live_id!(lk4),
        5 => live_id!(lk5),
        6 => live_id!(lk6),
        7 => live_id!(lk7),
        8 => live_id!(lk8),
        9 => live_id!(lk9),
        10 => live_id!(lk10),
        11 => live_id!(lk11),
        12 => live_id!(lk12),
        13 => live_id!(lk13),
        14 => live_id!(lk14),
        _ => live_id!(lk15),
    }
}

/// Whether an edLookup list of `list_h` opens above its cell: only when it does not fit
/// below it in the window and fits above it — the way a combo's popup flips.
fn lookup_opens_above(cell_top: f64, cell_h: f64, list_h: f64, window_h: f64) -> bool {
    window_h > 0.0 && cell_top + cell_h + list_h > window_h && cell_top - list_h >= 0.0
}

/// What of a [`crawrad_runtime::grid::CellStyle`] a cell's widgets hold: background,
/// text colour, bold, italic (the text is set every frame anyway).
type AppliedStyle = (
    Option<crawrad_runtime::grid::Color>,
    Option<crawrad_runtime::grid::Color>,
    bool,
    bool,
);

/// The most lookup rows the list shows at once, whatever the column's DropDownCount.
const LOOKUP_ROWS: usize = 16;
/// The height of one lookup row.
const LOOKUP_ROW_HEIGHT: f64 = 22.0;
/// The height of the list's header line and of its footer line.
const LOOKUP_LINE_HEIGHT: f64 = 20.0;
/// The width of the list's frame, on each side.
const LOOKUP_FRAME: f64 = 1.0;
/// The narrowest an edLookup list gets: a narrow key column still shows its labels whole.
const LOOKUP_MIN_WIDTH: f64 = 240.0;

/// Which of `len` rows the list shows when `fit` fit: the first and how many. It stays where
/// it was (`start`, as the wheel left it) unless `follow` (the highlight just moved) and the
/// `highlighted` row is out of sight; then it scrolls just enough to show it.
fn lookup_window(
    len: usize,
    highlighted: usize,
    fit: usize,
    start: usize,
    follow: bool,
) -> (usize, usize) {
    let n = len.min(fit).min(LOOKUP_ROWS);
    if n == 0 {
        return (0, 0);
    }
    let mut start = start.min(len - n);
    if follow && highlighted < start {
        start = highlighted;
    } else if follow && highlighted >= start + n {
        start = highlighted + 1 - n;
    }
    (start, n)
}

/// Where an open edLookup list is scrolled. A list that closes resets it: the next one
/// opens on its highlight.
#[derive(Clone, Copy, Debug, Default)]
struct LookupScroll {
    /// The first row shown.
    start: usize,
    /// The highlighted row last drawn: the list follows the highlight only when it moves.
    last_highlight: Option<usize>,
    /// How many rows there are and how many show: what the wheel scrolls through.
    len_shown: (usize, usize),
}

/// The scroll bar's thumb for `n` of `len` rows from `start`, in a track `track_h` tall:
/// its top and its height. `None` when every row fits: no bar.
fn lookup_thumb(len: usize, n: usize, start: usize, track_h: f64) -> Option<(f64, f64)> {
    if n == 0 || len <= n {
        return None;
    }
    let thumb_h = (track_h * n as f64 / len as f64).max(12.0).min(track_h);
    let top = (track_h - thumb_h) * start as f64 / (len - n) as f64;
    Some((top, thumb_h))
}

/// The header and the rows of the list as lines of a fixed-width font: each display column
/// padded to its widest text, so the columns line up side by side.
fn lookup_lines(names: &[String], rows: &[Vec<String>]) -> (String, Vec<String>) {
    let widths: Vec<usize> = (0..names.len())
        .map(|c| {
            rows.iter()
                .filter_map(|r| r.get(c))
                .chain(std::iter::once(&names[c]))
                .map(|t| t.chars().count())
                .max()
                .unwrap_or(0)
        })
        .collect();
    let line = |cells: &[String]| {
        let mut out = String::new();
        for (c, w) in widths.iter().enumerate() {
            let t = cells.get(c).map(String::as_str).unwrap_or("");
            out.push_str(t);
            if c + 1 < widths.len() {
                out.extend(std::iter::repeat_n(' ', w - t.chars().count() + 3));
            }
        }
        out
    };
    (line(names), rows.iter().map(|r| line(r)).collect())
}

/// How many rows of `row_height` fit wholly in a rows area `height` tall (at least one).
fn rows_that_fit(height: f64, row_height: f64) -> usize {
    if row_height <= 0.0 {
        return 1;
    }
    ((height / row_height).floor() as usize).max(1)
}

/// The first item the list must start at so row `rec` is wholly in view, when the list now
/// starts at `first` and `fits` rows fit; `None` when it already is.
fn scroll_target(first: usize, fits: usize, rec: usize) -> Option<usize> {
    if rec < first {
        Some(rec)
    } else if rec >= first + fits {
        Some(rec + 1 - fits)
    } else {
        None
    }
}

/// Where the list's first row must move when the row count shrank under it (a write ends
/// a new row, or the rows are being read again): Makepad's PortalList unwraps the first row
/// among the ones it drew, so a first row past the end crashes the app.
fn first_in_range(first: usize, total: usize) -> Option<usize> {
    (total > 0 && first >= total).then(|| total - 1)
}

/// The height of the error box under a cell in error (one line of 9 pt text and its padding).
const ERROR_BOX_HEIGHT: f64 = 24.0;

/// Where the error box of a cell at `y` (`h` tall) goes in a rows area `area_h` tall: right
/// under the cell, or right over it when it would leave the area.
fn error_box_top(y: f64, h: f64, area_h: f64) -> f64 {
    if y + h + 1.0 + ERROR_BOX_HEIGHT > area_h {
        (y - 1.0 - ERROR_BOX_HEIGHT).max(0.0)
    } else {
        y + h + 1.0
    }
}

/// Where the error box (`w` wide) of a cell at `x` goes in a rows area `area_w` wide: aligned
/// with the cell, or pulled left so the message is not cut by the grid's right edge.
fn error_box_left(x: f64, w: f64, area_w: f64) -> f64 {
    x.min(area_w - w).max(0.0)
}

/// Translates a Makepad key event into a grid editing key.
pub fn edit_key_of(key: &KeyEvent) -> Option<EditKey> {
    match key.key_code {
        KeyCode::Escape => Some(EditKey::Escape),
        KeyCode::ReturnKey if key.modifiers.control || key.modifiers.logo => {
            Some(EditKey::CtrlEnter)
        }
        KeyCode::ReturnKey => Some(EditKey::Enter),
        KeyCode::F2 => Some(EditKey::F2),
        KeyCode::Tab => Some(EditKey::Tab {
            back: key.modifiers.shift,
        }),
        KeyCode::Insert => Some(EditKey::Insert),
        KeyCode::ArrowLeft => Some(EditKey::Left),
        KeyCode::ArrowRight => Some(EditKey::Right),
        KeyCode::ArrowUp => Some(EditKey::Up),
        KeyCode::ArrowDown => Some(EditKey::Down),
        _ => None,
    }
}

fn header_id(idx: usize) -> LiveId {
    match idx {
        0 => live_id!(h0),
        1 => live_id!(h1),
        2 => live_id!(h2),
        _ => live_id!(h3),
    }
}

/// The width of `text` drawn with `pen`, at its current size and scale.
fn pen_width(pen: &mut DrawText, cx: &mut Cx2d, text: &str) -> f64 {
    if text.is_empty() {
        return 0.0;
    }
    pen.layout(cx, 0.0, 0.0, None, false, Align::default(), text)
        .size_in_lpxs
        .width as f64
        * pen_scale(pen)
}

/// The height of one line drawn with `pen`.
fn pen_line_height(pen: &mut DrawText, cx: &mut Cx2d) -> f64 {
    let h = pen
        .layout(cx, 0.0, 0.0, None, false, Align::default(), "Ag")
        .size_in_lpxs
        .height as f64;
    (h * pen_scale(pen)).max(1.0)
}

/// The pen's `font_scale`, 1 when unset.
fn pen_scale(pen: &DrawText) -> f64 {
    if pen.font_scale > 0.0 {
        pen.font_scale as f64
    } else {
        1.0
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum CrawradDataGridAction {
    HeaderHover(usize, String, DVec2),
    CellHover(usize, usize, String, DVec2),
    HoverOut,
    Pending,
    /// The current row (`RecNo`) moved to this row.
    RowSelected(usize),
    None,
}

#[note("data-grid-crawraddatagrid")]
#[derive(Live, LiveHook, Widget)]
pub struct CrawradDataGrid {
    #[deref]
    pub view: View,
    #[live]
    pub draw_measure: DrawText,
    #[rust]
    pub state: Option<GridState>,
    #[rust(24.0)]
    pub row_height: f64,
    /// The last row announced as `RowSelected`.
    #[rust]
    reported_rec_no: Option<usize>,
    #[rust]
    pub live: Option<LiveRows>,
    #[rust]
    pub requested: (usize, usize),
    #[rust]
    custom_timing: Option<crawrad_component::BusyTiming>,
    #[rust]
    custom_dim: Option<f64>,
    /// Whether the last event still had the card waiting, shown or fading: the
    /// frame AFTER it ends must still be drawn, or the screen keeps the last
    /// faded card on top of the rows.
    #[rust]
    was_animating: bool,
    /// The header's laid-out width from the last frame: the columns share it.
    #[rust]
    shared_width: f64,
    /// The header's width changed during the last draw: draw once more.
    #[rust]
    width_changed: bool,
    #[rust]
    headers_styled: bool,
    #[rust]
    styled_rows: HashSet<usize>,
    /// The raw value each (row, column) cell was last styled for (Renderer).
    #[rust]
    renderer_values: HashMap<(usize, usize), String>,
    /// The rule / OnDrawCell look each (row, column) cell was last styled with.
    #[rust]
    applied_styles: HashMap<(usize, usize), Option<AppliedStyle>>,
    #[rust]
    hint_cells: Vec<(Rect, String, usize, usize)>,
    #[rust]
    link_cells: Vec<(Rect, String, usize)>,
    #[rust]
    hovered_cell: Option<(usize, usize)>,
    #[rust]
    hovered_header: Option<usize>,
    #[rust]
    resizing_col: Option<(usize, f64, f64)>,
    #[rust]
    open_list: Option<usize>,
    /// The kind whose hints (min/max or from/to) each filter slot's boxes
    /// carry. `apply_over` on a TextInput every frame wipes what is being
    /// typed, so the hints are written only when a slot changes kind.
    #[rust]
    filter_hints: Vec<&'static str>,
    #[rust(true)]
    needs_auto_width: bool,
    #[rust]
    pub keyboard: bool,
    #[rust]
    pub open_cell_list: bool,
    #[rust]
    shown_total: usize,
    #[rust]
    last_open_cell: Option<(usize, usize)>,
    /// Where the edLookup list is scrolled.
    #[rust]
    lookup_scroll: LookupScroll,
    #[rust]
    last_editor_rect: Option<Rect>,
    #[rust]
    last_error_rect: Option<Rect>,
    /// Where `body` was laid out last frame; the cell editor and the error box sit on it.
    #[rust]
    body_rect: Rect,
    /// Where `body` sits inside the grid: the popup layer places the cell editor by it.
    #[rust]
    body_offset: DVec2,
    /// The cell editor was just opened and gets the keyboard on the next event.
    #[rust]
    focus_editor: bool,
    /// Whether the status bar shows the editing hint (its colour was switched for it).
    #[rust]
    status_editing: bool,
    /// A write was answered: bring `rec_no` into view once its rows are read again.
    #[rust]
    reveal_rec: bool,
    /// The first row the list moves to at the START of the next draw. Moving it while the
    /// list draws makes Makepad's PortalList look for a row it did not draw, and crash.
    #[rust]
    pending_first: Option<usize>,
    /// A mouse button is down (a focus lost then is a click, not a Tab).
    #[rust]
    pointer_down: bool,
    #[rust]
    last_gutter: Option<bool>,
    #[rust]
    open_cell_rect: Option<Rect>,
    #[rust]
    error_cell_rect: Option<Rect>,
    #[rust]
    last_pointer_pos: DVec2,
    #[rust]
    modifiers: KeyModifiers,
}

impl CrawradDataGrid {
    /// Sends the write or records the pending check an editing key or click produced.
    fn apply(&mut self, outcome: EditOutcome) {
        match outcome {
            EditOutcome::Write(write) => {
                if let (Some(state), Some(live)) = (&mut self.state, &mut self.live) {
                    let ticket = live.write(write);
                    state.sent(ticket);
                }
            }
            EditOutcome::Check(check) => {
                if let Some(state) = &mut self.state {
                    state.pending_check = Some(check);
                }
            }
            EditOutcome::Button(req) => {
                if let Some(state) = &mut self.state {
                    state.pending_button = Some(req);
                }
            }
            _ => {}
        }
    }

    /// Brings `rec_no` into view by scrolling the list when outside the visible range.
    fn bring_rec_into_view(&mut self, cx: &mut Cx, rec_no: usize) {
        let list = self.view.portal_list(id!(list));
        let fits = rows_that_fit(self.body_rect.size.y, self.row_height);
        if let Some(first) = scroll_target(list.first_id(), fits, rec_no) {
            list.set_first_id_and_scroll(first, 0.0);
            self.view.redraw(cx);
        }
    }

    /// Processes an editing key against the current row and applies the outcome.
    fn handle_edit_key(&mut self, cx: &mut Cx, key: EditKey) {
        let total_rows = self.shown_total;
        let (Some(state), Some(live)) = (&mut self.state, &mut self.live) else {
            return;
        };
        let rec_before = state.rec_no;
        let now = std::time::Instant::now();
        let view = state.view(live.screen(), now);
        let current = view.lines.iter().find_map(|l| match l {
            ViewLine::Row {
                index,
                id,
                raw_cells,
                marker,
                ..
            } if *index == state.rec_no => Some(CurrentRow {
                index: *index,
                id: *id,
                raw: raw_cells,
                new: *marker == state.marks.new_row,
            }),
            _ => None,
        });
        let outcome = state.edit_key(key, current.as_ref(), total_rows);
        let moved_to = (state.rec_no != rec_before).then_some(state.rec_no);
        if outcome != EditOutcome::Ignored {
            self.open_cell_list = false;
        }
        self.apply(outcome);
        if let Some(rec_no) = moved_to {
            self.bring_rec_into_view(cx, rec_no);
        }
        self.view.redraw(cx);
    }
}

impl Widget for CrawradDataGrid {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // The wheel over an open lookup list scrolls the list, not the grid under it.
        if let Event::Scroll(e) = event {
            let list = self.view.view(id!(lookup_box));
            let (len, n) = self.lookup_scroll.len_shown;
            if list.visible()
                && n > 0
                && len > n
                && list.area().rect(cx).contains(e.abs)
                && e.scroll.y != 0.0
            {
                let step = 3usize;
                self.lookup_scroll.start = if e.scroll.y > 0.0 {
                    (self.lookup_scroll.start + step).min(len - n)
                } else {
                    self.lookup_scroll.start.saturating_sub(step)
                };
                e.handled_y.set(true);
                self.view.redraw(cx);
                return;
            }
        }
        let mut changed = false;
        if let Some(live) = &mut self.live
            && live.poll()
        {
            changed = true;
            self.needs_auto_width = true;
        }

        if let (Some(state), Some(live)) = (&mut self.state, &mut self.live)
            && let Some(written) = live.screen().written.as_ref()
            && state.on_written(written)
        {
            // The rows read again after the write may move: once they arrive (the answer
            // comes before them), the user's row is brought into view.
            self.reveal_rec = true;
        }

        match event {
            Event::MouseMove(e) => {
                self.last_pointer_pos = e.abs;
                self.modifiers = e.modifiers;
            }
            Event::MouseDown(e) => {
                self.pointer_down = true;
                self.last_pointer_pos = e.abs;
                self.modifiers = e.modifiers;
                let grid_rect = self.view.area().rect(cx);
                if !grid_rect.contains(e.abs) && !self.open_cell_list {
                    self.keyboard = false;
                }
            }
            Event::MouseUp(e) => {
                self.pointer_down = false;
                self.last_pointer_pos = e.abs;
                self.modifiers = e.modifiers;
            }
            Event::KeyDown(ke) => {
                self.modifiers = ke.modifiers;
            }
            Event::KeyUp(ke) => {
                self.modifiers = ke.modifiers;
            }
            _ => {}
        }

        let (animating, pending) = if let (Some(state), Some(live)) = (&self.state, &self.live) {
            let view = state.view(live.screen(), std::time::Instant::now());
            (view.animating, live.screen().pending.is_some())
        } else {
            (false, false)
        };

        let waiting_distinct = self.open_list.is_some()
            && if let (Some(state), Some(live)) = (&self.state, &self.live) {
                let screen = live.screen();
                if let Some(col_idx) = self.open_list {
                    let col_name = state.columns.get(col_idx).map(|c| c.name.as_str());
                    screen.distinct.as_ref().map(|(c, _)| c.as_str()) != col_name
                } else {
                    false
                }
            } else {
                false
            };

        let width_unknown = self.width_changed && matches!(event, Event::NextFrame(_));
        if width_unknown {
            self.width_changed = false;
        }
        if changed
            || animating
            || pending
            || self.was_animating
            || waiting_distinct
            || width_unknown
            || self.pending_first.is_some()
        {
            self.view.redraw(cx);
            cx.new_next_frame();
        }
        self.was_animating = animating || pending;

        if let Event::KeyDown(ke) = event
            && ke.key_code == KeyCode::Escape
            && self.open_list.is_some()
        {
            self.open_list = None;
            self.view.redraw(cx);
        }

        let ed = self.view.view(id!(cell_editor)).text_input(id!(ed));
        if self.focus_editor && ed.area().is_valid(cx) {
            self.focus_editor = false;
            ed.set_key_focus(cx);
        }
        let editor_focused = ed.key_focus(cx);
        if self.keyboard && !editor_focused {
            if let Event::KeyDown(ke) = event {
                if let Some(key) = edit_key_of(ke) {
                    self.handle_edit_key(cx, key);
                }
            } else if let Event::TextInput(ti) = event
                && !self.modifiers.control
                && !self.modifiers.logo
                && !ti.input.chars().any(char::is_control)
                && !ti.input.is_empty()
            {
                self.handle_edit_key(cx, EditKey::Char(ti.input.clone()));
            }
        } else if editor_focused
            && let Event::KeyDown(ke) = event
            && (matches!(
                ke.key_code,
                KeyCode::Tab | KeyCode::ArrowUp | KeyCode::ArrowDown | KeyCode::Insert
            ) || (ke.key_code == KeyCode::ReturnKey
                && (ke.modifiers.control || ke.modifiers.logo)))
            && let Some(key) = edit_key_of(ke)
        {
            // The text box keeps these keys to itself (Tab never reaches its actions):
            // they move between cells and rows, so the grid takes them first.
            self.handle_edit_key(cx, key);
        }

        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));

        if let Some(text) = ed.changed(&actions)
            && let Some(state) = &mut self.state
            && let Some(edit) = &mut state.edit
        {
            let mask = edit
                .open
                .and_then(|col_idx| state.columns.get(col_idx))
                .map(|col| col.edit_mask.as_str())
                .unwrap_or("");
            let typed = text.clone();
            if !mask.is_empty() {
                let held = crate::edit::hold_mask(cx, &ed, &edit.editor_from, &text, mask);
                edit.editor_text = held.unwrap_or(text);
            } else {
                edit.editor_text = text;
            }
            if edit
                .open
                .and_then(|col_idx| state.columns.get(col_idx))
                .map(effective_editor)
                == Some("edLookup")
            {
                state.update_lookup_search(&typed);
            }
        }
        if ed.returned(&actions).is_some() {
            if self.modifiers.control || self.modifiers.logo {
                self.handle_edit_key(cx, EditKey::CtrlEnter);
            } else {
                self.handle_edit_key(cx, EditKey::Enter);
            }
        }
        if ed.escaped(&actions) {
            self.handle_edit_key(cx, EditKey::Escape);
        }
        // The window takes Tab for its own focus moves before any widget sees it: the open
        // editor losing the keyboard with no click is that Tab.
        let editor_open = self
            .state
            .as_ref()
            .is_some_and(|s| s.edit.as_ref().is_some_and(|e| e.open.is_some()));
        let focus_lost = actions
            .filter_widget_actions_cast::<TextInputAction>(ed.widget_uid())
            .any(|a| matches!(a, TextInputAction::KeyFocusLost));
        if focus_lost && editor_open && !self.pointer_down {
            let back = self.modifiers.shift;
            self.handle_edit_key(cx, EditKey::Tab { back });
        }

        let cell_editor_view = self.view.view(id!(cell_editor));
        let btn_dots = cell_editor_view.button(id!(btn_dots));
        if btn_dots.clicked(&actions)
            && let Some(state) = &mut self.state
        {
            let outcome = state.trigger_button();
            self.apply(outcome);
            self.view.redraw(cx);
        }
        let btn_arrow = cell_editor_view.button(id!(btn_arrow));
        if btn_arrow.clicked(&actions)
            && let Some(state) = &mut self.state
            && let Some((listed, typed)) = state
                .edit
                .as_ref()
                .map(|e| (e.lookup.is_some(), e.editor_text.clone()))
        {
            if listed {
                if let Some(edit) = state.edit.as_mut() {
                    edit.lookup = None;
                }
            } else {
                state.update_lookup_search(&typed);
            }
            self.view.redraw(cx);
        }

        // edList and edLookup pick buttons
        for idx in 0..12 {
            let btn = self.view.view(id!(cell_editor)).button(&[pick_id(idx)]);
            if btn.clicked(&actions)
                && let Some(state) = &mut self.state
                && let Some(open_col) = state.edit.as_ref().and_then(|e| e.open)
                && let Some(col) = state.columns.get(open_col)
                && let Some(opt) = col.pick_list.get(idx)
            {
                if let Some(edit) = &mut state.edit {
                    edit.editor_text = opt.clone();
                }
                state.commit_editor();
                self.open_cell_list = false;
                self.view.redraw(cx);
            }
        }

        for idx in 0..LOOKUP_ROWS {
            let row = self
                .view
                .view(id!(cell_editor))
                .view(&[lookup_row_id(idx)])
                .button(id!(btn));
            if row.clicked(&actions)
                && let Some(state) = &mut self.state
            {
                state.pick_lookup(self.lookup_scroll.start + idx);
                self.view.redraw(cx);
            }
        }

        if let Event::MouseMove(e) = event {
            if let Some((col_idx, start_x, start_w)) = self.resizing_col {
                cx.set_cursor(MouseCursor::ColResize);
                let delta = e.abs.x - start_x;
                let new_w = start_w + delta;
                if let Some(state) = &mut self.state {
                    let min_w = state
                        .column_order
                        .get(col_idx)
                        .and_then(|&pi| state.columns.get(pi))
                        .map(|c| (c.min_width as f64).max(MIN_COLUMN_WIDTH))
                        .unwrap_or(MIN_COLUMN_WIDTH);
                    let max_w = state
                        .column_order
                        .get(col_idx)
                        .and_then(|&pi| state.columns.get(pi))
                        .and_then(|c| c.max_width.map(f64::from));
                    let clamped = crawrad_runtime::grid::clamp_column_width(new_w, min_w, max_w);
                    state.set_column_width(col_idx, clamped as f32);
                    self.view.redraw(cx);
                }
            } else {
                let num_cols = self.state.as_ref().map_or(0, |s| s.columns.len());
                let header = self.view.view(id!(header));
                let mut is_resize = false;
                for idx in 0..num_cols.min(MAX_COLUMNS) {
                    let cell = header.view(&[cell_id(idx)]);
                    let rect = cell.area().rect(cx);
                    if rect.contains(e.abs) && e.abs.x >= rect.pos.x + rect.size.x - 6.0 {
                        let is_resizable = self
                            .state
                            .as_ref()
                            .and_then(|s| s.column_order.get(idx).and_then(|&pi| s.columns.get(pi)))
                            .is_none_or(|c| c.resizable);
                        if is_resizable {
                            cx.set_cursor(MouseCursor::ColResize);
                            is_resize = true;
                            break;
                        }
                    }
                }
                if !is_resize
                    && self
                        .link_cells
                        .iter()
                        .any(|(rect, _, _)| rect.contains(e.abs))
                {
                    cx.set_cursor(MouseCursor::Hand);
                }
            }

            let found = self
                .hint_cells
                .iter()
                .find(|(rect, _, _, _)| rect.contains(e.abs));
            match found {
                Some((_, full_text, row, col)) => {
                    let cell_key = (*row, *col);
                    if self.hovered_cell != Some(cell_key) {
                        self.hovered_cell = Some(cell_key);
                        cx.widget_action(
                            self.widget_uid(),
                            &scope.path,
                            CrawradDataGridAction::CellHover(*row, *col, full_text.clone(), e.abs),
                        );
                    }
                }
                None => {
                    if self.hovered_cell.is_some() {
                        self.hovered_cell = None;
                        if self.hovered_header.is_none() {
                            cx.widget_action(
                                self.widget_uid(),
                                &scope.path,
                                CrawradDataGridAction::HoverOut,
                            );
                        }
                    }
                }
            }
        }

        // Group chip close button handling
        let group_bar = self.view.view(id!(group_bar));
        for idx in 0..8 {
            let chip = group_bar.view(&[chip_id(idx)]);
            if chip.button(id!(btn_close)).clicked(&actions)
                && let (Some(state), Some(live)) = (&mut self.state, &mut self.live)
            {
                let q = state.ungroup(idx);
                live.set_query(q);
                self.requested = (0, 200);
                self.view.redraw(cx);
                cx.new_next_frame();
            }
        }

        // Header click and header drag to group bar or reorder
        let num_cols = self.state.as_ref().map_or(0, |s| s.columns.len());
        let header = self.view.view(id!(header));
        let bar_rect = group_bar.area().rect(cx);

        for idx in 0..num_cols.min(MAX_COLUMNS) {
            let cell = header.view(&[cell_id(idx)]);
            let cell_rect = cell.area().rect(cx);

            if let Some(down) = cell.finger_down(&actions) {
                let is_edge = down.abs.x >= cell_rect.pos.x + cell_rect.size.x - 6.0;
                let is_resizable = self
                    .state
                    .as_ref()
                    .and_then(|s| s.column_order.get(idx).and_then(|&pi| s.columns.get(pi)))
                    .is_none_or(|c| c.resizable);

                if is_edge && is_resizable {
                    self.resizing_col = Some((idx, down.abs.x, cell_rect.size.x));
                }
            }

            if let Some(up) = cell.finger_up(&actions) {
                if self.resizing_col.take().is_some() {
                    self.view.redraw(cx);
                } else if bar_rect.contains(up.abs) {
                    if let (Some(state), Some(live)) = (&mut self.state, &mut self.live) {
                        let q = state.group_by_column(idx);
                        live.set_query(q);
                        self.requested = (0, 200);
                        self.view.redraw(cx);
                        cx.new_next_frame();
                    }
                } else {
                    let mut dropped_on_other = false;
                    let is_movable = self
                        .state
                        .as_ref()
                        .and_then(|s| s.column_order.get(idx).and_then(|&pi| s.columns.get(pi)))
                        .is_none_or(|c| c.movable);

                    if is_movable {
                        for target_idx in 0..num_cols.min(MAX_COLUMNS) {
                            if target_idx != idx {
                                let target_cell = header.view(&[cell_id(target_idx)]);
                                if target_cell.area().rect(cx).contains(up.abs) {
                                    if let Some(state) = &mut self.state {
                                        state.move_column(idx, target_idx);
                                        self.headers_styled = false;
                                        self.styled_rows.clear();
                                        self.renderer_values.clear();
                                        self.applied_styles.clear();
                                        self.open_list = None;
                                        self.view.redraw(cx);
                                        cx.new_next_frame();
                                    }
                                    dropped_on_other = true;
                                    break;
                                }
                            }
                        }
                    }

                    if !dropped_on_other
                        && up.is_over
                        && let (Some(state), Some(live)) = (&mut self.state, &mut self.live)
                    {
                        let q = state.click_header(idx);
                        live.set_query(q);
                        self.requested = (0, 200);
                        self.view.redraw(cx);
                        cx.new_next_frame();
                    }
                }
            }

            if let Some(state) = &self.state
                && let Some(col) = state
                    .column_order
                    .get(idx)
                    .and_then(|&pi| state.columns.get(pi))
                && !col.title_hint.is_empty()
            {
                match event.hits(cx, cell.area()) {
                    Hit::FingerHoverIn(fe) | Hit::FingerHoverOver(fe)
                        if self.hovered_header != Some(idx) =>
                    {
                        self.hovered_header = Some(idx);
                        cx.widget_action(
                            self.widget_uid(),
                            &scope.path,
                            CrawradDataGridAction::HeaderHover(idx, col.title_hint.clone(), fe.abs),
                        );
                    }
                    Hit::FingerHoverOut(_) if self.hovered_header == Some(idx) => {
                        self.hovered_header = None;
                        if self.hovered_cell.is_none() {
                            cx.widget_action(
                                self.widget_uid(),
                                &scope.path,
                                CrawradDataGridAction::HoverOut,
                            );
                        }
                    }
                    _ => {}
                }
            }
        }

        // A release that the header never saw (the pointer left the grid
        // mid-drag) still ends the resize — checked AFTER the header loop so a
        // resize's own release is not read as a header click (sort).
        if let Event::MouseUp(e) = event {
            if self.resizing_col.take().is_some() {
                self.view.redraw(cx);
            } else if let Some((_, raw_val, row_idx)) = self
                .link_cells
                .iter()
                .find(|(rect, _, _)| rect.contains(e.abs))
            {
                if let Some(state) = &mut self.state {
                    state.rec_no = *row_idx;
                }
                if crawrad_runtime::shell::openable(raw_val) {
                    crawrad_runtime::shell::open_url(raw_val);
                }
                self.view.redraw(cx);
            } else if let Some(open_col) = self.open_list {
                let popup_rect = self.view.view(id!(filter_popup)).area().rect(cx);
                let filter_bar = self.view.view(id!(filter_bar));
                let open_slot = self
                    .state
                    .as_ref()
                    .and_then(|s| s.column_order.iter().position(|&ci| ci == open_col));
                let pick_rect = open_slot
                    .map(|slot| {
                        filter_bar
                            .view(&[cell_id(slot)])
                            .button(id!(pick))
                            .area()
                            .rect(cx)
                    })
                    .unwrap_or_default();

                if !popup_rect.contains(e.abs) && !pick_rect.contains(e.abs) {
                    self.open_list = None;
                    self.view.redraw(cx);
                }
            }
        }

        // Row click handling
        let list = self.view.portal_list(id!(list));
        let clicked = list
            .items_with_actions(&actions)
            .into_iter()
            .find(|(_, item)| item.view(id!(row)).button(id!(btn)).clicked(&actions))
            .map(|(i, _)| i);
        if let Some(i) = clicked
            && let (Some(state), Some(live)) = (&mut self.state, &self.live)
        {
            let now = std::time::Instant::now();
            let view = state.view(live.screen(), now);
            let gutter_w = if view.gutter { 22.0 } else { 0.0 };
            let num_cols = state.columns.len().min(MAX_COLUMNS);
            let ordered_cols = state.ordered_columns();
            let available_cols_w = (self.shared_width - gutter_w).max(0.0);
            let widths = column_widths(available_cols_w, &ordered_cols);

            let body_rect = self.view.view(id!(body)).area().rect(cx);
            let rel_x = self.last_pointer_pos.x - body_rect.pos.x;
            let mut cur_x = gutter_w;
            let mut position = state.focus_col;
            for (c_idx, &w) in widths.iter().enumerate().take(num_cols) {
                if rel_x >= cur_x && rel_x < cur_x + w {
                    position = c_idx;
                    break;
                }
                cur_x += w;
            }

            let current = view.lines.iter().find_map(|l| match l {
                ViewLine::Row {
                    index,
                    id,
                    raw_cells,
                    marker,
                    ..
                } if *index == i => Some(CurrentRow {
                    index: *index,
                    id: *id,
                    raw: raw_cells,
                    new: *marker == state.marks.new_row,
                }),
                _ => None,
            });
            let fallback = CurrentRow {
                index: i,
                id: 0,
                raw: &[],
                new: false,
            };
            let row = current.as_ref().unwrap_or(&fallback);
            self.keyboard = true;
            let outcome = state.click_cell(row, position);
            self.apply(outcome);
            self.view.redraw(cx);
        }

        // Filter distinct list check box handling
        let dlist = self.view.portal_list(id!(distinct_list));
        let toggled = dlist
            .items_with_actions(&actions)
            .into_iter()
            .find_map(|(i, row)| row.as_check_box().changed(&actions).map(|on| (i, on)));

        if let Some((item_idx, on)) = toggled
            && let Some(open_col) = self.open_list
            && let (Some(state), Some(live)) = (&mut self.state, &mut self.live)
        {
            let col_name = state
                .columns
                .get(open_col)
                .map(|c| c.name.as_str())
                .unwrap_or("");
            let distinct_vals = live
                .screen()
                .distinct
                .as_ref()
                .filter(|(col, _)| col == col_name)
                .map(|(_, vals)| vals.clone());

            if let Some(vals) = distinct_vals
                && let Some(val_opt) = vals.get(item_idx)
            {
                let mut current_checked: Vec<Option<String>> = match state.filters.get(&open_col) {
                    Some(FilterInput::List(list)) => list.clone(),
                    _ => Vec::new(),
                };

                if on {
                    if !current_checked.contains(val_opt) {
                        current_checked.push(val_opt.clone());
                    }
                } else {
                    current_checked.retain(|v| v != val_opt);
                }

                if current_checked.is_empty() {
                    state.filters.remove(&open_col);
                } else {
                    state
                        .filters
                        .insert(open_col, FilterInput::List(current_checked));
                }

                let q = state.query();
                live.set_query(q);
                self.requested = (0, 200);
                self.view.redraw(cx);
                cx.new_next_frame();
            }
        }

        // Filter bar input handling
        let mut filter_changed = false;
        let filter_bar = self.view.view(id!(filter_bar));
        if filter_bar.visible()
            && let Some(state) = &mut self.state
        {
            let num_cols = state.columns.len().min(MAX_COLUMNS);
            for slot in 0..num_cols {
                let cell = filter_bar.view(&[cell_id(slot)]);
                if !cell.visible() {
                    continue;
                }
                let Some(&col_idx) = state.column_order.get(slot) else {
                    continue;
                };
                let Some(col) = state.columns.get(col_idx) else {
                    continue;
                };
                if !col.filterable {
                    continue;
                }
                let kind = effective_filter_kind(col);
                match kind {
                    "fkText" => {
                        let txt = cell.text_input(id!(txt));
                        if let Some(text) = txt.changed(&actions) {
                            if text.trim().is_empty() {
                                state.filters.remove(&col_idx);
                            } else {
                                state.filters.insert(col_idx, FilterInput::Text(text));
                            }
                            filter_changed = true;
                        }
                    }
                    "fkRange" | "fkDate" => {
                        let is_range = kind == "fkRange";
                        let lo = cell.text_input(id!(range_box.lo));
                        let hi = cell.text_input(id!(range_box.hi));
                        if lo.changed(&actions).is_some() || hi.changed(&actions).is_some() {
                            let lo_text = lo.text();
                            let hi_text = hi.text();
                            if lo_text.trim().is_empty() && hi_text.trim().is_empty() {
                                state.filters.remove(&col_idx);
                            } else {
                                let input = if is_range {
                                    FilterInput::Range {
                                        min: lo_text,
                                        max: hi_text,
                                    }
                                } else {
                                    FilterInput::Date {
                                        from: lo_text,
                                        to: hi_text,
                                    }
                                };
                                state.filters.insert(col_idx, input);
                            }
                            filter_changed = true;
                        }
                    }
                    "fkList" => {
                        let pick = cell.button(id!(pick));
                        if pick.clicked(&actions) {
                            if self.open_list == Some(col_idx) {
                                self.open_list = None;
                            } else {
                                self.open_list = Some(col_idx);
                                if let Some(live) = &mut self.live {
                                    let q_without = state.query_without(col_idx);
                                    live.distinct(col.name.clone(), q_without);
                                }
                            }
                            self.view.redraw(cx);
                        }
                    }
                    _ => {}
                }
            }
        }

        if filter_changed && let (Some(state), Some(live)) = (&mut self.state, &mut self.live) {
            let q = state.query();
            live.set_query(q);
            self.requested = (0, 200);
            self.view.redraw(cx);
            cx.new_next_frame();
        }

        let has_pending_reqs = match (&self.state, &self.live) {
            (Some(state), Some(live)) => state.has_stale_screen(live.screen()),
            _ => false,
        };
        let has_pending_check = self
            .state
            .as_ref()
            .is_some_and(|s| s.pending_check.is_some());
        let has_pending_button = self
            .state
            .as_ref()
            .is_some_and(|s| s.pending_button.is_some());
        if has_pending_reqs || has_pending_check || has_pending_button {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                CrawradDataGridAction::Pending,
            );
        }
        if let Some(rec_no) = self.state.as_ref().map(|s| s.rec_no)
            && self.reported_rec_no != Some(rec_no)
        {
            self.reported_rec_no = Some(rec_no);
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                CrawradDataGridAction::RowSelected(rec_no),
            );
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.hint_cells.clear();
        self.link_cells.clear();
        let (Some(state), Some(live)) = (&mut self.state, &mut self.live) else {
            return self.view.draw_walk(cx, scope, walk);
        };

        let now = std::time::Instant::now();
        state.update_lookups(live.screen());
        state.refresh_rule_styles(live.screen());
        let view = state.view(live.screen(), now);

        if self.needs_auto_width {
            let screen = live.screen();
            for (col_idx, col) in state.columns.iter_mut().enumerate() {
                if col.auto_size {
                    let source_idx = col.source.unwrap_or(col_idx);
                    let cell_texts: Vec<&str> = screen
                        .lines
                        .iter()
                        .filter_map(|line| match line {
                            datagrid::ScreenLine::Row { cells, .. } => {
                                cells.get(source_idx).and_then(|c| c.as_deref())
                            }
                            _ => None,
                        })
                        .collect();
                    self.draw_measure.text_style.font_size = if col.font.size > 0.0 {
                        col.font.size
                    } else {
                        9.0
                    };
                    let min_w = (col.min_width as f64).max(MIN_COLUMN_WIDTH);
                    let max_w = col.max_width.map(f64::from);
                    let w = crawrad_runtime::grid::auto_width(
                        &col.title,
                        &cell_texts,
                        16.0,
                        min_w,
                        max_w,
                        |s| pen_width(&mut self.draw_measure, cx, s),
                    );
                    col.width = Some(w as f32);
                }
            }
            self.needs_auto_width = false;
        }

        let ordered_cols = state.ordered_columns();
        // 1. Sync header cells
        let header = self.view.view(id!(header));
        let num_cols = view.header.len().min(MAX_COLUMNS);
        let gutter_w = if view.gutter { 22.0 } else { 0.0 };
        let available_cols_w = (self.shared_width - gutter_w).max(0.0);
        let widths = if ordered_cols.iter().all(|c| {
            c.width.is_none() && !c.auto_size && c.min_width <= 0.0 && c.max_width.is_none()
        }) {
            // No designed widths: the same text-weighted share as the Designer.
            let titles: Vec<&str> = ordered_cols.iter().map(|c| c.title.as_str()).collect();
            let screen = live.screen();
            crawrad_runtime::grid::text_share_widths(
                &titles,
                |i| {
                    let source = ordered_cols[i].source.unwrap_or(i);
                    screen
                        .lines
                        .iter()
                        .filter_map(|line| match line {
                            datagrid::ScreenLine::Row { cells, .. } => {
                                cells.get(source).and_then(|c| c.as_deref())
                            }
                            _ => None,
                        })
                        .collect()
                },
                available_cols_w,
            )
        } else {
            column_widths(available_cols_w, &ordered_cols)
        };
        let fills =
            |i: usize| i + 1 == num_cols && ordered_cols.get(i).is_some_and(|c| c.width.is_none());

        if self.last_gutter != Some(view.gutter) {
            self.last_gutter = Some(view.gutter);
            self.styled_rows.clear();
        }
        header.view(id!(gutter)).set_visible(cx, view.gutter);

        // Header cell background and label styling is applied once (not per frame).
        if !self.headers_styled {
            // A transparent TitleColor keeps the header's own background.
            let mut max_header_h = 34.0;
            for (idx, col_info) in view.header.iter().take(num_cols).enumerate() {
                let cell = header.view(&[cell_id(idx)]);
                if col_info.title_color.a > 0 {
                    let bg = vec4(
                        col_info.title_color.r as f32 / 255.0,
                        col_info.title_color.g as f32 / 255.0,
                        col_info.title_color.b as f32 / 255.0,
                        col_info.title_color.a as f32 / 255.0,
                    );
                    cell.apply_over(cx, live! { show_bg: true, draw_bg: { color: (bg) } });
                }
                let s = if col_info.title_font.size > 0.0 {
                    col_info.title_font.size as f64
                } else {
                    9.0
                };
                cell.label(id!(lbl))
                    .apply_over(cx, live! { draw_text: { text_style: { font_size: (s) } } });

                if col_info.title_word_wrap {
                    let w = widths.get(idx).copied().unwrap_or(MIN_COLUMN_WIDTH);
                    let max_w = (w - 16.0).max(0.0);
                    self.draw_measure.text_style.font_size = if col_info.title_font.size > 0.0 {
                        col_info.title_font.size
                    } else {
                        9.0
                    };
                    let line_h = pen_line_height(&mut self.draw_measure, cx);
                    let full = format!("{}{}", col_info.title, col_info.arrow);
                    let lines = crawrad_runtime::grid::wrap_text(&full, max_w, 3, false, |s| {
                        pen_width(&mut self.draw_measure, cx, s)
                    });
                    let needed_h = (lines.len() as f64 * line_h + 12.0).max(34.0);
                    if needed_h > max_header_h {
                        max_header_h = needed_h;
                    }
                }
            }
            header.apply_over(cx, live! { height: (max_header_h) });
            self.headers_styled = true;
        }

        for idx in 0..MAX_COLUMNS {
            let cell = header.view(&[cell_id(idx)]);
            if idx < num_cols {
                cell.set_visible(cx, true);
                let col_info = &view.header[idx];
                let w = widths.get(idx).copied().unwrap_or(MIN_COLUMN_WIDTH);
                apply_cell_walk(cx, &cell, w, fills(idx), col_info.align, None);
                let title_text = if col_info.title_word_wrap {
                    let full = format!("{}{}", col_info.title, col_info.arrow);
                    let max_w = (w - 16.0).max(0.0);
                    self.draw_measure.text_style.font_size = if col_info.title_font.size > 0.0 {
                        col_info.title_font.size
                    } else {
                        9.0
                    };
                    let lines = crawrad_runtime::grid::wrap_text(&full, max_w, 3, false, |s| {
                        pen_width(&mut self.draw_measure, cx, s)
                    });
                    lines.join("\n")
                } else {
                    format!("{}{}", col_info.title, col_info.arrow)
                };
                // The sorted column stays blue; the others take their TitleFont colour.
                let text_color = if !col_info.arrow.is_empty() {
                    vec4(0.478, 0.655, 1.0, 1.0)
                } else if col_info.title_font.color.a > 0 {
                    vec4(
                        col_info.title_font.color.r as f32 / 255.0,
                        col_info.title_font.color.g as f32 / 255.0,
                        col_info.title_font.color.b as f32 / 255.0,
                        col_info.title_font.color.a as f32 / 255.0,
                    )
                } else {
                    vec4(0.875, 0.882, 0.898, 1.0)
                };
                cell.label(id!(lbl))
                    .apply_over(cx, live! { draw_text: { color: (text_color) } });
                cell.label(id!(lbl)).set_text(cx, &title_text);
            } else {
                cell.set_visible(cx, false);
            }
        }

        // 2. Sync group bar (chips & hint)
        let group_bar = self.view.view(id!(group_bar));
        let num_chips = view.chips.len();
        for idx in 0..8 {
            let chip = group_bar.view(&[chip_id(idx)]);
            if idx < num_chips {
                chip.set_visible(cx, true);
                chip.label(id!(lbl)).set_text(cx, &view.chips[idx]);
            } else {
                chip.set_visible(cx, false);
            }
        }
        let hint = group_bar.label(id!(lbl_hint));
        hint.set_visible(cx, num_chips == 0);
        hint.set_text(cx, view.group_hint);

        // 3. Sync footer
        let footer = self.view.view(id!(footer));
        let lbl_position = footer.label(id!(lbl_position));
        let lbl_state = footer.label(id!(lbl_state));
        // While a row is being edited its hint takes the place of the position, as in the
        // egui app; the colour changes only when that switches (never per frame).
        let editing = !view.status_text.is_empty();
        if self.status_editing != editing {
            self.status_editing = editing;
            let colour = if editing {
                vec4(1.0, 0.824, 0.478, 1.0)
            } else {
                vec4(0.875, 0.882, 0.898, 1.0)
            };
            lbl_position.apply_over(cx, live! { draw_text: { color: (colour) } });
        }
        lbl_position.set_text(
            cx,
            if editing {
                &view.status_text
            } else {
                &view.footer.position
            },
        );
        lbl_state.set_text(cx, if editing { "" } else { view.footer.state });
        footer
            .label(id!(lbl_order))
            .set_text(cx, &view.footer.order);
        // The editing hint is long: while it shows, the timing gives it the room.
        footer
            .label(id!(lbl_timing))
            .set_text(cx, if editing { "" } else { &view.footer.timing });
        let error_line = self.view.view(id!(error_line));
        let error = view.error.as_deref().unwrap_or("");
        if error_line.visible() == error.is_empty() {
            error_line.set_visible(cx, !error.is_empty());
        }
        error_line.label(id!(lbl_error)).set_text(cx, error);

        // Sync filter bar
        let filter_bar = self.view.view(id!(filter_bar));
        filter_bar.view(id!(gutter)).set_visible(cx, view.gutter);
        if !view.filter_row.is_empty() {
            filter_bar.set_visible(cx, true);
            sync_bar_cells(
                cx,
                &filter_bar,
                num_cols,
                &ordered_cols,
                &widths,
                fills,
                |cx, idx, cell, _| {
                    if let Some(f_cell) = view.filter_row.get(idx) {
                        if !f_cell.enabled {
                            cell.apply_over(cx, live! { draw_bg: { color: (CELL_BG_DIMMED) } });
                            cell.view(id!(txt_box)).set_visible(cx, false);
                            cell.view(id!(range_box)).set_visible(cx, false);
                            cell.button(id!(pick)).set_visible(cx, false);
                        } else {
                            cell.apply_over(cx, live! { draw_bg: { color: (CELL_BG_ENABLED) } });
                            match f_cell.kind {
                                "fkText" => {
                                    cell.view(id!(txt_box)).set_visible(cx, true);
                                    cell.view(id!(range_box)).set_visible(cx, false);
                                    cell.button(id!(pick)).set_visible(cx, false);

                                    let exp = match state.filters.get(&f_cell.column) {
                                        Some(FilterInput::Text(s)) => s.as_str(),
                                        _ => "",
                                    };
                                    let txt = cell.text_input(id!(txt));
                                    if !txt.key_focus(cx) && txt.text() != exp {
                                        txt.set_text(cx, exp);
                                    }
                                }
                                "fkRange" | "fkDate" => {
                                    cell.view(id!(txt_box)).set_visible(cx, false);
                                    cell.view(id!(range_box)).set_visible(cx, true);
                                    cell.button(id!(pick)).set_visible(cx, false);

                                    let is_range = f_cell.kind == "fkRange";
                                    let (exp_lo, exp_hi) = match state.filters.get(&f_cell.column) {
                                        Some(FilterInput::Range { min, max }) if is_range => {
                                            (min.as_str(), max.as_str())
                                        }
                                        Some(FilterInput::Date { from, to }) if !is_range => {
                                            (from.as_str(), to.as_str())
                                        }
                                        _ => ("", ""),
                                    };

                                    let (hint_lo, hint_hi) = if is_range {
                                        ("min", "max")
                                    } else {
                                        ("from", "to")
                                    };

                                    let lo = cell.text_input(id!(range_box.lo));
                                    let hi = cell.text_input(id!(range_box.hi));
                                    if self.filter_hints.len() <= idx {
                                        self.filter_hints.resize(idx + 1, "");
                                    }
                                    if self.filter_hints[idx] != f_cell.kind {
                                        self.filter_hints[idx] = f_cell.kind;
                                        lo.apply_over(cx, live! { empty_text: (hint_lo) });
                                        hi.apply_over(cx, live! { empty_text: (hint_hi) });
                                    }
                                    if !lo.key_focus(cx) && lo.text() != exp_lo {
                                        lo.set_text(cx, exp_lo);
                                    }
                                    if !hi.key_focus(cx) && hi.text() != exp_hi {
                                        hi.set_text(cx, exp_hi);
                                    }
                                }
                                "fkList" => {
                                    cell.view(id!(txt_box)).set_visible(cx, false);
                                    cell.view(id!(range_box)).set_visible(cx, false);
                                    cell.button(id!(pick)).set_visible(cx, true);

                                    cell.button(id!(pick)).set_text(cx, &f_cell.caption);
                                }
                                _ => {
                                    cell.view(id!(txt_box)).set_visible(cx, false);
                                    cell.view(id!(range_box)).set_visible(cx, false);
                                    cell.button(id!(pick)).set_visible(cx, false);
                                }
                            }
                        }
                    } else {
                        cell.view(id!(txt_box)).set_visible(cx, false);
                        cell.view(id!(range_box)).set_visible(cx, false);
                        cell.button(id!(pick)).set_visible(cx, false);
                    }
                },
            );
        } else {
            filter_bar.set_visible(cx, false);
        }

        // Sync summary bar
        let summary_bar = self.view.view(id!(summary_bar));
        summary_bar.view(id!(gutter)).set_visible(cx, view.gutter);
        if !view.summary.is_empty() {
            summary_bar.set_visible(cx, true);
            sync_bar_cells(
                cx,
                &summary_bar,
                num_cols,
                &ordered_cols,
                &widths,
                fills,
                |cx, idx, cell, _| {
                    let text = view.summary.get(idx).map(String::as_str).unwrap_or("");
                    cell.label(id!(lbl)).set_text(cx, text);
                },
            );
        } else {
            summary_bar.set_visible(cx, false);
        }

        // Sync filter popup
        let filter_popup = self.view.view(id!(filter_popup));
        let mut distinct_values: Option<Vec<Option<String>>> = None;
        let mut current_checked_list: Vec<Option<String>> = Vec::new();

        if let Some(open_col) = self.open_list {
            filter_popup.set_visible(cx, true);

            let open_slot = state
                .column_order
                .iter()
                .position(|&ci| ci == open_col)
                .unwrap_or(0);
            let mut col_x = 0.0;
            for w in widths.iter().take(open_slot) {
                col_x += *w;
            }
            let grid_w = self.shared_width.max(200.0);
            let popup_w = 160.0_f64.min(grid_w);
            let max_x = (grid_w - popup_w).max(0.0);
            let popup_x = col_x.min(max_x);

            let col_name = state
                .columns
                .get(open_col)
                .map(|c| c.name.as_str())
                .unwrap_or("");
            let screen_distinct = live
                .screen()
                .distinct
                .as_ref()
                .filter(|(col, _)| col == col_name)
                .map(|(_, vals)| vals.clone());

            if let Some(FilterInput::List(list)) = state.filters.get(&open_col) {
                current_checked_list = list.clone();
            }

            if let Some(vals) = screen_distinct {
                let popup_h = ((vals.len() as f64) * 22.0 + 8.0).clamp(40.0, 250.0);
                filter_popup.apply_over(
                    cx,
                    live! {
                        margin: { left: (popup_x), top: 0.0 },
                        width: (popup_w),
                        height: (popup_h),
                    },
                );
                filter_popup.label(id!(lbl_loading)).set_text(cx, "");
                filter_popup
                    .portal_list(id!(distinct_list))
                    .set_visible(cx, true);
                distinct_values = Some(vals);
            } else {
                filter_popup.apply_over(
                    cx,
                    live! {
                        margin: { left: (popup_x), top: 0.0 },
                        width: (popup_w),
                        height: 50.0,
                    },
                );
                filter_popup
                    .label(id!(lbl_loading))
                    .set_text(cx, "Loading…");
                filter_popup
                    .portal_list(id!(distinct_list))
                    .set_visible(cx, false);
            }
        } else {
            filter_popup.set_visible(cx, false);
        }

        // 4. Sync the veil over the old rows and the loading card
        let veil = self.view.view(id!(veil));
        let veil_alpha = (1.0 - view.rows_alpha) as f32;
        veil.set_visible(cx, veil_alpha > 0.0);
        veil.apply_over(
            cx,
            live! { draw_bg: { color: (vec4(0.118, 0.122, 0.133, veil_alpha)) } },
        );

        let loading_overlay = self.view.view(id!(loading_overlay));
        if let Some(ov) = &view.overlay {
            loading_overlay.set_visible(cx, true);
            let opacity = ov.opacity as f32;
            let card = loading_overlay.view(id!(card));
            let card_bg = vec4(0.169, 0.176, 0.188, opacity);
            let border_col = vec4(0.306, 0.3176, 0.341, opacity);
            card.apply_over(
                cx,
                live! { draw_bg: { color: (card_bg), border_color: (border_col) } },
            );

            let t = cx.seconds_since_app_start();
            let spinner = card.view(id!(spinner));
            spinner.apply_over(cx, live! { draw_bg: { spin: (t) } });

            let title_col = vec4(0.875, 0.882, 0.898, opacity);
            let detail_col = vec4(0.616, 0.627, 0.659, opacity);
            card.label(id!(lbl_title))
                .apply_over(cx, live! { draw_text: { color: (title_col) } });
            card.label(id!(lbl_title)).set_text(cx, &ov.title);
            card.label(id!(lbl_detail))
                .apply_over(cx, live! { draw_text: { color: (detail_col) } });
            card.label(id!(lbl_detail)).set_text(cx, &ov.detail);
        } else {
            loading_overlay.set_visible(cx, false);
        }

        // 5. Virtualized PortalList drawing
        let new_row = state.edit.as_ref().is_some_and(|e| e.index.is_none());
        let screen_total = live.screen().total.unwrap_or(0);
        let total = screen_total + usize::from(new_row);
        self.shown_total = total;
        let distinct_list_uid = self.view.portal_list(id!(distinct_list)).widget_uid();
        let rows_list = self.view.portal_list(id!(list));
        if let Some(first) = self.pending_first.take() {
            rows_list.set_first_id_and_scroll(first, 0.0);
        }
        if let Some(first) = first_in_range(rows_list.first_id(), total) {
            rows_list.set_first_id_and_scroll(first, 0.0);
        }

        while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut portal) = step.as_portal_list().borrow_mut() {
                if portal.widget_uid() == distinct_list_uid {
                    if let Some(vals) = &distinct_values {
                        portal.set_item_range(cx, 0, vals.len());
                        while let Some(i) = portal.next_visible_item(cx) {
                            if let Some(val_opt) = vals.get(i) {
                                let (row, _) = portal.item_with_existed(cx, i, live_id!(row));
                                let check = row.as_check_box();
                                let label = match val_opt {
                                    Some(s) if !s.is_empty() => s.as_str(),
                                    _ => "(empty)",
                                };
                                check.set_text(label);
                                let is_on = current_checked_list.contains(val_opt);
                                if check.active(cx) != is_on {
                                    check.set_active(cx, is_on);
                                }
                                row.draw_all(cx, scope);
                            }
                        }
                    } else {
                        portal.set_item_range(cx, 0, 0);
                    }
                    continue;
                }
                let mut list = portal;
                list.set_item_range(cx, 0, total);
                // The editor and the error box follow the edited row only while it is drawn
                // inside the rows' area; scrolled away, they hide.
                self.open_cell_rect = None;
                self.error_cell_rect = None;
                let rows_area = self.body_rect;
                let in_rows = |r: Rect| {
                    r.pos.y >= rows_area.pos.y - 1.0
                        && r.pos.y + r.size.y <= rows_area.pos.y + rows_area.size.y + 1.0
                };
                let mut first_visible = None;
                let mut last_visible = None;

                // At the end of the list the PortalList draws its last item FIRST and then
                // the ones above it: the visible range is the lowest and highest item seen.
                while let Some(i) = list.next_visible_item(cx) {
                    first_visible = Some(first_visible.map_or(i, |f: usize| f.min(i)));
                    last_visible = Some(last_visible.map_or(i, |l: usize| l.max(i)));

                    let item_lines = line_for_item(&view, self.requested.0, i);
                    let template = match item_lines.row {
                        Some(ViewLine::Row { selected: true, .. }) => live_id!(item_selected),
                        Some(ViewLine::Row { odd: true, .. }) => live_id!(item_odd),
                        _ => live_id!(item),
                    };
                    let (item, existed) = list.item_with_existed(cx, i, template);

                    // Stack headers above row
                    for h_idx in 0..4 {
                        let h_view = item.view(&[header_id(h_idx)]);
                        if h_idx < item_lines.headers.len() {
                            if let ViewLine::Header {
                                level,
                                title,
                                count,
                                totals,
                            } = item_lines.headers[h_idx]
                            {
                                h_view.set_visible(cx, true);
                                let pad_left = (*level as f64) * 16.0 + 8.0;
                                h_view.apply_over(cx, live! { padding: { left: (pad_left) } });
                                h_view.label(id!(lbl_title)).set_text(cx, title);
                                h_view.label(id!(lbl_count)).set_text(cx, count);
                                h_view.label(id!(lbl_totals)).set_text(cx, totals);
                            }
                        } else {
                            h_view.set_visible(cx, false);
                        }
                    }

                    // Populate and style row
                    let row_view = item.view(id!(row));

                    if let Some(ViewLine::Row {
                        id: row_id,
                        cells,
                        raw_cells,
                        selected,
                        ..
                    }) = item_lines.row
                    {
                        let cols = row_view.view(id!(cols));
                        let mut candidate_hints = Vec::new();
                        let mut candidate_links = Vec::new();
                        let needs_styling = !existed || !self.styled_rows.contains(&i);
                        if needs_styling {
                            self.styled_rows.insert(i);
                            row_view.apply_over(cx, live! { height: (self.row_height) });
                        }
                        row_view.view(id!(gutter)).set_visible(cx, view.gutter);
                        if view.gutter
                            && let Some(ViewLine::Row { marker, .. }) = item_lines.row
                        {
                            let mark_box = row_view.view(id!(gutter)).view(id!(mark_box));
                            mark_box.set_visible(cx, !marker.is_empty());
                            mark_box.label(id!(mark)).set_text(cx, marker);
                        }
                        for c_idx in 0..MAX_COLUMNS {
                            let cell = cols.view(&[cell_id(c_idx)]);
                            if c_idx < num_cols
                                && let Some(col) = ordered_cols.get(c_idx)
                            {
                                cell.set_visible(cx, true);
                                let w = widths.get(c_idx).copied().unwrap_or(MIN_COLUMN_WIDTH);
                                apply_cell_walk(
                                    cx,
                                    &cell,
                                    w,
                                    fills(c_idx),
                                    col.align,
                                    Some(col.valign),
                                );
                                let orig_col_idx = state.column_order.get(c_idx).copied();
                                let cell_style =
                                    orig_col_idx.and_then(|ci| state.cell_style(*row_id, ci));
                                let raw_cell = raw_cells.get(c_idx).and_then(|opt| opt.as_deref());
                                let default_text =
                                    cells.get(c_idx).map(|s| s.as_str()).unwrap_or("");
                                let full_text = match cell_style {
                                    Some(cs) if !cs.text.is_empty() => cs.text.as_str(),
                                    _ => default_text,
                                };
                                let pad_h = match col.padding {
                                    Some(pad) => (pad.left + pad.right) as f64,
                                    None => 16.0,
                                };
                                let max_w = (w - pad_h).max(0.0);
                                // Column styling is applied ONCE when the item is
                                // created or when columns change (never per frame: see the template note).
                                // The look a style rule / OnDrawCell gives this cell is applied
                                // when it changes (a row scrolled in, a value edited), never per frame.
                                let style_key: Option<AppliedStyle> = cell_style
                                    .map(|cs| (cs.color, cs.text_color, cs.bold, cs.italic));
                                let font_label = match style_key.map(|k| (k.2, k.3)) {
                                    Some((true, true)) => live_id!(lbl_bi),
                                    Some((true, false)) => live_id!(lbl_b),
                                    Some((false, true)) => live_id!(lbl_i),
                                    _ => live_id!(lbl),
                                };
                                let restyle = needs_styling
                                    || self.applied_styles.get(&(i, c_idx)) != Some(&style_key);
                                if restyle {
                                    self.applied_styles.insert((i, c_idx), style_key);
                                    let rgba = |c: crawrad_runtime::grid::Color| {
                                        vec4(
                                            c.r as f32 / 255.0,
                                            c.g as f32 / 255.0,
                                            c.b as f32 / 255.0,
                                            c.a as f32 / 255.0,
                                        )
                                    };
                                    let s = if col.font.size > 0.0 {
                                        col.font.size as f64
                                    } else {
                                        9.0
                                    };
                                    let labels = [
                                        live_id!(lbl),
                                        live_id!(lbl_b),
                                        live_id!(lbl_i),
                                        live_id!(lbl_bi),
                                    ];
                                    if !*selected {
                                        let bg =
                                            cell_style
                                                .and_then(|cs| cs.color)
                                                .or((col.color.a > 0).then_some(col.color));
                                        match bg {
                                            Some(c) => {
                                                let bg = rgba(c);
                                                cell.apply_over(cx, live! { draw_bg: { color: (bg) } });
                                            }
                                            None => cell.apply_over(
                                                cx,
                                                live! { draw_bg: { color: (vec4(0.0, 0.0, 0.0, 0.0)) } },
                                            ),
                                        }
                                        let tc = cell_style
                                            .and_then(|cs| cs.text_color)
                                            .or((col.font.color.a > 0).then_some(col.font.color))
                                            .or((col.text_color.a > 0).then_some(col.text_color))
                                            .map_or(vec4(0.875, 0.882, 0.898, 1.0), rgba);
                                        for l in labels {
                                            cell.label(&[l]).apply_over(
                                                cx,
                                                live! { draw_text: { text_style: { font_size: (s) }, color: (tc) } },
                                            );
                                        }
                                    } else if col.font.size > 0.0 {
                                        for l in labels {
                                            cell.label(&[l]).apply_over(
                                                cx,
                                                live! { draw_text: { text_style: { font_size: (s) } } },
                                            );
                                        }
                                    }
                                }
                                if needs_styling {
                                    if let Some(pad) = col.padding {
                                        let pl = pad.left as f64;
                                        let pt = pad.top as f64;
                                        let pr = pad.right as f64;
                                        let pb = pad.bottom as f64;
                                        cell.apply_over(
                                            cx,
                                            live! { padding: { left: (pl), top: (pt), right: (pr), bottom: (pb) } },
                                        );
                                    }
                                    // The renderer's own label takes the column font size too.
                                    if col.font.size > 0.0 {
                                        let s = col.font.size as f64;
                                        for path in [
                                            &[live_id!(pb_track), live_id!(pb_lbl)],
                                            &[live_id!(swatch_row), live_id!(swatch_lbl)],
                                            &[live_id!(link_view), live_id!(link_lbl)],
                                        ] {
                                            cell.label(path).apply_over(
                                                cx,
                                                live! { draw_text: { text_style: { font_size: (s) } } },
                                            );
                                        }
                                    }
                                }

                                self.draw_measure.text_style.font_size = if col.font.size > 0.0 {
                                    col.font.size
                                } else {
                                    9.0
                                };
                                let pad_v = match col.padding {
                                    Some(pad) => (pad.top + pad.bottom) as f64,
                                    None => 0.0,
                                };
                                let avail_h = (self.row_height - pad_v).max(0.0);
                                let line_h = pen_line_height(&mut self.draw_measure, cx);
                                let max_lines = (avail_h / line_h).floor().max(1.0) as usize;

                                let shown_text = if col.word_wrap {
                                    let lines = crawrad_runtime::grid::wrap_text(
                                        full_text,
                                        max_w,
                                        max_lines,
                                        col.ellipsis,
                                        |s| pen_width(&mut self.draw_measure, cx, s),
                                    );
                                    lines.join("\n")
                                } else {
                                    crawrad_runtime::grid::fit_text(
                                        full_text,
                                        max_w,
                                        col.ellipsis,
                                        |s| pen_width(&mut self.draw_measure, cx, s),
                                    )
                                };

                                // The part that draws this cell: the renderer's own
                                // when the raw value suits it, else the plain label.
                                let part = cell_part(col.renderer, raw_cell);
                                for (child, on) in [
                                    (id!(lbl), part == CellPart::Label),
                                    (id!(cb_box), part == CellPart::CheckBox),
                                    (id!(pb_track), part == CellPart::ProgressBar),
                                    (id!(swatch_row), part == CellPart::ColourSwatch),
                                    (id!(link_view), part == CellPart::Link),
                                ] {
                                    cell.widget(child).set_visible(cx, on);
                                }
                                // A value-driven look (bar fill, swatch colour) follows the
                                // row's data: re-applied only when it changed, never every frame.
                                let value_key = raw_cell.unwrap_or_default();
                                let value_changed = needs_styling
                                    || self.renderer_values.get(&(i, c_idx)).map(String::as_str)
                                        != Some(value_key);
                                if value_changed {
                                    self.renderer_values
                                        .insert((i, c_idx), value_key.to_string());
                                }
                                let label_text = match part {
                                    CellPart::CheckBox => {
                                        let checked =
                                            raw_cell.and_then(checkbox_state).unwrap_or(false);
                                        cell.view(id!(cb_box))
                                            .view(id!(cb_check))
                                            .set_visible(cx, checked);
                                        None
                                    }
                                    CellPart::ProgressBar => {
                                        if value_changed {
                                            let frac =
                                                raw_cell.and_then(progress_fraction).unwrap_or(0.0);
                                            let fill_w = max_w * frac;
                                            cell.view(id!(pb_track))
                                                .view(id!(pb_fill_row.pb_fill))
                                                .apply_over(cx, live! { width: (fill_w) });
                                        }
                                        cell.view(id!(pb_track))
                                            .label(id!(pb_lbl))
                                            .set_text(cx, full_text);
                                        None
                                    }
                                    CellPart::ColourSwatch => {
                                        if value_changed
                                            && let Some(c) = raw_cell.and_then(parse_hex_color)
                                        {
                                            let swatch_col = vec4(
                                                c.r as f32 / 255.0,
                                                c.g as f32 / 255.0,
                                                c.b as f32 / 255.0,
                                                c.a as f32 / 255.0,
                                            );
                                            cell.view(id!(swatch_row))
                                                .view(id!(swatch_box))
                                                .apply_over(
                                                    cx,
                                                    live! { draw_bg: { color: (swatch_col) } },
                                                );
                                        }
                                        let swatch_text = crawrad_runtime::grid::fit_text(
                                            full_text,
                                            (max_w - 18.0).max(0.0),
                                            col.ellipsis,
                                            |s| pen_width(&mut self.draw_measure, cx, s),
                                        );
                                        cell.view(id!(swatch_row))
                                            .label(id!(swatch_lbl))
                                            .set_text(cx, &swatch_text);
                                        Some(swatch_text)
                                    }
                                    CellPart::Link => {
                                        cell.view(id!(link_view))
                                            .label(id!(link_lbl))
                                            .set_text(cx, &shown_text);
                                        if let Some(raw_val) = raw_cell {
                                            candidate_links.push((c_idx, raw_val.to_string()));
                                        }
                                        Some(shown_text)
                                    }
                                    CellPart::Label => {
                                        for face in [
                                            live_id!(lbl),
                                            live_id!(lbl_b),
                                            live_id!(lbl_i),
                                            live_id!(lbl_bi),
                                        ] {
                                            let text = if face == font_label {
                                                shown_text.as_str()
                                            } else {
                                                ""
                                            };
                                            cell.label(&[face]).set_text(cx, text);
                                        }
                                        Some(shown_text)
                                    }
                                };
                                if let Some(shown) = label_text
                                    && col.cell_hint
                                    && shown.chars().count() < full_text.chars().count()
                                {
                                    candidate_hints.push((c_idx, full_text.to_string()));
                                }
                            } else {
                                cell.set_visible(cx, false);
                            }
                        }
                        item.draw_all(cx, scope);
                        if i == state.rec_no {
                            if let Some(open_col) = view.open_editor {
                                let rect = cols.view(&[cell_id(open_col)]).area().rect(cx);
                                self.open_cell_rect = in_rows(rect).then_some(rect);
                            }
                            if let Some((err_col, _)) = &view.error_cell {
                                let rect = cols.view(&[cell_id(*err_col)]).area().rect(cx);
                                self.error_cell_rect = in_rows(rect).then_some(rect);
                            }
                        }
                        for (c_idx, full_text) in candidate_hints {
                            let cell = cols.view(&[cell_id(c_idx)]);
                            self.hint_cells
                                .push((cell.area().rect(cx), full_text, i, c_idx));
                        }
                        for (c_idx, raw_val) in candidate_links {
                            let cell = cols.view(&[cell_id(c_idx)]);
                            self.link_cells.push((cell.area().rect(cx), raw_val, i));
                        }
                    } else {
                        let cols = row_view.view(id!(cols));
                        for c_idx in 0..MAX_COLUMNS {
                            let cell = cols.view(&[cell_id(c_idx)]);
                            cell.set_visible(cx, false);
                        }
                        item.draw_all(cx, scope);
                    }
                }
                // The page asked for starts at the top visible item (rows outside it draw blank).
                if let (Some(top), Some(bottom)) = (first_visible, last_visible)
                    && (top != self.requested.0 || bottom >= self.requested.0 + PAGE_ROWS)
                {
                    self.requested = (top, PAGE_ROWS);
                    live.show(top, PAGE_ROWS);
                }

                // The list is borrowed while `portal` lives: looking widgets up through `self.view` now panics.
                drop(list);

                // After a write, once its rows are read again and the list holds them: the
                // user's row is brought into view (the answer arrives before the rows).
                if self.reveal_rec
                    && live.screen().pending.is_none()
                    && live.screen().total.is_some_and(|t| state.rec_no < t)
                {
                    self.reveal_rec = false;
                    let list = self.view.portal_list(id!(list));
                    let fits = rows_that_fit(self.body_rect.size.y, self.row_height);
                    if let Some(first) = scroll_target(list.first_id(), fits, state.rec_no) {
                        self.pending_first = Some(first);
                        cx.new_next_frame();
                    }
                }
                let cell_editor = self.view.view(id!(cell_editor));
                // `body` is still being laid out here (its rect reads zero): the overlays are
                // placed against where it was last frame.
                let body_rect = self.body_rect;
                if let Some(open_col) = view.open_editor
                    && let Some(cell_rect) = self.open_cell_rect
                {
                    let col = ordered_cols.get(open_col);
                    let editor_kind = col.map(effective_editor);
                    let x = cell_rect.pos.x - body_rect.pos.x;
                    let y = cell_rect.pos.y - body_rect.pos.y;
                    let w = cell_rect.size.x;
                    let h = cell_rect.size.y;
                    let is_list = editor_kind == Some("edList");
                    let is_check = editor_kind == Some("edCheck");
                    let is_button = editor_kind == Some("edButton");
                    let is_lookup = editor_kind == Some("edLookup");

                    if is_check {
                        cell_editor.set_visible(cx, false);
                        self.lookup_scroll = LookupScroll::default();
                        self.last_editor_rect = None;
                        self.last_open_cell = None;
                    } else {
                        let mut list_above = false;
                        let mut list_height = 0.0;
                        let (target_y, target_h) = if is_list {
                            let options = col.map(|c| c.pick_list.as_slice()).unwrap_or(&[]);
                            let count = options.len().min(12);
                            let lh = (count as f64 * 22.0).max(22.0);
                            (y + h, lh)
                        } else if is_lookup {
                            // The list floats above the grid like a combo's popup: it is not held
                            // to the rows' area.
                            let fit = col.map(|c| c.drop_down_count).unwrap_or(8);
                            let (len, highlighted) = state
                                .edit
                                .as_ref()
                                .and_then(|e| e.lookup.as_ref())
                                .map(|l| (l.rows.len(), l.highlighted))
                                .unwrap_or((0, 0));
                            let follow = self.lookup_scroll.last_highlight != Some(highlighted);
                            self.lookup_scroll.last_highlight = Some(highlighted);
                            let (start, n) = lookup_window(
                                len,
                                highlighted,
                                fit,
                                self.lookup_scroll.start,
                                follow,
                            );
                            self.lookup_scroll.start = start;
                            self.lookup_scroll.len_shown = (len, n);
                            if state.edit.as_ref().is_some_and(|e| e.lookup.is_some()) {
                                let list_h = 2.0 * LOOKUP_LINE_HEIGHT
                                    + n as f64 * LOOKUP_ROW_HEIGHT
                                    + 2.0 * LOOKUP_FRAME;
                                let window_h = cx.current_pass_size().y;
                                list_above =
                                    lookup_opens_above(cell_rect.pos.y, h, list_h, window_h);
                                list_height = list_h;
                                if list_above {
                                    (y - list_h, h + list_h)
                                } else {
                                    (y, h + list_h)
                                }
                            } else {
                                (y, h)
                            }
                        } else {
                            (y, h)
                        };
                        // An open list is wider than a narrow cell, pulled left at the edge.
                        let listing =
                            is_lookup && state.edit.as_ref().is_some_and(|e| e.lookup.is_some());
                        let (target_x, target_w) = if listing {
                            let lw = w.max(LOOKUP_MIN_WIDTH);
                            (error_box_left(x, lw, body_rect.size.x), lw)
                        } else {
                            (x, w)
                        };
                        let target_rect = Rect {
                            pos: dvec2(target_x, target_y),
                            size: dvec2(target_w, target_h),
                        };
                        let (ed_top, list_top) = if list_above {
                            (list_height, 0.0)
                        } else {
                            (0.0, h)
                        };
                        if self.last_editor_rect != Some(target_rect) {
                            self.last_editor_rect = Some(target_rect);
                            cell_editor.apply_over(
                                cx,
                                live! {
                                    margin: {
                                        left: (target_x + self.body_offset.x),
                                        top: (target_y + self.body_offset.y)
                                    },
                                    width: (target_w),
                                    height: (target_h)
                                },
                            );
                            // The editor itself stays on the cell; only the list is wider.
                            let ed_left = x - target_x;
                            cell_editor.view(id!(ed_box)).apply_over(
                                cx,
                                live! {
                                    margin: { left: (ed_left), top: (ed_top) },
                                    width: (w),
                                    height: (h)
                                },
                            );
                            cell_editor
                                .view(id!(lookup_box))
                                .apply_over(cx, live! { margin: { top: (list_top) } });
                        }
                        cell_editor.set_visible(cx, true);

                        let ed = cell_editor.text_input(id!(ed));
                        let btn_dots = cell_editor.button(id!(btn_dots));
                        let btn_arrow = cell_editor.button(id!(btn_arrow));
                        let footer = cell_editor.view(id!(lookup_box));

                        btn_dots.set_visible(cx, is_button);
                        cell_editor
                            .view(id!(ed_sep))
                            .set_visible(cx, is_button || is_lookup);
                        btn_arrow.set_visible(cx, is_lookup);

                        if is_list {
                            cell_editor.view(id!(ed_box)).set_visible(cx, false);
                            footer.set_visible(cx, false);
                            self.lookup_scroll = LookupScroll::default();
                            let options = col.map(|c| c.pick_list.as_slice()).unwrap_or(&[]);
                            for idx in 0..12 {
                                let btn = cell_editor.button(&[pick_id(idx)]);
                                match options.get(idx) {
                                    Some(option) => {
                                        btn.set_visible(cx, true);
                                        btn.set_text(cx, option);
                                    }
                                    None => btn.set_visible(cx, false),
                                }
                            }
                        } else if is_lookup {
                            cell_editor.view(id!(ed_box)).set_visible(cx, true);
                            let lookup_opt = state.edit.as_ref().and_then(|e| e.lookup.as_ref());
                            for idx in 0..12 {
                                cell_editor.button(&[pick_id(idx)]).set_visible(cx, false);
                            }
                            if let Some(lookup_list) = lookup_opt {
                                let names =
                                    col.map(|c| c.lookup_display.clone()).unwrap_or_default();
                                let shown: Vec<Vec<String>> =
                                    lookup_list.rows.iter().map(|(_, d)| d.clone()).collect();
                                let (header, lines) = lookup_lines(&names, &shown);
                                let room_rows =
                                    ((target_h - h - 2.0 * LOOKUP_LINE_HEIGHT - 2.0 * LOOKUP_FRAME)
                                        / LOOKUP_ROW_HEIGHT)
                                        .round()
                                        .max(0.0) as usize;
                                let start = self.lookup_scroll.start;
                                footer.label(id!(lookup_header)).set_text(cx, &header);
                                for idx in 0..LOOKUP_ROWS {
                                    let row = cell_editor.view(&[lookup_row_id(idx)]);
                                    match lines.get(start + idx).filter(|_| idx < room_rows) {
                                        Some(text) => {
                                            row.set_visible(cx, true);
                                            row.button(id!(btn)).set_text(cx, text);
                                            let bg = if start + idx == lookup_list.highlighted {
                                                vec4(0.18, 0.263, 0.42, 1.0)
                                            } else {
                                                vec4(0.118, 0.118, 0.18, 1.0)
                                            };
                                            row.apply_over(cx, live! { draw_bg: { color: (bg) } });
                                        }
                                        None => row.set_visible(cx, false),
                                    }
                                }
                                let (len, n) = self.lookup_scroll.len_shown;
                                let track = footer.view(id!(lk_track));
                                let track_h = n as f64 * LOOKUP_ROW_HEIGHT;
                                match lookup_thumb(len, n, start, track_h) {
                                    Some((top, thumb_h)) => {
                                        track.set_visible(cx, true);
                                        track.apply_over(cx, live! { height: (track_h) });
                                        track.view(id!(lk_thumb)).apply_over(
                                            cx,
                                            live! { margin: { top: (top) }, height: (thumb_h) },
                                        );
                                    }
                                    None => track.set_visible(cx, false),
                                }
                                footer.set_visible(cx, true);
                                let shown_n = lookup_list.rows.len();
                                let footer_str = match lookup_list.total_hint {
                                    Some(total_m) => format!(
                                        "{shown_n} of {total_m} \u{b7} \u{2191}\u{2193} choose \u{b7} Enter picks \u{b7} Esc closes"
                                    ),
                                    None => format!(
                                        "{shown_n} \u{b7} \u{2191}\u{2193} choose \u{b7} Enter picks \u{b7} Esc closes"
                                    ),
                                };
                                footer
                                    .label(id!(lbl_lookup_footer))
                                    .set_text(cx, &footer_str);
                            } else {
                                footer.set_visible(cx, false);
                                self.lookup_scroll = LookupScroll::default();
                                // A pick puts the label in the editor: the box shows it too.
                                let want = state
                                    .edit
                                    .as_ref()
                                    .map(|e| e.editor_text.as_str())
                                    .unwrap_or("");
                                if ed.text() != want {
                                    ed.set_text(cx, want);
                                }
                            }
                        } else {
                            cell_editor.view(id!(ed_box)).set_visible(cx, true);
                            footer.set_visible(cx, false);
                            self.lookup_scroll = LookupScroll::default();
                            for idx in 0..12 {
                                cell_editor.button(&[pick_id(idx)]).set_visible(cx, false);
                            }
                            // What an OnButtonClick left in the text shows in the box at once.
                            if is_button {
                                let want = state
                                    .edit
                                    .as_ref()
                                    .map(|e| e.editor_text.as_str())
                                    .unwrap_or("");
                                if ed.text() != want {
                                    ed.set_text(cx, want);
                                }
                            }
                        }

                        if !is_list {
                            let current_open = (state.rec_no, open_col);
                            if self.last_open_cell != Some(current_open) {
                                self.last_open_cell = Some(current_open);
                                let editor_text = state
                                    .edit
                                    .as_ref()
                                    .map(|e| e.editor_text.as_str())
                                    .unwrap_or("");
                                let is_number = editor_kind == Some("edNumber");
                                let ax = if is_number { 1.0 } else { 0.0 };
                                ed.apply_over(cx, live! { label_align: { x: (ax) } });
                                ed.set_text(cx, editor_text);
                                ed.set_cursor(
                                    cx,
                                    makepad_widgets::makepad_draw::text::selection::Cursor {
                                        index: editor_text.len(),
                                        prefer_next_row: false,
                                    },
                                    false,
                                );
                                // Its area exists only once it is drawn: the focus is given on
                                // the next event (`handle_event`).
                                self.focus_editor = true;
                                cx.new_next_frame();
                            }
                        }
                    }
                } else {
                    cell_editor.set_visible(cx, false);
                    self.lookup_scroll = LookupScroll::default();
                    self.last_editor_rect = None;
                    self.last_open_cell = None;
                    // A hidden editor must not keep the keyboard: the grid's keys come back.
                    if cell_editor.text_input(id!(ed)).key_focus(cx) {
                        cx.set_key_focus(Area::Empty);
                    }
                }

                // Error overlay
                let edit_error_frame = self.view.view(id!(edit_error_frame));
                let edit_error = self.view.view(id!(edit_error));
                if let Some((_err_col, message)) = &view.error_cell
                    && let Some(cell_rect) = self.error_cell_rect
                {
                    let x = cell_rect.pos.x - body_rect.pos.x;
                    let y = cell_rect.pos.y - body_rect.pos.y;
                    let w = cell_rect.size.x;
                    let h = cell_rect.size.y;
                    let frame_rect = Rect {
                        pos: dvec2(x, y),
                        size: dvec2(w, h),
                    };
                    if self.last_error_rect != Some(frame_rect) {
                        self.last_error_rect = Some(frame_rect);
                        edit_error_frame.apply_over(
                            cx,
                            live! { margin: { left: (x), top: (y) }, width: (w), height: (h) },
                        );
                        let err_w = w.max(160.0);
                        // Under the cell, or over it when the rows' area has no room below.
                        let err_top = error_box_top(y, h, body_rect.size.y);
                        edit_error.apply_over(
                            cx,
                            live! { margin: { left: (error_box_left(x, err_w, body_rect.size.x)), top: (err_top) }, width: (err_w) },
                        );
                    }
                    edit_error.label(id!(lbl_msg)).set_text(cx, message);
                    edit_error_frame.set_visible(cx, true);
                    edit_error.set_visible(cx, true);
                } else {
                    edit_error_frame.set_visible(cx, false);
                    edit_error.set_visible(cx, false);
                    self.last_error_rect = None;
                }
            }
        }
        // The header's width is known only after it is laid out (before, it reads
        // 0): keep it for the next frame, and draw once more when it changed.
        let body_rect = self.view.view(id!(body)).area().rect(cx);
        let body_offset = body_rect.pos - self.view.area().rect(cx).pos;
        if body_rect != self.body_rect || body_offset != self.body_offset {
            self.body_rect = body_rect;
            self.body_offset = body_offset;
            self.last_editor_rect = None;
            self.last_error_rect = None;
            self.view.redraw(cx);
        }
        let laid_out = self.view.view(id!(header)).area().rect(cx).size.x;
        if laid_out > 0.0 && laid_out != self.shared_width {
            self.shared_width = laid_out;
            // A redraw asked inside the draw is dropped: draw again on the next
            // frame, or the columns keep the widths of the 0-wide first draw.
            self.width_changed = true;
            cx.new_next_frame();
        }
        DrawStep::done()
    }
}

impl CrawradDataGridRef {
    #[note("data-grid-crawraddatagridref-set-source")]
    pub fn set_source(
        &self,
        cx: &mut Cx,
        source: Arc<dyn datagrid::RowSource>,
        columns: Vec<GridColumn>,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            let mut state = GridState::new(columns);
            state.writes_through(source.as_ref());
            state.marks.edited_row = MAKEPAD_EDITED_ROW;
            state.marks.new_row = MAKEPAD_NEW_ROW;
            if let Some(timing) = inner.custom_timing {
                state.timing = timing;
            }
            if let Some(dim) = inner.custom_dim {
                state.dim = dim;
            }
            let mut live = LiveRows::spawn(source, state.query(), 200, 8);
            live.show(0, 200);
            inner.state = Some(state);
            inner.live = Some(live);
            inner.reported_rec_no = Some(0);
            inner.requested = (0, 200);
            inner.headers_styled = false;
            inner.styled_rows.clear();
            inner.renderer_values.clear();
            inner.applied_styles.clear();
            inner.open_list = None;
            inner.needs_auto_width = true;
            inner.view.redraw(cx);
        }
    }

    #[note("data-grid-crawraddatagridref-set-timing")]
    pub fn set_timing(&self, timing: crawrad_component::BusyTiming) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.custom_timing = Some(timing);
            if let Some(state) = &mut inner.state {
                state.timing = timing;
            }
        }
    }

    #[note("data-grid-crawraddatagridref-set-dim")]
    pub fn set_dim(&self, dim: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.custom_dim = Some(dim);
            if let Some(state) = &mut inner.state {
                state.dim = dim;
            }
        }
    }

    /// Shows or hides the "Group by" bar, the grid's `GroupBar`.
    pub fn set_group_bar(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.view.view(id!(group_bar)).set_visible(cx, visible);
            inner.view.redraw(cx);
        }
    }

    /// Sets the row height of the grid.
    pub fn set_row_height(&self, row_height: f64) {
        if let Some(mut inner) = self.borrow_mut()
            && (inner.row_height - row_height).abs() > 0.001
        {
            inner.row_height = row_height;
            inner.styled_rows.clear();
            inner.renderer_values.clear();
            inner.applied_styles.clear();
        }
    }

    #[note("data-grid-crawraddatagridref-rec-no")]
    pub fn rec_no(&self) -> usize {
        self.borrow()
            .and_then(|inner| inner.state.as_ref().map(|s| s.rec_no))
            .unwrap_or(0)
    }

    /// Makes `rec_no` the grid's current row without announcing it as `RowSelected`:
    /// the caller (a bound dataset) already stands on that row. The row comes into view
    /// once the grid has read it.
    pub fn set_rec_no(&self, cx: &mut Cx, rec_no: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            let Some(state) = inner.state.as_mut() else {
                return;
            };
            state.rec_no = rec_no;
            inner.reported_rec_no = Some(rec_no);
            inner.reveal_rec = true;
            inner.view.redraw(cx);
        }
    }

    /// The first of this grid's actions that `pick` accepts.
    fn find_action<T>(
        &self,
        actions: &Actions,
        pick: impl Fn(CrawradDataGridAction) -> Option<T>,
    ) -> Option<T> {
        let uid = self.widget_uid();
        actions
            .iter()
            .filter_map(|a| a.as_widget_action())
            .filter(|a| a.widget_uid == uid)
            .find_map(|a| pick(a.cast()))
    }

    /// The grid's current row moved to this row (a click, a key).
    pub fn row_selected(&self, actions: &Actions) -> Option<usize> {
        self.find_action(actions, |a| match a {
            CrawradDataGridAction::RowSelected(row) => Some(row),
            _ => None,
        })
    }

    /// The pointer rests on header `col`, whose `TitleHint` is the text.
    pub fn header_hover(&self, actions: &Actions) -> Option<(usize, String, DVec2)> {
        self.find_action(actions, |a| match a {
            CrawradDataGridAction::HeaderHover(col, hint, pos) => Some((col, hint, pos)),
            _ => None,
        })
    }

    /// The pointer rests on a shortened cell; the text is its full value.
    pub fn cell_hover(&self, actions: &Actions) -> Option<(usize, usize, String, DVec2)> {
        self.find_action(actions, |a| match a {
            CrawradDataGridAction::CellHover(row, col, text, pos) => Some((row, col, text, pos)),
            _ => None,
        })
    }

    /// The pointer left every header and cell hint.
    pub fn hover_out(&self, actions: &Actions) -> bool {
        self.find_action(actions, |a| {
            matches!(a, CrawradDataGridAction::HoverOut).then_some(())
        })
        .is_some()
    }

    /// Requests custom display texts for visible rows from columns with `get_text_hook`.
    pub fn text_requests(&self) -> Vec<crawrad_runtime::grid::TextRequest> {
        let Some(mut inner) = self.borrow_mut() else {
            return Vec::new();
        };
        let inner = &mut *inner;
        let (state, live) = match (&mut inner.state, &inner.live) {
            (Some(s), Some(l)) => (s, l),
            _ => return Vec::new(),
        };
        state.text_requests(live.screen())
    }

    /// Stores answered custom texts for cells.
    pub fn set_texts(&self, cx: &mut Cx, reqs: Vec<crawrad_runtime::grid::TextRequest>) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        let inner = &mut *inner;
        if let Some(state) = &mut inner.state {
            state.set_texts(reqs);
            inner.view.redraw(cx);
        }
    }

    /// Requests custom cell styles for visible rows from columns with `draw_cell_hook` or `style_rules`.
    pub fn style_requests(&self) -> Vec<crawrad_runtime::grid::StyleRequest> {
        let Some(mut inner) = self.borrow_mut() else {
            return Vec::new();
        };
        let inner = &mut *inner;
        let (state, live) = match (&mut inner.state, &inner.live) {
            (Some(s), Some(l)) => (s, l),
            _ => return Vec::new(),
        };
        state.style_requests(live.screen())
    }

    /// Stores answered custom cell styles.
    pub fn set_styles(&self, cx: &mut Cx, reqs: Vec<crawrad_runtime::grid::StyleRequest>) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        let inner = &mut *inner;
        if let Some(state) = &mut inner.state {
            state.set_styles(reqs);
            inner.styled_rows.clear();
            inner.view.redraw(cx);
        }
    }

    /// Returns the active row check waiting for OnSetText / OnValidate answers.
    pub fn pending_check(&self) -> Option<crawrad_runtime::grid::RowCheck> {
        let inner = self.borrow()?;
        let state = inner.state.as_ref()?;
        state.pending_check()
    }

    /// Applies answered cell checks, setting errors or proceeding to write.
    pub fn resolve_check(&self, cx: &mut Cx, check: crawrad_runtime::grid::RowCheck) {
        if let Some(mut inner) = self.borrow_mut() {
            let outcome = if let Some(state) = &mut inner.state {
                state.checked(check)
            } else {
                EditOutcome::Handled
            };
            inner.apply(outcome);
            inner.view.redraw(cx);
        }
    }

    /// Returns the active button request waiting for OnButtonClick.
    pub fn pending_button(&self) -> Option<crawrad_runtime::grid::ButtonRequest> {
        let inner = self.borrow()?;
        let state = inner.state.as_ref()?;
        state.pending_button()
    }

    /// Completes the button request, applying the returned text to the active editor.
    pub fn button_done(&self, cx: &mut Cx, req: crawrad_runtime::grid::ButtonRequest) {
        if let Some(mut inner) = self.borrow_mut() {
            let inner = &mut *inner;
            if let Some(state) = &mut inner.state {
                state.button_done(req);
                inner.view.redraw(cx);
            }
        }
    }
}

/// Which child of a data cell draws it (Renderer).
#[derive(Clone, Copy, PartialEq, Eq)]
enum CellPart {
    Label,
    CheckBox,
    ProgressBar,
    ColourSwatch,
    Link,
}

/// The column's renderer when the raw value suits it; the plain label otherwise
/// (NULL, a non-boolean under CheckBox, a non-number under ProgressBar...).
fn cell_part(renderer: CellRenderer, raw: Option<&str>) -> CellPart {
    match renderer {
        CellRenderer::CheckBox if raw.and_then(checkbox_state).is_some() => CellPart::CheckBox,
        CellRenderer::ProgressBar if raw.and_then(progress_fraction).is_some() => {
            CellPart::ProgressBar
        }
        CellRenderer::ColourSwatch if raw.and_then(parse_hex_color).is_some() => {
            CellPart::ColourSwatch
        }
        CellRenderer::Link => CellPart::Link,
        _ => CellPart::Label,
    }
}

#[cfg(test)]
#[path = "test/data_grid/edit/test.rs"]
mod tests;
