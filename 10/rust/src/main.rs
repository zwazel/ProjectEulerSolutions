fn main() {
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