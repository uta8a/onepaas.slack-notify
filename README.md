# onepaas.slack-notify
.onepaas/workflows/*.tomlをパースして通知を投げるDocker Image

通知を受けてCD(人間)が動く

releaseブランチにpushしたらPackage生成

環境変数
`HICODER_ONEPAAS_SLACK_TOKEN`: slack incoming webhookのURLを指定(https://...)

# todo
- appがひとつにしか対応してない
# 実装
- `.onepaas/workflows/*.toml` を探索
- 読み込む ここはtomlを使う。
- slackに通知投げる ここはhyperを使ってみる。
- 環境変数でwebhook URLを指定する。
- 使い方: cargo install path . したDocker Image -> git checkoutしてそこのディレクトリで動かす。

# Action
```
$ docker run --rm  uta8a/onepaas-slack-notify sh -c "git clone https://test.git && cd test/ && HICODER_ONEPAAS_SLACK_TOKEN=https://.../ onepaas-slack-notify"
```