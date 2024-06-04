# ImageFit Enum

用于确定图像在显示时的尺寸和比例

Used to determine the size and scale of the image when displayed

## Properties
|name|type|description|
|--|--|--|
|Stretch| |按照原始大小进行拉伸以填充整个容器<br>Stretch to the original size to fill the entire container |
|Horizontal| |图像将水平填充容器<br>The image will horizontally fill the container|
|Vertical| |图像将垂直填充容器<br>The image will vertically fill the container|
|Smallest| |图像将被缩放到最小尺寸以满足容器的大小要求<br>The image will be scaled to the minimum size to meet the size requirements of the container|
|Biggest||图像将被缩放到最大尺寸以满足容器的大小要求<br>The image will be scaled to the maximum size to meet the size requirements of the container|
|Size||图像将被缩放到指定的宽度和高度值<br>The image will be scaled to the specified width and height values|
## Example

```rust
fit: Stretch,
fit: Vertical,
```