fn main() {
    let mut longest = (0, 0);

    for i in 1..1_000_000 {
        let current = get_collatz(i);

        if current.1 > longest.1 {
            longest = (i, current.1);
        }
    }

    println!("{} has a length of {}", longest.0, longest.1);
}

fn get_collatz(mut n: i64) -> (i64, i64) {
    let start = n;
    let mut length = 1;

    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        length += 1;
    }

    (start, length)
}