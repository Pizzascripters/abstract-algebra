use super::arange;
use super::swap;

// If s is a permutation then for any i in 0..N there should exist a j such that s[j] = i
// Alternatively 0 <= s[i] < N and each s[i] is unique
pub type Permutation<const N: usize> = [usize;N];

pub struct IterablePermutation<const N: usize> {
    s: Permutation<N>,
    directions: [i8; N],
    is_even: bool
}

impl<const N: usize> IterablePermutation<N> {
    pub fn new() -> Self {
        let mut directions: [i8; N] = [-1; N];
        directions[0] = 0;
        return IterablePermutation {
            s: arange(),
            directions,
            is_even: true
        }
    }

    // Steinhaus–Johnson–Trotter algorithm with Even's speedup
    pub fn next(&mut self) -> Permutation<N> {
        let permutation_to_return = self.s;

        let mut k: usize = 0; // Index of greatest nonzero element
        for i in 0..N {
            if self.directions[i] != 0 && (self.s[i] >= self.s[k] || self.directions[k] == 0) {
                k = i;
            }
        }

        // Make swap
        let k_ = k;
        if self.directions[k] == 1 {
            swap(&mut self.s, k, k+1);
            swap(&mut self.directions, k, k+1);
            k += 1;
        } else if self.directions[k] == -1 {
            swap(&mut self.s, k, k-1);
            swap(&mut self.directions, k, k-1);
            k -= 1;
        }

        if k == 0 || k == N-1 || self.s[2*k-k_] > self.s[k] {
            self.directions[k] = 0;
        }

        for i in 0..N {
            if self.s[i] > self.s[k] {
                if k < i {
                    self.directions[i] = -1;
                } else {
                    self.directions[i] = 1;
                }
            }
        }

        if permutation_to_return == self.s {
            self.s = arange();
        }

        self.is_even = !self.is_even;

        return permutation_to_return;
    }

    pub fn is_even(&self) -> bool {
        return self.is_even;
    }
}

pub fn compose<const N: usize>(s1: Permutation<N>, s2: Permutation<N>) -> Permutation<N> {
    let mut composition: Permutation<N> = [0;N];
    for i in 0..N {
        composition[i] = s1[s2[i]];
    }
    return composition;
}

pub fn invert<const N: usize>(s: Permutation<N>) -> Permutation<N> {
    let mut inverse: Permutation<N> = [0;N];
    for i in 0..N {
        inverse[s[i]] = i;
    }
    return inverse;
}