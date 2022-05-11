use crate::util::format;
use crate::group;

macro_rules! center {
    ($name: expr, $success: expr, $fail: expr, $($prefix: literal, $constructor: expr)*) => {
        match $name {
            $(
                $prefix => {
                    $success;
                    println!("{}", format::conjugacy_classes($constructor))
                },
            )*
            _ => { $fail }
        }
    };
}

pub fn conjugacy_classes(group_id: String) {
    let mut name = "".to_string();
    let mut number = "".to_string();
    for c in group_id.chars() {
        if number.len() == 0 && c.is_alphabetic() {
            name.push(c);
        } else if c.is_numeric() {
            number.push(c);
        } else {
            invalid_group(&group_id);
            return;
        }
    };

    match number.parse::<usize>() {
        Ok(n) => {
            center!(
                name.as_str(),
                println!("Conjugacy classes of {}:", group_id),
                invalid_group(&group_id),
                "A", &group::AlternatingGroup::new(n)
                "D", &group::DihedralGroup::new(n)
                "Q", &group::QuaternionGroup::new()
                "S", &group::SymmetricGroup::new(n)
                "Z", &group::CyclicGroup::new(n)
            );
        },
        Err(_e) => invalid_group(&group_id)
    };
}

fn invalid_group(group_id: &String) {
    println!("Inalid group id {}!", *group_id);
}