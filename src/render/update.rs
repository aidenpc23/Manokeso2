use std::time;

use ndarray::{azip, s, Zip};
use wgpu::util::DeviceExt;

use crate::{state::GameState, world::Board};

const CHUNK_ALIGN: u32 = 5;
const CHUNK_SIZE: i32 = 2i32.pow(CHUNK_ALIGN);
const CHUNK_MASK: i32 = !(CHUNK_SIZE - 1);

use super::{
    uniform::{CameraUniform, TileViewUniform},
    Instance, Renderer,
};

impl Renderer {
    pub fn update(&mut self, state: &GameState, resize: bool) {
        let camera = &state.camera;
        let size = &self.window.inner_size();
        let uniform = CameraUniform::new(camera, size);

        // get positions in the world
        // s = start, e = end
        let [cxs, cys] = self.uniforms.camera.bottom_left();
        let [cxe, cye] = self.uniforms.camera.top_right();
        let [bx, by] = state.board.pos;
        let bw = state.board.width();
        let bh = state.board.height();
        // calculate camera position relative to board position
        // also account for tile mesh (-.5 -> .5)
        let xs = (cxs - bx + 0.5) as i32;
        let ys = (cys - by + 0.5) as i32;
        let xe = (cxe - bx + 1.5) as i32;
        let ye = (cye - by + 1.5) as i32;
        // align with chunks and add an extra chunk in each direction
        let xs = (xs & CHUNK_MASK) - 1 * CHUNK_SIZE;
        let ys = (ys & CHUNK_MASK) - 1 * CHUNK_SIZE;
        let xe = (xe & CHUNK_MASK) + 2 * CHUNK_SIZE;
        let ye = (ye & CHUNK_MASK) + 2 * CHUNK_SIZE;
        // cut off values for bounds
        let xs = (xs.max(0) as usize).min(bw);
        let ys = (ys.max(0) as usize).min(bh);
        let xe = (xe.max(0) as usize).min(bw);
        let ye = (ye.max(0) as usize).min(bh);

        let start = time::Instant::now();
        let len = state
            .board
            .update_instances(&mut self.instances, xs, xe, ys, ye);
        let taken = time::Instant::now() - start;
        println!("{:?}", taken);
        if self.instance_len != len {
            self.instance_len = len;
            self.buffers.instance =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Instance Buffer"),
                        contents: bytemuck::cast_slice(&self.instances[0..len]),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });
        } else {
            self.queue.write_buffer(
                &self.buffers.instance,
                0,
                bytemuck::cast_slice(&self.instances[0..len]),
            );
        }

        let view = TileViewUniform::new([bx + xs as f32, by + ys as f32], (xe - xs) as u32);
        if self.uniforms.tile_view != view {
            self.uniforms.tile_view = view;
            self.queue.write_buffer(
                &self.buffers.tile_view,
                0,
                bytemuck::cast_slice(&[self.uniforms.tile_view]),
            )
        }

        if self.uniforms.camera != uniform {
            self.uniforms.camera = uniform;
            self.queue.write_buffer(
                &self.buffers.camera,
                0,
                bytemuck::cast_slice(&[self.uniforms.camera]),
            );
            if resize {
                self.config.width = size.width;
                self.config.height = size.height;
                self.surface.configure(&self.device, &self.config);
            }
        }
    }
}

impl Board {
    fn update_instances(
        &self,
        instances: &mut Vec<Instance>,
        xs: usize,
        xe: usize,
        ys: usize,
        ye: usize,
    ) -> usize {
        let width = xe - xs;
        let size = width * (ye - ys);
        if instances.len() >= size {
            // let mut i = 0;
            // for rows in self.test[ys..ye].iter() {
            //     for row in rows[xs..xe].iter() {
            //         let tile = &mut instances[i];
            //         tile.connex_number = row[0] as u32;
            //         tile.stability = row[1];
            //         tile.reactivity = row[2];
            //         tile.energy = row[3];
            //         i += 1;
            //     }
            // }
            let mut i = 0;
            for row in self.test[ys..ye].iter() {
                instances[i..i+width].copy_from_slice(&row[xs..xe]);
                i += width;
            }
            // azip!((index (y, x),
            //     &c in &self.connex_numbers.slice(s![xs..xe, ys..ye]),
            //     &s in &self.stability.slice(s![xs..xe, ys..ye]),
            //     &r in &self.reactivity.slice(s![xs..xe, ys..ye]),
            //     &e in &self.energy.slice(s![xs..xe, ys..ye])
            // ) {
            //     let i = y * width + x;
            //     let attrs = &mut instances[i].attributes;
            //     attrs[0] = c as f32;
            //     attrs[1] = s;
            //     attrs[2] = r;
            //     attrs[3] = e;
            // });
            // for y in ys..ye {
            //     for x in xs..xe {
            //         let i = (y - ys) * width + (x - xs);
            //         let attrs = &mut instances[i].attributes;
            //         let test = self.test[x][y];
            //         attrs[0] = *self.connex_numbers.get((x, y)).unwrap() as f32;
            //         attrs[1] = *self.stability.get((x, y)).unwrap();
            //         attrs[2] = *self.reactivity.get((x, y)).unwrap();
            //         attrs[3] = *self.energy.get((x, y)).unwrap();
            //     }
            // }
        } else {
            instances.clear();
            for row in self.test[ys..ye].iter() {
                instances.extend_from_slice(&row[xs..xe]);
            }
            // for rows in self.test[ys..ye].iter() {
            //     for row in rows[xs..xe].iter() {
            //         instances.push(Instance {
            //             connex_number: row[0] as u32,
            //             stability: row[1],
            //             reactivity: row[2],
            //             energy: row[3],
            //         })
            //     }
            // }
        }
        size
    }
}
