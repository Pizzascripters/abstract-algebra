pub fn swap<T: Copy>(a: &mut Vec<T>, i: usize, j: usize) -> &mut Vec<T> {
    let temp = a[i];
    a[i] = a[j];
    a[j] = temp;
    a
}
