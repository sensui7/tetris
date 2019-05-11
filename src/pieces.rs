use crate::WIDTH;
use crate::randomizer::*;
use crate::randomizer::PieceTypes::*;
use std::fmt;

#[derive(Clone)]
pub struct Point {
	pub x: f64,
	pub y: f64
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

#[derive(Clone)]
pub struct Piece {
//	pub piece_type: String,
	pub piece_type: PieceTypes,
	pub rotation: u64,
	pub p1: Point,
	pub p2: Point,
	pub p3: Point,
	pub p4: Point
}

impl fmt::Display for Piece {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "hi")
    }
}


impl Piece {
	pub fn move_left(&mut self) {
		if self.p1.x > 0.0 && self.p2.x > 0.0 && self.p3.x > 0.0 && self.p4.x > 0.0 {
			self.p1.x -= 40.0;
			self.p2.x -= 40.0;
			self.p3.x -= 40.0;
			self.p4.x -= 40.0;
		}
	}

	pub fn move_right(&mut self) {
		if self.p1.x < WIDTH - 40.0 && self.p2.x < WIDTH - 40.0 && self.p3.x < WIDTH - 40.0 && self.p4.x < WIDTH - 40.0 {
			self.p1.x += 40.0;
			self.p2.x += 40.0;
			self.p3.x += 40.0;
			self.p4.x += 40.0;
		}
	}

	pub fn move_down(&mut self) {
		// gotta check if collision first
		// if collision, then need some sort of a next block flag
		self.p1.y += 40.0;
		self.p2.y += 40.0;
		self.p3.y += 40.0;
		self.p4.y += 40.0;
	}

	pub fn rotate_t(&mut self) {
		self.p3.x = self.p4.x;
		self.p3.y = self.p4.y;
		self.p4.x = self.p2.x;
		self.p4.y = self.p2.y;
	}

	pub fn rotate_t_right(&mut self) {
		self.p2.x = self.p4.x;
		self.p2.y = self.p4.y;
		self.p4.x = self.p3.x;
		self.p4.y = self.p3.y;
	}

