use proconio::input;

fn main() {
    input! {a1: String,a2: String,a3: String}
    let result = execute(a1, a2, a3);
    println!("{}", result);
}

fn execute(a1: String, a2: String, a3: String) -> String {
    let all_contest = ["ABC", "ARC", "AGC", "AHC"];
    let three_contest = [a1.as_str(), a2.as_str(), a3.as_str()];
    let answer = all_contest
        .iter()
        .find(|x| !three_contest.contains(x))
        .unwrap();
    format!("{}", answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = ("ARC".to_string(), "AGC".to_string(), "AHC".to_string());
        let result = execute(input.0, input.1, input.2);
        assert_eq!(result, "ABC");
    }

    #[test]
    fn test_no2() {
        let input = ("AGC".to_string(), "ABC".to_string(), "ARC".to_string());
        let result = execute(input.0, input.1, input.2);
        assert_eq!(result, "AHC");
    }
}
