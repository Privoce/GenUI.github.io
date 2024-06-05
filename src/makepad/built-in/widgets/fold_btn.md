# FoldButton

表示可折叠按钮的小部件，可以通过点击打开或关闭，带有动画效果。用于在UI中创建可扩展的部分。

A widget representing a foldable button that can be opened or closed with animation. Useful for creating expandable sections in a UI.

## Props
|decorate|name|type|description|
|--|--|--|--|
|animator|animator|`Animator`|Handles the animations for the fold button.|
|redraw, live|draw_bg|`DrawQuad`|Defines the background drawing properties of the fold button.|
|live|abs_size|`DVec2`|Defines the absolute size of the fold button.|
|live|abs_offset|`DVec2`|Defines the absolute offset position of the fold button.|
|walk|walk|`Walk`|Defines how the fold button should be positioned and sized within its parent.|

See [DrawQuad](../types/draw_quad.md)

See [DVec2](../types/dvec2.md)

See [Walk](../types/walk.md)

See [Animator](../../syntax/animator.md)


## Event
|name|description|
|--|--|
|Opening|Triggered when the fold button is opened.|
|Closing|Triggered when the fold button is closed.|
|Animating(f64)|Triggered when the fold button is animating with a value representing the animation progress.|

