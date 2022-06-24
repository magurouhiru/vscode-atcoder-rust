# vscode-atcoder-rust
VSCodeでAtCoderをRustでやるときのオレオレ環境です。  

## ※注意1 Rustのバージョンは「1.47.0」です。「1.42.0」じゃないです。
AtCoderの環境は1.42.0だけど、rust-lang.rust-analyzerが1.47.0以降じゃないと[動かない](https://github.com/rust-lang/rust-analyzer/issues/4172)からこれにしてます。  
なんかまずそうだったらその時考えます。

## ※注意2 対応しているコンテストは ABC(AtCoder Beginner Contest)だけです。

## 使い方
1. オレオレ拡張機能「atcoder-1.0.0.vsix」をインストールしてください。
![demo_install](https://raw.github.com/wiki/magurouhiru/vscode-extension-atcoder/images/atcoder_extension_install.gif)
2. オレオレ拡張機能「atcoder」でプロジェクト（Rustだとプロジェクトとは言わない？）を作成してください。
![demo_install](https://raw.github.com/wiki/magurouhiru/vscode-extension-atcoder/images/atcoder_extension_how.gif)
3. できたプロジェクトの「src/main.rs」にゴリゴリ書いてください。
4. テスト実行（cargo test）してあっていればOK！（...多分）
