# Project Structure

## Project Tree
```bash
src_gen
│  Cargo.toml
│  
├─src
│  │  app.rs
│  │  lib.rs
│  │  main.rs
│  │  
│  ├─src
│  │      app.rs
│  │      
│  └─views
│          mod.rs
│          root.rs
│
├─static
│      231.png
```

### lib.rs

```rust
pub use makepad_widgets;
pub use makepad_widgets::makepad_draw;
pub mod app;
pub mod views;
```

### views/mod.rs

```rust
pub mod root;
```

### app.rs
```rust
use makepad_widgets ::* ; 
live_design ! { 
    import makepad_widgets::base::*; 
    import makepad_widgets::theme_desktop_dark::*; 
    // ⚠️ look at here !!!
    // although the root mod is pub to super
    // you still need to write views::root
    // because here is path not rs mod structure
    import crate::views::root::*;
    

    App = {{ App }}{ 
       ui : <Root>{}
    } 
} 

// ....

impl LiveRegister for App { 
    fn live_register (cx : & mut Cx) { 
        crate :: makepad_widgets ::live_design (cx) ;
        // ⚠️ look at here !!! same as import
        crate::views::root::live_design(cx);
    } 
} 

app_main ! (App) ;
```