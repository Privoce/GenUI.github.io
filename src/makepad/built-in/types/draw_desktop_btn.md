# DrawDesktopButton

Handles the drawing properties specific to the desktop button.

## Properties
|decorate|name|type|description|
|--|--|--|--|
|deref|draw_super|`DrawQuad`|Inherits properties from `DrawQuad`.|
|live|hover|`f32`|Represents the hover state.|
|live|pressed|`f32`|Represents the pressed state.|
|live|button_type|`DesktopButtonType`|Specifies the type of desktop button (e.g., minimize, maximize, close).|

## Example

```rust
draw_bg: {
    button_type: WindowMin,
}
```

---

# DesktopButtonType

An enumeration defining different types of desktop buttons.

## Properties
|decorate|name|type|description|
|--|--|--|--|
|live|WindowsMin|`shader_enum(1)`|Represents the minimize button.|
|live|WindowsMax|`shader_enum(2)`|Represents the maximize button.|
|live|WindowsMaxToggled|`shader_enum(3)`|Represents the toggled maximize button.|
|live|WindowsClose|`shader_enum(4)`|Represents the close button.|
|live|XRMode|`shader_enum(5)`|Represents the XR mode button.|
|pick|Fullscreen|`shader_enum(6)`|Represents the fullscreen button.|

## Example

```rust
button_type: WindowMin,
button_type: WindowMax,
button_type: Fullscreen,
```