# Convert RSX DSL to AST
## How to convert
```shell
                                                   Real AST
-----------       ---------------   Strategy   ---------------
| RSX DSL |  -->  | ParseTarget |  ----------> | ParseResult |
-----------       ---------------              ---------------  
```
## RSX

- Template
- Script
- Style

### Template

1. Unrestricted tags (tag name is not constrained)
2. There are no styles, only properties, or in other words, all styles are properties
3. Nesting strings in tags is not allowed (example: <view>this is a view</view> ❌)

### Script
1. Allow Rust syntax

### Style

1. bind to tag by name
2. nesting allowed
3. function and bind allowed

