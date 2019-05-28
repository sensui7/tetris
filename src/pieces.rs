use crate::randomizer::PieceTypes::*;
use crate::randomizer::*;
use crate::WIDTH;
use std::fmt;

pub const UNIT: f64 = 30.0;

#[derive(PartialEq, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Dir {
    LEFT,
    RIGHT,
}

/*
1 - T
2 - L
3 - L (rev)
4 - Z
5 - Z (rev)
6 - Square
7 - I
*/

#[derive(PartialEq, Clone, Debug)]
pub struct Piece {
    pub piece_type: PieceTypes,
    pub rotation: u64,
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub p4: Point,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Piece: {}", self.piece_type)
    }
}

impl Piece {
    pub fn move_left(&mut self) {
        if self.p1.x > 0.0 && self.p2.x > 0.0 && self.p3.x > 0.0 && self.p4.x > 0.0 {
            self.p1.x -= UNIT;
            self.p2.x -= UNIT;
            self.p3.x -= UNIT;
            self.p4.x -= UNIT;
        }
    }

    pub fn move_right(&mut self) {
        if self.p1.x < WIDTH - UNIT
            && self.p2.x < WIDTH - UNIT
            && self.p3.x < WIDTH - UNIT
            && self.p4.x < WIDTH - UNIT
        {
            self.p1.x += UNIT;
            self.p2.x += UNIT;
            self.p3.x += UNIT;
            self.p4.x += UNIT;
        }
    }

    pub fn move_down(&mut self) {
        self.p1.y += UNIT;
        self.p2.y += UNIT;
        self.p3.y += UNIT;
        self.p4.y += UNIT;
    }

    pub fn rotate_t_left(&mut self) {
        self.p2.x = self.p4.x;
        self.p2.y = self.p4.y;
        self.p4.x = self.p3.x;
        self.p4.y = self.p3.y;
    }

    pub fn rotate_t_right(&mut self) {
        self.p3.x = self.p4.x;
        self.p3.y = self.p4.y;
        self.p4.x = self.p2.x;
        self.p4.y = self.p2.y;
    }

