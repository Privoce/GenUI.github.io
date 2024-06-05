use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    SplitterExample = <SolidView>{
        height: 300,
        width: Fill,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 10,
        <Splitter>{
            align: FromA(100),
            a: <View>{
                height: Fill,
                width: 200,
                show_bg: true,
                draw_bg: {color: #FF00F0},
            },
            b: <View>{
                height: Fill,
                width: 200,
                show_bg: true,
                draw_bg: {color: #FF0000},
            }
        }
        <Splitter>{
            // align: FromB(100),
            // same as align: FromA(20% * a.width),
            align: Weighted(0.5),
            axis: Vertical,
            a: <View>{
                height: Fill,
                width: 200,
                show_bg: true,
                draw_bg: {color: #FF00F0},
            },
            b: <View>{
                height: Fill,
                width: 200,
                show_bg: true,
                draw_bg: {color: #FF0000},
            }
        }
    }
}
