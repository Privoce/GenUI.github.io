# Import

在`<script>`标签中，带有内置的`import!`宏可以导入我们需要引入的组件

```rust
<template>
  <root id="ui">
    <window id="main_window">
      <view flow="Down" height="All" id="main_view">
        <checkbox_view /> 
        <header_view /> 
        <image_view></image_view>
        <icon_view></icon_view>
        <button_view></button_view>
      </view>
    </window>
  </root>
</template>

<script>
import!{
  crate::views::checkbox::*;
  crate::views::header::header::*;
  crate::views::icon::*;
  crate::views::button_view::*;
  crate::views::image_view::*;
}
</script>

<style>
#ui{
  #main_window{
    width: Fill;
    height: Fill;
    show_bg: true;
    draw_bg: #1C2128;
    flow: Down;
    inner_size: 600.0, 800.0;
    position: 300.0;
    #main_view{
      draw_bg: #FFF,
    }
  }
}
</style>
```