use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    FoldBtnExample = <SolidView>{
        height: 260,
        flow: Down,
        align: {x: 0.5, y: 0.5},
        spacing: 16,
        <FoldButton>{
            animator: {open = {default: yes}}, 
            margin: {left: 0}
        }
    }
}

