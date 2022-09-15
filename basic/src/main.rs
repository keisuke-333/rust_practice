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
}
