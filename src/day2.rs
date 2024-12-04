pub fn solve(input: &str) -> i32 {
    let mut total = 0;

    for line in input.lines() {

        let result: Vec<i32> = line
            .split(|c: char| !c.is_numeric())
            .filter_map(|num| num.parse::<i32>().ok())
            .collect::<Vec<_>>()
            .windows(2)
            .map(|pair| pair[0] - pair[1])
            .collect();



        for (x, y) in result.tuple_windows(2) {
            if x 
            

        }

        let min = result.iter().min().unwrap();
        let max = result.iter().max().unwrap();

        if *max <= 3 && *min >= 1 || *max <= -1 && *min >= -3 {
            println!("Yes");
            total += 1
        } else {
            println!("No");
        }
    }

    total
}
