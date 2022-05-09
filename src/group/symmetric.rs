use crate::util::factorial;
use crate::util::arange;
use crate::util::swap;
use super::Group;

// If s is a permutation then for any i in 0..N there should exist a j such that s[j] = i
// Alternatively 0 <= s[i] < N and each s[i] is unique
type Permutation<const N: usize> = [usize;N];

pub struct SymmetricGroup<const N: usize> {
    // For iterating through permutations
    member: Permutation<N>,
    directions: [i8; N]
}

impl<const N: usize> Group<Permutation<N>> for SymmetricGroup<N> {

    // Permutation multiplication
    fn op(&self, a: Permutation<N>, b: Permutation<N>) -> Permutation<N> {
        let mut ab: Permutation<N> = [0;N];
        for i in 0..N {
            ab[i] = a[b[i]];
        }
        return ab;
    }

    // Identity permutation e defined by e[i] = i
    fn identity(&self) -> Permutation<N> {
        return arange();
    }

    fn inv(&self, g: Permutation<N>) -> Permutation<N> {
        let mut g_inv: Permutation<N> = [0;N];
        for i in 0..N {
            g_inv[g[i]] = i;
        }
        return g_inv;
    }

    // Steinhaus–Johnson–Trotter algorithm with Even's speedup
    fn next(&mut self) -> Permutation<N> {
        let permutation_to_return = self.member;

        let mut k: usize = 0; // Index of greatest nonzero element
        for i in 0..N {
            if self.directions[i] != 0 && (self.member[i] >= self.member[k] || self.directions[k] == 0) {
                k = i;
            }
        }

        // Make swap
        let k_ = k;
        if self.directions[k] == 1 {
            swap(&mut self.member, k, k+1);
            swap(&mut self.directions, k, k+1);
            k += 1;
        } else if self.directions[k] == -1 {
            swap(&mut self.member, k, k-1);
            swap(&mut self.directions, k, k-1);
            k -= 1;
        }

        if k == 0 || k == N-1 || self.member[2*k-k_] > self.member[k] {
            self.directions[k] = 0;
        }

        for i in 0..N {
            if self.member[i] > self.member[k] {
                if k < i {
                    self.directions[i] = -1;
                } else {
                    self.directions[i] = 1;
                }
            }
        }

        if permutation_to_return == self.member {
            self.member = self.identity();
        }

        return permutation_to_return;
    }

    fn order(&self) -> u32 {
        return factorial(N as u32);
    }
}

impl<'a, const N: usize> SymmetricGroup<N> {
    
    pub fn new() -> Self {
        let mut directions: [i8; N] = [-1; N];
        directions[0] = 0;
        return SymmetricGroup {
            member: arange(),
            directions
        }
    }
}