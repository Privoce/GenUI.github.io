use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    DerefLabel = {{DerefLabel}}{
        instance = <Label>{
            draw_text:{
                text_style: { 
                    font: {
                        path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")
                    }, 
                } 
            }
        }
    }
}

#[derive(Live,Widget,LiveHook)]
pub struct  DerefLabel{
    #[live]
    pub text: RcStringMut,
    #[live]
    pub color: Vec4,
    #[live(16.0)]
    pub font_size: f64,
    #[deref]
    view:View
}

impl Widget for DerefLabel {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        
        self.label(id!(instance)).apply_over_and_redraw(cx, live!{
            text: (self.text),
            draw_text:{
                color: (self.color),
                text_style:{
                    font_size: (self.font_size)
                },
            }
        });
        self.view.draw_walk(cx, scope, walk)
    }
    fn set_text(&mut self, v: &str) {
        self.label(id!(instance)).set_text(v);
    }
    fn text(&self) -> String {
       self.text.as_ref().to_string()
    }
}

