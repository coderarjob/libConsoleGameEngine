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

                let fps = 1.0/elapsed_time;
                //let legend = format!("Generation: {:10} FPS: {:3.2}", self.generation, fps);
                let legend = format!("Generation: {} FPS: {}", self.generation, fps);
                engine.draw_string(0, 0, &legend, BackgroundColors::White, ForegroundColors::Black);

                let cell = |x: usize, y: usize| snapshot[y * self.width + x];
                let live_neighbours = cell(x - 1, y - 1) + cell(x, y - 1) + cell(x + 1, y - 1) +
                                      cell(x - 1, y) + 0 + cell(x + 1, y) +
                                      cell(x - 1, y + 1) + cell(x, y + 1) + cell(x + 1, y + 1);

                let is_alive = cell(x, y) == 1;

                /*self.world[y * self.width + x] = match live_neighbours {
                    2 => if is_alive { 1 } else { 0 },
                    3 => 1,
                    0..=1 => 0,
                    4.. => 0
                };*/

                /*self.world[y * self.width + x] = match live_neighbours {
                    2..=4 => 1, // Reproducing
                    0..=1 => 0, // Under population
                    5.. => 0,   // Over population
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
            self.world[y * self.width + x] = if *c == b'#' { 1 } else { 0 };
            x += 1;
        }
    }
}

fn main() {
    let mut game = Game::new(420, 80);
    //game.cell_pattern(60, 60, "##");
    game.cell_pattern (30, 40, "######## #####   ###      ####### #####");
    /*game.cell_pattern (100, 100, "######## #####   ###      ####### #####");
    game.cell_pattern (200, 100, "######## #####   ###      ####### #####");
    game.cell_pattern (0, 100, "######## #####   ###      ####### #####");*/

    let mut engine = GameEngine::new(game.width, game.height);
    engine.begin(&mut game).unwrap();
}
