use super::swap_buffer::SwapBuffer;
use crate::common::view::BoardSlice;
use rayon::{
    prelude::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use std::ops::AddAssign;

#[macro_export]
macro_rules! board_attrs {
    ( $bname:ident, $vname:ident, $tname:ident, [$( $name:ident : $type:ident ),* $(,)? ] ) => {
        pub struct $tname {
            $(
                $name: $type,
            )*
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        pub struct $bname {
            $(
                pub $name: crate::board::swap_buffer::SwapBuffer<$type>,
            )*
        }

        impl $bname {
            pub fn set_tile(&mut self, pos: usize, tile: Tile) {
                $(
                    self.$name.r[pos] = tile.$name;
                )*
            }
            pub fn get_tile(&self, pos: usize) -> $tname {
                $tname {
                    $(
                        $name: self.$name.r[pos],
                    )*
                }
            }
            pub fn copy_to_view(&self, view: &mut $vname, slice: &crate::common::view::BoardSlice) {
                $(
                    crate::board::attrs::copy_swap_buf(&mut view.$name, &self.$name, &slice);
                )*
            }
        }

        #[derive(Debug)]
        pub struct $vname {
            $(
                pub $name: Vec<$type>,
            )*
        }

        impl $vname {
            pub fn empty() -> Self {
                Self {
                    $(
                        $name: Vec::new(),
                    )*
                }
            }
        }
    }
}

pub fn copy_swap_buf<T: Send + Sync + Copy + AddAssign>(
    dest: &mut Vec<T>,
    sb: &SwapBuffer<T>,
    slice: &BoardSlice,
) {
    if dest.len() != slice.size {
        *dest = Vec::with_capacity(slice.size);
        unsafe { dest.set_len(slice.size) }
    }
    if slice.size != 0 {
        dest.par_chunks_exact_mut(slice.width)
            .zip(sb.par_rows(slice.start.y, slice.end.y))
            .for_each(|(data, row)| {
                data.copy_from_slice(&row[slice.start.x..slice.end.x]);
            });
    }
}
