use crate::pieces::*;
use crate::randomizer::PieceTypes::*;
use rand;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum PieceTypes {
    T,
    I,
    O,
    L,
    J,
    Z,
    S,
}

impl fmt::Display for PieceTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PieceTypes::T => write!(f, "T"),
            PieceTypes::I => write!(f, "I"),
            PieceTypes::O => write!(f, "O"),
            PieceTypes::L => write!(f, "L"),
            PieceTypes::J => write!(f, "J"),
            PieceTypes::Z => write!(f, "Z"),
            PieceTypes::S => write!(f, "S"),
        }
    }
}

pub struct Randomizer {
    pub bag: Vec<Piece>,
}

pub const T_PIECE: Piece = Piece {
    piece_type: T,
    rotation: 0,
    p1: Point {
        x: UNIT * 5.0,
        y: 0.0,
    },
    p2: Point {
        x: UNIT * 6.0,
        y: 0.0,
    },
    p3: Point {
        x: UNIT * 4.0,
        y: 0.0,
    },
    p4: Point {
        x: UNIT * 5.0,
        y: UNIT * -1.0,
    },
};

pub const I_PIECE: Piece = Piece {
    piece_type: I,
    rotation: 0,
    p1: Point {
        x: UNIT * 3.0,
        y: 0.0,
    },
    p2: Point {
        x: UNIT * 4.0,
        y: 0.0,
    },
    p3: Point {
        x: UNIT * 5.0,
        y: 0.0,
    },
    p4: Point {
        x: UNIT * 6.0,
        y: 0.0,
    },
};

// Square
pub const O_PIECE: Piece = Piece {
    piece_type: O,
    rotation: 0,
    p1: Point {
        x: UNIT * 4.0,
        y: UNIT * -1.0,
    },
    p2: Point {
        x: UNIT * 4.0,
        y: 0.0,
    },
    p3: Point {
        x: UNIT * 5.0,
        y: UNIT * -1.0,
    },
    p4: Point {
        x: UNIT * 5.0,
        y: 0.0,
    },
};

pub const Z_PIECE: Piece = Piece {
    piece_type: Z,
    rotation: 0,
    p1: Point {
        x: UNIT * 4.0,
        y: 0.0,
    },
    p2: Point {
        x: UNIT * 5.0,
        y: 0.0,
    },
    p3: Point {
        x: UNIT * 5.0,
        y: UNIT,
    },
    p4: Point {
        x: UNIT * 6.0,
        y: UNIT,
    },
};

pub const S_PIECE: Piece = Piece {
    piece_type: S,
    rotation: 0,
    p1: Point {
        x: UNIT * 4.0,
        y: UNIT,
    },
    p2: Point {
        x: UNIT * 5.0,
        y: UNIT,
    },
    p3: Point {
        x: UNIT * 5.0,
        y: 0.0,
    },
    p4: Point {
        x: UNIT * 6.0,
        y: 0.0,
    },
};

pub const L_PIECE: Piece = Piece {
    piece_type: L,
    rotation: 0,
    p1: Point {
        x: UNIT * 5.0,
        y: 0.0,
    },
    p2: Point {
        x: UNIT * 5.0,
        y: UNIT,
    },
    p3: Point {
        x: UNIT * 5.0,
        y: UNIT * 2.0,
    },
    p4: Point {
        x: UNIT * 6.0,
        y: UNIT * 2.0,
    },
};

pub const J_PIECE: Piece = Piece {
    piece_type: J,
    rotation: 0,
    p1: Point {
        x: UNIT * 5.0,
        y: 0.0,
    },
    p2: Point {
        x: UNIT * 5.0,
        y: UNIT,
    },
    p3: Point {
        x: UNIT * 5.0,
        y: UNIT * 2.0,
    },
    p4: Point {
        x: UNIT * 4.0,
        y: UNIT * 2.0,
    },
};

impl Randomizer {
    // Get a random piece from a randomly generated bag
    pub fn get(&mut self) -> Piece {
        if self.bag.is_empty() {
            self.generate()
        }

        self.bag.pop().unwrap()
    }

    pub fn generate(&mut self) {
        // https://rust-num.github.io/num/rand/fn.thread_rng.html
        // Retrieve the lazily-initialized thread-local random number generator, seeded by the system.
        let mut rng = thread_rng();
        self.bag = vec![
            T_PIECE, I_PIECE, O_PIECE, L_PIECE, J_PIECE, Z_PIECE, S_PIECE,
        ];

        // Shuffles mutable slice in-place
        self.bag.shuffle(&mut rng);
    }
}

#[cfg(test)]
mod tests {

    use crate::randomizer::Randomizer;
    use crate::randomizer::J_PIECE;

    #[test]
    fn get_test_one() {
        let mut test: Randomizer = Randomizer { bag: vec![J_PIECE] };

        assert_eq!(test.get(), J_PIECE);
    }

    #[test]
    fn get_test_two() {
        let mut test: Randomizer = Randomizer { bag: vec![] };

        test.get();

        // should expect 6 pieces after taking the 7th random piece out
        assert_eq!(test.bag.len(), 6);
    }
}
