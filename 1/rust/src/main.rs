fn main() {
    let mut sum = 0;

    for i in 1..1000 {
        if i % 3 == 0 {
            println!("{}", i);
            sum += i;
        } else if i % 5 == 0 {
            println!("{}", i);
            sum += i;
        }
    }

    println!("Sum: {}", sum);
}
