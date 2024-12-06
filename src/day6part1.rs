fn safe_size_tuple_add(
    (a1, b1): (usize, usize),
    (a2, b2): (isize, isize),
    limit: (usize, usize),
) -> Option<(usize, usize)> {
    let candidate = (a1.checked_add_signed(a2), b1.checked_add_signed(b2));

    match candidate {
        (Some(x), Some(y)) if x < limit.0 && y < limit.1 => Some((x, y)),
        _ => None,
    }
}

pub fn solve(input: &str) -> i32 {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut pos: (usize, usize) = (usize::MAX, usize::MAX);
    let limit = (map.len(), map[0].len());
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' {
                pos = (i, j);
                map[i][j] = 'X';
            }
        }
    }
    let mut dir = (-1, 0);

    while let Some((newi, newj)) = safe_size_tuple_add(pos, dir, limit) {
        if map[newi][newj] == '#' {
            dir = match dir {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => panic!("Impossible"),
            };
        } else {
            pos = (newi, newj);
            map[newi][newj] = 'X';
        }
    }

    let mut cnt = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'X' {
                cnt += 1
            }
        }
    }

    cnt
}
