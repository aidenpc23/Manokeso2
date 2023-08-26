use crate::{client::ClientState, render::primitive::UIPrimatives, util::point::Point};

use super::element::{RoundedRect, Text, UIPoint};

pub struct GameUI {
    pub text: Vec<Text>,
    pub shapes: Vec<RoundedRect>,
}

impl GameUI {
    pub fn compile(&self, state: &ClientState) -> UIPrimatives {
        let mut primatives = UIPrimatives {
            text: self.text.iter().map(|t| t.into_primitive(state)).collect(),
            rounded_rects: self.shapes.iter().map(|t| t.to_primitive()).collect(),
        };
        if let Some(tile) = state.hovered_tile {
            let pos: Point<f32> = tile.pos.into();
            let start = pos + state.world.view_info.pos;
            let end = start + 1.0;
            let mut start = state.renderer.world_to_pixel(start);
            let mut end = state.renderer.world_to_pixel(end);
            let y = end.y;
            end.y = start.y;
            start.y = y;
            primatives.rounded_rects.push(
                RoundedRect {
                    top_left: UIPoint {
                        anchor: Point::zero(),
                        offset: start,
                    },
                    bottom_right: UIPoint {
                        anchor: Point::zero(),
                        offset: end,
                    },
                    colors: [
                        [0.0, 0.0, 0.0, 0.6],
                        [0.3, 0.3, 0.3, 0.4],
                        [0.3, 0.3, 0.3, 0.4],
                        [0.0, 0.0, 0.0, 0.6],
                    ],
                    thickness: 3.0 * state.camera.scale,
                    ..Default::default()
                }
                .to_primitive(),
            );
        }
        if let Some(tile) = state.held_tile {
            let pos: Point<f32> = tile.pos.into();
            let start = pos + state.world.view_info.pos;
            let end = start + 1.0;
            let mut start = state.renderer.world_to_pixel(start);
            let mut end = state.renderer.world_to_pixel(end);
            let y = end.y;
            end.y = start.y;
            start.y = y;
            primatives.rounded_rects.push(
                RoundedRect {
                    top_left: UIPoint {
                        anchor: Point::zero(),
                        offset: start,
                    },
                    bottom_right: UIPoint {
                        anchor: Point::zero(),
                        offset: end,
                    },
                    thickness: 3.0 * state.camera.scale,
                    ..Default::default()
                }
                .to_primitive(),
            );
        }

        primatives.rounded_rects.append(&mut state.player.to_primitives(&state.renderer));

        primatives
    }
}
