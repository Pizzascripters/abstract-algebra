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

    let mut s5: SymmetricGroup<5> = SymmetricGroup::new();
    for _ in 0..s5.order() {
        let g = s5.next();
        println!("({}, {}, {}, {}, {})", g[0], g[1], g[2], g[3], g[4]);
    }
}