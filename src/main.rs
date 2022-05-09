mod group;
mod util;

use std::collections::HashMap;

use group::Group;
use group::symmetric::SymmetricGroup;
use group::abelian::AbelianGroup;

fn main() {
    let subgroups: HashMap<u32, u8> = HashMap::from([(5, 1), (4, 1)]);
    let z20 = AbelianGroup::new(&subgroups);
    println!("{}", z20.op(1, 5));

    println!("Members of S5:");
    let mut s5: SymmetricGroup<5> = SymmetricGroup::new();
    for _ in 0..s5.order() {
        let g = s5.next();
        print_permutation_5(g);
        print!(" - ");
    }
    print!("\n");

    let g1 = [2,1,3,0,4];
    let g2 = [2,1,3,0,4];
    print!("The product of ");
    print_permutation_5(g1);
    print!(" and ");
    print_permutation_5(g2);
    print!(" is ");
    print_permutation_5(s5.op(g1, g2));
    print!("\n");
}

fn print_permutation_5(g: [usize; 5]) {
    print!("({}, {}, {}, {}, {})", g[0], g[1], g[2], g[3], g[4]);
}