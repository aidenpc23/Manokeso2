use crate::{
    util::point::Point, world::decode_alpha,
};

use super::text::{Text, Align};

pub const BOARD: [Text; 3] = [
    Text {
        content: |state| {
            if let Some(tile) = &state.hovered_tile {
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
                    tile.pos,
                    tile.connex_number,
                    tile.stability,
                    tile.reactivity,
                    tile.energy,
                    decode_alpha(tile.alpha),
                    tile.beta,
                    tile.gamma,
                    tile.delta,
                    tile.omega,
                )
            } else {
                "no tile selected".to_string()
            }
        },
        pos: |(_, _)| Point { x: 10.0, y: 10.0 },
        align: Align::Left,
        bounds: |(w, h)| (w / 3.0, h),
    },
    Text {
        content: |state| format!("total energy: {}", state.world.view_info.total_energy),
        pos: |(w, _)| Point {
            x: w / 2.0,
            y: 10.0,
        },
        align: Align::Center,
        bounds: |(w, h)| (w / 3.0, h),
    },
    Text {
        content: |state| {
            let adp_info = state.renderer.render_surface.adapter.get_info();
            format!(
                "adapter: {}\nbackend: {:?}\nclient update: {:.3}ms\nworld update: {:.3}ms",
                adp_info.name,
                adp_info.backend,
                state.timer.avg().as_secs_f32() * 1000.0,
                state.world.view_info.time_taken.as_secs_f32() * 1000.0
            )
        },
        pos: |(w, _)| Point {
            x: w - 10.0,
            y: 10.0,
        },
        align: Align::Right,
        bounds: |(w, h)| (w / 3.0, h),
    },
];
