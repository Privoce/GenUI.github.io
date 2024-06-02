# View

The `View` widget represents a UI element that can be rendered on the screen with various properties such as background color, layout, visibility, and more. It handles different events related to user interactions like finger movements and key presses.

“视图”小部件表示一个UI元素，该元素可以用各种属性（如背景色、布局、可见性等）在屏幕上呈现。它处理与用户交互相关的不同事件，如手指移动和按键。

## Props
|decorate|name|type|description|
|--|--|--|--|
|live|draw_bg|`DrawColor`|视图的背景图形颜色。<br />The background drawing color of the view.|
|live|show_bg|`bool`|确定是否显示背景。<br />Determines whether the background is shown.|
|layout|layout|`Layout`|定义视图的布局特性。<br />Defines the layout properties for the view.|
|walk|walk|`Walk`|指定视图的walk属性。<br />Specifies the walk properties for the view.|
|live|dpi_factor|`Option<f64>`|视图的DPI因子。<br />The DPI factor for the view.|
|live|optimize|`ViewOptimize`|视图的优化设置。<br />Optimization settings for the view.|
|live|debug|`ViewDebug`|Debug settings for the view.|
|live|event_order|`EventOrder`|The order in which events are processed.|
|live|visible|`bool`|Controls the visibility of the view.|
|live|grab_key_focus|`bool`|Indicates whether the view grabs key focus.|
|live|block_signal_event|`bool`|Determines if signal events are blocked.|
|live|cursor|`Option<MouseCursor>`|Specifies the mouse cursor for the view.|
|live|scroll_bars|`Option<LivePtr>`|Pointer to the scroll bars, if any.|
|live|design_mode|`bool`|Indicates if the view is in design mode.|
|rust|find_cache|`HashMap<u64, WidgetSet>`|Cache for finding widgets.|
|rust|scroll_bars_obj|`Option<Box<ScrollBars>>`|Scroll bars object, if any.|
|rust|view_size|`Option<DVec2>`|The size of the view.|
|rust|area|`Area`|The area occupied by the view.|
|rust|draw_list|`Option<DrawList2d>`|List of drawing operations for the view.|
|rust|texture_cache|`Option<ViewTextureCache>`|Cache for view textures.|
|rust|defer_walks|`Vec<(LiveId, DeferWalk)>`|Deferred walks for the view.|
|rust|draw_state|`DrawStateWrap<DrawState>`|State of the drawing operations.|
|rust|children|`ComponentMap<LiveId, WidgetRef>`|Child components of the view.|
|rust|draw_order|`Vec<LiveId>`|Order in which components are drawn.|
|animator|animator|`Animator`|Animator for the view.|
---

## Event
|name|description|
|--|--|
|FingerDown|当手指向下触摸视图时触发<br />Triggered when a finger touches down on the view.|
|FingerUp|当手指从视图中抬起时触发<br />Triggered when a finger is lifted from the view.|
|FingerMove|当手指在视图上移动时触发<br />Triggered when a finger moves on the view.|
|FingerHoverIn|当手指悬停在视图上时触发<br />Triggered when a finger hovers over the view.|
|FingerHoverOut|当手指停止在视图上悬停时触发<br />Triggered when a finger stops hovering over the view.|
|KeyDown|当视图具有焦点时按下某个键时触发<br />Triggered when a key is pressed down while the view has focus.|
|KeyUp|当视图具有焦点时释放键时触发<br />Triggered when a key is released while the view has focus.|