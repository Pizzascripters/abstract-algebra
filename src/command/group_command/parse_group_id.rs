macro_rules! match_group {
    ($name: expr, $format: expr, $fail: expr, $($prefix: literal, $constructor: expr)*) => {
        match $name {
            $(
                $prefix => { println!("{}", $format($constructor)) },
            )*
            _ => { $fail }
        }
    };
}
pub(crate) use match_group;

macro_rules! parse_group_id {
    ($group_id: expr, $on_success: expr) => {
        use crate::group;
        use parse_group_id::match_group;
        use parse_group_id::invalid_group;

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
pub(crate) use parse_group_id;

pub fn invalid_group(group_id: &String) {
    println!("Invalid group id {}!", *group_id);
    println!("Valid group ids are A<n>, D<n>, Q8, S<n>, and Z<n>");
}