use super::arange;
use super::swap;

// If s is a permutation then for any i in 0..N there should exist a j such that s[j] = i
// Alternatively 0 <= s[i] < N and each s[i] is unique
pub type Permutation<const N: usize> = [usize;N];

pub struct IterablePermutation<const N: usize> {
    s: Permutation<N>,
    directions: [i8; N],
    even_only: bool,
    is_even: bool,
    done: bool
}

impl<const N: usize> Iterator for IterablePermutation<N> {
    type Item = Permutation<N>;

    // Steinhaus–Johnson–Trotter algorithm with Even's speedup
    fn next(&mut self) -> Option<Self::Item> {
        if self.even_only && !self.is_even {
            self.advance();
        }
        return self.advance();
    }
}

impl<const N: usize> IterablePermutation<N> {
    pub fn new(even_only: bool) -> Self {
        let mut directions: [i8; N] = [-1; N];
        directions[0] = 0;
        return IterablePermutation {
            s: arange(),
            directions,
            even_only,
            is_even: true,
            done: false
        }
    }

    pub fn advance(&mut self) -> Option<Permutation<N>> {
        if self.done {
            return Option::None;
        }

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
            self.done = true;
            return Option::None;
        }

        self.is_even = !self.is_even;

        return Some(permutation_to_return);
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