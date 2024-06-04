# DrawCheckBox

Handles the drawing properties specific to the checkbox. 

## Properties
|decorate|name|type|description|
|--|--|--|--|
|deref|draw_super|`DrawQuad`|Inherits properties from `DrawQuad`.|
|live|check_type|`CheckType`|Specifies the type of checkbox (Check, Radio, Toggle, None).|
|live|hover|`f32`|Represents the hover state.|
|live|focus|`f32`|Represents the focus state.|
|live|selected|`f32`|Represents the selected state.|

## Example
```rust
draw_check: {
    check_type: Radio,
    selected: 1.0,
},
```