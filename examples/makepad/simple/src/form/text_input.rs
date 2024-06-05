use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    TextInputExample = <SolidView>{
        height: 260,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 16,
        <TextInput> {
            width: 300, 
            height: Fit,
            text: "Text Input",
            draw_bg: {
                border_width: 1.0
                border_color: #ddd
            }
        }
        <TextInput>{
            text: "Real Text"
            width: 300,
            height: Fit
            draw_bg: {
                color: #1
            }
        }
        <TextInput> {
            height: Fit,
            width: 300,
            empty_message: "Placeholder"
            draw_bg: {
                color:  #444
                border_width: 1.0
                border_color: #x00000044
            }
            ascii_only: true,
        }
        <TextInput> {
            height: Fit,
            width: 300,
            margin: {top: 0.0, left: 0.0, bottom: 0.0, right: 0.0},
            empty_message: "Placeholder"
            draw_bg: {
                color: #666,
                border_width: 1.0,
                border_color: #x00000044,
            }
            draw_text: {
                text_style: {font_size: 16.0}
                fn get_color(self) -> vec4 {
                    return
                    mix(
                        mix(
                            mix(
                                #xFFFFFF55,
                                #xFFFFFF88,
                                self.hover
                            ),
                            #xFFFFFFCC,
                            self.focus
                        ),
                        #xFFFFFF66,
                        self.is_empty
                    )
                }
            }
        }      
    }
}

