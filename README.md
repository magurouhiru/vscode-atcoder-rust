# vscode-atcoder-rust
VSCodeでAtCoderをRustでやるときのオレオレ環境です。  

## バージョン情報

| ソフトウェア       | バージョン |
|-------------------|-----------|
| Rust              | 1.70.0    |
| rust-analyzer     | 0.3.2029  |

**注意：2025/04/20現在のAtCoderに合わせているため、最新バージョンではありません。**  
**注意：バージョン変更する場合は`.devcontainer\devcontainer.json`を変更してください。**

## 前提
- 使用するエディタはVSCode
- 必須のVSCode の拡張機能
    - `ms-vscode-remote.remote-containers`
- Docker 環境

## 使用方法
1. コンテナに接続
1. `src` ディレクトリ配下にソースファイルを作成
1. `Cargo.toml` に`[[bin]]`を追記
1. `tests` ディレクトリ配下にテストファイルを作成
1. コーディング
    - スニペットを用意しています。詳細は`.vscode\vscode-atcoder-rust.code-snippets`を確認ください。
    - 実行(デバッグ)はmian(test)関数上部に表示される`▷Run|⚙Debug`を押してください。
