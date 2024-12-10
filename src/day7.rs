fn anticoncat(concattand: i64, concatter: i64) -> Option<i64> {
    let mut placeval: i64 = 1;
    while placeval <= concatter {
        placeval *= 10;
    }
    if concattand % placeval == concatter {
        Some(concattand / placeval)
    } else {
        None
    }
}

fn test_product(target: i64, numbers: &[i64]) -> bool {
    //println!("{}", target);
    if target == 0 && numbers.is_empty() {
        return true;
    }
    if numbers.is_empty() {
        return false;
    }
    let last_num = numbers[numbers.len() - 1];
    let first_numbers = &numbers[0..numbers.len() - 1];

    if let Some(anticoncat_res) = anticoncat(target, last_num) {
        if test_product(anticoncat_res, first_numbers) {
            return true;
        }
    }

    if test_product(target - last_num, first_numbers) {
        return true;
    }

    if target % last_num == 0 && test_product(target / last_num, first_numbers) {
        return true;
    }

    false
}

pub fn solve(input: &str) -> i64 {
    let lines = input.lines();

    let mut total = 0;

    for line in lines {
        if let Some((left, right)) = line.split_once(": ") {
            let target = left.parse::<i64>().unwrap();
            let numbers: Vec<i64> = right
                .split(' ')
                .map(|s| s.parse::<i64>())
                .collect::<Result<Vec<i64>, _>>()
                .unwrap();

            if test_product(target, &numbers) {
                println!("Solution found for target {}", target);
                total += target;
            }
        }
    }

    total
}
