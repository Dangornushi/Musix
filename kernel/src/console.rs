use crate::graphics::{FrameBuffer, Graphics, ModeInfo, PixelColor};
use crate::println;
use core::fmt;
use core::{fmt::Write, mem::MaybeUninit};
use x86_64::instructions::port::{PortReadOnly, PortWriteOnly};

static mut RAW_CONSOLE: MaybeUninit<Console> = MaybeUninit::<Console>::uninit();

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
            cursor_x: 10,
            cursor_y: 10,
            w: width,
            h: height,
            back_color: PixelColor(10, 10, 10),
            font_color: PixelColor(136, 233, 84),
            console_graphic: Graphics::new(fb, mi),
        }
    }

    pub fn instance() -> &'static mut Console {
        unsafe { &mut *RAW_CONSOLE.as_mut_ptr() }
    }

    pub fn start(&mut self) {
        let ascii: &str = "
 /$$      /$$                     /$$
| $$$    /$$$                    |__/
| $$$$  /$$$$ /$$   /$$  /$$$$$$$ /$$ /$$   /$$
| $$ $$/$$ $$| $$  | $$ /$$_____/| $$|  $$ /$$/
| $$  $$$| $$| $$  | $$|  $$$$$$ | $$ \\  $$$$/
| $$\\  $| $$| $$  | $$ \\____  $$| $$  >$$  $$
| $$ \\/| $$ |  $$$$$$/ /$$$$$$$/| $$ /$$/\\  $$
|__/    |__/ \\______/ |_______/ |__/|__/  \\__/\n 
";

        self.background_render();
        self.print(ascii);
    }

    pub fn print(&mut self, word: &str) {
        (self.console_graphic).print(self.cursor_x, &mut self.cursor_y, word, self.font_color);
        self.cursor_y += 18;
    }

    pub fn background_render(&mut self) {
        for y in 0..(self.h) {
            for x in 0..(self.w) {
                unsafe { self.console_graphic.write_pixel(x, y, self.back_color) };
            }
        }
    }
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s);
        Ok(())
    }
}
