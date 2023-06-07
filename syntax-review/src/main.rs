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
}
