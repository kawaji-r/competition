use proconio::input;

fn main() {
    input! {
        a: String,
    }
    let result = execute(a);
    println!("{}", result);
}

fn execute(a: String) -> String {
    let count = a.matches('1').count();
    format!("{}", count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = ("101".to_string(),);
        let result = execute(input.0);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_no2() {
        let input = ("000".to_string(),);
        let result = execute(input.0);
        assert_eq!(result, "0");
    }
}
