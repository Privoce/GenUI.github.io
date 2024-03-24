use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    MyLabel = {{MyLabel}}{
        draw_text:{
            color: #EF5350,
            text_style: { 
                font: {
                    path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")
                }, 
                    brightness: 1.1,  
                    font_size: 32.0
                } 
        }
    }
}

#[derive(Live,Widget,LiveHook)]
pub struct  MyLabel{
    #[redraw]
    #[deref]
    instance: Label
}

impl Widget for MyLabel {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.instance.draw_walk(cx, scope, walk)
    }
}