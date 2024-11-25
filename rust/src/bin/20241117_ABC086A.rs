use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    execute(a, b);
    let result = execute(a, b);
    println!("{}", result);
}

fn execute(a: i32, b: i32) -> String {
    let product = a * b;
    let remainder = product % 2;
    let result = if remainder == 0 { "Even" } else { "Odd" };
    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = (3, 4);
        execute(input.0, input.1);
        let result = execute(input.0, input.1);
        assert_eq!(result, "Even");
    }

    #[test]
    fn test_no2() {
        let input = (1, 21);
        execute(input.0, input.1);
        let result = execute(input.0, input.1);
        assert_eq!(result, "Odd");
    }
}
