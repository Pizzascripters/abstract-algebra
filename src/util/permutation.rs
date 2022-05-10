use super::arange;
use super::swap;

// If s is a permutation then for any i in 0..N there should exist a j such that s[j] = i
// Alternatively 0 <= s[i] < N and each s[i] is unique
pub struct Permutation<const N: usize> {
    pub s: [usize; N]
}

impl<const N: usize> ToString for Permutation<N> {
    fn to_string(&self) -> String {
        return format!("({})", self.s.map(|i| i.to_string()).join(", "));
    }
}

impl<const N: usize> PartialEq for Permutation<N> {
    fn eq(&self, other: &Self) -> bool {
        return self.s == other.s;
    }
}

impl<const N: usize> Copy for Permutation<N> {}
impl<const N: usize> Clone for Permutation<N> {
    fn clone(&self) -> Self {
        return Permutation {
            s: self.s
        }
    }
}

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
            s: Permutation {
                s: arange()
            },
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
            if self.directions[i] != 0 && (self.s.s[i] >= self.s.s[k] || self.directions[k] == 0) {
                k = i;
            }
        }

        // Make swap
        let k_ = k;
        if self.directions[k] == 1 {
            swap(&mut self.s.s, k, k+1);
            swap(&mut self.directions, k, k+1);
            k += 1;
        } else if self.directions[k] == -1 {
            swap(&mut self.s.s, k, k-1);
            swap(&mut self.directions, k, k-1);
            k -= 1;
        }

        if k == 0 || k == N-1 || self.s.s[2*k-k_] > self.s.s[k] {
            self.directions[k] = 0;
        }

        for i in 0..N {
            if self.s.s[i] > self.s.s[k] {
                if k < i {
                    self.directions[i] = -1;
                } else {
                    self.directions[i] = 1;
                }
            }
        }

        if permutation_to_return == self.s {
            self.done = true;
        }

        self.is_even = !self.is_even;

        return Some(permutation_to_return);
    }
}

pub fn compose<const N: usize>(s1: Permutation<N>, s2: Permutation<N>) -> Permutation<N> {
    let mut composition: Permutation<N> = Permutation {
        s: [0; N]
    };
    for i in 0..N {
        composition.s[i] = s1.s[s2.s[i]];
    }
    return composition;
}

pub fn invert<const N: usize>(s: Permutation<N>) -> Permutation<N> {
    let mut inverse: Permutation<N> = Permutation {
        s: [0; N]
    };
    for i in 0..N {
        inverse.s[s.s[i]] = i;
    }
    return inverse;
}