pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size]; size];
    let mut loc = (0, 0); // location
    let mut dir = (0, 1); // direction
    let valid_indexes = 0..size as isize;

    for i in 1..=(size * size) as u32 {
        matrix[loc.0 as usize][loc.1 as usize] = i;
        let mut next = (loc.0 + dir.0, loc.1 + dir.1);

        if !valid_indexes.contains(&next.0)
            || !valid_indexes.contains(&next.1)
            || matrix[next.0 as usize][next.1 as usize] != 0
        {
            dir = match dir {
                (0, 1) => (1, 0),   // right => down
                (1, 0) => (0, -1),  // down => left
                (0, -1) => (-1, 0), // left => up
                (-1, 0) => (0, 1),  // up => right
                _ => unreachable!(),
            };
            next = (loc.0 + dir.0, loc.1 + dir.1);
        }
        loc = next;
    }
    matrix
}
