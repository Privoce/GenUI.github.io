# DropDown

代表下拉菜单的小部件，允许用户从列表中选择项目。

A widget representing a dropdown menu, allowing users to select an item from a list.

## Props
|decorate|name|type|description|
|--|--|--|--|
|animator|animator|`Animator`|Handles the animations for the dropdown menu.|
|redraw, live|draw_bg|`DrawQuad`|Defines the background drawing properties of the dropdown menu.|
|live|draw_text|`DrawLabelText`|Defines the text drawing properties of the dropdown menu.|
|walk|walk|`Walk`|Defines how the dropdown menu should be positioned and sized within its parent.|
|live|bind|`String`|Binding property for the dropdown menu.|
|live|bind_enum|`String`|Enum binding property for the dropdown menu.|
|live|popup_menu|`Option<LivePtr>`|Pointer to the popup menu.|
|live|labels|`Vec<String>`|List of labels to display in the dropdown menu.|
|live|values|`Vec<LiveValue>`|List of values associated with the labels in the dropdown menu.|
|live|popup_menu_position|`PopupMenuPosition`|Position of the popup menu relative to the dropdown (OnSelected or BelowInput).|
|rust|is_open|`bool`|Indicates whether the dropdown menu is open.|
|live|selected_item|`usize`|Index of the selected item in the dropdown menu.|
|layout|layout|`Layout`|Defines the layout of the dropdown menu.|

See [Layout](../types/layout.md)

See [DrawLabelText](../types/draw_label_text.md)

See [DrawQuad](../types/draw_quad.md)

See [Walk](../types/walk.md)

See [PopupMenuPosition](../types/popup_menu_pos.md)


## Event
|name|description|
|--|--|
|Select(usize, LiveValue)|Triggered when an item is selected in the dropdown menu.|
|None||
