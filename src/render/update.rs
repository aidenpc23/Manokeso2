use super::Renderer;

impl Renderer {
    pub fn write_instances(&self) {
        self.queue.write_buffer(
            &self.buffer.instance,
            0,
            bytemuck::cast_slice(&self.instances),
        );
    }
}
