use makepad_widgets::*;
live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    App = {{App}}{ 
        ui: <Window>{
            show_bg: true, 
            draw_bg: { color: #1C2630 }, 
            width: Fill, 
            height: Fill,  
            body = <View>{
                flow: Down,
                align: {x: 0.5, y: 0.5},  
                btn1 = <Button>{
                    text: "Click Me!"
                } 
                t_label = <Label>{ 
                    margin: {top: 16.0},
                    text: "Click Number: 0",
                    draw_text: {
                        color: #f, 
                        text_style: { 
                            font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")}, 
                        } 
                    }
                } 
            } 
        } 
    }
}
#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    counter: usize,
}


impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}


impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions) {
        if self.ui.button(id!(btn1)).clicked(&actions){
            self.counter += 1;
            let label = self.ui.label(id!(t_label));
            label.set_text(&format!("Click Number: {}", self.counter));
            label.redraw(cx);
        }
    }
}
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
app_main!(App);
