fn fibonacci(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    a
}

fn main() {
    let n = 10;
    println!("Fibonacci of {} is {}", n, fibonacci(n));
}
