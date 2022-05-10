mod group;
mod action;
mod util;

use group::Group;
use group::cyclic::CyclicGroup;
use group::dihedral::DihedralGroup;
use group::quaternion::QuaternionGroup;
use group::symmetric::SymmetricGroup;
use group::alternating::AlternatingGroup;
use action::conjugate::Conjugate;
use util::permutation::Permutation;
use util::format;

fn main() {
    let z9 = CyclicGroup::new(9);
    let d6 = DihedralGroup::new(6);
    let q8 = QuaternionGroup::new();
    let s4: SymmetricGroup<4> = SymmetricGroup::new();
    let a4: AlternatingGroup<4> = AlternatingGroup::new();
    let a5: AlternatingGroup<5> = AlternatingGroup::new();

    println!("Members of Z9:\n{}\n", z9.to_string());
    println!("Members of A4:\n{}\n", a4.to_string());

    let conjugacy_s4: Conjugate<Permutation<4>> = Conjugate::new(&s4);
    let orbit_s4 = action::orbit(&SymmetricGroup::new(), &conjugacy_s4, Permutation {s:[0, 3, 2, 1]});
    println!(
        "Orbit of (3, 2, 1, 0) in S4 acting on itself under conjugation:\n{}\n",
        format::vec(&orbit_s4, ", ")
    );

    println!(
        "Center of D6 (hexagon symmetries):\n{}\n",
        format::vec(&group::find_center(&d6), ", ")
    );

    println!("Conjugacy classes of A5:\n{}\n", group::format_conjugacy_classes(&a5));
    println!("Conjugacy classes of Q8:\n{}\n", group::format_conjugacy_classes(&q8));
}