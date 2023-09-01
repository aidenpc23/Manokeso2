use noise::OpenSimplex;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

use crate::{
    board_attrs,
    rsc::{ENERGY_RANGE, REACTIVITY_RANGE},
    util::point::Point,
};

use rand::{seq::SliceRandom, Rng, SeedableRng};
use std::collections::{HashMap, HashSet, VecDeque};

use super::{encode_alpha, gen::SwapBufferGen, get_bit, set_bit, swap_buffer::SwapBuffer};

board_attrs!(BoardBufs, BoardViewBufs, Tile, [
    connex_number: u32,
    stability: f32,
    reactivity: f32,
    energy: f32,
    alpha: u64,
    beta: u64,
    gamma: f32,
    omega: f32,
    delta: u64
]);

#[derive(Debug, Clone, Copy)]
pub struct BoardSettings {
    pub pos: Point<f32>,
    pub width: usize,
    pub height: usize,
    pub seed: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub pos: Point<f32>,
    pub width: usize,
    pub height: usize,
    pub bufs: BoardBufs,
    pub total_energy: f32,
}

impl Board {
    pub fn new(settings: BoardSettings) -> Board {
        let BoardSettings { pos, width, height, seed } = settings;
        let mut gen = (width, height);

        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        let mut open_simplex = OpenSimplex::new(seed as u32);

        let stability = gen.gen_map_base(&mut open_simplex, [0.6, 0.2], [0.6, 0.0], 0.058, 0.015, 0.06);
        let connex_numbers = SwapBuffer::from_arr(
            stability.r.iter().map(|a| (a * 20.0) as u32).collect(),
            width,
        );
        let reactivity = gen.gen_map(&mut open_simplex, REACTIVITY_RANGE, 0.05);
        let energy = gen.gen_map(&mut open_simplex, ENERGY_RANGE, 0.01);
        let alpha = SwapBuffer::from_arr(
            vec![encode_alpha(0, 0, 0.0, 0.0, 0.0); width * height],
            width,
        );
        let beta = SwapBuffer::from_arr(vec![0; width * height], width);
        let gamma = SwapBuffer::from_arr(vec![0.0; width * height], width);
        let omega = SwapBuffer::from_arr(vec![0.0; width * height], width);
        // let delta = SwapBuffer::from_rand(&mut rand::thread_rng(), width, height, [0, 10000000000]);
        let delta = SwapBuffer::gen_delta(&mut rng, width, height);

        let total_energy = energy.r.iter().sum();

        let mut board = Board {
            pos,
            width,
            height,
            bufs: BoardBufs {
                connex_number: connex_numbers,
                stability,
                reactivity,
                energy,
                alpha,
                beta,
                gamma,
                omega,
                delta,
            },
            total_energy,
        };
        board.generate_maze(&mut rng);
        board
    }

