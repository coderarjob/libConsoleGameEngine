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

        // Game logic and drawing
        let snapshot = self.clone();

        for x in 1..engine.width() - 1 {
            for y in 1..engine.height() - 1 {
                let mut block_char = BlockChars::Solid;
                let mut bg_color = BackgroundColors::Black;
                let mut fg_color = ForegroundColors::Green;

                if self.get_cell(x, y) == 0 {
                    block_char = BlockChars::LightShaded;
                    bg_color = BackgroundColors::Black;
                    fg_color = ForegroundColors::White;
                }

                engine.draw_pixel(x, y, block_char, bg_color, fg_color);

                let cell = |x: usize, y: usize| snapshot.get_cell(x, y);
                let live_neighbours = cell(x - 1, y - 1) + cell(x, y - 1) + cell(x + 1, y - 1) +
                                      cell(x - 1, y) + 0 + cell(x + 1, y) +
                                      cell(x - 1, y + 1) + cell(x, y + 1) + cell(x + 1, y + 1);

                let is_alive = cell(x, y) == 1;
                let new_value = match live_neighbours {
                    2 => if is_alive { true } else { false },
                    3 => true,
                    0..=1 => false,
                    4.. => false,
                };
                self.set_cell(x, y, new_value);

                /*self.world[y * self.width + x] = match live_neighbours {
                    2..=2 => 1, // Reproducing
                    0..=1 => 0, // Under population
                    3.. => 0,   // Over population
                };*/
            }
        }

        self.generation += 1;
        //std::thread::sleep(std::time::Duration::from_millis(500));
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

    pub fn get_cell(&self, x: usize, y: usize) -> u8 {
        self.world[y * self.width + x]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, is_alive: bool) {
        self.world[y * self.width + x] = is_alive as u8;
    }

    pub fn fill_pattern(&mut self, x: usize, y: usize, pattern: &str) {
        let mut x = x;

        for c in pattern.as_bytes() {
            self.world[y * self.width + x] = if *c == b'0' { 1 } else { 0 };
            x += 1;
        }
    }
}

fn main() {
    let mut game = Game::new(220, 120);
    game.fill_pattern(1, 1, ".0.");
    game.fill_pattern(1, 2, "..0");
    game.fill_pattern(1, 3, "000");

    game.fill_pattern(55, 60, "00000000.00000...000......0000000.00000");
    game.fill_pattern(55, 2, "00000000.00000...000......0000000.00000");

    let mut engine = GameEngine::new(game.width, game.height);
    engine.begin(&mut game).unwrap();
}
