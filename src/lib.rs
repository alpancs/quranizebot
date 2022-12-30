use serde::Deserialize;
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    Router::new()
        .post_async("/webhook/telegram/update", |mut req, _ctx| async move {
            match req.json::<Update>().await {
                Ok(update) => handle_update(update),
                Err(err) => Response::error(err.to_string(), 400),
            }
        })
        .run(req, env)
        .await
}

fn handle_update(update: Update) -> Result<Response> {
    Response::ok(format!("{:?}", update))
}

#[derive(Deserialize, Debug)]
struct Update {
    inline_query: InlineQuery,
}

#[derive(Deserialize, Debug)]
struct InlineQuery {
    id: String,
    query: String,
}
