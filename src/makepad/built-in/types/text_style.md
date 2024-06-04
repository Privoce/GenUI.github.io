# TextStyle

`TextStyle` 是一个用于描述文本样式的结构体。

`TextStyle` is used for describe text style

## Properties
|decorate|name|type|description|
| --- | --- | --- | --- |
| live | font | Font | 字体<br>font family |
| live(9.0) | font_size | f64 | 字体大小<br>define the size of the text |
| live(1.0) | brightness | f32 | 亮度<br>the brightness of text|
| live(0.5) | curve | f32 | 曲线<br>word curve|
| live(1.4) | line_spacing | f64 | 行间距<br>the spacing of word |
| live(1.1) | top_drop | f64 | 顶部偏移<br>top drop of the text |
| live(1.3) | height_factor | f64 | 高度因子<br>height factor of the text|

## Example

```rust
text_style: {
    // so here font size = 16 * 1.5 = 24
    font_size: 16,
    font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")},
    // brightness > 1.0 will make the text brighter
    // < 1.0 will make the text darker
    brightness: 1.0,
},
```