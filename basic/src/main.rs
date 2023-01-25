// smart pointer
use basic::sample_trait::{double_area, Circle1, Rectangle1, Shape1};
use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    env,
    rc::Rc,
};

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

// custom iterator

struct Counter {
    start: u32,
    end: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.start > self.end {
            None
        } else {
            let result = Some(self.start);
            self.start += 1;
            result
        }
    }
}

// unit test
#[test]
fn test_sample() {
    let a = 1 + 1;
    let b = 2;
    assert_eq!(a, b);
}

// panic test
fn maybe_panic(flag: bool) {
    if flag == false {
        println!("safe!");
        // do something
        panic!("unexpected!");
    } else {
        panic!("flag is true!");
    }
}

// test control
#[cfg(test)]
mod test_module {
    #[test]
    #[should_panic(expected = "flag is true")]
    fn test_maybe_panic() {
        super::maybe_panic(true);
    }

    #[test]
    fn test_calc_add() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    #[ignore]
    fn test_calc_diff() {
        assert_eq!(1 - 1, 0);
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
    let c3 = '😀';
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

    // custom iterator
    let ci1 = Counter { start: 1, end: 5 };
    for i in ci1 {
        println!("{}", i);
    }

    // iterator methods
    let im1 = vec![1, 2, 3, 4, 5];
    let im2 = im1.iter().map(|x| x * 2);
    for val in im2 {
        println!("im2 = {:?}", val);
    }

    let im3: Vec<_> = im1.iter().map(|x| x * 2).collect();
    println!("{:?}", im3);

    let im4: Vec<_> = im1.iter().filter(|x| *x % 2 != 0).collect();
    println!("{:?}", im4);

    let im5 = im1.iter().count();
    println!("{:?}", im5);

    let im6: i32 = im1.iter().sum();
    let im7: i32 = im1.iter().product();
    println!("im6 = {:?}", im6);
    println!("im7 = {:?}", im7);

    let im8 = im1.iter().min();
    let im9 = im1.iter().max();
    println!("min = {:?}", im8);
    println!("max = {:?}", im9);

    let im10 = im1.iter().fold(0, |sum, x| sum + x);
    println!("im10 = {:?}", im10);

    // collection - vector
    let cv1 = vec!["Rust", "Python", "Java"];
    println!("{:?}", cv1);
    println!("{:?}", cv1.as_ptr());
    println!("{:?}", cv1.len());
    println!("{:?}", cv1.capacity());

    println!("{:?}", cv1[0]);
    println!("{:?}", &cv1[0]);
    println!("{:?}", cv1.get(0));

    let mut cv2 = vec!["Rust", "Python", "Java"];
    cv2.push("PHP");
    println!("{:?}", cv2);
    let cv3 = cv2.pop();
    println!("{:?}", cv3);
    println!("{:?}", cv2);
    cv2.insert(1, "PHP");
    println!("{:?}", cv2);
    cv2.remove(2);
    println!("{:?}", cv2);

    let cv4 = vec!["Rust", "Python", "Java"];
    let cv5 = vec!["PHP", "Go"];
    let cv6 = [cv4, cv5].concat();
    println!("{:?}", cv6);
    let (cv7, cv8) = cv6.split_at(2);
    println!("{:?}", cv7);
    println!("{:?}", cv8);
    let mut cv9 = vec![3, 6, 1, 7, 2];
    cv9.sort();
    println!("{:?}", cv9);
    cv9.reverse();
    println!("{:?}", cv9);

    #[derive(Debug)]
    struct Cv10 {
        val1: i32,
        val2: i32,
    }
    let mut cv11 = vec![
        Cv10 { val1: 3, val2: 2 },
        Cv10 { val1: 2, val2: 1 },
        Cv10 { val1: 1, val2: 3 },
    ];
    cv11.sort_by_key(|s| s.val1);
    println!("sort by key 1 = {:?}", cv11);
    cv11.sort_by_key(|s| s.val2);
    println!("sort by key 2 = {:?}", cv11);

    let cv12 = vec![3, 6, 1, 7, 2];
    println!("{:?}", cv12.contains(&6));

    let cv13 = cv12.iter().position(|x| *x == 2);
    println!("{:?}", cv13);

    // queue
    let mut qu1 = VecDeque::new();
    // let mut qu1 = VecDeque::from(vec![1, 2, 3]);
    qu1.push_back(1);
    qu1.push_back(2);
    qu1.push_back(3);
    println!("{:?}", qu1);
    println!("{:?}", qu1.pop_front());
    println!("{:?}", qu1);

    let mut qubh = BinaryHeap::new();
    qubh.push(1);
    qubh.push(10);
    qubh.push(20);
    qubh.push(2);
    println!("{:?}", qubh);
    println!("{:?}", qubh.pop());
    println!("{:?}", qubh);

    // map
    let mut map1 = HashMap::new();
    map1.insert("Japan", 11);
    map1.insert("USA", 3);
    map1.insert("China", 1);
    map1.insert("India", 2);
    println!("{:?}", map1);

    map1.insert("Japan", 10);
    println!("{:?}", map1);

    println!("{:?}", map1.get("USA"));
    println!("{:?}", map1.remove("India"));
    println!("{:?}", map1);

    for (k, v) in &map1 {
        println!("{:?}", k);
        println!("{:?}", v);
    }

    // set
    let mut set1 = HashSet::new();
    set1.insert(1);
    set1.insert(1);
    set1.insert(1);
    println!("{:?}", set1);
    set1.insert(2);
    set1.insert(3);
    set1.insert(4);
    println!("{:?}", set1);

    println!("{:?}", set1.contains(&2));
    println!("{:?}", set1.remove(&2));
    println!("{:?}", set1);

    let mut set2 = HashSet::new();
    set2.insert(1);
    set2.insert(2);
    set1.insert(3);
    set1.insert(5);

    println!("set1: {:?}", set1);
    println!("set2: {:?}", set2);

    let set3 = &set1 | &set2;
    println!("set3: {:?}", set3);
    let set4 = &set1 & &set2;
    println!("set4: {:?}", set4);
    let set5 = &set1 - &set2;
    println!("set5: {:?}", set5);
    let set6 = &set1 ^ &set2;
    println!("set6: {:?}", set6);

    // command line argument
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
