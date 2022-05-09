pub mod abelian;
pub mod symmetric;
pub mod alternating;

pub trait Group<G: ?Sized + Copy> {
    // Associative group operation G x G -> G
    fn op(&self, a: G, b: G) -> G;

    // The identity e satisfies op(g,e) = op(e,g) = g for any member g
    fn identity(&self) -> G;

    // inv(g) satiesfied op(g,inv(g)) = op(inv(g),g) = e
    fn inv(&self, g: G) -> G;

    // Used for iterating through group
    fn reset(&mut self) -> ();
    fn next(&mut self) -> G;

    // Number of members in group
    fn order(&self) -> u32;

    fn conjugate(&self, g: G, h: G) -> G {
        return self.op(self.op(h, g), self.inv(h));
    }
}