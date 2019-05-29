# Tetris - Rust

## Preview
![Game Image](https://i.imgur.com/1MWAXAK.png)
![Gameplay](https://media.giphy.com/media/j52OZknhdrPQCievsn/giphy.gif)

## Instructions
Execute `cargo build && cargo run` to install the dependencies/build the program and then run it.

The keybindings are `UP, DOWN, LEFT, RIGHT, SPACE, Z, and X`.

## Future Features to Consider
- All T-Spin setups
- All Z/S/L/J spin setups
- Combos (incremental sound effects)
- Hold a piece option
- Scores

## Code Overview
- main.rs
  - Sets up the objects for the game, and starts the game loop
  - Handles the initialization of sounds (e.g. voice person saying TETRIS)
- app.rs
  - Handles the functions for the game loop (input, update, render)
  - Handles all of the GlGraphics for rendering the pieces
- board.rs
  - Handles the logic for collision and clearing
- pieces.rs
  - Handles the rotation logic for the 7 different pieces (T, I, O, S, Z, J, L)
- randomizer.rs
  - Handles the random generation of the tetris pieces
