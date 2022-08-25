#![allow(dead_code)]

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum ForegroundColors {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum BackgroundColors {
    Black = 40,
    Red = 41,
    Green = 42,
    Yellow = 43,
    Blue = 44,
    Magenta = 45,
    Cyan = 46,
    White = 47,
}

#[derive(Copy, Clone, PartialEq)]
pub enum BlockChars {
    Blank,
    Solid,
    LightShaded,
    MediumShaded,
    DarkShade,
    Custom(char),
}

impl From<BlockChars> for char {
    fn from(bchar: BlockChars) -> Self {
        use BlockChars::*;
        match bchar {
            Blank => ' ',
            Solid => '█',
            LightShaded => '░',
            MediumShaded => '▒',
            DarkShade => '▓',
            Custom(c) => c,
        }
    }
}

#[derive(Copy, Clone)]
struct Pixel {
    character: BlockChars,
    bg_color: BackgroundColors,
    fg_color: ForegroundColors,
    dirty: bool,
}

pub struct GameEngine {
    grid: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl GameEngine {
    pub fn new(width: usize, height: usize) -> GameEngine {
        GameEngine {
            width,
            height,
            grid: vec![
                Pixel {
                    character: BlockChars::Blank,
                    bg_color: BackgroundColors::Black,
                    fg_color: ForegroundColors::White,
                    dirty: true
                };
                width * height
            ],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn fill(
        &mut self,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
        c: BlockChars,
        bg_color: BackgroundColors,
        fg_color: ForegroundColors,
    ) -> Result<(), &'static str> {
        for y in top..(top + height) {
            for x in left..(left + width) {
                let index = y * self.width + x;

                let mut pixel = &mut self.grid[index];
                let is_same = pixel.bg_color == bg_color
                    && pixel.fg_color == fg_color
                    && pixel.character == c;

                if !is_same {
                    pixel.bg_color = bg_color;
                    pixel.fg_color = fg_color;
                    pixel.character = c;
                    pixel.dirty = true;
                }
            }
        }

        Ok(())
    }

    pub fn flush(&mut self) {
        println!("\x1b[1;1f");

        /*for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                let pixel = &self.grid[index];
                let fg_color = match pixel.dirty {
                    true => ForegroundColors::Red,
                    false => ForegroundColors::White
                };

                print!("\x1b[{};{}m", BackgroundColors::Black as u32, fg_color as u32);
                print! ("{{{},{}}}", x,y);
                print!("\x1b[0m");
            }
            print!("\n");
        }*/

        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                let mut pixel = &mut self.grid[index];

                if pixel.dirty {
                    let c: char = pixel.character.into();
                    pixel.dirty = false;

                    //print!("\x1b[{};{}f",y + 20,x + 1);
                    print!("\x1b[{};{}f", y + 1, x + 1);
                    print!("\x1b[{};{}m", pixel.bg_color as u32, pixel.fg_color as u32);
                    print!("{}", c);
                    print!("\x1b[0m");
                }
            }
            print!("\n");
        }
    }

    pub fn begin<T>(&mut self, mut func: T)
    where
        T: FnMut(&mut Self, f64) -> bool,
    {
        use std::time::*;

        let mut now = Instant::now();
        loop {
            let elapsed_time = now.elapsed();
            now = Instant::now();

            if func(self, elapsed_time.as_secs_f64()) == false {
                break;      // exit game loop
            }

            self.flush();
            std::thread::sleep (Duration::from_millis(10));
        }
    }
}
