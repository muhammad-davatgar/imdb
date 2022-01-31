use warp::{Filter};


#[path="template/mod.rs"]
mod template;
#[path="api/mod.rs"]
mod api;

pub fn return_final_filter() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {

    let api_filter = api::api_filter();
    api_filter
}