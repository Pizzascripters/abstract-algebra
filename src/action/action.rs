use crate::group::Group;

// X acts on some Group<G>
pub trait Action<G, X: PartialEq> {
    // Group action should be associative op(group.op(g1, g2), x) = op(g1, op(g2, x))
    fn op(&self, g: G, x: X) -> X;
}

pub fn orbit<G: Clone + ToString, X: Clone + PartialEq>(grp: &dyn Group<G>, action: &dyn Action<G, X>, x: X) -> Vec<X> {
    let mut orbit: Vec<X> = Vec::new();
    for i in 0..grp.order() {
        let g = grp.index(i);
        let y = action.op(g, x.clone());
        if !orbit.contains(&y) {
            orbit.push(y);
        }
    }
    return orbit;
}