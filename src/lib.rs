use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    Router::new()
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .run(req, env)
        .await
}
