use crate::{client::Client, render::primitive::UIPrimatives, util::point::Point};

use super::element::{RoundedRect, Text, UIPoint};

pub struct GameUI {
    pub text: Vec<Text>,
    pub shapes: Vec<RoundedRect>,
}

impl GameUI {
    pub fn compile(&self, client: &Client) -> UIPrimatives {
        let mut primatives = UIPrimatives {
            text: self.text.iter().map(|t| t.into_primitive(client)).collect(),
            rounded_rects: self.shapes.iter().map(|t| t.to_primitive()).collect(),
        };
        if let Some(tile) = client.hovered_tile {
            let pos: Point<f32> = tile.pos.into();
            let start = pos + client.worker.view.board_pos;
            let end = start + 1.0;
            let mut start = client.renderer.world_to_pixel(start);
            let mut end = client.renderer.world_to_pixel(end);
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
                    thickness: 3.0 * client.state.camera.scale,
                    ..Default::default()
                }
                .to_primitive(),
            );
        }
        if let Some(tile) = client.state.selected_tile {
            let pos: Point<f32> = tile.pos.into();
            let start = pos + client.worker.view.board_pos;
            let end = start + 1.0;
            let mut start = client.renderer.world_to_pixel(start);
            let mut end = client.renderer.world_to_pixel(end);
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
                        [0.0, 0.0, 0.0, 1.0],
                        [0.5, 0.5, 0.5, 1.0],
                        [0.5, 0.5, 0.5, 1.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ],
                    thickness: 3.0 * client.state.camera.scale,
                    ..Default::default()
                }
                .to_primitive(),
            );
        }

        primatives.rounded_rects.append(&mut client.state.player.to_primitives(&client.renderer));

        primatives
    }
}
