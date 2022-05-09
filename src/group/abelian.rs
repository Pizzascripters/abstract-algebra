use std::collections::HashMap;

use super::Group;

pub struct AbelianGroup<'a> {
    // Orders of cyclic subgroups
    pub subgroups: &'a HashMap<u32, u8>,
    pub order: u32
}

impl<'a> AbelianGroup<'a> {
    
    pub fn new(subgroups: &'a HashMap<u32, u8>) -> Self {
        let mut order = 1;
        for (subgroup_order, multiplicity) in subgroups {
            order *= (*subgroup_order) * (*multiplicity as u32);
        }

        return AbelianGroup {
            subgroups: subgroups,
            order
        }
    }
}

impl<'a> Group<u32> for AbelianGroup<'a> {

    fn op(&self, mut a: u32, mut b: u32) -> u32 {
        let mut ab = 0;
        let mut x = 1;
        for (order, multiplicity) in self.subgroups {
            for _ in 1..(*multiplicity + 1) {
                ab += x * ((a + b) % *order);
                a = a / *order;
                b = b / *order;
                x *= *order;
            }
        }
        return ab;
    }

    fn identity(&self) -> u32 {
        return 1;
    }

    fn inv(&self, g: u32) -> u32 {
        return g;
    }

    fn order(&self) -> u32 {
        return self.order;
    }

    fn next(&mut self) -> u32 {
        return 0; // STUB
    }
}

// // Chinese remainder theorem
// fn crt(remainders: &[u32], primes: &[u32], powers: &[u32]) -> u32 {
//     let mut M = 1;
//     for i in 0..primes.len() {
//         M *= primes[i].pow(powers[i]);
//     }
//     let mut n = 0;
//     for i in 0..primes.len() {
//         let m = primes[i].pow(powers[i]);
//         let x = M / m;
//     }
//     return 1;
// }