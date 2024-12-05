use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Order {
    U,
    B,
}

impl fmt::Debug for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if *self == Order::U { " " } else { "B" })
    }
}

pub fn solve(input: &str) -> i32 {
    const GSIZE: usize = 100;

    let mut lines = input.lines();

    let mut graph = [[Order::U; GSIZE]; GSIZE];

    println!("{:?} {:?}", graph[0][0], graph[1][0]);

    loop {
        let rule = lines.next().unwrap();
        if rule.is_empty() {
            break;
        }
        let nums: Vec<usize> = rule
            .split('|')
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
        let (n, m) = (nums[0], nums[1]);

        graph[n][m] = Order::B;
        // graph[m][n] = Order::After;
    }

    for row in graph {
        println!("{:?}", row);
    }
    println!("\n\n");

    /*
    for row in graph {
        println!("{:?}", row);
    }
    */

    let mut total = 0;
    for line in lines {
        let mut nums: Vec<usize> = line
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let mut rotated = false;
        let mut k = 0;
        'outer: while k < nums.len() {
            // assume 0, .., k-1 is sorted
            for i in (0..k).rev() {
                if graph[nums[k]][nums[i]] == Order::B {
                    //println!("{:?}, {}, {}", nums, i, k);
                    nums[i..k + 1].rotate_right(1);
                    //println!("{:?}, {}, {}", nums, i, k);
                    rotated = true;
                    k = i;
                    continue 'outer;
                }
            }
            k += 1;
        }

        if rotated {
            let middle = nums[nums.len() / 2];
            total += middle;
        }
    }

    total as i32
}
