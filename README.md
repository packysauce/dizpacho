I got tired of writing boilerplate to wire up functions to traits,
so I made a helper

Decorate an impl block with `#[dizpacho]` and you can then use `dizpacho` attributes
on methods and associated functions to wire them up to whatever trait you like!

```rust
struct TooLazyToType(String);
struct OtherThing;

#[dizpacho::dizpacho]
impl TooLazyToType {
    /// Just call my new() function for default!
    #[dizpacho(Default::default)]
    fn new() -> Self {
        Self("howdy!".to_string())
    }
}
assert_eq!(&TooLazyToType::default().0, "howdy!");
```

```rust
struct TooLazyToType(String);
struct OtherThing;

#[dizpacho::dizpacho]
impl TooLazyToType {
    #[dizpacho(std::ops::Deref<Target = str>::deref)]
    fn as_str(&self) -> &str {
        &self.0
    }
}
#[dizpacho::dizpacho]
impl OtherThing {

    /// You can even do generics!
    #[dizpacho(From<Self>::from for TooLazyToType)]
    fn from_other(thing: OtherThing) -> TooLazyToType {
        TooLazyToType("I came from the other thing!".to_string())
    }
}

assert!(TooLazyToType::from(OtherThing).0.ends_with("thing!"))
```