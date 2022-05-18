use super::parse_group_id;
use crate::group;

pub fn members(group_id: String) {
    parse_group_id!(&group_id, format);
}

fn format<G: Clone + ToString + PartialEq>(grp: &dyn group::Group<G>) -> String {
    format!(
        "Elements of {}:\n{}\n",
        grp.get_name(),
        grp.to_string()
    )
}