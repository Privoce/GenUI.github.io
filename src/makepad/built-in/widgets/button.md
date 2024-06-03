# Button

`Button`小部件是一个可自定义的交互式UI元素，用于响应用户操作，如点击、按下和释放。它包括背景、文本和图标渲染的属性，并支持不同状态（例如悬停、按下）的动画。

The `Button` widget is a customizable, interactive UI element that responds to user actions such as clicks, presses, and releases. It includes properties for background, text, and icon rendering, and supports animations for different states (e.g., hover, pressed).

## Props
|decorate|name|type|description|
|--|--|--|--|
|animator|animator|`Animator`|Controls the animations for the button states.|
|redraw|draw_bg|`DrawQuad`|Draws the background of the button.|
|live|draw_text|`DrawText`|Draws the text label of the button.|
|live|draw_icon|`DrawIcon`|Draws the icon for the button.|
|live|icon_walk|`Walk`|Defines the walk properties for the icon.|
|live|label_walk|`Walk`|Defines the walk properties for the label.|
|walk|walk|`Walk`|Defines the walk properties for the button.|
|layout|layout|`Layout`|Defines the layout properties for the button.|
|live|grab_key_focus|`bool`|Determines if the button should grab key focus when interacted with.|
|live|text|`RcStringMut`|The text label displayed on the button.|

See [Walk](../types/walk.md)

See [Layout](../types/layout.md)

See [DrawText](../types/draw_text.md)

See [DrawQuad](../types/draw_quad.md)

See [DrawIcon](../types/draw_icon.md)

## Event
|name|description|
|--|--|
|Clicked|Triggered when the button is clicked.|
|Pressed|Triggered when the button is pressed.|
|Released|Triggered when the button is released.|
