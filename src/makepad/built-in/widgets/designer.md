# Designer

Makepad中的`Designer`组件是一种高级UI设计工具，允许开发人员以交互方式创建和管理应用程序布局和组件。提供的屏幕截图显示了`Designer`组件的界面，其中包括项目结构的层次视图和设计工作的画布区域。以下是对其目的和功能的更详细解释：

1. **层次视图**：
    - 在界面的左侧，有一个显示应用程序结构的层次树视图。这包括`应用程序`、`小部件`、`绘图`、`示例`等文件夹及其子文件夹。
    - 层次结构中的每个条目表示项目中的一个组件或文件。例如，`app`包含`KeyboardView`组件，`draw`包含`shader`和`geometry`等子文件夹。

2. **画布面积**：
    - 界面的右侧是一个画布，实际的设计工作就是在这里进行的。在这里，您可以直观地操作UI组件，并查看更改的实时效果。
    - 该区域当前在屏幕截图中显示为空，表示没有选择或显示任何组件进行编辑。

3. **组件属性和编辑**：
    - `设计器`组件允许用户在层次视图中选择不同的元素并编辑其属性。
    - 特性可以包括布局设置、视觉样式和功能属性。这些更改可以通过画布中的直接操作或通过调整属性面板（未在屏幕截图中显示，但通常是此类设计工具的一部分）中的属性来进行。

4. **项目组织机构**：
    - 层次视图有助于系统地组织项目文件和构件。用户可以浏览项目的不同部分，打开组件，并根据需要进行编辑。
    - 这种结构化视图有助于维护有组织的工作流，尤其是在具有多个组件和依赖项的大型项目中。

5. **实时预览**：
    - `设计器`组件的一个显著优势是能够实时预览更改。当您修改属性或布局元素时，画布会立即更新以反映这些更改，从而提供即时反馈。


The `Designer` component in Makepad serves as an advanced UI design tool, allowing developers to create and manage their application layouts and components interactively. The screenshot provided illustrates the interface of the `Designer` component, which includes a hierarchical view of the project structure and a canvas area for design work. Here's a more detailed explanation of its purpose and functionality:

1. **Hierarchical View**:
   - On the left side of the interface, there is a hierarchical tree view that displays the structure of the application. This includes folders such as `app`, `widgets`, `draw`, `examples`, and their subfolders.
    - Each entry in the hierarchy represents a component or a file in the project. For instance, `app` contains the `KeyboardView` component, and `draw` contains subfolders like `shader` and `geometry`.

2. **Canvas Area**:
    - The right side of the interface is a canvas where the actual design work takes place. Here, you can visually manipulate the UI components and see the real-time effects of your changes.
    - This area is currently shown as empty in the screenshot, indicating that no component is selected or displayed for editing.

3. **Component Properties and Editing**:
    - The `Designer` component allows users to select different elements in the hierarchical view and edit their properties.
    - Properties can include layout settings, visual styles, and functional attributes. These changes can be made through direct manipulation in the canvas or by adjusting properties in a properties panel (not shown in the screenshot but typically part of such design tools).

4. **Project Organization**:
    - The hierarchical view helps in organizing the project files and components systematically. Users can navigate through different parts of their project, open components, and edit them as needed.
    - This structured view aids in maintaining an organized workflow, especially in larger projects with multiple components and dependencies.

5. **Real-Time Preview**:
    - One of the significant advantages of the `Designer` component is the ability to see real-time previews of changes. As you modify properties or layout elements, the canvas updates immediately to reflect these changes, providing instant feedback.


## Props
|decorate|name|type|description|
|--|--|--|--|
|deref|ui|`View`|The UI view that the designer interacts with.|
|rust|data|`DesignerData`|Holds the internal data for the designer, including the design hierarchy and selected elements.|

---

## Event
This widget does not define any specific events.

## DesignerView

`DesignerView`小部件是Makepad UI框架中的一个专用视图，旨在为设计和编辑UI组件提供界面。它支持各种特性和功能，以管理设计器界面中的布局、背景图形和组件交互。

The `DesignerView` widget is a specialized view within the Makepad UI framework, designed to provide an interface for designing and editing UI components. It supports various properties and functionalities to manage the layout, background drawing, and component interactions within the designer interface.

### Props
|decorate|name|type|description|
|--|--|--|--|
|walk|walk|`Walk`|Defines the walking properties for the `DesignerView`.|
|rust|area|`Area`|The area occupied by the `DesignerView`.|
|rust|reapply|`bool`|A flag indicating whether properties need to be reapplied.|
|live|container|`Option<LivePtr>`|Pointer to the container, if any.|
|live|draw_bg|`DrawColor`|The background drawing color of the designer view.|
|rust|components|`ComponentMap<LivePtr, WidgetRef>`|Map of components within the designer view, keyed by `LivePtr`.|
|redraw rust|draw_list|`DrawList2d`|List of drawing operations for the designer view.|
|rust|pass|`Pass`|Rendering pass for the designer view.|
|rust|color_texture|`Option<Texture>`|Optional texture for color rendering.|

---

### Event
This widget does not define any specific events.