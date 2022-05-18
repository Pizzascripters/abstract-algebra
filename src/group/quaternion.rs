use super::Group;

const QUATERNIONS: [Quaternion; 8] = [
    Quaternion{symbol: QuaternionSymbol::E, negative: true},
    Quaternion{symbol: QuaternionSymbol::E, negative: false},
    Quaternion{symbol: QuaternionSymbol::I, negative: true},
    Quaternion{symbol: QuaternionSymbol::I, negative: false},
    Quaternion{symbol: QuaternionSymbol::J, negative: true},
    Quaternion{symbol: QuaternionSymbol::J, negative: false},
    Quaternion{symbol: QuaternionSymbol::K, negative: true},
    Quaternion{symbol: QuaternionSymbol::K, negative: false}
];

#[derive(PartialEq, Clone, Copy)]
enum QuaternionSymbol {
    E, I, J, K
}

pub struct Quaternion {
    symbol: QuaternionSymbol,
    negative: bool
}

impl ToString for Quaternion {
    fn to_string(&self) -> String {
        let symbol = match self.symbol {
            QuaternionSymbol::E => "e",
            QuaternionSymbol::I => "i",
            QuaternionSymbol::J => "j",
            QuaternionSymbol::K => "k"
        };
        return format!("{}{}", if self.negative {"-"} else {""}, symbol);
    }
}

impl PartialEq for Quaternion {
    fn eq(&self, other: &Self) -> bool {
        return self.symbol == other.symbol && self.negative == other.negative;
    }
}

impl Copy for Quaternion {}
impl Clone for Quaternion {
    fn clone(&self) -> Self {
        return Quaternion {
            symbol: self.symbol,
            negative: self.negative
        }
    }
}

pub struct QuaternionGroup {}

impl Group<Quaternion> for QuaternionGroup {

    fn op(&self, a: Quaternion, b: Quaternion) -> Quaternion {
        if matches!(a.symbol, QuaternionSymbol::E) {
            // eq = q
            Quaternion {
                symbol: b.symbol,
                negative: a.negative ^ b.negative
            }
        } else if matches!(b.symbol, QuaternionSymbol::E) {
            // qe = q
            Quaternion {
                symbol: a.symbol,
                negative: a.negative ^ b.negative
            }
        } else if a.symbol == b.symbol {
            // i^2 = j^2 = k^2 = -1
            Quaternion {
                symbol: QuaternionSymbol::E,
                negative: a.negative == b.negative
            }
        } else if matches!(a.symbol, QuaternionSymbol::I) && matches!(b.symbol, QuaternionSymbol::J)
               || matches!(a.symbol, QuaternionSymbol::J) && matches!(b.symbol, QuaternionSymbol::I) {
            // ij = k and ji = -k
            Quaternion {
                symbol: QuaternionSymbol::K,
                negative: a.negative ^ b.negative ^ matches!(a.symbol, QuaternionSymbol::J)
            }
        } else if matches!(a.symbol, QuaternionSymbol::I) && matches!(b.symbol, QuaternionSymbol::K)
               || matches!(a.symbol, QuaternionSymbol::K) && matches!(b.symbol, QuaternionSymbol::I) {
            // ik = -j and ki = j
            Quaternion {
                symbol: QuaternionSymbol::J,
                negative: a.negative ^ b.negative ^ matches!(a.symbol, QuaternionSymbol::I)
            }
        } else {
            // jk = i and kj = -i
            Quaternion {
                symbol: QuaternionSymbol::I,
                negative: a.negative ^ b.negative ^ matches!(a.symbol, QuaternionSymbol::K)
            }
        }
    }

    fn identity(&self) -> Quaternion {
        return Quaternion {
            symbol: QuaternionSymbol::E,
            negative: false
        }
    }

    fn inv(&self, g: Quaternion) -> Quaternion {
        return Quaternion {
            symbol: g.symbol,
            negative: matches!(g.symbol, QuaternionSymbol::E) == g.negative
        };
    }

    fn order(&self) -> usize {
        return 8;
    }

    fn index(&self, i: usize) -> Quaternion {
        return QUATERNIONS[i];
    }

    fn get_name(&self) -> String {
        "Q8".to_owned()
    }
}

impl QuaternionGroup {
    
    pub fn new() -> Self {
        return QuaternionGroup {}
    }
}