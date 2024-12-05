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
    for k in 0..GSIZE {
        for i in 0..GSIZE {
            for j in 0..GSIZE {
                if graph[i][k] == Order::B && graph[k][j] == Order::B {
                    //if 0 <= k && k <= 99 {
                    //    continue;
                    //}
                    graph[i][j] = Order::B;
                    // graph[j][i] = Order::After;
                }
            }
        }
    }
    */

    for row in graph {
        println!("{:?}", row);
    }

    let mut total = 0;
    'outer: for line in lines {
        let nums: Vec<usize> = line
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
        let middle = nums[nums.len() / 2];

        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                //println!("{} {} {:?}", i, j, graph[i][j]);
                if graph[nums[j]][nums[i]] == Order::B {
                    continue 'outer;
                }
            }
        }

        //for window in nums.windows(2) {
        //    if graph[window[0]][window[1]] == Order::After {
        //        continue 'outer;
        //    }
        //}

        total += middle;
    }

    total as i32
}
