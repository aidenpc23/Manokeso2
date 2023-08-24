use glyphon::{
    Attrs, Buffer, Color, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea,
    TextAtlas, TextBounds, TextRenderer,
};
use wgpu::{MultisampleState, RenderPass};

use crate::{
    client::ui::text::Align,
    render::{primitive::TextElement, surface::RenderSurface},
};

pub struct TextPipeline {
    pub renderer: glyphon::TextRenderer,
    pub font_system: glyphon::FontSystem,
    pub atlas: glyphon::TextAtlas,
    pub cache: glyphon::SwashCache,
    pub text_buffers: Vec<glyphon::Buffer>,
    pub old_text: Vec<TextElement>,
}

impl TextPipeline {
    pub fn new(surface: &RenderSurface) -> Self {
        let RenderSurface {
            device,
            config,
            queue,
            ..
        } = surface;

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
            old_text: Vec::new(),
        }
    }

    pub fn draw<'a>(&'a self, pass: &mut RenderPass<'a>) {
        self.renderer.render(&self.atlas, pass).unwrap();
    }

    pub fn update(&mut self, surface: &RenderSurface, text: &[TextElement]) {
        let buffers = &mut self.text_buffers;
        if buffers.len() < text.len() {
            self.old_text.resize(text.len(), TextElement::empty());
            buffers.resize_with(text.len(), || {
                Buffer::new(&mut self.font_system, Metrics::new(20.0, 25.0))
            })
        }
        for ((buffer, text), old) in buffers.iter_mut().zip(text).zip(&mut self.old_text) {
            if text != old {
                *old = text.clone();
                buffer.set_size(&mut self.font_system, text.bounds.0, text.bounds.1);
                buffer.set_text(
                    &mut self.font_system,
                    &text.content,
                    Attrs::new().family(Family::SansSerif),
                    Shaping::Basic,
                );
            }
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
                    width: surface.config.width,
                    height: surface.config.height,
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
