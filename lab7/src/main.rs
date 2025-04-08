use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::time::Instant;
use std::fs;
use std::io::Read;

async fn async_version() {
    let start = Instant::now();

    let mut file = File::open("sample.txt").await.expect("Cannot open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.expect("Cannot read file");
    println!("(async) File read: {} bytes", contents.len());

    let response = reqwest::get("https://httpbin.org/get")
        .await
        .expect("Failed to fetch")
        .text()
        .await
        .expect("Failed to read response");
    println!("(async) API response length: {}", response.len());

    let duration = start.elapsed();
    println!("(async) Time taken: {:?}", duration);
}

fn sync_version() {
    let start = std::time::Instant::now();

    let mut file = fs::File::open("sample.txt").expect("Cannot open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Cannot read file");
    println!("(sync) File read: {} bytes", contents.len());

    let response = reqwest::blocking::get("https://httpbin.org/get")
        .expect("Failed to fetch")
        .text()
        .expect("Failed to read response");
    println!("(sync) API response length: {}", response.len());

    let duration = start.elapsed();
    println!("(sync) Time taken: {:?}", duration);
}

fn main() {
    println!("Running async version...");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async_version());

    println!("\nRunning sync version...");
    sync_version();
}
