use makepad_widgets::*;

live_design! {
    link widgets;
    use link::shaders::*;
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

    // A band whose TOP corners are rounded like the code editor's tab bar; the
    // bottom ones stay square so it sits on whatever comes below.
    pub CrawradTopRounded = <View> {
        show_bg: true,
        draw_bg: {
            uniform radius: 5.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y + self.radius * 2.0, self.radius);
                sdf.fill(self.color);
                return sdf.result;
            }
        }
    }
}
