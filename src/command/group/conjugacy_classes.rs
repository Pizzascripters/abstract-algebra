use crate::util::format;
use super::parse_group_id;
use crate::group;

pub fn conjugacy_classes(group_id: String) {
    parse_group_id!(&group_id, format);
}

fn format<G: Clone + ToString + PartialEq>(grp: &dyn group::Group<G>) -> String {
    let classes = group::find_conjugacy_classes(grp);
    format!(
        "Conjugacy classes of {}:\n{}\n",
        grp.get_name(),
        classes.iter().map(|v| format::vec(v, ", ")).collect::<Vec<_>>().join("\n")
    )
}