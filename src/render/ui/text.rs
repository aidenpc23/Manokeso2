use glyphon::{
    Attrs, Buffer, Color, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea,
    TextAtlas, TextBounds, TextRenderer,
};
use wgpu::{Device, MultisampleState, Queue, RenderPass, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

use crate::{state::ClientState, util::point::Point, render::surface::RenderSurface};

use super::layout;

pub type TextUpdate = fn(&ClientState, &RenderSurface) -> String;

pub struct Text {
    pub update: TextUpdate,
    pub align: Align,
    pub pos: fn((f32, f32)) -> Point<f32>,
    pub bounds: fn((f32, f32)) -> (f32, f32),
}

pub struct TextElement {
    pub buffer: glyphon::Buffer,
    pub update: TextUpdate,
    pub align: Align,
    pub pos: fn((f32, f32)) -> Point<f32>,
    pub bounds: fn((f32, f32)) -> (f32, f32),
}

impl TextElement {
    pub fn new(system: &mut FontSystem, desc: Text) -> Self {
        Self {
            buffer: Buffer::new(system, Metrics::new(20.0, 25.0)),
            update: desc.update,
            align: desc.align,
            pos: desc.pos,
            bounds: desc.bounds,
        }
    }
}

pub enum Align {
    Left,
    Center,
    Right,
}

pub struct UIText {
    pub elements: Vec<TextElement>,
    pub renderer: TextRenderer,
    pub font_system: FontSystem,
    pub atlas: TextAtlas,
    pub cache: SwashCache,
}

impl UIText {
    pub fn init(device: &Device, queue: &Queue, config: &SurfaceConfiguration) -> Self {
        let mut font_system = FontSystem::new();
        let cache = SwashCache::new();
        let mut atlas = TextAtlas::new(&device, &queue, config.format);
        let renderer = TextRenderer::new(&mut atlas, &device, MultisampleState::default(), None);

        Self {
            elements: layout::BOARD
                .into_iter()
                .map(|t| TextElement::new(&mut font_system, t))
                .collect(),
            font_system,
            atlas,
            cache,
            renderer,
        }
    }

    pub fn draw<'a>(&'a self, pass: &mut RenderPass<'a>) {
        self.renderer.render(&self.atlas, pass).unwrap();
    }

    pub fn update(
        &mut self,
        state: &ClientState,
        size: &PhysicalSize<u32>,
        surface: &RenderSurface,
    ) {
        let bounds = (size.width as f32, size.height as f32);
        for element in &mut self.elements {
            element.buffer.set_text(
                &mut self.font_system,
                &(element.update)(state, surface),
                Attrs::new().family(Family::SansSerif),
                Shaping::Advanced,
            );
            let size = (element.bounds)(bounds);
            element
                .buffer
                .set_size(&mut self.font_system, size.0, size.1);
        }
        let color = Color::rgb(255, 255, 255);
        let areas = self.elements.iter().map(|e| {
            let width = measure(&e.buffer).0;
            let pos = (e.pos)(bounds);
            let left = pos.x
                - match e.align {
                    Align::Left => 0.0,
                    Align::Center => width / 2.0,
                    Align::Right => width,
                };
            TextArea {
                buffer: &e.buffer,
                left,
                top: pos.y,
                scale: 1.0,
                bounds: TextBounds {
                    left: 0,
                    top: 0,
                    right: bounds.0 as i32,
                    bottom: bounds.1 as i32,
                },
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
