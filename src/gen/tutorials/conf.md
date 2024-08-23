# GenUI Config Toml

## Priority

Compiler > Env > Conf

> Although Conf Toml has lowest level, it is the most recommended!

## Toml Description

```toml
[compiler]
target = "makepad"
log_level = "info"
logo = true

[makepad]
entry = "app"
root = "E:/Rust/try/makepad/Gen-UI/examples/gen_makepad_simple/ui/views/root.gen"
[makepad.dependencies] 
makepad-widgets = { path = "E:/Rust/try/makepad/makepad/rik/makepad/widgets" }
[makepad.wasm]
check = false
fresh = true
port = 8016
```

### `[compiler]` Args

|Name|Type|Options|Default Value|
|--|--|--|--|
|target|String|1. "makepad"|"makepad"|
|log_level|String|1. `error`<br />2. `warn`<br />3. `info`<br />4. `debug`<br />5. `trace`|"info"|
|logo|bool|1. true<br />2. false|true|

- `target`: Set GenUI Compiler target
- `logo`: Set Logo is print or not
- `logo_level`: Set GenUI Log Level

### `[makepad]` Args

|Name|Type|Options|Default Value|
|--|--|--|--|
|entry|String|-|"app"|
|root|PathBuf|-|-|

- `entry`: makepad entry app rs file
- `root`: your ui dir root gen file

#### `[makepad.dependencies]`

required now!

format: `makepad-widgets = { path = "the/latest/makepad-widget/package/path" }`

recommand: `makepad-widgets = { git = "https://github.com/makepad/makepad", branch = "rik" }`

#### `[makepad.wasm]`

not required

|Name|Type|Options|Default Value|
|--|--|--|--|
|check|bool|1. true<br />2. false|false|
|fresh|bool|1. true<br />2. false|false|
|port|u16|0~65535|8010|