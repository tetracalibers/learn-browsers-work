Challenge to build a toy browser in Rust

## WIP: HTML Parser

### 実行方法

次のコマンドで`src/main.rs`の`run_html`関数に定義されたサンプル動作を見ることができます。

```bash
cargo run -- html
```

ログレベルを`debug`にすると、パーサーの状態遷移やトークン発行のログが表示されるようになります。

```bash
RUST_LOG=debug cargo run -- html
```

ログレベルを`trace`にすると、Tokenizerの各状態で検出した処理対象の文字も表示されるようになります。

```bash
RUST_LOG=trace cargo run -- html
```

`-- html`ではなく、`-- fast_html`とすると、高速化バージョンを実行できます。

### Maybe later...

- 古いDOCTYPE
- 文字参照
- コメント
- scriptタグ
- styleタグ
- framesetタグ
- templateタグ
- searchタグ + form関連タグ
- SVG関連タグ

## WIP: CSS Parser

現時点ではセレクタのパーサーしか実装されていません。

### 実行方法

次のコマンドで`src/main.rs`の`run_css`関数に定義されたサンプル動作を見ることができます。

```bash
cargo run -- css
```
