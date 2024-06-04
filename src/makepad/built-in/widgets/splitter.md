# Splitter

`Splitter`小部件允许将容器划分为水平或垂直两个可调整大小的面板。它提供了拖动分界线以动态调整两个面板大小的能力。

The `Splitter` widget allows the division of a container into two resizable panels, either horizontally or vertically. It provides the ability to drag the dividing line to adjust the sizes of the two panels dynamically.

## Props
|decorate|name|type|description|
|--|--|--|--|
|live|axis|`SplitterAxis`|Defines the direction of the split: horizontal or vertical.|
|live|align|`SplitterAlign`|Specifies the initial alignment or weight of the splitter.|
|rust|rect|`Rect`|Stores the rectangle area of the splitter.|
|rust|position|`f64`|Holds the current position of the splitter.|
|rust|drag_start_align|`Option<SplitterAlign>`|Tracks the alignment during the drag operation.|
|rust|area_a|`Area`|Defines the area of the first panel.|
|rust|area_b|`Area`|Defines the area of the second panel.|
|animator|animator|`Animator`|Manages animations for the splitter.|
|live|min_vertical|`f64`|Minimum size for vertical splitting.|
|live|max_vertical|`f64`|Maximum size for vertical splitting.|
|live|min_horizontal|`f64`|Minimum size for horizontal splitting.|
|live|max_horizontal|`f64`|Maximum size for horizontal splitting.|
|redraw live|draw_splitter|`DrawSplitter`|Handles the drawing of the splitter.|
|live|split_bar_size|`f64`|Width of the splitter bar.|
|rust|draw_state|`DrawStateWrap<DrawState>`|Tracks the drawing state of the splitter.|
|find live|a|`WidgetRef`|Reference to the first widget.|
|find live|b|`WidgetRef`|Reference to the second widget.|
|walk|walk|`Walk`|Defines the layout and positioning of the splitter.|

See [SplitterAxis](../types/splitter.md)

See [SplitterAlign](../types/splitter.md)

See [Walk](../types/walk.md)

See [WidgetRef](../types/widget_ref.md)


---

## Event

No specific events associated with this widget.