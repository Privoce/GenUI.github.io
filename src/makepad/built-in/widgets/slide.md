# SlidesView

`SlidesView`(幻灯片视图)小部件代表幻灯片的容器，用于管理幻灯片的布局、渲染和动画。它允许幻灯片之间的平滑过渡。

The `SlidesView` widget represents a container for slides, managing their layout, rendering, and animations. It allows for smooth transitions between slides.

## Props
|decorate|name|type|description|
|--|--|--|--|
|layout| layout| `Layout`| Layout properties for arranging the slides.|
|rust| area| `Area`| Rendering area of the slides.|
|walk| walk| `Walk`| Walking properties to define how the slides occupy space.|
|rust| children| `ComponentMap<LiveId, WidgetRef>`| Map of child widgets representing individual slides.|
|rust| draw_order| `Vec<LiveId>`| Order in which slides are drawn.|
|rust| next_frame| `NextFrame`| Manages the scheduling of the next frame for animations.|
|live| current_slide| `f64`| Index of the currently visible slide.|
|live| goal_slide| `f64`| Target slide index for animations.|
|live| anim_speed| `f64`| Speed of the slide transition animations.|
|rust| draw_state| `DrawStateWrap<DrawState>`| Manages the state of the drawing process.|

---

## Event
No events defined for this widget.