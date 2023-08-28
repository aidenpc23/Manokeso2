use crate::{util::point::Point, world::decode_alpha};

use super::{
    element::{Align, Text},
    ui::GameUI,
};

pub fn board() -> GameUI {
    let text = vec![
        Text {
            content: |state| {
                if let Some(tile) = &state.hovered_tile {
                    let mut str = format!(
                        concat!(
                            "connex number: {}\n",
                            "stability: {}\n",
                            "reactivity: {}\n",
                            "energy: {}\n",
                            "radiation: {}\n"
                        ),
                        tile.connex_number, tile.stability, tile.reactivity, tile.energy, tile.gamma
                    );
                    if state.debug {
                        str = format!("tile pos: {:?}\n", tile.pos) + &str;
                    }
                    if state.player.creative {
                        str.push_str(&format!(
                            concat!(
                                "alpha: {:?}\n",
                                "beta: {:?}\n",
                                "delta: {:b}\n",
                                "omega: {:?}\n",
                            ),
                            decode_alpha(tile.alpha),
                            tile.beta,
                            tile.delta,
                            tile.omega,
                        ));
                    }
                    str
                } else {
                    "no tile selected".to_string()
                }
            },
            pos: |(_, _)| Point { x: 20.0, y: 15.0 },
            align: Align::Left,
            bounds: |(w, h)| (w / 3.0 - 30.0, h),
        },
        Text {
            content: |state| format!("total energy: {}", state.world.view.total_energy),
            pos: |(w, _)| Point {
                x: w / 2.0,
                y: 15.0,
            },
            align: Align::Center,
            bounds: |(w, h)| (w / 3.0 - 20.0, h),
        },
        Text {
            content: |state| {
                if state.debug {
                    let adp_info = state.renderer.render_surface.adapter.get_info();
                    format!(
                        "adapter: {}\nbackend: {:?}\nclient update: {:.3}ms\nworld update: {:.3}ms",
                        adp_info.name,
                        adp_info.backend,
                        state.debug_stats.client_update_time,
                        state.debug_stats.world_update_time,
                    )
                } else {
                    String::new()
                }
            },
            pos: |(w, _)| Point {
                x: w - 20.0,
                y: 15.0,
            },
            align: Align::Right,
            bounds: |(w, h)| (w / 3.0 - 30.0, h),
        },
    ];
    let shapes = vec![];
    GameUI { text, shapes }
}
