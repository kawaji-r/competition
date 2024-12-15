use proconio::input;

fn main() {
    input! {a1: String,a2: String}
    let result = execute(a1, a2);
    println!("{}", result);
}

fn execute(a1: String, a2: String) -> String {
    let answer = {
        if a1 < a2 {
            "Yes"
        } else {
            "No"
        }
    };
    format!("{}", answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = ("abc".to_string(), "atcoder".to_string());
        let result = execute(input.0, input.1);
        assert_eq!(result, "Yes");
    }

    #[test]
    fn test_no2() {
        let input = ("arc".to_string(), "agc".to_string());
        let result = execute(input.0, input.1);
        assert_eq!(result, "No");
    }

    #[test]
    fn test_no3() {
        let input = ("a".to_string(), "aa".to_string());
        let result = execute(input.0, input.1);
        assert_eq!(result, "Yes");
    }
}
