## Console Game Engine for Linux-like OS

![Game of life](/doc/life.png)

API for the creation of character based games in Linux. The inspiration came from the
[olcConsoleGameEngine](https://www.youtube.com/watch?v=u5BhrA8ED0o). It is my attempt to recreate 
few of its features in Linux.

At the current stage, the library contains API for the following.

1. To create a game board (which is a rectangle of some width and height in characters), and
   includes functions to fill the board with block Unicode characters. This can use used to
   draw the 'World' as well as other 'assets' for the game.

2. Function to read input from keyboard in a non-blocking faction.

### Dependencies

* [Arjob's Rust Library](https://github.com/coderarjob/libarl) - For FFI calls.

### Example

```rust
use libconsolegameengine::terminal::Keys::*;
use libconsolegameengine::*;

struct MyGamePlay;
impl GamePlay for MyGamePlay {
    fn draw(&mut self, engine: &mut GameEngine, elapsed_time: f64) -> bool {

        // Clear the game board.
        engine.fill(
                    0,
                    0,
                    engine.width(),
                    engine.height(),
                    BlockChars::DarkShade,
                    BackgroundColors::Black,
                    ForegroundColors::White,
                ).unwrap();
        true
    }
}
 fn main() {
    let mut game_play = MyGamePlay;
    let mut engine = GameEngine::new(80, 40);
    engine.begin(&mut game_play).unwrap();
 }
 ```
