use bootloader_api::info::FrameBuffer;
use spin::Mutex;

use super::{writer::FrameBufferWriter, WRITER};

pub fn init(frame_buffer: FrameBuffer) {
    let info = frame_buffer.info();
    let frame_buffer = frame_buffer.into_buffer();

    WRITER.call_once(|| Mutex::new(FrameBufferWriter::new(frame_buffer, info)));
}
