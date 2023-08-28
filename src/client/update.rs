use std::time::Instant;

use crate::{common::view::TileInfo, util::point::Point};

use super::{input::Input, Client};

pub fn update(client: &mut Client, input: &Input, now: Instant) {
    let view = &mut client.worker.view;

    let ddelta = now - client.debug.last_update;
    if ddelta > client.debug.period {
        client.debug.last_update = now;
        client.debug.client_update_time = client.timer.avg().as_secs_f32() * 1000.0;
        client.debug.board_update_time = view.time_taken.as_secs_f32() * 1000.0;
    }

    let mouse_world_pos = client.renderer.pixel_to_world(input.mouse_pixel_pos);
    let Point { x, y } = mouse_world_pos - view.slice.world_pos;
    client.hovered_tile =
        if x >= 0.0 && y >= 0.0 && x < view.slice.width as f32 && y < view.slice.height as f32 {
            let pos = Point::new(x as usize, y as usize);
            let i = pos.index(view.slice.width);
            let pos = pos + view.slice.start;
            Some(TileInfo {
                pos,
                connex_number: view.connex_numbers[i],
                stability: view.stability[i],
                reactivity: view.reactivity[i],
                energy: view.energy[i],
                alpha: view.alpha[i],
                beta: view.beta[i],
                gamma: view.gamma[i],
                delta: view.delta[i],
                omega: view.omega[i],
            })
        } else {
            None
        };

    if !client.state.player.creative {
        handle_collisions(client);
    }
}

pub fn handle_collisions(client: &mut Client) {
    let view = &mut client.worker.view;
    let player = &mut client.state.player;

    if view.connex_numbers.len() != 0 {
        // cardinal edges

        let rad = player.size / 2.0;
        let player_rel_pos = player.pos - view.board_pos;
        let player_edges = Point::<f32>::CARDINAL_DIRECTIONS.map(|v| player_rel_pos + v * rad);
        let slice = view.slice;
        for i in 0..4 {
            let mut edge = player_edges[i];
            let board_edge = edge.x < 0.0
                || edge.y < 0.0
                || edge.x >= slice.end.x as f32
                || edge.y >= slice.end.y as f32;
            let tile_pos: Point<i32> = edge.floor().into();
            let solid_tile = if board_edge {
                true
            } else {
                let board_pos: Point<usize> = tile_pos.into();
                let tile_i = (board_pos - slice.start).index(slice.width);
                let cn = view.connex_numbers[tile_i];
                let s = view.stability[tile_i];
                cn > 10 && s > 0.8
            };
            if solid_tile {
                let dir = Point::<f32>::CARDINAL_DIRECTIONS[i];
                if dir.x < 0.0 || dir.y < 0.0 {
                    edge = edge - 1.0;
                }
                let a: Point<f32> = tile_pos.into();
                player.pos += (edge - a) * -dir.abs();
            }
        }

        // corners

        let player_rel_pos = player.pos - view.board_pos;
        let player_tile: Point<i32> = player_rel_pos.floor().into();
        let start: Point<i32> = (player_rel_pos - player.size / 2.0).floor().into();
        let end: Point<i32> = (player_rel_pos + player.size / 2.0).floor().into();
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                if x < 0 || y < 0 || x >= slice.end.x as i32 || y >= slice.end.y as i32 {
                    continue;
                }
                if x != player_tile.x && y != player_tile.y {
                    let tile = Point {
                        x: x as usize,
                        y: y as usize,
                    };
                    let rel_pos = tile - view.slice.start;
                    let i = rel_pos.index(view.slice.width);
                    let cn = view.connex_numbers[i];
                    let s = view.stability[i];
                    if cn > 10 && s > 0.8 {
                        let mut corner: Point<f32> = tile.into();
                        if x < player_tile.x {
                            corner += Point::X_UNIT;
                        }
                        if y < player_tile.y {
                            corner += Point::Y_UNIT;
                        }
                        let dist = player_rel_pos.dist(corner);
                        if dist < rad {
                            let move_dist = rad - dist;
                            player.pos += (player_rel_pos - corner).norm() * move_dist;
                        }
                    }
                }
            }
        }
    }
}
