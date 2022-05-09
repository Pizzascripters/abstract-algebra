mod group;

use std::collections::HashMap;

use group::Group;

fn main() {
    let subgroups: HashMap<u32, u8> = HashMap::from([(5, 1), (4, 1)]);
    let z20 = group::AbelianGroup::new(&subgroups);
    println!("{}", z20.op(1, 5));
}