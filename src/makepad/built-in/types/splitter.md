# SplitterAxis

定义拆分器沿其水平或垂直方向划分容器的轴。

Defines the axis along which the splitter divides its container, either horizontally or vertically.

## Properties
|decorate|name|type|description|
|--|--|--|--|
|live|Horizontal||Splits the container horizontally.|
|live|Vertical||Splits the container vertically.|


## Example

```rust
axis: Vertical,
axis: Horizontal,
```

---

# SplitterAlign

指定拆分器在其容器内的对齐或定位。

Specifies the alignment or positioning of the splitter within its container.

## Properties
|decorate|name|type|description|
|--|--|--|--|
|live|FromA|`f64`|Specifies the position of the splitter from the start of the first panel (A).|
|live|FromB|`f64`|Specifies the position of the splitter from the end of the second panel (B).|
|live|Weighted|`f64`|Specifies the position of the splitter based on a weight, with 0.0 being all the way to the start of panel A and 1.0 being all the way to the end of panel B.|

<strong style="color: #FF0000">
<br>
    Weighted range: [0,1]
</strong>

## Example

```rust
align: FromA(100.0),
align: Weighted(0.5),
```