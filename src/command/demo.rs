use crate::command;

pub fn demo() {
    println!("abstract-algebra group Z9 --elements");
    command::members("Z9".to_owned());

    println!("abstract-algebra group S4 -e");
    command::members("S4".to_owned());

    println!("abstract-algebra group D6 --center");
    command::center("D6".to_owned());

    println!("abstract-algebra group A5 --conjugacy-classes");
    command::conjugacy_classes("A5".to_owned());

    println!("abstract-algebra group Q8 -c");
    command::conjugacy_classes("Q8".to_owned());
}