# RadioButton

表示单选按钮的小部件，可用于从一组互斥选项中选择一个选项。`RadioButton`小部件支持各种类型和媒体，包括图标和图像。

A widget representing a radio button, which can be used for selecting one option from a set of mutually exclusive options. The `RadioButton` widget supports various types and media, including icons and images.

## Props
|decorate|name|type|description|
|--|--|--|--|
|redraw, live|draw_radio|`DrawRadioButton`|Handles the drawing properties of the radio button.|
|live|draw_icon|`DrawIcon`|Handles the drawing properties of the icon within the radio button.|
|live|draw_text|`DrawText`|Handles the drawing properties of the text within the radio button.|
|live|value|`LiveValue`|The value associated with the radio button.|
|live|media|`MediaType`|Specifies the type of media (Image, Icon, None) for the radio button.|
|live|icon_walk|`Walk`|Defines the position and size of the icon within the radio button.|
|walk|walk|`Walk`|Defines how the widget should be positioned and sized within its parent.|
|live|image|`Image`|Specifies the image to be used for the radio button if media type is Image.|
|layout|layout|`Layout`|Specifies the layout properties of the widget.|
|animator|animator|`Animator`|Handles animations for the widget.|
|live|label_walk|`Walk`|Defines the position and size of the label text within the radio button.|
|live|label_align|`Align`|Specifies the alignment of the label text.|
|live|label|`String`|The text label of the radio button.|
|live|bind|`String`|Specifies a binding string for the radio button state.|

## Event
|name|description|
|--|--|
|Clicked|Triggered when the radio button is clicked.|
|None|No event.|

