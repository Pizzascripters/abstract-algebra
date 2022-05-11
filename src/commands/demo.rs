use crate::group;
use crate::group::Group;
use crate::util::format;

pub fn demo() {
    let z9 = group::cyclic::CyclicGroup::new(9);
    let d6 = group::dihedral::DihedralGroup::new(6);
    let q8 = group::quaternion::QuaternionGroup::new();
    let s4: group::symmetric::SymmetricGroup<4> = group::symmetric::SymmetricGroup::new();
    let a5: group::alternating::AlternatingGroup<5> = group::alternating::AlternatingGroup::new();

    println!("Members of Z9:\n{}\n", z9.to_string());
    println!("Members of S4:\n{}\n", s4.to_string());
    println!("Center of D6:\n{}\n", format::center(&d6));
    println!("Conjugacy classes of A5:\n{}\n", format::conjugacy_classes(&a5));
    println!("Conjugacy classes of Q8:\n{}\n", format::conjugacy_classes(&q8));
}