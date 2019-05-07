use crate::pieces::*;
use crate::board::*;
use piston::input::*;
use opengl_graphics::{ GlGraphics };

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
	pub board: Board,
	pub piece: Piece,
	pub time: f64,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

		const TEAL: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
		const PURPLE: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
		const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        //const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

		// if next block flag is true, generate a new piece here
        let square = rectangle::square(self.piece.p1.x, self.piece.p1.y, 40.0);
        let square2 = rectangle::square(self.piece.p2.x, self.piece.p2.y, 40.0);
        let square3 = rectangle::square(self.piece.p3.x, self.piece.p3.y, 40.0);
        let square4 = rectangle::square(self.piece.p4.x, self.piece.p4.y, 40.0);

        //let rotation = self.rotation;
		/*
        let (x, y) = (args.width / 2.0,
                      args.height / 2.0);
		*/

		//let border = rectangle::Rectangle::new_border(BLACK, 1.0);
		//let rect = rectangle::rectangle_by_corners(140.0, 0.0, 340.0, 640.0);
		let grid = grid::Grid { cols: 10, rows: 20, units: 40.0 };
		let line = line::Line::new(WHITE, 1.0);
		let history = &self.board.data;
		let curr_piece = &self.piece.piece_type;

        self.gl.draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {
            // Clear the screen.
            clear(BLACK, gl);
			/*
            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);
			*/
			//let transform = c.transform.trans(0.0, 0.0);
            // Draw a box rotating around the middle of the screen.
			match curr_piece.as_str() {
				"T" => {
					rectangle(PURPLE, square, c.transform, gl);
					rectangle(PURPLE, square2, c.transform, gl);
					rectangle(PURPLE, square3, c.transform, gl);
					rectangle(PURPLE, square4, c.transform, gl);
				},
				"I" => {
					rectangle(TEAL, square, c.transform, gl);
					rectangle(TEAL, square2, c.transform, gl);
					rectangle(TEAL, square3, c.transform, gl);
					rectangle(TEAL, square4, c.transform, gl);
				}
				_ => println!("ERROR")
			}

			// Render grid history of other blocks!
			let mut row: u64 = 0;
			let mut col: u64 = 0;
			for i in history.iter() {
				for j in i {
					let sq = rectangle::square((col * 40) as f64, (row * 40) as f64, 40.0);
					if j == &1 {
						//println!("{}, {}", row, col);
						rectangle(PURPLE, sq, c.transform, gl);
						//println!("{}", col);
					} else if j == &2 {
						rectangle(TEAL, sq, c.transform, gl);
					}
					col += 1;
				}
				row += 1;
				col = 0;
			}

			//border.draw(rect, &c.draw_state, c.transform, gl);
			grid.draw(&line, &c.draw_state, c.transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // self.rotation += 2.0 * args.dt;

		// if collision here, update the piece to the next one!
		// update the board as well

		if self.time - args.dt >= 1.0 {
			if self.board.collision(&self.piece) == false {
				self.piece.move_down();
			}
			else {
				self.board.set(&self.piece);
				self.board.display();

				// Need to implement a random piece creator that returns a Piece struct
				let t = Piece {
					piece_type: "I".to_string(),
					rotation: 0,
					p1: Point { x: 120.0, y: 0.0 },
					p2: Point { x: 160.0, y: 0.0 },
					p3: Point { x: 200.0, y: 0.0 },
					p4: Point { x: 240.0, y: 0.0 }
				};

				self.piece = t;
			}
			/*
			println!("p1: {}, {}", self.piece.p1.x / 40.0, self.piece.p1.y / 40.0);
			println!("p2: {}, {}", self.piece.p2.x / 40.0, self.piece.p2.y / 40.0);
			println!("p3: {}, {}", self.piece.p3.x / 40.0, self.piece.p3.y / 40.0);
			println!("p4: {}, {}", self.piece.p4.x / 40.0, self.piece.p4.y / 40.0);
			*/
			self.time = 0.0;
		}

		//println!("{}", self.time);
		self.time += args.dt;
    }

	pub fn input(&mut self, button: &Button, is_press: bool) {
		if is_press {
			if let Button::Keyboard(key) = button {
				match key {
					//Key::Left => println!("Left pressed (x,y): {}, {}", self.x, self.y),
					Key::Space => {
							
					},
					Key::Left => {
						if self.board.check_overlap(&self.piece, 0) != true {
							self.piece.move_left()
						}
					},
					Key::Right => {
						if self.board.check_overlap(&self.piece, 1) != true {
							self.piece.move_right()
						}
					},
					Key::Down => {
						if self.board.collision(&self.piece) != true {
							self.piece.move_down()
						}
					},
					Key::Z => {
						//if self.board.check_can_rotate(&self.piece) != false {
							self.piece.rotate_left()
						//}
					},
					_ => ()
				}
			}
		}
	}
}
