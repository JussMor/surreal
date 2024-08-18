
use futures_util::{FutureExt, StreamExt};
use warp::Filter;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = echo_route();

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}


pub fn echo_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("echo")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| {
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        })
}