use crate::group;
use group::Group;
use crate::util::format;

pub fn demo() {
    let z9 = group::CyclicGroup::new(9);
    let d6 = group::DihedralGroup::new(6);
    let q8 = group::QuaternionGroup::new();
    let s4: group::SymmetricGroup<4> = group::SymmetricGroup::new();
    let a5: group::AlternatingGroup<5> = group::AlternatingGroup::new();

    println!("Members of Z9:\n{}\n", z9.to_string());
    println!("Members of S4:\n{}\n", s4.to_string());
    println!("Center of D6:\n{}\n", format::center(&d6));
    println!("Conjugacy classes of A5:\n{}\n", format::conjugacy_classes(&a5));
    println!("Conjugacy classes of Q8:\n{}\n", format::conjugacy_classes(&q8));
}