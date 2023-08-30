use crate::rsc::CONNEX_NUMBER_RANGE;

const MAP_LEN: usize = (CONNEX_NUMBER_RANGE[1] + 1) as usize;

const fn lfsr_1(seed: usize) -> usize {
    seed.wrapping_mul(17624813).wrapping_add(7069067389)
}

const fn lfsr_2(seed: usize) -> usize {
    seed.wrapping_mul(9737333).wrapping_add(326851121)
}

const fn lfsr_3(seed: usize) -> usize {
    seed.wrapping_mul(648391).wrapping_add(174440041)
}

const fn gen_map() -> [(bool, bool, bool, bool, bool); MAP_LEN] {
    let mut res = [(false, false, false, false, false); MAP_LEN];

    let mut i: usize = 0;
    while i < MAP_LEN {
        let isub = i.saturating_sub(1);
        let g2 = (isub / 5) % 5;
        let (g4, g5, g6) = if i < 21 {
                (5, 5, 5)
             } else {
                (lfsr_1(isub) % 5, lfsr_2(isub) % 5, lfsr_3(isub) % 5)
             };

        res[i].0 = g2 == 0 || g4 == 0 || g5 == 0 || g6 == 0 || i == 200;
        res[i].1 = g2 == 1 || g4 == 1 || g5 == 1 || g6 == 1 || i == 200;
        res[i].2 = g2 == 2 || g4 == 2 || g5 == 2 || g6 == 2 || i == 200;
        res[i].3 = g2 == 3 || g4 == 3 || g5 == 3 || g6 == 3 || i == 200;
        res[i].4 = (g2 == 4 && (g4 == 1 || g5 == 2 || g6 == 3) && (i%2==0) && (i%10!=0)) || i == 20;
        i += 1;
    }
    res
}

pub const CONX_MAP: [(bool, bool, bool, bool, bool); 201] = gen_map();
