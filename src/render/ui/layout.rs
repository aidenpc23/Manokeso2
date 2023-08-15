use crate::{
    render::ui::text::{Align, Text},
    util::point::Point,
};

pub const BOARD: [Text; 2] = [
    // Text {
    //     update: |state| {
    //         if let Some(pos) = state.hovered_tile {
    //             let i = pos.index(b.width());
    //             format!(
    //                 concat!(
    //                     "tile pos: {:?}\n",
    //                     "connex number: {:?}\n",
    //                     "stability: {:?}\n",
    //                     "reactivity: {:?}\n",
    //                     "energy: {:?}\n",
    //                     "alpha: {:?}\n",
    //                     "beta: {:?}\n",
    //                     "gamma: {:?}\n",
    //                     "delta: {:?}\n",
    //                     "omega: {:?}\n",
    //                 ),
    //                 pos,
    //                 b.connex_numbers.read()[i],
    //                 b.stability.read()[i],
    //                 b.reactivity.read()[i],
    //                 b.energy.read()[i],
    //                 b.alpha.read()[i],
    //                 b.beta.read()[i],
    //                 b.gamma.read()[i],
    //                 b.delta.read()[i],
    //                 b.omega.read()[i],
    //             )
    //         } else {
    //             "no tile selected".to_string()
    //         }
    //     },
    //     pos: |(_, _)| Point { x: 10.0, y: 10.0 },
    //     align: Align::Left,
    //     bounds: |(w, h)| (w / 3.0, h),
    // },
    Text {
        update: |state| format!("total energy: {:?}", state.tile_view.total_energy),
        pos: |(w, _)| Point {
            x: w / 2.0,
            y: 10.0,
        },
        align: Align::Center,
        bounds: |(w, h)| (w / 3.0, h),
    },
    Text {
        update: |state| {
            format!(
                concat!("frame time: {:?}\n", "update time: {:?}",),
                state.timers.render.avg(),
                state.timers.update.avg()
            )
        },
        pos: |(w, _)| Point {
            x: w - 10.0,
            y: 10.0,
        },
        align: Align::Right,
        bounds: |(w, h)| (w / 3.0, h),
    },
];
