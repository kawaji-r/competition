use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        s: String,
    }
    let result = execute(a, b, c, s);
    println!("{}", result);
}

fn execute(a: i32, b: i32, c: i32, s: String) -> String {
    let sum = a + b + c;
    format!("{} {}", sum, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = (1, 5, 7, "star".to_string());
        let result = execute(input.0, input.1, input.2, input.3);
        assert_eq!(result, "13 star");
    }
}
