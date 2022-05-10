mod group;
mod action;
mod util;

use std::collections::HashMap;

use group::Group;
use group::symmetric::SymmetricGroup;
use group::abelian::AbelianGroup;
use group::alternating::AlternatingGroup;
use action::conjugate::Conjugate;
use util::permutation::Permutation;

fn main() {
    let subgroups: HashMap<u32, u8> = HashMap::from([(5, 1), (4, 1)]);
    let z20 = AbelianGroup::new(&subgroups);
    println!("{}", z20.op(1, 5));

    println!("Members of S4:");
    let mut s4: SymmetricGroup<4> = SymmetricGroup::new();
    print_permutation_group(&mut s4);
    print!("\n\n");

    println!("Members of A4:");
    let mut a4: AlternatingGroup<4> = AlternatingGroup::new();
    print_permutation_group(&mut a4);
    print!("\n\n");

    println!("Orbit of [3, 2, 1, 0] in S4 acting on itself under conjugation:");
    let action: Conjugate<Permutation<4>> = Conjugate::new(&s4);
    let orbit = action::orbit(&SymmetricGroup::new(), &action, [0, 3, 2, 1]);
    for s in orbit {
        print_permutation_4(s);
        print!(" - ");
    }
    print!("\n\n");

    println!("Center of S4:");
    let center = group::find_center(&s4);
    for g in center {
        print_permutation_4(g);
        print!(" - ");
    }
    print!("\n\n");

    println!("Conjugacy classes of A4:");
    let classes = group::find_conjugacy_classes(&a4);
    for class in classes {
        for s in class {
            print_permutation_4(s);
            print!(" - ");
        }
        print!("\n");
    }
    print!("\n");
}

fn print_permutation_group(grp: &mut dyn Group<Permutation<4>>) {
    for i in 0..grp.order() {
        let g = grp[i];
        print_permutation_4(g);
        print!(" - ");
    }
}

fn print_permutation_4(g: [usize; 4]) {
    print!("({}, {}, {}, {})", g[0], g[1], g[2], g[3]);
}