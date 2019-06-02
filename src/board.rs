use crate::pieces::Dir::*;
use crate::pieces::UNIT;
use crate::pieces::*;
use crate::randomizer::PieceTypes::*;
use crate::randomizer::I_PIECE;
use crate::randomizer::J_PIECE;
use crate::randomizer::L_PIECE;
use crate::randomizer::O_PIECE;
use crate::randomizer::S_PIECE;
use crate::randomizer::T_PIECE;
use crate::randomizer::Z_PIECE;
use crate::Sound;
use music;

const LEFT_BOUNDARY: f64 = 0.0;
const RIGHT_BOUNDARY: f64 = 9.0;
const BOTTOM_BOUNDARY: f64 = 19.0;

#[derive(PartialEq, Clone, Debug)]
pub struct Board {
    pub rows: u64,
    pub cols: u64,
    pub data: Vec<Vec<u64>>,
}

impl Board {
    // Called when it lands on bottom/collides
    pub fn set(&mut self, p: &Piece) {
        let piece_digit: u64;
        match p.piece_type {
            T => piece_digit = 1,
            I => piece_digit = 2,
            O => piece_digit = 3,
            L => piece_digit = 4,
            J => piece_digit = 5,
            Z => piece_digit = 6,
            S => piece_digit = 7,
        };

        self.data[(p.p1.y / UNIT).round() as usize][(p.p1.x / UNIT).round() as usize] = piece_digit;
        self.data[(p.p2.y / UNIT).round() as usize][(p.p2.x / UNIT).round() as usize] = piece_digit;
        self.data[(p.p3.y / UNIT).round() as usize][(p.p3.x / UNIT).round() as usize] = piece_digit;
        self.data[(p.p4.y / UNIT).round() as usize][(p.p4.x / UNIT).round() as usize] = piece_digit;
    }

    pub fn clear(&mut self) {
        let empty = vec![0; 10];
        let mut count = 0;
        let mut new_data = vec![];

        for i in self.data.iter_mut() {
            let mut iter = i.iter();
            if iter.find(|&&x| x == 0) == None {
                new_data.push(count);
            }
            count += 1;
        }

        let mut removed = 0;

        for j in new_data {
            self.data.remove(j);
            self.data.insert(0 as usize, empty.clone());
            removed += 1;
        }

        match removed {
            4 => music::play_sound(&Sound::Tetris, music::Repeat::Times(0), music::MAX_VOLUME),
            3 => music::play_sound(&Sound::Triple, music::Repeat::Times(0), music::MAX_VOLUME),
            2 => music::play_sound(&Sound::Double, music::Repeat::Times(0), music::MAX_VOLUME),
            _ => {}
        }
    }

    pub fn display(&self) {
        for i in self.data.iter() {
            for j in i {
                print!("{}", j);
            }
            println!();
        }
    }

    pub fn convert_xy(&self, x: f64, y: f64) -> (usize, usize) {
        if y < LEFT_BOUNDARY {
            return ((x / UNIT).round() as usize, 0 as usize);
        }

        ((x / UNIT).round() as usize, (y / UNIT).round() as usize)
    }

    pub fn collision(&mut self, p: &Piece) -> bool {
        let p1 = self.convert_xy(p.p1.x, p.p1.y);
        let p2 = self.convert_xy(p.p2.x, p.p2.y);
        let p3 = self.convert_xy(p.p3.x, p.p3.y);
        let p4 = self.convert_xy(p.p4.x, p.p4.y);

        // Check when too blocks are too high
        if self.check_rotate_overlap(p) {
            let test = p.clone();
            let pieces = vec![
                T_PIECE, I_PIECE, O_PIECE, S_PIECE, Z_PIECE, L_PIECE, J_PIECE,
            ];

            for pie in pieces {
                if test == pie {
                    self.data = vec![vec![0; 10]; 20];
                    return true;
                }
            }
        }

        if p1.1 >= 19 || p2.1 >= 19 || p3.1 >= 19 || p4.1 >= 19 {
            return true;
        }

        if self.check_piece_collision(p1.0, p1.1)
            || self.check_piece_collision(p2.0, p2.1)
            || self.check_piece_collision(p3.0, p3.1)
            || self.check_piece_collision(p4.0, p4.1)
        {
            return true;
        }

        false
    }

