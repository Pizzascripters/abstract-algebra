pub fn arange<const N: usize>() -> [usize; N] {
    let mut a: [usize; N] = [0; N];
    for i in 0..N {
        a[i] = i;
    }
    return a;
}