# GenUI Logger

## Logo

You can control whether the logo is printed using the system environment variable `GENUI_LOGO` or through the configuration file in TOML format.

- For more details, see [GenUI Environment Setup](https://palpus-rs.github.io/Gen-UI.github.io/gen/tutorials/env.html).
- For configuration, see [GenUI Config TOML](https://palpus-rs.github.io/Gen-UI.github.io/gen/tutorials/conf.html).

Example:

```rust
GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>> 

     _/_/_/  _/_/_/_/  _/      _/  _/    _/  _/_/_/
  _/        _/        _/_/    _/  _/    _/    _/
 _/  _/_/  _/_/_/    _/  _/  _/  _/    _/    _/
_/    _/  _/        _/    _/_/  _/    _/    _/
 _/_/_/  _/_/_/_/  _/      _/    _/_/    _/_/_/

```

## Services

The GenUI Logger provides detailed information about the state of various services. Here are some log examples:

```rust
GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>> 🔧 Log Service is starting... Log entries will be available after the `app event::Change` occurs!
GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>> 🔧 Source Generator Service started successfully!
GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>> ✅ Cache Service: Cache file written successfully!
GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>> 🔧 App is running...
GenUI-Compiler :: [2024-06-29T08:53:57Z] :: INFO >>> 🔧 Watcher Service started successfully!
```

## Compile Timing

The logger also tracks and displays compile timings, helping you monitor the compilation process:

```rust
GenUI-Compiler :: [2024-06-28T19:09:24Z] :: INFO >>> File "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\gen_makepad_simple\\ui\\views\\root.gen" compiled successfully.
GenUI-Compiler :: [2024-06-28T19:09:24Z] :: INFO >>> ✅ Cache Service: Cache file written successfully!
GenUI-Compiler :: [2024-06-28T19:09:24Z] :: INFO >>> File "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\gen_makepad_simple\\ui\\views\\root.gen" compiled successfully.
```