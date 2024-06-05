# Slider

`Slider`小部件是一个多功能的滑块组件，支持各种滑块类型（水平、垂直和旋转）。它与`TextInput`集成以实现精确的值输入，并支持动画、自定义绘图和事件处理以实现交互式用户体验。

The `Slider` widget is a versatile slider component that supports various slider types (horizontal, vertical, and rotary). It integrates with a `TextInput` for precise value entry and supports animations, custom drawing, and event handling for interactive user experiences.

## Props
|decorate|name|type|description|
|--|--|--|--|
|redraw, live|draw_slider|`DrawSlider`|Properties for drawing the slider.|
|walk|walk|`Walk`|Layout and positioning properties.|
|layout|layout|`Layout`|Layout properties for arranging child elements.|
|animator|animator|`Animator`|Animator for handling animation states.|
|live|label_walk|`Walk`|Walk properties for the label.|
|live|label_align|`Align`|Alignment properties for the label.|
|live|draw_text|`DrawText`|Properties for drawing the text.|
|live|text|`String`|The text content displayed by the slider.|
|live|text_input|`TextInput`|Embedded text input for precise value entry.|
|live|precision|`usize`|Number of decimal places for the displayed value.|
|live|min|`f64`|Minimum value of the slider.|
|live|max|`f64`|Maximum value of the slider.|
|live|step|`f64`|Step size for the slider.|
|live|default|`f64`|Default value of the slider.|
|live|bind|`String`|Binding identifier for data binding.|
|rust|value|`f64`|Current value of the slider.|
|rust|dragging|`Option<f64>`|Indicates if the slider is currently being dragged.|

See [DrawSlider](../types/draw_slider.md)

## Event
|name|description|
|--|--|
|StartSlide|Triggered when the user starts sliding.|
|TextSlide(f64)|Triggered when the text input value changes, updating the slider.|
|Slide(f64)|Triggered when the slider value changes.|
|EndSlide|Triggered when the user ends sliding.|
|None|Represents no action.|