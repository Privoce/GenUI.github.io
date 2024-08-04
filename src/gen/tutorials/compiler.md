# GenUI Compiler
GenUI Compiler is used to compile the current UI project into the underlying target project code

### Attention
you should write from project root path as relative path
### Example No `gen.toml`
we can create a compiler without `gen.toml` file, but we need to specify the target and other configurations

compiler use `builder` pattern, so you can chain the method to build the compiler, 
and finally call `build` method to get the compiler.
```rust
use gen_compiler::{app, Target, Builder};

fn main() {
    let compiler = Target::makepad()
        .entry("app")
        .root("E:/Rust/try/makepad/Gen-UI/examples/gen_makepad_simple/ui/views/root.gen")
        .add_dep("makepad-widgets")
        .local("E:/Rust/try/makepad/makepad/rik/makepad/widgets")
        .build()
        .wasm() // do not use if you don't need wasm
        .build()
        .build();

    // set app and specify target
    let mut app = app(Some(Box::new(compiler))).build();

    let _ = app.run();
}

```
---
### Example With `gen.toml`
if you have a `gen.toml` file, you can create a compiler without specifying the target and other configurations
the `gen.toml` file should be in the project root path, such as:
```text
hello
├── src_gen
├────── // ....
├── ui
├────── src
├────── gen.toml
```
#### gen.toml
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
```
#### main.rs
gen compiler will read the `gen.toml` file and create, so you do not need to pass the compiler

If you pass the compiler, the compiler will be used instead of the `gen.toml` file
```rust
use gen_compiler::{app, Builder};

fn main() {
    let mut app = app(None).build();
    let _ = app.run();
}
```