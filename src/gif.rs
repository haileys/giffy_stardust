use std::io::{Reader, IoResult};
use std::vec::Vec;

#[deriving(Show)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn read_color(reader: &mut Reader) -> IoResult<Color> {
    let r = match reader.read_u8() { Ok(r) => r, Err(e) => return Err(e) };
    let g = match reader.read_u8() { Ok(g) => g, Err(e) => return Err(e) };
    let b = match reader.read_u8() { Ok(b) => b, Err(e) => return Err(e) };

    Ok(Color { r: r, g: g, b: b })
}

#[deriving(Show)]
pub struct Gif {
    pub width: u16,
    pub height: u16,
    background_color_index: u8,
    pixel_aspect_ratio: u8,
    gct: Vec<Color>,
}

pub fn read(r: &mut Reader) -> Result<Gif, &'static str> {
    let gif89a = match r.read_exact(6) {
        Ok(buf) => buf,
        Err(err) => return Err(err.desc)
    };

    if gif89a != "GIF89a".to_string().into_bytes() {
        return Err("not a GIF89a")
    }

    let width = match r.read_le_u16() {
        Ok(width) => width,
        Err(err) => return Err(err.desc)
    };

    let height = match r.read_le_u16() {
        Ok(height) => height,
        Err(err) => return Err(err.desc)
    };

    let flags = match r.read_u8() {
        Ok(flags) => flags,
        Err(err) => return Err(err.desc)
    };

    let gct_size = if (flags & 1) == 1 {
        1 << ((flags as uint >> 5) + 1)
    } else {
        0
    };

    let background_color_index = match r.read_u8() {
        Ok(background_color_index) => background_color_index,
        Err(err) => return Err(err.desc),
    };

    let pixel_aspect_ratio = match r.read_u8() {
        Ok(pixel_aspect_ratio) => pixel_aspect_ratio,
        Err(err) => return Err(err.desc),
    };

    let mut gct = Vec::new();

    for _ in range(0u, gct_size) {
        match read_color(r) {
            Ok(color) => gct.push(color),
            Err(err) => return Err(err.desc),
        };
    }

    Ok(Gif {
        width: width,
        height: height,
        background_color_index: background_color_index,
        pixel_aspect_ratio: pixel_aspect_ratio,
        gct: gct,
    })
}
