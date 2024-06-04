# Image

An `Image` widget is used to display images within the UI. It supports various fitting modes and scales, and it can load images from different sources.

## Props

| decorate | name        | type               | description                                             |
|----------|-------------|--------------------|---------------------------------------------------------|
| walk     | walk        | `Walk`             | Determines the layout and positioning of the image.     |
| live     | draw_bg     | `DrawQuad`         | The drawing background quad for the image.              |
| live     | min_width   | `i64`              | The minimum width of the image.                         |
| live     | min_height  | `i64`              | The minimum height of the image.                        |
| live     | width_scale | `f64`              | The scale factor for the width of the image.            |
| live     | fit         | `ImageFit`         | The fitting mode for the image.                         |
| live     | source      | `LiveDependency`   | The source path or dependency for the image.            |
| rust     | texture     | `Option<Texture>`  | The texture of the image, if loaded.                    |

See[Walk](../types/walk.md)

See[DrawQuad](../types/draw_quad.md)

See[ImageFit](../types/image_fit.md)

See[LiveDependency](../types/live_dep.md)

## Event

No specific events are defined for the `Image` widget.