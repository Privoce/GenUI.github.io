# Animation

`[]` 表示可选


```rust
animator:{
    $event_name = {
        default: on | off // open or close you can define yourself
        // action when close
        off = {
            from: {$event_name|all: $play}
            [ease]: $ease // default: Linear
            [redraw]: true|false
            apply: {
                $draw_name: {$event_name: 0.0|1.0, ...} // 1.0 is true 0.0 is false 
            }
        }
        // action when open
        on = {
            // ...
        }
    }
}
```
## Example

```rust
animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {pressed: 0.0, hover: 0.0}
                        draw_icon: {pressed: 0.0, hover: 0.0}
                        draw_text: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: 0.1}
                        pressed: Forward {duration: 0.01}
                    }
                    apply: {
                        draw_bg: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        draw_icon: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        draw_text: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_bg: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        draw_icon: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        draw_text: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                    }
                }
            }
        }
```

## `$play`
在 Makepad 中，Play 枚举用于定义动画的播放模式。

- duration: 动画播放的持续时间，以秒为单位，默认1.0秒

|类型|参数|示例|说明|
|--|--|--|--|
|Snap|None|`from: {all: Snap}`|动画瞬间跳转到结束点，没有播放过程。这种模式下没有持续时间参数，因为动画是瞬时完成的。|
|Forward|`duration: f64`|`from: {all: Forward{duration: 0.2}}`|动画从开始点播放到结束点|
|Reverse|`duration: f64`<br>`end: f64`|`from: {all: Reverse{duration: 0.5}}`|动画从结束点向开始点反向播放。|
|Loop|`duration: f64`<br>`end: f64`|``|动画从开始点播放到结束点，然后重新开始，循环播放|
|ReverseLoop|`duration: f64`<br>`end: f64`|``|动画从开始点播放到结束点，然后反向播放回开始点，反复循环|
|BounceLoop|`duration: f64`<br>` end: f64`||动画从开始点播放到结束点，然后反向播放回开始点，反复循环，就像在两点之间反弹|

## `$ease`

`Ease` 每个枚举变体及其参数定义了不同的缓动行为，可以用于实现各种复杂的动画效果

| 类型          | 参数                           | 示例                          | 说明                                |
|---------------|-------------------------------|-----------------------------|-------------------------------------|
| `Linear`      | None                          | `Linear`               | 线性插值                              |
| `None`        | None                          | `None`                 | 无缓动效果                             |
| `Constant`    | `f64`                         | `Constant(1.0)`        | 常量值缓动                             |
| `InQuad`      | None                          | `InQuad`               | 二次方缓动（加速）                         |
| `OutQuad`     | None                          | `OutQuad`              | 二次方缓动（减速）                         |
| `InOutQuad`   | None                          | `InOutQuad`            | 二次方缓动（加速减速）                      |
| `InCubic`     | None                          | `InCubic`              | 三次方缓动（加速）                         |
| `OutCubic`    | None                          | `OutCubic`             | 三次方缓动（减速）                         |
| `InOutCubic`  | None                          | `InOutCubic`           | 三次方缓动（加速减速）                      |
| `InQuart`     | None                          | `InQuart`              | 四次方缓动（加速）                         |
| `OutQuart`    | None                          | `OutQuart`             | 四次方缓动（减速）                         |
| `InOutQuart`  | None                          | `InOutQuart`           | 四次方缓动（加速减速）                      |
| `InQuint`     | None                          | `InQuint`              | 五次方缓动（加速）                         |
| `OutQuint`    | None                          | `OutQuint`             | 五次方缓动（减速）                         |
| `InOutQuint`  | None                          | `InOutQuint`           | 五次方缓动（加速减速）                      |
| `InSine`      | None                          | `InSine`               | 正弦缓动（加速）                         |
| `OutSine`     | None                          | `OutSine`              | 正弦缓动（减速）                         |
| `InOutSine`   | None                          | `InOutSine`            | 正弦缓动（加速减速）                      |
| `InExp`       | None                          | `InExp`                | 指数缓动（加速）                         |
| `OutExp`      | None                          | `OutExp`               | 指数缓动（减速）                         |
| `InOutExp`    | None                          | `InOutExp`             | 指数缓动（加速减速）                      |
| `InCirc`      | None                          | `InCirc`               | 圆形缓动（加速）                         |
| `OutCirc`     | None                          | `OutCirc`              | 圆形缓动（减速）                         |
| `InOutCirc`   | None                          | `InOutCirc`            | 圆形缓动（加速减速）                      |
| `InElastic`   | None                          | `InElastic`            | 弹性缓动（加速）                         |
| `OutElastic`  | None                          | `OutElastic`           | 弹性缓动（减速）                         |
| `InOutElastic`| None                          | `InOutElastic`         | 弹性缓动（加速减速）                      |
| `InBack`      | None                          | `InBack`               | 回退缓动（加速）                         |
| `OutBack`     | None                          | `OutBack`              | 回退缓动（减速）                         |
| `InOutBack`   | None                          | `InOutBack`            | 回退缓动（加速减速）                      |
| `InBounce`    | None                          | `InBounce`             | 跳跃缓动（加速）                         |
| `OutBounce`   | None                          | `OutBounce`            | 跳跃缓动（减速）                         |
| `InOutBounce` | None                          | `InOutBounce`          | 跳跃缓动（加速减速）                      |
| `ExpDecay`    | `d1: f64, d2: f64, max: usize`| `ExpDecay {d1: 0.82, d2: 0.97, max: 100}` | 指数衰减缓动，具有初始和最终的衰减因子，以及最大迭代次数 |
| `Pow`         | `begin: f64, end: f64`        | `Pow {begin: 0.0, end: 1.0}`             | 幂次方缓动，具有起始和结束位置参数                   |
| `Bezier`      | `cp0: f64, cp1: f64, cp2: f64, cp3: f64` | `Bezier {cp0: 0.0, cp1: 0.0, cp2: 1.0, cp3: 1.0}` | 贝塞尔曲线缓动，具有四个控制点参数                   |

## `$draw_name`

draw_name is a struct which can use glsl code

### example

```rust
#[derive(Live, LiveRegister)]
#[repr(C)]
pub struct DrawText {
    #[rust] pub many_instances: Option<ManyInstances>,
    
    #[live] pub geometry: GeometryQuad2D,
    #[live] pub text_style: TextStyle,
    #[live] pub wrap: TextWrap,
    
    #[live] pub ignore_newlines: bool,
    #[live] pub combine_spaces: bool,
    
    #[live(1.0)] pub font_scale: f64,
    #[live(1.0)] pub draw_depth: f32,
    
    #[deref] pub draw_vars: DrawVars,
    // these values are all generated
    #[live] pub color: Vec4,
    #[calc] pub font_t1: Vec2,
    #[calc] pub font_t2: Vec2,
    #[calc] pub rect_pos: Vec2,
    #[calc] pub rect_size: Vec2,
    #[calc] pub draw_clip: Vec4,
    #[calc] pub char_depth: f32,
    #[calc] pub delta: Vec2,
    #[calc] pub shader_font_size: f32,
    #[calc] pub advance: f32,
}
```