mod group;
mod action;
mod util;

use group::Group;
use group::cyclic::CyclicGroup;
use group::symmetric::SymmetricGroup;
use group::alternating::AlternatingGroup;
use action::conjugate::Conjugate;
use util::permutation::Permutation;
use util::format;

fn main() {
    let z9 = CyclicGroup::new(9);
    let s4: SymmetricGroup<4> = SymmetricGroup::new();
    let a4: AlternatingGroup<4> = AlternatingGroup::new();
    let a5: SymmetricGroup<5> = SymmetricGroup::new();

    println!("Members of Z9:\n{}\n", z9.to_string());
    println!("Members of A4:\n{}\n", a4.to_string());

    let conjugacy_action: Conjugate<Permutation<4>> = Conjugate::new(&s4);
    let orbit = action::orbit(&SymmetricGroup::new(), &conjugacy_action, Permutation {s:[0, 3, 2, 1]});
    println!(
        "Orbit of (3, 2, 1, 0) in S4 acting on itself under conjugation:\n{}\n",
        format::vec(&orbit, ", ")
    );

    println!(
        "Center of S4:\n{}\n",
        format::vec(&group::find_center(&s4), ", ")
    );

    let classes = group::find_conjugacy_classes(&a5);
    println!(
        "Conjugacy classes of A5:\n{}",
        classes.iter().map(|v| format::vec(v, ", ")).collect::<Vec<_>>().join("\n")
    );
}