# ScrollView

## ScrollXView

ScrollXView组件是一个带有横向ScrollBar的视图组件

The ScrollXView component is a view component with a horizontal ScrollBar
```rust
ScrollXView = <ViewBase> {
    scroll_bars: <ScrollBars> {show_scroll_x: true, show_scroll_y: false}
}
```

### Example

![](../../../static/widget/scrollxview.png)

```rust
<ScrollXView>{
    height: Fill,
    width: Fill,
    spacing: 10,
    <View>{
        height: 30,
        width: 120,
        show_bg: true,
        draw_bg: {color: #FF0000},
    }
    <View>{
        height: 30,
        width: 420,
        show_bg: true,
        draw_bg: {color: #FFFF00},
    }
    <View>{
        height: 80,
        width: 120,
        show_bg: true,
        draw_bg: {color: #FF00FF},
    }
}
```

## ScrollYView

ScrollYView组件是一个带有纵向ScrollBar的视图组件

ScrollYView component is a view component with a vertical ScrollBar

```rust
ScrollYView = <ViewBase> {
    scroll_bars: <ScrollBars> {show_scroll_x: false, show_scroll_y: true}
}
```

### Example

![](../../../static/widget/scrollyview.png)

```rust
use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    SYViewExample = <ScrollYView>{
        height: 80,
        width: 300,
        flow: Down,
        <View>{
            height: 30,
            width: 120,
            show_bg: true,
            draw_bg: {color: #FF0000},
        }
        <View>{
            height: 30,
            width: 420,
            show_bg: true,
            draw_bg: {color: #FFFF00},
        }
        <View>{
            height: 80,
            width: 120,
            show_bg: true,
            draw_bg: {color: #FF00FF},
        }
    }
}
```

## ScrollXYView

ScrollXYView组件是一个同时带有纵向和横向ScrollBar的视图组件

The ScrollXYView component is a view component with both a horizontal and a vertical ScrollBar

```rust
ScrollXYView = <ViewBase> {
    scroll_bars: <ScrollBars> {show_scroll_x: true,show_scroll_y: true}
}
```
### Example

![](../../../static/widget/scrollxyview.png)

```rust
use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 

    SXYViewExample = <ScrollXYView>{
        height: 120,
        width: 200,
        spacing: 10,
        <View>{
            height: 30,
            width: 120,
            show_bg: true,
            draw_bg: {color: #FF0000},
        }
        <View>{
            height: 30,
            width: 420,
            show_bg: true,
            draw_bg: {color: #FFFF00},
        }
        <View>{
            height: 280,
            width: 120,
            show_bg: true,
            draw_bg: {color: #FF00FF},
        }
    }
}
```

