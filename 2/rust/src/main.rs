fn main() {
    let mut sum = 2;
    let mut numbers = vec![1, 2];
    let mut num = 0;
    while num < 4_000_000 {
        let num1 = numbers.get(numbers.len() - 2);
        let num2 = numbers.get(numbers.len() - 1);
        if num1.is_none() || num2.is_none() {
            eprintln!("Error: numbers.get(numbers.len() - 2) or numbers.get(numbers.len() - 1) is None");
            break;
        } else {
            num = num1.unwrap() + num2.unwrap();
            numbers.push(num);

            if num & 2 == 0 {
                sum += num;
            }
            println!("num: {}", num);
        }
    }
    println!("sum: {}", sum);
}
