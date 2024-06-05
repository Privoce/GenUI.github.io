# FoldHeader

一种小部件，表示具有可扩展和可折叠主体部分的可折叠头部。它使用折叠按钮来控制身体的打开和关闭状态。

A widget representing a foldable header with an expandable and collapsible body section. It uses a fold button to control the open and close state of the body.

## Props
|decorate|name|type|description|
|--|--|--|--|
|rust|draw_state|`DrawStateWrap<DrawState>`|Handles the drawing state of the fold header.|
|rust|rect_size|`f64`|Stores the size of the rectangle for the body section.|
|rust|area|`Area`|Defines the drawing area of the fold header.|
|find, redraw, live|header|`WidgetRef`|Reference to the header widget.|
|find, redraw, live|body|`WidgetRef`|Reference to the body widget.|
|animator|animator|`Animator`|Handles the animations for the fold header.|
|live|opened|`f64`|Defines the open state of the body section (0.0 = closed, 1.0 = fully open).|
|layout|layout|`Layout`|Defines the layout of the fold header.|
|walk|walk|`Walk`|Defines how the fold header should be positioned and sized within its parent.|
|live|body_walk|`Walk`|Defines how the body section should be positioned and sized within the fold header.|

See [Walk](../types/walk.md)

See [Animator](../../syntax/animator.md)

See [Layout](../types/layout.md)

## Event
|name|description|
|--|--|
|Opening|Triggered when the fold button is opening.|
|Closing|Triggered when the fold button is closing.|
|None||

