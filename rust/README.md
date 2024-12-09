## 新しい問題の始め方

1. bin フォルダ内の過去の問題をコピーして新しい rust ファイルを作る
   ```
   cp src/bin/template.rs new_file.rs
   ```
1. 問題文を読む
1. 問題のインプットに応じて `modify` の箇所を修正する
1. 問題を解く
1. テスト実施
   - vscode の拡張機能で、クリックでやるのが簡単
   - println!したい場合は、`cargo test --bin <file_name> -- --nocapture`
1. 提出。テストコード以外の部分をコピペする。
