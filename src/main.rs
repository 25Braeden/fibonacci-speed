use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct Fibonacci {
    memo: HashMap<u64, BigUint>,
}

impl Fibonacci {
    fn new() -> Self {
        Self {
            memo: HashMap::new(),
        }
    }

    fn start(&mut self, n: u64) -> BigUint {
        if n <= 1 {
            return if n == 0 { BigUint::zero() } else { BigUint::one() };
        }

        if let Some(result) = self.memo.get(&n) {
            return result.clone();
        }

        let result1 = self.start(n - 1);
        let result2 = self.start(n - 2);
        
        let result = result1 + result2;
        self.memo.insert(n, result.clone());

        result
    }

    fn run_for_one_second(&mut self) -> (u64, BigUint) {
        let start_time = Instant::now();
        let mut max_fib = BigUint::zero();
        let mut n = 0;

        while start_time.elapsed() < Duration::from_secs(1) {
            max_fib = self.start(n);
            n += 1;
        }

        (n - 1, max_fib)
    }
}

fn scientific_notation(fib_value: &BigUint) -> (u64, i64) {
    let fib_str = fib_value.to_str_radix(10);
    let first_8_digits: u64 = fib_str.chars().take(8).collect::<String>().parse().unwrap();
    
    let exponent = (fib_str.len() as i64) - 1;
    
    (first_8_digits, exponent)
}

fn main() {
    let mut fib = Fibonacci::new();
    let (sequence_number, fib_value) = fib.run_for_one_second();

    let (first_8_digits, exponent) = scientific_notation(&fib_value);
    println!(
        "Highest Fibonacci number calculated: F({}) = {} × 10^{}",
        sequence_number,
        first_8_digits,
        exponent
    );

    /*
    checking fib(1000) against http://www.fullbooks.com/The-first-1001-Fibonacci-Numbers.html 
    assuming that if fib(1000) works all future nums should also work.
    */

    let fib_1000 = fib.start(1000);
    let (fib_1000_digits, fib_1000_exp) = scientific_notation(&fib_1000);
    println!(
        "Fibonacci(1000) = {} × 10^{}",
        fib_1000_digits, fib_1000_exp
    );
}