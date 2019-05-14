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
    pub gl: GlGraphics, // OpenGL drawing backend.
	pub board: Board,
	pub piece: Piece,
	pub time: f64,
	pub randomizer: Randomizer
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

		const GREY: 	 [f32; 4] = [0.5, 0.5, 0.5, 0.8];
		const GREEN: 	 [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   	 [f32; 4] = [1.0, 0.0, 0.0, 1.0];
		const DARK_BLUE: [f32; 4] = [0.0, 0.0, 0.5, 1.0];
		const ORANGE: 	 [f32; 4] = [1.0, 0.6, 0.0, 1.0];
		const YELLOW: 	 [f32; 4] = [1.0, 1.0, 0.0, 1.0];
		const TEAL: 	 [f32; 4] = [0.0, 1.0, 1.0, 1.0];
		const PURPLE: 	 [f32; 4] = [1.0, 0.0, 1.0, 1.0];
		const BLACK: 	 [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: 	 [f32; 4] = [1.0, 1.0, 1.0, 1.0];

		// if next block flag is true, generate a new piece here
        let square1 = rectangle::square(self.piece.p1.x, self.piece.p1.y, 40.0);
        let square2 = rectangle::square(self.piece.p2.x, self.piece.p2.y, 40.0);
        let square3 = rectangle::square(self.piece.p3.x, self.piece.p3.y, 40.0);
        let square4 = rectangle::square(self.piece.p4.x, self.piece.p4.y, 40.0);

		let mut shadow: Piece = self.piece.clone();

		while self.board.collision(&shadow) != true {
			shadow.move_down();
		}
        let s1 = rectangle::square(shadow.p1.x, shadow.p1.y, 40.0);
        let s2 = rectangle::square(shadow.p2.x, shadow.p2.y, 40.0);
        let s3 = rectangle::square(shadow.p3.x, shadow.p3.y, 40.0);
        let s4 = rectangle::square(shadow.p4.x, shadow.p4.y, 40.0);

		let grid = grid::Grid { cols: 10, rows: 20, units: 40.0 };
		let line = line::Line::new(WHITE, 1.0);
		let history: &Vec<Vec<u64>> = &self.board.data;
		let curr_piece: &PieceTypes = &self.piece.piece_type;

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
					let sq = rectangle::square((j * 40) as f64, (i * 40) as f64, 40.0);
					match history[i][j] {
						0 => {}
						1 => rectangle(PURPLE, sq, c.transform, gl),
						2 => rectangle(TEAL, sq, c.transform, gl),
						3 => rectangle(YELLOW, sq, c.transform, gl),
						4 => rectangle(ORANGE, sq, c.transform, gl),
						5 => rectangle(DARK_BLUE, sq, c.transform, gl),
						6 => rectangle(RED, sq, c.transform, gl),
						7 => rectangle(GREEN, sq, c.transform, gl),
						_ => println!("error")
					}
				}
			}

			grid.draw(&line, &c.draw_state, c.transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
		if self.time - args.dt >= 0.8 {
			if self.board.collision(&self.piece) == false {
				self.piece.move_down();
			}
			else {
				self.board.set(&self.piece);

				// Debugging purposes
				// self.board.display();

				self.piece = self.randomizer.get();
				self.board.clear();
			}

			self.time = 0.0;
		}

		// Debugging purposes
		// println!("{}", self.time);
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
							music::play_sound(&Sound::Ding, music::Repeat::Times(0), 0.10);
						}
					},
					Key::Up => {
						if self.board.check_can_rotate(&mut self.piece, LEFT) == true {
							self.piece.rotate_left()
						}
					},
					Key::Left => {
						if self.board.check_overlap(&self.piece, LEFT) != true {
							self.piece.move_left();
						}
					},
					Key::Right => {
						if self.board.check_overlap(&self.piece, RIGHT) != true {
							self.piece.move_right()
						}
					},
					Key::Down => {
						if self.board.collision(&self.piece) != true {
							self.piece.move_down()
						}
					},
					Key::Z => {
						if self.board.check_can_rotate(&mut self.piece, LEFT) == true {
							self.piece.rotate_left()
						}
					},
					Key::X => {
						if self.board.check_can_rotate(&mut self.piece, RIGHT) == true {
							self.piece.rotate_right()
						}
					},
					_ => ()
				}
			}
		}
	}
}
