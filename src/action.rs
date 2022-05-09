pub mod conjugate;

use crate::group::Group;

// X acts on some Group<G>
pub trait Action<'a, G: ?Sized + Copy, X: Eq> {
    fn get_group(&mut self) -> &mut (dyn Group<G> + 'a);

    // Group action should be associative op(group.op(g1, g2), x) = op(g1, op(g2, x))
    fn op(&self, g: G, x: X) -> X;

    // Used for iterating through set
    fn reset(&mut self) -> ();
    fn next(&mut self) -> G;
    fn cardinality(&self) -> u32;

    // fn orbit(&mut self, x: X) -> Vec<X> {
    //     let g = self.get_group();
    //     let orbit: Vec<X> = Vec::new();
    //     for _ in 1..g.order() {
    //         let y = self.op(g.next(), x);
    //         if !orbit.contains(&y) {
    //             orbit.push(y);
    //         }
    //     }
    //     return orbit;
    // }
}