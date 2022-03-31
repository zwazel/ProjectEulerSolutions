fn main() {
    let mut num: i64 = 600851475143;

    let mut prime_factors: Vec<i64> = Vec::new();

    while num % 2 == 0 {
        prime_factors.push(2);
        num /= 2;
    }

    for i in 3..(num as f64).sqrt() as i64 {
        while num % i == 0 {
            prime_factors.push(i);
            num /= i;
        }
    }

    if num > 2 {
        prime_factors.push(num);
    }

    prime_factors.sort();
    prime_factors.reverse();

    println!("{}", prime_factors[0]);
}