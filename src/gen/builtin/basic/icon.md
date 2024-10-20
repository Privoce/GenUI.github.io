# GIcon component
The `GIcon` component is a customizable graphical icon with support for animations, events, and various properties to control its appearance and behavior.

## Animation
The `GIcon` component provides built-in animations for hover and focus effects. These animations transition smoothly between states based on user interactions.

- `hover`: 
  - **off**: Transitions to a state where the `hover` effect is disabled across all icons.
  - **on**: Applies a `hover` effect to the icons, transitioning them to a highlighted state.
  - **focus**: Applies a `focus` effect, transitioning the icons into focus mode.
  
## Event
The `GIcon` component supports various interaction events, enabling developers to listen and respond to user actions.

- `HoverIn(GIconHoverParam)`: Triggered when the icon is hovered over.
- `HoverOut(GIconHoverParam)`: Triggered when the hover effect is lost.
- `Focus(GIconFocusParam)`: Triggered when the icon gains focus.
- `Clicked(GIconClickedParam)`: Triggered when the icon is clicked.
- `FocusLost(GIconFocusLostParam)`: Triggered when the icon loses focus.

## Props

| macro  | prop               | description                                             | type               | default |
|--------|--------------------|---------------------------------------------------------|--------------------|---------|
| live   | theme               | Themes for styling the component                        | `Themes`           | -       |
| live   | color               | Icon color                                              | `Option<Vec4>`      | `None`  |
| live   | stroke_hover_color  | Stroke color on hover                                   | `Option<Vec4>`      | `None`  |
| live   | stroke_focus_color  | Stroke color on focus                                   | `Option<Vec4>`      | `None`  |
| live   | stroke_width        | Stroke width                                            | `f32`               | `1.0`   |
| live   | cursor              | Icon cursor style                                       | `Option<MouseCursor>`| `None`  |
| live   | visible             | Controls visibility of the icon                         | `bool`              | `true`  |
| live   | grab_key_focus      | Determines if the icon grabs key focus                  | `bool`              | `true`  |
| live   | animation_key       | Animation key control                                   | `bool`              | `false` |
| animator | animator          | Manages icon animations                                 | `Animator`          | -       |
| redraw | draw_icon           | Handles redrawing the icon                              | `DrawQuad`          | -       |
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
| live   | icon_base           | Base icon drawable                                      | `Option<DrawGIconBase>`| `None` |
| live   | icon_arrow          | Arrow icon drawable                                     | `Option<DrawGIconArrow>`| `None` |
| live   | icon_code           | Code icon drawable                                      | `Option<DrawGIconCode>` | `None` |
| live   | icon_emoji          | Emoji icon drawable                                     | `Option<DrawGIconEmoji>`| `None` |
| live   | icon_fs             | File system icon drawable                               | `Option<DrawGIconFs>`  | `None` |
| live   | icon_ui             | UI icon drawable                                        | `Option<DrawGIconUI>`  | `None` |
| live   | icon_person         | Person icon drawable                                    | `Option<DrawGIconPerson>`| `None` |
| live   | icon_relation       | Relation icon drawable                                  | `Option<DrawGIconRelation>`| `None` |
| live   | icon_state          | State icon drawable                                     | `Option<DrawGIconState>`| `None` |
| live   | icon_time           | Time icon drawable                                      | `Option<DrawGIconTime>`| `None` |
| live   | icon_tool           | Tool icon drawable                                      | `Option<DrawGIconTool>`| `None` |
| rust   | draw_type           | Controls the type of icon being drawn                   | `Option<DrawGIconType>`| `None` |
| live   | icon_type           | Specifies the type of icon                              | `IconType`          | -       |
| live   | event_key           | Event key control                                       | `bool`              | `true`  |
| rust   | scope_path          | Path to scope the icon                                  | `Option<HeapLiveIdPath>`| `None` |
