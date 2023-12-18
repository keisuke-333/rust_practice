use std::fmt::{Debug, Display};

struct Point<T> {
    x: T,
    y: T,
}

// ジェネリックな構造体のメソッド
impl<T: PartialOrd + Debug> Point<T> {
    fn max(&self) -> &T {
        if self.x >= self.y {
            &self.x
        } else {
            &self.y
        }
    }

    // 別のジェネリック型のメソッド
    fn print_arg<U: Display>(&self, val: U) {
        println!("self.x = {:?}", self.x);
        println!("val = {}", val);
    }
}

// 特定の型の場合のメソッド
impl Point<i32> {
    fn min(&self) -> i32 {
        if self.x <= self.y {
            self.x
        } else {
            self.y
        }
    }
}

fn main() {
    let p1 = Point { x: 3, y: 6 };
    let p2 = Point { x: 3.0, y: 3.0 };
    let p3 = Point { x: "a", y: "z" };
    println!("{}", p1.max());
    println!("{}", p2.max());
    println!("{}", p3.max());

    p1.print_arg("test");

    println!("{}", p1.min());
    // println!("{}", p2.min());  // ← This is error!Z
}
