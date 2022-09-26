pub fn factors(mut n: u64) -> Vec<u64> {
    // unimplemented!("This should calculate the prime factors of {}", n)
    // let mut n:
    let mut divisor: u64 = 2;
    let mut factors = Vec::new();

    while n != 1 {
        if n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        } else {
            // divisor = divisor + 1;
            divisor += 1;
        }
    }
    factors
}
