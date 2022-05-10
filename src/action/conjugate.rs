use crate::group::Group;
use super::Action;

pub struct Conjugate<'a, G: ?Sized + Copy> {
    group: &'a mut dyn Group<G, Item=G>
}

impl<'a, G: ?Sized + Copy + Eq> Action<G, G> for Conjugate<'a, G> {

    fn op(&self, g: G, h: G) -> G {
        return self.group.conjugate(g, h);
    }
}

impl<'a, G: ?Sized + Copy> Conjugate<'a, G> {

    pub fn new(group: &'a mut dyn Group<G, Item=G>) -> Self {
        return Conjugate {
            group
        }
    }
}