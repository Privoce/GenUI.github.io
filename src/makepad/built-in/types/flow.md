# Flow

The `Flow` enum defines the direction and behavior of content flow within a layout in Makepad. It determines how child elements are arranged and wrapped.

## Properties
|decorate|name|type|description|
|--|--|--|--|
|live| Right| `Flow`| Content flows from left to right. |
|live| Down| `Flow`| Content flows from top to bottom. |
|live| Overlay| `Flow`| Content is overlaid on top of each other. |
|live| RightWrap| `Flow`| Content flows from left to right and wraps to the next line when it reaches the end of the container. |

## Example

```rust
flow: Down,
flow: Overlay,
flow: RightWrap,
flow: Right,
```