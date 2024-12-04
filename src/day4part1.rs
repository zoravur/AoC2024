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

pub fn solve(input: &str) -> i32 {
    let lines = input.lines();
    let lines: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let mut count = 0;

    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if j >= 3
                && check(
                    &lines,
                    (i as i32, j as i32),
                    std::iter::repeat(0),
                    (-3..=-0).rev(),
                )
            {
                count += 1;
            }
            if i >= 3
                && check(
                    &lines,
                    (i as i32, j as i32),
                    (-3..=-0).rev(),
                    std::iter::repeat(0),
                )
            {
                count += 1;
            }
            if i >= 3
                && j >= 3
                && check(
                    &lines,
                    (i as i32, j as i32),
                    (-3..=-0).rev(),
                    (-3..=-0).rev(),
                )
            {
                count += 1;
            }
            println!("{} {} {}", j, lines[0].len(), 3);
            if j < lines[0].len().saturating_sub(3)
                && check(&lines, (i as i32, j as i32), std::iter::repeat(0), 0..=3)
            {
                count += 1;
            }
            println!("{} {} {}", i, lines.len(), 4);
            if i < lines.len().saturating_sub(3)
                && check(&lines, (i as i32, j as i32), 0..=3, std::iter::repeat(0))
            {
                count += 1;
            }
            if i < lines.len().saturating_sub(3)
                && j < lines[0].len().saturating_sub(3)
                && check(&lines, (i as i32, j as i32), 0..=3, 0..=3)
            {
                count += 1;
            }
            if i >= 3
                && j < lines[0].len().saturating_sub(3)
                && check(&lines, (i as i32, j as i32), (-3..=-0).rev(), 0..=3)
            {
                count += 1;
            }
            if i < lines.len().saturating_sub(3)
                && j >= 3
                && check(&lines, (i as i32, j as i32), 0..=3, (-3..=-0).rev())
            {
                count += 1;
            }
        }
    }

    count
    //println!("{}", lines[0][0]);
}
