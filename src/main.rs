use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::time::{Duration, Instant};
use std::mem;

fn run_for_one_second() -> (u64, BigUint) {
    let start_time = Instant::now();
    let mut a = BigUint::zero();
    let mut b = BigUint::one();
    let mut sequence_number = 0;
    let mut max_fib = a.clone();

    if start_time.elapsed() < Duration::from_secs(1) {
        max_fib = a.clone();
        sequence_number = 0;
    }

    if start_time.elapsed() < Duration::from_secs(1) {
        a = BigUint::one();
        b = BigUint::one();
        max_fib = a.clone();
        sequence_number = 1;
    }

    while start_time.elapsed() < Duration::from_secs(1) {
        let next = &a + &b;
        a = mem::replace(&mut b, next);
        sequence_number += 1;
        max_fib = b.clone();
    }

    (sequence_number, max_fib)
}

fn scientific_notation(fib_value: &BigUint) -> (u64, i64) {
    let fib_str = fib_value.to_str_radix(10);
    let first_8_digits: u64 = fib_str.chars().take(8).collect::<String>().parse().unwrap();
    let exponent = (fib_str.len() as i64) - 1;
    (first_8_digits, exponent)
}

fn main() {
    let (sequence_number, fib_value) = run_for_one_second();

    let (first_8_digits, exponent) = scientific_notation(&fib_value);
    println!(
        "Highest Fibonacci number calculated: F({}) = {} × 10^{}",
        sequence_number,
        first_8_digits,
        exponent
    );

    // test against known number
    let mut a = BigUint::zero();
    let mut b = BigUint::one();
    for _ in 2..=1000 {
        let next = &a + &b;
        a = mem::replace(&mut b, next);
    }
    let (fib_1000_digits, fib_1000_exp) = scientific_notation(&b);
    println!(
        "Fibonacci(1000) = {} × 10^{}",
        fib_1000_digits, fib_1000_exp
    );
}