use makepad_widgets::*;

live_design!{
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    const SSPACING_1 = 10
    const COLOR_DOWN_0 = #x00000000
    const COLOR_DOWN_1 = #x00000011
    const COLOR_DOWN_2 = #x00000022
    const COLOR_DOWN_3 = #x00000044
    const COLOR_DOWN_4 = #x00000066
    const COLOR_DOWN_5 = #x000000AA
    const COLOR_DOWN_6 = #x000000CC
    const COLOR_UP_0 = #xFFFFFF00
    const COLOR_UP_1 = #xFFFFFF0A
    const COLOR_UP_2 = #xFFFFFF10
    const COLOR_UP_3 = #xFFFFFF20
    const COLOR_UP_4 = #xFFFFFF40
    const COLOR_UP_5 = #xFFFFFF66
    const COLOR_UP_6 = #xFFFFFF88
    // <View> {
    //     width: 130,
    //     height: Fit,
    //     margin: {top: 0},
    //    /* label = <BarLabel> {
    //         width: Fit,
    //         margin: {left: 5},
    //         align: {x: 1.0}
    //     }*/
    //     input = <FishSlider> {
    //         padding: 0
    //         height: 37,
    //         width: 125,
    //         margin: {top: 1, left: 2}
    //         text: "HELLO WOLD"
    //     }
    // }
    FishSlider = <Slider> {
        height: 36
        text: "CutOff1"
        draw_text: { color: (#0)}
        text_input: {
            cursor_margin_bottom: (SSPACING_1),
            cursor_margin_top: (SSPACING_1),
            select_pad_edges: (SSPACING_1),
            cursor_size: (SSPACING_1),
            empty_message: "0",
            numeric_only: true,
            draw_bg: {
                color: (COLOR_DOWN_0)
            },
            draw_text:{
                color: (#ffff00ff)
            }
        }
        draw_slider: {
            instance line_color: #f00
            instance bipolar: 0.0
            fn pixel(self) -> vec4 {
                let nub_size = 3

                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                let top = 20.0;

                sdf.box(1.0, top, self.rect_size.x - 2, self.rect_size.y - top - 2, 1);
                sdf.fill_keep(
                    mix(
                        mix((COLOR_DOWN_4), (COLOR_DOWN_4) * 0.1, pow(self.pos.y, 1.0)),
                        mix((COLOR_DOWN_4) * 1.75, (COLOR_DOWN_4) * 0.1, pow(self.pos.y, 1.0)),
                        self.drag
                    )
                ) // Control backdrop gradient

                sdf.stroke(mix(mix(#x00000060, #x00000070, self.drag), #xFFFFFF10, pow(self.pos.y, 10.0)), 1.0) // Control outline
                let in_side = 5.0;
                let in_top = 5.0; // Ridge: vertical position
                sdf.rect(1.0 + in_side, top + in_top, self.rect_size.x - 2 - 2 * in_side, 3);
                sdf.fill(mix((COLOR_DOWN_4), #00000088, self.drag)); // Ridge color
                let in_top = 7.0;
                sdf.rect(1.0 + in_side, top + in_top, self.rect_size.x - 2 - 2 * in_side, 3);
                sdf.fill(#FFFFFF18); // Ridge: Rim light catcher

                let nub_x = self.slide_pos * (self.rect_size.x - nub_size - in_side * 2 - 9);
                sdf.move_to(mix(in_side + 3.5, self.rect_size.x * 0.5, self.bipolar), top + in_top);

                sdf.line_to(nub_x + in_side + nub_size * 0.5, top + in_top);
                sdf.stroke_keep(mix((COLOR_UP_0), self.line_color, self.drag), 1.5)
                sdf.stroke(
                    mix(mix(self.line_color * 0.85, self.line_color, self.hover), #xFFFFFF80, self.drag),
                    1
                )

                let nub_x = self.slide_pos * (self.rect_size.x - nub_size - in_side * 2 - 3) - 3;
                sdf.box(nub_x + in_side, top + 1.0, 12, 12, 1.)

                sdf.fill_keep(mix(mix(#x7, #x8, self.hover), #3, self.pos.y)); // Nub background gradient
                sdf.stroke(
                    mix(
                        mix(#xa, #xC, self.hover),
                        #0,
                        pow(self.pos.y, 1.5)
                    ),
                    1.
                ); // Nub outline gradient


                return sdf.result
            }
        }
    }
    ViewExample = <View>{
        show_bg: true,
        // inherits parent width
        width: All,
        // inherits parent height
        height: 100,
        padding: 10.0,
        spacing: 16.0,
        draw_bg: {color: #ADBABD},
        flow: Right,
        <View>{
            height: 30,
            width: 120,
            show_bg: true,
            draw_bg: {color: #FF0000},
        }
        <View>{
            height: 30,
            width: 90,
            show_bg: true,
            draw_bg: {color: #FF00FF},
        }
        <View>{
            height: 30,
            width: 120,
            show_bg: true,
            draw_bg: {color: #FF00FF},
        }
    }
}