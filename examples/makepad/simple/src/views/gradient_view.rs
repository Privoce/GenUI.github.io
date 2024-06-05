use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    GradientViewExample = <SolidView>{
        height: 260,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 16,
        <GradientXView>{
            height: 100,
            width: 100,
        }
        <GradientYView>{
            height: 100,
            width: 100,
            draw_bg: {
                instance color2: #f0f, 
                instance dither: 10.5
            }
        }
    }
}
