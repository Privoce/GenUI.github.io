# ImageBlend

ImageBlend是一个用于处理和显示具有混合效果的图像的结构。

ImageBlend is a struct for handling and displaying images with blending effects.

## Props
|decorate|name|type|description|
|--|--|--|--|
|walk|walk|`Walk`|Controls the layout of the ImageBlend|
|animator|animator|`Animator`|Animator used to animate the ImageBlend|
|live, redraw|draw_bg|`DrawQuad`|DrawQuad used to draw the background|
|live|min_width|`i64`|The minimum width of the ImageBlend|
|live|min_height|`i64`|The minimum height of the ImageBlend|
|live(1.0)|width_scale|`f64`|The width scale of the ImageBlend|
|live|fit|`ImageFit`|The fit mode of the ImageBlend|
|live|breathe|`bool`|The breathe effect of the ImageBlend|
|live|source|`LiveDependency`|The source of the ImageBlend|
|rust|texture|`[Option<Texture>;2]`|The textures of the ImageBlend|

See [Walk](../types/walk.md)

See [LiveDependency](../types/live_dep.md)

See [DrawQuad](../types/draw_quad.md)

See [ImageFit](../types/image_fit.md)

## Event
This struct does not define any events.