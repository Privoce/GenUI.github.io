

use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    SliderExample = <SolidView>{
        height: 260,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 16,
        <Slider> {
            width: 100,
            height: 30, 
            draw_slider:{
                slider_type: Horizontal
            },
            text: "sss",
        }  
    }
}

