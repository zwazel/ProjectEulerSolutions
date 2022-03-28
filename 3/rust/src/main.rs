fn main() {
    let num = 13195;

    let prime_factors = get_prime_factors(num);

    println!("{:?}", prime_factors);
}

fn get_prime_factors(num: i32) -> Vec<i32> {
    let mut prime_factors: Vec<i32> = vec![];

    let mut multiply1 = if num % 10 == 0 {
        num / 10
    } else if num % 5 == 0 {
        num / 5
    } else if num % 2 == 0 {
        num / 2
    } else {
        1
    };

    let mut multiply2 = num / multiply1;

    println!("multiply1: {}\nmultiply2: {}", multiply1, multiply2);

    if (multiply1 * multiply2) == num {
        println!("Prime numbers equal to {} if multiplied", num);
    } else {
        println!("Prime numbers do not equal to {} if multiplied, they equal to {}", num, multiply1 * multiply2);
    }

    if is_prime(multiply1) {
        prime_factors.push(multiply1);
    } else {
        
    }

    if is_prime(multiply2) {
        prime_factors.push(multiply2);
    }

    prime_factors.sort();
    prime_factors.reverse();

    prime_factors
}

fn is_prime(num: i32) -> bool {
    let mut is_prime = true;

    for i in 2..num {
        if num % i == 0 {
            is_prime = false;
            break;
        }
    }

    is_prime
}