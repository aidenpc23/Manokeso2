use glyphon::{
    Attrs, Buffer, Color, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea,
    TextAtlas, TextBounds, TextRenderer,
};
use wgpu::{Device, MultisampleState, Queue, RenderPass, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

use crate::{
    client::ui::text::Align,
    render::surface::RenderSurface,
    util::point::Point,
};

pub struct UIText {
    pub renderer: TextRenderer,
    pub font_system: FontSystem,
    pub atlas: TextAtlas,
    pub cache: SwashCache,
    pub text_buffers: Vec<glyphon::Buffer>,
}

impl UIText {
    pub fn init(device: &Device, queue: &Queue, config: &SurfaceConfiguration) -> Self {
        let font_system = FontSystem::new();
        let cache = SwashCache::new();
        let mut atlas = TextAtlas::new(&device, &queue, config.format);
        let renderer = TextRenderer::new(&mut atlas, &device, MultisampleState::default(), None);

        Self {
            font_system,
            atlas,
            cache,
            renderer,
            text_buffers: Vec::new(),
        }
    }

    pub fn draw<'a>(&'a self, pass: &mut RenderPass<'a>) {
        self.renderer.render(&self.atlas, pass).unwrap();
    }

    pub fn update(
        &mut self,
        size: &PhysicalSize<u32>,
        surface: &RenderSurface,
        text: &[TextElement],
    ) {
        let buffers = &mut self.text_buffers;
        if buffers.len() < text.len() {
            buffers.extend(
                (0..(text.len() - buffers.len()))
                    .map(|_| Buffer::new(&mut self.font_system, Metrics::new(20.0, 25.0))),
            )
        }
        for (buffer, text) in buffers.iter_mut().zip(text) {
            buffer.set_text(
                &mut self.font_system,
                &text.content,
                Attrs::new().family(Family::SansSerif),
                Shaping::Advanced,
            );
            buffer.set_size(&mut self.font_system, text.bounds.0, text.bounds.1);
        }
        let color = Color::rgb(255, 255, 255);
        let areas = buffers.iter().zip(text).map(|(buffer, text)| {
            let width = measure(&buffer).0;
            let left = text.pos.x
                - match text.align {
                    Align::Left => 0.0,
                    Align::Center => width / 2.0,
                    Align::Right => width,
                };
            TextArea {
                buffer: &buffer,
                left,
                top: text.pos.y,
                scale: 1.0,
                bounds: TextBounds::default(),
                default_color: color,
            }
        });
        self.renderer
            .prepare(
                &surface.device,
                &surface.queue,
                &mut self.font_system,
                &mut self.atlas,
                Resolution {
                    width: size.width,
                    height: size.height,
                },
                areas,
                &mut self.cache,
            )
            .unwrap();
    }
}

fn measure(buffer: &glyphon::Buffer) -> (f32, f32) {
    let (width, total_lines) = buffer
        .layout_runs()
        .fold((0.0, 0usize), |(width, total_lines), run| {
            (run.line_w.max(width), total_lines + 1)
        });

    (width, total_lines as f32 * buffer.metrics().line_height)
}

pub struct TextElement {
    pub content: String,
    pub align: Align,
    pub pos: Point<f32>,
    pub bounds: (f32, f32),
}
