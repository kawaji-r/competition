use proconio::input;

fn main() {
    // modify
    input! {
        a: String,
    }
    // modify
    let result = execute(a);
    println!("{}", result);
}

// modify
fn execute(a: String) -> String {
    let arr: Vec<&str> = a.split('.').collect();
    let x = arr[0];
    let y = arr[1].parse().unwrap();
    let suffix = match y {
        0..=2 => "-",
        3..=6 => "",
        7..=9 => "+",
        _ => "",
    };
    format!("{}{}", x, suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = ("15.8".to_string(),);
        let result = execute(input.0);
        assert_eq!(result, "15+");
    }

    #[test]
    fn test_no2() {
        let input = ("1.0".to_string(),);
        let result = execute(input.0);
        assert_eq!(result, "1-");
    }

    #[test]
    fn test_no3() {
        let input = ("12.5".to_string(),);
        let result = execute(input.0);
        assert_eq!(result, "12");
    }
}
