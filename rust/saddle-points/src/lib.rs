extern crate ndarray;

use ndarray::prelude::*;
use std::collections::HashSet;

fn find<F>(input: &[u64], value: &u64, index: F) -> Vec<(usize, usize)>
where
    F: Fn(usize) -> (usize, usize),
{
    input
        .iter()
        .enumerate()
        .filter_map(|(idx, x)| if value == x { Some(index(idx)) } else { None })
        .collect()
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let flat: Vec<u64> = input.iter().flat_map(|x| x.clone()).collect();
    let arr = Array::from_shape_vec((input.len(), flat.len() / input.len()), flat).unwrap();

    let mut maxs: HashSet<(usize, usize)> = HashSet::new();
    for (row_idx, row) in input.iter().enumerate() {
        let max = row.iter().max().unwrap();
        maxs.extend(find(&row, max, |i| (row_idx, i)));
    }

    let mut mins: HashSet<(usize, usize)> = HashSet::new();
    for (col_idx, col_view) in arr.lanes(Axis(0)).into_iter().enumerate() {
        let col: Vec<u64> = col_view.iter().map(|x| x.clone()).collect();
        let min = col.iter().min().unwrap();
        mins.extend(find(&col, min, |i| (i, col_idx)));
    }
    let mut value: Vec<(usize, usize)> = maxs.intersection(&mins).map(|x| x.clone()).collect();
    value.sort_unstable();
    value
}
