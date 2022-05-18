use crate::util::format;
use super::parse_group_id;

pub fn conjugacy_classes(group_id: String) {
    parse_group_id!(&group_id, format::conjugacy_classes);
}