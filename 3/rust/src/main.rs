fn main() {
    let num = 13195;

    let prime_factors = get_prime_factors(num);

    println!("{:?}", prime_factors);
}

fn get_prime_factors(num: i32) -> Vec<i32> {
    let mut prime_factors: Vec<i32> = vec![1, 2, 3];


    prime_factors.sort();
    prime_factors.reverse();

    prime_factors
}

