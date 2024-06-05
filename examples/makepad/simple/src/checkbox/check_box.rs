use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    CheckboxExample = <SolidView>{
        height: 260,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 10,
        <CheckBox>{
            text: "Check Box None",
            draw_check: {
                check_type: None,
            },
        } 
        <CheckBox>{
            text: "Check Box Default",
        }
        // look like Radio, but it can be clicked multiple times
        // Radio only can be clicked once
        <CheckBox>{
            text: "Check Box Radio",
            draw_check: {
                check_type: Radio,
            },
        }
        // look like Switch
        <CheckBox>{
            text: "Check Box3",
            draw_check: {
                check_type: Toggle,
            },
            label_walk: {
                margin: {left: 32.0},
            },
        }
        <CheckBox>{
            text: "Check Box",
            draw_check: {
                check_type: Check,
            },
        } 
    }
}


