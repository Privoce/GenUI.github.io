# Types(LiveValue)


表示应用程序中使用的各种活动值的枚举。

Enumeration representing various live values used in the application.

## Properties
|name|type|description|
|--|--|--|
|None|`LiveValue`|Represents no value.|
|Str|`&'static str`|Static string type.|
|String|`Rc<String>`|Reference-counted string type.|
|InlineString|`InlineString`|Inline string type.|
|Dependency|`Rc<String>`|Reference-counted dependency string type.|
|Bool|`bool`|Boolean type.|
|Int64|`i64`|64-bit integer type.|
|Uint64|`u64`|64-bit unsigned integer type.|
|Float32|`f32`|32-bit floating-point type.|
|Float64|`f64`|64-bit floating-point type.|
|Color|`u32`|Color value type represented as a 32-bit unsigned integer.|
|Vec2|`Vec2`|2D vector type.|
|Vec3|`Vec3`|3D vector type.|
|Vec4|`Vec4`|4D vector type.|
|Id|`LiveId`|Identifier type.|
|IdPath|`Rc<Vec<LiveId>>`|Reference-counted vector of identifiers.|
|ExprBinOp|`LiveBinOp`|Binary operation expression type.|
|ExprUnOp|`LiveUnOp`|Unary operation expression type.|
|ExprMember|`LiveId`|Member expression type.|
|ExprCall|`LiveId`|Function call expression type with identifier and argument count.|
|BareEnum|`LiveId`|Bare enumeration type.|
|Root|`Box<HashMap<LiveId,LiveScopeTarget>>`|Root type with a box containing a hashmap of identifiers and scope targets.|
|Array|`LiveValue`|Array type.|
|Expr|`LiveValue`|Expression type with optional expansion index.|
|TupleEnum|`LiveId`|Tuple enumeration type.|
|NamedEnum|`LiveId`|Named enumeration type.|
|Object|`LiveValue`|Object type.|
|Clone|`LiveId`|Clone type.|
|Deref|`LiveType`|Dereference type with live type and clone identifier.|
|Class|`LiveType`|Class type with live type and optional class parent pointer.|
|Close|`LiveValue`|Close type.|
|DSL|`LiveValue`|Domain-Specific Language type with token start, token count, and optional expansion index.|
|Import|`Box<LiveImport>`|Import type with a box containing live import.|