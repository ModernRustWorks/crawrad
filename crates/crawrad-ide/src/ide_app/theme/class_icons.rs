use makepad_widgets::*;

live_design! {
    link crawrad_theme;
    use link::theme::*;
    use link::widgets::*;
    use link::shaders::*;
    use link::crawrad_theme::*;
    pub CrawradClassIcon = <View> {
        width: (CRAWRAD_ICONE_GRADE), height: (CRAWRAD_ICONE_GRADE),
        show_bg: true,
        draw_bg: {
            texture image: texture2d
            instance kind: 0.0
            uniform count: (CRAWRAD_ICONE_QUANTIDADE)
            uniform grade: (CRAWRAD_ICONE_GRADE)
            color: (CRAWRAD_TEXTO_TERCIARIO)

            fn pixel(self) -> vec4 {
                if self.kind < -0.5 {
                    return vec4(0.0, 0.0, 0.0, 0.0);
                }
                let u = min(self.rect_size.x, self.rect_size.y) / self.grade;
                let o = (self.rect_size - vec2(self.grade, self.grade) * u) * 0.5;
                let g = (self.pos * self.rect_size - o) / u;
                if g.x < 0.0 || g.y < 0.0 || g.x > self.grade || g.y > self.grade {
                    return vec4(0.0, 0.0, 0.0, 0.0);
                }
                let uv = vec2((self.kind + g.x / self.grade) / self.count, g.y / self.grade);
                let a = sample2d(self.image, uv).x;
                return vec4(self.color.rgb * self.color.a * a, self.color.a * a);
            }
        }
    }
}
