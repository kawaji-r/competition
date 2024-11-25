use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    let result = execute(a, b);
    println!("{}", result);
}

fn execute(a: i32, b: i32) -> String {
    let result = a ^ b;
    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = (3, 6);
        let result = execute(input.0, input.1);
        assert_eq!(result, "5");
    }

    #[test]
    fn test_no2() {
        let input = (10, 12);
        let result = execute(input.0, input.1);
        assert_eq!(result, "6");
    }
}
