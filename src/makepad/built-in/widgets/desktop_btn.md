# DesktopButton

代表桌面风格按钮的小部件，具有悬停、按下和不同按钮类型（例如最小化、最大化、关闭）的不同视觉状态。

A widget representing a desktop-style button with different visual states for hovering, pressing, and different button types (e.g., minimize, maximize, close).

## Props
|decorate|name|type|description|
|--|--|--|--|
|animator|animator|`Animator`|Handles animations for the widget.|
|walk|walk|`Walk`|Defines how the widget should be positioned and sized within its parent.|
|redraw, live|draw_bg|`DrawDesktopButton`|Handles the drawing properties of the desktop button, including its type and visual states.|

See [DrawDesktopButton](../types/draw_desktop_btn.md)

See [Walk](../types/walk.md)

See [Animator](../../syntax/animator.md)
## Event
|name|description|
|--|--|
|Pressed|Triggered when the button is pressed.|
|Clicked|Triggered when the button is clicked.|
|Released|Triggered when the button is released.|
