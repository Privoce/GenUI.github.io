# QuickStart

这个示例将向你展示如何使用GenUI框架结合Makepad作为转换底层

## Step1: Create a new WorkSpace Rust Project

### Step1.1 创建项目

使用`cargo new {{project_name}}`创建一个简单Rust项目

```shell
cargo new gen_makepad_simple
```

### Step1.2 清理项目并改为WorkSpace

接下来删除项目中的`src`目录并修改`Cargo.toml`文件

```toml
[workspace]
members = ["ui"] # ui表示GenUI的代码编写目录
```
### Step1.3 创建GenUI项目目录

```shell
cd ./gen_makepad_simple
cargo new ui
```

### Step1.3 在ui目录中创建UI根文件

在ui目录中创建`views`目录，在`views`中创建`root.gen`和`mod.gen`文件

```bash
--- ui
|------ views
|-------------- mod.gen
|-------------- root.gen
|------ src
|-------------- main.rs
```

![](../../static/gen/examples/quickstart/struct.png)

## Step2: 创建root.gen

- template: widget结构部分(我称这三个widget为root铁三角)
  - root: makepad UI root根
    - window: 主窗口
      - view: 主视图
- style: 嵌套编写widget的props(你也可以称之为styles)

```html
<template>
  <root id="ui">
    <window id="main_window">
      <view flow="Down" height="All" align="0.5 0.5">
        <label text="Gen + Makepad Project Hello World!!!" font_size="16.0"></label>
      </view>
    </window>
  </root>
</template>

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
  }
}
</style>
```
## Step3: 编写mod.rs导出root

```rust
<script>
pub mod root;
</script>
```

## Step4: main.rs

是GenUI的编译入口，提供编译指向的能力

- 在这个实例中，我们通过`app()`函数创建一个App编译器实例，指定编译底层目标为Makepad平台
- 创建`makepad-widgets`依赖并指定使用本地Git库作为依赖地址
- 设置目标入口文件的文件名字为`app`，这会让最终Makepad的入口为`app.rs`
- 指定`E:/Rust/try/makepad/Gen-UI/examples/gen_makepad_simple/ui/views/root.gen`这个文件作为UI的根文件(通过这种方式，来切换多个UI根)
- `add_dep()`方法将依赖添加到编译器中
- 调用`build()`方法进行编译
- 调用`run()`方法启动编译器

```rust
use gen_compiler::{app, Target, Builder};

fn main() {
    let compiler = Target::makepad()
        .entry("app")
        .root("E:/Rust/try/makepad/Gen-UI/examples/gen_makepad_simple/ui/views/root.gen")
        .add_dep("makepad-widgets")
        .local("E:/Rust/try/makepad/makepad/rik/makepad/widgets")
        .build()
        .build();

    // set app and specify target
    let mut app = app(Some(Box::new(compiler))).build();

    let _ = app.run();
}
```

