/// 文字列の切り出し
use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let result = execute(s);
    println!("{}", result);
}

fn execute(s: String) -> String {
    let numbers: Vec<i32> = s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    let all_same = numbers.windows(2).all(|w| w[0] == w[1]);
    let sequential = numbers.windows(2).all(|w| w[1] == (w[0] + 1) % 10);

    if all_same || sequential {
        "Weak".to_string()
    } else {
        "Strong".to_string()
    }
}

#[allow(dead_code)]
fn execute2(s: String) -> String {
    let no1: i32 = s[0..1].parse().unwrap();
    let no2: i32 = s[1..2].parse().unwrap();
    let no3: i32 = s[2..3].parse().unwrap();
    let no4: i32 = s[3..4].parse().unwrap();
    let flag_1 = no1 == no2 && no2 == no3 && no3 == no4;
    let flag_2 = chk_next_num(no1, no2);
    let flag_3 = chk_next_num(no2, no3);
    let flag_4 = chk_next_num(no3, no4);
    let result = if flag_1 || (flag_2 && flag_3 && flag_4) {
        "Weak"
    } else {
        "Strong"
    };
    format!("{}", result)
}

fn chk_next_num(first: i32, second: i32) -> bool {
    if first == 9 {
        if second == 0 {
            true
        } else {
            false
        }
    } else {
        if second == first + 1 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = ("7777".to_string(),);
        let result = execute(input.0);
        assert_eq!(result, "Weak");
    }

    #[test]
    fn test_no2() {
        let input = ("0112".to_string(),);
        let result = execute(input.0);
        assert_eq!(result, "Strong");
    }

    #[test]
    fn test_no3() {
        let input = ("9012".to_string(),);
        let result = execute(input.0);
        assert_eq!(result, "Weak");
    }
}
