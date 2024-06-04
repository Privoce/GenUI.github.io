# DrawRadioButton

Handles the drawing properties specific to the radio button.

## Properties
|decorate|name|type|description|
|--|--|--|--|
|deref|draw_super|`DrawQuad`|Inherits properties from `DrawQuad`.|
|live|radio_type|`RadioType`|Specifies the type of radio button (Round, Tab).|
|live|hover|`f32`|Represents the hover state.|
|live|focus|`f32`|Represents the focus state.|
|live|selected|`f32`|Represents the selected state.|

---

~# RadioButtonGroup⛔~

~A widget representing a group of radio buttons, allowing for mutually exclusive selection.~

## Properties

|decorate|name|type|description|
|--|--|--|--|
|deref|frame|`View`|Inherits properties from `View`.|