	pub fn rotate_right(&mut self) {
		if self.piece_type == T {
			match self.rotation {
				0 => {
					self.rotate_t_right();				
					self.p3.x = self.p1.x;	
					self.p3.y = self.p1.y + 40.0;	
					self.rotation = 3;
				},
				3 => {
					self.rotate_t_right();
					self.p3.x = self.p1.x + 40.0;	
					self.p3.y = self.p1.y;	
					self.rotation = 2;
				},
				2 => {
					self.rotate_t_right();
					self.p3.x = self.p1.x;	
					self.p3.y = self.p1.y - 40.0;	
					self.rotation = 1;
				},
				1 => {
					self.rotate_t_right();
					self.p3.x = self.p1.x - 40.0;	
					self.p3.y = self.p1.y;	
					self.rotation = 0;
				}
				_ => println!("error in rotating right")
			}
		}
		if self.piece_type == I {
			match self.rotation {
				0 => { 
					self.p1.x = self.p3.x;
					self.p1.y = self.p3.y + 80.0;
					self.p2.x = self.p3.x;
					self.p2.y = self.p3.y + 40.0;
					self.p4.x = self.p3.x;
					self.p4.y = self.p3.y - 40.0;
					self.rotation = 1;
				},
				1 => {
					self.p1.x = self.p3.x - 80.0;
					self.p1.y = self.p3.y;
					self.p2.x = self.p3.x - 40.0;
					self.p2.y = self.p3.y;
					self.p4.x = self.p3.x + 40.0;
					self.p4.y = self.p3.y;
					self.rotation = 0;
				},
				_ => println!("error")
			}
		}
		if self.piece_type == Z {
			match self.rotation {
				0 => {
					self.p2 = self.p4.clone();
					self.p4.x = self.p3.x;
					self.p4.y = self.p3.y + 40.0;
					self.p1.x = self.p2.x;
					self.p1.y = self.p2.y - 40.0;
					self.rotation = 3;
				},
				3 => {
					self.p2 = self.p4.clone();
					self.p4.x = self.p3.x - 40.0;
					self.p4.y = self.p3.y;
					self.p1.x = self.p2.x + 40.0;
					self.p1.y = self.p2.y;
					self.rotation = 2;
				},
				2 => {
					self.p2 = self.p4.clone();
					self.p4.x = self.p3.x;
					self.p4.y = self.p3.y - 40.0;
					self.p1.x = self.p2.x;
					self.p1.y = self.p2.y + 40.0;
					self.rotation = 1;
				},
				1 => {
					self.p2 = self.p4.clone();
					self.p4.x = self.p3.x + 40.0;
					self.p4.y = self.p3.y;
					self.p1.x = self.p2.x - 40.0;
					self.p1.y = self.p2.y;
					self.rotation = 0;
				}
				_ => println!("error")
			}
		}
		if self.piece_type == S {
			match self.rotation {
				0 => { 
					self.p4 = self.p2.clone();
					self.p2.x = self.p3.x - 40.0;
					self.p2.y = self.p3.y;
					self.p1.x = self.p2.x;
					self.p1.y = self.p2.y - 40.0;
					self.rotation = 3;
				},
				3 => {
					self.p4 = self.p2.clone();
					self.p2.x = self.p3.x;
					self.p2.y = self.p3.y - 40.0;
					self.p1.x = self.p2.x + 40.0;
					self.p1.y = self.p2.y;
					self.rotation = 2;
				},
				2 => {
					self.p4 = self.p2.clone();
					self.p2.x = self.p3.x + 40.0;
					self.p2.y = self.p3.y;
					self.p1.x = self.p2.x;
					self.p1.y = self.p2.y + 40.0;
					self.rotation = 1;
				},
				1 => {
					self.p4 = self.p2.clone();
					self.p2.x = self.p3.x;
					self.p2.y = self.p3.y + 40.0;
					self.p1.x = self.p2.x - 40.0;
					self.p1.y = self.p2.y;
					self.rotation = 0;
				},
				_ => println!("error")
			}
		}
		if self.piece_type == L {
			match self.rotation {
				0 => { 
					self.p4.x = self.p3.x;
					self.p4.y = self.p3.y;
					self.p3.x = self.p2.x;
					self.p3.y = self.p2.y;
					self.p2.x = self.p3.x + 40.0;
					self.p2.y = self.p3.y;
					self.p1.x = self.p2.x + 40.0;
					self.p1.y = self.p2.y;
					self.rotation = 3;
				},
				3 => {
					self.p4.x = self.p3.x;
					self.p4.y = self.p3.y;
					self.p3.x = self.p2.x;
					self.p3.y = self.p2.y;
					self.p2.x = self.p3.x;
					self.p2.y = self.p3.y + 40.0;
					self.p1.x = self.p2.x;
					self.p1.y = self.p2.y + 40.0;
					self.rotation = 2;
				},
				2 => {
					self.p4.x = self.p3.x;
					self.p4.y = self.p3.y;
					self.p3.x = self.p2.x;
					self.p3.y = self.p2.y;
					self.p2.x = self.p3.x - 40.0;
					self.p2.y = self.p3.y;
					self.p1.x = self.p2.x - 40.0;
					self.p1.y = self.p2.y;
					self.rotation = 1;
				},
				1 => {
					self.p4.x = self.p3.x;
					self.p4.y = self.p3.y;
					self.p3.x = self.p2.x;
					self.p3.y = self.p2.y;
					self.p2.x = self.p3.x;
					self.p2.y = self.p3.y - 40.0;
					self.p1.x = self.p2.x;
					self.p1.y = self.p2.y - 40.0;
					self.rotation = 0;
				},
				_ => println!("error")
			}
		}
		if self.piece_type == J {
			match self.rotation {
				0 => { 
					self.p4.x = self.p2.x;
					self.p4.y = self.p2.y;
					self.p2.y = self.p3.y;
					self.p2.x = self.p3.x + 40.0;
					self.p1.y = self.p2.y;
					self.p1.x = self.p2.x + 40.0;
					self.rotation = 3;
				},
				3 => {
					self.p4.x = self.p2.x;
					self.p4.y = self.p2.y;
					self.p2.y = self.p3.y + 40.0;
					self.p2.x = self.p3.x;
					self.p1.y = self.p2.y + 40.0;
					self.p1.x = self.p2.x;
					self.rotation = 2;
				},
				2 => {
					self.p4.x = self.p2.x;
					self.p4.y = self.p2.y;
					self.p2.y = self.p3.y;
					self.p2.x = self.p3.x - 40.0;
					self.p1.y = self.p2.y;
					self.p1.x = self.p2.x - 40.0;
					self.rotation = 1;
				},
				1 => {
					self.p4.x = self.p2.x;
					self.p4.y = self.p2.y;
					self.p2.y = self.p3.y - 40.0;
					self.p2.x = self.p3.x;
					self.p1.y = self.p2.y - 40.0;
					self.p1.x = self.p2.x;
					self.rotation = 0;
				},
				_ => println!("error")
			}
		}
	}

