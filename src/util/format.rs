use crate::group;
use group::Group;

pub fn vec<T: ToString>(v: &Vec<T>, delim: &str) -> String {
    return format!(
        "[{} {}] {{{}}}",
        v.len(),
        if v.len() == 1 { "element" } else { "elements" },
        v.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(delim)
    );
}

pub fn center<G: ?Sized + Copy + ToString + PartialEq>(grp: &dyn Group<G>) -> String {
    return vec(&group::find_center(grp), ", ");
}

pub fn conjugacy_classes<G: ?Sized + Copy + ToString + PartialEq>(grp: &dyn group::Group<G>) -> String {
    let classes = group::find_conjugacy_classes(grp);
    return classes.iter().map(|v| vec(v, ", ")).collect::<Vec<_>>().join("\n");
}