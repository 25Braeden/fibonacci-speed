use rug::Integer;
use std::time::{Duration, Instant};

fn run_for_one_second() -> (u64, Integer) {
    let start_time = Instant::now();
    let mut a = Integer::from(0);
    let mut b = Integer::from(1);
    let mut n = 0;

    while start_time.elapsed() < Duration::from_secs(1) {
        let next: Integer = (&a + &b).into();
        // magic memory trick -> set a to b in memory then set b to next in memory
        a = std::mem::replace(&mut b, next);
        n += 1;
    }

    (n, a)
}

fn scientific_notation(fib_value: &Integer) -> (u64, i64) {
    let fib_str = fib_value.to_string();
    let first_8_digits: u64 = fib_str.chars()
        .take(10)
        .collect::<String>()
        .parse()
        .unwrap();
    let exponent = (fib_str.len() as i64) - 1;
    (first_8_digits, exponent)
}

fn main() {
    let (sequence_number, fib_value) = run_for_one_second();

    let (first_8_digits, exponent) = scientific_notation(&fib_value);
    println!(
        "Highest Fibonacci number calculated: F({}) = {} × 10^{}",
        sequence_number, first_8_digits, exponent
    );
}