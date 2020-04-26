use env_logger::Env;
use std::env;
use warp::Filter;

#[macro_use]
extern crate error_chain;

mod errors {
    error_chain! {}
}

pub use errors::*;

mod filters;

#[tokio::main]
async fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let api = filters::ip();
    let routes = api.with(warp::log("ip"));
    let port = match env::var("PORT") {
        Ok(val) => val.parse::<u16>().unwrap(),
        Err(_) => 8080,
    };
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
