use std::cmp::Ordering;
use std::collections::HashSet;

fn find_indexes<F>(slice: &[u64], ordering: Ordering, reindex: F) -> Vec<(usize, usize)>
where
    F: Fn(usize) -> (usize, usize),
{
    let min_or_max = match ordering {
        Ordering::Greater => slice.iter().max(),
        Ordering::Less => slice.iter().min(),
        _ => None,
    };

    slice
        .iter()
        .enumerate()
        .filter_map(|(idx, &x)| {
            if &x == min_or_max.unwrap() {
                return Some(reindex(idx));
            }
            None
        })
        .collect()
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut maxs = Vec::<(usize, usize)>::new();
    let mut mins = HashSet::<(usize, usize)>::new();

    for (row_idx, row) in input.iter().enumerate() {
        maxs.extend(find_indexes(&row, Ordering::Greater, |i| (row_idx, i)));
    }

    let (rows, cols) = (input.len(), input[0].len());
    for col_idx in 0..cols {
        let col: Vec<u64> = (0..rows).map(|row_idx| input[row_idx][col_idx]).collect();
        mins.extend(find_indexes(&col, Ordering::Less, |i| (i, col_idx)));
    }

    maxs.retain(|&max| mins.contains(&max));
    maxs
}
