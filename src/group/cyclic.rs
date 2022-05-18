use crate::group::Group;

pub struct CyclicGroup {
    order: usize
}

impl Group<usize> for CyclicGroup {

    fn op(&self, a: usize, b: usize) -> usize {
        return (a + b) % self.order;
    }

    fn identity(&self) -> usize {
        return 0;
    }

    fn inv(&self, g: usize) -> usize {
        return (self.order - g) % self.order;
    }

    fn order(&self) -> usize {
        return self.order as usize;
    }

    fn index(&self, i: usize) -> usize {
        return i;
    }

    fn get_name(&self) -> String {
        format!("Z{}", self.order)
    }
}

impl CyclicGroup {
    
    pub fn new(order: usize) -> Self {
        return CyclicGroup {
            order
        }
    }
}