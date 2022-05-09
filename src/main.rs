mod group;
mod action;
mod util;

use std::collections::HashMap;

use group::Group;
use group::symmetric::SymmetricGroup;
use group::abelian::AbelianGroup;
use group::alternating::AlternatingGroup;
use action::Action;
use action::conjugate::Conjugate;
use util::permutation;
use util::permutation::Permutation;

fn main() {
    let subgroups: HashMap<u32, u8> = HashMap::from([(5, 1), (4, 1)]);
    let z20 = AbelianGroup::new(&subgroups);
    println!("{}", z20.op(1, 5));

    println!("Members of S4:");
    let mut s4: SymmetricGroup<4> = SymmetricGroup::new();
    print_permutation_group(&mut s4);
    print!("\n");

    println!("Members of A4:");
    let mut a4: AlternatingGroup<4> = AlternatingGroup::new();
    print_permutation_group(&mut a4);
    print!("\n");

    let action: Conjugate<permutation::Permutation<4>> = Conjugate::new(&mut s4);
    print_permutation_4(action.op([3, 2, 1, 0], [1, 3, 2, 0]));
}

fn print_permutation_group(grp: &mut dyn Group<Permutation<4>, Item=Permutation<4>>) {
    for g in grp {
        print_permutation_4(g);
        print!(" - ");
    }
}

fn print_permutation_4(g: [usize; 4]) {
    print!("({}, {}, {}, {})", g[0], g[1], g[2], g[3]);
}