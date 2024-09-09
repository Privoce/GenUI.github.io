# Enum

> ❗Note:
>
> Makepad属性Enum常有以下两种用途
> 1. Event
> 2. Types 

## Types

`Types`常用于live系统切换选项用途，必须遵守以下原则

### 原则1: 宏标记
当作为`Types`进行使用时该Enum必须被标注:
1. `#[derive(Live)]`
2. `#[derive(LiveHook)]`
3. `#[live_ignore]`

可选择添加`#[repr(u32)]`
### 原则2: 无嵌套

即不可使用嵌套方式嵌套多个枚举或结构
### 原则3: shader_enum

首个枚举项必须使用`shader_enum(1)`注明以确保顺序

### Example

```rust
use makepad_widgets::*;

#[derive(Live, LiveHook, Clone, Debug, Copy)]
#[live_ignore]
#[repr(u32)]
pub enum Base {
    #[pick]
    Min = shader_enum(1),
    Max,
    FullScreen,
    FullScreenExpand,
}
```

## Event