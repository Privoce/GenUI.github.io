use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    FoldBtnExample = <SolidView>{
        height: 260,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 16,
        <DropDown>{
            draw_text: {
                fn get_color(self) -> vec4 {
                    return #FFFFFF
                }
            },
            line_spacing: 1.5,
            height: 24,
            width: 200
            labels: ["ValueOne", "ValueTwo","Thrice","FourthValue","OptionE","Hexagons"],
            values: [  ValueOne,ValueTwo,Thrice,FourthValue,OptionE,Hexagons]
        }        
    }
}
