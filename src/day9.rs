use std::cmp::min;

pub fn solve(input: &str) -> i64 {
    let mut input: Vec<(Option<i32>, i32)> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .enumerate()
        .map(|(i, val)| {
            if i % 2 == 0 {
                (Some((i / 2) as i32), val)
            } else {
                (None, val)
            }
        })
        .collect();

    let mut 


    let ordering = vec![];

    for (value, size) in input.into_iter().rev() {
        if let Some(


    }

    println!("{:?}", input);

    let mut block_number = 0;
    let mut i = 0;
    let mut j = input.len() - 1;
    let mut checksum: i64 = 0;

    //println!("expanded: {}", expanded);
    checksum
}
