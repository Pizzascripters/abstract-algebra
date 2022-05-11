pub fn swap<T: Copy, const N: usize>(a: &mut [T; N], i: usize, j: usize) -> &[T; N] {
    let temp = a[i];
    a[i] = a[j];
    a[j] = temp;
    return a;
}
