以下是对你的文档的补全与优化，保持了你原有的风格，并增强了清晰度、术语一致性和表达流畅性：

---

# Router 配置

从 GenUI v0.1.2 开始，框架引入了路由配置功能。但需要注意的是，GenUI 的路由机制与传统 Web 前端中的“前端路由”是不同的概念。在 GUI 应用中并没有真正的 URL 路由，GenUI 的路由本质上是一种页面切换机制，用于在多个 UI 界面间进行状态管理和导航。

GenUI 的路由机制基于一个专用组件，并通过项目配置文件进行启用。你可以通过在 `router.toml` 中声明路由结构，并在 `gen_ui.toml` 中启用配置，再通过 `route!` 宏将其引入具体页面。

示例详见：
- [路由组件示例](https://github.com/Privoce/made_with_GenUI/tree/main/tests/router)
- [根路由示例](https://github.com/Privoce/made_with_GenUI/tree/main/tests/router_tabbar)

## 配置与使用流程

### Step 1: 配置 `router.toml`

你可以通过 `router.toml` 定义一个完整的路由配置，包括路由模式、默认激活页面、是否启用 Tabbar、页面路径等：

```toml title="router.toml"
name = "MyRouter"
id = "app_router"
mode = "History"         # 可选值: History / Switch
active = "login"         # 默认页面 ID

[tabbar]
theme = "Dark"
active = false

[tabbar.bars]
login = { icon = "crate://self/resources/login.svg", text = "Login Page" }
register = { icon = "crate://self/resources/register.svg", text = "Register Page" }

[bar_pages]
login = { path = "crate::views::login::*", component = "Login" }
register = "crate::views::register::Register"

[nav_pages]
nav_about = { path = "crate::views::about::*", component = "About" }
```

### Step 2: 在 `gen_ui.toml` 中启用路由配置

```toml title="gen_ui.toml"
[compiler]
#...

[makepad]
router = "./router.toml"

[plugins]
#...
```

### Step 3: 在组件中引入路由

在需要使用路由的 `.gen` 文件中，使用 `route!` 宏引入已声明的路由组件：

```rust
<script>
route!(app_router);
</script>
```

> [!WARNING]
> 如果使用了 `route!` 宏，则该 `.gen` 文件不能包含其他元素，必须仅用于路由承载。

### Step 4: 在其他组件中使用路由组件

你可以像使用普通组件一样，将路由组件嵌入到其他页面中：

```html
<template>
    <component name="Home">
        <view>
            <button @clicked="to_page('login')"></button>
        </view>
        <MyRouter id="my_router"></MyRouter>
    </component>
</template>
```

### Step 5: 使用路由作为根组件

如果希望以路由组件作为应用根组件，请在 `root.gen` 文件中使用 `route!` 宏。

> 此时 `router.toml` 中的 `name` 必须设置为 `"UiRoot"`，以保证编译器识别为主入口。

---

## 页面跳转操作

### 在父组件中控制跳转

你可以通过 `c_ref!` 宏获取路由实例，并调用其提供的方法：

```rust
fn to_page(&mut self, page: &str) {
    let mut my_router = c_ref!(my_router);

    match page {
        "login" => my_router.nav_to(id!(login)),
        "register" => my_router.nav_to(id!(register)),
        "about" => my_router.nav_to(id!(nav_about)),
        "back" => my_router.nav_back(),
        _ => {}
    }
}
```

### 在子组件中跳转

对于子组件，GenUI 提供了便捷的宏 `nav_to!` 和 `nav_back!` 用于处理跳转：

```rust
pub fn to_register(&mut self) {
    nav_to!(register);
}

pub fn back(&mut self) {
    nav_back!();
}
```

---

## 路由配置结构说明

### Rust 中的数据结构定义：

```rust
pub struct RouterBuilder {
    pub name: String,
    pub id: String,
    pub mode: NavMode,
    pub active: Option<String>,
    pub tabbar: Option<TabbarBuilder>,
    pub bar_pages: Vec<(String, Page)>,
    pub nav_pages: HashMap<String, Page>,
}

pub struct TabbarBuilder {
    pub theme: Option<Themes>,
    pub active: bool,
    pub bars: HashMap<String, TabbarItem>,
}

pub struct TabbarItem {
    pub icon: Option<LiveDependency>,
    pub text: Option<String>,
}

pub enum Page {
    Path(Import),
    Component { path: Import, component: String },
}

pub enum NavMode {
    History,
    Switch,
}
```

### 字段说明表：

| Key | 类型 | 描述 |
|-----|------|------|
| `name` | `String` | 路由组件名称 |
| `id` | `String` | 路由组件唯一标识 |
| `mode` | `NavMode` | 路由模式，支持 `History`（历史栈）与 `Switch`（切换模式） |
| `active` | `Option<String>` | 默认激活的页面 ID |
| `tabbar` | `Option<TabbarBuilder>` | Tabbar 配置项（可选） |
| `bar_pages` | `Vec<(String, Page)>` | 主要页面配置，支持组件指定 |
| `nav_pages` | `HashMap<String, Page>` | 次要页面配置，不显示在 Tabbar 中 |
| `TabbarItem.icon` | `Option<LiveDependency>` | 图标资源路径 |
| `TabbarItem.text` | `Option<String>` | Tabbar 显示文本 |

---

如需进一步结构化文档（比如拆分子章节、整理多个路由配置策略对比等），也可以继续提出，我可以帮你统一风格和结构。你也可以指定这份文档最终用于生成 Markdown / HTML / Book 模式，我可以按目标格式优化结构。