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
    let mut cnt = 0;
    for i in 0..=a {
        for j in 0..=a - i {
            for k in 0..=a - i - j {
                if i * j * k <= b {
                    cnt += 1;
                }
            }
        }
    }
    format!("{}", cnt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = (1, 0);
        let result = execute(input.0, input.1);
        assert_eq!(result, "4");
    }

    #[test]
    fn test_no2() {
        let input = (2, 5);
        let result = execute(input.0, input.1);
        assert_eq!(result, "10");
    }

    #[test]
    fn test_no3() {
        let input = (10, 10);
        let result = execute(input.0, input.1);
        assert_eq!(result, "213");
    }

    #[test]
    fn test_no4() {
        let input = (30, 100);
        let result = execute(input.0, input.1);
        assert_eq!(result, "2471");
    }
}
