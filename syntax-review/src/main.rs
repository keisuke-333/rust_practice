mod vars;

fn main() {
    println!("Hello, world!");
    vars::run();
    vars::sub_a::fun_a();
    vars::sub_b::fun_b();

    let t1 = (3, true, 2.0);
    println!("{:?}", t1);
    println!("{}, {}, {}", t1.0, t1.1, t1.2);
    let (x1, y1, z1) = t1;
    println!("{}, {}, {}", x1, y1, z1);

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
}
