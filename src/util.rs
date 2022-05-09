pub fn factorial(n: u32) -> u32 {
    let mut n_factorial = 1;
    for i in 1..(n+1) {
        n_factorial *= i;
    }
    return n_factorial;
}