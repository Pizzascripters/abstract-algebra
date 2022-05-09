pub mod conjugate;

// X acts on some Group<G>
pub trait Action<'a, G: ?Sized + Copy, X: Eq> {
    // Group action should be associative op(group.op(g1, g2), x) = op(g1, op(g2, x))
    fn op(&self, g: G, x: X) -> X;

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