	pub fn rotate_left(&mut self) {
		if self.piece_type == T {
			match self.rotation {
				0 => { 
					self.rotate_t();
					self.p2.x = self.p1.x;
					self.p2.y = self.p1.y + 40.0;
					self.rotation = 1;
				},
				1 => {
					self.rotate_t();
					self.p2.x = self.p1.x - 40.0;
					self.p2.y = self.p1.y;
					self.rotation = 2;
				},
				2 => {
					self.rotate_t();
					self.p2.x = self.p1.x;		
					self.p2.y = self.p1.y - 40.0;		
					self.rotation = 3;
				},
				3 => {
					self.rotate_t();
					self.p2.x = self.p1.x + 40.0;
					self.p2.y = self.p2.y + 40.0;
					self.rotation = 0;
				},
				_ => println!("error")
			}
		}
		if self.piece_type == I {
			match self.rotation {
				0 => { 
					self.p1.x = self.p3.x;
					self.p1.y = self.p3.y - 80.0;
					self.p2.x = self.p3.x;
					self.p2.y = self.p3.y - 40.0;
					self.p4.x = self.p3.x;
					self.p4.y = self.p3.y + 40.0;
					self.rotation = 1;
				},
				1 => {
					self.p1.x = self.p3.x + 80.0;
					self.p1.y = self.p3.y;
					self.p2.x = self.p3.x + 40.0;
					self.p2.y = self.p3.y;
					self.p4.x = self.p3.x - 40.0;
					self.p4.y = self.p3.y;
					self.rotation = 0;
				},
				_ => println!("error")
			}
		}
		if self.piece_type == Z {
			match self.rotation {
				0 => { 
					self.p4 = self.p2.clone();
					self.p2.x = self.p3.x - 40.0;
					self.p2.y = self.p3.y;
					self.p1.x = self.p2.x;
					self.p1.y = self.p2.y + 40.0;
					self.rotation = 1;
				},
				1 => {
					self.p4 = self.p2.clone();
					self.p2.x = self.p3.x;
					self.p2.y = self.p3.y + 40.0;
					self.p1.x = self.p2.x + 40.0;
					self.p1.y = self.p2.y;
					self.rotation = 2;
				},
				2 => {
					self.p4 = self.p2.clone();
					self.p2.x = self.p3.x + 40.0;
					self.p2.y = self.p3.y;
					self.p1.x = self.p2.x;
					self.p1.y = self.p2.y - 40.0;
					self.rotation = 3;
				},
				3 => {
					self.p4 = self.p2.clone();
					self.p2.x = self.p3.x;
					self.p2.y = self.p3.y - 40.0;
					self.p1.x = self.p2.x - 40.0;
					self.p1.y = self.p2.y;
					self.rotation = 0;
				},
				_ => println!("error")
			}
		}
		if self.piece_type == S {
			match self.rotation {
				0 => { 
					self.p2 = self.p4.clone();
					self.p4.x = self.p3.x;
					self.p4.y = self.p3.y - 40.0;
					self.p1.x = self.p2.x;
					self.p1.y = self.p2.y + 40.0;
					self.rotation = 1;
				},
				1 => {
					self.p2 = self.p4.clone();
					self.p4.x = self.p3.x - 40.0;
					self.p4.y = self.p3.y;
					self.p1.x = self.p2.x + 40.0;
					self.p1.y = self.p2.y;
					self.rotation = 2;
				},
				2 => {
					self.p2 = self.p4.clone();
					self.p4.x = self.p3.x;
					self.p4.y = self.p3.y + 40.0;
					self.p1.x = self.p2.x;
					self.p1.y = self.p2.y - 40.0;
					self.rotation = 3;
				},
				3 => {
					self.p2 = self.p4.clone();
					self.p4.x = self.p3.x + 40.0;
					self.p4.y = self.p3.y;
					self.p1.x = self.p2.x - 40.0;
					self.p1.y = self.p2.y;
					self.rotation = 0;
				},
				_ => println!("error")
			}
		}
		if self.piece_type == L {
			match self.rotation {
				0 => { 
					self.p4 = self.p2.clone();
					self.p2.x = self.p3.x - 40.0;
					self.p2.y = self.p3.y;
					self.p1.x = self.p2.x - 40.0;
					self.p1.y = self.p2.y;
					self.rotation = 1;
				},
				1 => {
					self.p4 = self.p2.clone();
					self.p2.x = self.p3.x;
					self.p2.y = self.p3.y + 40.0;
					self.p1.x = self.p2.x;
					self.p1.y = self.p2.y + 40.0;
					self.rotation = 2;
				},
				2 => {
					self.p4 = self.p2.clone();
					self.p2.x = self.p3.x + 40.0;
					self.p2.y = self.p3.y;
					self.p1.x = self.p2.x + 40.0;
					self.p1.y = self.p2.y;
					self.rotation = 3;
				},
				3 => {
					self.p4 = self.p2.clone();
					self.p2.x = self.p3.x;
					self.p2.y = self.p3.y - 40.0;
					self.p1.x = self.p2.x;
					self.p1.y = self.p2.y - 40.0;
					self.rotation = 0;
				},
				_ => println!("error")
			}
		}
		if self.piece_type == J {
			match self.rotation {
				0 => { 
					self.p4 = self.p3.clone();
					self.p3 = self.p2.clone();
					self.p2.y = self.p3.y;
					self.p2.x = self.p3.x - 40.0;
					self.p1.y = self.p2.y;
					self.p1.x = self.p2.x - 40.0;
					self.rotation = 1;
				},
				1 => {
					self.p4 = self.p3.clone();
					self.p3 = self.p2.clone();
					self.p2.y = self.p3.y + 40.0;
					self.p2.x = self.p3.x;
					self.p1.y = self.p2.y + 40.0;
					self.p1.x = self.p2.x;
					self.rotation = 2;
				},
				2 => {
					self.p4 = self.p3.clone();
					self.p3 = self.p2.clone();
					self.p2.y = self.p3.y;
					self.p2.x = self.p3.x + 40.0;
					self.p1.y = self.p2.y;
					self.p1.x = self.p2.x + 40.0;
					self.rotation = 3;
				},
				3 => {
					self.p4 = self.p3.clone();
					self.p3 = self.p2.clone();
					self.p2.y = self.p3.y - 40.0;
					self.p2.x = self.p3.x;
					self.p1.y = self.p2.y - 40.0;
					self.p1.x = self.p2.x;
					self.rotation = 0;
				},
				_ => println!("error")
			}
		}
	}
}
