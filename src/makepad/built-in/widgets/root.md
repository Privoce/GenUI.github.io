# Root

Root是Makepad的UI入口，扮演十分重要的角色，所有的子widget都应该在该Root下被编写。

Root中包含一个窗口(Window)，因此当Root被创建后我们可以使用Window中的属性对它进行调节

The `Root` serves as the entry point for the UI in Makepad, playing a crucial role in the application. All child widgets should be defined under this `Root`.

The `Root` contains a `Window`, which allows us to adjust its properties once the `Root` is created.

Here is the documentation for the `Root` widget based on the provided struct:


## Props
|decorate|name|type|description|
|--|--|--|--|
|rust|draw_state|`DrawStateWrap<DrawState>`|Holds the state of the drawing operations for the `Root` widget.|
|rust|windows|`ComponentMap<LiveId, Window>`|A map of windows managed by the `Root`, keyed by their `LiveId`.|

---

## Event

This widget does not define any specific events.