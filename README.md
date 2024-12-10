# AlephZeroContracts

このリポジトリには、Aleph Zeroブロックチェーン用のスマートコントラクトの例と、そのデプロイおよび操作スクリプトが含まれています。

## フォルダ構成

- src/lib.rs: スマートコントラクトのコードを含むRustのソースファイル。
- target/ink/mytoken.contract: cargo contractによって生成されたコンパイル済みスマートコントラクトファイル。
- deploy/deploy.sh: スマートコントラクトをデプロイするスクリプト。
- interaction/interact.sh: デプロイ済みのスマートコントラクトとやり取りするスクリプト。

## 必要条件

- Rustとcargo-contractをインストールする。
- Aleph ZeroのPolkadot.jsウォレットを設定し、テストネットの資金を取得する。
- シェルスクリプトに必要なcurlなどの依存関係をイン

## ストールする。
クイックスタート
スマートコントラクトをコンパイルする:

bash

   cargo +nightly contract build
