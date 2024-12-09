use proconio::input;
use std::collections::HashSet;

fn main() {
    // modify
    input! {
        a: String,
        b: usize,
    }
    // modify
    let result = execute(a, b);
    println!("{}", result);
}

// modify
fn execute(a: String, b: usize) -> String {
    let mut permutations = generate_permutations(&a);
    permutations.sort(); // 辞書順にソート
    let result = permutations.into_iter().nth(b - 1); // b 番目の要素を取得 (0-indexed)
    format!("{}", result.unwrap())
}

fn generate_permutations(s: &str) -> Vec<String> {
    let mut chars: Vec<char> = s.chars().collect();
    let mut result = HashSet::new();
    permute(&mut chars, 0, &mut result);
    result.into_iter().collect()
}

fn permute(chars: &mut Vec<char>, start: usize, result: &mut HashSet<String>) {
    if start == chars.len() {
        result.insert(chars.iter().collect());
    } else {
        for i in start..chars.len() {
            chars.swap(start, i);
            permute(chars, start + 1, result);
            chars.swap(start, i); // 元に戻す
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = ("aab".to_string(), 2);
        let result = execute(input.0, input.1);
        assert_eq!(result, "aba");
    }

    #[test]
    fn test_no2() {
        let input = ("baba".to_string(), 4);
        let result = execute(input.0, input.1);
        assert_eq!(result, "baab");
    }

    #[test]
    fn test_no3() {
        let input = ("ydxwacbz".to_string(), 40320);
        let result = execute(input.0, input.1);
        assert_eq!(result, "zyxwdcba");
    }
}
