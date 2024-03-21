# MatchEvent Trait

The `MatchEvent` trait provides a framework for handling a wide array of events, encompassing application lifecycle, user interactions, network responses, and more.

Whenever there's a need to handle events for widgets, you should opt to implement this trait rather than crafting your own implementation.

## Implementing MatchEvent

```rust
/// Struct requiring implementation
#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

/// Implementing the MatchEvent trait
impl MatchEvent for App {
    // Implementation of handle_actions
    fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions) {
        // Handling button click events
        // Use id! macro to access button: btn1
        if self.ui.button(id!(btn1)).clicked(&actions) {
            self.counter += 1;
            let label = self.ui.label(id!(t_label));
            // Setting text
            label.set_text(&format!("Click Number: {}", self.counter));
            // Redrawing label
            label.redraw(cx);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // Registering match event for it to work
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
```

### Complete Example

```rust
use makepad_widgets::*;
live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    App = {{App}} { 
        ui: <Window> {
            show_bg: true, 
            draw_bg: { color: #1C2630 }, 
            width: Fill, 
            height: Fill,  
            body = <View> {
                flow: Down,
                align: {x: 0.5, y: 0.5},  
                btn1 = <Button> {
                    text: "Click Me!"
                } 
                t_label = <Label> { 
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
        if self.ui.button(id!(btn1)).clicked(&actions) {
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
```

## MatchEvent Functions
    
The `MatchEvent` trait encapsulates various event handling functions:

- `handle_startup`: Initializes event handling.
- `handle_shutdown`: Handles shutdown events.
- `handle_foreground`: Manages app foreground entry events.
- `handle_background`: Manages app background entry events.
- `handle_pause`: Manages app pause events.
- `handle_resume`: Manages app resume events.
- `handle_app_got_focus`: Handles app focus acquisition events.
- `handle_app_lost_focus`: Handles app focus loss events.
- `handle_next_frame`: Prepares for the next frame events.
- `handle_action`: Handles individual action events.
- `handle_actions`: Handles multiple action events.
- `handle_signal`: Manages signal events (may be used for custom events or messaging).
- `handle_audio_devices`: Manages audio device events.
- `handle_midi_ports`: Manages MIDI port events.
- `handle_video_inputs`: Manages video input events.
- `handle_http_response`: Handles HTTP response events.
- `handle_http_request_error`: Handles HTTP request error events.
- `handle_http_progress`: Manages HTTP progress events.
- `handle_network_responses`: Handles network response events, calling corresponding handlers based on response type.
- `handle_draw`: Manages drawing events.
- `handle_timer`: Manages timer events.
- `handle_draw_2d`: Manages 2D drawing events.
- `handle_key_down`: Handles key press events.
- `handle_key_up`: Handles key release events.
- `handle_back_pressed`: Manages back button events (commonly used in mobile devices).
- `match_event`: Matches and processes incoming events, calling corresponding handlers based on the event type.
- `match_event_with_draw_2d`: Specialized event matching and handling logic for drawing events, using Cx2d as context.