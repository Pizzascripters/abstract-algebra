use crate::util;
use crate::util::permutation;
use crate::util::permutation::Permutation;
use crate::util::factorial;
use super::Group;

pub struct AlternatingGroup<const N: usize> {
    members: Vec<Permutation<N>>
}

impl<'a, const N: usize> Group<Permutation<N>> for AlternatingGroup<N> {

    fn op(&self, a: Permutation<N>, b: Permutation<N>) -> Permutation<N> {
        return permutation::compose(a, b);
    }

    fn identity(&self) -> Permutation<N> {
        return Permutation {
            s: util::arange()
        };
    }

    fn inv(&self, g: Permutation<N>) -> Permutation<N> {
        return permutation::invert(g);
    }

    fn order(&self) -> usize {
        return factorial(N as u32) as usize / 2;
    }

    fn index(&self, i: usize) -> Permutation<N> {
        return self.members[i];
    }
}

impl<'a, const N: usize> AlternatingGroup<N> {
    
    pub fn new() -> Self {
        let iterator = permutation::IterablePermutation::new(true);
        return AlternatingGroup {
            members: iterator.collect()
        }
    }
}