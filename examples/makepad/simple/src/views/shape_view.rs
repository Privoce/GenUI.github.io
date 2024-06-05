use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    ShapeViewExample = <SolidView>{
        height: 300,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 16,
        <RectView>{
            height: 60,
            width: 60,
            draw_bg: {color: #FF0000},
        }
        <RoundedView>{
            height: 60,
            width: 60,
            draw_bg: {color: #00FF00},
        }
        <RoundedXView>{
            height: 60,
            width: 60,
            draw_bg: {color: #0000FF},
        }
        <RoundedYView>{
            height: 60,
            width: 60,
            draw_bg: {color: #223311},
        }
        <RoundedAllView>{
            height: 60,
            width: 60,
            draw_bg: {color: #ff00ff},
        }
        <CircleView>{
            height: 60,
            width: 60,
            draw_bg: {color: #ff00ff},
        }
    }
}