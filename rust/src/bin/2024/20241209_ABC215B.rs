use proconio::input;

fn main() {
    // modify
    input! {
        a: i64,
    }
    // modify
    let result = execute(a);
    println!("{}", result);
}

// modify
fn execute(a: i64) -> String {
    let mut num = 2;
    let mut cnt = 0;
    while num <= a {
        num *= 2;
        cnt += 1;
    }
    format!("{}", cnt.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = (6,);
        let result = execute(input.0);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_no2() {
        let input = (1,);
        let result = execute(input.0);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_no3() {
        let input = (1000000000000000000,);
        let result = execute(input.0);
        assert_eq!(result, "59");
    }
}
