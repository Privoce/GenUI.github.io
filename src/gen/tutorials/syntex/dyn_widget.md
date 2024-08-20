以下是优化后的文档内容：

# 动态 Widget

## 动态 Widget 类型分类

动态 Widget 根据使用方式可分为以下三类：

1. **A 类**：未使用 `gen_macros::Prop` 标注，且未声明 `id` 属性。
2. **B 类**：未使用 `gen_macros::Prop` 标注，但声明了 `id` 属性。
3. **C 类**：使用了 `gen_macros::Prop` 标注，但未声明 `id` 属性。

> 注意：
> 1. 如果同时使用了 `gen_macros::Prop` 并且声明了 `id` 属性，而 `${prop_struct_name} == ${id}`，则该组件属于 C 类。
> 2. 但如果 `${prop_struct_name} != ${id}`，程序将会 panic。

### A 类

当组件既没有使用 `Prop` 标注，也没有在 `<component></component>` 标签中声明 `id` 属性时：

```rust
<template>
    <component inherits="view"></component>
</template>
```

此时，动态 Widget 的结构体名为：`${source_name}_${inherits}`。

### B 类

当组件未使用 `Prop` 标注，但在 `<component></component>` 标签中声明了 `id` 属性时：

```rust
<template>
    <component id="MyView" inherits="view"></component>
</template>
```

此时，动态 Widget 的结构体名为 `${id}`，在这个例子中就是 `MyView`。

### C 类

当组件使用了 `Prop` 标注，但未声明 `id` 属性时：

```rust
<template>
    <component inherits="view"></component>
</template>

<script>
use gen_macros::Prop; // 可省略

#[derive(Prop)]
pub struct MyView {}
</script>
```

此时，动态 Widget 的结构体名为 `${prop_struct_name}`，在这个例子中就是 `MyView`。

---

## 脚本使用原则

### 生命周期管理

- **before**: 最早调用，用于初始化 `PropStruct`。
- **after**: 在 `before` 之后调用，用于声明式地修改 `PropStruct` 的值。

#### 示例

```rust
<script>
use gen_macros::{Prop} // 可以省略

#[derive(Prop)]
pub struct MyView{
    pub label_value1: String
}
---------------------------------------------------------------|
impl Default for MyView{                                       |
    fn default() -> Self {                                     |
        Self{                                                  |
            label_value1: "Click The Button".to_string()       |
        }                                                    before
    }                                                          |
}                                                              |
                                                               |
let mut prop = MyView::default();                              |
---------------------------------------------------------------|

---------------------------------------------------------------|
prop.label_value1 = "Click Here!".to_string();               after
---------------------------------------------------------------|
</script>
```

#### 编译后的代码

```rust
impl LiveHook for MyView {
    fn before_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.label_value1 = "Click The Button".to_string();
    }
    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.label_value1 = "Click Here!".to_string();
    }
}
```

## 示例

动态 Widget 采用 `<component>` 标签作为唯一根标签，使用 `inherits` 属性指定要继承的 Widget 类型，通常推荐继承 `view`。

- `use gen_macros::{Event, Prop};` 这行代码可以省略，`Event` 和 `Prop` 都是 GenUI 内置的宏。
  - `Prop`: 用于声明 Widget 的属性。
  - `Event`: 用于声明 Widget 的事件。
- Widget 的回调函数使用 Rust 闭包语法：`let mut event_callback = ||{...}`。
- `active!`: 内置宏，用于触发 Widget 的事件。

```rust
<template>
    <component inherits="view">
        <label id="first_lb" class="t_label" font_size="32" :text="props.label1"/>
        <label id="second_lb" class="t_label" :font_size="fs" text="label 2"/>
        <button id="bb" text="text btn" @clicked="btn_click" />
    </component>
</template>

<script>
use gen_macros::{Event, Prop};

#[derive(Event, Clone, Debug)]
pub enum Events {
    Clicked(String),
}

#[derive(Prop)]
pub struct ButtonView {
    pub label1: String,
}

impl Default for ButtonView {
    fn default() -> Self {
        Self {
            label1: "Click The Button".to_string(),
        }
    }
}

let mut props = ButtonView::default();
props.label1 = String::from("sss");
let fs: f64 = 18.0;

let mut btn_click = || {
    props.label1 = String::from("I have been clicked");
    println!("Button bb Clicked");
    active!(Events::Clicked("Hello".to_string()));
};
</script>

<style>
.t_label {
    brightness: 1.1;
    color: #fff;
    wrap: Word;
    font: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf");
}
</style>
```