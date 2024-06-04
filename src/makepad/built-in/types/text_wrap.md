# TextWrap
`TextWrap` 是一个枚举类型，用于定义文本的换行方式。

`TextWrap` is a enum, used for define the wrap way of the text

## Option Value

- `Ellipsis`: if overflow the defined width, the end of the text display ellipsis
- `Word`: if it overflow the defined width, it will automatically wrap
- `Line`: no wrap

## Example

```rust
wrap: Ellipsis,
wrap: Word,
wrap: Line,
```