## telegram bot on cloudflare worker by rust demo

使用rust编写的cf worker tgbot demo

demo功能只包含反馈用户输入，有50%的概率反馈，50%的概率不反馈

## 配置wrangler

- 安装

```shell
npm install -g wrangler
```

- 修改wrangler.toml

```toml
name = "cf-worker-test1"  # 修改成你想要的名字
```

- 设置bot_token

```shell
npx wrangler secret put BOT_TOKEN

```

这个时候应该会让你登陆cf,登陆后会要求你输入（这时把botfather给你的token复制过来）

## 部署

```shell
npx wrangler deploy
```

去cf官网上看看给你分配的worker的url

## setWebhook

非常关键的一步

向tg发送一个get请求

```shell
curl -X GET https://api.telegram.org/bot{my_bot_token}/setWebhook?url={url_to_send_updates_to}
```

my_bot_token是上面的bot_token (注意前`bot` 需要带上，中括号不需要)

url_to_send_updates_to是cf给的worker地址

具体细节可以参考：<https://xabaras.medium.com/setting-your-telegram-bot-webhook-the-easy-way-c7577b2d6f72>

eg:

```
https://api.telegram.org/bot123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11/setWebhook?url=https://www.example.com/my-telegram-bot
```
