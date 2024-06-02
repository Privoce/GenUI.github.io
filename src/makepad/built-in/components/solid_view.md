# SolidView

SolidView是一个简单的View组件，它继承父组件的背景色，除此外并没有什么特别的

SolidView is a simple View component that inherits the background color of the parent component, without any special features
```rust
SolidView = <ViewBase> {
    show_bg: true, 
    draw_bg: {
        fn get_color(self) -> vec4 {
            return self.color
        }
        
        fn pixel(self) -> vec4 {
            return Pal::premul(self.get_color())
        }
    }
}
```

## Example

在例子中SolidView和View一样，都可以去设置需要的属性，我们也可以去覆盖`draw_bg`属性

In the example, SolidView and View can both set the required properties, and we can also override the `draw_bg` property

```rust
<SolidView>{
    height: 100,
    width: 100,
    draw_bg: {color: #FF0000},
}
```