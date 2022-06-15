fn main() {
    //let b: Bar = Foo::default().into();
    macrotest::expand_without_refresh("tests/expand/*.rs");
}
