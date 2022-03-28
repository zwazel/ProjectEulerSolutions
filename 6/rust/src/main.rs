fn main() {
    let num = 100;
    let square_of_sum = square_of_sum(num);
    let sum_of_squares = sum_of_squares(num);

    println!("{}", square_of_sum - sum_of_squares);
}

fn square_of_sum(n: i32) -> i32 {
    let sum: i32 = (1..n + 1).sum();
    sum * sum
}

fn sum_of_squares(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n + 1 {
        sum += i * i;
    }
    sum
}