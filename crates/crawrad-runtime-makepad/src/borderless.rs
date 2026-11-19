use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;

    // A button that draws no border in any state; its users give it their own colours.
    pub CrawradBorderlessButton = <Button> {
        draw_bg: {
            border_color_1: #0000, border_color_2: #0000,
            border_color_1_hover: #0000, border_color_2_hover: #0000,
            border_color_1_down: #0000, border_color_2_down: #0000,
            border_color_1_focus: #0000, border_color_2_focus: #0000,
        }
    }
}
