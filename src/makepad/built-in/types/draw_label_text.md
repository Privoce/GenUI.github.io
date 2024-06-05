# DrawLabelText
Structure representing the drawing properties of label text.

## Properties
|decorate|name|type|description|
|--|--|--|--|
|deref|draw_super|`DrawText`|Base drawing properties inherited from `DrawText`.|
|live|focus|`f32`|Focus state for the label text.|
|live|hover|`f32`|Hover state for the label text.|
|live|pressed|`f32`|Pressed state for the label text.|

See [DrawText](./draw_text.md)

## Example

```rust
{
    draw_super: {
        wrap: Word,
        // real font size = font size * font scale
        font_scale: 1.5,
        text_style: {
            // so here font size = 16 * 1.5 = 24
            font_size: 16,
            font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")},
            // brightness > 1.0 will make the text brighter
            // < 1.0 will make the text darker
            brightness: 1.0,
        },
        color: #FF0000,
    },
    // 0.0 is false
    focus: 0.0,
    hover: 1.0,
    pressed: 0.0,
}
```
