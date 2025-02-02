use std::collections::HashMap;
use std::time::{Duration, Instant};

fn main() {
    let mut fib = Fibonacci::new();
    let (sequence_num, fib_val) = fib.run_for_one_second();

    println!(
        "Highest fibonacci number calculated: F({}) = {}",
        sequence_num, fib_val
    );
}

struct Fibonacci {
    memo: HashMap<u64, u64>,
}
impl Fibonacci {
    fn new() -> Self {
        Self {
            memo: HashMap::new(),
        }
    }

    fn start(&mut self, n: u64) -> u64 {
        if n <= 1 {
            return n;
        } else if self.memo.contains_key(&n) {
            return *self.memo.get(&n).unwrap();
        }

        let result = self.start(n - 1) + self.start(n - 2);
        self.memo.insert(n, result);

        result
    }

    fn run_for_one_second(&mut self) -> (u64, u64) {
        let start = Instant::now();
        let mut max_fib = 0;
        let mut n = 0;

        while start.elapsed() < Duration::from_secs(1) {
            let fib_num = self.start(n);
            max_fib = fib_num;
            n += 1;
        }

        (n - 1, max_fib)
    }
}
