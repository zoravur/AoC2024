#[derive(PartialEq, Debug)]
enum DFA {
    Start,
    Em,
    You,
    Ell,
    Left,
    LeftDigit,
    Comma,
    RightDigit,
    Right,
}

pub fn solve(input: &str) -> i64 {
    let mut state = DFA::Start;

    let mut sum: i64 = 0;
    let mut left_num: i64 = 0;
    let mut right_num: i64 = 0;

    for c in input.chars() {
        state = match (state, c) {
            (DFA::Start, 'm') => DFA::Em,
            (DFA::Em, 'u') => DFA::You,
            (DFA::You, 'l') => DFA::Ell,
            (DFA::Ell, '(') => DFA::Left,
            (DFA::Left, '0'..='9') => DFA::LeftDigit,
            (DFA::LeftDigit, '0'..='9') => DFA::LeftDigit,
            (DFA::LeftDigit, ',') => DFA::Comma,
            (DFA::Comma, '0'..='9') => DFA::RightDigit,
            (DFA::RightDigit, '0'..='9') => DFA::RightDigit,
            (DFA::RightDigit, ')') => DFA::Right,
            (_, _) => DFA::Start,
        };

        // println!("state = {:?}", state);

        if state == DFA::LeftDigit {
            left_num *= 10;
            left_num += (c as i64) - ('0' as i64);
        };

        if state == DFA::RightDigit {
            right_num *= 10;
            right_num += (c as i64) - ('0' as i64);
        };

        if state == DFA::Right {
            sum += left_num * right_num;
            state = DFA::Start;
        }

        if state == DFA::Start {
            left_num = 0;
            right_num = 0;
        }

        // println!(
        //     "left_num: {:?}, right_num: {:?}, sum: {:?}, final state: {:?}",
        //     left_num, right_num, sum, state
        // );
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(""), 0);
        assert_eq!(solve("mul(
    }
}
