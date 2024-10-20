# GDivider component
The `GDivider` component is used to create a simple dividing line between other UI elements. It can be horizontal or vertical based on the `direction` property.

## Animation
The `GDivider` inherits animation properties from `GView`, but typically, animations are not the primary focus for dividers. Instead, animations should be handled within inner components.

## Event
The `GDivider` inherits event handling from `GView`. However, since it functions mainly as a visual separator, its event handling is generally minimal.

## Props
|macro |prop           |description                          |type         |default|
|------|---------------|--------------------------------------|-------------|-------|
|live  |direction       |Divider direction: horizontal (1.0) or vertical (0.0)|f32 |1.0|
|live  |stroke_width    |The width of the divider's stroke    |f32          |1.4    |

> Other Props see: [GView](./view.md)
