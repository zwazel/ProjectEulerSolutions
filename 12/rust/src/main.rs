fn main() {
    let amount_of_divisors = 6;
    let mut num = 0;

    let mut current_triangle_counter = 1;
    loop {
        let mut sum = 0;
        for i in 1..current_triangle_counter + 1 {
            sum += i;
        }

        let mut divisors = vec![];
        for i in 1..sum + 1 {
            if is_a_divisor(sum, i) {
                divisors.push(i);
            }
        }

        if divisors.len() >= amount_of_divisors {
            num = sum;
            break;
        }

        current_triangle_counter += 1;
    }

    println!("{}", num);
}

fn is_a_divisor(num: i32, divisor: i32) -> bool {
    num % divisor == 0
}