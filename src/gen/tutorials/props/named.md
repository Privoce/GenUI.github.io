# Named

> 说明:
> 
> 命名规则是属性命名的基础, 用于对属性命名进行规范化处理, 规则如下:
> 1. 若无特殊情况属性命名必须依照基础命名原则
> 2. 当出现基础命名不足够时采用基础组件命名法

## 基础组件命名法

基础组件命名法只有在出现基础命名不足够时进行使用, 它的原则很简单

即: `${base_ele}_${base_prop_name}`

例如:

当某个组件中已经有代表hover状态时的`hover_color`时，还需要对线条处于hover状态时定义颜色属性

那么这个新属性为: `stroke_hover_color`

## base ele

base ele指的是最基本的构成部分，他是所有其他组件的基础

- stroke: 线条
- text: 文本
- 

## Color

### 基础命名说明 

|命名|说明|
|--|--|
|background_color|背景色, 在任意具有区域化的组件中都应该含有|
|color|文字颜色, 仅在带有文字的组件中含有|
|stroke_color|线条颜色, 在含有线条的组件中使用, 例如图标|
|hover_color|当hover时的颜色|
|pressed_color|当pressed时的颜色|
|border_color|边框颜色|
|shadow_color|阴影颜色|