# RadioType

定义不同类型单选按钮的枚举。

An enumeration defining different types of radio buttons.

## Properties
|decorate|name|type|description|
|--|--|--|--|
|pick|Round|`shader_enum(1)`|Standard round radio button.|
|live|Tab|`shader_enum(2)`|Tab-style radio button.|

## Example
```rust
draw_radio: {
    radio_type: Tab,
}
```