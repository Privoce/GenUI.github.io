use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    FoldBtnExample = <SolidView>{
        height: 260,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 16,
        <FoldHeader>{
            header: <View>{
                show_bg: true,
                width: Fill  
                height: Fit                                 
                draw_bg: {                     
                    fn pixel(self) -> vec4{
                        return #FFFF00
                    }
                }        
                padding: 5,
                flow: Right, 
                fold_button = <FoldButton>{} <Label>{text: "Fold me!"} 
            }
            body: <View>{
                show_bg: true,
                width: Fill  
                height: Fit                                 
                draw_bg: {                     
                    fn pixel(self) -> vec4{
                        return #00FF00
                    }
                }        
                padding: 5,
                <Label>{
                    text:"This is the body that can be folded away"
                }
            }
        }
    }
}


