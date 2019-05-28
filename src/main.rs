pub mod app;
pub mod board;
pub mod pieces;
pub mod randomizer;

use crate::app::*;
use crate::board::*;
use crate::randomizer::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

const ADDITIONAL: f64 = 180.0;
const WIDTH: f64 = 300.0;
const HEIGHT: f64 = 600.0;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Music {
    // Credits to https://musescore.com/zakuramusic/korobeiniki-tetris-Tetris
    Theme,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Sound {
    Drop,
    Switch,
    Tetris,
    Triple,
    Double,
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window
    let mut window: Window = WindowSettings::new("Tetris", [WIDTH + ADDITIONAL, HEIGHT])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a piece randomizer
    let mut randomizer: Randomizer = Randomizer { bag: vec![] };

    // Create a board object
    let b: Board = Board {
        rows: 10,
        cols: 20,
        data: vec![vec![0; 10]; 20],
    };

    // Create a new game and run it
    let mut app = App {
        gl: GlGraphics::new(opengl),
        board: b,
        piece: randomizer.get(),
        time: 0.0,
        randomizer,
    };

    // Capture events for standard game loop
    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(40);

    music::start::<Music, Sound, _>(100, || {
        //music::set_volume(music::MAX_VOLUME);
        //music::bind_music_file(Music::Theme,  "./assets/theme.mp3");
        music::bind_sound_file(Sound::Drop, "./assets/drop.wav");
        music::bind_sound_file(Sound::Switch, "./assets/switch.wav");
        music::bind_sound_file(Sound::Tetris, "./assets/tetris.mp3");
        music::bind_sound_file(Sound::Triple, "./assets/triple.mp3");
        music::bind_sound_file(Sound::Double, "./assets/double.mp3");
        //music::play_music(&Music::Theme, music::Repeat::Forever);

        while let Some(e) = events.next(&mut window) {
            if let Some(i) = e.press_args() {
                app.input(&i, true);
            }

            if let Some(r) = e.render_args() {
                app.render(&r);
            }

            if let Some(u) = e.update_args() {
                app.update(u);
            }
        }
    });
}
