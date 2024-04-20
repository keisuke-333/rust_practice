use std::thread;

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("main thread!");

    t1.join().unwrap();
    t2.join().unwrap();
}

fn f() {
    println!("another thread!");

    let id = thread::current().id();
    println!("thread id: {id:?}");
}
