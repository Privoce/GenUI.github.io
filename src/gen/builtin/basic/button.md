# GButton Component

The `GButton` component is a customizable button designed for interactive UI elements. It supports hover, focus, and click animations, with various configurable properties for appearance, including background color, shadow, and border styles.

## Animation

This component supports animations, particularly for hover and focus states. The default hover and focus animations are defined using the `animator` field:

- **hover.off**:  
  - `draw_button.hover`: changes to `0.0`  
  - `draw_button.focus`: changes to `0.0`  
  - Animation transition: uses `Forward` with a duration of `0.25s`
- **hover.on**:  
  - `draw_button.hover`: changes to `1.0`  
  - `draw_button.focus`: changes to `0.0`  
  - Animation transition: uses `Forward` with a duration of `0.25s`
- **hover.focus**:  
  - `draw_button.hover`: changes to `0.0`  
  - `draw_button.focus`: changes to `1.0`  
  - Animation transition: uses `Forward` with a duration of `0.25s`

## Event

The `GButton` component supports the following events:

- **HoverIn**: Triggered when the mouse hovers over the button.
- **HoverOut**: Triggered when the mouse leaves the button.
- **Clicked**: Triggered when the button is clicked.
- **Focus**: Triggered when the button receives focus (e.g., via a keyboard event).
- **FocusLost**: Triggered when the button loses focus.

These events can be customized to trigger additional behaviors based on user interactions.

## Props

| macro  | prop               | description                               | type             | default   |
|--------|--------------------|-------------------------------------------|------------------|-----------|
| live   | theme              | Theme of the button                       | `Themes`         |           |
| live   | background_color   | Background color of the button            | `Option<Vec4>`   | `None`    |
| live   | background_visible | Visibility of the background              | `bool`           | `true`    |
| live   | hover_color        | Color of the button when hovered          | `Option<Vec4>`   | `None`    |
| live   | focus_color        | Color of the button when focused          | `Option<Vec4>`   | `None`    |
| live   | shadow_color       | Color of the shadow                       | `Option<Vec4>`   | `None`    |
| live   | spread_radius      | Spread radius of the shadow               | `f32`            | `0.0`     |
| live   | blur_radius        | Blur radius of the shadow                 | `f32`            | `4.8`     |
| live   | shadow_offset      | Offset of the shadow                      | `Vec2`           |           |
| live   | border_color       | Color of the border                       | `Option<Vec4>`   | `None`    |
| live   | border_width       | Width of the border                       | `f32`            | `0.0`     |
| live   | border_radius      | Radius of the border's corners            | `f32`            | `2.0`     |
| live   | cursor             | Mouse cursor when hovering over the button| `Option<MouseCursor>`|        |
| live   | visible            | Whether the button is visible             | `bool`           | `true`    |
| live   | grab_key_focus     | Whether the button grabs keyboard focus   | `bool`           | `true`    |
| animator | animator         | Animation controller for the button       | `Animator`       |           |
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

