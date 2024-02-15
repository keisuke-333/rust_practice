use std::sync::{RwLock, Arc};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(300)); // 初期データをRwLockで保護

    // 読み込みスレッド1
    let r1 = Arc::clone(&data);
    let read_thread_1 = thread::spawn(move || {
        let read_data = r1.read().unwrap(); // 読み込みアクセス
        println!("読み込みスレッド1: {}", *read_data);
    });

    // 読み込みスレッド2
    let r2 = Arc::clone(&data);
    let read_thread_2 = thread::spawn(move || {
        let read_data = r2.read().unwrap(); // 読み込みアクセス
        println!("読み込みスレッド2: {}", *read_data);
    });

    // 書き込みスレッド
    let w = Arc::clone(&data);
    let write_thread = thread::spawn(move || {
        let mut write_data = w.write().unwrap(); // 書き込みアクセス
        *write_data += 1;
        println!("書き込みスレッド: {}", *write_data);
    });

    read_thread_1.join().unwrap();
    read_thread_2.join().unwrap();
    write_thread.join().unwrap();
}