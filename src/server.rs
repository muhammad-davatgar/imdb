use warp::{Filter};


// mod template;
// mod api;



pub fn return_final_filter() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {

    let first = warp::path::end().and(warp::get()).map(||{
        "12"
    });
    first
}