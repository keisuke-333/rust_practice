use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let shared_data = Arc::new(Mutex::new(0));

    let shared_data_clone1 = Arc::clone(&shared_data);
    let thread1 = thread::spawn(move || {
        let data = shared_data_clone1.lock().unwrap();
        println!("スレッド1: データ読み取り - {}", *data);
    });

    let shared_data_clone2 = Arc::clone(&shared_data);
    let thread2 = thread::spawn(move || {
        let mut data = shared_data_clone2.lock().unwrap();
        *data += 2; // データ書き込み
        println!("スレッド2: データ書き込み - {}", *data);
    });

    let shared_data_clone3 = Arc::clone(&shared_data);
    let thread3 = thread::spawn(move || {
        let data = shared_data_clone3.lock().unwrap();
        println!("スレッド3: データ読み取り - {}", *data);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();

    let final_data = shared_data.lock().unwrap();
    println!("最終データ: {}", *final_data);
}