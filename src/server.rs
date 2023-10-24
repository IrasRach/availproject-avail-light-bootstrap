use crate::types::Addr;
use std::net::SocketAddr;
use tracing::info;
use warp::Filter;

pub async fn run(addr: Addr) {
    let health_route_get = warp::get()
        .and(warp::path("health"))
        .map(|| warp::reply::with_status("", warp::http::StatusCode::OK));

    let health_route_head = warp::head()
        .and(warp::path("health"))
        .map(|| warp::reply::with_status("", warp::http::StatusCode::OK));

    info!("HTTP server running on http://{addr}. Health endpoint available at '/health'.");

    let socket_addr: SocketAddr = addr.try_into().unwrap();

    warp::serve(health_route_get.or(health_route_head))
        .run(socket_addr)
        .await;
}
