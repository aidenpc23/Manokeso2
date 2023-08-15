use glyphon::{
    Attrs, Buffer, Color, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea,
    TextAtlas, TextBounds, TextRenderer,
};
use wgpu::{Device, MultisampleState, Queue, RenderPass, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

use crate::state::GameState;

pub struct UIText {
    pub buffers: UITextBuffers,
    pub renderer: TextRenderer,
    pub font_system: FontSystem,
    pub atlas: TextAtlas,
    pub cache: SwashCache,
}

pub struct UITextBuffers {
    pub performance_stats: Buffer,
    pub total_energy: Buffer,
    pub tile_info: Buffer,
}

impl UITextBuffers {
    pub fn init(system: &mut FontSystem, size: &PhysicalSize<u32>, scale: f64) -> Self {
        let width = (size.width as f64 * scale) as f32;
        let height = (size.height as f64 * scale) as f32;

        let mut performance_stats = Buffer::new(system, Metrics::new(20.0, 25.0));
        let mut total_energy = Buffer::new(system, Metrics::new(20.0, 25.0));
        let mut tile_info = Buffer::new(system, Metrics::new(20.0, 25.0));
        performance_stats.set_size(system, width, height);
        total_energy.set_size(system, width, height);
        tile_info.set_size(system, width, height);

        Self {
            performance_stats,
            total_energy,
            tile_info,
        }
    }
}

impl UIText {
    pub fn init(
        device: &Device,
        queue: &Queue,
        config: &SurfaceConfiguration,
        size: &PhysicalSize<u32>,
        scale: f64,
    ) -> Self {
        let mut font_system = FontSystem::new();
        let cache = SwashCache::new();
        let mut atlas = TextAtlas::new(&device, &queue, config.format);
        let renderer = TextRenderer::new(&mut atlas, &device, MultisampleState::default(), None);

        let buffers = UITextBuffers::init(&mut font_system, size, scale);

        Self {
            font_system,
            atlas,
            cache,
            renderer,
            buffers,
        }
    }

    pub fn draw<'a>(&'a self, pass: &mut RenderPass<'a>) {
        self.renderer.render(&self.atlas, pass).unwrap();
    }

    pub fn update(
        &mut self,
        state: &GameState,
        size: &PhysicalSize<u32>,
        device: &Device,
        queue: &Queue,
    ) {
        self.update_text(state);
        let areas = Self::create_areas(&self.buffers, (size.width as f32, size.height as f32));
        self.renderer
            .prepare(
                device,
                queue,
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

    pub fn update_text(&mut self, state: &GameState) {
        let buffers = &mut self.buffers;
        let font_system = &mut self.font_system;
        let attrs = Attrs::new().family(Family::SansSerif);
        let shaping = Shaping::Advanced;

        buffers.performance_stats.set_text(
            font_system,
            &format!(
                concat!("frame time: {:?}\n", "update time: {:?}",),
                state.timers.render.avg(),
                state.timers.update.avg()
            ),
            attrs,
            shaping,
        );
        buffers.total_energy.set_text(
            font_system,
            &format!("total energy: {:?}", state.board.total_energy()),
            attrs,
            shaping,
        );
        buffers.tile_info.set_text(
            font_system,
            &if let Some(pos) = state.hovered_tile {
                let b = &state.board;
                let i = pos.index(b.width());
                format!(
                    concat!(
                        "tile pos: {:?}\n",
                        "connex number: {:?}\n",
                        "stability: {:?}\n",
                        "reactivity: {:?}\n",
                        "energy: {:?}\n",
                        "alpha: {:?}\n",
                        "beta: {:?}\n",
                        "gamma: {:?}\n",
                        "delta: {:?}\n",
                        "omega: {:?}\n",
                    ),
                    pos,
                    b.connex_numbers.read()[i],
                    b.stability.read()[i],
                    b.reactivity.read()[i],
                    b.energy.read()[i],
                    b.alpha.read()[i],
                    b.beta.read()[i],
                    b.gamma.read()[i],
                    b.delta.read()[i],
                    b.omega.read()[i],
                )
            } else {
                "no tile selected".to_string()
            },
            attrs,
            shaping,
        );
    }

    pub fn create_areas(buffers: &UITextBuffers, sbounds: (f32, f32)) -> Vec<TextArea> {
        let bounds = TextBounds {
            left: 0,
            top: 0,
            right: sbounds.0 as i32,
            bottom: sbounds.1 as i32,
        };
        let color = Color::rgb(255, 255, 255);
        let padding = 10.0;

        let tile_info = TextArea {
            buffer: &buffers.tile_info,
            left: padding,
            top: padding,
            scale: 1.0,
            bounds,
            default_color: color,
        };
        let w = measure(&buffers.total_energy).0;
        let total_energy = TextArea {
            buffer: &buffers.total_energy,
            left: sbounds.0 / 2.0 - w / 2.0,
            top: padding,
            scale: 1.0,
            bounds,
            default_color: color,
        };
        let w = measure(&buffers.performance_stats).0;
        let performance = TextArea {
            buffer: &buffers.performance_stats,
            left: sbounds.0 - padding - w,
            top: padding,
            scale: 1.0,
            bounds,
            default_color: color,
        };
        vec![tile_info, total_energy, performance]
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
