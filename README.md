# procon-rust

## フォルダ構造

- ライブラリは `src/` に入れる
- 解いている問題は `examples/` に入れる
  - RLS や rust-anlyzer による補完が効くため
- 解けた問題は `accepted/` に入れる
  - `examples/` に溜まりすぎると重くなるため

## VS Code による機能

### タスク

デフォルトでは Ctrl + Shift + B で下記のタスクを実行する

- Download
- Run
- Test
- Submit

### スニペット

`use モジュール名` で貼り付け

## Thanks

- [online-judge-tools/oj](https://github.com/online-judge-tools/oj)
- [hatoo/cargo-snippet](https://github.com/hatoo/cargo-snippet)
