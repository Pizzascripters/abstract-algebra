pub mod abelian;
pub mod symmetric;
pub mod alternating;

pub trait Group<G: ?Sized> {
    // Associative group operation G x G -> G
    fn op(&self, a: G, b: G) -> G;

    // The identity e satisfies op(g,e) = op(e,g) = g for any member g
    fn identity(&self) -> G;

    // inv(g) satiesfied op(g,inv(g)) = op(inv(g),g) = e
    fn inv(&self, g: G) -> G;

    // Used for iterating through group
    fn next(&mut self) -> G;

    // Number of members in group
    fn order(&self) -> u32;
}