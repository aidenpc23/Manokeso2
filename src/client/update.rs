use std::time::Instant;

use crate::{util::point::Point, sync::TileInfo};

use super::{ClientState, input::Input};

pub fn update(state: &mut ClientState, input: &Input, now: Instant) {
    let view = &mut state.world.view;

    let ddelta = now - state.last_debug;
    if ddelta > state.debug_stats.period {
        state.last_debug = now;
        state.debug_stats.client_update_time = state.timer.avg().as_secs_f32() * 1000.0;
        state.debug_stats.world_update_time = view.time_taken.as_secs_f32() * 1000.0;
    }

    let mouse_world_pos = state.renderer.pixel_to_world(input.mouse_pixel_pos);
    let rinfo = view.render_info;
    let Point { x, y } = mouse_world_pos - rinfo.pos;
    state.hovered_tile =
        if x >= 0.0 && y >= 0.0 && x < rinfo.slice.width as f32 && y < rinfo.slice.height as f32 {
            let pos = Point::new(x as usize, y as usize);
            let i = pos.index(rinfo.slice.width);
            let pos = pos + rinfo.slice.start;
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

    // collisions (move to another function)

    if view.connex_numbers.len() != 0 {
        let rad = state.player.size / 2.0;
        let player_rel_pos = state.player.pos - view.pos;
        let player_edges = Point::<f32>::CARDINAL_DIRECTIONS.map(|v| player_rel_pos + v * rad);
        let slice = view.render_info.slice;

        // cardinal edges

        for i in 0..4 {
            let mut edge = player_edges[i];
            if edge.x < 0.0
                || edge.y < 0.0
                || edge.x >= slice.end.x as f32
                || edge.y >= slice.end.y as f32
            {
                continue;
            }
            let tile: Point<usize> = edge.into();
            let tile_i = (tile - slice.start).index(slice.width);
            let cn = view.connex_numbers[tile_i];
            let s = view.stability[tile_i];
            if cn > 10 && s > 0.8 {
                let dir = Point::<f32>::CARDINAL_DIRECTIONS[i];
                if dir.x < 0.0 || dir.y < 0.0 {
                    edge = edge - 1.0;
                }
                let a: Point<f32> = tile.into();
                state.player.pos += (edge - a) * -dir.abs();
            }
        }

        // corners

        let player_rel_pos = state.player.pos - view.pos;
        let player_tile: Point<i32> = player_rel_pos.floor().into();
        let start: Point<i32> = (player_rel_pos - state.player.size / 2.0).floor().into();
        let end: Point<i32> = (player_rel_pos + state.player.size / 2.0).floor().into();
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
                    let rel_pos = tile - view.render_info.slice.start;
                    let i = rel_pos.index(view.render_info.slice.width);
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
                            state.player.pos += (player_rel_pos - corner).norm() * move_dist;
                        }
                    }
                }
            }
        }
    }
}
