# Font

文字字体相关属性

## Overview

|Prop|Description|Type|Default|
|--|--|--|--|
|`font_family`|字体类型|`Dep`||
|`font_size`|字体大小|`f32`|9.0|
|⚠️`font_weight`|字体粗细|`GFontWeight`|500|
|`font_scale`|字体缩放|`f32`|1.0|
|⚠️`brightness`|文字亮度|`f32`|1.0|
|⚠️`curve`|字体曲线|`f32`|0.0|
|`top_drop`|起始字符高度|`f32`|0.0|
|`height_factor`|高度因子|`f32`|1.0|

## Font Family

字体类型, 来源于字体包, 一般使用字体包的地址或字体包名作为指向, 格式为:
- ✅ ttf (当前makepad plugin支持)
- ⛔ woff
- ⛔ otf
- ⛔ ttc

### Example

#### in line

```
<label 
    text="hello world" 
    font_family="crate://self/resources/GoNotoKurrent-Bold.ttf">
</label>
```

#### in style

```
<template>
    <label id="label1" text="hello world"></label>
</template>

<style>
#label1{
    font_family: "crate://self/resources/GoNotoKurrent-Bold.ttf";
}
</style>
```

## Font Size

字体大小, 在Web端默认基础字体大小为16px, 但在GenUI中为9.0

### Example

```
<label text="hello world" font_size="32.0"></label>
```

## Font Scale

字体缩放倍数, 默认情况下缩放倍数为1.0, 即不缩放

### Example
```
<label text="hello world" font_scale="1.2"></label>
```

## Top Drop

起始字符高度

### Example

```
<label text="hello world" top_drop="10.0"></label>
```

## Height Factor

### Example

```
<label text="hello world" height_factor="1.5"></label>
```