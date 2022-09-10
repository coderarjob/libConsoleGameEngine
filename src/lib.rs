//! Console Game Engine for Linux.
//!
//! This library was inspired by the [`olcConsoleGameEngine`] library. It is my attempt to
//! recreate few of its features in Linux.
//!
//! 1. Creates a game board (which is a rectangle of some width and height in characters), and
//!    includes functions to fill the board with block Unicode characters. This can use used to
//!    draw the 'World' as well as other 'assets' for the game.
//!
//! 2. Function to read input from keyboard in a non-blocking faction.
//!
//! # Example
//!
//! ```rust
//! use libconsolegameengine::terminal::Keys::*;
//! use libconsolegameengine::*;
//! use libconsolegameengine::game_engine::*;
//!
//! struct MyGamePlay;
//! impl GamePlay for MyGamePlay {
//!     fn draw(&mut self, engine: &mut GameEngine, elapsed_time: f64) -> bool {
//!         engine.fill(
//!                     0,
//!                     0,
//!                     engine.width(),
//!                     engine.height(),
//!                     BlockChars::DarkShade,
//!                     BackgroundColors::Black,
//!                     ForegroundColors::White,
//!                 );
//!         true
//!     }
//! }
//!  fn main() {
//!     let mut game_play = MyGamePlay;
//!     let mut engine = GameEngine::new(80, 40);
//!     engine.begin(&mut game_play).unwrap();
//!  }
//!  ```
//!
//! [`olcConsoleGameEngine`]: https://github.com/OneLoneCoder/videos/blob/master/olcConsoleGameEngine.h

pub mod terminal;
pub mod game_engine;
