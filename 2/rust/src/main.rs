fn main() {
    let mut sum = 0;

    let mut num1 = 1;
    let mut num2 = 2;
    let mut num = 0;
    while num < 100 {
        num = num1 + num2;
        num1 = num2;
        num2 = num;

        sum += num;
        println!("num: {}", num);
    }
}
