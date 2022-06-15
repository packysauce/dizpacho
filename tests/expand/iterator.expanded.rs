struct MyCollection(Vec<i32>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for MyCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            MyCollection(ref __self_0_0) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_tuple(f, "MyCollection");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl MyCollection {
    fn new() -> MyCollection {
        MyCollection(Vec::new())
    }
    fn add(&mut self, elem: i32) {
        self.0.push(elem);
    }
    fn extend<T>(&mut self, iter: T) {
        ::core::panicking::panic("not yet implemented")
    }
}
impl std::iter::Extend<i32> for MyCollection {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = i32>,
    {
        <MyCollection>::extend(self, iter)
    }
}
fn main() {
    let mut c = MyCollection::new();
    c.add(5);
    c.add(6);
    c.add(7);
    c.extend(<[_]>::into_vec(box [1, 2, 3]));
    match (&"MyCollection([5, 6, 7, 1, 2, 3])", &{
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &[""],
            &[::core::fmt::ArgumentV1::new_debug(&c)],
        ));
        res
    }) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
