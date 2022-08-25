use libconsolegameengine::*;

fn main() {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let xvel: f64 = 2.0;

    let mut engine = GameEngine::new(80, 40);
    engine.begin(|e, elapsed_time| {
        e.fill(
            0,
            0,
            e.width(),
            e.height(),
            BlockChars::DarkShade,
            BackgroundColors::Black,
            ForegroundColors::White,
        )
        .unwrap();

        e.fill(
            x as usize,
            y as usize,
            10,
            1,
            BlockChars::Solid,
            BackgroundColors::Black,
            ForegroundColors::White,
        )
        .unwrap();

        x += xvel * elapsed_time;
        y = x.sin() * e.height() as f64 / 2.0;
        true
    });
}

/*fn main() {
    let mut engine = GameEngine::new(80, 40);
    let mut x:f64 = 0.0;
    let mut y:f64 = 0.0;

    let mut now = Instant::now();
    let xvel:f64 = 2.0;
    let yvel:f64 = 2.0;

    for _ in 0.. {
        let elapsed_time = now.elapsed();
        now = Instant::now();

        engine.fill(
            0,
            0,
            engine.width(),
            engine.height(),
            BlockChars::DarkShade,
            BackgroundColors::Black,
            ForegroundColors::White,
        ).unwrap();

        engine.fill(
            x as usize,
            y as usize,
            10,
            1,
            BlockChars::Solid,
            BackgroundColors::Black,
            ForegroundColors::White,
        ).unwrap();

        engine.flush();

        x += xvel * elapsed_time.as_secs_f64();
        y = x.sin() * engine.height() as f64/2.0;
        std::thread::sleep (Duration::from_millis(10));
    }
}*/
