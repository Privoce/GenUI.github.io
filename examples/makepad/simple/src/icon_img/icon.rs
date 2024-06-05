use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    IconExample = <SolidView>{
        height: 260,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 16,
        <Icon>{
            draw_icon: {
                svg_file: dep("crate://self/icons/all.svg"),
            }
        }
        
        <Icon>{
            draw_icon: {
                svg_file: dep("crate://self/icons/all.svg"),
                brightness: 1.2,
                // same as: `color: #dddd00,`
                fn get_color(self)-> vec4{
                    return #dddd00
                },
                // size 95% of the original
                scale: 0.95,
            },
            draw_bg: {
                fn pixel(self)-> vec4{
                    return #FF00FF
                }
            },
            icon_walk:{
                height: 16,
                width: Fit,
            }
            height: 60,
            width: 66,
            margin: 16,
            padding: 8,
            // move icon to the container center
            align: {x:0.5, y:0.5},
        }
    }
}

