use proconio::input;

fn main() {
    input! {
        a: i32,
    }
    let result = execute(a);
    println!("{}", result);
}

fn execute(a: i32) -> String {
    let result = match a {
        1..=125 => Some(4),
        126..=211 => Some(6),
        212..=214 => Some(8),
        _ => None,
    };
    format!("{}", result.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = (1,);
        let result = execute(input.0);
        assert_eq!(result, "4");
    }

    #[test]
    fn test_no2() {
        let input = (126,);
        let result = execute(input.0);
        assert_eq!(result, "6");
    }

    #[test]
    fn test_no3() {
        let input = (214,);
        let result = execute(input.0);
        assert_eq!(result, "8");
    }
}
