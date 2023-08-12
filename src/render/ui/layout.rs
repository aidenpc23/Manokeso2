use wgpu_glyph::{BuiltInLineBreaker, HorizontalAlign, Section, Text, VerticalAlign};

use crate::{input::Input, state::GameState};

#[derive(Default)]
pub struct UIText {
    pub performance_stats: String,
    pub total_energy: String,
    pub tile_info: String,
}

impl UIText {
    pub fn new() -> Self {
        Default::default()
    }
}

pub fn create_sections<'a>(
    state: &GameState,
    input: &Input,
    text: &'a mut UIText,
    bounds: (f32, f32),
) -> Vec<Section<'a>> {
    text.performance_stats = format!(
        "frame time: {:?}\nupdate time: {:?}",
        state.timers.render.avg(),
        state.timers.update.avg()
    );
    text.total_energy = format!(
        "total energy: {:?}", state.board.total_energy()
    );
    if let Some(pos) = state.board.tile_at(input.mouse_tile_pos) {
        let b = &state.board;
        let i = pos[0] + pos[1] * b.width();
        text.tile_info = format!(
            concat!(
                "tile pos: {:?}\n",
                "connex number: {:?}\n",
                "stability: {:?}\n",
                "reactivity: {:?}\n",
                "energy: {:?}"
            ),
            pos,
            b.connex_numbers.read()[i],
            b.stability.read()[i],
            b.reactivity.read()[i],
            b.energy.read()[i]
        );
    }

    let perf = Section {
        screen_position: (bounds.0 - 10.0, 10.0),
        bounds,
        text: vec![Text::new(&text.performance_stats)
            .with_color([1.0, 1.0, 1.0, 1.0])
            .with_scale(30.0)],
        layout: wgpu_glyph::Layout::SingleLine {
            line_breaker: BuiltInLineBreaker::default(),
            h_align: HorizontalAlign::Right,
            v_align: VerticalAlign::Top,
        },
    };
    let total_energy = Section {
        screen_position: (bounds.0 / 2.0, 10.0),
        bounds,
        text: vec![Text::new(&text.total_energy)
            .with_color([1.0, 1.0, 1.0, 1.0])
            .with_scale(30.0)],
        layout: wgpu_glyph::Layout::SingleLine {
            line_breaker: BuiltInLineBreaker::default(),
            h_align: HorizontalAlign::Center,
            v_align: VerticalAlign::Top,
        },
    };
    let tile_info = Section {
        screen_position: (10.0, 10.0),
        bounds,
        text: vec![Text::new(&text.tile_info)
            .with_color([1.0, 1.0, 1.0, 1.0])
            .with_scale(30.0)],
        ..Section::default()
    };

    vec![perf, total_energy, tile_info]
}
