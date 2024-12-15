use proconio::input;

fn main() {
    input! {
        a: i32,
        b: [i32; a]
    };
    let result = execute(b);
    println!("{}", result);
}

// 処理時間とメモリ容量で、execute2よりこっちの方が優れている
fn execute(b: Vec<i32>) -> String {
    let mut largest = (0, 1); // 最大値とそのインデックス
    let mut second_largest = (0, 1); // 2番目の最大値とそのインデックス
    for (i, &num) in b.iter().enumerate() {
        if num > largest.0 {
            // 最大値を更新
            second_largest = largest;
            largest = (num, i);
        } else if num > second_largest.0 {
            // 2番目の最大値を更新
            second_largest = (num, i);
        }
    }
    format!("{}", second_largest.1 + 1) // 2番目に大きい値のインデックスを返す
}

#[allow(dead_code)]
fn execute2(b: Vec<i32>) -> String {
    // インデックスと値のペアを作成
    let mut indexed_numbers: Vec<(usize, i32)> =
        b.iter().enumerate().map(|(i, &val)| (i, val)).collect();

    // 値でソート（降順）
    indexed_numbers.sort_by(|a, b| b.1.cmp(&a.1));

    // 2番目に大きい値のインデックスを返す
    // let second_largest = indexed_numbers.get(1).map(|&(index, _)| index).unwrap_or(); // パニックにならない書き方
    let second_largest = indexed_numbers[1].0;

    format!("{}", second_largest + 1) // 2番目に大きい値のインデックスを返す
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no1() {
        let input = (6, vec![1, 123, 12345, 12, 1234, 123456]);
        let result = execute(input.1);
        assert_eq!(result, "3");
    }

    #[test]
    fn test_no2() {
        let input = (5, vec![3, 1, 4, 15, 9]);
        let result = execute(input.1);
        assert_eq!(result, "5");
    }
}
