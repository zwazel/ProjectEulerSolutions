use std::time::Instant;

fn main() {
    let start = Instant::now();
    let max_searches = 2_000_000;
    let mut sum: i128 = 0;

    let mut primes: Vec<bool> = vec![false, false];
    for _i in 2..max_searches {
        primes.push(true);
    }

    cross_out(2, &mut primes);

    for i in 0..primes.len() {
        if primes[i] {
            sum += i as i128;
        }
    }


    println!("Sum of primes below {} is {}", max_searches, sum);
    let elapsed = start.elapsed();
    println!("Millis: {} ms", elapsed.as_millis());

    println!("Doing it the other way:");
    let start = Instant::now();

    sum = 0;
    for i in 0..max_searches {
        if is_prime(i) {
            sum += i as i128;
        }
    }

    println!("Sum of primes below {} is {}", max_searches, sum);
    let elapsed = start.elapsed();
    println!("Millis: {} ms", elapsed.as_millis());
}

fn is_prime(num: i64) -> bool {
    if num < 2 {
        return false;
    }

    if num == 2 {
        return true;
    }

    let mut i = 3;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 2;
    }

    return true;
}

fn cross_out(mut p: usize, primes: &mut Vec<bool>) {
    while p < primes.len() {
        let mut i = p * p;
        while i < primes.len() {
            primes[i] = false;

            i += p;
            if i > primes.len() {
                break;
            }
        }
        p += 1;
    }
}