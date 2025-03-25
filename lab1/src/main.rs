fn fib(n: usize) -> Vec<u64> {
    let mut fib_list = vec![0, 1];
    while fib_list.len() < n {
        let next_fib = fib_list[fib_list.len() - 1] + fib_list[fib_list.len() - 2];
        fib_list.push(next_fib);
    }
    fib_list
}

fn pythagor_table(size: usize) {
    print!("{:<4}", "");

    for i in 1..=size {
        print!("{:<4}", i);
    }
    println!();

    for i in 1..=size {
        print!("{:<4}", i);
        for j in 1..=size {
            print!("{:<4}", i * j);
        }
        println!();
    }
}

fn main() {
    let n = 5;
    let result = fib(n);
    println!("Task 1: for n = {}, Fibonacci sequence is {:?}", n, result);

    println!("Task 2:");
    let size = 10;
    pythagor_table(size);
}
