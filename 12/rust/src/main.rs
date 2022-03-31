fn main() {
    let triangle_number_to_find = 7; // 7 = the 7th number

    let mut sum = 0;
    for i in 1..triangle_number_to_find + 1 {
        sum += i;
    }

    let mut divisors = vec![];
    for i in 1..sum + 1 {
        if is_a_divisor(sum, i) {
            divisors.push(i);
        }
    }

    println!("{}: {:?}", sum, divisors);
}

fn is_a_divisor(num: i32, divisor: i32) -> bool {
    num % divisor == 0
}