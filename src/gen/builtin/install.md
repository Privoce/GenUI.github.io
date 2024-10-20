# Install

## Crates.io (Comming Soon)

## Github

[GenUI Builtin Components](https://github.com/Privoce/GenUI/tree/components/gen/components)

### 1. Add the following to your Cargo.toml file

```toml
makepad-widgets = {git = "https://github.com/makepad/makepad.git", branch = "rik"}
gen_components = {git = "https://github.com/Privoce/GenUI.git/gen/components", branch = "components"} 
```

### 2. Import into Live Design

```rust
live_design!{
    import gen_components::components::*;
}
```

### 3. Register into LiveRegister

```rust
crate::gen_components::live_design(cx);
```