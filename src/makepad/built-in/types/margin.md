# `Margin` Struct

Defines the external padding around a widget, determining its offset from adjacent items or container edges.

| Property Name | Type  |
|---------------|-------|
| left          | `f64` |
| top           | `f64` |
| right         | `f64` |
| bottom        | `f64` |

## Example

```rust
margin: 16.0,
margin: {left: 10.0},
margin: {left: 5.0, top: 6.0},
margin: {left: 10.0, right: 6.0, top: 10.0, bottom: 18.0},
```