use crate::{
    render::ui::text::{Align, Text},
    util::point::Point,
};

pub const BOARD: [Text; 3] = [
    Text {
        update: |client, _| {
            if let Some(tile) = &client.hovered_tile {
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
                    tile.alpha,
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
        update: |client, _| format!("total energy: {}", client.world.view_info.total_energy),
        pos: |(w, _)| Point {
            x: w / 2.0,
            y: 10.0,
        },
        align: Align::Center,
        bounds: |(w, h)| (w / 3.0, h),
    },
    Text {
        update: |client, surface| {
            let adp_info = surface.adapter.get_info();
            format!(
                "adapter: {}\nbackend: {:?}\nframe time: {:?}\nupdate time: {:?}",
                adp_info.name,
                adp_info.backend,
                client.frame_timer.avg(),
                client.world.view_info.time_taken
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
