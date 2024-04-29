use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let shared_data = Arc::new(RwLock::new(0));

    let shared_data_clone1 = Arc::clone(&shared_data);
    let thread1 = thread::spawn(move || {
        let data = shared_data_clone1.read().unwrap();
        println!("スレッド1: データ読み取り - {}", data);
    });

    let shared_data_clone2 = Arc::clone(&shared_data);
    let thread2 = thread::spawn(move || {
        let data = shared_data_clone2.read().unwrap();
        println!("スレッド2: データ読み取り - {}", data);
    });

    let shared_data_clone3 = Arc::clone(&shared_data);
    let thread3 = thread::spawn(move || {
        let mut data = shared_data_clone3.write().unwrap();
        *data += 1; // データ書き込み
        println!("スレッド3: データ書き込み - {}", data);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();

    let final_data = shared_data.read().unwrap();
    println!("最終データ: {}", *final_data);
}