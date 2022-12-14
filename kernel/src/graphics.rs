use crate::ascii::FONTS;

#[derive(Debug, Copy, Clone)]
pub enum PixelFormat {
    // Each pixel is 32-bit long, with 24-bit RGB, and the last byte is reserved.
    Rgb = 0,

    // Each pixel is 32-bit long, with 24-bit BGR, and the last byte is reserved.
    Bgr,

    // Custom pixel format, check the associated bitmask.
    Bitmask,

    // The graphics mode does not support drawing directly to the frame buffer.
    //
    // This means you will have to use the `blt` function which will
    // convert the graphics data to the device's internal pixel format.
    BltOnly,
    // SAFETY: UEFI also defines a PixelFormatMax variant, and states that all
    //         valid enum values are guaranteed to be smaller. Since that is the
    //         case, adding a new enum variant would be a breaking change, so it
    //         is safe to model this C enum as a Rust enum.
}

/// Bitmask used to indicate which bits of a pixel represent a given color.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct PixelBitmask {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
    pub reserved: u32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ModeInfo {
    pub version: u32,
    pub hor_res: u32,
    pub ver_res: u32,
    pub format: PixelFormat,
    pub mask: PixelBitmask,
    pub stride: u32,
}

impl ModeInfo {
    pub fn resolution(&self) -> (usize, usize) {
        (self.hor_res as usize, self.ver_res as usize)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FrameBuffer {
    base: *mut u8,
    size: usize,
}

impl FrameBuffer {
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.base
    }

    pub fn size(&self) -> usize {
        self.size
    }

    // Write to th index-th byte of the framebuffer
    //
    // # Safety
    // This is unsafe : no bound check.
    pub unsafe fn write_byte(&mut self, index: usize, val: u8) {
        self.base.add(index).write_volatile(val);
    }

    // Write to th index-th byte of the framebuffer
    //
    // # Safety
    // This is unsafe : no bound check.
    pub unsafe fn write_value(&mut self, index: usize, value: [u8; 3]) {
        (self.base.add(index) as *mut [u8; 3]).write_volatile(value)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PixelColor(pub u8, pub u8, pub u8); // RGB

#[derive(Copy, Clone)]
pub struct Graphics {
    fb: FrameBuffer,
    mi: ModeInfo,
    pixel_writer: unsafe fn(&mut FrameBuffer, usize, PixelColor),
}

impl Graphics {
    // ???????????????????????????
    pub fn new(fb: FrameBuffer, mi: ModeInfo) -> Self {
        unsafe fn write_pixel_rgb(fb: &mut FrameBuffer, index: usize, rgb: PixelColor) {
            fb.write_value(index, [rgb.0, rgb.1, rgb.2]);
        }
        unsafe fn write_pixel_bgr(fb: &mut FrameBuffer, index: usize, rgb: PixelColor) {
            fb.write_value(index, [rgb.2, rgb.1, rgb.0]);
        }

        // pixel_writter??????????????????(?????????????????????????????????????????????????????????????????????)
        let pixel_writer = match mi.format {
            PixelFormat::Rgb => write_pixel_rgb,
            PixelFormat::Bgr => write_pixel_bgr,
            _ => {
                panic!("This pixel format is not supported by the drawing demo");
            }
        };

        Graphics {
            fb,
            mi,
            pixel_writer,
        }
    }

    // Write to th index-th byte of the framebuffer
    //
    // # Safety
    // This is unsafe : no bound check.
    pub unsafe fn write_pixel(&mut self, x: usize, y: usize, color: PixelColor) {
        if x > self.mi.hor_res as usize {
            panic!("bad x coord");
        }
        if y > self.mi.ver_res as usize {
            panic!("bad x coord");
        }
        let pixel_index = y * (self.mi.stride as usize) + x;
        let base = 4 * pixel_index;
        (self.pixel_writer)(&mut self.fb, base, color);
    }

    pub fn putchar(&mut self, x: usize, y: usize, c: char, color: PixelColor) {
        let string = FONTS[c as usize];
        for (dy, line) in string.iter().enumerate() {
            for dx in 0..8 {
                if (line << dx) & 0x80 != 0 {
                    unsafe { self.write_pixel(x + dx, y + dy, color) };
                }
            }
        }
    }

    pub fn print(&mut self, x: usize, y: &mut usize, word: &str, color: PixelColor) {
        let mut add_str = 1;
        const W_DISTANCE: usize = 8;

        for c in word.chars() {
            if c == '\n' {
                add_str = 1;
                *y += 18;
                continue;
            }
            self.putchar(x + W_DISTANCE * add_str, *y, c, color);
            add_str += 1;
        }
    }

    pub fn resolution(&self) -> (usize, usize) {
        self.mi.resolution()
    }

    pub fn fb(&self) -> FrameBuffer {
        self.fb
    }

    pub fn mi(&self) -> ModeInfo {
        self.mi
    }
}
