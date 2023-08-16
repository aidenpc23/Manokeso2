use std::time::Duration;

use crate::{
    render::ui::text::{Align, Text},
    util::point::Point,
};

pub const BOARD: [Text; 3] = [
    Text {
        update: |client, _| {
            if let Some(pos) = client.hovered_tile {
                if let Ok(view) = client.board_view.try_read() {
                    let i = pos.index(view.slice.width);
                    Some(format!(
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
                        pos + view.slice.start.into(),
                        view.connex_numbers[i],
                        view.stability[i],
                        view.reactivity[i],
                        view.energy[i],
                        view.alpha[i],
                        view.beta[i],
                        view.gamma[i],
                        view.delta[i],
                        view.omega[i],
                    ))
                } else {
                    None
                }
            } else {
                Some("no tile selected".to_string())
            }
        },
        pos: |(_, _)| Point { x: 10.0, y: 10.0 },
        align: Align::Left,
        bounds: |(w, h)| (w / 3.0, h),
    },
    Text {
        update: |client, _| {
            client
                .board_view
                .try_read()
                .map(|v| Some(format!("total energy: {}", v.total_energy)))
                .unwrap_or(None)
        },
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
            Some(format!(
                "adapter: {}\nbackend: {:?}\nframe time: {:?}\nupdate time: {:?}",
                adp_info.name,
                adp_info.backend,
                client.frame_timer.avg(),
                client
                    .board_view
                    .try_read()
                    .map(|v| v.time_taken)
                    .unwrap_or(Duration::ZERO)
            ))
        },
        pos: |(w, _)| Point {
            x: w - 10.0,
            y: 10.0,
        },
        align: Align::Right,
        bounds: |(w, h)| (w / 3.0, h),
    },
];
