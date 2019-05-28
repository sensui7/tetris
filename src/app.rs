use crate::board::*;
use crate::pieces::Dir::*;
use crate::pieces::*;
use crate::randomizer::PieceTypes::*;
use crate::randomizer::*;
use crate::Sound;
use graphics::*;
use music;
use opengl_graphics::GlGraphics;
use piston::input::*;

const GREY: [f32; 4] = [0.5, 0.5, 0.5, 0.8];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const DARK_BLUE: [f32; 4] = [0.0, 0.0, 0.5, 1.0];
const ORANGE: [f32; 4] = [1.0, 0.6, 0.0, 1.0];
const YELLOW: [f32; 4] = [1.0, 0.8, 0.0, 1.0];
const TEAL: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
const PURPLE: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend
    pub board: Board,
    pub piece: Piece,
    pub time: f64,
    pub randomizer: Randomizer,
}

// Render tetris shapes from individual square blocks
fn render_rectangles(
    color: &[f32; 4],
    sq: [[f64; 4]; 4],
    transform: [[f64; 3]; 2],
    gl: &mut GlGraphics,
) {
    rectangle(*color, sq[0], transform, gl);
    rectangle(*color, sq[1], transform, gl);
    rectangle(*color, sq[2], transform, gl);
    rectangle(*color, sq[3], transform, gl);
}

