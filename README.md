Challenge to build a toy browser in Rust

## WIP: HTML Parser

### 実行方法

次のコマンドで動作を見ることができます。

```bash
TRACE_HTML_TREE_BUILDER=true TRACE_TOKENIZER=true cargo run
```

### サポートしない予定のもの

- 古いDOCTYPE
- 文字参照
- scriptタグ
- framesetタグ
- templateタグ
- SVG
