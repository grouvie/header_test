use std::net::SocketAddr;

use axum::{ Router, routing::get, response::Html, http::HeaderMap, extract::ConnectInfo };
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // build our application with a route
    let routes_all = Router::new().route("/", get(handler));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    // run it
    axum::serve(
        listener,
        routes_all.into_make_service_with_connect_info::<SocketAddr>()
    ).await.unwrap();
}

async fn handler(
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>
) -> Html<&'static str> {
    println!("Request from: {}", addr);
    println!("Headers:");
    println!("{:#?} \n", headers);
    Html("<h1>Hello, World!</h1>")
}
