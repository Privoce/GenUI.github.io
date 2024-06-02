# View

View是一个最基础的视图组件，其他视图组件有: `ScrollXYView`, `ScrollXView`, `ScrollYView` 等

View is the most basic view component, and other view components include: `ScrollXYView`, `ScrollXView`, `ScrollYView` ...

## Example

```rust
use makepad_widgets::*;
       
live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    
    App = {{App}} {
        ui: <Root>{
            main_window = <Window>{
                block_signal_event: true;
                window: {inner_size: vec2(600, 400)},
                pass: {clear_color: #1C2128},
                <View>{
                    show_bg: true,
                    // inherits parent width
                    width: All,
                    // inherits parent height
                    height: All,
                    padding: 10.0,
                    spacing: 16.0,
                    draw_bg: {color: #ADBABD},
                    flow: Right,
                    <View>{
                        height: 30,
                        width: 120,
                        show_bg: true,
                        draw_bg: {color: #FF0000},
                    }
                    <View>{
                        height: 30,
                        width: 90,
                        show_bg: true,
                        draw_bg: {color: #FF00FF},
                    }
                    <View>{
                        height: 30,
                        width: 120,
                        show_bg: true,
                        draw_bg: {color: #FF00FF},
                    }
                }
            }
        }
    }
}  
 
```

### flow Right
![](../../../static/widget/view_flow_right.png)
### flow Down
![](../../../static/widget/view_flow_down.png)
### flow Overlay
![](../../../static/widget/view_flow_overlay.png)