use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    ShapeViewExample = <SolidView>{
        height: 460,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 16,
        <Image>{
            source: dep("crate://self/icons/github.png"),
        }
        <Image>{
            height: 60,
            width: Fit,
            source: dep("crate://self/icons/github.png"),
            fit: Vertical,
        }
        <Image>{
            height: 46,
            width: 146,
            source: dep("crate://self/icons/github.png"),
            // biggest make image size overflow
            fit: Biggest,
            // if you use draw_bg it will cover the image
            draw_bg: {
                fn pixel(self)-> vec4{
                    return #FF0000
                },
            },
        }
        <ImageBlend>{
            height: 100,
            width: 100,
            source: dep("crate://self/icons/github.png"),
            draw_bg: {
                
                instance blend: 1.0,
            }
        }
        <RotatedImage>{
            height: 100,
            width: 100,
            source: dep("crate://self/icons/github.png"),
            draw_bg: {
                // how to calc: 1.57 = 90 deg
                // rotation = 90 * PI / 180
                instance rotation: 1.57
            }
        }
    }
}
