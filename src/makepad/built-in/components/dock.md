# Dock

## Example

```rust
<Dock> {
    height: 500., width: Fill

    tab_bar: {
        StandardTab = <Tab> {
            spacing: (THEME_SPACE_2)
            closeable: false
        }
        PermaTab = <Tab> {
            spacing: (THEME_SPACE_2)
            closeable: true
        }
     }

    root = Splitter {
        axis: Horizontal,
        align: FromA(300.0),
        a: tab_set_1,
        b: tab_set_2
    }

    tab_set_1 = Tabs {
        tabs: [tab_a, tab_b],
        selected: 0
    }

    tab_set_2 = Tabs {
        tabs: [tab_c, tab_d, tab_e, tab_f],
        selected: 0
    }

    tab_a = Tab {
        name: "Tab A"
        template: PermaTab,
        kind: Container_A
    }

    tab_b = Tab {
        name: "Tab B"
        template: PermaTab,
        kind: Container_B
    }

    tab_c = Tab {
        name: "Tab C"
        template: StandardTab,
        kind: Container_C
    }

    tab_d = Tab {
        name: "Tab D"
        template: StandardTab,
        kind: Container_D
    }

    tab_e = Tab {
        name: "Tab E"
        template: StandardTab,
        kind: Container_E
    }

    tab_f = Tab {
        name: "Tab F"
        template: StandardTab,
        kind: Container_F
    }

    Container_A = <RectView> {
        draw_bg: { color: (THEME_COLOR_U_1) }
        height: Fill, width: Fill
        padding: 10.,
        <Label> {text: "Hallo"}
    }

    Container_B = <RectView> {
        draw_bg: { color: (THEME_COLOR_U_1) }
        height: Fill, width: Fill
        padding: 10.,
        <Label> {text: "Kuckuck"}
    }

    Container_C = <RectView> {
        draw_bg: { color: (THEME_COLOR_U_1) }
        height: Fill, width: Fill
        padding: 10.,
        <Label> {text: "Ahoy"}
    }

    Container_D = <RectView> {
        draw_bg: { color: (THEME_COLOR_U_1) }
        height: Fill, width: Fill
        padding: 10.,
        <Label> {text: "Hi"}
    }

    Container_E = <RectView> {
        draw_bg: { color: (THEME_COLOR_U_1) }
        height: Fill, width: Fill
        padding: 10.,
        <Label> {text: "Ahoy"}
    }

    Container_F = <RectView> {
        draw_bg: { color: (THEME_COLOR_U_1) }
        height: Fill, width: Fill
        padding: 10.,
        <Label> {text: "Hi"}
    }
}
```
