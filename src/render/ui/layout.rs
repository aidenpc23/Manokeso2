use wgpu_glyph::{Section, Text};

use crate::{input::Input, state::GameState};

#[derive(Default)]
pub struct UIText {
    pub performance_stats: String,
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
    vec![
        Section {
            screen_position: (300.0, 10.0),
            bounds,
            text: vec![Text::new(&text.performance_stats)
                .with_color([0.0, 0.0, 0.0, 1.0])
                .with_scale(30.0)],
            ..Section::default()
        },
        Section {
            screen_position: (10.0, 10.0),
            bounds,
            text: vec![Text::new(&text.tile_info)
                .with_color([0.0, 0.0, 0.0, 1.0])
                .with_scale(30.0)],
            ..Section::default()
        },
    ]
}
