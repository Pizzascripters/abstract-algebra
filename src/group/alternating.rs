use std::ops::Index;
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
}

impl<const N: usize> Index<usize> for AlternatingGroup<N> {

    type Output = Permutation<N>;

    fn index(&self, i: usize) -> &Self::Output {
        return &self.members[i];
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