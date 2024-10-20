# GSvg Component
The `GSvg` component is designed to handle scalable vector graphics (SVG) with hover and focus animations, supporting various visual properties such as stroke colors, scale, and cursor interactions.

## Animation
The `GSvg` component utilizes an animator to handle hover and focus transitions, allowing smooth changes in visual states. Below are the animations defined for different states:

- **hover.off**:  
  - `draw_svg.hover`: changes to `0.0`  
  - `draw_svg.focus`: changes to `0.0`  
  - Animation transition: uses `Forward` with a duration of `0.25s`

- **hover.on**:  
  - `draw_svg.hover`: changes to `1.0`  
  - `draw_svg.focus`: remains `0.0`  
  - Animation transition: uses `Forward` with a duration of `0.25s` for both `hover` and `focus` states

- **hover.focus**:  
  - `draw_svg.hover`: changes to `0.0`  
  - `draw_svg.focus`: changes to `1.0`  
  - Animation transition: uses `Forward` with a duration of `0.25s`

## Event
The `GSvg` component can handle various events, allowing interaction with the user. It triggers events like `Clicked`, `HoverIn`, `HoverOut`, `Focus`, and `FocusLost` when corresponding actions occur.

## Props
|macro  |prop                 |description                   |type            |default|
|-------|---------------------|-------------------------------|----------------|-------|
|live   |theme                |Theme settings                 |Themes          |       |
|live   |brightness           |Brightness level               |f32             |1.0    |
|live   |curve                |Curve intensity                |f32             |0.6    |
|live   |linearize            |Linearize factor               |f32             |0.5    |
|live   |src                  |SVG source dependency          |LiveDependency  |       |
|live   |scale                |Scaling factor                 |f64             |1.0    |
|live   |color                |SVG fill color                 |Option<Vec4>    |None   |
|live   |draw_depth           |Drawing depth                  |f32             |1.0    |
|live   |stroke_hover_color   |Color on hover                 |Option<Vec4>    |None   |
|live   |stroke_focus_color   |Color on focus                 |Option<Vec4>    |None   |
|live   |cursor               |Mouse cursor when hovered      |Option<MouseCursor>|None |
|live   |grab_key_focus       |Enable key focus grabbing      |bool            |true   |
|live   |visible              |Visibility of the component    |bool            |true   |
|live   |animation_key        |Triggers animation when true   |bool            |false  |
|animator|animator            |Handles animations             |Animator        |       |
|walk   |abs_pos              |Absolute position              |Option<DVec2>   |None   |
|walk   |margin               |Margin space                   |Margin          |       |
|walk   |width                |Component width                |Size            |       |
|walk   |height               |Component height               |Size            |       |
|layout |scroll               |Scroll position                |DVec2           |       |
|layout |clip_x               |Enable horizontal clipping     |bool            |true   |
|layout |clip_y               |Enable vertical clipping       |bool            |true   |
|layout |padding              |Padding around content         |Padding         |       |
|layout |align                |Alignment of content           |Align           |       |
|layout |flow                 |Flow direction of content      |Flow            |       |
|layout |spacing              |Spacing between elements       |f64             |       |
