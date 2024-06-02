# Window

In the Makepad framework, Window Widgets are the fundamental components used to create and manage application windows. As a top-level container, it is responsible for hosting other widgets of the application interface and provides a series of configuration options, allowing developers to customize the behavior and appearance of windows. By using Window Widgets, developers can define properties such as window size, title, and background color to create a window interface that meets application requirements.

在Makepad框架中，Window Widget 是用于创建和管理应用程序窗口的基础组件。它作为顶层容器，负责承载应用界面的其它Widgets，并且提供了一系列的配置选项，允许开发者定制窗口的行为和外观。通过使用 Window Widget，开发者能够定义窗口的大小、标题、背景颜色等属性，从而创建出符合应用需求的窗口界面。


## Props

|decorate 修饰|name|type|description|
|--|--|--|--|
|live|last_mouse_pos|`DVec2`|存储了最后一次鼠标位置|
|live|mouse_cursor_size|`DVec2`|存储鼠标光标的大小|
|live|demo|`bool`|指示是否处于演示模式|
|rust|demo_next_frame|`NextFrame`|控制演示模式下的下一帧渲染|
|live|cursor_draw_list|`DrawList2d`|定义在窗口中绘制光标时所需的绘制指令列表|
|live|draw_cursor|`DrawQuad`|绘制光标的矩形区域|
|live|debug_view|`DebugView`|显示调试信息|
|live|performance_view|`PerformanceView`|性能指标|
|live|nav_control|`NavControl`|用于处理导航相关的控制逻辑|
|live|window|`WindowHandle`|代表窗口的句柄或引用|
|live|stdin_size|`DrawColor`||
|rust|overlay|`Overlay`|用于在窗口上层显示额外的内容或界面元素|
|rust|main_draw_list|`DrawList2d`|存储主要内容的绘制指令|
|live|pass|`Pass`|用于描述渲染过程中的一个阶段或步骤|
|rust|depth_texture|`Texture`|存储深度纹理，这在3D渲染或需要深度测试的场景中非常重要|
|live|hide_caption_on_fullscreen|`bool`|控制全屏时是否隐藏标题栏|
|live|show_performance_view|`bool`|控制全屏时是否显示性能视图|
|deref|view|`View`|表示`Window`内部包含一个`View`结构体，用于处理窗口内部的视图渲染和布局|
|rust|draw_state|`DrawStateWrap<DrawState>`||


`View` props are deref. See: [View](./view.md)

## Event


|name|description|
|--|--|
|EventForOtherWindow|用于另一个窗口的事件。这用于在窗口之间传递事件。<br /> An event intended for another window. This is used to pass events between windows.|
|WindowClosed|表示窗口已关闭。当用户关闭窗口时会触发此事件。<br />Indicates that the window has been closed. This event is triggered when the user closes the window.|
|WindowGeomChange|窗口几何图形（大小、位置等）的更改。此事件通过“WindowGeomChangeEvent”携带有关几何图形更改的数据。<br />A change in the window's geometry (size, position, etc.). This event carries data about the geometry change through a `WindowGeomChangeEvent`.|

