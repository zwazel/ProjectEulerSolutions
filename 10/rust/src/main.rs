use std::fmt::{Debug, Formatter};

fn main() {
    let max_searches = 2_000_000;
    let mut sum: i64 = 0;

    let mut primes: Vec<PrimeSieve> = Vec::new();
    for i in 2..max_searches {
        primes.push(
            PrimeSieve {
                num: i,
                is_prime: false,
                checked: false,
            }
        );
    }

    println!("{:?}", primes);


    println!("Sum of primes below {} is {}", max_searches, sum);
}

struct PrimeSieve {
    num: i64,
    is_prime: bool,
    checked: bool,
}

impl Debug for PrimeSieve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimeSieve")
            .field("num", &self.num)
            .field("is_prime", &self.is_prime)
            .field("checked", &self.checked)
            .finish()
    }
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}