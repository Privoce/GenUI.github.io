# ScrollBar(s)

## ScrollBar

`滚动条`小部件管理一个具有水平和垂直选项的可自定义滚动条，提供平滑滚动和各种用户交互。

The `ScrollBar` widget manages a customizable scrollbar with both horizontal and vertical options, offering smooth scrolling and various user interactions.

## Props
|decorate|name|type|description|
|--|--|--|--|
|live| draw_bar| `DrawScrollBar`| The visual representation of the scroll bar.|
|live| pub bar_size| `f64`| The size of the scroll bar.|
|live| pub min_handle_size| `f64`| The minimum size of the scroll handle in pixels.|
|live| bar_side_margin| `f64`| The margin on the side of the scroll bar.|
|live| pub axis| `ScrollAxis`| The axis of scrolling (horizontal or vertical).|
|live| use_vertical_finger_scroll| `bool`| Enables vertical scrolling using finger gestures.|
|live| smoothing| `Option<f64>`| Optional smoothing factor for scrolling.|
|animator| animator| `Animator`| The animator for handling scroll animations.|
|rust| next_frame| `NextFrame`| Manages the next frame rendering.|
|rust| visible| `bool`| Visibility status of the scroll bar.|
|rust| view_total| `f64`| The total size of the view area.|
|rust| view_visible| `f64`| The visible portion of the view area.|
|rust| scroll_size| `f64`| The size of the scrollable area.|
|rust| scroll_pos| `f64`| The current scroll position.|
|rust| scroll_target| `f64`| The target scroll position.|
|rust| scroll_delta| `f64`| The change in scroll position.|
|rust| drag_point| `Option<f64>`| The point in pixels where dragging occurs.|

---
## Event
|name|description|
|--|--|
|Scroll|Triggered during scrolling with details on the position and view sizes.|
|ScrollDone|Triggered when scrolling is completed.|



## ScrollBars

`滚动条s`小部件管理水平和垂直滚动条的显示和行为。

The `ScrollBars` widget manages the display and behavior of horizontal and vertical scroll bars.

### Props
|decorate|name|type|description|
|--|--|--|--|
|live| show_scroll_x| `bool`| Determines if the horizontal scroll bar is shown.|
|live| show_scroll_y| `bool`| Determines if the vertical scroll bar is shown.|
|live| scroll_bar_x| `ScrollBar`| The horizontal scroll bar component.|
|live| scroll_bar_y| `ScrollBar`| The vertical scroll bar component.|
|rust| nav_scroll_index| `Option<NavScrollIndex>`| Navigation scroll index.|
|rust| scroll| `DVec2`| Current scroll position.|
|rust| area| `Area`| The rendering area of the scroll bars.|

---
### Event
|name|description|
|--|--|
|ScrollX|Triggered when the horizontal scroll position changes.|
|ScrollY|Triggered when the vertical scroll position changes.|

