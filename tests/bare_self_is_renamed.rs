#[path = "expand/bare_self_is_renamed.expanded.rs"]
mod bare_self;

#[test]
pub fn pass() {
    //let b: Bar = Foo::default().into();
    macrotest::expand_without_refresh("tests/expand/*.rs");
}
