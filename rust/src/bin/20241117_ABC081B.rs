/// 不定数の入力
use proconio::input;

fn main() {
    input! {
        mut a: i32,
        mut b: [i32; a]
    };
    let result = execute(b);
    println!("{}", result);
}

fn execute(mut b: Vec<i32>) -> String {
    let mut count = 0;
    while b.iter().all(|&x| x % 2 == 0) {
        b.iter_mut().for_each(|x| *x /= 2);
        count += 1;
    }
    count.to_string()
}

#[allow(dead_code)]
fn execute2(mut b: Vec<i32>) -> String {
    let mut count = 0;
    let mut flag = true;
    while flag {
        for item in b.iter_mut() {
            if *item % 2 == 0 {
                *item /= 2;
            } else {
                flag = false;
            };
        }
        if flag {
            count += 1;
        }
    }
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = (3, vec![8, 12, 40]);
        let result = execute(input.1);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_no2() {
        let input = (4, vec![5, 6, 8, 10]);
        let result = execute(input.1);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_no3() {
        let input = (
            6,
            vec![
                382253568, 723152896, 37802240, 379425024, 404894720, 471526144,
            ],
        );
        let result = execute(input.1);
        assert_eq!(result, "8");
    }
    #[test]
    fn test_no4() {
        let input = (3, vec![8, 12, 40]);
        let result = execute2(input.1);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_no5() {
        let input = (4, vec![5, 6, 8, 10]);
        let result = execute2(input.1);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_no6() {
        let input = (
            6,
            vec![
                382253568, 723152896, 37802240, 379425024, 404894720, 471526144,
            ],
        );
        let result = execute2(input.1);
        assert_eq!(result, "8");
    }
}
