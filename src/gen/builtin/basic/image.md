# GImage Component
The `GImage` component handles scalable images with adjustable rotation, scaling, and visibility, supporting event triggers such as hovering and clicking.

## Animation
No animation is available for the `GImage` component.

## Event
The `GImage` component can trigger various events in response to user interactions:
- `HoverIn(GImageHoverParam)`: Triggered when the mouse pointer enters the image area.
- `HoverOut(GImageHoverParam)`: Triggered when the mouse pointer leaves the image area.
- `Clicked(GImageClickedParam)`: Triggered when the image is clicked.

## Props
|macro   |prop             |description                    |type               |default |
|--------|-----------------|-------------------------------|-------------------|--------|
|live    |visible           |Visibility of the component    |bool               |true    |
|live    |grab_key_focus    |Enable key focus grabbing      |bool               |true    |
|live    |opacity           |Opacity level                  |f32                |1.0     |
|live    |cursor            |Mouse cursor when hovered      |Option<MouseCursor>|None    |
|live    |scale             |Scaling factor                 |f64                |1.0     |
|live    |fit               |Image fit type                 |ImageFit           |        |
|live    |min_width         |Minimum width of the image     |i64                |16      |
|live    |min_height        |Minimum height of the image    |i64                |16      |
|live    |rotation          |Rotation angle in radians      |f32                |0.0     |
| walk   | `abs_pos`           | Absolute position for layout             | `Option<DVec2>`    | `None`   |
| walk   | `margin`            | Margin size around the view              | `Margin`           | `Margin::default()` |
| walk   | `width`             | Width of the view                        | `Size`             | `Size::default()` |
| walk   | `height`            | Height of the view                       | `Size`             | `Size::default()` |
| layout | `scroll`            | Scroll position for layout               | `DVec2`            | `(0.0, 0.0)` |
| layout | `clip_x`            | Clip content horizontally                | `bool`             | `true`   |
| layout | `clip_y`            | Clip content vertically                  | `bool`             | `true`   |
| layout | `padding`           | Padding within the view                  | `Padding`          | `Padding::default()` |
| layout | `align`             | Alignment for content                    | `Align`            | `Align::default()` |
| layout | `flow`              | Flow direction of the content            | `Flow`             | `Flow::default()` |
| layout | `spacing`           | Spacing between elements                 | `f64`              | `0.0`    |
|live    |draw_image        |The image drawing object       |DrawGView          |        |
|live    |src               |Image source dependency        |LiveDependency     |        |
|live    |texture           |Texture object                 |Option<Texture>    |None    |
|live    |event_key         |Trigger events when true       |bool               |true    |