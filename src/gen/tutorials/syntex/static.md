# Static Page

静态页面指的是不带有脚本的纯静态组件组成的页面，通常仅由`<template>`和`<style>`两个部分组成，但这不代表`<script>`标签不能使用，而是不在`<script>`中编写带有逻辑的脚本语句，在`<script>`标签中使用内置`import!`宏引入组件和`use`来引入库都不被视为违反静态原则。

## Example1

```rust
<template>
  <root id="ui">
    <window id="main_window">
      <view flow="Down" height="All" id="main_view">
      </view>
    </window>
  </root>
</template>

<style>
#ui{
  #main_window{
    width: Fill;
    height: Fill;
    background_visible: true;
    background_color: #1C2128;
    flow: Down;
    window_size: 600.0 800.0;
    window_position: 300.0;
    #main_view{
      background_color: #FFF,
    }
  }
}
</style>
```

## Example2 ⛔
```rust
<template>
  <view id="checkbox_view">
    <checkbox class="checkbox1" check_type="Radio"></checkbox>
    <checkbox class="checkbox1"></checkbox>
    <checkbox id="checkbox2"></checkbox>
    <radio_button id="radio1"></radio_button>
    <radio_button id="radio2"></radio_button>
    <checkbox class="checkbox1" check_type="None"></checkbox>
    
    <button text="click" @click="change"></button>
    <label :text="label_text" margin="16.0"></label>
  </view>
</template>

// 这是错误的，因为script只能在非static模板中使用(除了内置`import!`)
// 非static模板: `<template><component inherits="view"></component></template>`
<script>
let mut label_text = String::from("this is a test label!");

let change = ||{
  label_text = String::from("I have been clicked");
};
</script>

<style>
#checkbox_view{
  width: 300;
  height: 300;
  flow: Down;
  .checkbox1{
    text: "CheckBox1";
    margin: 10;
    label_margin: 0 0 0 10;
  }
  #checkbox2{
    text: "Checkbox Toggle";
    label_margin: 0 0 0 16.0;
    font_brightness: 1.5;
    check_type: Toggle;
  }
  #radio1{
    text: "Radio1";
    margin: 16.0;
    font_size: 16.0;
  }
  #radio2{
    radio_type: Tab;
    text: "Radio Tab";
    margin: 0 16.0;
    padding: 12.0;
    height: 32.0;
    label_align: 0.5;
    label_margin: 0 0 0 0;

  }
}
</style>
```