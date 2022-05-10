use crate::util;
use crate::util::permutation;
use crate::util::permutation::Permutation;
use super::Group;

pub struct SymmetricGroup<const N: usize> {
    iterator: permutation::IterablePermutation<N>
}

impl<'a, const N: usize> Group<Permutation<N>> for SymmetricGroup<N> {

    fn op(&self, a: Permutation<N>, b: Permutation<N>) -> Permutation<N> {
        return permutation::compose(a, b);
    }

    fn identity(&self) -> Permutation<N> {
        return util::arange();
    }

    fn inv(&self, g: Permutation<N>) -> Permutation<N> {
        return permutation::invert(g);
    }
}

impl<const N: usize> Iterator for SymmetricGroup<N> {

    type Item = Permutation<N>;

    fn next(&mut self) -> Option<Permutation<N>> {
        return self.iterator.next();
    }
}

impl<const N: usize> SymmetricGroup<N> {
    
    pub fn new() -> Self {
        let iterator = permutation::IterablePermutation::new(false);
        return SymmetricGroup {
            iterator
        }
    }
}