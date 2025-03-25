use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;


fn fibonacci(n: u64) -> u64 {
    if n <= 1 { return n; }
    let (mut a, mut b) = (0, 1);
    for _ in 0..n - 1 {
        let tmp = a;
        a = b;
        b = tmp + b;
    }
    b
}

fn generate_array(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(-200..=100)).collect()
}

fn sequential_search(arr: &[i32]) -> i32 {
    let mut counts = vec![0; 301];

    for &num in arr { counts[(num + 200) as usize] += 1; }

    let mut max_count = 0;
    let mut frequent_num = 0;

    for (num, &count) in counts.iter().enumerate() {
        if count > max_count {
            max_count = count;
            frequent_num = num as i32 - 200;
        }
    }
    frequent_num
}

fn parallel_search(arr_chunks: Vec<Vec<i32>>) -> i32 {
    let mut handles = vec![];
    let counts = Arc::new(Mutex::new(vec![0; 301]));

    for chunk in arr_chunks {
        let chunk_counts = counts.clone();
        let handle = thread::spawn(move || {
            let mut local_counts = vec![0; 301];
            for &num in &chunk {local_counts[(num + 200) as usize] += 1; }
            let mut counts = chunk_counts.lock().unwrap();
            for (i, &count) in local_counts.iter().enumerate() { (*counts)[i] += count; }
        });
        handles.push(handle);
    }

    for handle in handles { handle.join().unwrap(); }

    let counts = counts.lock().unwrap();
    let (frequent_num, _) = counts
        .iter()
        .enumerate()
        .max_by_key(|(_, &count)| count)
        .unwrap();
    frequent_num as i32 - 200
}

fn main() {
    // Task 1
    println!("Task1\n");
    let start_range = 0;
    let end_range = 10;
    let mut handles = vec![];

    for i in start_range..end_range {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            println!("Fibonacci({}) = {}", i, fibonacci(i));
            let duration = start.elapsed();
            println!("Thread {} execution time: {:?}", i, duration);
        });
        handles.push(handle);
    }

    for handle in handles { handle.join().unwrap(); }

    // Task 2
    println!("\nTask2\n");
    let array_size = 100000;
    let array = generate_array(array_size);
    let m_values = vec![1, 2, 4, 8];
    println!("Sequential Search:");
    let start_sequential = Instant::now();
    let frequent_num_sequential = sequential_search(&array);
    let duration_sequential = start_sequential.elapsed().as_secs_f64();
    println!("Most frequent number: {}", frequent_num_sequential);
    println!("Time taken: {:.6} seconds", duration_sequential);
    println!("Parallel Search:");

    for &m in &m_values {
        println!("With {} threads:", m);
        let start_parallel = Instant::now();
        let arr_chunks: Vec<Vec<i32>> = array
            .chunks(array_size / m)
            .map(|chunk| chunk.to_vec())
            .collect();
        let frequent_num_parallel = parallel_search(arr_chunks);
        let duration_parallel = start_parallel.elapsed().as_secs_f64();
        println!("Most frequent number: {}", frequent_num_parallel);
        println!("Time taken: {:.6} seconds", duration_parallel);
    }
}
