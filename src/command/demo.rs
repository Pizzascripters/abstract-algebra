use crate::group;
use group::Group;
use crate::util::format;

pub fn demo() {
    let z9 = group::CyclicGroup::new(9);
    let d6 = group::DihedralGroup::new(6);
    let q8 = group::QuaternionGroup::new();
    let s4: group::SymmetricGroup = group::SymmetricGroup::new(4);
    let a5: group::AlternatingGroup = group::AlternatingGroup::new(5);

    println!("abstract-algebra group Z9 --elements\n{}", z9.to_string());
    println!("abstract-algebra group S4 -e\n{}", s4.to_string());
    println!("abstract-algebra group D6 --center\n{}", format::center(&d6));
    println!("abstract-algebra group A5 --conjugacy-classes\n{}", format::conjugacy_classes(&a5));
    print!("abstract-algebra group Q8 -c\n{}", format::conjugacy_classes(&q8));
}