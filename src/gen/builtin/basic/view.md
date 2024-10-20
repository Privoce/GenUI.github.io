# GView Component

The `GView` component is designed for creating a custom graphical view with advanced layout, drawing, and event handling capabilities. It supports various graphical properties like background color, borders, shadows, and animations, providing flexibility in appearance and interaction.

## Animation

This component supports animations, particularly for hover and focus states. The default hover and focus animations are defined using the `animator` field:

- **hover.off**:  
  - `draw_view.hover`: changes to `0.0`  
  - `draw_view.focus`: changes to `0.0`  
  - Animation transition: uses `Forward` with a duration of `0.25s`
- **hover.on**:  
  - `draw_view.hover`: changes to `1.0`  
  - `draw_view.focus`: changes to `0.0`  
  - Animation transition: uses `Forward` with a duration of `0.25s`
- **hover.focus**:  
  - `draw_view.hover`: changes to `0.0`  
  - `draw_view.focus`: changes to `1.0`  
  - Animation transition: uses `Forward` with a duration of `0.25s`

Animations can be customized to control transitions between different states, ensuring smooth visual feedback for user interactions.

## Event

The `GView` component supports a variety of events for user interaction. It includes:
- **HoverIn**: Triggered when the mouse hovers into the component area.
- **HoverOut**: Triggered when the mouse leaves the component area.
- **Click**: Triggered when the component is clicked.
- **Drag**: Triggered when the component is dragged.
- **Key Events**: Handles `KeyDown` and `KeyUp` events for keyboard interactions.

Each event is processed through methods like `handle_event`, where interactions are managed and animations are triggered in response to user actions.

## Props

| Macro  | Prop               | Description                                      | Type               | Default  |
|--------|--------------------|--------------------------------------------------|--------------------|----------|
| live   | `theme`             | The visual theme (Dark or Light)                 | `Themes`           | `Themes::Dark` |
| live   | `background_color`  | The background color of the view                 | `Option<Vec4>`     | `None`   |
| live   | `hover_color`       | The color of the view on hover                   | `Option<Vec4>`     | `None`   |
| live   | `focus_color`       | The color of the view on focus                   | `Option<Vec4>`     | `None`   |
| live   | `border_color`      | The color of the view’s border                   | `Option<Vec4>`     | `None`   |
| live   | `border_width`      | The width of the border                          | `f32`              | `0.0`    |
| live   | `border_radius`     | The radius for rounded corners                   | `f32`              | `2.0`    |
| live   | `visible`           | Controls the visibility of the component         | `bool`             | `true`   |
| live   | `background_visible`| Controls the visibility of the background        | `bool`             | `true`   |
| live   | `shadow_color`      | The color of the shadow                          | `Option<Vec4>`     | `None`   |
| live   | `spread_radius`     | The radius of the shadow spread                  | `f32`              | `4.8`    |
| live   | `blur_radius`       | The radius of the shadow blur                    | `f32`              | `4.8`    |
| live   | `shadow_offset`     | The offset of the shadow                         | `Vec2`             | `(0.0, 0.0)` |
| live   | `cursor`            | The cursor to display when hovering over the view| `Option<MouseCursor>` | `None` |
| live   | `animation_key`     | Boolean to enable animations                     | `bool`             | `false`  |
| walk   | `abs_pos`           | Absolute position for layout                     | `Option<DVec2>`    | `None`   |
| walk   | `margin`            | Margin size around the view                      | `Margin`           | `Margin::default()` |
| walk   | `width`             | Width of the view                               | `Size`             | `Size::default()` |
| walk   | `height`            | Height of the view                              | `Size`             | `Size::default()` |
| layout | `scroll`            | Scroll position for layout                      | `DVec2`            | `(0.0, 0.0)` |
| layout | `clip_x`            | Clip content horizontally                       | `bool`             | `true`   |
| layout | `clip_y`            | Clip content vertically                         | `bool`             | `true`   |
| layout | `padding`           | Padding within the view                         | `Padding`          | `Padding::default()` |
| layout | `align`             | Alignment for content                           | `Align`            | `Align::default()` |
| layout | `flow`              | Flow direction of the content                   | `Flow`             | `Flow::default()` |
| layout | `spacing`           | Spacing between elements                        | `f64`              | `0.0`    |

