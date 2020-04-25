use env_logger::Env;
use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let api = filters::ip();
    let routes = api.with(warp::log("ip"));
    let port = 3030;
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

mod filters {
    use super::handlers;
    use warp::Filter;

    pub fn ip() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        ip_update()
    }

    pub fn ip_update() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("update")
            .and(warp::put())
            .and(warp::addr::remote())
            .and_then(handlers::update_ip)
    }
}

mod handlers {
    use std::convert::Infallible;
    use std::fs;
    use std::net::SocketAddr;
    use warp::http::StatusCode;

    pub async fn update_ip(addr_opt: Option<SocketAddr>) -> Result<impl warp::Reply, Infallible> {
        if let None = addr_opt {
            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let addr = addr_opt.unwrap();
        let ip = addr.ip();

        match fs::write("/tmp/foo", ip.to_string()) {
            Err(e) => {
                log::error!("Writing file: {:?}", e);
                return Ok(StatusCode::INTERNAL_SERVER_ERROR);
            }
            Ok(()) => log::info!("udpated ip to {}", ip.to_string()),
        }

        Ok(StatusCode::ACCEPTED)
    }
}
