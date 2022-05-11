use crate::action;
use crate::action::Conjugate;

pub trait Group<G: ?Sized + Copy + ToString> {
    // Associative group operation G x G -> G
    fn op(&self, a: G, b: G) -> G;

    // The identity e satisfies op(g,e) = op(e,g) = g for any member g
    fn identity(&self) -> G;

    // inv(g) satiesfies op(g,inv(g)) = op(inv(g),g) = e
    fn inv(&self, g: G) -> G;

    // Number of members in group
    fn order(&self) -> usize;

    fn index(&self, i: usize) -> G;

    // conjugate(g, h) = hgh^(-1)
    fn conjugate(&self, h: G, g: G) -> G {
        return self.op(self.op(h, g), self.inv(h));
    }

    fn to_string(&self) -> String {
        let mut s: String = String::new();
        let order = self.order();
        for i in 0..order {
            s += &self.index(i).to_string();
            if i < order - 1 {
                s += ", ";
            }
        }
        return format!("[order {}] {{{}}}", order, s);
    }
}

pub fn find_center<G: ?Sized + Copy + ToString + PartialEq>(grp: &dyn Group<G>) -> Vec<G> {
    let action: Conjugate<G> = Conjugate::new(grp);
    let mut center: Vec<G> = Vec::new();
    for i in 0..grp.order() {
        let orbit = action::orbit(grp, &action, grp.index(i));
        if orbit.len() == 1 {
            center.push(grp.index(i));
        }
    }
    return center;
}

pub fn find_conjugacy_classes<G: ?Sized + Copy + ToString + PartialEq>(grp: &dyn Group<G>) -> Vec<Vec<G>> {
    let action: Conjugate<G> = Conjugate::new(grp);
    let mut classes: Vec<Vec<G>> = Vec::new();
    for i in 0..grp.order() {
        let g = grp.index(i);
        // Skip member if already in a class
        let mut already_in_class = false;
        for j in 0..classes.len() {
            if classes[j].contains(&g) {
                already_in_class = true;
                break;
            }
        }
        if already_in_class {
            continue;
        }
        classes.push(action::orbit(grp, &action, g));
    }
    return classes;
}