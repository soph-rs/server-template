use axum::routing::get;
use soph::prelude::*;
use soph_server::{response::response, Server};

struct App;

impl ApplicationTrait for App {
    type Service = Server;

    fn with_routing() -> impl ServiceTrait {
        Server::new().register(
            axum::Router::new().route("/", get(|| async { response().text("Hello soph!") })),
        )
    }
}

fn main() -> Result<()> {
    run::<App>()
}
