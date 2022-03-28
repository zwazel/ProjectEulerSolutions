fn main() {
    let s = 1000;

    for a in 1..s {
        for b in a..s {
            let c = s - a - b;
            if a * a + b * b == c * c {
                println!("{}", a * b * c);
                return;
            }
        }
    }
}
