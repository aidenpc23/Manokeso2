use crate::{util::point::Point, world::decode_alpha};

use super::{
    element::{Align, Text},
    ui::GameUI, element::{RoundedRect, UIPoint},
};

pub fn board() -> GameUI {
    let text = vec![
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
                            "delta: {:b}\n",
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
            pos: |(_, _)| Point { x: 20.0, y: 15.0 },
            align: Align::Left,
            bounds: |(w, h)| (w / 3.0 - 30.0, h),
        },
        Text {
            content: |state| format!("total energy: {}", state.world.view_info.total_energy),
            pos: |(w, _)| Point {
                x: w / 2.0,
                y: 15.0,
            },
            align: Align::Center,
            bounds: |(w, h)| (w / 3.0 - 20.0, h),
        },
        Text {
            content: |state| {
                let adp_info = state.renderer.render_surface.adapter.get_info();
                format!(
                    "adapter: {}\nbackend: {:?}\nclient update: {:.3}ms\nworld update: {:.3}ms",
                    adp_info.name,
                    adp_info.backend,
                    state.debug_stats.client_update_time,
                    state.debug_stats.world_update_time,
                )
            },
            pos: |(w, _)| Point {
                x: w - 20.0,
                y: 15.0,
            },
            align: Align::Right,
            bounds: |(w, h)| (w / 3.0 - 30.0, h),
        },
    ];
    let shapes = vec![RoundedRect {
        top_left: UIPoint::anchor_offset(0.0, 0.0, 10.0, 10.0),
        bottom_right: UIPoint::anchor_offset(1.0/3.0, 0.0, 0.0, 275.0),
        colors: [
            [0.0, 0.0, 0.0, 0.5],
            [0.0, 0.0, 0.0, 0.5],
            [0.0, 0.0, 0.0, 0.5],
            [0.0, 0.0, 0.0, 0.5],
        ],
        radius: 20.0,
        ..Default::default()
    }];
    GameUI { text, shapes }
}
