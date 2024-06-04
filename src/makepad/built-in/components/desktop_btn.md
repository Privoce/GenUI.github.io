# DesktopButton

`DesktopButton`用于构建软件控制栏

`DesktopButton` used to build software control bar

## Example
![](../../../static/widget/desktop_btn.png)

```rust
<DesktopButton> {draw_bg: {button_type: Fullscreen}}
<DesktopButton> {draw_bg: {button_type: XRMode}}
<DesktopButton> {draw_bg: {button_type: WindowsMin}}
<DesktopButton> {draw_bg: {button_type: WindowsMax}}
<DesktopButton> {draw_bg: {button_type: WindowsClose}}
<DesktopButton> {draw_bg: {button_type: WindowsMaxToggled}}
```

## Default

```rust
    DesktopButton = <DesktopButtonBase> {
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.aa *= 3.0;
                let sz = 4.5;
                let c = self.rect_size * vec2(0.5, 0.5);

                // WindowsMin
                match self.button_type {
                    DesktopButtonType::WindowsMin => {
                        sdf.clear(mix(#3, mix(#6, #9, self.pressed), self.hover));
                        sdf.move_to(c.x - sz, c.y);
                        sdf.line_to(c.x + sz, c.y);
                        sdf.stroke(#f, 0.5 + 0.5 * self.dpi_dilate);
                        return sdf.result;
                    }
                    DesktopButtonType::WindowsMax => {
                        sdf.clear(mix(#3, mix(#6, #9, self.pressed), self.hover));
                        sdf.rect(c.x - sz, c.y - sz, 2. * sz, 2. * sz);
                        sdf.stroke(#f, 0.5 + 0.5 * self.dpi_dilate);
                        return sdf.result;
                    }
                    DesktopButtonType::WindowsMaxToggled => {
                        let clear = mix(#3, mix(#6, #9, self.pressed), self.hover);
                        sdf.clear(clear);
                        let sz = 3.5;
                        sdf.rect(c.x - sz + 1., c.y - sz - 1., 2. * sz, 2. * sz);
                        sdf.stroke(#f, 0.5 + 0.5 * self.dpi_dilate);
                        sdf.rect(c.x - sz - 1., c.y - sz + 1., 2. * sz, 2. * sz);
                        sdf.fill_keep(clear);
                        sdf.stroke(#f, 0.5 + 0.5 * self.dpi_dilate);
                        return sdf.result;
                    }
                    DesktopButtonType::WindowsClose => {
                        sdf.clear(mix(#3, mix(#e00, #c00, self.pressed), self.hover));
                        sdf.move_to(c.x - sz, c.y - sz);
                        sdf.line_to(c.x + sz, c.y + sz);
                        sdf.move_to(c.x - sz, c.y + sz);
                        sdf.line_to(c.x + sz, c.y - sz);
                        sdf.stroke(#f, 0.5 + 0.5 * self.dpi_dilate);
                        return sdf.result;
                    }
                    DesktopButtonType::XRMode => {
                        sdf.clear(mix(#3, mix(#0aa, #077, self.pressed), self.hover));
                        let w = 12.;
                        let h = 8.;
                        sdf.box(c.x - w, c.y - h, 2. * w, 2. * h, 2.);
                        // subtract 2 eyes
                        sdf.circle(c.x - 5.5, c.y, 3.5);
                        sdf.subtract();
                        sdf.circle(c.x + 5.5, c.y, 3.5);
                        sdf.subtract();
                        sdf.circle(c.x, c.y + h - 0.75, 2.5);
                        sdf.subtract();
                        sdf.fill(#8);

                        return sdf.result;
                    }
                    DesktopButtonType::Fullscreen => {
                        sz = 8.;
                        sdf.clear(mix(#3, mix(#6, #9, self.pressed), self.hover));
                        sdf.rect(c.x - sz, c.y - sz, 2. * sz, 2. * sz);
                        sdf.rect(c.x - sz + 1.5, c.y - sz + 1.5, 2. * (sz - 1.5), 2. * (sz - 1.5));
                        sdf.subtract();
                        sdf.rect(c.x - sz + 4., c.y - sz - 2., 2. * (sz - 4.), 2. * (sz + 2.));
                        sdf.subtract();
                        sdf.rect(c.x - sz - 2., c.y - sz + 4., 2. * (sz + 2.), 2. * (sz - 4.));
                        sdf.subtract();
                        sdf.fill(#f); //, 0.5 + 0.5 * dpi_dilate);

                        return sdf.result;
                    }
                }
                return #f00;
            }
        }
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: 0.1}
                        state_down: Snap
                    }
                    apply: {
                        draw_bg: {
                            pressed: 0.0,
                            hover: 1.0,
                        }
                    }
                }

                pressed = {
                    from: {all: Snap}
                    apply: {
                        draw_bg: {
                            pressed: 1.0,
                            hover: 1.0,
                        }
                    }
                }
            }
        }
    }
```