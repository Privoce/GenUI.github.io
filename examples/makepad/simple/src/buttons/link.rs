use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    LinkExample = <SolidView>{
        height: 260,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 16,
        <LinkLabel>{
            text: "Link label inherits Button not Label!",
        }
        // you can add icon too
        <LinkLabel>{
            text: "more styles!",
            draw_icon: {
                svg_file: dep("crate://self/icons/all.svg"),
                fn get_color(self) -> vec4 { 
                    return #00FFFF
                }
            },
            icon_walk: {
                height: 16,
                width: Fit,
                margin: {left: 10.0}
            },
            draw_text: {
                text_style: {
                    font_size: 16,
                },
                fn get_color(self) -> vec4 { 
                    return #00FFFF
                }
            }
        }
    }
}
