# onepaas.slack-notify
.onepaas/workflows/*.tomlをパースして通知を投げるDocker Image

# todo
- `.onepaas/workflows/*.toml` を探索
- 読み込む ここはtomlを使う。
- slackに通知投げる ここはhyperを使ってみる。
- 環境変数でwebhook URLを指定する。
- 使い方: cargo install path . したDocker Image -> git checkoutしてそこのディレクトリで動かす。