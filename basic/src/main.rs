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

}
