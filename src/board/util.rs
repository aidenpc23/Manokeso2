use super::update::CARDINAL_DIRECTIONS;

const CNUMBER_OFFSET: u64 = 200;
const STABILITY_OFFSET: f32 = 1000.0;
const ENERGY_OFFSET: f32 = 10000.0;
const REACTIVITY_OFFSET: f32 = 500.0;

use lazy_static::lazy_static;

lazy_static!{
    pub static ref ZERO_ALPHA: u64 = encode_alpha(0, 0, 0.0, 0.0, 0.0);
}

fn clamp(val: i32, min: i32, max: i32) -> i32 {
    val.max(min).min(max)
}

fn clampf(val: f32, min: f32, max: f32) -> f32 {
    val.max(min).min(max)
}

pub fn encode_alpha(counter: u64, cnumber: i32, stability: f32, energy: f32, reactivity: f32) -> u64 {
    let enc_cnumber = (clamp(cnumber, -200, 200) + CNUMBER_OFFSET as i32) as u64 & 0x1FF;
    let enc_stability = (clampf(stability, -1.0, 1.0) * STABILITY_OFFSET + STABILITY_OFFSET) as u64 & 0x7FF;
    let enc_energy = (energy * 10.0 + ENERGY_OFFSET) as u64 & 0x7FFF;
    let enc_reactivity = (clampf(reactivity, -1.0, 1.0) * REACTIVITY_OFFSET + REACTIVITY_OFFSET) as u64 & 0x3FF;

    let enc_counter = counter & 0x1FF;

    (enc_counter << 55) |
    (enc_cnumber << 46) |
    (enc_stability << 35) |
    (enc_energy << 20) |
    (enc_reactivity << 10)
}

pub fn decode_alpha(encoded: u64) -> (u64, i32, f32, f32, f32) {
    let dec_counter = (encoded >> 55) & 0x1FF;

    let dec_cnumber = ((encoded >> 46) & 0x1FF) as i32 - CNUMBER_OFFSET as i32;
    let dec_stability = ((encoded >> 35) & 0x7FF) as f32 / STABILITY_OFFSET - 1.0;
    let dec_energy = ((encoded >> 20) & 0x7FFF) as f32 / 10.0 - ENERGY_OFFSET / 10.0;
    let dec_reactivity = ((encoded >> 10) & 0x3FF) as f32 / REACTIVITY_OFFSET - 1.0;

    (dec_counter, dec_cnumber, dec_stability, dec_energy, dec_reactivity)
}


pub fn encode_beta(x: i32, y: i32) -> u64 {
    for i in 0..CARDINAL_DIRECTIONS.len() {
        if x == CARDINAL_DIRECTIONS[i].0 && y == CARDINAL_DIRECTIONS[i].1 {
            return i as u64
        }
    }
    0
}

pub fn decode_beta(value: u64) -> (i32, i32) {
    let index = ((value as usize)) % CARDINAL_DIRECTIONS.len();
    CARDINAL_DIRECTIONS[index]
}

pub fn set_bit(bitmask: &mut u64, value: bool, bit_index: u8) {
    if value {
        *bitmask |= 1 << bit_index;
    } else {
        *bitmask &= !(1 << bit_index);
    };
}

pub fn get_bit(bitmask: u64, bit_index: u8) -> bool {
    bitmask & (1 << bit_index) != 0
}
