# LiveDependency

`LiveDependency`是一种引入资源的一种方式, 你可以用它来导入许多静态文件，使用关键字: `dep()`

`LiveDependency`is a kind of way to import resource, you can use it to import many static files, use key word: `dep()`

See [Import](../../syntax/import.md)

## Example

```rust
source: dep("crate://self/path/name.ty"),

font: {
    path: dep("crate://makepad-widgets/resources/GoNotoKurrent-Regular.ttf"),
},
```