use std::num::NonZeroU64;

use wgpu::{util::StagingBelt, Buffer, BufferViewMut, CommandEncoder, Device};

pub struct StagingBufWriter<'a> {
    pub device: &'a Device,
    pub encoder: &'a mut CommandEncoder,
    pub belt: &'a mut StagingBelt,
}

impl StagingBufWriter<'_> {
    pub fn mut_view<T>(&mut self, buffer: &Buffer, size: usize) -> BufferViewMut {
        self.belt.write_buffer(
            self.encoder,
            buffer,
            0,
            unsafe { NonZeroU64::new_unchecked((size * std::mem::size_of::<T>()) as u64) },
            self.device,
        )
    }
}
