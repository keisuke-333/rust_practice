pub fn test_fn1() {
    println!("test_fn1");
}

pub struct TestStruct {
    pub val1: i32,
    pub val2: i32,
}

impl TestStruct {
    pub fn new(val1: i32, val2: i32) -> Self {
        TestStruct { val1, val2 }
    }
}
