# CheckBox

表示复选框的小部件，可用于选择选项或切换状态。`CheckBox`小部件支持各种类型的复选框，包括标准复选框、单选按钮和切换。

A widget representing a check box, which can be used for selecting options or toggling states. The `CheckBox` widget supports various types of checkboxes including standard checks, radio buttons, and toggles.

## Props
|decorate|name|type|description|
|--|--|--|--|
|walk|walk|`Walk`|Defines how the widget should be positioned and sized within its parent.|
|layout|layout|`Layout`|Specifies the layout properties of the widget.|
|animator|animator|`Animator`|Handles animations for the widget.|
|live|icon_walk|`Walk`|Defines the position and size of the icon within the checkbox.|
|live|label_walk|`Walk`|Defines the position and size of the label text within the checkbox.|
|live|label_align|`Align`|Specifies the alignment of the label text.|
|redraw, live|draw_check|`DrawCheckBox`|Handles the drawing properties of the checkbox.|
|live|draw_text|`DrawText`|Handles the drawing properties of the label text.|
|live|draw_icon|`DrawIcon`|Handles the drawing properties of the icon.|
|live|text|`RcStringMut`|The text label of the checkbox.|
|live|bind|`String`|Specifies a binding string for the checkbox state.|

## Event
|name|description|
|--|--|
|Change(bool)|Triggered when the state of the checkbox changes.|
|None|No event.|

