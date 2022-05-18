use super::Group;

pub struct DihedralMember {
    r: usize,  // Rotations
    s: bool    // Flips
}

impl ToString for DihedralMember {
    fn to_string(&self) -> String {
        if self.r == 0 && !self.s {
            return format!("{}", "e");
        } else if self.r == 0 {
            return format!("{}", "s");
        } else if !self.s {
            return format!("sr^{}", self.r);
        } else {
            return format!("sr^{}", self.r);
        }
    }
}

impl PartialEq for DihedralMember {
    fn eq(&self, other: &Self) -> bool {
        return self.r == other.r && self.s == other.s;
    }
}

impl Copy for DihedralMember {}
impl Clone for DihedralMember {
    fn clone(&self) -> Self {
        return DihedralMember {
            r: self.r,
            s: self.s
        }
    }
}

pub struct DihedralGroup {
    half_order: usize
}

impl Group<DihedralMember> for DihedralGroup {

    fn op(&self, a: DihedralMember, b: DihedralMember) -> DihedralMember {
        return DihedralMember {
            r: (if b.s {self.half_order + b.r - a.r} else {a.r + b.r}) % self.half_order,
            s: a.s ^ b.s
        }
    }

    fn identity(&self) -> DihedralMember {
        return DihedralMember {
            r: 0,
            s: false
        }
    }

    fn inv(&self, g: DihedralMember) -> DihedralMember {
        return DihedralMember {
            r: if g.s {g.r} else {self.half_order - g.r},
            s: g.s
        }
    }

    fn order(&self) -> usize {
        return 2 * self.half_order;
    }

    fn index(&self, i: usize) -> DihedralMember {
        return DihedralMember {
            r: i % self.half_order,
            s: i >= self.half_order
        };
    }

    fn get_name(&self) -> String {
        format!("D{}", self.half_order)
    }
}

impl DihedralGroup {
    
    pub fn new(half_order: usize) -> Self {
        return DihedralGroup {
            half_order
        }
    }
}