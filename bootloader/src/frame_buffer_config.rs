use uefi::proto::console::gop;
use uefi::proto::console::gop::{BltOp, BltPixel, FrameBuffer, GraphicsOutput, PixelFormat};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FrameBufferConfig<'a> {
    pub frame_buffer: *mut FrameBuffer<'a>,
    pub horizontal: (usize, usize),
    pub vertical: (usize, usize),
    pub format: PixelFormat,
}
