pub mod abelian;
pub mod symmetric;
pub mod alternating;

use std::ops::Index;
use crate::action::conjugate::Conjugate;

pub trait Group<G: ?Sized + Copy>: Index<usize, Output=G> {
    // Associative group operation G x G -> G
    fn op(&self, a: G, b: G) -> G;

    // The identity e satisfies op(g,e) = op(e,g) = g for any member g
    fn identity(&self) -> G;

    // inv(g) satiesfies op(g,inv(g)) = op(inv(g),g) = e
    fn inv(&self, g: G) -> G;

    // Number of members in group
    fn order(&self) -> usize;

    // conjugate(g, h) = hgh^(-1)
    fn conjugate(&self, h: G, g: G) -> G {
        return self.op(self.op(h, g), self.inv(h));
    }
}

fn find_conjugacy_classes<G: ?Sized + Copy + Eq>(grp: &dyn Group<G>) {
    let action = Conjugate::new(grp);
    for i in 0..grp.order() {
        let g = grp[i];
    }

    let v: Vec<u32> = vec!(1, 2, 3);
    for i in 0..v.len() {
        for j in 0..v.len() {

        }
    }
}