extern crate sdl2;

use std::os;
use std::io::File;

mod gif;

fn main() {
    let args = os::args();

    if args.len() < 2 {
        println!("Usage: {} <gif file>", args[0]);
        return;
    }

    let path = &Path::new(args[1].as_bytes());

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => return println!("Could not open file: {}", err.desc)
    };

    let gif = match gif::read(&mut file) {
        Ok(gif) => gif,
        Err(msg) => return println!("Could not read GIF: {}", msg)
    };

    sdl2::init(sdl2::INIT_VIDEO);

    let window = match sdl2::video::Window::new(
            "giffy_stardust",
            sdl2::video::PosCentered,
            sdl2::video::PosCentered,
            gif.width as int,
            gif.height as int,
            sdl2::video::OPENGL) {
        Ok(window) => window,
        Err(err) => return println!("Could not create window: {}", err),
    };

    loop {
        match sdl2::event::poll_event() {
            sdl2::event::QuitEvent(_) => break,
            sdl2::event::KeyDownEvent(_, _, sdl2::keycode::EscapeKey, _, _) => break,
            _ => {},
        }
    };

    sdl2::quit();
}
