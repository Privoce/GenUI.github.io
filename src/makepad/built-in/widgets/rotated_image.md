# RotatedImage

RotatedImage是一个用于处理和显示旋转图像的结构。

RotatedImage is a struct for handling and displaying rotated images.

## Props
|decorate|name|type|description|
|--|--|--|--|
|walk|walk|`Walk`|Controls the layout of the RotatedImage|
|layout|layout|`Layout`|Defines the layout of the RotatedImage|
|live, redraw|draw_bg|`DrawColor`|DrawColor used to draw the background|
|live|source|`LiveDependency`|The source of the RotatedImage|
|rust|texture|`Option<Texture>`|The texture of the RotatedImage|
|live|scale|`f64`|The scale of the RotatedImage|

See [Walk](../types/walk.md)

See [Layout](../types/layout.md)

See [DrawColor](../types/draw_color.md)

See [LiveDependency](../types/live_dep.md)

## Event
This struct does not define any events.
