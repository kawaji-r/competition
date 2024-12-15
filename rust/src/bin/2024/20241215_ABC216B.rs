use proconio::input;

fn main() {
    input! {a1: i32,a2: [(String, String); a1]}
    let result = execute(a2);
    println!("{}", result);
}

fn execute(a2: Vec<(String, String)>) -> String {
    let mut result = "No";
    for i in 0..a2.len() {
        for j in i + 1..a2.len() {
            if a2[i].0 == a2[j].0 && a2[i].1 == a2[j].1 {
                result = "Yes";
                break;
            }
        }
    }
    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = (
            3,
            vec![
                ("tanaka".to_string(), "taro".to_string()),
                ("sato".to_string(), "hanako".to_string()),
                ("tanaka".to_string(), "taro".to_string()),
            ],
        );
        let result = execute(input.1);
        assert_eq!(result, "Yes");
    }

    #[test]
    fn test_no2() {
        let input = (
            3,
            vec![
                ("saito".to_string(), "ichiro".to_string()),
                ("saito".to_string(), "jiro".to_string()),
                ("saito".to_string(), "saburo".to_string()),
            ],
        );
        let result = execute(input.1);
        assert_eq!(result, "No");
    }

    #[test]
    fn test_no3() {
        let input = (
            4,
            vec![
                ("sypdgidop".to_string(), "bkseq".to_string()),
                ("bajsqz".to_string(), "hh".to_string()),
                ("ozjekw".to_string(), "mcybmtt".to_string()),
                ("qfeysvw".to_string(), "dbo".to_string()),
            ],
        );
        let result = execute(input.1);
        assert_eq!(result, "No");
    }
}
