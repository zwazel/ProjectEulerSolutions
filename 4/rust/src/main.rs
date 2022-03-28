fn main() {
    let mut biggest_palindrome = 0;
    let mut done = false;

    for i in (10..100).rev().step_by(1) {
        if done {
            break;
        }
        for j in (10..100).rev().step_by(1) {
            let num = i * j;
            let digits = num.to_string().chars().collect::<Vec<char>>();
            if digits.len() % 2 == 0 {
                let mut same = true;
                let len = digits.len();
                for k in 0..len / 2 {
                    if digits[k] == digits[digits.len() - 1 - k] {
                        continue;
                    } else {
                        same = false;
                        break;
                    }
                }
                if same {
                    biggest_palindrome = num;
                    done = true;
                    break;
                }
            }
        }
    }
    println!("{}", biggest_palindrome);
}