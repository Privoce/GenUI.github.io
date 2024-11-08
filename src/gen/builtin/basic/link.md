# GLink component
A GLink component is used to create interactive, styled links with hover, focus, and click events.
It supports animations and various customizable properties like color, text, and visibility.

## Animation
The GLink component supports hover and focus animations, transitioning between different visual states.
- **hover.off**:
    - `draw_link.hover`: 0.0 and `draw_text.hover`: 0.0
    - `draw_link.focus`: 0.0 and `draw_text.focus`: 0.0
    - Animation transition: uses Forward with a duration of 0.25s
- **hover.on**:
   - `draw_link.hover`: 1.0 and `draw_text.hover`: 1.0
   - `draw_link.focus`: 0.0 and `draw_text.focus`: 0.0
   - Animation transition: uses Forward with a duration of 0.25s
- **hover.focus**:
   - `draw_link.hover`: 1.0 and `draw_text.hover`: 1.0
   - `draw_link.focus`: 1.0 and `draw_text.focus`: 1.0
   - Animation transition: uses Forward with a duration of 0.25s
## Event
GLink handles several user events such as hover and clicks.
- `HoverIn`: Triggered when the mouse starts hovering over the link.
- `HoverOut`: Triggered when the mouse stops hovering over the link.
- `Clicked`: Triggered when the link is clicked.
- `Focus`: Triggered when the link receives focus.
- `FocusLost`: Triggered when the link loses focus.

## Props
|macro  |prop                     |description                                 |type              |default                |
|-------|-------------------------|--------------------------------------------|------------------|-----------------------|
|live   |theme                    | Theme of the link                          |`Themes`          |`None`                 |
|live   |background_color         | Background color                           |`Option<Vec4>`    |`None`                 |
|live   |hover_color              | Hover background color                     |`Option<Vec4>`    |`None`                 |
|live   |focus_color              | Focus background color                     |`Option<Vec4>`    |`None`                 |
|live   |border_color             | Border color                               |`Option<Vec4>`    |`None`                 |
|live   |underline_visible        | Show underline when visible                |`bool`            |`true`                 |
|live   |underline_color          | Underline color                            |`Option<Vec4>`    |`None`                 |
|live   |underline_hover_color    | Underline color when hovering              |`Option<Vec4>`    |`None`                 |
|live   |underline_focus_color    | Underline color when focused               |`Option<Vec4>`    |`None`                 |
|live   |underline_width          | Width of the underline                     |`f32`             |`1.0`                  |
|live   |border_radius            | Border radius for rounding corners         |`f32`             |`4.0`                  |
|live   |round                    | Make the link round                        |`bool`            |`false`                |
|live   |background_visible       | Toggle visibility of the background        |`bool`            |`false`                |
|live   |text                     | The text content of the link               |`ArcStringMut`    |`""`                   |
|live   |font_size                | Size of the font                           |`f64`             |`10.0`                 |
|live   |color                    | Text color                                 |`Option<Vec4>`    |`None`                 |
|live   |text_hover_color         | Text color when hovered                    |`Option<Vec4>`    |`None`                 |
|live   |text_focus_color         | Text color when focused                    |`Option<Vec4>`    |`None`                 |
|live   |font_family              | Font family for the text                   |`LiveDependency`  |`None`                 |
|live   |cursor                   | Cursor style when hovering over the link   |`Option<MouseCursor>` |`None`             |
|live   |href                     | The URL for the link                       |`Option<String>`  |`None`                 |
|live   |link_type                | Type of link (internal, external, etc.)    |`LinkType`        |`None`                 |
|live   |visible                  | Visibility of the link                     |`bool`            |`true`                 |
|live   |draw_text                | Draw settings for text                     |`DrawGText`       |`None`                 |
|walk   |abs_pos                  | Absolute position for layout               |`Option<DVec2>`   |`None`                 |
|walk   |margin                   | Margin size around the view                |`Margin`          |`Margin::default()`    |
|walk   |width                    | Width of the view                          |`Size`            |`Size::default()`      |
|walk   |height                   | Height of the view                         |`Size`            |`Size::default()`      |
|layout |scroll                   | Scroll position for layout                 |`DVec2`           |`(0.0, 0.0)`           |
|layout |clip_x                   | Clip content horizontally                  |`bool`            |`true`                 |
|layout |clip_y                   | Clip content vertically                    |`bool`            |`true`                 |
|layout |padding                  | Padding within the view                    |`Padding`         |`Padding::default()`   |
|layout |align                    | Alignment for content                      |`Align`           |`Align::default()`     |
|layout |flow                     | Flow direction of the content              |`Flow`            |`Flow::default()`      |
|layout |spacing                  | Spacing between elements                   |`f64`             |`0.0`                  |