// Render grid history of other blocks
fn render_history(history: &[Vec<u64>], transform: [[f64; 3]; 2], gl: &mut GlGraphics) {
    for (i, col) in history.iter().enumerate() {
        for (j, _row) in col.iter().enumerate() {
            let sq =
                rectangle::square((j * UNIT as usize) as f64, (i * UNIT as usize) as f64, UNIT);

            match history[i][j] {
                0 => {}
                1 => rectangle(PURPLE, sq, transform, gl),
                2 => rectangle(TEAL, sq, transform, gl),
                3 => rectangle(YELLOW, sq, transform, gl),
                4 => rectangle(ORANGE, sq, transform, gl),
                5 => rectangle(DARK_BLUE, sq, transform, gl),
                6 => rectangle(RED, sq, transform, gl),
                7 => rectangle(GREEN, sq, transform, gl),
                _ => println!("Error: something went wrong with rendering the block history"),
            }
        }
    }
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        let square1 = rectangle::square(self.piece.p1.x, self.piece.p1.y, UNIT);
        let square2 = rectangle::square(self.piece.p2.x, self.piece.p2.y, UNIT);
        let square3 = rectangle::square(self.piece.p3.x, self.piece.p3.y, UNIT);
        let square4 = rectangle::square(self.piece.p4.x, self.piece.p4.y, UNIT);

        let mut shadow: Piece = self.piece.clone();

        while !self.board.collision(&shadow) {
            shadow.move_down();
        }

        let s1 = rectangle::square(shadow.p1.x, shadow.p1.y, UNIT);
        let s2 = rectangle::square(shadow.p2.x, shadow.p2.y, UNIT);
        let s3 = rectangle::square(shadow.p3.x, shadow.p3.y, UNIT);
        let s4 = rectangle::square(shadow.p4.x, shadow.p4.y, UNIT);

        let grid = grid::Grid {
            cols: 10,
            rows: 20,
            units: UNIT,
        };
        let line = line::Line::new(WHITE, 0.8);
        let history: &Vec<Vec<u64>> = &self.board.data;
        let curr_piece: &PieceTypes = &self.piece.piece_type;

        let mut bag = self.randomizer.bag.clone();
        if bag.is_empty() {
            self.randomizer.generate();
            bag = self.randomizer.bag.clone();
        }

        let next_piece = bag.iter_mut().rev().next().unwrap();
        let next_grid = grid::Grid {
            cols: 4,
            rows: 4,
            units: UNIT,
        };

        // Place the future pieces into the box nicely
        match next_piece.piece_type {
            T => next_piece.move_down(),
            I => next_piece.move_down(),
            O => next_piece.move_down(),
            S => {
                next_piece.rotate_right();
                next_piece.move_down();
            }
            Z => next_piece.rotate_left(),
            L => next_piece.move_left(),
            J => {}
        }

        // Squares used to draw next piece
        let n1 = rectangle::square(
            next_piece.p1.x + (UNIT * 8.0),
            next_piece.p1.y + UNIT * 2.0,
            UNIT,
        );
        let n2 = rectangle::square(
            next_piece.p2.x + (UNIT * 8.0),
            next_piece.p2.y + UNIT * 2.0,
            UNIT,
        );
        let n3 = rectangle::square(
            next_piece.p3.x + (UNIT * 8.0),
            next_piece.p3.y + UNIT * 2.0,
            UNIT,
        );
        let n4 = rectangle::square(
            next_piece.p4.x + (UNIT * 8.0),
            next_piece.p4.y + UNIT * 2.0,
            UNIT,
        );

        self.gl
            .draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {
                // Clear the screen
                clear(BLACK, gl);

                // Render shadow for the shape (helps user place pieces easier)
                render_rectangles(&GREY, [s1, s2, s3, s4], c.transform, gl);

                match curr_piece {
                    T => {
                        render_rectangles(
                            &PURPLE,
                            [square1, square2, square3, square4],
                            c.transform,
                            gl,
                        );
                    }
                    I => {
                        render_rectangles(
                            &TEAL,
                            [square1, square2, square3, square4],
                            c.transform,
                            gl,
                        );
                    }
                    O => {
                        render_rectangles(
                            &YELLOW,
                            [square1, square2, square3, square4],
                            c.transform,
                            gl,
                        );
                    }
                    L => {
                        render_rectangles(
                            &ORANGE,
                            [square1, square2, square3, square4],
                            c.transform,
                            gl,
                        );
                    }
                    J => {
                        render_rectangles(
                            &DARK_BLUE,
                            [square1, square2, square3, square4],
                            c.transform,
                            gl,
                        );
                    }
                    Z => {
                        render_rectangles(
                            &RED,
                            [square1, square2, square3, square4],
                            c.transform,
                            gl,
                        );
                    }
                    S => {
                        render_rectangles(
                            &GREEN,
                            [square1, square2, square3, square4],
                            c.transform,
                            gl,
                        );
                    }
                }

                render_history(history, c.transform, gl);
                grid.draw(&line, &c.draw_state, c.transform, gl);

                match next_piece.piece_type {
                    T => {
                        render_rectangles(&PURPLE, [n1, n2, n3, n4], c.transform, gl);
                    }
                    I => {
                        render_rectangles(&TEAL, [n1, n2, n3, n4], c.transform, gl);
                    }
                    O => {
                        render_rectangles(&YELLOW, [n1, n2, n3, n4], c.transform, gl);
                    }
                    L => {
                        render_rectangles(&ORANGE, [n1, n2, n3, n4], c.transform, gl);
                    }
                    J => {
                        render_rectangles(&DARK_BLUE, [n1, n2, n3, n4], c.transform, gl);
                    }
                    Z => {
                        render_rectangles(&RED, [n1, n2, n3, n4], c.transform, gl);
                    }
                    S => {
                        render_rectangles(&GREEN, [n1, n2, n3, n4], c.transform, gl);
                    }
                }

                next_grid.draw(
                    &line,
                    &c.draw_state,
                    c.transform.trans(0.0, 0.0).trans(330.0, 30.0),
                    gl,
                );
            });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        if self.time - args.dt >= 0.8 {
            if !self.board.collision(&self.piece) {
                self.piece.move_down();
            } else {
                self.board.set(&self.piece);
                if self.randomizer.bag.is_empty() {
                    self.randomizer.generate();
                }
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
                        while !self.board.collision(&self.piece) {
                            self.piece.move_down();
                            self.time = 1.0;
                            music::play_sound(&Sound::Drop, music::Repeat::Times(0), 0.10);
                        }
                    }
                    Key::Up => {
                        if self.board.check_can_rotate(&mut self.piece, LEFT) {
                            self.piece.rotate_left();
                        }
                    }
                    Key::Left => {
                        if !self.board.check_overlap(&self.piece, LEFT) {
                            self.piece.move_left();
                        }
                    }
                    Key::Right => {
                        if !self.board.check_overlap(&self.piece, RIGHT) {
                            self.piece.move_right();
                        }
                    }
                    Key::Down => {
                        if !self.board.collision(&self.piece) {
                            self.piece.move_down();
                        }
                    }
                    Key::Z => {
                        if self.board.check_can_rotate(&mut self.piece, LEFT) {
                            self.piece.rotate_left();
                        }
                    }
                    Key::X => {
                        if self.board.check_can_rotate(&mut self.piece, RIGHT) {
                            self.piece.rotate_right();
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
