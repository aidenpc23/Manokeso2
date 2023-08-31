use crate::{board::decode_alpha, util::point::Point};

use super::{
    element::{Align, Text},
    ui::GameUI,
};

pub fn board() -> GameUI {
    let text = vec![
        Text {
            content: |client| {
                if let Some(tile) = &client.hovered_tile() {
                    let bufs = &tile.view.bufs;
                    let i = tile.i;
                    let mut str = format!(
                        concat!(
                            "connex number: {}\n",
                            "stability: {}\n",
                            "reactivity: {}\n",
                            "energy: {}\n",
                            "radiation: {}\n"
                        ),
                        bufs.connex_number[i],
                        bufs.stability[i],
                        bufs.reactivity[i],
                        bufs.energy[i],
                        bufs.gamma[i]
                    );
                    if client.debug.show {
                        str = format!("tile pos: {:?}\n", tile.pos) + &str;
                    }
                    if client.state.player.creative {
                        str.push_str(&format!(
                            concat!(
                                "alpha: {:?}\n",
                                "beta: {:?}\n",
                                "delta: {:b}\n",
                                "omega: {:?}\n",
                            ),
                            decode_alpha(bufs.alpha[i]),
                            bufs.beta[i],
                            bufs.delta[i],
                            bufs.omega[i],
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
            content: |client| {
                format!(
                    "total energy: {}",
                    client
                        .worker
                        .get(client.state.main_id)
                        .map(|v| v.total_energy)
                        .unwrap_or(0.0)
                )
            },
            pos: |(w, _)| Point {
                x: w / 2.0,
                y: 15.0,
            },
            align: Align::Center,
            bounds: |(w, h)| (w / 3.0 - 20.0, h),
        },
        Text {
            content: |client| {
                if client.debug.show {
                    let adp_info = client.renderer.render_surface.adapter.get_info();
                    let Point { x, y } = client.state.player.pos;
                    format!(
                        concat!(
                            "pos: {:.3}, {:.3}\n",
                            "adapter: {}\n",
                            "backend: {:?}\n",
                            "client update: {:.3}ms\n",
                            "world update: {:.3}ms",
                        ),
                        x,
                        y,
                        adp_info.name,
                        adp_info.backend,
                        client.debug.client_update_time,
                        client.debug.board_update_time,
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
