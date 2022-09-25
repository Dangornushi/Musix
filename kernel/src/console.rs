use crate::graphics::{FrameBuffer, Graphics, ModeInfo, PixelColor};

#[derive(Copy, Clone)]

pub struct Console {
    cursor_x: usize,
    cursor_y: usize,
    w: usize,
    h: usize,
    back_color: PixelColor,
    font_color: PixelColor,
    console_graphic: Graphics,
}

impl Console {
    pub fn new(fb: FrameBuffer, mi: ModeInfo) -> Self {
        let (width, height) = mi.resolution();
        Self {
            cursor_x: 0,
            cursor_y: 0,
            w: width,
            h: height,
            back_color: PixelColor(10, 10, 10),
            font_color: PixelColor(136, 233, 84),
            console_graphic: Graphics::new(fb, mi),
        }
    }

    pub fn print(&mut self, word: &str) {
        (self.console_graphic).print(self.cursor_x, self.cursor_y, word, self.font_color);
    }
}
