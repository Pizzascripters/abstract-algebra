use crate::util::format;
use crate::group;
use crate::cli::GroupContext;

macro_rules! match_group {
    ($name: expr, $format: expr, $fail: expr, $($prefix: literal, $constructor: expr)*) => {
        match $name {
            $(
                $prefix => { print!("{}", $format($constructor)) },
            )*
            _ => { $fail }
        }
    };
}

macro_rules! parse_group_id {
    ($group_id: expr, $on_success: expr) => {
        use crate::group;

        let mut name = "".to_string();
        let mut number = "".to_string();
        for c in $group_id.chars() {
            if number.len() == 0 && c.is_alphabetic() {
                name.push(c);
            } else if c.is_numeric() {
                number.push(c);
            } else {
                invalid_group($group_id);
                return;
            }
        };
    
        match number.parse::<usize>() {
            Ok(n) => {
                match_group!(
                    name.as_str(),
                    $on_success,
                    invalid_group($group_id),
                    "A", &group::AlternatingGroup::new(n)
                    "D", &group::DihedralGroup::new(n)
                    "Q", &group::QuaternionGroup::new()
                    "S", &group::SymmetricGroup::new(n)
                    "Z", &group::CyclicGroup::new(n)
                );
            },
            Err(_e) => invalid_group($group_id)
        };
    }
}

pub fn group(context: GroupContext) {
    parse_group_id!(&context.id, |grp| {
        format!(
            "{}{}{}",
            if context.elements { format_members(grp) } else { "".to_owned() },
            if context.conjugacy_classes { format_conjugacy_classes(grp) } else { "".to_owned() },
            if context.center { format_center(grp) } else { "".to_owned() }
        )
    });
}

pub fn members(group_id: String) {
    parse_group_id!(&group_id, format_members);
}

pub fn conjugacy_classes(group_id: String) {
    parse_group_id!(&group_id, format_conjugacy_classes);
}

pub fn center(group_id: String) {
    parse_group_id!(&group_id, format_center);
}

fn format_members<G: Clone + ToString + PartialEq>(grp: &impl group::Group<G>) -> String {
    format!(
        "Elements of {}:\n{}\n\n",
        grp.get_name(),
        grp.to_string()
    )
}

fn format_conjugacy_classes<G: Clone + ToString + PartialEq>(grp: &impl group::Group<G>) -> String {
    let classes = group::find_conjugacy_classes(grp);
    format!(
        "Conjugacy classes of {}:\n{}\n\n",
        grp.get_name(),
        classes.iter().map(|v| format::vec(v, ", ")).collect::<Vec<_>>().join("\n")
    )
}

fn format_center<G: Clone + ToString + PartialEq>(grp: &impl group::Group<G>) -> String {
    format!(
        "Center of {}:\n{}\n\n",
        grp.get_name(),
        format::vec(&group::find_center(grp), ", ")
    )
}

pub fn invalid_group(group_id: &String) {
    println!("Invalid group id {}!", *group_id);
    println!("Valid group ids are A<n>, D<n>, Q8, S<n>, and Z<n>");
}