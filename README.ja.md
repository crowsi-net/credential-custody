# credential-custody

汎用Crowsi部品を機能単位にまとめたworkspaceです。[package一覧と実行例](README.md)を参照してください。

製品CLI、Windows helper、環境登録、保守timerは[vPremises environment-operations](https://github.com/vpremises/environment-operations)が持ちます。authority、broker、実行点、独立検証、復旧のtrust roleは統合しません。既存crate名・wire/schema ID・publish=falseを維持し、配置変更だけでregistry公開済みとは扱いません。元の公開履歴とLICENSE/NOTICEを保持しています。

```sh
cargo +1.97 test --locked --workspace --all-targets --all-features
cargo +1.97 test --locked --workspace --doc --all-features
```

テストは合成入力で実施し、実account・service・timerへの配置は行いません。
