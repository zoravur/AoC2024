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

fn next_dir(dir: (isize, isize)) -> (isize, isize) {
    match dir {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => panic!("Impossible"),
    }
}

fn dir_to_chr(dir: (isize, isize)) -> char {
    match dir {
        (-1, 0) => '^',
        (0, 1) => '>',
        (1, 0) => 'v',
        (0, -1) => '<',
        _ => panic!("Impossible"),
    }
}

pub fn solve(input: &str) -> i32 {
    let mut map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().into_iter().collect())
        .collect();

    let mut pos: (usize, usize) = (usize::MAX, usize::MAX);
    let limit = (map.len(), map[0].len());
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' {
                pos = (i, j);
                //map[i][j] = 'X';
            }
        }
    }
    let mut dir = (0, -1);

    let opposite_dir = next_dir(next_dir(dir));

    let mut ray = pos;
    while let Some(new_ray) = safe_size_tuple_add(ray, opposite_dir, limit) {
        if map[new_ray.0][new_ray.1] != '#' {
            ray = new_ray;
            map[ray.0][ray.1] = dir_to_chr(dir);
        }
    }

    let mut cnt = 0;
    while let Some(next_pos) = safe_size_tuple_add(pos, dir, limit) {
        if map[next_pos.0][next_pos.1] == '#' {
            dir = next_dir(dir);
            let opposite_dir = next_dir(next_dir(dir));

            let mut ray = pos;
            while let Some(new_ray) = safe_size_tuple_add(ray, opposite_dir, limit) {
                if map[new_ray.0][new_ray.1] != '#' {
                    ray = new_ray;
                    map[ray.0][ray.1] = dir_to_chr(dir);
                } else {
                    break;
                }
            }
        } else {
            pos = next_pos;
            if map[pos.0][pos.1] == dir_to_chr(next_dir(dir)) {
                if let Some((obsi, obsj)) = safe_size_tuple_add(pos, dir, limit) {
                    println!("({},{}) <- an obstacle can be placed here", obsi, obsj);
                    cnt += 1;
                }
            }
            map[pos.0][pos.1] = dir_to_chr(dir);
        }

        for line in &map {
            for c in line {
                print!("{}", c);
            }
            println!();
        }
        println!();
    }

    //let mut cnt = 0;
    //for i in 0..map.len() {
    //     for j in 0..map[0].len() {
    //         if map[i][j] == 'X' {
    //             cnt += 1
    //         }
    //     }
    // }

    cnt
}
