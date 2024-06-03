# `Size` Enum

Specifies the sizing behavior of a widget, allowing for flexible layout arrangements.

| Variant Name | Associated Data |
|--------------|-----------------|
| Fill         | None            |
| Fixed        | `f64`           |
| Fit          | None            |
| All          | None            |

The `Size` enum facilitates various sizing strategies, such as:
- `Fill`: filling available space
- `Fixed`: setting a fixed dimension
- `Fit`: adjusting to content
- `All`: a special mode that applies a unique sizing logic

## Example

```rust
height: Fill,
width: Fit,
width: 64.0,
width: All,
```