use warp::Filter;

mod handlers;

pub fn ip() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    ip_update()
}

fn ip_update() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("update")
        .and(warp::put())
        .and(warp::header("Authorization"))
        .and(warp::header("X-Forwarded-For"))
        .and_then(handlers::update_ip)
}
