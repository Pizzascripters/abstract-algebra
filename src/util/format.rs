pub fn vec<T: ToString>(v: &Vec<T>, delim: &str) -> String {
    return format!(
        "[{} {}] {{{}}}",
        v.len(),
        if v.len() == 1 { "element" } else { "elements" },
        v.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(delim)
    );
}