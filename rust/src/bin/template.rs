use proconio::input;

fn main() {
    // modify
    input! {
        a: i32,
        b: String,
    }
    // modify
    let result = execute(a, b);
    println!("{}", result);
}

// modify
fn execute(a: i32, b: String) -> String {
    let result = a.to_string() + &b;
    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        // modify
        let input = (1, "star".to_string());
        // modify
        let result = execute(input.0, input.1);
        // modify
        assert_eq!(result, "1star");
    }
}
