use super::arange;
use super::swap;

// If s is a permutation then for any i in 0..N there should exist a j such that s[j] = i
// Alternatively 0 <= s[i] < N and each s[i] is unique
pub struct Permutation {
    pub s: Vec<usize>,
    pub length: usize
}

impl<'a> ToString for Permutation {
    fn to_string(&self) -> String {
        let v: Vec<String> = self.s.iter().map(|i| i.to_string()).collect();
        return format!("({})", v.join(", "));
    }
}

impl<'a> PartialEq for Permutation {
    fn eq(&self, other: &Self) -> bool {
        return self.s == other.s;
    }
}

impl<'a> Clone for Permutation {
    fn clone(&self) -> Self {
        return Permutation {
            s: self.s.clone(),
            length: self.length
        }
    }
}

pub struct IterablePermutation {
    s: Permutation,
    directions: Vec<i8>,
    even_only: bool,
    is_even: bool,
    done: bool
}

impl<'a> Iterator for IterablePermutation {
    type Item = Permutation;

    // Steinhaus–Johnson–Trotter algorithm with Even's speedup
    fn next(&mut self) -> Option<Self::Item> {
        if self.even_only && !self.is_even {
            self.advance();
        }
        return self.advance();
    }
}

impl<'a> IterablePermutation {
    pub fn new(length: usize, even_only: bool) -> Self {
        let mut directions = vec!(-1; length);
        directions[0] = 0;
        return IterablePermutation {
            s: Permutation {
                s: arange(length),
                length
            },
            directions,
            even_only,
            is_even: true,
            done: false
        }
    }

    pub fn advance(&mut self) -> Option<Permutation> {
        if self.done {
            return Option::None;
        }

        let permutation_to_return = self.s.clone();

        let mut k: usize = 0; // Index of greatest nonzero element
        for i in 0..self.s.length {
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

        if k == 0 || k == self.s.length-1 || self.s.s[2*k-k_] > self.s.s[k] {
            self.directions[k] = 0;
        }

        for i in 0..self.s.length {
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

pub fn compose<'a>(s1: Permutation, s2: Permutation) -> Permutation {
    let mut composition: Permutation = Permutation {
        s: vec!(0; s1.length),
        length: s1.length
    };
    for i in 0..s1.length {
        composition.s[i] = s1.s[s2.s[i]];
    }
    composition
}

pub fn invert(s: Permutation) -> Permutation {
    let mut inverse = Permutation {
        s: vec!(0; s.length),
        length: s.length
    };
    for i in 0..s.length {
        inverse.s[s.s[i]] = i;
    }
    inverse
}