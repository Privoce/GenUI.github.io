# ImageBlend

它是一个用于将两个图像混合在一起的图像组件

It is an image component used to blend two images together

## Example

<strong style="color: #FF0000">
Comming Soon
</strong>

## Default
`draw_bg`: 包含了一些关于图像的属性和方法，如纹理、透明度、混合模式等。
- `texture image0和texture image1`: 分别表示两个要混合的图像。
- `instance opacity`: 表示混合后的图像的透明度，这里是1.0，即完全不透明。
- `instance blend`: 表示混合模式，这里是0.0，表示不进行混合。
- `instance image_scale`: 表示图像缩放比例，这里是(1.0, 1.0)，表示不缩放。
- `instance image_pan`: 表示图像平移距离，这里是(0.0, 0.0)，表示不平移。
- `instance breathe`: 表示呼吸效果，这里是0.0，表示没有呼吸效果。

`draw_bg`: Contains some attributes and methods related to images, such as texture, transparency, blend mode, etc.
- `texture image0 and texture image1`: respectively represent two images to be mixed.
- `instance opacity`: represents the transparency of the mixed image, which is 1.0, which means it is completely opaque.
- `instance blend`: represents mixed mode, where 0.0 indicates no blending.
- `instance image_scale`: represents the image scaling ratio, where (1.0, 1.0) represents no scaling.
- `instance image_pan`: represents the translation distance of the image, where (0.0, 0.0) represents no translation.
- `instance breathe`: represents breathing effect, here it is 0.0, indicating no breathing effect.

```rust
    ImageBlend = <ImageBlendBase> {
        width: 100
        height: 100
                
        draw_bg: {
            texture image0: texture2d
            texture image1: texture2d
            instance opacity: 1.0
            instance blend: 0.0
            instance image_scale: vec2(1.0, 1.0)
            instance image_pan: vec2(0.0, 0.0)
            instance breathe: 0.0            
            fn get_color_scale_pan(self, scale: vec2, pan: vec2) -> vec4 {
                let b = 1.0 - 0.1*self.breathe;
                let s = vec2(0.05*self.breathe);
                return mix(
                    sample2d(self.image0, self.pos * scale*b + pan+s).xyzw,
                    sample2d(self.image1, self.pos * scale*b + pan+s).xyzw,
                    self.blend
                )
            }
            
            fn get_color(self) -> vec4 {
                return self.get_color_scale_pan(self.image_scale, self.image_pan)
            }
                        
            fn pixel(self) -> vec4 {
                let color = self.get_color();
                return Pal::premul(vec4(color.xyz, color.w * self.opacity))
            }
        }
                
        animator: {
            blend = {
                default: zero,
                zero = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {blend: 0.0}
                    }
                }
                one = {
                    from: {
                        all: Forward {duration: 0.1}
                    }
                    apply: {
                        draw_bg: {blend: 1.0}
                    }
                }
            } 
            breathe = {
                default: off,
                on = {
                    from: {all: BounceLoop {duration: 10., end:1.0}}
                    apply:{
                        draw_bg:{breathe:[{time: 0.0, value: 0.0}, {time:1.0,value:1.0}]}
                    }
                }
                off = {
                    from: {all: Forward {duration: 1}}
                    apply:{
                        draw_bg:{breathe:0.0}
                    }
                }
            }
        }
    }
```