pub struct Foo(Box<Bar>);
pub struct Bar;
#[dizpacho::dizpacho]
impl Foo {
    #[dizpacho(From<Self>::from for Bar)]
    fn make_one(_other: Self) -> Bar {
        Bar
    }
}
