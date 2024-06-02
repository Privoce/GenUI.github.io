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