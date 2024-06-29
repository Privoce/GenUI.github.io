# Dyn Widget

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
            label1: "Click The Button"
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