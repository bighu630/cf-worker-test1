use telbot_cf_worker::types::update::{Update, UpdateKind};
use rand::Rng;
use worker::*;

#[event(fetch)]
async fn fetch(
    _req: Request,
    _env: Env,
) -> Result<Response> {
    let api = telbot_cf_worker::Api::new(_env.secret("BOT_TOKEN").unwrap().to_string());
    let router = Router::with_data(api);

    router
        .post_async("/", |mut req, ctx| async move {
            let update = req.json::<Update>().await.unwrap();
            if let UpdateKind::Message { message } = update.kind {
                if let Some(text) = message.kind.text() {
                    console_log!("{}", text);
                    let api = ctx.data();
                    let mut rng = rand::thread_rng();
                    let random_number = rng.gen_range(0..=100);
                    if random_number < 50 {
                        api.send_json(&message.reply_text(text))
                            .await
                            .expect("failed to send message");
                    }
                }
            }else if let UpdateKind::ChannelPost { channel_post } = update.kind{
                if let Some(text) = channel_post.kind.text() {
                    console_log!("{}", text);
                    let api = ctx.data();
                    let mut rng = rand::thread_rng();
                    let random_number = rng.gen_range(0..=100);
                    if random_number < 10 {
                        api.send_json(&channel_post.reply_text(text))
                            .await
                            .expect("failed to send message");
                    } 
                }
            }
            Response::empty()
        }).run(_req, _env).await
}
