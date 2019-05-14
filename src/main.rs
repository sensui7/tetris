pub mod randomizer;
pub mod pieces;
pub mod board;
pub mod app;

extern crate music;
extern crate rand;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use crate::board::*;
use crate::app::*;
use crate::randomizer::*;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

const WIDTH: f64 = 300.0;
const HEIGHT: f64 = 600.0;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Music {}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Sound {
    Drop,
    Switch,
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window
    let mut window: Window = WindowSettings::new(
            "Tetris",
            [WIDTH, HEIGHT]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

	// Create a piece randomizer
	let mut randomizer: Randomizer = Randomizer {
		bag: vec![]
	};

	// Create a board object
	let b: Board = Board {
		rows: 10,
		cols: 20,
		data: vec![vec![0; 10]; 20]
	};

    // Create a new game and run it
    let mut app = App {
        gl: GlGraphics::new(opengl),
		board: b,
		piece: randomizer.get(),
		time: 0.0,
		randomizer: randomizer
    };

	// Capture events for standard game loop
    let mut events = Events::new(EventSettings::new());
	events.set_max_fps(40);

	music::start::<Music, Sound, _>(32, || {
        //music::bind_music_file(Music::Piano, "./assets/theme.mp3");
        //music::set_volume(music::MAX_VOLUME);
        music::bind_sound_file(Sound::Drop,   "./assets/drop.wav");
        music::bind_sound_file(Sound::Switch, "./assets/switch.wav");

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
    });
}
