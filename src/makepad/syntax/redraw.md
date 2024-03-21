# Redraw

- `apply_over_and_redraw`: redraw immediately
- `apply_over` then `draw_all`: redraw together

the most commonly used should be `apply_over` and then use `draw all` to render

sometimes you dont want it to redraw because you do apply over in a redraw function

if you use `apply_over_and_redraw` in the redraw flow it will redraw continuously

## redraw immediately

立即重新渲染指的是当代码进行执行后会即刻对目标widget进行重新渲染操作

## redraw together

