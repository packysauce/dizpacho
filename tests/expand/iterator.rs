#[derive(Debug)]
struct MyCollection(Vec<i32>);

#[allow(dead_code)]
#[dizpacho::dizpacho]
impl MyCollection {
    fn new() -> MyCollection {
        MyCollection(Vec::new())
    }

    fn add(&mut self, elem: i32) {
        self.0.push(elem);
    }

    #[dizpacho(std::iter::Extend<i32>::extend<T>)]
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = i32>,
    {
        self.0.extend(iter)
    }
}

#[allow(dead_code)]
fn main() {
    let mut c = MyCollection::new();

    c.add(5);
    c.add(6);
    c.add(7);

    // let's extend our collection with three more numbers
    c.extend(vec![1, 2, 3]);

    // we've added these elements onto the end
    assert_eq!("MyCollection([5, 6, 7, 1, 2, 3])", format!("{c:?}"));
}
