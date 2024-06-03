# shape

## Border-radius

1. set radius, 2.5 is radius size: `instance radius: 2.5`
2. use sdf.box() and return max() to get radius 

```rust
draw_bg: {
    instance border_width: 0.0
    instance border_color: #0000
    instance inset: vec4(0.0, 0.0, 0.0, 0.0)
    instance radius: 2.5
    
    fn get_color(self) -> vec4 {
        return self.color
    }
    
    fn get_border_color(self) -> vec4 {
        return self.border_color
    }
    
    fn pixel(self) -> vec4 {
        let sdf = Sdf2d::viewport(self.pos * self.rect_size)
        sdf.box(
            self.inset.x + self.border_width,
            self.inset.y + self.border_width,
            self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
            self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0),
            max(1.0, self.radius)
        )
        sdf.fill_keep(self.get_color())
        if self.border_width > 0.0 {
            sdf.stroke(self.get_border_color(), self.border_width)
        }
        return sdf.result;
    }
}
```