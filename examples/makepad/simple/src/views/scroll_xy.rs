use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    SXYViewExample = <ScrollXYView>{
        height: 120,
        width: 200,
        spacing: 10,
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
            height: 280,
            width: 120,
            show_bg: true,
            draw_bg: {color: #FF00FF},
        }
    }
}