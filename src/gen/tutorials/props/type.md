# Type

类型说明文档, 在GenUI中声明值的类型仅限于文档中所示

## 基础类型

1. `usize`: 无符号整型
2. `isize`: 有符号整型
3. `f32`: 单精度浮点数
4. `f64`: 双精度浮点数
5. `bool`: 布尔值
6. `String`: 字符串
7. ⚠️ ~`Void`~ (废弃)

## 特殊类型

1. `Dep`: 静态资源引用
2. `Vec`: 动态数组
3. `Bind`: 值绑定类型
4. `Function`: 方法闭包
5. `Struct`: 结构体
6. `Enum`: 枚举
7. `UnKnown`: 未知推测, 该类型会自动对值类型进行推测处理
8. `Animation`: 动画类型

## 组件库内置类型
 
组件库内置类型指的是在组件库中直接构建的枚举和结构体类型

See [Buitin Type](./builtin_types.md) (Comming Soon, about 09-24)