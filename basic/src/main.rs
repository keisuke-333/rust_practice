// smart pointer
use basic::sample_trait::{double_area, Circle1, Rectangle1, Shape1};
use std::rc::Rc;

// function
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// ownership
fn concat(a: String, b: String) -> (String, String, String) {
    let c = format!("{} {}", a, b);
    (c, a, b)
}

// reference
fn concat2(a: &String, b: &String) -> String {
    let c = format!("{} {}", a, b);
    c
}

// struct
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // method
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // associated function
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

// enum
enum Shape {
    Circle,
    Triangle(u32),
}

impl Shape {
    fn sample_method(&self) {
        println!("call sample method");
    }
}

// generics
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}

fn main() {
    // print
    println!("Hello, world!");
    print!("Hello, Rust!\n");
    println!("Hello, {}", "user!");

    // variable
    let a = 1;
    println!("{}", a);

    let mut b = 10; // mutable
    println!("{}", b);
    b = 100;
    println!("{}", b);

    let c = 1000;
    println!("{}", c);
    let c = "test"; // shadowing
    println!("{}", c);

    // constant
    const A: i32 = 111;
    println!("{}", A);
    const B: &str = "test";
    println!("{}", B);

    // numeric type
    let f = 1 as f64 + 2.0;
    println!("{}", f);

    // tuple
    let t1 = (1, true, 2.0);
    println!("{:?}", t1);
    let i = t1.0;
    println!("{}", i);
    let (x, y, z) = t1;
    let (x1, y1, _) = t1;
    println!("{} {} {} {} {}", x, y, z, x1, y1);

    // array
    let a1 = [1, 2, 3];
    let a2 = [0; 9];
    let a3 = a1[0];
    println!("{:?} {:?} {:?}", a1, a2, a3);
    let [ax, ay, az] = a1;
    println!("{:?} {:?} {:?}", ax, ay, az);

    // slice
    let s = [1, 2, 3];
    let s1 = &s[0..2];
    let s2 = &s[0..=2];
    let s3 = &s[..];
    println!("{:?} {:?} {:?}", s1, s2, s3);

    // vector
    let mut v1 = vec![1, 2, 3];
    let vv1 = v1.pop();
    let mut v2 = vec![0; 9];
    v2.push(10);
    println!("{:?} {:?} {:?}", v1, v2, vv1);
    let mut v3 = Vec::new();
    v3.push("test1");
    v3.push("test2");
    v3.push("test3");
    println!("{:?}", v3);
    let v4 = v3[0];
    let v5 = v3.get(1);
    let v6 = &v3[..];
    println!("{:?} {:?} {:?}", v4, v5, v6);

    // char
    let c1 = 'a';
    let c2 = '@';
    let c3 = '????';
    println!("{} {} {}", c1, c2, c3);

    // string
    let s1 = "Hello";
    let s2 = String::from("Rust");
    let s3 = "user".to_string();
    println!("{} {} {}", s1, s2, s3);
    let mut s4 = String::from("test");
    s4.push_str("!");
    println!("{}", s4 + "!!"); // String + &str
    let s5 = format!("{} {} {}", s1, s2, s3);
    println!("{}", s5);

    // function
    println!("{}", add(1, 2));

    // block
    let b1 = 10;
    println!("{}", b1);
    {
        let b1 = 5; // not shadowing
        println!("{}", b1);
    }
    println!("{}", b1);
    let block = { 100 };
    println!("{}", block);

    // if
    let ifx = 5;
    if ifx > 0 && ifx < 10 {
        println!("OK!");
    }
    let ify = if ifx < 10 { ifx } else { 0 };
    println!("{}", ify);

    // match
    let m1 = 2;
    match m1 {
        0 => println!("zero"),
        1 => println!("one"),
        _ => {
            println!("other");
            println!("!");
        }
    };
    let m2 = match m1 {
        0 => 0,
        1 => 10,
        _ => 100,
    };
    println!("{}", m2);

    // loop
    let mut lcnt = 0;
    loop {
        println!("loop{}", lcnt + 1);
        if lcnt == 9 {
            break;
        }
        lcnt += 1;
    }

    // while
    let mut wcnt = 0;
    while wcnt < 10 {
        println!("while{}", wcnt + 1);
        wcnt += 1;
    }

    // for
    let fcnts = 1..=10;
    for fcnt in fcnts {
        println!("for{}", fcnt);
    }

    // ownership
    let mut o1 = vec![1, 2, 3];
    println!("o1 ptr: {:?}", o1.as_ptr());
    println!("o1[o] : {:p}", &o1[0]);
    println!("o1 len: {}", o1.len());
    println!("o1 capacity: {}", o1.capacity());
    o1.push(4);
    println!("o1 ptr: {:?}", o1.as_ptr());
    println!("o1 len: {}", o1.len());
    println!("o1 capacity: {}", o1.capacity());
    let o2 = o1;
    let o3 = o2.clone();
    println!("o2 ptr: {:?}", o2.as_ptr());
    println!("o3 ptr: {:?}", o3.as_ptr());
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let (s, s1, s2) = concat(s1, s2);
    println!("{}", s);
    println!("{}", s1);
    println!("{}", s2);

    // reference
    let r1 = String::from("Hello");
    let r2 = String::from("Rust");
    let r3 = concat2(&r1, &r2);
    println!("{}", r3);
    println!("{}", r1);
    println!("{}", r2);

    // lifetime
    let lt1;
    {
        let lt2 = 1;
        lt1 = &lt2;
        println!("{}", lt1);
    }
    // println!("{}", lt1); // error!

    // smart pointer
    let sp1 = Box::new(1); // Box
    println!("sp1:{:p}", sp1);
    println!("{}", *sp1 + 2);
    let sp2 = Rc::new("hello".to_string());
    println!("refCount: {}", Rc::strong_count(&sp2));
    {
        let sp3 = Rc::clone(&sp2);
        println!("{:p}", sp2);
        println!("{:p}", sp3);
        println!("refCount: {}", Rc::strong_count(&sp2));
    }
    println!("refCount: {}", Rc::strong_count(&sp2));

    // struct
    let mut rectangle = Rectangle {
        width: 10,
        height: 5,
    };
    println!("width: {}", rectangle.width);
    println!("height: {}", rectangle.height);
    rectangle.height = 10;
    println!("height: {}", rectangle.height);
    println!("area: {}", rectangle.area());
    let rectangle2 = Rectangle::new(9, 9);
    println!("width: {}", rectangle2.width);
    println!("height: {}", rectangle2.height);
    println!("area: {}", rectangle2.area());

    // enum
    let ec = Shape::Circle;
    let et = Shape::Triangle(3);
    ec.sample_method();
    et.sample_method();

    // option
    let ov = vec![1, 2, 3];
    let oval = ov.get(2);
    match oval {
        // match guard
        Some(x) if *x == 3 => println!("value is 3"),
        Some(x) => println!("value exists: {}", x),
        None => println!("value is None"),
    }
    if let Some(x) = oval {
        println!("val={}", x)
    }

    // trait
    let rect = Rectangle1 {
        width: 4.0,
        height: 5.0,
    };
    let circle = Circle1 { radius: 2.0 };
    println!("Rectangle area is: {}", rect.calc_area());
    println!("Rectangle perimeter is: {}", rect.calc_perimeter());
    Rectangle1::do_something();
    println!("Circle area is: {}", circle.calc_area());
    println!("Circle perimeter is: {}", circle.calc_perimeter());
    Circle1::do_something();
    println!("Rectangle default: {}", rect.default_something());
    println!("Circle default: {}", circle.default_something());
    println!("Rectangle double area: {}", double_area(&rect));
    println!("Circle double area: {}", double_area(&circle));

    // derive
    #[derive(Debug, PartialEq)]
    struct Drv {
        val1: i32,
        val2: i32,
    }
    println!("{:?}", Drv { val1: 1, val2: 2 });
    let de1 = Drv { val1: 1, val2: 2 };
    let de2 = Drv { val1: 1, val2: 2 };
    println!("{:?}", de1 == de2);

    // generics
    println!("{}", max(1, 2));
    println!("{}", max(1.1, 1.2));
    println!("{}", max("x", "a"));

    // closure
    let cl1 = 10;
    let cl2 = |x: i32| x + cl1;
    println!("{:?}", cl2(10));

    // iterator
    let it1 = vec![1, 2, 3, 4, 5];
    let v1_iter = it1.iter();
    for x in v1_iter {
        println!("{:?}", x);
    }
    let mut v2_iter = it1.iter();
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
}
