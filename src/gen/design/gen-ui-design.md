# GenUI-Design Documentation

## Template Section

### HTML-LIKE Syntax:

The syntax is reminiscent of HTML but has been enhanced for more structured processing.

Key considerations for maintaining a clear and focused template:

1. Direct string literals are not permitted.
2. Template syntax is prohibited; use Rust's `format!` for value bindings instead.
3. Declare basic attributes on tags using primitive types only; complex types should be bound separately.
4. Function bodies must not be directly written within attributes; instead, employ function bindings.
5. Attributes are strongly-typed, ensuring clear and explicit type associations.

## Script Section

1. Full support for Rust syntax is provided, allowing for powerful scripting capabilities.
2. Integration with Special Frameworks is permitted, expanding the possibilities for functionality and customization.

## Style Section

1. Styles are bound to tags by their names, enabling easy and intuitive styling.
2. Nesting of styles is allowed, offering a hierarchical approach to styling that mirrors traditional CSS.
3. Functions and bindings are supported within styles, allowing for dynamic styling based on logic and conditions.