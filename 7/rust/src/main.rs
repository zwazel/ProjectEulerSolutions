fn main() {
    let prime_num_to_find = 10001;
    let mut prime_count = 0;
    let mut num = 2;
    loop {
        if is_prime(num) {
            prime_count += 1;
            if prime_count == prime_num_to_find {
                break;
            }
        }

        num += 1;
    }
    println!("{}", num);
}

fn is_prime(num: i32) -> bool {
    if num == 2 {
        return true;
    }
    if num % 2 == 0 {
        return false;
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