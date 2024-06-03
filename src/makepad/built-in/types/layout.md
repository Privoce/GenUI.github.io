# Layout
`Layout`结构定义了用于管理布局的各种属性，包括滚动、剪切、填充、对齐、流和间距的参数。

The `Layout` struct defines various properties for managing layout, including parameters for scrolling, clipping, padding, alignment, flow, and spacing.

## Properties
|decorate|name|type|description|
|--|--|--|--|
|live| scroll| `DVec2`| Specifies the scrolling vector for the layout. |
|live| clip_x| `bool`| Determines whether the layout clips horizontally. Default is true. |
|live| clip_y| `bool`| Determines whether the layout clips vertically. Default is true. |
|live| padding| `Padding`| Sets the padding around the layout. |
|live| align| `Align`| Controls the alignment within the layout. |
|live| flow| `Flow`| Defines the flow direction of elements in the layout. |
|live| spacing| `f64`| Specifies the spacing between elements in the layout. |
|live| line_spacing| `f64`| Determines the line spacing for elements in the layout. |

See [DVec2](./dvec2.md)

See [Padding](./padding.md)

See [Align](./align.md)

See [Flow](./flow.md)
