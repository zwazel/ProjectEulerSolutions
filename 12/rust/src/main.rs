use std::fmt::{Debug, Formatter};

fn main() {
    let amount_of_divisors = 500;
    let mut num = 0;

    let mut current_triangle_counter = 1;
    loop {
        let mut sum = 0;
        for i in 1..current_triangle_counter + 1 {
            sum += i;
        }

        let prime_factors = prime_factorization(sum);

        let mut divisors: Vec<i64> = vec![];


        let amount_divisors = divisors.len();
        println!("divisors: {}, sum: {}, triangle: {}", amount_divisors, sum, current_triangle_counter);
        if amount_divisors >= amount_of_divisors as usize {
            num = sum;
            break;
        }

        current_triangle_counter += 1;
    }

    println!("{}", num);
}

fn is_a_divisor(num: i32, divisor: i32) -> bool {
    num % divisor == 0
}

fn prime_factorization(mut num: i64) -> Vec<PrimeFactor> {
    let mut prime_factors: Vec<i64> = Vec::new();

    while num % 2 == 0 {
        prime_factors.push(2);
        num /= 2;
    }

    for i in 3..(num as f64).sqrt() as i64 + 1 {
        while num % i == 0 {
            prime_factors.push(i);
            num /= i;
        }
    }

    if num > 2 {
        prime_factors.push(num);
    }

    let mut prime_factors_factors: Vec<PrimeFactor> = Vec::new();
    for i in 0..prime_factors.len() {
        let prime_num = prime_factors[i];

        if prime_factors_factors.len() <= 0 {
            let prime_factor = PrimeFactor {
                num: prime_num,
                factor: 1,
            };
            prime_factors_factors.push(prime_factor);
            continue;
        }

        let mut found = false;
        for j in 0..prime_factors_factors.len() {
            let mut prime_factor_option = prime_factors_factors.get_mut(j);

            match prime_factor_option {
                None => {}
                Some(ref mut prime_factor) => {
                    if prime_factor.num == prime_num {
                        prime_factor.set_factor(prime_factor.factor + 1);
                        found = true;
                    }
                }
            }
        }
        if !found {
            prime_factors_factors.push(PrimeFactor {
                num: prime_num,
                factor: 1,
            })
        }
    }

    println!("{:?}", prime_factors_factors);
    prime_factors_factors
}

struct PrimeFactor {
    num: i64,
    factor: i64,
}

impl PrimeFactor {
    fn set_num(&mut self, num: i64) {
        self.num = num;
    }

    fn set_factor(&mut self, factor: i64) {
        self.factor = factor;
    }
}

impl Debug for PrimeFactor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimeFactor")
            .field("num", &self.num)
            .field("factor", &self.factor)
            .finish()
    }
}