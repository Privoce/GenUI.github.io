# FoldButton

`FoldButton`常使用在类似`Collapse`这些需要折叠操作的组件作为折叠触发的按钮

`FoldButton` is commonly used as a folding trigger button in components such as `Collapse` that require folding operations
## Example

![](../../../static/widget/fold_btn.gif)

<strong style="color: #FF0000">
<br>
当前该组件的大小设置似乎存在问题，待修复
<br><br>
There seems to be an issue with the current size setting of this component, which needs to be fixed
</strong>

```rust
<FoldButton>{
    animator: {open = {default: yes}}, 
    height: 25, 
    width: 15,
    margin: {left: 5}
}
```

## Default

```rust
    FoldButton = <FoldButtonBase> {
        draw_bg: {
            instance open: 0.0
            instance hover: 0.0

            uniform fade: 1.0

            fn pixel(self) -> vec4 {

                let sz = 3.;
                let c = vec2(5.0, 0.5 * self.rect_size.y);
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.clear(vec4(0.));
                // we have 3 points, and need to rotate around its center
                sdf.rotate(self.open * 0.5 * PI + 0.5 * PI, c.x, c.y);
                sdf.move_to(c.x - sz, c.y + sz);
                sdf.line_to(c.x, c.y - sz);
                sdf.line_to(c.x + sz, c.y + sz);
                sdf.close_path();
                sdf.fill(mix(#a, #f, self.hover));
                return sdf.result * self.fade;
            }
        }

        abs_size: vec2(32, 12)
        abs_offset: vec2(4., 0.)
        width: 12,
        height: 12,

        animator: {

            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {draw_bg: {hover: 0.0}}
                }

                on = {
                    from: {all: Snap}
                    apply: {draw_bg: {hover: 1.0}}
                }
            }

            open = {
                default: yes
                no = {
                    from: {all: Forward {duration: 0.2}}
                    ease: ExpDecay {d1: 0.96, d2: 0.97}
                    redraw: true
                    apply: {
                        draw_bg: {open: [{time: 0.0, value: 1.0}, {time: 1.0, value: 0.0}]}
                    }
                }
                yes = {
                    from: {all: Forward {duration: 0.2}}
                    ease: ExpDecay {d1: 0.98, d2: 0.95}
                    redraw: true
                    apply: {
                        draw_bg: {open: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}]}
                    }
                }
            }
        }
    }
```