pub fn arange(length: usize) -> Vec<usize> {
    let mut a: Vec<usize> = vec!(0; length);
    for i in 0..length {
        a[i] = i;
    }
    a
}