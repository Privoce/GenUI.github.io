# Padding

The `Padding` struct defines padding values for all four sides (left, top, right, bottom) of a widget.

## Properties
|decorate|name|type|description|
|--|--|--|--|
|live| left| `f64`| The padding on the left side. |
|live| top| `f64`| The padding on the top side. |
|live| right| `f64`| The padding on the right side. |
|live| bottom| `f64`| The padding on the bottom side. |

## Example

```rust
padding: 16.0,
padding: {left: 10.0},
padding: {left: 5.0, top: 6.0},
padding: {left: 10.0, right: 6.0, top: 10.0, bottom: 18.0},
```