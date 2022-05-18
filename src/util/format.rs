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

pub fn members<G: Clone + ToString + PartialEq>(grp: &dyn Group<G>) -> String {
    grp.to_string()
}

pub fn center<G: Clone + ToString + PartialEq>(grp: &dyn Group<G>) -> String {
    format!(
        "Center of {}:\n{}",
        grp.get_name(),
        vec(&group::find_center(grp), ", ")
    )
}

pub fn conjugacy_classes<G: Clone + ToString + PartialEq>(grp: &dyn group::Group<G>) -> String {
    let classes = group::find_conjugacy_classes(grp);
    format!(
        "Conjugacy classes of {}:\n{}",
        grp.get_name(),
        classes.iter().map(|v| vec(v, ", ")).collect::<Vec<_>>().join("\n")
    )
}