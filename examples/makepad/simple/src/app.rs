use makepad_widgets::*;
       
live_design!{
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import crate::views::view::*;
    import crate::views::solid_view::*;
    import crate::views::scroll_x::*;
    import crate::views::scroll_y::*;
    import crate::views::scroll_xy::*;
    import crate::views::shape_view::*;
    import crate::views::gradient_view::*;
    import crate::buttons::button::*;
    import crate::label::*;
    import crate::buttons::link::*;
    import crate::splitter::*;
    import crate::buttons::desktop_btn::*;
    import crate::buttons::fold_btn::*;
    import crate::form::text_input::*;
    import crate::form::slider::*;
    import crate::form::slide::*;
    import crate::checkbox::check_box::*;
    import crate::checkbox::radio::*;

    
    App = {{App}} {
        ui: <Root>{
            main_window = <Window>{
                block_signal_event: true;
                window: {inner_size: vec2(400, 600)},
                pass: {clear_color: #1C2128},
                
                <ScrollYView>{
                    height: All,
                    width: Fill,
                    flow: Down,
                    spacing: 16,
                    <ViewExample>{}
                    <SolidViewExample>{}
                    <SXViewExample>{}
                    <SXYViewExample>{}
                    <SYViewExample>{}
                    <ShapeViewExample>{}
                    <GradientViewExample>{}
                    <LabelExample>{}
                    <ButtonExample>{}
                    <LinkExample>{}
                    <SplitterExample>{}
                    <DesktopBtnExample>{}
                    <FoldBtnExample>{}
                    <TextInputExample>{}
                    <SliderExample>{}
                    <SlideExample>{}
                    <CheckboxExample>{}
                    <RadioExample>{}
                }
            }
        }
    }
}  
              
app_main!(App); 
 
#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
 }

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::views::view::live_design(cx);
        crate::views::solid_view::live_design(cx);
        crate::views::scroll_x::live_design(cx);
        crate::views::scroll_y::live_design(cx);
        crate::views::scroll_xy::live_design(cx);
        crate::views::shape_view::live_design(cx);
        crate::label::live_design(cx);
        crate::splitter::live_design(cx);
        crate::buttons::button::live_design(cx);
        crate::buttons::link::live_design(cx);
        crate::buttons::desktop_btn::live_design(cx);
        crate::buttons::fold_btn::live_design(cx);
        crate::views::gradient_view::live_design(cx);
        crate::form::text_input::live_design(cx);
        crate::form::slider::live_design(cx);
        crate::form::slide::live_design(cx);
        crate::checkbox::check_box::live_design(cx);
        crate::checkbox::radio::live_design(cx);
    }
}

impl MatchEvent for App{
    fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
} 
