use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    ButtonExample = <SolidView>{
        height: 260,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 16,
        <Button>{
            text: "easy button",
        }
        <Button>{
            text: "Icon btn",
            // override draw_bg
            draw_bg: {
                fn pixel() -> vec4 { return #FF0000 }
            },
            draw_icon: {
                svg_file: dep("crate://self/icons/all.svg"),
                fn get_color(self) -> vec4 { 
                    return #00FFFF
                }
            },
            icon_walk: {
                height: 16,
                width: 16,
                margin: {left: 10.0, right: 10.0},
            }
        }
        <Button>{
            text: "other styles",
            draw_text: {
                text_style: {
                    font_size: 16.0,
                    brightness: 0.8,
                },
                fn get_color(self) -> vec4 { 
                    // if !hover use #00FFFF else #fff
                    // eq: !self.hover ? #00FFFF : #fff 
                    // return mix(
                    //     #00FFFF,
                    //     #fff,
                    //     self.hover
                    // )
                    // !hover ? #FF0000 : (!pressed ? #00FFFF : #0000FF)
                    return mix(
                        #FF0000,
                        mix(
                            #00FFFF,
                            #0000FF,
                            self.pressed
                        ),
                        self.hover
                    )
                }
            },
            padding: {left: 16.0, right: 16.0, top: 6.0, bottom: 6.0},
        }
    }
}
