use super::Group;

// If s is a permutation then for any i in 0..N there should exist a j such that s[j] = i
// Alternatively 0 <= s[i] < N and each s[i] is unique
type Permutation<const N: usize> = [usize;N];

pub struct SymmetricGroup<const N: usize> {}

impl<const N: usize> Group<Permutation<N>> for SymmetricGroup<N> {

    // Permutation multiplication
    fn op(&self, a: Permutation<N>, b: Permutation<N>) -> Permutation<N> {
        let mut ab: Permutation<N> = [0;N];
        for i in 0..N {
            ab[i] = a[b[i]];
        }
        return ab;
    }

    fn inv(&self, g: Permutation<N>) -> Permutation<N> {
        let mut g_inv: Permutation<N> = [0;N];
        for i in 0..N {
            g_inv[g[i]] = i;
        }
        return g_inv;
    }
}