use crate::util;
use crate::util::permutation;
use crate::util::permutation::Permutation;
use super::Group;

pub struct SymmetricGroup<const N: usize> {
    iterator: permutation::IterablePermutation<N>
}

impl<const N: usize> Group<Permutation<N>> for SymmetricGroup<N> {

    // Permutation multiplication
    fn op(&self, a: Permutation<N>, b: Permutation<N>) -> Permutation<N> {
        return permutation::compose(a, b);
    }

    // Identity permutation e defined by e[i] = i
    fn identity(&self) -> Permutation<N> {
        return util::arange();
    }

    fn inv(&self, g: Permutation<N>) -> Permutation<N> {
        return permutation::invert(g);
    }

    // Steinhaus–Johnson–Trotter algorithm with Even's speedup
    fn next(&mut self) -> Permutation<N> {
        return self.iterator.next();
    }

    fn order(&self) -> u32 {
        return util::factorial(N as u32);
    }
}

impl<'a, const N: usize> SymmetricGroup<N> {
    
    pub fn new() -> Self {
        return SymmetricGroup {
            iterator: permutation::IterablePermutation::new()
        }
    }
}