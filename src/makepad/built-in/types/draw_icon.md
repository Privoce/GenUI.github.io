# DrawIcon

`DrawIcon`结构定义了绘制图标的各种属性，包括亮度、缩放和几何图形的参数，以及对SVG文件和路径的引用。

The `DrawIcon` struct defines various properties for drawing an icon, including parameters for brightness, scaling, and geometry, as well as references to SVG files and paths.

## Properties
|decorate|name|type|description|
|--|--|--|--|
|live           | brightness    | `f32`              | Sets the brightness level of the icon. Default is 1.0. |
|live           | curve         | `f32`              | Controls the curve parameter for rendering. Default is 0.6. |
|live           | linearize     | `f32`              | Determines the linearization level of the icon. Default is 0.5. |
|live           | svg_file      | `LiveDependency`   | Specifies the SVG file used for the icon. |
|live           | svg_path      | `Rc<String>`       | Path to the SVG file for the icon. |
|live           | translate     | `DVec2`            | Translation vector for positioning the icon. |
|live           | scale         | `f64`              | Scale factor for the icon. Default is 1.0. |
|rust           | many_instances| `Option<ManyInstances>` | Option for handling multiple instances. |
|live           | geometry      | `GeometryQuad2D`   | Geometry definition for the icon. |
|deref          | draw_vars     | `DrawVars`         | Variables used for drawing operations. |
|calc           | rect_pos      | `Vec2`             | Calculated position of the rectangle. |
|calc           | rect_size     | `Vec2`             | Calculated size of the rectangle. |
|calc           | draw_clip     | `Vec4`             | Calculated clipping rectangle for drawing. |
|live           | draw_depth    | `f32`              | Depth value for drawing the icon. Default is 1.0. |
|live           | color         | `Vec4`             | Color value for the icon. |
|calc           | icon_t1       | `Vec2`             | Transformation vector 1 for the icon. |
|calc           | icon_t2       | `Vec2`             | Transformation vector 2 for the icon. |

See [LiveDependency](./live_dep.md)

See [DVec2](./dvec2.md)

See [DrawVars](./draw_vars.md)

See [Vec2](./vec2.md)

See [Vec4](./vec4.md)
