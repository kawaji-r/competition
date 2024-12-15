use proconio::input;

fn main() {
    input! {a1: usize,a2: String}
    let result = execute(a1, a2);
    println!("{}", result);
}

fn execute(a1: usize, a2: String) -> String {
    let c = a2.chars().nth(a1 - 1).unwrap();
    let result = if c == 'o' { "Yes" } else { "No" };
    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = (4, "oooxoox".to_string());
        let result = execute(input.0, input.1);
        assert_eq!(result, "No");
    }

    #[test]
    fn test_no2() {
        let input = (7, "ooooooo".to_string());
        let result = execute(input.0, input.1);
        assert_eq!(result, "Yes");
    }
}
