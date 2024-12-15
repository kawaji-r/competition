use std::env::var;
use std::fs::{metadata, File};
use std::io::{stdin, Write};
use std::process::Command;

use chrono::Local;

/// 設定情報を生成する関数
fn setting<'a>() -> Settings<'a> {
    Settings {
        // コンテスト名
        contest_name: "ABC218A",
        // 入力の型
        input_types: vec!["i32", "String"],
        // execute関数の引数の型
        execute_types: vec!["i32", "String"],
        // テストの入力と出力
        test: vec![
            vec!["4", r#""oooxoox".to_string()"#],
            vec!["No"],
            vec!["7", r#""ooooooo".to_string()"#],
            vec!["Yes"],
        ],
        // 記入が終わったらコマンドを実行
        // cargo run --bin new_contest

        // test記入例
        // test: vec![
        //     vec!["1", "2", r#""star".to_string()"#],
        //     vec!["Yes"],
        // ],
    }
}

/// 設定情報を管理する構造体
/// - `contest_name`: コンテスト名
/// - `args`: 関数の引数の型
/// - `test`: テストケースの入力と期待される出力
struct Settings<'a> {
    /// コンテスト名
    contest_name: &'a str,
    /// 入力の型
    input_types: Vec<&'a str>,
    /// execute関数の引数の型
    execute_types: Vec<&'a str>,
    /// テストケースの入力と出力
    test: Vec<Vec<&'a str>>,
}

fn main() {
    // 設定を取得
    let settings = setting();
    // 新しいファイル名を生成
    let new_file = make_new_filename(settings.contest_name);
    // ファイルの内容を生成
    let contents = generate_contents(&settings);

    if metadata(&new_file).is_ok() {
        eprintln!("既に同名のファイルが存在します！{}", new_file);
        eprintln!("上書きしてもよいですか？ [y/N]: ");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");
        if input.trim().to_lowercase() != "y" {
            println!("Operation cancelled.");
            return;
        }
    }

    // ファイルを作成して内容を書き込む
    if let Err(e) = File::create(&new_file).and_then(|mut file| write!(file, "{}", contents)) {
        eprintln!("Failed to create or write to file {}: {}", new_file, e);
        return;
    }

    println!("{} を作成しました！", new_file);

    // cargo fmt を使用してコードを整形
    if let Err(e) = format_with_cargo(&new_file) {
        eprintln!("Failed to format file {}: {}", new_file, e);
    }
}

/// 新しいファイル名を生成する関数
///
/// # 引数
/// - `filename`: コンテスト名
///
/// # 戻り値
/// ファイルパスを含む新しいファイル名
fn make_new_filename(filename: &str) -> String {
    let root = var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
    let date = Local::now().format("%Y%m%d").to_string();
    format!("{}/src/bin/{}_{}.rs", root, date, filename)
}

/// テンプレートに基づいてファイルの内容を生成する関数
///
/// # 引数
/// - `settings`: 設定情報
///
/// # 戻り値
/// 生成されたファイルの内容
fn generate_contents(settings: &Settings) -> String {
    let input_type = generate_arguments(&settings.input_types, true);
    let execute_args = generate_arguments(&settings.execute_types, false);
    let execute_types = generate_arguments(&settings.execute_types, true);
    let test_code = generate_test_code(&settings.test);
    template_base(input_type, execute_args, execute_types, test_code)
}

/// 関数の引数リストを生成する関数
///
/// # 引数
/// - `args`: 引数の型リスト
/// - `with_types`: 型を含めるかどうか
///
/// # 戻り値
/// 引数リストの文字列
fn generate_arguments(args: &Vec<&str>, with_types: bool) -> String {
    args.into_iter()
        .enumerate()
        .map(|(index, val)| {
            if with_types {
                format!("a{}: {}", index + 1, val)
            } else {
                format!("a{}", index + 1)
            }
        })
        .collect::<Vec<String>>()
        .join(",")
}

/// テストコードを生成する関数
///
/// # 引数
/// - `test_cases`: テストケースの入力と期待される出力のリスト
///
/// # 戻り値
/// テストコードの文字列
fn generate_test_code(test_cases: &[Vec<&str>]) -> String {
    test_cases
        .chunks(2)
        .enumerate()
        .map(|(index, chunk)| {
            let input = chunk[0].join(",");
            let result = chunk[1].join(",");
            let arg_str = chunk[0]
                .iter()
                .enumerate()
                .map(|(i, _)| format!("input.{}", i))
                .collect::<Vec<String>>()
                .join(",");

            template_test(index + 1, &input, &arg_str, &result)
        })
        .collect::<String>()
}

/// コードテンプレートを取得する関数
///
/// # 引数
/// - `input_type`: `input!` マクロに渡す引数の型
/// - `execute_args`: `execute` 関数に渡す引数
/// - `test_code`: テストコードの文字列
///
/// # 戻り値
/// テンプレート文字列
fn template_base(
    input_type: String,
    execute_args: String,
    execute_types: String,
    test_code: String,
) -> String {
    format!(
        r#"
        use proconio::input;

        fn main() {{
            input! {{{}}}
            let result = execute({});
            println!("{{}}", result);
        }}

        fn execute({}) -> String {{
            let answer = format!("{{}}{{}}", a1 + a2, a3);
            format!("{{}}", answer)
        }}

        #[cfg(test)]
        mod tests {{
            use super::*;

            {}
        }}
        "#,
        input_type, execute_args, execute_types, test_code
    )
}

/// テストコードのテンプレートを生成する関数
///
/// # 引数
/// - `index`: テスト番号
/// - `input`: テストケースの入力
/// - `arg_str`: 関数呼び出し時の引数リスト
/// - `result`: 期待される出力
///
/// # 戻り値
/// テストコードの文字列
fn template_test(index: usize, input: &str, arg_str: &str, result: &str) -> String {
    format!(
        r#"
    #[test]
    fn test_no{}() {{
        let input = ({});
        let result = execute({});
        assert_eq!(result, "{}");
    }}
    "#,
        index, input, arg_str, result
    )
}

/// `cargo fmt` を使ってコードを整形する関数
///
/// # 引数
/// - `file`: 整形するファイルパス
///
/// # 戻り値
/// 整形処理の結果
fn format_with_cargo(file: &str) -> std::io::Result<()> {
    Command::new("cargo")
        .arg("fmt")
        .arg("--")
        .arg(file)
        .status()
        .map(|status| {
            if !status.success() {
                panic!("cargo fmt failed for file: {}", file);
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_base() {
        let input_type = "a1: i32, a2: i32, a3: String".to_string();
        let execute_args = "a1, a2, a3".to_string();
        let execute_types = "a1, a2, a3".to_string();
        let test_code = "#[test] fn test_example() { assert_eq!(1, 1); }".to_string();

        let result = template_base(input_type, execute_args, execute_types, test_code);

        assert!(result.contains("use proconio::input"));
        assert!(result.contains("input! {a1: i32, a2: i32, a3: String}"));
        assert!(result.contains("let result = execute(a1, a2, a3);"));
        assert!(result.contains("#[test] fn test_example() { assert_eq!(1, 1); }"));
    }

    #[test]
    fn test_generate_arguments() {
        let args = vec!["i32", "i32", "String"];

        let result_with_types = generate_arguments(&args, true);
        assert_eq!(result_with_types, "a1: i32,a2: i32,a3: String");

        let result_without_types = generate_arguments(&args, false);
        assert_eq!(result_without_types, "a1,a2,a3");
    }

    #[test]
    fn test_generate_test_code() {
        let test_cases = vec![
            vec!["1", "2", "\"star\".to_string()"],
            vec!["3star"],
            vec!["10", "2", "\"planet\".to_string()"],
            vec!["12planet"],
        ];

        let result = generate_test_code(&test_cases);

        assert!(result.contains("#[test]"));
        assert!(result.contains("let input = (1,2,\"star\".to_string());"));
        assert!(result.contains("assert_eq!(result, \"3star\");"));
    }
}
