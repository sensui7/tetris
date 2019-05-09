use crate::pieces::*;
use crate::randomizer::PieceTypes::*;

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

		self.data[(p.p1.y / 40.0).round() as usize][(p.p1.x / 40.0).round() as usize] = piece_digit;
		self.data[(p.p2.y / 40.0).round() as usize][(p.p2.x / 40.0).round() as usize] = piece_digit;
		self.data[(p.p3.y / 40.0).round() as usize][(p.p3.x / 40.0).round() as usize] = piece_digit;
		self.data[(p.p4.y / 40.0).round() as usize][(p.p4.x / 40.0).round() as usize] = piece_digit;
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
		if y < 0.0 {
			return ((x / 40.0).round() as usize, 0 as usize);
		}
		return ((x / 40.0).round() as usize, (y / 40.0).round() as usize);
	}

	fn convert_xy_f64(&self, x: f64, y: f64) -> (f64, f64) {
		if y < 0.0 {
			return (x / 40.0, 0.0);
		}
		return (x / 40.0, y / 40.0);
	}

	pub fn collision(&mut self, p: &Piece) -> bool {
		let p1 = self.convert_xy(p.p1.x, p.p1.y);
		let p2 = self.convert_xy(p.p2.x, p.p2.y);
		let p3 = self.convert_xy(p.p3.x, p.p3.y);
		let p4 = self.convert_xy(p.p4.x, p.p4.y);

		/*
		if  p1.1 == 0 || p2.1 == 0 || p3.1 == 0 || p4.1 == 0 {
			return false;
		}
		*/

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

	// Let dir = 0 for left
	// Let dir = 1 for right
	pub fn check_side_collision(&self, row: usize, col: usize, dir: u64) -> bool {
		if dir == 0 && row == 0 {
			return true;
		}

		if dir == 0 && row >= 1 {
			if self.data[col][row - 1] != 0 {
				return true;
			}
		}

		if dir == 1 && row as u64 == self.rows - 1 {
			println!("{}, {}", row, col);
			return true;
		}

		if dir == 1 && (row as u64) < self.rows - 1 {
			if self.data[col][row + 1] != 0 {
				return true;
			}
		}
		
		false
	}

	pub fn check_overlap(&self, p: &Piece, dir: u64) -> bool {
		let p1 = self.convert_xy(p.p1.x, p.p1.y);
		let p2 = self.convert_xy(p.p2.x, p.p2.y);
		let p3 = self.convert_xy(p.p3.x, p.p3.y);
		let p4 = self.convert_xy(p.p4.x, p.p4.y);

		return self.check_side_collision(p1.0, p1.1, dir) || 
			   self.check_side_collision(p2.0, p2.1, dir) || 
			   self.check_side_collision(p3.0, p3.1, dir) || 
			   self.check_side_collision(p4.0, p4.1, dir);
	}

	pub fn check_rotate_overlap(&self, p: &Piece) -> bool {
		let p1 = self.convert_xy(p.p1.x, p.p1.y);
		let p2 = self.convert_xy(p.p2.x, p.p2.y);
		let p3 = self.convert_xy(p.p3.x, p.p3.y);
		let p4 = self.convert_xy(p.p4.x, p.p4.y);

		// https://doc.rust-lang.org/std/iter/struct.Enumerate.html
		for (i, col) in self.data.iter().enumerate() {
			for (j, row) in col.iter().enumerate() {
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
	
	pub fn check_can_rotate(&self, p: &mut Piece) -> bool {
		let p1 = self.convert_xy_f64(p.p1.x, p.p1.y);
		let p2 = self.convert_xy_f64(p.p2.x, p.p2.y);
		let p3 = self.convert_xy_f64(p.p3.x, p.p3.y);
		let p4 = self.convert_xy_f64(p.p4.x, p.p4.y);

		let mut test = p.clone();
		test.rotate_left();

		if self.check_rotate_overlap(&test) == true {
			return false;
		}

		match p.piece_type {
			T => {
				if p.rotation == 3 && p3.0 == 9.0 {
					p.move_left();
				} else if p.rotation == 1 && p3.0 == 0.0 {
					p.move_right();
				}
			},
			I => {
				if p.rotation == 1 && p3.0 == 8.0 {
					p.move_left();
				} else if p.rotation == 1 && p3.0 == 9.0 {
					p.move_left();
					p.move_left();
				} else if p.rotation == 1 && p3.0 == 0.0 {
					p.move_right();
				}
			},
			O => {
				return true;
			},
			L => {
				if p.rotation == 0 && p4.0 == 9.0 {
					p.move_left();
				} else if p.rotation == 2 && p4.0 <= 1.0 {
					p.move_right();
				}
			},
			J => {
				if p.rotation == 0 && p1.0 == 8.0 {
					p.move_left();
				} else if p.rotation == 0 && p1.0 == 9.0 {
					p.move_left();
					p.move_left();
				} else if p.rotation == 2 && p1.0 == 1.0 {
					p.move_right();
				} else if p.rotation == 2 && p1.0 == 0.0 {
					p.move_right();
					p.move_right();
				}
			},
			Z => {
				if p.rotation == 1 && p2.0 == 9.0 {
					p.move_left();
				}
			},
			S => {
				if p.rotation == 1 && p3.0 == 9.0 {
					p.move_left();
				}
			}
		}

		true
	}

}
