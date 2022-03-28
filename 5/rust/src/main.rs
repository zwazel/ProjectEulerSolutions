fn main() {
    let min = 2;
    let max = 20;
    let mut done = false;

    let mut num = max;
    while !done {
        let mut good = true;
        for i in min..max + 1 {
            if num % i != 0 {
                good = false;
                break;
            }
        }

        if good {
            done = true;
        } else {
            num += 1;
        }
    }

    println!("{}", num);
}