    pub fn check_piece_collision(&self, row: usize, col: usize) -> bool {
        if self.data[col + 1][row] != 0 {
            return true;
        }

        false
    }

    // Check if a piece collides horizontally
    pub fn check_side_collision(&self, row: usize, col: usize, dir: Dir) -> bool {
        if dir == LEFT && row == 0 {
            return true;
        }

        if dir == LEFT && row >= 1 && self.data[col][row - 1] != 0 {
            return true;
        }

        if dir == RIGHT && row as u64 == self.rows - 1 {
            return true;
        }

        if dir == RIGHT && (row as u64) < self.rows - 1 && self.data[col][row + 1] != 0 {
            return true;
        }

        false
    }

    // Check if a piece overlaps with another piece via side collision
    pub fn check_overlap(&self, p: &Piece, dir: Dir) -> bool {
        let mut t1: Piece = p.clone();
        let mut t2: Piece = p.clone();

        t1.move_left();
        t2.move_right();

        if dir == LEFT {
            return self.check_rotate_overlap(&t1);
        }

        if dir == RIGHT {
            return self.check_rotate_overlap(&t2);
        }

        false
    }

    // Check if a piece overlaps with another piece before rotating
    pub fn check_rotate_overlap(&self, p: &Piece) -> bool {
        let p1 = self.convert_xy(p.p1.x, p.p1.y);
        let p2 = self.convert_xy(p.p2.x, p.p2.y);
        let p3 = self.convert_xy(p.p3.x, p.p3.y);
        let p4 = self.convert_xy(p.p4.x, p.p4.y);

        // https://doc.rust-lang.org/std/iter/struct.Enumerate.html
        for (i, col) in self.data.iter().enumerate() {
            for (j, _row) in col.iter().enumerate() {
                if self.data[i][j] != 0 {
                    if p1.1 == i && p1.0 == j {
                        return true;
                    }
                    if p2.1 == i && p2.0 == j {
                        return true;
                    }
                    if p3.1 == i && p3.0 == j {
                        return true;
                    }
                    if p4.1 == i && p4.0 == j {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn t_spin(&self, p: &mut Piece, dir: Dir) -> bool {
        let mut test: Piece = p.clone();
        // Need to check if out of bounds
        if dir == LEFT {
            test.rotate_left();
            test.move_down();
            test.move_left();

            let mut first: Piece = p.clone();
            first.rotate_left();

            if self.check_rotate_overlap(&first)
                && !self.check_rotate_overlap(&test)
                && p.piece_type == T
            {
                p.rotate_left();
                p.move_down();
                p.move_left();
                return true;
            }

            test = p.clone();
            test.rotate_left();
            test.move_down();
            test.move_right();

            if self.check_rotate_overlap(&first)
                && !self.check_rotate_overlap(&test)
                && p.piece_type == T
            {
                p.rotate_left();
                p.move_down();
                p.move_right();
                return true;
            }
        } else {
            test.rotate_right();
            test.move_down();
            test.move_right();

            let mut first: Piece = p.clone();
            first.rotate_right();

            if self.check_rotate_overlap(&first)
                && !self.check_rotate_overlap(&test)
                && p.piece_type == T
            {
                p.rotate_right();
                p.move_down();
                p.move_right();
                return true;
            }

            // Check for special case
            test = p.clone();
            test.rotate_right();
            test.move_down();
            test.move_left();

            if self.check_rotate_overlap(&first)
                && !self.check_rotate_overlap(&test)
                && p.piece_type == T
            {
                p.rotate_right();
                p.move_down();
                p.move_left();
                return true;
            }
        }

        false
    }

    // Check if a piece can rotate left/right
    pub fn check_can_rotate(&self, p: &mut Piece, dir: Dir) -> bool {
        let mut test = p.clone();

        if dir == LEFT {
            test.rotate_left();
        }
        if dir == RIGHT {
            test.rotate_right();
        }

        if self.t_spin(p, dir) {
            return false;
        }

        if self.check_rotate_overlap(&test) {
            return false;
        }

        let mut c = 0;

        while (test.p1.x / UNIT) > RIGHT_BOUNDARY
            || (test.p2.x / UNIT) > RIGHT_BOUNDARY
            || (test.p3.x / UNIT) > RIGHT_BOUNDARY
            || (test.p4.x / UNIT) > RIGHT_BOUNDARY
        {
            test.move_left();
            c += 1;
        }

        for _x in 0..c {
            if !self.check_overlap(p, dir) {
                p.move_left();
            } else {
                return false;
            }
        }

        c = 0;

        while (test.p1.x / UNIT) < LEFT_BOUNDARY
            || (test.p2.x / UNIT) < LEFT_BOUNDARY
            || (test.p3.x / UNIT) < LEFT_BOUNDARY
            || (test.p4.x / UNIT) < LEFT_BOUNDARY
        {
            test.move_right();
            c += 1;
        }

        for _x in 0..c {
            if !self.check_overlap(p, dir) {
                p.move_right();
            } else {
                return false;
            }
        }

        // Check bottom pit
        if (test.p4.y / UNIT) > BOTTOM_BOUNDARY
            || (test.p3.y / UNIT) > BOTTOM_BOUNDARY
            || (test.p2.y / UNIT) > BOTTOM_BOUNDARY
            || (test.p1.y / UNIT) > BOTTOM_BOUNDARY
        {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {

    use crate::board::Board;
    use crate::board::BOTTOM_BOUNDARY;
    use crate::board::RIGHT_BOUNDARY;
    use crate::pieces::Dir::LEFT;
    use crate::pieces::Dir::RIGHT;
    use crate::pieces::Piece;
    use crate::pieces::UNIT;
    use crate::randomizer::I_PIECE;

    #[test]
    fn set_test() {
        let mut test: Board = Board {
            rows: 10,
            cols: 20,
            data: vec![vec![0; 10]; 20],
        };

        let test_piece: Piece = I_PIECE;
        test.set(&test_piece);

        for i in 3..6 {
            assert_eq!(test.data[0][i], 2);
        }
    }

    #[test]
    fn clear_test() {
        let mut test: Board = Board {
            rows: 10,
            cols: 20,
            data: vec![vec![0; 10]; 20],
        };

        for i in test.data[19].iter_mut() {
            *i = 1;
        }

        test.clear();

        for i in 0..RIGHT_BOUNDARY as usize {
            assert_eq!(test.data[19][i], 0);
        }
    }

    #[test]
    fn collision_test_one() {
        let mut test: Board = Board {
            rows: 10,
            cols: 20,
            data: vec![vec![0; 10]; 20],
        };

        assert_eq!(test.collision(&I_PIECE), false);
    }

    #[test]
    fn collision_test_two() {
        let mut test: Board = Board {
            rows: 10,
            cols: 20,
            data: vec![vec![0; 10]; 20],
        };

        let mut test_piece: Piece = I_PIECE.clone();
        test_piece.p1.y = BOTTOM_BOUNDARY * UNIT;

        assert_eq!(test.collision(&test_piece), true);
    }

    #[test]
    fn collision_test_three() {
        let mut test: Board = Board {
            rows: 10,
            cols: 20,
            data: vec![vec![0; 10]; 20],
        };

        for i in test.data[10].iter_mut() {
            *i = 1;
        }

        let mut test_piece: Piece = I_PIECE.clone();
        test_piece.p1.y = RIGHT_BOUNDARY * UNIT;

        assert_eq!(test.collision(&test_piece), true);
    }

    #[test]
    fn check_side_collision_test_one() {
        let test: Board = Board {
            rows: 10,
            cols: 20,
            data: vec![vec![0; 10]; 20],
        };

        assert_eq!(test.check_side_collision(5, 5, LEFT), false);
    }

    #[test]
    fn check_side_collision_test_two() {
        let test: Board = Board {
            rows: 10,
            cols: 20,
            data: vec![vec![0; 10]; 20],
        };

        assert_eq!(test.check_side_collision(0, 5, LEFT), true);
    }

    #[test]
    fn check_side_collision_test_three() {
        let mut test: Board = Board {
            rows: 10,
            cols: 20,
            data: vec![vec![0; 10]; 20],
        };

        test.data[0][4] = 1;
        assert_eq!(test.check_side_collision(5, 0, LEFT), true);
    }

    #[test]
    fn check_side_collision_test_four() {
        let test: Board = Board {
            rows: 10,
            cols: 20,
            data: vec![vec![0; 10]; 20],
        };

        assert_eq!(test.check_side_collision(9, 9, RIGHT), true);
    }

    #[test]
    fn check_side_collision_test_five() {
        let mut test: Board = Board {
            rows: 10,
            cols: 20,
            data: vec![vec![0; 10]; 20],
        };

        test.data[5][6] = 1;

        assert_eq!(test.check_side_collision(5, 5, RIGHT), true);
    }

    #[test]
    fn check_rotate_overlap_test_one() {
        let mut test: Board = Board {
            rows: 10,
            cols: 20,
            data: vec![vec![0; 10]; 20],
        };

        let test_piece: Piece = I_PIECE.clone();
        test.data[0][4] = 2;

        assert_eq!(test.check_rotate_overlap(&test_piece), true);
    }

    #[test]
    fn check_rotate_overlap_test_two() {
        let test: Board = Board {
            rows: 10,
            cols: 20,
            data: vec![vec![0; 10]; 20],
        };

        let test_piece: Piece = I_PIECE.clone();

        assert_eq!(test.check_rotate_overlap(&test_piece), false);
    }

    #[test]
    fn check_can_rotate_test_one() {
        let test: Board = Board {
            rows: 10,
            cols: 20,
            data: vec![vec![0; 10]; 20],
        };

        let mut test_piece: Piece = I_PIECE.clone();

        // A normal rotation test
        assert_eq!(test.check_can_rotate(&mut test_piece, LEFT), true);
    }

    #[test]
    fn check_can_rotate_test_two() {
        let test: Board = Board {
            rows: 10,
            cols: 20,
            data: vec![vec![0; 10]; 20],
        };

        let mut test_piece: Piece = I_PIECE.clone();
        while test_piece.p1.y / UNIT != BOTTOM_BOUNDARY {
            test_piece.move_down();
        }

        // Test the rotation when piece reaches the bottom
        assert_eq!(test.check_can_rotate(&mut test_piece, LEFT), false);
    }

    #[test]
    fn check_can_rotate_test_three() {
        // Need to when there's a block on the left, and cannot rotate on the right side
        let mut test: Board = Board {
            rows: 10,
            cols: 20,
            data: vec![vec![0; 10]; 20],
        };

        let mut test_piece: Piece = I_PIECE.clone();
        test_piece.rotate_left();

        while test_piece.p1.x / UNIT < RIGHT_BOUNDARY {
            test_piece.move_right();
        }

        // Insert a random block for testing
        test.data[0][8] = 1;

        assert_eq!(test.check_can_rotate(&mut test_piece, LEFT), false);
    }

    #[test]
    fn check_can_rotate_test_four() {
        // Need to when there's a block on the right, and cannot rotate on the left side
        let mut test: Board = Board {
            rows: 10,
            cols: 20,
            data: vec![vec![0; 10]; 20],
        };

        let mut test_piece: Piece = I_PIECE.clone();
        test_piece.rotate_left();

        while test_piece.p1.x / UNIT > 0.0 {
            test_piece.move_left();
        }

        // Insert a random block for testing
        test.data[0][1] = 1;

        assert_eq!(test.check_can_rotate(&mut test_piece, LEFT), false);
    }
}
