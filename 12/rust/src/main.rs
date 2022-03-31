fn main() {
    let amount_of_divisors = 500;
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

        let amount_divisors = divisors.len();
        println!("divisors: {}, sum: {}, triangle: {}", amount_divisors, sum, current_triangle_counter);
        if amount_divisors >= amount_of_divisors as usize {
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