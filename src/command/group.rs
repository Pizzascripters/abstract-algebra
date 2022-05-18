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

pub fn group(context: GroupContext) {
    match parse_group_id(&context.id) {
        Some((name, number)) => match number.parse::<usize>() {
            Ok(n) => format_group(&context, &name, n),
            Err(_e) => invalid_group(&context.id)
        },
        None => invalid_group(&context.id)
    }
}

fn parse_group_id(id: &String) -> Option<(String, String)> {
    let mut name = "".to_string();
    let mut number = "".to_string();
    for c in id.chars() {
        if number.len() == 0 && c.is_alphabetic() {
            name.push(c);
        } else if c.is_numeric() {
            number.push(c);
        } else {
            return None;
        }
    };
    Some((name, number))
}

fn format_group(context: &GroupContext, name: &String, n: usize) {
    match_group!(
        name.as_str(),
        |grp| {
            format!(
                "{}{}{}",
                if context.elements { format_members(grp) } else { "".to_owned() },
                if context.conjugacy_classes { format_conjugacy_classes(grp) } else { "".to_owned() },
                if context.center { format_center(grp) } else { "".to_owned() }
            )
        },
        invalid_group(&context.id),
        "A", &group::AlternatingGroup::new(n)
        "D", &group::DihedralGroup::new(n)
        "Q", &group::QuaternionGroup::new()
        "S", &group::SymmetricGroup::new(n)
        "Z", &group::CyclicGroup::new(n)
    )
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