
# DrawSlider
The `DrawSlider` struct represents the drawable properties of a slider, including its position and type. It builds upon the `DrawQuad` base to provide customized slider visuals.

## Properties
|decorate|name|type|description|
|--|--|--|--|
|deref| draw_super| `DrawQuad`| Base properties for drawing the quad.|
|live| slide_pos| `f32`| Position of the slider on its axis.|
|live| slider_type| `SliderType`| Type of the slider (horizontal, vertical, rotary).|

See [SliderType](./slider_type.md)