pub mod format;
pub mod permutation;

pub fn arange<const N: usize>() -> [usize; N] {
    let mut a: [usize; N] = [0; N];
    for i in 0..N {
        a[i] = i;
    }
    return a;
}

pub fn swap<T: Copy, const N: usize>(a: &mut [T; N], i: usize, j: usize) -> &[T; N] {
    let temp = a[i];
    a[i] = a[j];
    a[j] = temp;
    return a;
}

pub fn factorial(n: u32) -> u32 {
    let mut n_factorial = 1;
    for i in 1..(n+1) {
        n_factorial *= i;
    }
    return n_factorial;
}