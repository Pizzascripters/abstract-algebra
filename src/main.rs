mod group;
mod action;
mod util;

use group::Group;
use group::cyclic::CyclicGroup;
use group::dihedral::DihedralGroup;
use group::quaternion::QuaternionGroup;
use group::symmetric::SymmetricGroup;
use group::alternating::AlternatingGroup;
use util::format;

fn main() {
    let z9 = CyclicGroup::new(9);
    let d6 = DihedralGroup::new(6);
    let q8 = QuaternionGroup::new();
    let s4: SymmetricGroup<4> = SymmetricGroup::new();
    let a5: AlternatingGroup<5> = AlternatingGroup::new();

    println!("Members of Z9:\n{}\n", z9.to_string());
    println!("Members of S4:\n{}\n", s4.to_string());
    println!("Center of D6:\n{}\n", format::center(&d6));
    println!("Conjugacy classes of A5:\n{}\n", format::conjugacy_classes(&a5));
    println!("Conjugacy classes of Q8:\n{}\n", format::conjugacy_classes(&q8));
}