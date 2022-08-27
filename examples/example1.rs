use libconsolegameengine::*;

struct MyGamePlay {
    x: f64,
    y: f64,
    xvel: f64
}

impl GamePlay for MyGamePlay {
    fn draw(&mut self, engine: &mut GameEngine, elapsed_time: f64) -> bool {
        engine.fill(
            0,
            0,
            engine.width(),
            engine.height(),
            BlockChars::DarkShade,
            BackgroundColors::Black,
            ForegroundColors::White,
        )
        .unwrap();

        engine.fill(
            self.x as usize,
            self.y as usize,
            10,
            1,
            BlockChars::Solid,
            BackgroundColors::Black,
            ForegroundColors::White,
        ).unwrap();

        self.x += self.xvel * elapsed_time;
        self.y = self.x.sin() * engine.height() as f64/2.0;

        true
    }
}

fn main() {
    let mut game_play = MyGamePlay { x: 0.0, y: 0.0, xvel: 2.0};
    let mut engine = GameEngine::new(80, 40);
    engine.begin(&mut game_play);
}
