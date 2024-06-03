# Icon

`Icon`小部件是一个UI元素，用于显示具有可选背景的图标。它支持各种布局和漫游属性来控制其外观和位置。

The `Icon` widget is a UI element designed to display an icon with an optional background. It supports various layout and walk properties to control its appearance and positioning.


## Props
|decorate|name|type|description|
|--|--|--|--|
|redraw|draw_bg|`DrawQuad`|Draws the background of the icon.|
|live|draw_icon|`DrawIcon`|Renders the icon.|
|live|icon_walk|`Walk`|Defines the walk properties specifically for the icon.|
|walk|walk|`Walk`|Defines the walk properties for the entire widget.|
|layout|layout|`Layout`|Defines the layout properties for the widget.|

See [DrawIcon](../types/draw_icon.md)

## Event

This widget does not define any specific events.