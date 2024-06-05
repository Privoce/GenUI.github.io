# ScrollBar(s)

`ScrollBar`和`ScrollBars`这两个组件并不是单独去使用的，它们常使用在构建一个可以滚动的视图中充当属性

The `ScrollBar` and `ScrollBars` components are not used separately and are often used as properties in building a scrollable view

## Example

```rust
ScrollXView = <ViewBase> {
    scroll_bars: <ScrollBars> {show_scroll_x: true, show_scroll_y: false}
}

// in widget define struct
#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct View {
    #[live]
    scroll_bars: Option<LivePtr>,
    #[rust]
    scroll_bars_obj: Option<Box<ScrollBars>>,
}
```

## Default

```rust
ScrollBar = <ScrollBarBase> {
        bar_size: 10.0,
        bar_side_margin: 3.0
        min_handle_size: 30.0
        draw_bar: {
            //draw_depth: 5.0
            uniform border_radius: 1.5
            instance bar_width: 6.0
            instance pressed: 0.0
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                if self.is_vertical > 0.5 {
                    sdf.box(
                        1.,
                        self.rect_size.y * self.norm_scroll,
                        self.bar_width,
                        self.rect_size.y * self.norm_handle,
                        self.border_radius
                    );
                }
                else {
                    sdf.box(
                        self.rect_size.x * self.norm_scroll,
                        1.,
                        self.rect_size.x * self.norm_handle,
                        self.bar_width,
                        self.border_radius
                    );
                }
                return sdf.fill(mix(
                    THEME_COLOR_SCROLL_BAR_DEFAULT,
                    mix(
                        THEME_COLOR_CONTROL_HOVER,
                        THEME_COLOR_CONTROL_PRESSED,
                        self.pressed
                    ),
                    self.hover
                ));
            }
        }
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bar: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    cursor: Default,
                    from: {
                        all: Forward {duration: 0.1}
                        pressed: Forward {duration: 0.01}
                    }
                    apply: {
                        draw_bar: {
                            pressed: 0.0,
                            hover: [{time: 0.0, value: 1.0}],
                        }
                    }
                }

                pressed = {
                    cursor: Default,
                    from: {all: Snap}
                    apply: {
                        draw_bar: {
                            pressed: 1.0,
                            hover: 1.0,
                        }
                    }
                }
            }
        }
    }

    ScrollBars = <ScrollBarsBase> {
        show_scroll_x: true,
        show_scroll_y: true,
        scroll_bar_x: <ScrollBar> {}
        scroll_bar_y: <ScrollBar> {}
    }
```