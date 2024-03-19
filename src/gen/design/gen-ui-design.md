# GenUI-Design

## Template
HTML-LIKE：
与HTML相似，但语法经过处理

所有的被处理的被认为是不必要的，可能使得模版不够专注！

1. 不允许直接书写字符串
2. 禁止使用模版语法，使用Rust `format!`进行值绑定
3. 标签上声明基本属性只能使用基本类型，复杂类型需要绑定
4. 函数体不允许直接书写在属性中，使用函数绑定
5. 属性类型具有强类型指向型

## Script
1. Allow Rust syntax

## Style

1. bind to tag by name
2. nesting allowed
3. function and bind allowed