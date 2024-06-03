# Label

`Label`小部件用于在Makepad UI框架中显示文本。它包括用于自定义文本外观、对齐方式和填充的属性。

The `Label` widget is used to display text within the Makepad UI framework. It includes properties for customizing the text appearance, alignment, and padding.

## Props
|decorate|name|type|description|
|--|--|--|--|
|live|draw_text|`DrawText`|定义文字的图形特性，例如字体、大小和颜色。<br>Defines the drawing properties for the text, such as font, size, and color.|
|walk|walk|`Walk`|确定小部件在其父级中的布局行为<br>Determines the layout behavior of the widget within its parent.|
|live|align|`Align`|指定小部件内文本的对齐方式<br>Specifies the alignment of the text within the widget.|
|live|padding|`Padding`|Sets the padding around the text, affecting its positioning within the widget.|
|live|text|`RcStringMut`|小部件显示的文本内容。这可以动态更新。<br>The text content displayed by the widget. This can be dynamically updated.|

See [DrawText](../types/draw_text.md)

---

## Event
No events specified for this widget.

