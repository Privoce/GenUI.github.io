# Window

## Example

![](../../../static/widget/window.png)

```rust
use makepad_widgets::*;
       
live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    
    App = {{App}} {
        ui: <Root>{
            main_window = <Window>{
                block_signal_event: true;
                show_bg: true
                // set size
                window: {inner_size: vec2(1280, 1000)},
                // width: Fill,
                // height: Fill,
                // recommend pass
                pass: {clear_color: #1C2128},
                // draw_bg: {
                //     fn pixel(self) -> vec4 {
                //         // test
                //         return mix(#7, #3, self.pos.y);
                //     }
                // }
            }
        }
    }
}
```

## Default

```rust
    Window = <WindowBase> {
        pass: {clear_color: (THEME_COLOR_CLEAR)}
        flow: Down
        nav_control: <NavControl> {}
        caption_bar = <SolidView> {
            visible: false,
            flow: Right
            draw_bg: {color: (THEME_COLOR_BG_APP)}
            height: 27
            caption_label = <View> {
                width: Fill,
                height: Fill
                align: {x: 0.5, y: 0.5},
                label = <Label> {text: "Makepad", margin: {left: 100}}
            }
            windows_buttons = <View> {
                visible: false,
                width: Fit,
                height: Fit
                min = <DesktopButton> {draw_bg: {button_type: WindowsMin}}
                max = <DesktopButton> {draw_bg: {button_type: WindowsMax}}
                close = <DesktopButton> {draw_bg: {button_type: WindowsClose}}
            }
            web_fullscreen = <View> {
                visible: false,
                width: Fit,
                height: Fit
                fullscreen = <DesktopButton> {draw_bg: {button_type: Fullscreen}}
            }
            web_xr = <View> {
                visible: false,
                width: Fit,
                height: Fit
                xr_on = <DesktopButton> {draw_bg: {button_type: XRMode}}
            }
        }
        
        window_menu = <WindowMenu>{
            main = Main{items:[app]}
            app = Sub{name:"Makepad",items:[quit]}
            quit = Item{
                name:"Quit",
                shift: false,
                key: KeyQ,
                enabled: true
            }
        }
        body = <KeyboardView>{
            keyboard_min_shift: 30,
            width: Fill,
            height: Fill
        }
        cursor: Default
        mouse_cursor_size: vec2(20, 20),
        draw_cursor: {
            instance border_width: 1.5
            instance color: #000
            instance border_color: #fff
            fn get_color(self) -> vec4 {
                return self.color
            }
            fn get_border_color(self) -> vec4 {
                return self.border_color
            }
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                sdf.move_to(1.0, 1.0);
                sdf.line_to(self.rect_size.x - 1.0, self.rect_size.y * 0.5)
                sdf.line_to(self.rect_size.x * 0.5, self.rect_size.y - 1.0)
                sdf.close_path();
                sdf.fill_keep(self.get_color())
                if self.border_width > 0.0 {
                    sdf.stroke(self.get_border_color(), self.border_width)
                }
                return sdf.result
            }
        }
        window: {
            inner_size: vec2(1024, 768)
        }
    }
```

