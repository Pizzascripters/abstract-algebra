pub mod conjugate;

use crate::group::Group;

// X acts on some Group<G>
pub trait Action<G: ?Sized + Copy, X: Eq> {
    // Group action should be associative op(group.op(g1, g2), x) = op(g1, op(g2, x))
    fn op(&self, g: G, x: X) -> X;
}

pub fn orbit<G: ?Sized + Copy, X: Eq + Copy>(grp: &mut dyn Group<G, Item=G>, action: &dyn Action<G, X>, x: X) -> Vec<X> {
    let mut orbit: Vec<X> = Vec::new();
    for g in grp {
        let y = action.op(g, x);
        if !orbit.contains(&y) {
            orbit.push(y);
        }
    }
    return orbit;
}