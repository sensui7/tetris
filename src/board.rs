use crate::pieces::*;
use crate::randomizer::PieceTypes::*;
use crate::pieces::Dir::*;
use crate::pieces::UNIT;

const LEFT_BOUNDARY: f64 = 0.0;
const RIGHT_BOUNDARY: f64 = 9.0;
const BOTTOM_BOUNDARY: f64 = 19.0;

#[derive(PartialEq, Clone, Debug)]
pub struct Board {
	pub rows: u64,
	pub cols: u64,
	pub data: Vec<Vec<u64>>
}

impl Board {
	// Called when it lands on bottom/collides
	pub fn set(&mut self, p: &Piece) {
		let piece_digit: u64;
		match p.piece_type {
			T => { piece_digit = 1 },
			I => { piece_digit = 2 },
			O => { piece_digit = 3 },
			L => { piece_digit = 4 },
			J => { piece_digit = 5 },
			Z => { piece_digit = 6 },
			S => { piece_digit = 7 }
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

		for j in new_data {
			self.data.remove(j);
			self.data.insert(0 as usize, empty.clone());
		}
	}

	pub fn display(&self) {
		for i in self.data.iter() {
			for j in i {
				print!("{}", j);
			}
			println!("");
		}
	}

	pub fn convert_xy(&self, x: f64, y: f64) -> (usize, usize) {
		if y < LEFT_BOUNDARY {
			return ((x / UNIT).round() as usize, 0 as usize);
		}
		return ((x / UNIT).round() as usize, (y / UNIT).round() as usize);
	}

	pub fn collision(&mut self, p: &Piece) -> bool {
		let p1 = self.convert_xy(p.p1.x, p.p1.y);
		let p2 = self.convert_xy(p.p2.x, p.p2.y);
		let p3 = self.convert_xy(p.p3.x, p.p3.y);
		let p4 = self.convert_xy(p.p4.x, p.p4.y);

		if p1.1 >= 19 || p2.1 >= 19 || p3.1 >= 19 || p4.1 >= 19 {
			return true;
		}

		if self.check_piece_collision(p1.0, p1.1) == true ||
		   self.check_piece_collision(p2.0, p2.1) == true ||
		   self.check_piece_collision(p3.0, p3.1) == true ||
		   self.check_piece_collision(p4.0, p4.1) == true {
			return true;
		}

		return false;
	}

	pub fn check_piece_collision(&self, row: usize, col: usize) -> bool {
		//println!("row: {}, col: {}", row, col);
		if self.data[col + 1][row] != 0 {
			return true;
		}

		false
	}

	pub fn check_side_collision(&self, row: usize, col: usize, dir: Dir) -> bool {
		if dir == LEFT && row == 0 {
			return true;
		}

		if dir == LEFT && row >= 1 {
			if self.data[col][row - 1] != 0 {
				return true;
			}
		}

		if dir == RIGHT && row as u64 == self.rows - 1 {
			return true;
		}

		if dir == RIGHT && (row as u64) < self.rows - 1 {
			if self.data[col][row + 1] != 0 {
				return true;
			}
		}
		
		false
	}

	// Check if a piece overlaps with another piece via side collision
	pub fn check_overlap(&self, p: &Piece, dir: Dir) -> bool {
		let p1 = self.convert_xy(p.p1.x, p.p1.y);
		let p2 = self.convert_xy(p.p2.x, p.p2.y);
		let p3 = self.convert_xy(p.p3.x, p.p3.y);
		let p4 = self.convert_xy(p.p4.x, p.p4.y);

		return self.check_side_collision(p1.0, p1.1, dir) || 
			   self.check_side_collision(p2.0, p2.1, dir) || 
			   self.check_side_collision(p3.0, p3.1, dir) || 
			   self.check_side_collision(p4.0, p4.1, dir);
	}

	// Check if a piece overlaps with another piece before rotating
	pub fn check_rotate_overlap(&self, p: &Piece) -> bool {
		let p1 = self.convert_xy(p.p1.x, p.p1.y);
		let p2 = self.convert_xy(p.p2.x, p.p2.y);
		let p3 = self.convert_xy(p.p3.x, p.p3.y);
		let p4 = self.convert_xy(p.p4.x, p.p4.y);

		/* debugging purposes
		println!("p1 {}, {}", p1.0, p1.1);
		println!("p2 {}, {}", p2.0, p2.1);
		println!("p3 {}, {}", p3.0, p3.1);
		println!("p4 {}, {}", p4.0, p4.1);
		println!("");
		*/

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

	// Check if a piece can rotate left/right
	pub fn check_can_rotate(&self, p: &mut Piece, dir: Dir) -> bool {
		let mut test = p.clone();
	
		if dir == LEFT  { test.rotate_left(); }
		if dir == RIGHT { test.rotate_right(); }

		if self.check_rotate_overlap(&test) == true {
			return false;
		}

		let mut c = 0;

		while (test.p1.x / UNIT) > RIGHT_BOUNDARY || (test.p2.x / UNIT) > RIGHT_BOUNDARY || (test.p3.x / UNIT) > RIGHT_BOUNDARY || (test.p4.x / UNIT) > RIGHT_BOUNDARY {
			if self.check_overlap(p, LEFT) != true {
				test.move_left();
				c += 1;
			}
			else {
				return false;
			}
		}

		for _x in 0..c {
			if self.check_overlap(p, LEFT) != true {
				p.move_left();	
			}
		}

		c = 0;

		while (test.p1.x / UNIT) < LEFT_BOUNDARY || (test.p2.x / UNIT) < LEFT_BOUNDARY || (test.p3.x / UNIT) < LEFT_BOUNDARY || (test.p4.x / UNIT) < LEFT_BOUNDARY {
			if self.check_overlap(p, RIGHT) != true {
				test.move_right();
				c += 1;
			}
			else {
				return false;
			}
		}

		for _x in 0..c {
			if self.check_overlap(p, RIGHT) != true {
				p.move_right();	
			}
		}


		// Check bottom pit
		if (test.p4.y / UNIT) > BOTTOM_BOUNDARY || (test.p3.y / UNIT) > BOTTOM_BOUNDARY || (test.p2.y / UNIT) > BOTTOM_BOUNDARY || (test.p1.y / UNIT) > BOTTOM_BOUNDARY {
			return false;
		}
		
		true

	}
}

#[cfg(test)]
mod tests {

	use crate::board::Board;
	use crate::randomizer::I_PIECE;

	#[test]
	fn set_test() {
		let mut test: Board = Board {
			rows: 10,
			cols: 20,
			data: vec![vec![0; 10]; 20]
		};

		let test_piece = I_PIECE;
		test.set(&test_piece);

		assert_eq!(test.data[0][3], 2);
		assert_eq!(test.data[0][4], 2);
		assert_eq!(test.data[0][5], 2);
		assert_eq!(test.data[0][6], 2);
	}

	#[test]
	fn clear_test() {
		let mut test: Board = Board {
			rows: 10,
			cols: 20,
			data: vec![vec![0; 10]; 20]
		};

		for i in test.data[19].iter_mut() {
			*i = 1;	
		}

		test.clear();

		for i in 0..9 {
			assert_eq!(test.data[19][i], 0);
		}
	}

	#[test]
	fn collision_test() {

	}

	#[test]
	fn check_side_collision() {

	}

	#[test]
	fn check_overlap() {

	}

	#[test]
	fn check_rotate_overlap() {

	}

	#[test]
	fn check_can_rotate_overlap() {

	}
}
