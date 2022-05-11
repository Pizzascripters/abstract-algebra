use crate::util;
use crate::util::permutation;
use permutation::Permutation;
use permutation::IterablePermutation;
use crate::util::factorial;
use super::Group;

pub struct SymmetricGroup<const N: usize> {
    members: Vec<Permutation<N>>
}

impl<'a, const N: usize> Group<Permutation<N>> for SymmetricGroup<N> {

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
        return factorial(N as u32) as usize;
    }

    fn index(&self, i: usize) -> Permutation<N> {
        return self.members[i];
    }
}

impl<const N: usize> SymmetricGroup<N> {
    
    pub fn new() -> Self {
        let iterator = IterablePermutation::new(false);
        return SymmetricGroup {
            members: iterator.collect()
        }
    }
}