use std::os;
use std::io::File;

mod gif;

fn main() {
    let args = os::args();

    if args.len() < 2 {
        println!("Usage: {} <gif file>", args[0]);
        return;
    }

    let path = &Path::new(args[1]);

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => return println!("Could not open file: {}", err.desc)
    };

    let gif = match gif::read(&mut file) {
        Ok(gif) => gif,
        Err(msg) => return println!("Could not read GIF: {}", msg)
    };

    println!("{}", gif);
}
