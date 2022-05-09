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

    let s5: SymmetricGroup<5> = SymmetricGroup::new();
    println!("{}", s5.order());
}