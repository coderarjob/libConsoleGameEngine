//! Conways game of life
//!
//! This example creates a game board of 220 x 120 characters, which can be too large for the
//! terminals with large font sizes. If you want to view the whole game board, lower the font size.
//! You can use the below command to run the example:
//! ```bash
//! $ xterm -en UTF-8 -bg black -fg white -fa 'DejaVu Sans Mono:size=4.0:antialias=true' -e cargo run --example life
//! ```

use libconsolegameengine::game_engine::*;
use libconsolegameengine::terminal::*;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct Game {
    world: Vec<u8>,
    width: usize,
    height: usize,
    generation: u64,
    max_fps: u32,
    minimum_frame_period_ms: u64,
}

impl GamePlay for Game {
    fn init(&mut self, engine: &mut GameEngine) -> bool {
        // Static asset. Need to be drawn only once.
        let notice = "Press Up/Down to increase/decrease speed | Press Enter to quit";

        engine.draw_string(
            self.width - notice.len(),
            0,
            notice,
            BackgroundColors::White,
            ForegroundColors::Black,
        );

        true
    }

    fn draw(&mut self, engine: &mut GameEngine, elapsed_time: f64) -> bool {
        // Draw the Generation and FPS
        let fps = 1.0 / elapsed_time;
        let legend = format!("Generation: {} FPS: {:3.2}", self.generation, fps);

        engine.draw_string(
            0,
            0,
            &format!("{:30}", &legend),
            BackgroundColors::White,
            ForegroundColors::Black,
        );

        // Press enter to exit
        match get_keypress() {
            Ok(Keys::Up) => {
                self.max_fps += 1;
                self.minimum_frame_period_ms = Game::get_frame_period_ms(self.max_fps);
            }
            Ok(Keys::Down) if self.max_fps > 1 => {
                self.max_fps -= 1;
                self.minimum_frame_period_ms = Game::get_frame_period_ms(self.max_fps);
            }
            Ok(Keys::Enter) => return false,
            _ => (),
        }

        // Game logic and drawing
        let snapshot = self.clone();

        for x in 1..engine.width() - 1 {
            for y in 1..engine.height() - 1 {
                let mut block_char = BlockChars::Solid;
                let mut bg_color = BackgroundColors::Black;
                let mut fg_color = ForegroundColors::Yellow;

                if self.get_cell(x, y) == 0 {
                    block_char = BlockChars::Blank;
                    bg_color = BackgroundColors::Black;
                    fg_color = ForegroundColors::Black;
                }

                engine.draw_pixel(x, y, block_char, bg_color, fg_color);

                let cell = |x: usize, y: usize| snapshot.get_cell(x, y);

                #[rustfmt::skip]
                let live_neighbours = cell(x - 1, y - 1) + cell(x, y - 1) + cell(x + 1, y - 1) +
                                      cell(x - 1, y)     + 0              + cell(x + 1, y)     +
                                      cell(x - 1, y + 1) + cell(x, y + 1) + cell(x + 1, y + 1);

                let is_alive = cell(x, y) == 1;

                #[rustfmt::skip]
                let new_value = match live_neighbours {
                    2 => if is_alive { true } else { false },
                    3 => true,
                    0..=1 => false,
                    4.. => false,
                };
                self.set_cell(x, y, new_value);
            }
        }

        self.generation += 1;

        thread::sleep(Duration::from_millis(self.minimum_frame_period_ms));

        true
    }
}

impl Game {
    pub fn new(width: usize, height: usize, max_fps: u32) -> Game {
        Game {
            width,
            height,
            world: vec![0; width * height],
            generation: 0,
            max_fps,
            minimum_frame_period_ms: Self::get_frame_period_ms(max_fps)
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
            self.world[y * self.width + x] = if *c == b'O' { 1 } else { 0 };
            x += 1;
        }
    }

    pub fn get_frame_period_ms (max_fps: u32) -> u64 {
        1000 / max_fps as u64
    }

}

fn main() {
    let mut game = Game::new(220, 120, 5);
    game.fill_pattern(2, 2, "........................O");
    game.fill_pattern(2, 3, "......................O.O");
    game.fill_pattern(2, 4, "............OO......OO............OO");
    game.fill_pattern(2, 5, "...........O...O....OO............OO");
    game.fill_pattern(2, 6, "OO........O.....O...OO");
    game.fill_pattern(2, 7, "OO........O...O.OO....O.O");
    game.fill_pattern(2, 8, "..........O.....O.......O");
    game.fill_pattern(2, 9, "...........O...O");
    game.fill_pattern(2, 10, "............OO");

    /*game.fill_pattern(1, 1, ".O.");
    game.fill_pattern(1, 2, "..O");
    game.fill_pattern(1, 3, "OOO");*/

    //game.fill_pattern(55, 2, "OOOOOOOO.OOOOO...OOO......OOOOOOO.OOOOO");
    game.fill_pattern(55, 60, "OOOOOOOO.OOOOO...OOO......OOOOOOO.OOOOO");

    let mut engine = GameEngine::new(game.width, game.height);
    engine.begin(&mut game).unwrap();
}
