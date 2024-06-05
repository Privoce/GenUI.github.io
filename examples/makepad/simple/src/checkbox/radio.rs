use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    RadioExample = <SolidView>{
        height: 260,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 10,
        <RadioButton>{
            label: "Radio Button1",
            
            draw_radio: {
                radio_type: Tab,
            }
        }
        <RadioButton>{
            label: "Radio Button2",
            label_align: {x: 1.5, y: 0.5},
            label_walk: {
                margin: {left: 16.0}
            }
        }
    }
}


