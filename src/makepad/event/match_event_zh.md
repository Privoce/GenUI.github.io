# MatchEvent Trait

`MatchEvent` trait 提供了一个处理广泛事件的框架，包括应用程序生命周期、用户交互、网络响应等。

每当需要处理Widget的事件时，您应该选择实现这一特性，而不是自己手动实现

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
    
 `MatchEvent` trait 封装了各种事件处理功能:

- `fn handle_startup(&mut self, _cx: &mut Cx){}`: 启动事件处理
- `fn handle_shutdown(&mut self, _cx: &mut Cx){}`: 关闭事件处理
- `fn handle_foreground(&mut self, _cx: &mut Cx){}`: 应用进入前台事件处理
- `fn handle_background(&mut self, _cx: &mut Cx){}`: 应用进入后台事件处理
- `fn handle_pause(&mut self, _cx: &mut Cx){}`: 应用暂停事件处理
- `fn handle_resume(&mut self, _cx: &mut Cx){}`: 应用恢复事件处理
- `fn handle_app_got_focus(&mut self, _cx: &mut Cx){}`: 应用获得焦点事件处理
- `fn handle_app_lost_focus(&mut self, _cx: &mut Cx){}`: 应用失去焦点事件处理
- `fn handle_next_frame(&mut self, _cx: &mut Cx, _e:&NextFrameEvent){}`: 准备下一帧事件处理
- `fn handle_action(&mut self, _cx: &mut Cx, _e:&Action){}`: 单个动作事件处理
- `fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){}`: 多个动作事件的处理
- `fn handle_signal(&mut self, _cx: &mut Cx){}`: 信号事件处理（可能用于自定义事件或消息传递）
- `fn handle_audio_devices(&mut self, _cx: &mut Cx, _e:&AudioDevicesEvent){}`: 音频设备事件处理
- `fn handle_midi_ports(&mut self, _cx: &mut Cx, _e:&MidiPortsEvent){}` MIDI 端口事件处理
- `fn handle_video_inputs(&mut self, _cx: &mut Cx, _e:&VideoInputsEvent){}`: 视频输入事件处理
- `fn handle_http_response(&mut self, _cx:&mut Cx, _request_id:LiveId, response:&HttpResponse){}`: 处理 HTTP 响应事件
- `fn handle_http_request_error(&mut self, _cx:&mut Cx, _request_id:LiveId, err:&str){}`: 处理 HTTP 请求错误事件。
- `fn handle_http_progress(&mut self, _cx:&mut Cx, _request_id:LiveId, loaded:u64, _total:u64){}`: 处理 HTTP 进度事件。
- `fn handle_network_responses(&mut self, cx: &mut Cx, e:&NetworkResponsesEvent ){}`: 处理网络响应事件，根据响应类型调用相应的处理函数
- `fn handle_draw(&mut self, _cx: &mut Cx, _e:&DrawEvent){}`: 绘制事件处理
- `fn handle_timer(&mut self, _cx: &mut Cx, _e:&TimerEvent){}`: 定时器事件处理
- `fn handle_draw_2d(&mut self, _cx: &mut Cx2d){}`: 2D 绘制处理
- `fn handle_key_down(&mut self, _cx: &mut Cx, _e:&KeyEvent){}`: 键盘按下事件处理
- `fn handle_key_up(&mut self, _cx: &mut Cx, _e:&KeyEvent){}`: 键盘释放事件处理
- `fn handle_back_pressed(&mut self, _cx: &mut Cx){}`: 返回按钮事件处理（通常用于移动设备）
- `fn match_event(&mut self, cx:&mut Cx, event:&Event){}`: 匹配并处理传入的事件，根据事件类型调用相应的处理函数
- `fn match_event_with_draw_2d(&mut self, cx:&mut Cx, event:&Event)->Result<(),()>{}`:特定于绘制事件的匹配与处理逻辑，使用 Cx2d 作为上下文