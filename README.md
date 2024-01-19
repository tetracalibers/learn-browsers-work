Challenge to build a toy browser in Rust

## WIP: HTML Parser

### 実行方法

次のコマンドで`src/main.rs`の`run_html`関数に定義されたサンプル動作を見ることができます。

```bash
cargo run -- html
```

パーサーの状態遷移など、より細かいログを見たい場合は、次のように環境変数を添えてください。

```bash
RUST_LOG=trace cargo run -- html
```

`-- html`ではなく、`-- fast_html`とすると、高速化バージョンを実行できます。

### サポートしない予定のもの

- 古いDOCTYPE
- 文字参照
- scriptタグ
- framesetタグ
- templateタグ
- SVG

## WIP: CSS Parser

現時点ではセレクタのパーサーしか実装されていません。

### 実行方法

次のコマンドで`src/main.rs`の`run_css`関数に定義されたサンプル動作を見ることができます。

```bash
cargo run -- css
```
