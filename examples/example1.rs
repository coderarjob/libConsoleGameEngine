use libconsolegameengine::terminal::Keys::*;
use libconsolegameengine::*;

struct MyGamePlay {
    x: f64,
    y: f64,
    xvel: f64,
    yvel: f64,
}

impl GamePlay for MyGamePlay {
    fn draw(&mut self, engine: &mut GameEngine, elapsed_time: f64) -> bool {
        engine
            .fill(
                0,
                0,
                engine.width(),
                engine.height(),
                BlockChars::DarkShade,
                BackgroundColors::Black,
                ForegroundColors::White,
            )
            .unwrap();

        engine
            .fill(
                self.x as usize,
                self.y as usize,
                10,
                1,
                BlockChars::Solid,
                BackgroundColors::Black,
                ForegroundColors::White,
            )
            .unwrap();

        if let Ok(key) = terminal::get_keypress() {
            match key {
                Up => self.y -= self.yvel * elapsed_time,
                Down => self.y += self.yvel * elapsed_time,
                Left => self.x -= self.xvel * elapsed_time,
                Right => self.x += self.xvel * elapsed_time,
                Other(c) => if c == "q" { return false },
                _ => (),
            }
        }

        //self.x += self.xvel * elapsed_time;
        //self.y = self.x.sin() * engine.height() as f64 / 2.0;

        true
    }
}

fn main() {
    let mut game_play = MyGamePlay {
        x: 0.0,
        y: 0.0,
        xvel: 400.0,
        yvel: 100.0,
    };
    let mut engine = GameEngine::new(80, 40);
    engine.begin(&mut game_play).unwrap();
}
