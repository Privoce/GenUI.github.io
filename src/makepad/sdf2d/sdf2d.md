# Sdf2d

`Sdf2d` is a std lib in Makepad use `GLSL`([glsl|glslang](../glsl/glsl.md))

## 

```rust
Sdf2d = struct {
    // Fields of the Sdf2d struct
    field pos: vec2        // Current position
    field result: vec4     // Result color
    field last_pos: vec2   // Last position in path
    field start_pos: vec2  // Start position in path
    field shape: float     // Shape distance
    field clip: float      // Clipping distance
    field has_clip: float  // Clip flag
    field old_shape: float // Previous shape distance
    field blur: float      // Blur amount
    field aa: float        // Anti-aliasing factor
    field scale_factor: float // Scaling factor
    field dist: float      // Distance to shape

    // Calculate anti-aliasing factor based on the derivative of the position
    fn antialias(p: vec2) -> float {
        return 1.0 / length(vec2(length(dFdx(p)), length(dFdy(p))));
    }

    // Initialize a new Sdf2d struct with the given position
    fn viewport(pos: vec2) -> Self {
        return Self {
            pos: pos
            result: vec4(0.)
            last_pos: vec2(0.)
            start_pos: vec2(0.)
            shape: 1e+20
            clip: -1e+20
            has_clip: 0.0
            old_shape: 1e+20
            blur: 0.00001
            aa: antialias(pos)
            scale_factor: 1.0
            dist: 0.0
        };
    }

    // Translate the current position by (x, y)
    fn translate(inout self, x: float, y: float) -> vec2 {
        self.pos -= vec2(x, y);
        return self.pos;
    }

    // Rotate the current position around (x, y) by angle a
    fn rotate(inout self, a: float, x: float, y: float) {
        let ca = cos(-a);
        let sa = sin(-a);
        let p = self.pos - vec2(x, y);
        self.pos = vec2(p.x * ca - p.y * sa, p.x * sa + p.y * ca) + vec2(x, y);
    }

    // Scale the current position relative to (x, y) by factor f
    fn scale(inout self, f: float, x: float, y: float) {
        self.scale_factor *= f;
        self.pos = (self.pos - vec2(x, y)) * f + vec2(x, y);
    }

    // Clear the result color with the given color
    fn clear(inout self, color: vec4) {
        self.result = vec4(color.rgb * color.a + self.result.rgb * (1.0 - color.a), color.a);
    }

    // Calculate blur based on width w
    fn calc_blur(inout self, w: float) -> float {
        let wa = clamp(-w * self.aa, 0.0, 1.0);
        let wb = 1.0;
        if self.blur > 0.001 {
            wb = clamp(-w / self.blur, 0.0, 1.0);
        }
        return wa * wb;
    }

    // Fill the current shape with premultiplied alpha, keeping the previous fill
    fn fill_keep_premul(inout self, source: vec4) -> vec4 {
        let f = self.calc_blur(self.shape);
        self.result = source * f + self.result * (1. - source.a * f);
        if self.has_clip > 0.5 {
            let f2 = 1.0 - self.calc_blur(-self.clip);
            self.result = source * f2 + self.result * (1. - source.a * f2);
        }
        return self.result;
    }

    // Fill the current shape with premultiplied alpha and reset the shape
    fn fill_premul(inout self, color: vec4) -> vec4 {
        self.fill_keep_premul(color);
        self.old_shape = self.shape = 1e+20;
        self.clip = -1e+20;
        self.has_clip = 0.;
        return self.result;
    }

    // Fill the current shape, keeping the previous fill
    fn fill_keep(inout self, color: vec4) -> vec4 {
        return self.fill_keep_premul(vec4(color.rgb * color.a, color.a));
    }

    // Fill the current shape and reset the shape
    fn fill(inout self, color: vec4) -> vec4 {
        return self.fill_premul(vec4(color.rgb * color.a, color.a));
    }

    // Stroke the current shape, keeping the previous stroke
    fn stroke_keep(inout self, color: vec4, width: float) -> vec4 {
        let f = self.calc_blur(abs(self.shape) - width / self.scale_factor);
        let source = vec4(color.rgb * color.a, color.a);
        let dest = self.result;
        self.result = source * f + dest * (1.0 - source.a * f);
        return self.result;
    }

    // Stroke the current shape and reset the shape
    fn stroke(inout self, color: vec4, width: float) -> vec4 {
        self.stroke_keep(color, width);
        self.old_shape = self.shape = 1e+20;
        self.clip = -1e+20;
        self.has_clip = 0.;
        return self.result;
    }

    // Glow the current shape, keeping the previous glow
    fn glow_keep(inout self, color: vec4, width: float) -> vec4 {
        let f = self.calc_blur(abs(self.shape) - width / self.scale_factor);
        let source = vec4(color.rgb * color.a, color.a);
        let dest = self.result;
        self.result = vec4(source.rgb * f, 0.) + dest;
        return self.result;
    }

    // Glow the current shape and reset the shape
    fn glow(inout self, color: vec4, width: float) -> vec4 {
        self.glow_keep(color, width);
        self.old_shape = self.shape = 1e+20;
        self.clip = -1e+20;
        self.has_clip = 0.;
        return self.result;
    }

    // Union the current shape with the previous shape
    fn union(inout self) {
        self.old_shape = self.shape = min(self.dist, self.old_shape);
    }

    // Intersect the current shape with the previous shape
    fn intersect(inout self) {
        self.old_shape = self.shape = max(self.dist, self.old_shape);
    }

    // Subtract the current shape from the previous shape
    fn subtract(inout self) {
        self.old_shape = self.shape = max(-self.dist, self.old_shape);
    }

    // Apply a gloop effect to the current shape with factor k
    fn gloop(inout self, k: float) {
        let h = clamp(0.5 + 0.5 * (self.old_shape - self.dist) / k, 0.0, 1.0);
        self.old_shape = self.shape = mix(self.old_shape, self.dist, h) - k * h * (1.0 - h);
    }

    // Blend the current shape with the previous shape using factor k
    fn blend(inout self, k: float) {
        self.old_shape = self.shape = mix(self.old_shape, self.dist, k);
    }

    // Draw a circle at (x, y) with radius r
    fn circle(inout self, x: float, y: float, r: float) {
        let c = self.pos - vec2(x, y);
        let len = sqrt(c.x * c.x + c.y * c.y);
        self.dist = (len - r) / self.scale_factor;
        self.old_shape = self.shape;
        self.shape = min(self.shape, self.dist);
    }

    // Draw an arc at (x, y) with radius r from angle s to e
    fn arc2(inout self, x: float, y: float, r: float, s: float, e: float) -> vec4 {
        let c = self.pos - vec2(x, y);
        let pi = 3.141592653589793; // PI constant
        
        // Calculate angle
        let ang = (atan(c.y, c.x) + pi) / (2.0 * pi);
        let ces = (e - s) * 0.5;
        let ang2 = 1.0 - abs(ang - ces) + ces;
        return mix(vec4(0., 0., 0., 1.0), vec4(1.0), ang2);
    }

    // Draw a horizontal line at y with half height h
    fn hline(inout self, y: float, h: float) {
        let c = self.pos.y - y;
        self.dist = -h + abs(c) / self.scale_factor;
        self.old_shape = self.shape;
        self.shape = min(self.shape, self.dist);
    }

    // Draw a rounded box at (x, y) with width w, height h, and radius r
    fn box(inout self, x: float, y: float, w: float, h: float, r: float) {
        let p = self.pos - vec2(x, y);
        let size = vec2(0.5 * w, 0.5 * h);
        let bp = max(abs(p - size.xy) - (size.xy - vec2(2. * r, 2. * r).xy), vec2(0., 0.));
        self.dist = (length(bp) - 2. * r) / self.scale_factor;
        self.old_shape = self.shape;
        self.shape = min(self.shape, self.dist);
    }

    // Draw a box with different radii for top and bottom corners
    fn box_y(inout self, x: float, y: float, w: float, h: float, r_top: float, r_bottom: float) {
        let size = vec2(0.5 * w, 0.5 * h);
        let p_r = self.pos - vec2(x, y);
        let p = abs(p_r - size.xy) - size.xy;
        
        let bp_top = max(p + vec2(2. * r_top, 2. * r_top).xy, vec2(0., 0.));
        let bp_bottom = max(p + vec2(2. * r_bottom, 2. * r_bottom).xy, vec2(0., 0.));
        
        self.dist = mix(
            (length(bp_top) - 2. * r_top),
            (length(bp_bottom) - 2. * r_bottom),
            step(0.5 * h, p_r.y)
        ) / self.scale_factor;
        
        self.old_shape = self.shape;
        self.shape = min(self.shape, self.dist);
    }

    // Draw a box with different radii for left and right corners
    fn box_x(inout self, x: float, y: float, w: float, h: float, r_left: float, r_right: float) {
        let size = vec2(0.5 * w, 0.5 * h);
        let p_r = self.pos - vec2(x, y);
        let p = abs(p_r - size.xy) - size.xy;
        
        let bp_left = max(p + vec2(2. * r_left, 2. * r_left).xy, vec2(0., 0.));
        let bp_right = max(p + vec2(2. * r_right, 2. * r_right).xy, vec2(0., 0.));
        
        self.dist = mix(
            (length(bp_left) - 2. * r_left),
            (length(bp_right) - 2. * r_right),
            step(0.5 * w, p_r.x)
        ) / self.scale_factor;
        
        self.old_shape = self.shape;
        self.shape = min(self.shape, self.dist);
    }

    // Draw a box with different radii for each corner
    fn box_all(
        inout self,
        x: float,
        y: float,
        w: float,
        h: float,
        r_left_top: float,
        r_right_top: float,
        r_right_bottom: float,
        r_left_bottom: float
    ) {
        let size = vec2(0.5 * w, 0.5 * h);
        let p_r = self.pos - vec2(x, y);
        let p = abs(p_r - size.xy) - size.xy;
        
        let bp_lt = max(p + vec2(2. * r_left_top, 2. * r_left_top).xy, vec2(0., 0.));
        let bp_rt = max(p + vec2(2. * r_right_top, 2. * r_right_top).xy, vec2(0., 0.));
        let bp_rb = max(p + vec2(2. * r_right_bottom, 2. * r_right_bottom).xy, vec2(0., 0.));
        let bp_lb = max(p + vec2(2. * r_left_bottom, 2. * r_left_bottom).xy, vec2(0., 0.));
        
        self.dist = mix(
            mix(
                (length(bp_lt) - 2. * r_left_top),
                (length(bp_lb) - 2. * r_left_bottom),
                step(0.5 * h, p_r.y)
            ),
            mix(
                (length(bp_rt) - 2. * r_right_top),
                (length(bp_rb) - 2. * r_right_bottom),
                step(0.5 * h, p_r.y)
            ),
            step(0.5 * w, p_r.x)
        ) / self.scale_factor;
        
        self.old_shape = self.shape;
        self.shape = min(self.shape, self.dist);
    }

    // Draw a rectangle at (x, y) with width w and height h
    fn rect(inout self, x: float, y: float, w: float, h: float) {
        let s = vec2(w, h) * 0.5;
        let d = abs(vec2(x, y) - self.pos + s) - s;
        let dm = min(d, vec2(0., 0.));
        self.dist = max(dm.x, dm.y) + length(max(d, vec2(0., 0.)));
        self.old_shape = self.shape;
        self.shape = min(self.shape, self.dist);
    }

    // Draw a hexagon at (x, y) with radius r
    fn hexagon(inout self, x: float, y: float, r: float) {
        let dx = abs(x - self.pos.x) * 1.15;
        let dy = abs(y - self.pos.y);
        self.dist = max(dy + cos(60.0 * TORAD) * dx - r, dx - r);
        self.old_shape = self.shape;
        self.shape = min(self.shape, self.dist);
    }

    // Move to position (x, y) without drawing
    fn move_to(inout self, x: float, y: float) {
        self.last_pos =
        self.start_pos = vec2(x, y);
    }

    // Draw a line to position (x, y)
    fn line_to(inout self, x: float, y: float) {
        let p = vec2(x, y);
        
        let pa = self.pos - self.last_pos;
        let ba = p - self.last_pos;
        let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
        let s = sign(pa.x * ba.y - pa.y * ba.x);
        self.dist = length(pa - ba * h) / self.scale_factor;
        self.old_shape = self.shape;
        self.shape = min(self.shape, self.dist);
        self.clip = max(self.clip, self.dist * s);
        self.has_clip = 1.0;
        self.last_pos = p;
    }

    // Close the current path
    fn close_path(inout self) {
        self.line_to(self.start_pos.x, self.start_pos.y);
    }
}

```