    pub fn rotate_right(&mut self) {
        if self.piece_type == T {
            match self.rotation {
                0 => {
                    self.rotate_t_right();
                    self.p2.x = self.p1.x;
                    self.p2.y = self.p1.y + UNIT;
                    self.rotation = 3;
                }
                3 => {
                    self.rotate_t_right();
                    self.p2.x = self.p1.x - UNIT;
                    self.p2.y = self.p1.y;
                    self.rotation = 2;
                }
                2 => {
                    self.rotate_t_right();
                    self.p2.x = self.p1.x;
                    self.p2.y = self.p1.y - UNIT;
                    self.rotation = 1;
                }
                1 => {
                    self.rotate_t_right();
                    self.p2.x = self.p1.x + UNIT;
                    self.p2.y = self.p1.y;
                    self.rotation = 0;
                }
                _ => println!("Error: could not rotate right for T piece"),
            }
        }
        if self.piece_type == I {
            match self.rotation {
                0 => {
                    self.p4.x = self.p3.x;
                    self.p4.y = self.p3.y + UNIT;
                    self.p2.x = self.p3.x;
                    self.p2.y = self.p3.y - UNIT;
                    self.p1.x = self.p2.x;
                    self.p1.y = self.p2.y - UNIT;
                    self.rotation = 3;
                }
                3 => {
                    self.p4.x = self.p3.x - UNIT;
                    self.p4.y = self.p3.y;
                    self.p2.x = self.p3.x + UNIT;
                    self.p2.y = self.p3.y;
                    self.p1.x = self.p2.x + UNIT;
                    self.p1.y = self.p2.y;
                    self.rotation = 2;
                }
                2 => {
                    self.p4.x = self.p3.x;
                    self.p4.y = self.p3.y - UNIT;
                    self.p2.x = self.p3.x;
                    self.p2.y = self.p3.y + UNIT;
                    self.p1.x = self.p2.x;
                    self.p1.y = self.p2.y + UNIT;
                    self.rotation = 1;
                }
                1 => {
                    self.p4.x = self.p3.x + UNIT;
                    self.p4.y = self.p3.y;
                    self.p2.x = self.p3.x - UNIT;
                    self.p2.y = self.p3.y;
                    self.p1.x = self.p2.x - UNIT;
                    self.p1.y = self.p2.y;
                    self.rotation = 0;
                }
                _ => println!("Error: could not rotate right for I piece"),
            }
        }
        if self.piece_type == Z {
            match self.rotation {
                0 => {
                    self.p2 = self.p4.clone();
                    self.p4.x = self.p3.x;
                    self.p4.y = self.p3.y + UNIT;
                    self.p1.x = self.p2.x;
                    self.p1.y = self.p2.y - UNIT;
                    self.rotation = 3;
                }
                3 => {
                    self.p2 = self.p4.clone();
                    self.p4.x = self.p3.x - UNIT;
                    self.p4.y = self.p3.y;
                    self.p1.x = self.p2.x + UNIT;
                    self.p1.y = self.p2.y;
                    self.rotation = 2;
                }
                2 => {
                    self.p2 = self.p4.clone();
                    self.p4.x = self.p3.x;
                    self.p4.y = self.p3.y - UNIT;
                    self.p1.x = self.p2.x;
                    self.p1.y = self.p2.y + UNIT;
                    self.rotation = 1;
                }
                1 => {
                    self.p2 = self.p4.clone();
                    self.p4.x = self.p3.x + UNIT;
                    self.p4.y = self.p3.y;
                    self.p1.x = self.p2.x - UNIT;
                    self.p1.y = self.p2.y;
                    self.rotation = 0;
                }
                _ => println!("Error: could not rotate right for Z piece"),
            }
        }
        if self.piece_type == S {
            match self.rotation {
                0 => {
                    self.p4 = self.p2.clone();
                    self.p2.x = self.p3.x - UNIT;
                    self.p2.y = self.p3.y;
                    self.p1.x = self.p2.x;
                    self.p1.y = self.p2.y - UNIT;
                    self.rotation = 3;
                }
                3 => {
                    self.p4 = self.p2.clone();
                    self.p2.x = self.p3.x;
                    self.p2.y = self.p3.y - UNIT;
                    self.p1.x = self.p2.x + UNIT;
                    self.p1.y = self.p2.y;
                    self.rotation = 2;
                }
                2 => {
                    self.p4 = self.p2.clone();
                    self.p2.x = self.p3.x + UNIT;
                    self.p2.y = self.p3.y;
                    self.p1.x = self.p2.x;
                    self.p1.y = self.p2.y + UNIT;
                    self.rotation = 1;
                }
                1 => {
                    self.p4 = self.p2.clone();
                    self.p2.x = self.p3.x;
                    self.p2.y = self.p3.y + UNIT;
                    self.p1.x = self.p2.x - UNIT;
                    self.p1.y = self.p2.y;
                    self.rotation = 0;
                }
                _ => println!("Error: could not rotate right for S piece"),
            }
        }
        if self.piece_type == L {
            match self.rotation {
                0 => {
                    self.p4.x = self.p3.x;
                    self.p4.y = self.p3.y;
                    self.p3.x = self.p2.x;
                    self.p3.y = self.p2.y;
                    self.p2.x = self.p3.x + UNIT;
                    self.p2.y = self.p3.y;
                    self.p1.x = self.p2.x + UNIT;
                    self.p1.y = self.p2.y;
                    self.rotation = 3;
                }
                3 => {
                    self.p4.x = self.p3.x;
                    self.p4.y = self.p3.y;
                    self.p3.x = self.p2.x;
                    self.p3.y = self.p2.y;
                    self.p2.x = self.p3.x;
                    self.p2.y = self.p3.y + UNIT;
                    self.p1.x = self.p2.x;
                    self.p1.y = self.p2.y + UNIT;
                    self.rotation = 2;
                }
                2 => {
                    self.p4.x = self.p3.x;
                    self.p4.y = self.p3.y;
                    self.p3.x = self.p2.x;
                    self.p3.y = self.p2.y;
                    self.p2.x = self.p3.x - UNIT;
                    self.p2.y = self.p3.y;
                    self.p1.x = self.p2.x - UNIT;
                    self.p1.y = self.p2.y;
                    self.rotation = 1;
                }
                1 => {
                    self.p4.x = self.p3.x;
                    self.p4.y = self.p3.y;
                    self.p3.x = self.p2.x;
                    self.p3.y = self.p2.y;
                    self.p2.x = self.p3.x;
                    self.p2.y = self.p3.y - UNIT;
                    self.p1.x = self.p2.x;
                    self.p1.y = self.p2.y - UNIT;
                    self.rotation = 0;
                }
                _ => println!("Error: could not rotate right for L piece"),
            }
        }
        if self.piece_type == J {
            match self.rotation {
                0 => {
                    self.p4.x = self.p2.x;
                    self.p4.y = self.p2.y;
                    self.p2.y = self.p3.y;
                    self.p2.x = self.p3.x + UNIT;
                    self.p1.y = self.p2.y;
                    self.p1.x = self.p2.x + UNIT;
                    self.rotation = 3;
                }
                3 => {
                    self.p4.x = self.p2.x;
                    self.p4.y = self.p2.y;
                    self.p2.y = self.p3.y + UNIT;
                    self.p2.x = self.p3.x;
                    self.p1.y = self.p2.y + UNIT;
                    self.p1.x = self.p2.x;
                    self.rotation = 2;
                }
                2 => {
                    self.p4.x = self.p2.x;
                    self.p4.y = self.p2.y;
                    self.p2.y = self.p3.y;
                    self.p2.x = self.p3.x - UNIT;
                    self.p1.y = self.p2.y;
                    self.p1.x = self.p2.x - UNIT;
                    self.rotation = 1;
                }
                1 => {
                    self.p4.x = self.p2.x;
                    self.p4.y = self.p2.y;
                    self.p2.y = self.p3.y - UNIT;
                    self.p2.x = self.p3.x;
                    self.p1.y = self.p2.y - UNIT;
                    self.p1.x = self.p2.x;
                    self.rotation = 0;
                }
                _ => println!("Error: could not rotate right for J piece"),
            }
        }
    }

