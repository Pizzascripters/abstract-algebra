use crate::util;
use crate::util::permutation;
use permutation::Permutation;
use permutation::IterablePermutation;
use crate::util::factorial;
use super::Group;

pub struct SymmetricGroup {
    members: Vec<Permutation>,
    length: usize // Length of permutations
}

impl<'a> Group<Permutation> for SymmetricGroup {

    fn op(&self, a: Permutation, b: Permutation) -> Permutation {
        permutation::compose(a, b)
    }

    fn identity(&self) -> Permutation {
        Permutation {
            s: util::arange(self.length),
            length: self.length
        }
    }

    fn inv(&self, g: Permutation) -> Permutation {
        permutation::invert(g)
    }

    fn order(&self) -> usize {
        factorial(self.length as u32) as usize
    }

    fn index(&self, i: usize) -> Permutation {
        self.members[i].clone()
    }
}

impl SymmetricGroup {
    
    pub fn new(length: usize) -> Self {
        let iterator = IterablePermutation::new(length, false);
        SymmetricGroup {
            members: iterator.collect(),
            length
        }
    }
}