fn main() {
    let max_searches = 2_000_000;
    let mut sum: i64 = 0;

    for i in 0..max_searches {
        if is_prime(i) {
            sum += i as i64;
        }
    }

    println!("Sum of primes below {} is {}", max_searches, sum);
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