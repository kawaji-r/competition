use proconio::input;

fn main() {
    // modify
    input! {
        a: String,
    }
    println!("{}", if a == "Hello,World!" { "AC" } else { "WA" });
}

#[allow(dead_code)]
fn execute(a: String) -> String {
    format!("{}", if a == "Hello,World!" { "AC" } else { "WA" })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = ("Hello,World!".to_string(),);
        let result = execute(input.0);
        assert_eq!(result, "AC");
    }

    #[test]
    fn test_no2() {
        let input = ("Hello,world!".to_string(),);
        let result = execute(input.0);
        assert_eq!(result, "WA");
    }

    #[test]
    fn test_no3() {
        let input = ("Hello!World!".to_string(),);
        let result = execute(input.0);
        assert_eq!(result, "WA");
    }
}
