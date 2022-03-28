fn main() {
    let mut biggest_palindrome = 0;
    for i in 10..100 {
        for j in 10..100 {
            let num = i * j;
            let mut digits = num.to_string().chars().collect::<Vec<char>>();
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
                    if num > biggest_palindrome {
                        biggest_palindrome = num;
                        println!("{}", num);
                    }
                }
            }
        }
    }
}