    pub fn generate_maze(&mut self, rng: &mut ChaCha8Rng) {
        let maze_width = self.width / 7;
        let maze_height = self.height;

        let x_offset = self.width / 2;
        let y_offset = rng.gen_range(0..=(self.height - maze_height));

        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut stack: Vec<(usize, usize)> = Vec::new();

        for x in x_offset..x_offset + maze_width {
            for y in y_offset..y_offset + maze_height {
                let index = x + y * self.width;
                self.bufs.stability.r[index] = 1.0;
                self.bufs.reactivity.r[index] = 0.0;
                self.bufs.delta.r[index] = 0;
                self.bufs.delta.r[index] |= 1 << 0;
                self.bufs.delta.r[index] |= 1 << 1;
                // self.bufs.delta.r[index] |= 1 << 3;
                // self.bufs.delta.r[index] |= 1 << 4;
                self.bufs.delta.r[index] |= 1 << 10;
                // self.bufs.energy.r[index] = rng.gen_range(0.0..1000.0);
                self.bufs.connex_number.r[index] = 200;
            }
        }

        // Using the algorithm DFS because it garuntees everything is connected
        let start_x = x_offset + rng.gen_range(0..maze_width / 2) * 2;
        let start_y = y_offset + rng.gen_range(0..maze_height / 2) * 2;
        visited.insert((start_x, start_y));
        stack.push((start_x, start_y));

        while let Some((cx, cy)) = stack.pop() {
            let mut neighbors = vec![];

            // Get all valid neighbors
            for &(dx, dy) in &[(0, 2), (2, 0), (0, -2), (-2, 0)] {
                let (nx, ny) = ((cx as isize + dx) as usize, (cy as isize + dy) as usize);
                if nx > x_offset
                    && nx < x_offset + maze_width
                    && ny > y_offset
                    && ny < y_offset + maze_height
                    && !visited.contains(&(nx, ny))
                {
                    neighbors.push((nx, ny));
                }
            }

            if !neighbors.is_empty() {
                // Randomly select one of the neighbors
                let &(nx, ny) = neighbors.choose(rng).unwrap();

                visited.insert((nx, ny));
                stack.push((cx, cy));
                stack.push((nx, ny));

                // Remove the wall
                let (wx, wy) = ((cx + nx) / 2 - 1, (cy + ny) / 2 - 1);
                let (ux, uy) = (nx - 1, ny - 1);
                self.bufs.stability.r[wx + wy * self.width] = 0.0;
                self.bufs.stability.r[ux + uy * self.width] = 0.0;
                set_bit(&mut self.bufs.delta.r[wx + wy * self.width], false, 0);
                set_bit(&mut self.bufs.delta.r[ux + uy * self.width], false, 0);
            }
        }

        let room_width = 10;
        let room_height = 5;

        let center_x = x_offset + maze_width / 2;
        let center_y = y_offset + maze_height / 2;

        for x in (center_x - room_width / 2 - 1)..=(center_x + room_width / 2 + 1) {
            for y in (center_y - room_height / 2 - 1)..=(center_y + room_height / 2 + 1) {
                let index = x + y * self.width;
                if x == center_x - room_width / 2 - 1
                    || x == center_x + room_width / 2 + 1
                    || y == center_y - room_height / 2 - 1
                    || y == center_y + room_height / 2 + 1
                {
                    self.bufs.stability.r[index] = 1.0;
                    set_bit(&mut self.bufs.delta.r[index], true, 0);
                } else {
                    self.bufs.stability.r[index] = 0.0;
                    set_bit(&mut self.bufs.delta.r[index], false, 0);
                }
            }
        }

        let mut potential_doors_top: Vec<usize> = Vec::new();
        let mut potential_doors_bottom: Vec<usize> = Vec::new();
        let mut potential_doors_left: Vec<usize> = Vec::new();
        let mut potential_doors_right: Vec<usize> = Vec::new();

        // Iterate through the outer border of the maze
        for x in x_offset..x_offset + maze_width {
            for y in y_offset..y_offset + maze_height {
                let index = x + y * self.width;

                // Exclude corners for top and bottom walls
                if y == y_offset
                    && self.bufs.stability.r[x + (y + 1) * self.width] == 0.0
                    && x != x_offset
                    && x != x_offset + maze_width - 1
                {
                    potential_doors_top.push(index);
                }

                if y == y_offset + maze_height - 1
                    && self.bufs.stability.r[x + (y - 1) * self.width] == 0.0
                    && x != x_offset
                    && x != x_offset + maze_width - 1
                {
                    potential_doors_bottom.push(index);
                }

                // Exclude corners for left and right walls
                if x == x_offset
                    && self.bufs.stability.r[(x + 1) + y * self.width] == 0.0
                    && y != y_offset
                    && y != y_offset + maze_height - 1
                {
                    potential_doors_left.push(index);
                }

                if x == x_offset + maze_width - 1
                    && self.bufs.stability.r[(x - 1) + y * self.width] == 0.0
                    && y != y_offset
                    && y != y_offset + maze_height - 1
                {
                    potential_doors_right.push(index);
                }
            }
        }

        let mut outside_doors: HashSet<usize> = HashSet::new();
        select_door(self, &potential_doors_top, &mut outside_doors);
        select_door(self, &potential_doors_bottom, &mut outside_doors);
        select_door(self, &potential_doors_left, &mut outside_doors);
        select_door(self, &potential_doors_right, &mut outside_doors);

        let mut potential_doors: Vec<usize> = Vec::new();

        for x in (center_x - room_width / 2 - 1)..=(center_x + room_width / 2 + 1) {
            for y in (center_y - room_height / 2 - 1)..=(center_y + room_height / 2 + 1) {
                let index = x + y * self.width;
                // Check if it is in bordre
                if x == center_x - room_width / 2 - 1
                    || x == center_x + room_width / 2 + 1
                    || y == center_y - room_height / 2 - 1
                    || y == center_y + room_height / 2 + 1
                {
                    // Top
                    if y == center_y - room_height / 2 - 1
                        && self.bufs.stability.r[x + (y - 1) * self.width] == 0.0
                        && x != center_x - room_width / 2 - 1
                        && x != center_x + room_width / 2 + 1
                    {
                        potential_doors.push(index);
                    }

                    // Bottom
                    if y == center_y + room_height / 2 + 1
                        && self.bufs.stability.r[x + (y + 1) * self.width] == 0.0
                        && x != center_x - room_width / 2 - 1
                        && x != center_x + room_width / 2 + 1
                    {
                        potential_doors.push(index);
                    }

                    // Left
                    if x == center_x - room_width / 2 - 1
                        && self.bufs.stability.r[(x - 1) + y * self.width] == 0.0
                        && y != center_y - room_height / 2 - 1
                        && y != center_y + room_height / 2 + 1
                    {
                        potential_doors.push(index);
                    }

                    // Right
                    if x == center_x + room_width / 2 + 1
                        && self.bufs.stability.r[(x + 1) + y * self.width] == 0.0
                        && y != center_y - room_height / 2 - 1
                        && y != center_y + room_height / 2 + 1
                    {
                        potential_doors.push(index);
                    }
                }
            }
        }

        let valid_inside_doors: Vec<usize> = potential_doors
            .iter()
            .cloned()
            .filter(|&door| has_valid_path(self, door, &outside_doors).is_ok())
            .collect();

        if valid_inside_doors.len() >= 2 {
            let doors: Vec<&usize> = valid_inside_doors.choose_multiple(rng, 2).collect();
            for &&door in doors.iter() {
                self.bufs.stability.r[door] = 0.0;
                set_bit(&mut self.bufs.delta.r[door], false, 0);

                // Get the path and set Connex numbers to 0
                if let Ok(path) = has_valid_path(self, door, &outside_doors) {
                    for idx in path {
                        // self.bufs.energy.r[idx] = rng.gen_range(0.0..50.0);
                    }
                }
            }
        }
    }

