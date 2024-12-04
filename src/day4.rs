fn check<I, J>(puzzle: &Vec<Vec<char>>, (i, j): (i32, i32), x_offs: I, y_offs: J) -> bool
where
    I: IntoIterator<Item = i32>,
    J: IntoIterator<Item = i32>,
{
    println!("check called");
    let prospect = x_offs
        .into_iter()
        .zip(y_offs.into_iter())
        .map(|(di, dj)| puzzle[(i + di) as usize][(j + dj) as usize])
        .collect::<String>();

    println!("{}", prospect);
    prospect == "XMAS"
}

fn check_x(puzzle: &Vec<Vec<char>>, (i, j): (i32, i32)) -> bool {
    println!("check called");

    let mut counts = [0; 256];

    if puzzle[i as usize][j as usize] != 'A' {
        return false;
    }

    counts[puzzle[(i - 1) as usize][(j - 1) as usize] as usize] += 1;
    counts[puzzle[(i + 1) as usize][(j - 1) as usize] as usize] += 1;
    counts[puzzle[(i + 1) as usize][(j + 1) as usize] as usize] += 1;
    counts[puzzle[(i - 1) as usize][(j + 1) as usize] as usize] += 1;

    counts['M' as usize] == 2
        && counts['S' as usize] == 2
        && puzzle[(i - 1) as usize][(j - 1) as usize] != puzzle[(i + 1) as usize][(j + 1) as usize]
}

pub fn solve(input: &str) -> i32 {
    let lines = input.lines();
    let lines: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let mut count = 0;

    for i in 1..lines.len() - 1 {
        for j in 1..lines[0].len() - 1 {
            if check_x(&lines, (i as i32, j as i32)) {
                count += 1
            };
        }
    }

    count
    //println!("{}", lines[0][0]);
}
