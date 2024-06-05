# TextInput

`TextInput`小部件是一个可自定义的文本输入字段，具有各种样式和行为选项。它支持动画、光标样式、文本选择和撤消/重做功能等功能。

The `TextInput` widget is a customizable text input field with various styling and behavior options. It supports features like animation, cursor styling, text selection, and undo/redo functionality.

## Props
|decorate|name|type|description|
|--|--|--|--|
|animator|animator|`Animator`|Animator for handling animation states.|
|redraw, live|draw_bg|`DrawColor`|Background color drawing properties.|
|live|draw_select|`DrawQuad`|Drawing properties for text selection.|
|live|draw_cursor|`DrawQuad`|Drawing properties for the cursor.|
|live|draw_text|`DrawLabel`|Drawing properties for the text.|
|walk|walk|`Walk`|Layout and positioning properties.|
|layout|layout|`Layout`|Layout properties for arranging child elements.|
|live|label_align|`Align`|Alignment properties for the label.|
|live|cursor_size|`f64`|Size of the cursor.|
|live|cursor_margin_bottom|`f64`|Bottom margin for the cursor.|
|live|cursor_margin_top|`f64`|Top margin for the cursor.|
|live|select_pad_edges|`f64`|Padding edges for text selection.|
|live|empty_message|`String`|Message displayed when the input is empty.|
|live|numeric_only|`bool`|Restricts input to numeric values only.|
|live|secret|`bool`|Hides the text input for password fields.|
|live|on_focus_select_all|`bool`|Selects all text when the input gains focus.|
|live|read_only|`bool`|Makes the text input read-only.|
|live|text|`String`|The text content of the input field.|
|live|ascii_only|`bool`|Restricts input to ASCII characters only.|
|rust|double_tap_start|`Option<(usize, usize)>`|Stores the start position for double-tap selection.|
|rust|undo_id|`u64`|Identifier for the undo action.|
|rust|last_undo|`Option<UndoItem>`|The last undo item.|
|rust|undo_stack|`Vec<UndoItem>`|Stack of undo items.|
|rust|redo_stack|`Vec<UndoItem>`|Stack of redo items.|
|rust|cursor_tail|`usize`|Tail position of the cursor.|
|rust|cursor_head|`usize`|Head position of the cursor.|

---
## Event
|name|description|
|--|--|
|Change(String)|Triggered when the text content changes.|
|Return(String)|Triggered when the return key is pressed.|
|Escape|Triggered when the escape key is pressed.|
|KeyFocus|Triggered when the input gains keyboard focus.|
|KeyFocusLost|Triggered when the input loses keyboard focus.|
|None|Represents no action.|