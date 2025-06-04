# Field Attributes

To ignore a field or variant, annotate it with `#[serde(skip)]`. This is useful when transferring partially private data because it allows you to specify which data is exposed to Dart. See Serde's documentation on [variant](https://serde.rs/variant-attrs.html) and [field](https://serde.rs/field-attrs.html) attributes for more information on how Serde handles this attribute.

```{code-block} rust
:caption: Rust
#[derive(Serialize, RustSignal)]
struct UpdateMessage {
  event: String,
  struct_data: StructData,
  enum_data: EnumData,
}

#[derive(Serialize, SignalPiece)]
struct StructData {
  my_public_field: bool,
  #[serde(skip)]
  my_private_field: bool,
}

#[derive(Serialize, SignalPiece)]
enum EnumData {
  Variant1(i32, #[serde(skip)] i32),
  Variant2 {
    my_public_field: bool,
    #[serde(skip)]
    my_private_field: bool,
  },
}
```

Some attributes from Serde are banned at compile-time. This is because `rinf gen` analyzes Rust code statically by reading type annotations, and it cannot infer the type behind special Serde attributes like `#[serde(with = "...")]`. This mechanism ensures that `rinf gen` always produces exactly corresponding Dart code from Rust structs.
