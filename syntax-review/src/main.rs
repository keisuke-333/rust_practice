mod vars;

fn add(a: i32, b: i32) -> i32 {
    // return a + b;
    a + b
}

fn main() {
    println!("Hello, world!");
    vars::run();
    vars::sub_a::fun_a();
    vars::sub_b::fun_b();

    let mut t1 = (3, true, 2.0);
    println!("{:?}", t1);
    println!("{}, {}, {}", t1.0, t1.1, t1.2);
    let (x1, y1, z1) = t1;
    println!("{}, {}, {}", x1, y1, z1);
    let (ref mut x1_ptr, _, _) = t1;
    *x1_ptr = 30;
    println!("{:p}", x1_ptr);
    println!("{:?}", t1);

    let a1 = [1, 2, 3];
    let [ar1, ar2, ar3] = a1;
    println!("{}, {}, {}", ar1, ar2, ar3);
    let a2 = &a1[1..];
    println!("{:?}", a2);

    let mut ary1 = vec![1, 2, 3];
    ary1.push(100);
    println!("{:?}", ary1);
    let mut ary2 = Vec::new();
    ary2.push(10);
    ary2.push(100);
    ary2.push(1000);
    println!("{:?}", ary2);

    let a3 = ary2.pop();
    println!("{:?}", ary2);
    println!("{:?}", a3);

    let a4 = ary2[0];
    let a5 = ary2.get(0);
    let a6 = ary2.get(100);
    println!("{}, {:?}, {:?}", a4, a5, a6);

    let a7 = String::from("Rust");
    let mut a8 = "C".to_string();
    a8.push_str("++");
    println!("{}, {}", a7, a8);
    println!("{}", a7 + " 1.68.0");
    let a9 = format!("{} {}", "C", a8);
    println!("{}", a9);

    println!("{}", add(3, 6));
}
