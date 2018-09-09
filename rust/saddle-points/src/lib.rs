use std::cmp::Ordering;
use std::collections::HashSet;

fn find_indexes<F>(input: &[&u64], ordering: Ordering, reindex: F) -> Vec<(usize, usize)>
where
    F: Fn(usize) -> (usize, usize),
{
    let mut min_or_max: Option<&u64> = None;
    input
        .iter()
        .enumerate()
        .fold(
            Vec::<(usize, usize)>::new(),
            |mut vector, (idx, &x)| match min_or_max {
                None => {
                    min_or_max = Some(x);
                    vec![reindex(idx)]
                }
                Some(current) => match x.cmp(current) {
                    order if order == ordering => {
                        min_or_max = Some(x);
                        vec![reindex(idx)]
                    }
                    Ordering::Equal => {
                        vector.push(reindex(idx));
                        vector
                    }
                    _ => vector,
                },
            },
        )
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut maxs: Vec<(usize, usize)> = Vec::new();
    let mut mins: HashSet<(usize, usize)> = HashSet::new();

    let flat: Vec<&u64> = input.iter().flat_map(|x| x).collect();
    let (height, width) = (input.len(), flat.len() / input.len());
    if height == 0 || width == 0 {
        return maxs;
    }

    for (row_idx, row) in flat.chunks(width).enumerate() {
        maxs.extend(find_indexes(&row, Ordering::Greater, |i| (row_idx, i)));
    }

    for col_idx in 0..width {
        let col: Vec<&u64> = (col_idx..flat.len())
            .step_by(width)
            .map(|x| flat[x])
            .collect();
        mins.extend(find_indexes(&col, Ordering::Less, |i| (i, col_idx)));
    }

    maxs.retain(|&x| mins.contains(&x));
    maxs
}
