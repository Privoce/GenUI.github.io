# GHLayout

A horizontal layout component use CardBase

layout don't have border, background color, border-radius, ... (but you can add if you want)

## Example
![](../../../static/gen/components/g_h_layout.png)
```rust
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GHLayoutExample = <ScrollYView>{
        height: 100.0,
        width: Fill,
        flow: Down,
        spacing: 10.0,
        <Label>{
            text: "GHLayout",
        }
        <GHLayout>{
            height: Fit,
            width: 300,
            background_color: #FFFFFF,
            spacing: 10.0,
            <GLabel>{
                text: "Hello",
                color: #0,
                margin: 10.0,
            }
            <GCard>{
                theme: Error,
                height: 30.0,
                width: 30.0,
            }
            <GCard>{
                theme: Warning,
                height: 30.0,
                width: 30.0,
            }
            <GButton>{
                text: "hello"
            }
        }
    }
}
```