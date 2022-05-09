use crate::group::Group;
use super::Action;

pub struct Conjugate<'a, G: ?Sized + Copy> {
    group: &'a mut (dyn Group<G> + 'a)
}

impl<'a, G: ?Sized + Copy + Eq> Action<'a, G, G> for Conjugate<'a, G> {

    fn get_group(&mut self) -> &mut (dyn Group<G> + 'a) {
        return self.group;
    }

    fn reset(&mut self) -> () {
        self.group.reset();
    }

    fn next(&mut self) -> G {
        return self.group.next();
    }

    fn cardinality(&self) -> u32 {
        return self.group.order();
    }

    // Group action should be associative op(group.op(g1, g2), x) = op(g1, op(g2, x))
    fn op(&self, g: G, h: G) -> G {
        return self.group.conjugate(g, h);
    }
}

impl<'a, G: ?Sized + Copy> Conjugate<'a, G> {

    pub fn new(group: &'a mut dyn Group<G>) -> Self {
        return Conjugate {
            group
        }
    }
}