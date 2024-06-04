# CheckType

定义不同类型复选框的枚举。

An enumeration defining different types of checkboxes.

## Properties
|decorate|name|type|description|
|--|--|--|--|
|pick|Check|`shader_enum(1)`|Standard checkbox.|
|live|Radio|`shader_enum(2)`|Radio button.|
|live|Toggle|`shader_enum(3)`|Toggle switch.|
|live|None|`shader_enum(4)`|No checkbox.|

## Example
```rust
draw_check: {
    check_type: Radio,
},
draw_check: {
    check_type: Toggle,
},
```