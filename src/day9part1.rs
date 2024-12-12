use std::cmp::min;

pub fn solve(input: &str) -> i64 {
    let mut input: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect();

    let mut block_number = 0;
    let mut i = 0;
    let mut j = input.len() - 1;
    let mut checksum: i64 = 0;
    //let mut expanded = String::new();

    // let mut extend_left = |j: i32| {
    //     let new_block_number = block_number + input[i];

    //     while block_number < new_block_number {
    //         checksum += (i as i32 / 2) * block_number;
    //         block_number += 1;
    //     }

    //     let new_block_number = block_number + input[i + 1];

    //     while block_number < new_block_number {
    //         checksum += (j as i32 / 2) * block_number;
    //         block_number += 1;
    //     }

    //     i += 2;
    // };

    loop {
        if i % 2 == 0 {
            let new_block_number = block_number + input[i];

            while block_number < new_block_number {
                checksum += (i as i64 / 2) * (block_number as i64);
                //print!("{} * {} + ", block_number, i / 2);
                //expanded.push(char::from_digit((i / 2) as u32, 10).unwrap());
                block_number += 1;
            }

            i += 1;
        }

        if i >= j {
            break;
        }

        let amount_to_shift = min(input[i], input[j]);
        input[j] -= amount_to_shift;
        input[i] -= amount_to_shift;

        let new_block_number = block_number + amount_to_shift;

        while block_number < new_block_number {
            checksum += ((j / 2) as i64) * (block_number as i64);
            //print!("{} * {} + ", block_number, j / 2);
            //expanded.push(char::from_digit((j / 2) as u32, 10).unwrap());
            block_number += 1;
        }

        if input[i] == 0 {
            i += 1;
        }

        if input[j] == 0 {
            j -= 2;
        }
    }

    //println!("expanded: {}", expanded);
    checksum
}
