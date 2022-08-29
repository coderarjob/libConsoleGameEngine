use libarl::ffi::*;
use std::io::Error;
use std::os::unix::io::*;

const KEY_UP: &str = "\x1b[A";
const KEY_RIGHT: &str = "\x1b[C";
const KEY_DOWN: &str = "\x1b[B";
const KEY_LEFT: &str = "\x1b[D";

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

pub fn enable_non_blocking_stdio() -> Result<(), Error> {
    let stdin_fd = std::io::stdin().as_raw_fd();

    // Enable Non Blocking read and write.
    libc_fcntl(stdin_fd, F_SETFL, Some(O_NONBLOCK))?;
    Ok(())
}

pub fn disable_non_blocking_stdio() -> Result<(), Error> {
    let stdin_fd = std::io::stdin().as_raw_fd();

    // Disable Non Blocking read and write.
    let fd_flags = libc_fcntl(stdin_fd, F_GETFL, None)?;
    libc_fcntl(stdin_fd, F_SETFL, Some(fd_flags & !O_NONBLOCK))?;
    Ok(())
}

pub fn enter_raw_mode() -> Result<(), Error> {
    let stdout_fd = std::io::stdout().as_raw_fd();

    // Switch to raw mode (Non cannonical and no echo)
    let mut t: Termios = Default::default();
    libc_tcgetattr(stdout_fd, &mut t)?;

    t.c_lflag &= !(ICANON | ECHO);
    libc_tcsetattr(stdout_fd, TCSAFLUSH, &t)?;
    Ok(())
}

pub fn enter_canon_mode() -> Result<(), Error> {
    let stdout_fd = std::io::stdout().as_raw_fd();

    // Disable to raw mode and enable echoing
    let mut t: Termios = Default::default();
    libc_tcgetattr(stdout_fd, &mut t)?;

    //t.c_lflag |= ICANON | ECHO;
    t.c_lflag |= ICANON;
    libc_tcsetattr(stdout_fd, TCSAFLUSH, &t)?;
    Ok(())
}

fn get_keybytes() -> Option<String> {
    let mut key_bytes = String::default();

    loop {
        match libc_getchar() {
            Err(GetcharErrors::EOF) => {
                return match key_bytes.len() {
                    0 => None,
                    _ => Some(key_bytes),
                };
            }
            Ok(c) => key_bytes.push(c),
        }
    }
}

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
