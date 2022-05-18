use crate::util::format;
use super::parse_group_id;
use crate::group;

pub fn center(group_id: String) {
    parse_group_id!(&group_id, format);
}

fn format<G: Clone + ToString + PartialEq>(grp: &dyn group::Group<G>) -> String {
    format!(
        "Center of {}:\n{}\n",
        grp.get_name(),
        format::vec(&group::find_center(grp), ", ")
    )
}