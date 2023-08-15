use glyphon::{TextArea, TextBounds, Color, Buffer, FontSystem, Metrics};

use crate::state::GameState;

pub struct UITextBuffers {
    pub performance_stats: Buffer,
    pub total_energy: Buffer,
    pub tile_info: Buffer,
}

impl UITextBuffers {
    pub fn init(system: &mut FontSystem) -> Self {
        Self {
            performance_stats: Buffer::new(system, Metrics::new(30.0, 42.0)),
            total_energy: Buffer::new(system, Metrics::new(30.0, 42.0)),
            tile_info: Buffer::new(system, Metrics::new(30.0, 42.0))
        }
    }
}

const PADDING: f32 = 10.0;
const SIZE: f32 = 30.0;
const COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub fn create_sections<'a>(
    state: &GameState,
    text: &mut UITextBuffers,
    bounds: (f32, f32),
    buffer: &'a Buffer,
) -> Vec<TextArea<'a>> {
    text.performance_stats = format!(
        concat!(
            "frame time: {:?}\n",
            "update time: {:?}",
        ),
        state.timers.render.avg(),
        state.timers.update.avg()
    );
    text.total_energy = format!("total energy: {:?}", state.board.total_energy());
    text.tile_info = if let Some(pos) = state.hovered_tile {
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
    };

    let w = measure(buffer).0;
    let perf = TextArea {
        buffer,
        left: bounds.0 - 10.0 - w,
        top: 10.0,
        scale: 1.0,
        bounds: TextBounds {
            left: 0,
            top: 0,
            right: bounds.0 as i32,
            bottom: bounds.1 as i32,
        },
        default_color: Color::rgb(255, 255, 255),
    };
    // let perf = Section {
    //     screen_position: (bounds.0 - PADDING, PADDING),
    //     bounds,
    //     text: vec![Text::new(&text.performance_stats)
    //         .with_color(COLOR)
    //         .with_scale(SIZE)],
    //     layout: wgpu_glyph::Layout::Wrap {
    //         line_breaker: BuiltInLineBreaker::default(),
    //         h_align: HorizontalAlign::Right,
    //         v_align: VerticalAlign::Top,
    //     },
    // };
    // let total_energy = Section {
    //     screen_position: (bounds.0 / 2.0, PADDING),
    //     bounds,
    //     text: vec![Text::new(&text.total_energy)
    //         .with_color(COLOR)
    //         .with_scale(SIZE)],
    //     layout: wgpu_glyph::Layout::SingleLine {
    //         line_breaker: BuiltInLineBreaker::default(),
    //         h_align: HorizontalAlign::Center,
    //         v_align: VerticalAlign::Top,
    //     },
    // };
    // let tile_info = Section {
    //     screen_position: (PADDING, PADDING),
    //     bounds,
    //     text: vec![Text::new(&text.tile_info)
    //         .with_color(COLOR)
    //         .with_scale(SIZE)],
    //     ..Section::default()
    // };

    vec![perf]
}

fn measure(buffer: &glyphon::Buffer) -> (f32, f32) {
    let (width, total_lines) = buffer
        .layout_runs()
        .fold((0.0, 0usize), |(width, total_lines), run| {
            (run.line_w.max(width), total_lines + 1)
        });

    (width, total_lines as f32 * buffer.metrics().line_height)
}

