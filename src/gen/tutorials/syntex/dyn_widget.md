# Dyn Widget

## 动态Widget类型

1. A: 没有使用`gen_macros::Prop`标注的且没有声明id
2. B: 没有使用`gen_macros::Prop`标注但声明id
3. C: 使用`gen_macros::Prop`不声明id

> Note!:
>   1. 同时使用`gen_macros::Prop`且声明id时，若`${prop_struct_name} == ${id}`，属于C类
>   2. 但若`${prop_struct_name} != ${id}`时，panic!

### A类

当没有使用`Prop`进行标注且没有在`<component></component>`中声明组件id时:

```rust
<template>
    <component inherits="view"></component>
</template>
```

Dyn Widget的结构体名为: `${source_name}_${inherits}`

### B类

当没有使用`Prop`进行标注但在`<component></component>`中声明组件id时:

```rust
<template>
    <component id="MyView" inherits="view"></component>
</template>
```

Dyn Widget的结构体名为: `${id}`, 在这个例子中就是`MyView`

### C类

使用`Prop`进行标注但不声明id

```rust
<template>
    <component inherits="view"></component>
</template>

<script>
use gen_macros::{Prop} // 可以省略

#[derive(Prop)]
pub struct MyView{}
</script>
```

Dyn Widget的结构体名为`${prop_struct_name}`, 在这个例子中就是`MyView`

---

## Script脚本原则

### LifeTime

- before: 最早被调用， 对PropStruct的初始化会在这里处理
- after: 在before之后调用，声明式修改PropStruct Value

#### Example

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
#### After Compiled
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

## Example

动态Widget采用`<component>`标签作为单根标签，使用`inherits`属性指定某个需要继承的widget，
当前推荐继承view widget

- `use gen_macros::{Event, Prop};`: 这行是可以忽略的，Event和Prop也是GenUI内置的宏
  - Prop: 用于声明widget的属性
  - Event: 用于声明widget的事件
- widget的回调使用Rust闭包语法`let mut event_callback = ||{...}`
- active!: 这也是一个内置宏用于调用widget事件
```rust
<template>
    <component inherits="view">
        <label id="first_lb" class="t_label" font_size="32" :text="props.label1"/>
        <label id="second_lb" class="t_label" :font_size="fs"  text="label 2"/>
        <button id="bb" text="text btn" @clicked="btn_click" />
    </component>
</template>

<script>
use gen_macros::{Event, Prop};

#[derive(Event,Clone,Debug)]
pub enum Events{
    Clicked(String),
}

#[derive(Prop)]
pub struct ButtonView{
    pub label1: String,
}

impl Default for ButtonView{
    fn default() -> Self {
        Self{
            label1: "Click The Button".to_string()
        }
    }
}

let mut props = ButtonView::default();
props.label1 = String::from("sss");
let fs: f64 = 18.0;

let mut btn_click = ||{
    props.label1 = String::from("I have been clicked");
    println!("Button bb Clicked");
    active!(Events::Clicked("Hello".to_string()));
};
</script>

<style>
.t_label{
    brightness: 1.1;
    color: #fff;
    wrap: Word;
    font: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf");
}
</style>
```