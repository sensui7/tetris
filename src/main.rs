pub mod randomizer;
pub mod pieces;
pub mod board;
pub mod app;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use crate::pieces::*;
use crate::board::*;
use crate::app::*;
use crate::randomizer::*;
use crate::randomizer::PieceTypes::*;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

const WIDTH: f64 = 400.0;
// const HEIGHT: f64 = 800.0;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Tetris",
            [400, 800]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

	/*
	let t = Piece {
		piece_type: T,
		rotation: 0,
		p1: Point { x: 200.0, y: 0.0 },
		p2: Point { x: 240.0, y: 0.0 },
		p3: Point { x: 160.0, y: 0.0 },
		p4: Point { x: 200.0, y: -40.0 }
	};
	*/
	let randomizer = Randomizer {
		bag: vec![]
	};

	let b = Board {
		rows: 10,
		cols: 20,
		data: vec![vec![0; 10]; 20]
	};

	//b.display();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
		board: b,
		piece: randomizer.get(),
		time: 0.0,
		randomizer: randomizer
    };

    let mut events = Events::new(EventSettings::new());
	events.set_max_fps(60);

    while let Some(e) = events.next(&mut window) {
		if let Some(i) = e.press_args() {
			app.input(&i, true);
		}

        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
