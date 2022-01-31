use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};
use http::StatusCode;
use async_graphql_warp::GraphQLBadRequest;

mod server;

#[tokio::main]
async fn main() {

    
    let routes = server::return_final_filter().recover(|err: Rejection| async move {
        if let Some(GraphQLBadRequest(err)) = err.find() {
            return Ok::<_, Infallible>(warp::reply::with_status(
                err.to_string(),
                StatusCode::BAD_REQUEST,
            ));
        }

        Ok(warp::reply::with_status(
            "INTERNAL_SERVER_ERROR".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    });
    // let routes = warp::path::end().and(warp::get()).map(|| "hello world");
    println!("running on {}" , "localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
