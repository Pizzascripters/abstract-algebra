use crate::util;
use crate::util::permutation;
use crate::util::permutation::Permutation;
use super::Group;

pub struct AlternatingGroup<const N: usize> {
    iterator: permutation::IterablePermutation<N>
}

impl<const N: usize> Group<Permutation<N>> for AlternatingGroup<N> {

    fn op(&self, a: Permutation<N>, b: Permutation<N>) -> Permutation<N> {
        return permutation::compose(a, b);
    }

    fn identity(&self) -> Permutation<N> {
        return util::arange();
    }

    fn inv(&self, g: Permutation<N>) -> Permutation<N> {
        return permutation::invert(g);
    }

    fn reset(&mut self) {
        self.iterator = permutation::IterablePermutation::new();
    }

    fn next(&mut self) -> Permutation<N> {
        if !self.iterator.is_even() {
            self.iterator.next();
        }
        return self.iterator.next();
    }

    fn order(&self) -> u32 {
        return util::factorial(N as u32) / 2;
    }
}

impl<'a, const N: usize> AlternatingGroup<N> {
    
    pub fn new() -> Self {
        return AlternatingGroup {
            iterator: permutation::IterablePermutation::new()
        }
    }
}