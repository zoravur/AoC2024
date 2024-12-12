fn dfs(map: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    let mut total = 0;

    let mut stack: Vec<(i32, i32, i32)> = vec![(i, j, -1)];

    while !stack.is_empty() {
        let (i, j, l) = stack.pop().unwrap();
        //println!("{} {} {}", i, j, l);
        if i < 0 || j < 0 {
            continue;
        }
        let si: usize = i.try_into().unwrap();
        let sj: usize = j.try_into().unwrap();
        if si >= map.len() || sj >= map[0].len() {
            continue;
        }

        if map[si][sj] != l + 1 {
            continue;
        }

        if visited[si][sj] {
            continue;
        }

        //println!("Visiting {} {}", i, j);

        visited[si][sj] = true;

        if map[si][sj] == 9 {
            total += 1;
        }

        stack.push((i + 1, j, map[si][sj]));
        stack.push((i - 1, j, map[si][sj]));
        stack.push((i, j + 1, map[si][sj]));
        stack.push((i, j - 1, map[si][sj]));
    }

    total
}

pub fn solve(input: &str) -> i32 {
    let lines = input.lines();
    let map: Vec<Vec<i32>> = lines
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let mut total = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                let res = dfs(&map, i as i32, j as i32);
                println!("total trails found at {} {}: {}", i, j, res);

                total += res;
            }
        }
    }

    total
}
