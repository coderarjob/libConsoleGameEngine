//! Game Engine implements various functions to sets up an internal buffer, runs the game play 
//! logic to update the buffer and then draws the buffer on screen. This module also contains
//! methods to draw shapes and text on the buffer.
//!
//! Only the parts which have changed from the previous frame are drawn. 

#![warn(dead_code)]

use std::io::Error;
use std::time::*;
use super::terminal;

/// Unix terminal Foreground colors.
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

/// Unix terminal Background colors.
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

/// Unicode Block characters used to draw items in the game board.
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

/// Represents each character which can be drawn.
#[derive(Copy, Clone)]
struct Pixel {
    character: BlockChars,
    bg_color: BackgroundColors,
    fg_color: ForegroundColors,
    dirty: bool,
}

/// Need to be implemented for the game state struct.
pub trait GamePlay {
    /// Use to draw static parts of the game.
    fn init(&mut self, engine: &mut GameEngine) -> bool;

    /// Contains the logic to change game state and draw the game world.
    fn draw(&mut self, engine: &mut GameEngine, elapsed_time: f64) -> bool;
}

/// Contains the game board and its properties.
pub struct GameEngine {
    grid: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl GameEngine {
    /// Creates a new game engine to represent a game board of the specified dimensions.
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

    /// Game board width (in characters).
    pub fn width(&self) -> usize {
        self.width
    }

    /// Game board height (in characters).
    pub fn height(&self) -> usize {
        self.height
    }

    /// Draws a string at the specified coordinate.
    ///
    /// Only those pixels are updated which has changed since the previous.
    pub fn draw_string(
        &mut self,
        x: usize,
        y: usize,
        s: &str,
        bg_color: BackgroundColors,
        fg_color: ForegroundColors,
    ) {
        let mut x = x;
        for c in s.chars() {
            self.draw_pixel(x, y, BlockChars::Custom(c), bg_color, fg_color);
            x += 1;
        }
    }

    /// Fills a rectangle within the game board with the foregound and background colors.
    ///
    /// This is used to both draw assets and clear the game board.
    /// Only those pixels are updated which has changed since the previous.
    pub fn fill(
        &mut self,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
        c: BlockChars,
        bg_color: BackgroundColors,
        fg_color: ForegroundColors,
    ) {
        for y in top..(top + height) {
            for x in left..(left + width) {
                self.draw_pixel(x, y, c, bg_color, fg_color);
            }
        }
    }

    // Sets a pixel at a specific coordinate.
    //
    /// This can be used to draw assets on the game board.
    /// Only those pixels are updated which has changed since the previous.
    pub fn draw_pixel(
        &mut self,
        x: usize,
        y: usize,
        c: BlockChars,
        bg_color: BackgroundColors,
        fg_color: ForegroundColors,
    ) {
        let index = y * self.width + x;

        let mut pixel = &mut self.grid[index];
        let is_same =
            pixel.bg_color == bg_color && pixel.fg_color == fg_color && pixel.character == c;

        if !is_same {
            pixel.bg_color = bg_color;
            pixel.fg_color = fg_color;
            pixel.character = c;
            pixel.dirty = true;
        }
    }

    /// Prints the game board and game world on the terminal.
    ///
    /// Only those pixels are drawn which has changed since the previous.
    pub fn flush(&mut self) {
        // Move cursor to 1,1 location.
        println!("\x1b[1;1f");

        /*for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                let pixel = &self.grid[index];
                let fg_color = match pixel.dirty {
                    true => ForegroundColors::Red,
                    false => ForegroundColors::White,
                };

                print!(
                    "\x1b[{};{}m",
                    BackgroundColors::Black as u32,
                    fg_color as u32
                );
                print!("{{{},{}}}", x, y);
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

                    //print!("\x1b[{};{}f", y + self.height, x + 1);
                    print!("\x1b[{};{}f", y + 1, x + 1); // Move cursor
                    print!("\x1b[{};{}m", pixel.bg_color as u32, pixel.fg_color as u32);
                    print!("{}", c);
                }
            }
        }

        print!("\x1b[0m"); // Reset colors
    }

    /// This contains the game loop and executes the use logic for game play and drawing of the
    /// game world.
    pub fn begin<T: GamePlay>(&mut self, game_play: &mut T) -> Result<(), Error> {
        terminal::enter_raw_mode()?;
        let mut now = Instant::now();

        // For drawing static parts of the game.
        if game_play.init (self) == true {
            loop {
                let elapsed_time = now.elapsed();
                now = Instant::now();

                if game_play.draw(self, elapsed_time.as_secs_f64()) == false {
                    break; // exit game loop
                }

                self.flush();
            }
        }

        terminal::enter_canon_mode()?;
        terminal::disable_non_blocking_stdio()?;
        Ok(())
    }
}
