# Tetris - Rust

## Preview
![Game Image](https://i.imgur.com/1MWAXAK.png)
![Gameplay](https://media.giphy.com/media/j52OZknhdrPQCievsn/giphy.gif)

## Instructions
Execute `cargo build && cargo run` to install the dependencies/build the program and then run it.

Execute `cargo test` to run the unit tests for this project.

The keybindings are `UP, DOWN, LEFT, RIGHT, SPACE, Z, and X`.

## Requirements
You will need to install SDL2 mixer (for audio) and possibly SDL2: `sudo apt-get install libsdl2-dev libsdl2-mixer-dev`

## Future Features to Consider
- All T-Spin setups
- All Z/S/L/J spin setups
- Combos (incremental sound effects)
- Hold a piece option
- Scores

## Code Overview
- main.rs
  - Sets up the objects for the game, and starts the game loop
    - Source referenced (and for the rest of this project): https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started
  - Handles the initialization of sounds (e.g. voice person saying TETRIS)
    - Source referenced: https://github.com/PistonDevelopers/music
    - Source for the voices: https://www.animaker.com/voice
    - Source for the other sound effects: http://soundbible.com/tags-game.html
- app.rs
  - Handles the functions for the game loop (input, update, render)
  - Handles all of the GlGraphics for rendering the pieces
- board.rs
  - Handles the logic for collision and clearing
- pieces.rs
  - Handles the rotation logic for the 7 different pieces (T, I, O, S, Z, J, L)
- randomizer.rs
  - Handles the random generation of the tetris pieces

## Writeup
Developing this project required some experimenting with how to draw squares onto the screen using GlGraphics. I started off with a somewhat monolithic program file that had the bare minimum required to compile and run, which included the game loop and a simple square on the screen. The PistonTutorials link (referenced above) was really useful towards getting started. Before continuing, I planned to split the monolithic program into 5 different modules to keep the development process simple. From playing tetris for years, I drew a lot of diagrams on paper of how I understood the concepts to work, and translated the actual logic afterwards such as rotations for the different types of tetronimos. After getting all of the rotations done, the next step was to figure out the collisions between blocks and boundaries. Once the collisions, rotations, and placement problems were solved, fun features such as t-spin, sounds, and a randomizer (similar to the one used in TetrisFriends) were implemented. Overall, a lot of the development process involved experimenting, reading the documentation, and drawing the majority of the logic on paper before moving onto actual development.
