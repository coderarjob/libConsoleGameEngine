//! Conways game of life
//!
//! This example creates a game board of 220 x 120 characters, which can be too large for the
//! terminals with large font sizes. If you want to view the whole game board, lower the font size.
//! You can use the below command to run the example:
//! ```bash
//! $ xterm -en UTF-8 -bg black -fg white -fa 'DejaVu Sans Mono:size=4.0:antialias=true' -e cargo run --example life
//! ```

use libconsolegameengine::terminal::{Keys, *};
use libconsolegameengine::*;

#[derive(Clone)]
struct Game {
    world: Vec<u8>,
    width: usize,
    height: usize,
    generation: u64,
}

impl GamePlay for Game {
    fn draw(&mut self, engine: &mut GameEngine, elapsed_time: f64) -> bool {
        let snapshot = self.world.clone();

        // Draw the Generation and FPS
        let fps = 1.0 / elapsed_time;
        let legend = format!("Generation: {} FPS: {:3.2}", self.generation, fps);
        engine.draw_string(
            0,
            0,
            &format!("{:40}", &legend),
            BackgroundColors::White,
            ForegroundColors::Black,
        );

        // Press enter to exit
        if let Ok(Keys::Enter) = get_keypress() {
            return false;
        }

        for x in 1..engine.width() - 1 {
            for y in 1..engine.height() - 1 {
                if self.world[y * self.width + x] == 1 {
                    engine.draw_pixel(
                        x,
                        y,
                        BlockChars::Solid,
                        BackgroundColors::Black,
                        ForegroundColors::Green,
                    )
                } else {
                    engine.draw_pixel(
                        x,
                        y,
                        BlockChars::Blank,
                        BackgroundColors::Black,
                        ForegroundColors::White,
                    )
                }

                let cell = |x: usize, y: usize| snapshot[y * self.width + x];
                let live_neighbours = cell(x - 1, y - 1) + cell(x, y - 1) + cell(x + 1, y - 1) +
                                      cell(x - 1, y) + 0 + cell(x + 1, y) +
                                      cell(x - 1, y + 1) + cell(x, y + 1) + cell(x + 1, y + 1);

                let is_alive = cell(x, y) == 1;

                self.world[y * self.width + x] = match live_neighbours {
                    2 => if is_alive { 1 } else { 0 },
                    3 => 1,
                    0..=1 => 0,
                    4.. => 0,
                };

                /*self.world[y * self.width + x] = match live_neighbours {
                    2..=2 => 1, // Reproducing
                    0..=1 => 0, // Under population
                    3.. => 0,   // Over population
                };*/
            }
        }

        self.generation += 1;
        true
    }
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            width,
            height,
            world: vec![0; width * height],
            generation: 0,
        }
    }
    pub fn cell_pattern(&mut self, x: usize, y: usize, pattern: &str) {
        let mut x = x;

        for c in pattern.as_bytes() {
            self.world[y * self.width + x] = if *c == b'0' { 1 } else { 0 };
            x += 1;
        }
    }
}

fn main() {
    let mut game = Game::new(220, 120);
    game.cell_pattern(1, 1, ".0.");
    game.cell_pattern(1, 2, "..0");
    game.cell_pattern(1, 3, "000");

    game.cell_pattern(55, 60, "00000000.00000...000......0000000.00000");

    let mut engine = GameEngine::new(game.width, game.height);
    engine.begin(&mut game).unwrap();
}
