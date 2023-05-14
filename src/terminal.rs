//! This module contains functions that interact with the terminal in order to set its mode and to
//! read back key presses without blocking.

use libarl::*;
use std::io::Error;
use std::os::unix::io::*;

const KEY_UP: &str = "\x1b[A";
const KEY_RIGHT: &str = "\x1b[C";
const KEY_DOWN: &str = "\x1b[B";
const KEY_LEFT: &str = "\x1b[D";

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

#[derive(Debug, PartialEq)]
pub enum Keys {
    Left,
    Right,
    Up,
    Down,
    Enter,
    Nothing,
    Other(String),
}

pub(crate) fn enable_non_blocking_stdio() -> Result<(), Error> {
    let stdin_fd = std::io::stdin().as_raw_fd();

    // Enable Non Blocking read and write.
    fcntl(stdin_fd, F_SETFL, Some(O_NONBLOCK))?;
    Ok(())
}

pub(crate) fn disable_non_blocking_stdio() -> Result<(), Error> {
    let stdin_fd = std::io::stdin().as_raw_fd();

    // Disable Non Blocking read and write.
    let fd_flags = fcntl(stdin_fd, F_GETFL, None)?;
    fcntl(stdin_fd, F_SETFL, Some(fd_flags & !O_NONBLOCK))?;
    Ok(())
}

pub(crate) fn enter_raw_mode() -> Result<(), Error> {
    let stdout_fd = std::io::stdout().as_raw_fd();

    // Switch to raw mode (Non cannonical and no echo)
    let mut t: Termios = Default::default();
    tcgetattr(stdout_fd, &mut t)?;

    t.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdout_fd, TCSAFLUSH, &t)?;
    Ok(())
}

pub(crate) fn enter_canon_mode() -> Result<(), Error> {
    let stdout_fd = std::io::stdout().as_raw_fd();

    // Disable to raw mode and enable echoing
    let mut t: Termios = Default::default();
    tcgetattr(stdout_fd, &mut t)?;

    //t.c_lflag |= ICANON | ECHO;
    t.c_lflag |= ICANON;
    tcsetattr(stdout_fd, TCSAFLUSH, &t)?;
    Ok(())
}

/// Moves cursor at the position (x, y).
///
/// (0,0) is the top left corner.
///
/// # Arguments
/// * `x` - 0 based number representing X axis.
/// * `y` - 0 based number representing Y axis.
///
pub(crate) fn set_cursor_position(x: usize, y: usize) {
    print!("\x1b[{};{}f", y + 1, x + 1);
}

pub(crate) fn set_cursor_color(bg: BackgroundColors, fg: ForegroundColors) {
    print!("\x1b[{};{}m", bg as u32, fg as u32);
}

pub(crate) fn reset_cursor_color() {
    print!("\x1b[0m");
}

fn get_keybytes() -> Option<String> {
    let mut key_bytes = String::default();

    loop {
        match getchar() {
            Err(_) => {
                return match key_bytes.len() {
                    0 => None,
                    _ => Some(key_bytes),
                };
            }
            Ok(c) => key_bytes.push(c),
        }
    }
}

/// Returns keyboard key presses without blocking.
pub fn get_keypress() -> Result<Keys, Error> {
    // Note:
    // Settings non blocking flag to stdin, has an undesired side-effect in Linux - both the stdin
    // and stdout become non blocking. This causes the undesired issue, of output buffer overflow.
    // To prevent this, we enter non blocking mode just before reading keys and then disable this
    // flag immediately after.
    enable_non_blocking_stdio()?;
    let bytes = get_keybytes();
    disable_non_blocking_stdio()?;

    if let Some(bytes) = bytes {
        return match bytes.as_str() {
            "\n" => Ok(Keys::Enter),
            KEY_UP => Ok(Keys::Up),
            KEY_DOWN => Ok(Keys::Down),
            KEY_LEFT => Ok(Keys::Left),
            KEY_RIGHT => Ok(Keys::Right),
            _ => Ok(Keys::Other(bytes)),
        };
    }

    Ok(Keys::Nothing)
}
