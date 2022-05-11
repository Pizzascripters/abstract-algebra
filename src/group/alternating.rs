use crate::util;
use crate::util::permutation;
use permutation::Permutation;
use permutation::IterablePermutation;
use crate::util::factorial;
use super::Group;

pub struct AlternatingGroup {
    members: Vec<Permutation>,
    length: usize // Length of permutations
}

impl<'a> Group<Permutation> for AlternatingGroup {

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
        factorial(self.length as u32) as usize / 2
    }

    fn index(&self, i: usize) -> Permutation {
        self.members[i].clone()
    }
}

impl AlternatingGroup {
    
    pub fn new(length: usize) -> Self {
        let iterator = IterablePermutation::new(length, true);
        AlternatingGroup {
            members: iterator.collect(),
            length
        }
    }
}