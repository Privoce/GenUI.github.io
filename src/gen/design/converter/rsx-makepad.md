# Makepad Generate Documentation

## Utilizing Only Rust Std

GenUI empowers users to create Makepad code using only the Rust standard library, offering a streamlined and beginner-friendly approach.

This method necessitates indicating the use of the standard library within the `<script>` tag either by specifying `<script lang="std">` or by employing a tag without the `lang` prop.

```rust
<template>
    <window id="ui" background_visible="true">
        <view id="body" :spacing="view_space" :flow="view_flow">
            <button id="btn1" :text="btn_text" @clicked="change_text"></button>
            <label id="t_label" :text="label_text" :font_size="label_size" />
        </view>
    </window>
</template>

<script>
let view_space: f64 = 20.0;
let mut view_flow = String::from("Down");
let mut label_text = String::from("This is a Hello, World! The emoji failed to display.");
let label_size = 24.0;
let btn_text = String::from("Click Me");

let mut change_text = || {
    label_text = String::from("I have been clicked!");
};
</script>

<style>
#ui {
    width: Fill;
    height: Fill;
    background_color: #96CEF8;
    #body {
        align: 0.5;
        #t_label {
            brightness: 1.1;
            color: #fff;
            wrap: Word;
            font: "crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf";
        }
    }
}
</style>
```

### Crafting Makepad Code

GenUI also supports direct code creation using Makepad. However, this approach means that direct property and method bindings are not permitted.

To directly write Makepad code, you need to add the `lang` prop to the `<script>` tag and set it to makepad, `<script lang="makepad">`.

Thanks to GenUI's conversion mechanism, some nested properties in Makepad can still be written directly in the `<style>` section without the need for nesting.

```rust
<template>
    <window id="ui" background_visible="true">
        <view id="body">
            <button id="btn1" />
            <label id="t_label" />
        </view>
    </window>
</template>

<script lang="makepad">
#[derive(Live, LiveHook)]
pub struct MyApp {
    #[live]
    ui: WidgetRef,
    #[rust]
    instance: Instance,
}
#[derive(Debug, Clone, Default)]
struct Instance {
    pub view_flow: Flow,
    pub label_text: String,
}
impl Instance {
    pub fn new() -> Self {
        Self {
            view_flow: Flow::Down,
            label_text: String::from("this is a Hello, World!! emoji failed"),
        }
    }
    pub fn get_view_flow(&self) -> &Flow {
        &self.view_flow
    }
    pub fn set_view_flow(&mut self, view_flow: Flow) {
        self.view_flow = view_flow
    }
    pub fn get_label_text(&self) -> &String {
        &self.label_text
    }
    pub fn set_label_text(&mut self, label_text: String) {
        self.label_text = label_text
    }
}
impl MatchEvent for MyApp {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.instance = Instance::new();
        let label_t_label = self.ui.label(id!(t_label));
        label_t_label.apply_over_and_redraw(
            cx,
            live! { text: "this is a Hello, World!! emoji failed",  draw_text: {  } },
        );
        let view_body = self.ui.view(id!(body));
        view_body.apply_over_and_redraw(cx, live! { flow: Down,  });
        let view_body = self.ui.view(id!(body));
        view_body.apply_over_and_redraw(cx, live! { spacing: 20,  });
        let label_t_label = self.ui.label(id!(t_label));
        label_t_label.apply_over_and_redraw(
            cx,
            live! {  draw_text: { text_style: { font_size: 24,  } } },
        );
        let button_btn1 = self.ui.button(id!(btn1));
        button_btn1.apply_over_and_redraw(cx, live! { text: "Click Me",  });
    }
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(btn1)).clicked(&actions) {
            let mut change_text = || {
                self.instance.label_text = String::from("I have been clicked!");
            };
            change_text();
            let label_t_label = self.ui.label(id!(t_label));
            label_t_label
                .apply_over_and_redraw(cx, live! { text: (self.instance.get_label_text()), });
        }
    }
}
impl LiveRegister for MyApp {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}
impl AppMain for MyApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::Startup => self.handle_startup(cx),
            _ => (),
        };
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
app_main!(MyApp);
</script>

<style>
#ui {
    width: Fill;
    height: Fill;
    background_color: #96CEF8;
    #body {
        spacing: 20.0;
        align: 0.5;
        flow: Down;
        #btn1 {
            text: "Click Me";
        }
        #t_label {
            brightness: 1.1;
            color: #fff;
            wrap: Word;
            font: "crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf";
            font_size: 24.0;
            text: "This is a Hello, World! The emoji failed to display.";
        }
    }
}
</style>
```

### Makepad Code

```rust
use makepad_widgets::*;
live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    MyApp = {{MyApp}}{ 
        ui: <Window>{
            show_bg: true, 
            draw_bg: { color: #96CEF8 }, 
            height: Fill, 
            width: Fill,  
            body = <View>{
                align: {x: 0.5, y: 0.5},  
                btn1 = <Button>{} 
                t_label = <Label>{ 
                    draw_text: { 
                        wrap: Word, 
                        color: #ffffff, 
                        text_style: { brightness: 1.1, font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")},  } 
                    }
                } 
            } 
        } 
    }
}
#[derive(Live, LiveHook)]
pub struct MyApp {
    #[live]
    ui: WidgetRef,
    #[rust]
    instance: Instance,
}
#[derive(Debug, Clone, Default)]
struct Instance {
    pub view_flow: Flow,
    pub label_text: String,
}
impl Instance {
    pub fn new() -> Self {
        Self {
            view_flow: Flow::Down,
            label_text: String::from("this is a Hello, World!! emoji failed"),
        }
    }
    pub fn get_view_flow(&self) -> &Flow {
        &self.view_flow
    }
    pub fn set_view_flow(&mut self, view_flow: Flow) {
        self.view_flow = view_flow
    }
    pub fn get_label_text(&self) -> &String {
        &self.label_text
    }
    pub fn set_label_text(&mut self, label_text: String) {
        self.label_text = label_text
    }
}
impl MatchEvent for MyApp {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.instance = Instance::new();
        let label_t_label = self.ui.label(id!(t_label));
        label_t_label.apply_over_and_redraw(
            cx,
            live! { text: "this is a Hello, World!! emoji failed",  draw_text: {  } },
        );
        let view_body = self.ui.view(id!(body));
        view_body.apply_over_and_redraw(cx, live! { flow: Down,  });
        let view_body = self.ui.view(id!(body));
        view_body.apply_over_and_redraw(cx, live! { spacing: 20,  });
        let label_t_label = self.ui.label(id!(t_label));
        label_t_label.apply_over_and_redraw(
            cx,
            live! {  draw_text: { text_style: { font_size: 24,  } } },
        );
        let button_btn1 = self.ui.button(id!(btn1));
        button_btn1.apply_over_and_redraw(cx, live! { text: "Click Me",  });
    }
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(btn1)).clicked(&actions) {
            let mut change_text = || {
                self.instance.label_text = String::from("I have been clicked!");
            };
            change_text();
            let label_t_label = self.ui.label(id!(t_label));
            label_t_label
                .apply_over_and_redraw(cx, live! { text: (self.instance.get_label_text()), });
        }
    }
}
impl LiveRegister for MyApp {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}
impl AppMain for MyApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::Startup => self.handle_startup(cx),
            _ => (),
        };
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
app_main!(MyApp);
```