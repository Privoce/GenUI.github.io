# DrawText

The `DrawText` struct is used for rendering text with various styles and properties in Makepad. It includes options for geometry, text style, wrapping, and various rendering details.

## Properties
|decorate|name|type|description|
|--|--|--|--|
|rust| many_instances| `Option<ManyInstances>`| Optional instance data for drawing many text objects. |
|live| geometry| `GeometryQuad2D`| The geometry used for rendering the text. |
|live| text_style| `TextStyle`| The style of the text, including font and size. |
|live| wrap| `TextWrap`| Specifies how text should wrap within its container. |
|live| ignore_newlines| `bool`| Determines whether newlines in the text should be ignored. |
|live| combine_spaces| `bool`| Specifies whether multiple consecutive spaces should be combined into one. |
|live| font_scale| `f64`| The scale factor for the font size. Default is 1.0. |
|live| draw_depth| `f32`| The drawing depth for layering text. Default is 1.0. |
|deref| draw_vars| `DrawVars`| Various variables used during the drawing process. |
|live| color| `Vec4`| The color of the text. |
|calc| font_t1| `Vec2`| Calculated texture coordinate t1 for the font. |
|calc| font_t2| `Vec2`| Calculated texture coordinate t2 for the font. |
|calc| rect_pos| `Vec2`| Calculated position of the text rectangle. |
|calc| rect_size| `Vec2`| Calculated size of the text rectangle. |
|calc| draw_clip| `Vec4`| Calculated clipping area for the text drawing. |
|calc| char_depth| `f32`| Calculated depth for individual characters. |
|calc| delta| `Vec2`| Calculated delta for text positioning. |
|calc| shader_font_size| `f32`| Calculated font size for the shader. |
|calc| advance| `f32`| Calculated advance value for text positioning. |

See [Vec2](./dvec2.md)

See [TextStyle](./text_style.md)

See [TextWrap](./text_wrap.md)

See [DrawVars](./draw_vars.md)

## Example

```rust
draw_text: {
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
}
```
