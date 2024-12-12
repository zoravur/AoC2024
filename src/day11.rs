pub fn num_digits(num: i64) -> i64 {
    let mut i = 1;
    let mut cnt = 0;
    // let mut half_cnt = 0;
    loop {
        i *= 10;
        cnt += 1;
        if i >= num {
            return cnt;
        }
    }
}

pub fn try_split(num: i64) -> Option<(i64, i64)> {
    let mut i = 1;
    let mut j = 1;
    let mut cnt = 0;
    let mut half_cnt = 0;
    // let mut half_cnt = 0;
    loop {
        i *= 10;
        cnt += 1;
        if cnt % 2 == 0 {
            j *= 10;
            half_cnt += 1;
        }
        if i > num {
            if half_cnt * 2 == cnt {
                return Some((num / j, num % j));
            } else {
                return None;
            }
        }
    }
}

pub fn solve(input: &str) -> i64 {
    let mut stone_cnt: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();

    let stones = input
        .trim()
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    for stone in stones.as_slice() {
        if let Some(cnt) = stone_cnt.get(stone) {
            stone_cnt.insert(*stone, cnt + 1);
        } else {
            stone_cnt.insert(*stone, 1);
        }
    }

    for step in 0..75 {
        let mut new_stone_cnt: std::collections::HashMap<i64, i64> =
            std::collections::HashMap::new();
        for (stone, cnt) in stone_cnt.iter() {
            if *stone == 0 {
                *new_stone_cnt.entry(1).or_insert(0) += cnt;
            } else if let Some((left, right)) = try_split(*stone) {
                *new_stone_cnt.entry(left).or_insert(0) += cnt;
                *new_stone_cnt.entry(right).or_insert(0) += cnt;
            } else {
                *new_stone_cnt.entry(stone * 2024).or_insert(0) += cnt;
            }
        }

        stone_cnt = new_stone_cnt;
        /*
        for i in 0..stones.len() {
            // let i: usize = i.try_into().unwrap();
            if stones[i] == 0 {
                stones[i] = 1;
            } else if let Some((left, right)) = try_split(stones[i]) {
                stones[i] = left;
                stones.push(right);
            } else {
                stones[i] *= 2024;
            }
        }
        */

        if step < 6 {
            println!("Stones: {:?}", stone_cnt);
        }
        println!("Stones length at step {}: {}", step + 1, stones.len());
    }

    stone_cnt.values().sum()
}