    pub fn player_can_move(&self, pos: Point<usize>) -> bool {
        let pos = pos.index(self.width);

        if (self.bufs.connex_number.r[pos] > 20 && self.bufs.stability.r[pos] > 0.8)
            || get_bit(self.bufs.delta.r[pos], 10)
        {
            false
        } else {
            true
        }
    }
}

fn select_door(
    board: &mut Board,
    potential_doors: &Vec<usize>,
    outside_doors: &mut HashSet<usize>,
) {
    if !potential_doors.is_empty() {
        let door = potential_doors.choose(&mut rand::thread_rng()).unwrap();
        outside_doors.insert(*door);
        board.bufs.stability.r[*door] = 0.0;
        set_bit(&mut board.bufs.delta.r[*door], false, 0);
    }
}

fn has_valid_path(
    board: &Board,
    start: usize,
    outside_doors: &HashSet<usize>,
) -> Result<Vec<usize>, ()> {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut predecessor: HashMap<usize, usize> = HashMap::new();

    visited.insert(start);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if outside_doors.contains(&current) {
            let mut path = vec![current];
            let mut pred = current;
            while let Some(&p) = predecessor.get(&pred) {
                path.push(p);
                pred = p;
            }
            path.reverse();
            return Ok(path);
        }

        let x = current % board.width;
        let y = current / board.width;

        let neighbors = vec![
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        for (nx, ny) in neighbors {
            if nx >= 0 && nx < board.width as isize && ny >= 0 && ny < board.height as isize {
                let neighbor_idx = nx as usize + ny as usize * board.width;
                if board.bufs.stability.r[neighbor_idx] == 0.0 && !visited.contains(&neighbor_idx) {
                    visited.insert(neighbor_idx);
                    queue.push_back(neighbor_idx);
                    predecessor.insert(neighbor_idx, current);
                }
            }
        }
    }

    Err(())
}