    pub fn rotate_left(&mut self) {
        if self.piece_type == T {
            match self.rotation {
                0 => {
                    self.rotate_t_left();
                    self.p3.x = self.p1.x;
                    self.p3.y = self.p1.y + UNIT;
                    self.rotation = 1;
                }
                1 => {
                    self.rotate_t_left();
                    self.p3.x = self.p1.x + UNIT;
                    self.p3.y = self.p1.y;
                    self.rotation = 2;
                }
                2 => {
                    self.rotate_t_left();
                    self.p3.x = self.p1.x;
                    self.p3.y = self.p1.y - UNIT;
                    self.rotation = 3;
                }
                3 => {
                    self.rotate_t_left();
                    self.p3.x = self.p1.x - UNIT;
                    self.p3.y = self.p1.y;
                    self.rotation = 0;
                }
                _ => println!("Error: couldn't left rotate T"),
            }
        }
        if self.piece_type == I {
            match self.rotation {
                0 => {
                    self.p4.x = self.p3.x;
                    self.p4.y = self.p3.y - UNIT;
                    self.p2.x = self.p3.x;
                    self.p2.y = self.p3.y + UNIT;
                    self.p1.x = self.p2.x;
                    self.p1.y = self.p2.y + UNIT;
                    self.rotation = 1;
                }
                1 => {
                    self.p4.x = self.p3.x - UNIT;
                    self.p4.y = self.p3.y;
                    self.p2.x = self.p3.x + UNIT;
                    self.p2.y = self.p3.y;
                    self.p1.x = self.p2.x + UNIT;
                    self.p1.y = self.p2.y;
                    self.rotation = 2;
                }
                2 => {
                    self.p4.x = self.p3.x;
                    self.p4.y = self.p3.y + UNIT;
                    self.p2.x = self.p3.x;
                    self.p2.y = self.p3.y - UNIT;
                    self.p1.x = self.p2.x;
                    self.p1.y = self.p2.y - UNIT;
                    self.rotation = 3;
                }
                3 => {
                    self.p4.x = self.p3.x + UNIT;
                    self.p4.y = self.p3.y;
                    self.p2.x = self.p3.x - UNIT;
                    self.p2.y = self.p3.y;
                    self.p1.x = self.p2.x - UNIT;
                    self.p1.y = self.p2.y;
                    self.rotation = 0;
                }
                _ => println!("Error: couldn't left rotate I"),
            }
        }
        if self.piece_type == Z {
            match self.rotation {
                0 => {
                    self.p4 = self.p2.clone();
                    self.p2.x = self.p3.x - UNIT;
                    self.p2.y = self.p3.y;
                    self.p1.x = self.p2.x;
                    self.p1.y = self.p2.y + UNIT;
                    self.rotation = 1;
                }
                1 => {
                    self.p4 = self.p2.clone();
                    self.p2.x = self.p3.x;
                    self.p2.y = self.p3.y + UNIT;
                    self.p1.x = self.p2.x + UNIT;
                    self.p1.y = self.p2.y;
                    self.rotation = 2;
                }
                2 => {
                    self.p4 = self.p2.clone();
                    self.p2.x = self.p3.x + UNIT;
                    self.p2.y = self.p3.y;
                    self.p1.x = self.p2.x;
                    self.p1.y = self.p2.y - UNIT;
                    self.rotation = 3;
                }
                3 => {
                    self.p4 = self.p2.clone();
                    self.p2.x = self.p3.x;
                    self.p2.y = self.p3.y - UNIT;
                    self.p1.x = self.p2.x - UNIT;
                    self.p1.y = self.p2.y;
                    self.rotation = 0;
                }
                _ => println!("Error: couldn't left rotate Z"),
            }
        }
        if self.piece_type == S {
            match self.rotation {
                0 => {
                    self.p2 = self.p4.clone();
                    self.p4.x = self.p3.x;
                    self.p4.y = self.p3.y - UNIT;
                    self.p1.x = self.p2.x;
                    self.p1.y = self.p2.y + UNIT;
                    self.rotation = 1;
                }
                1 => {
                    self.p2 = self.p4.clone();
                    self.p4.x = self.p3.x - UNIT;
                    self.p4.y = self.p3.y;
                    self.p1.x = self.p2.x + UNIT;
                    self.p1.y = self.p2.y;
                    self.rotation = 2;
                }
                2 => {
                    self.p2 = self.p4.clone();
                    self.p4.x = self.p3.x;
                    self.p4.y = self.p3.y + UNIT;
                    self.p1.x = self.p2.x;
                    self.p1.y = self.p2.y - UNIT;
                    self.rotation = 3;
                }
                3 => {
                    self.p2 = self.p4.clone();
                    self.p4.x = self.p3.x + UNIT;
                    self.p4.y = self.p3.y;
                    self.p1.x = self.p2.x - UNIT;
                    self.p1.y = self.p2.y;
                    self.rotation = 0;
                }
                _ => println!("Error: couldn't left rotate S"),
            }
        }
        if self.piece_type == L {
            match self.rotation {
                0 => {
                    self.p4 = self.p2.clone();
                    self.p2.x = self.p3.x - UNIT;
                    self.p2.y = self.p3.y;
                    self.p1.x = self.p2.x - UNIT;
                    self.p1.y = self.p2.y;
                    self.rotation = 1;
                }
                1 => {
                    self.p4 = self.p2.clone();
                    self.p2.x = self.p3.x;
                    self.p2.y = self.p3.y + UNIT;
                    self.p1.x = self.p2.x;
                    self.p1.y = self.p2.y + UNIT;
                    self.rotation = 2;
                }
                2 => {
                    self.p4 = self.p2.clone();
                    self.p2.x = self.p3.x + UNIT;
                    self.p2.y = self.p3.y;
                    self.p1.x = self.p2.x + UNIT;
                    self.p1.y = self.p2.y;
                    self.rotation = 3;
                }
                3 => {
                    self.p4 = self.p2.clone();
                    self.p2.x = self.p3.x;
                    self.p2.y = self.p3.y - UNIT;
                    self.p1.x = self.p2.x;
                    self.p1.y = self.p2.y - UNIT;
                    self.rotation = 0;
                }
                _ => println!("Error: couldn't left rotate L"),
            }
        }
        if self.piece_type == J {
            match self.rotation {
                0 => {
                    self.p4 = self.p3.clone();
                    self.p3 = self.p2.clone();
                    self.p2.y = self.p3.y;
                    self.p2.x = self.p3.x - UNIT;
                    self.p1.y = self.p2.y;
                    self.p1.x = self.p2.x - UNIT;
                    self.rotation = 1;
                }
                1 => {
                    self.p4 = self.p3.clone();
                    self.p3 = self.p2.clone();
                    self.p2.y = self.p3.y + UNIT;
                    self.p2.x = self.p3.x;
                    self.p1.y = self.p2.y + UNIT;
                    self.p1.x = self.p2.x;
                    self.rotation = 2;
                }
                2 => {
                    self.p4 = self.p3.clone();
                    self.p3 = self.p2.clone();
                    self.p2.y = self.p3.y;
                    self.p2.x = self.p3.x + UNIT;
                    self.p1.y = self.p2.y;
                    self.p1.x = self.p2.x + UNIT;
                    self.rotation = 3;
                }
                3 => {
                    self.p4 = self.p3.clone();
                    self.p3 = self.p2.clone();
                    self.p2.y = self.p3.y - UNIT;
                    self.p2.x = self.p3.x;
                    self.p1.y = self.p2.y - UNIT;
                    self.p1.x = self.p2.x;
                    self.rotation = 0;
                }
                _ => println!("Error: couldn't left rotate J"),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::pieces::Piece;
    use crate::pieces::UNIT;
    use crate::randomizer::T_PIECE;

    #[test]
    fn move_left_test() {
        let mut test: Piece = T_PIECE.clone();
        let mut expected: Piece = T_PIECE.clone();

        expected.p1.x -= UNIT;
        expected.p2.x -= UNIT;
        expected.p3.x -= UNIT;
        expected.p4.x -= UNIT;

        test.move_left();

        assert_eq!(test, expected);
    }

    #[test]
    fn move_right_test() {
        let mut test: Piece = T_PIECE.clone();
        let mut expected: Piece = T_PIECE.clone();

        expected.p1.x += UNIT;
        expected.p2.x += UNIT;
        expected.p3.x += UNIT;
        expected.p4.x += UNIT;

        test.move_right();

        assert_eq!(test, expected);
    }

    #[test]
    fn move_down_test() {
        let mut test: Piece = T_PIECE.clone();
        let mut expected: Piece = T_PIECE.clone();

        expected.p1.y += UNIT;
        expected.p2.y += UNIT;
        expected.p3.y += UNIT;
        expected.p4.y += UNIT;

        test.move_down();

        assert_eq!(test, expected);
    }
}
