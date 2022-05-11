use crate::group;
use group::Group;

macro_rules! members {
    ($name: expr, $success: expr, $fail: expr, $($prefix: literal, $constructor: expr)*) => {
        match $name {
            $(
                $prefix => {
                    $success;
                    println!("{}", $constructor.to_string())
                },
            )*
            _ => { $fail }
        }
    };
}

pub fn members(group_id: String) {
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
            members!(
                name.as_str(),
                println!("Members of {}:", group_id),
                invalid_group(&group_id),
                "A", group::AlternatingGroup::new(n)
                "D", group::DihedralGroup::new(n)
                "Q", group::QuaternionGroup::new()
                "S", group::SymmetricGroup::new(n)
                "Z", group::CyclicGroup::new(n)
            );
        },
        Err(_e) => invalid_group(&group_id)
    };
}

fn invalid_group(group_id: &String) {
    println!("Inalid group id {}!", *group_id);
}