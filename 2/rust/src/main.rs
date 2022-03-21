fn main() {
    let mut sum = 0;

    let mut num1 = 0;
    let mut num2 = 1;
    while num2 < 4_000_000 {
        let num = num1 + num2;
        num1 = num2;
        num2 = num;

        if num % 2 == 0 {
            sum += num;
        }
    }
    println!("sum: {}", sum);
}
