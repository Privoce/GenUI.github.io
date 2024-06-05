use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    SYViewExample = <ScrollYView>{
        height: 80,
        width: 300,
        flow: Down,
        <View>{
            height: 30,
            width: 120,
            show_bg: true,
            draw_bg: {color: #FF0000},
        }
        <View>{
            height: 30,
            width: 420,
            show_bg: true,
            draw_bg: {color: #FFFF00},
        }
        <View>{
            height: 80,
            width: 120,
            show_bg: true,
            draw_bg: {color: #FF00FF},
        }
    }
}