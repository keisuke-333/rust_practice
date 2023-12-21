struct Foo<'a> {
    message: &'a str,
    number: i32,
}

// impl<'a> Foo<'a>の省略形。implブロック内で'aを使わない場合に使用できる
impl Foo<'_> {
    fn baa(&self) {
        println!("message: {}, number: {}", self.message, self.number)
    }
}

fn main() {
    let test = Foo {
        message: "test",
        number: 3,
    };
    test.baa();
}
