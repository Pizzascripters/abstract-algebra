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

fn main() {
    let subgroups: HashMap<u32, u8> = HashMap::from([(5, 1), (4, 1)]);
    let z20 = AbelianGroup::new(&subgroups);
    println!("{}", z20.op(1, 5));

    println!("Members of S4:");
    let mut s4: SymmetricGroup<4> = SymmetricGroup::new();
    for _ in 0..s4.order() {
        let g = s4.next();
        print_permutation_4(g);
        print!(" - ");
    }
    print!("\n");

    println!("Members of A4:");
    let mut a4: AlternatingGroup<4> = AlternatingGroup::new();
    for _ in 0..a4.order() {
        let g = a4.next();
        print_permutation_4(g);
        print!(" - ");
    }
    print!("\n");

    let action: Conjugate<permutation::Permutation<4>> = Conjugate::new(&mut s4);
    print_permutation_4(action.op([3, 2, 1, 0], [1, 3, 2, 0]));
}

fn print_permutation_4(g: [usize; 4]) {
    print!("({}, {}, {}, {})", g[0], g[1], g[2], g[3]);
}