# GLabel component
The `GLabel` component is a customizable label widget with animation and event handling features. It allows for hover, focus, and text styling through various properties, animations, and events.

## Animation
The `GLabel` component comes with built-in animations for hover and focus states. The following animations are provided:

- **hover.off**:  
  - `draw_text.hover`: changes to `0.0`  
  - `draw_text.focus`: changes to `0.0`  
  - Animation transition: uses `Forward` with a duration of `0.25s`
- **hover.on**:  
  - `draw_text.hover`: changes to `1.0`  
  - `draw_text.focus`: changes to `0.0`  
  - Animation transition: uses `Forward` with a duration of `0.25s`
- **hover.focus**:  
  - `draw_text.hover`: changes to `0.0`  
  - `draw_text.focus`: changes to `1.0`  
  - Animation transition: uses `Forward` with a duration of `0.25s`

Animations are controlled by the `Animator` property, which defines the behavior for transitions between hover and focus states.

## Event
The `GLabel` component supports the following events:
- `HoverIn`: Triggered when a user hovers over the label.
- `HoverOut`: Triggered when the user moves the cursor away from the label.
- `Focus`: Triggered when the label is clicked or focused.
- `FocusLost`: Triggered when the label loses focus.

These events allow interaction with the label and can be used to trigger actions or further animations in response to user input.

## Props
| Macro   | Prop                | Description                                      | Type            | Default     |
|---------|---------------------|--------------------------------------------------|-----------------|-------------|
| live    | stroke_hover_color   | The color of the text stroke when hovered        | `Option<Vec4>`  | `None`      |
| live    | stroke_focus_color   | The color of the text stroke when focused        | `Option<Vec4>`  | `None`      |
| live    | color                | The base color of the text                       | `Option<Vec4>`  | `None`      |
| live    | font_size            | The font size of the label text                  | `f64`           | `9.0`       |
| live    | cursor               | The cursor type when hovering over the label     | `Option<MouseCursor>` | `None`      |
| live    | line_spacing         | The line spacing between the label text          | `f64`           | `1.5`       |
| live    | height_factor        | Factor controlling the height of the text        | `f64`           | `0.0`       |
| live    | wrap                 | Text wrapping behavior                          | `TextWrap`      | `TextWrap::Word` |
| live    | font_family          | The font family used for the label               | `LiveDependency` | N/A         |
| live    | visible              | Whether the label is visible                     | `bool`          | `true`      |
| redraw  | draw_text            | Controls the drawing of the label's text         | `DrawGText`     | N/A         |
| walk    | walk                 | Defines the positioning of the label             | `Walk`          | N/A         |
| live    | align                | Text alignment                                  | `Align`         | N/A         |
| live    | padding              | The padding around the label                    | `Padding`       | N/A         |
| live    | text                 | The text content of the label                   | `ArcStringMut`  | N/A         |
| animator| animator             | Handles animation states for hover and focus    | `Animator`      | N/A         |
| rust    | area                 | Represents the area occupied by the label       | `Area`          | N/A         |
| live    | event_key            | Controls the event triggering behavior           | `bool`          | `false`     |
| live    | grab_key_focus       | Whether the label grabs keyboard focus           | `bool`          | `true`      |
| rust    | scope_path           | The path scope for the label                    | `Option<HeapLiveIdPath>` | N/A  |
