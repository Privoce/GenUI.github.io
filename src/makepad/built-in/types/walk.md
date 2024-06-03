# `Walk` 

The `Walk` struct is a fundamental part of layout management in Makepad, allowing for flexible and precise positioning and sizing of widgets. Here's a detailed breakdown of its properties:

| Property Name | Type               |
|---------------|--------------------|
| abs_pos       | `Option<DVec2>`    |
| margin        | `Margin`           |
| width         | `Size`             |
| height        | `Size`             |


See [DVec2](./dvec2.md)

See [Margin](./margin.md)

See [Size](./size.md)

## Example

```rust
{
    height: Fit,
    width: Fill,
    abs_pos: vec2(200. ,9.),
    margin: 16.0,
}
```