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
    let result = if a != 0 && b != 0 {
        "Alloy"
    } else if a == 0 {
        "Silver"
    } else {
        "Gold"
    };
    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = (50, 50);
        let result = execute(input.0, input.1);
        assert_eq!(result, "Alloy");
    }

    #[test]
    fn test_no2() {
        let input = (100, 0);
        let result = execute(input.0, input.1);
        assert_eq!(result, "Gold");
    }

    #[test]
    fn test_no3() {
        let input = (0, 100);
        let result = execute(input.0, input.1);
        assert_eq!(result, "Silver");
    }

    #[test]
    fn test_no4() {
        let input = (2, 100);
        let result = execute(input.0, input.1);
        assert_eq!(result, "Alloy");
    }
}
