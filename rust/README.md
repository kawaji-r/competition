## 新しい問題の始め方

1. 問題文を読む
1. `new_contest.rs` の `setting` を編集する
1. `new_contest` バイナリを実行する
   ```
   cargo run --bin new_contest
   ```
1. 問題を解く
1. テストする
   - vscode の拡張機能で、クリックでやるのが簡単
   - println!したい場合は、`cargo test --bin <file_name> -- --nocapture`
1. 言語が rustc になっていることを確認して提出。テストコード以外の部分をコピペする。
