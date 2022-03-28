fn main() {
    let mut biggest_palindrome = 0;

    let min = 100;
    let max = 1000;

    for i in (min..max).rev().step_by(1) {
        for j in (min..max).rev().step_by(1) {
            let num = i * j;
            if is_palindrome(num) {
                if num > biggest_palindrome {
                    biggest_palindrome = num;
                }
            }
        }
    }
    println!("{}", biggest_palindrome);
}

fn is_palindrome(num: i32) -> bool {
    let mut product_string = num.to_string();
    let mut product_string_rev = product_string.chars().rev().collect::<String>();

    product_string == product_string_rev
}