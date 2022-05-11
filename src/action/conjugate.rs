use crate::group::Group;
use super::Action;

pub struct Conjugate<'a, G> {
    group: &'a dyn Group<G>
}

impl<'a, G: Clone + ToString + PartialEq> Action<G, G> for Conjugate<'a, G> {

    fn op(&self, g: G, h: G) -> G {
        return self.group.conjugate(g, h);
    }
}

impl<'a, G> Conjugate<'a, G> {

    pub fn new(group: &'a dyn Group<G>) -> Self {
        return Conjugate {
            group
        }
    }
}