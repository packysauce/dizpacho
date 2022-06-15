pub struct Foo(Box<Bar>);
pub struct Bar;
impl Foo {
    fn make_one(_other: Self) -> Bar {
        Bar
    }
}
impl From<Foo> for Bar {
    fn from(_other: Foo) -> Bar {
        <Foo>::make_one(_other)
    }
}
