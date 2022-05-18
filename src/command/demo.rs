use crate::command;
use crate::cli::GroupContext;

pub fn demo() {
    println!("abstract-algebra group Z9 --elements");
    command::group(GroupContext {
        id: "Z9".to_owned(),
        center: false,
        conjugacy_classes: false,
        elements: true
    });

    println!("abstract-algebra group S4 -e");
    command::group(GroupContext {
        id: "S4".to_owned(),
        center: false,
        conjugacy_classes: false,
        elements: true
    });

    println!("abstract-algebra group D6 --center");
    command::group(GroupContext {
        id: "D6".to_owned(),
        center: true,
        conjugacy_classes: false,
        elements: false
    });

    println!("abstract-algebra group A5 --conjugacy-classes");
    command::group(GroupContext {
        id: "A5".to_owned(),
        center: false,
        conjugacy_classes: true,
        elements: false
    });

    println!("abstract-algebra group Q8 -c");
    command::group(GroupContext {
        id: "Q8".to_owned(),
        center: false,
        conjugacy_classes: true,
        elements: false
    });
}