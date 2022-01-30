use warp::Filter;

mod server;

#[tokio::main]
async fn main() {

    
    let routes = server::return_final_filter();
    // let routes = warp::path::end().and(warp::get()).map(|| "hello world");
    println!("running on {}" , "localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
