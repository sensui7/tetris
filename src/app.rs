use crate::pieces::*;
use crate::board::*;
use crate::randomizer::*;
use crate::randomizer::PieceTypes::*;
use crate::pieces::Dir::*;
use crate::Sound;
use piston::input::*;
use opengl_graphics::{ GlGraphics };
use music;

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend
	pub board: Board,
	pub piece: Piece,
	pub time: f64,
	pub randomizer: Randomizer,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

		const GREY: 	 [f32; 4] = [0.5, 0.5, 0.5, 0.8];
		const GREEN: 	 [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   	 [f32; 4] = [1.0, 0.0, 0.0, 1.0];
		const DARK_BLUE: [f32; 4] = [0.0, 0.0, 0.5, 1.0];
		const ORANGE: 	 [f32; 4] = [1.0, 0.6, 0.0, 1.0];
		const YELLOW: 	 [f32; 4] = [1.0, 0.8, 0.0, 1.0];
		const TEAL: 	 [f32; 4] = [0.0, 1.0, 1.0, 1.0];
		const PURPLE: 	 [f32; 4] = [1.0, 0.0, 1.0, 1.0];
		const BLACK: 	 [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: 	 [f32; 4] = [1.0, 1.0, 1.0, 1.0];

		// if next block flag is true, generate a new piece here
        let square1 = rectangle::square(self.piece.p1.x, self.piece.p1.y, UNIT);
        let square2 = rectangle::square(self.piece.p2.x, self.piece.p2.y, UNIT);
        let square3 = rectangle::square(self.piece.p3.x, self.piece.p3.y, UNIT);
        let square4 = rectangle::square(self.piece.p4.x, self.piece.p4.y, UNIT);

		let mut shadow: Piece = self.piece.clone();

		while self.board.collision(&shadow) != true {
			shadow.move_down();
		}
        let s1 = rectangle::square(shadow.p1.x, shadow.p1.y, UNIT);
        let s2 = rectangle::square(shadow.p2.x, shadow.p2.y, UNIT);
        let s3 = rectangle::square(shadow.p3.x, shadow.p3.y, UNIT);
        let s4 = rectangle::square(shadow.p4.x, shadow.p4.y, UNIT);

		let grid = grid::Grid { cols: 10, rows: 20, units: UNIT };
		let line = line::Line::new(WHITE, 0.8);
		let history: &Vec<Vec<u64>> = &self.board.data;
		let curr_piece: &PieceTypes = &self.piece.piece_type;

		let mut bag = self.randomizer.bag.clone();
		if bag.len() == 0 {
			self.randomizer.generate();
			bag = self.randomizer.bag.clone();
		}

		let next_piece = bag.iter_mut().rev().next().unwrap();
		let next_grid = grid::Grid { cols: 4, rows: 4, units: UNIT };

		// Place the future pieces into the box nicely
		match next_piece.piece_type {
			T => next_piece.move_down(),
			I => next_piece.rotate_left(),
			O => next_piece.move_down(),
			S => {},
			Z => {},
			L => {},
			J => {}
		}
		
		// Squares used to draw next piece
		let n1 = rectangle::square(next_piece.p1.x + (UNIT * 8.0), next_piece.p1.y + UNIT * 2.0, UNIT);
		let n2 = rectangle::square(next_piece.p2.x + (UNIT * 8.0), next_piece.p2.y + UNIT * 2.0, UNIT);
		let n3 = rectangle::square(next_piece.p3.x + (UNIT * 8.0), next_piece.p3.y + UNIT * 2.0, UNIT);
		let n4 = rectangle::square(next_piece.p4.x + (UNIT * 8.0), next_piece.p4.y + UNIT * 2.0, UNIT);

        self.gl.draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {
            // Clear the screen.
            clear(BLACK, gl);

			// render shadow
			rectangle(GREY, s1, c.transform, gl);
			rectangle(GREY, s2, c.transform, gl);
			rectangle(GREY, s3, c.transform, gl);
			rectangle(GREY, s4, c.transform, gl);

			match curr_piece {
				T => {
					rectangle(PURPLE, square1, c.transform, gl);
					rectangle(PURPLE, square2, c.transform, gl);
					rectangle(PURPLE, square3, c.transform, gl);
					rectangle(PURPLE, square4, c.transform, gl);
				},
				I => {
					rectangle(TEAL, square1, c.transform, gl);
					rectangle(TEAL, square2, c.transform, gl);
					rectangle(TEAL, square3, c.transform, gl);
					rectangle(TEAL, square4, c.transform, gl);
				},
				O => {
					rectangle(YELLOW, square1, c.transform, gl);
					rectangle(YELLOW, square2, c.transform, gl);
					rectangle(YELLOW, square3, c.transform, gl);
					rectangle(YELLOW, square4, c.transform, gl);
				},
				L => {
					rectangle(ORANGE, square1, c.transform, gl);
					rectangle(ORANGE, square2, c.transform, gl);
					rectangle(ORANGE, square3, c.transform, gl);
					rectangle(ORANGE, square4, c.transform, gl);
				},
				J => {
					rectangle(DARK_BLUE, square1, c.transform, gl);
					rectangle(DARK_BLUE, square2, c.transform, gl);
					rectangle(DARK_BLUE, square3, c.transform, gl);
					rectangle(DARK_BLUE, square4, c.transform, gl);
				}
				Z => {
					rectangle(RED, square1, c.transform, gl);
					rectangle(RED, square2, c.transform, gl);
					rectangle(RED, square3, c.transform, gl);
					rectangle(RED, square4, c.transform, gl);
				},
				S => {
					rectangle(GREEN, square1, c.transform, gl);
					rectangle(GREEN, square2, c.transform, gl);
					rectangle(GREEN, square3, c.transform, gl);
					rectangle(GREEN, square4, c.transform, gl);
				}
			}

			// render grid history of other blocks
			for (i, col) in history.iter().enumerate() {
				for (j, _row) in col.iter().enumerate() {
					let sq = rectangle::square((j * UNIT as usize) as f64, (i * UNIT as usize) as f64, UNIT);
					match history[i][j] {
						0 => {}
						1 => rectangle(PURPLE, sq, c.transform, gl),
						2 => rectangle(TEAL, sq, c.transform, gl),
						3 => rectangle(YELLOW, sq, c.transform, gl),
						4 => rectangle(ORANGE, sq, c.transform, gl),
						5 => rectangle(DARK_BLUE, sq, c.transform, gl),
						6 => rectangle(RED, sq, c.transform, gl),
						7 => rectangle(GREEN, sq, c.transform, gl),
						_ => println!("Error: something went wrong with rendering the block history")
					}
				}
			}

			grid.draw(&line, &c.draw_state, c.transform, gl);

			match next_piece.piece_type {
				T => {
					rectangle(PURPLE, n1, c.transform, gl);
					rectangle(PURPLE, n2, c.transform, gl);
					rectangle(PURPLE, n3, c.transform, gl);
					rectangle(PURPLE, n4, c.transform, gl);
				},
				I => {
					rectangle(TEAL, n1, c.transform, gl);
					rectangle(TEAL, n2, c.transform, gl);
					rectangle(TEAL, n3, c.transform, gl);
					rectangle(TEAL, n4, c.transform, gl);
				},
				O => {
					rectangle(YELLOW, n1, c.transform, gl);
					rectangle(YELLOW, n2, c.transform, gl);
					rectangle(YELLOW, n3, c.transform, gl);
					rectangle(YELLOW, n4, c.transform, gl);
				},
				L => {
					rectangle(ORANGE, n1, c.transform, gl);
					rectangle(ORANGE, n2, c.transform, gl);
					rectangle(ORANGE, n3, c.transform, gl);
					rectangle(ORANGE, n4, c.transform, gl);
				},
				J => {
					rectangle(DARK_BLUE, n1, c.transform, gl);
					rectangle(DARK_BLUE, n2, c.transform, gl);
					rectangle(DARK_BLUE, n3, c.transform, gl);
					rectangle(DARK_BLUE, n4, c.transform, gl);
				}
				Z => {
					rectangle(RED, n1, c.transform, gl);
					rectangle(RED, n2, c.transform, gl);
					rectangle(RED, n3, c.transform, gl);
					rectangle(RED, n4, c.transform, gl);
				},
				S => {
					rectangle(GREEN, n1, c.transform, gl);
					rectangle(GREEN, n2, c.transform, gl);
					rectangle(GREEN, n3, c.transform, gl);
					rectangle(GREEN, n4, c.transform, gl);
				}
			}

			next_grid.draw(&line, &c.draw_state, c.transform.trans(0.0, 0.0).trans(330.0, 30.0), gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
		if self.time - args.dt >= 0.8 {
			if self.board.collision(&self.piece) == false {
				self.piece.move_down();
			}
			else {
				self.board.set(&self.piece);
				self.piece = self.randomizer.get();
				self.board.clear();
			}

			self.time = 0.0;
		}

		self.time += args.dt;
    }

	pub fn input(&mut self, button: &Button, is_press: bool) {
		if is_press {
			if let Button::Keyboard(key) = button {
				match key {
					Key::Space => {
						while self.board.collision(&self.piece) != true {
							self.piece.move_down();
							self.time = 1.0;
							music::play_sound(&Sound::Drop, music::Repeat::Times(0), 0.10);
						}
					},
					Key::Up => {
						if self.board.check_can_rotate(&mut self.piece, LEFT) == true {
							self.piece.rotate_left();
						}
					},
					Key::Left => {
						if self.board.check_overlap(&self.piece, LEFT) != true {
							self.piece.move_left();
						}
					},
					Key::Right => {
						if self.board.check_overlap(&self.piece, RIGHT) != true {
							self.piece.move_right();
						}
					},
					Key::Down => {
						if self.board.collision(&self.piece) != true {
							self.piece.move_down();
						}
					},
					Key::Z => {
						if self.board.check_can_rotate(&mut self.piece, LEFT) == true {
							self.piece.rotate_left();
						}
					},
					Key::X => {
						if self.board.check_can_rotate(&mut self.piece, RIGHT) == true {
							self.piece.rotate_right();
						}
					},
					_ => ()
				}
			}
		}
	}
}
