use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Starting the task...");
    delay().await;
    println!("Task completed.");
}

async fn delay() {
    sleep(Duration::from_secs(2)).await;
    println!("2 seconds have passed.");
}
