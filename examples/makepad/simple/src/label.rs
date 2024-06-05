use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    LabelExample = <SolidView>{
        height: 200,
        width: Fill,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 10,
        <Label>{
            height: 30,
            // if width is not set, it will be calculated based on the text length
            // if width < text length, text will be truncated
            // you can use `\n`... to format the text
            width: 80,
            margin: 10,
            text: "Emoji \nnot allowed",
        }
        <Label>{
            width: 60,
            text: "wrap Ellipsis",
            draw_text: {
                wrap: Ellipsis,
            }
        }
        // if wrap is Word, text will be wrapped by word when text length > width
        // if is Line, no wrap
        <Label>{
            width: 30,
            height: 30,
            text: "wrap Line",
            draw_text: {
                wrap: Word,
            }
        }
        <Label>{
            draw_text: {
                wrap: Word,
                // real font size = font size * font scale
                font_scale: 1.5,
                text_style: {
                    // so here font size = 16 * 1.5 = 24
                    font_size: 16,
                    font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")},
                    // brightness > 1.0 will make the text brighter
                    // < 1.0 will make the text darker
                    brightness: 1.0,
                },
                color: #FF0000,
            }
            text: "other styles",
        }
    }
}

