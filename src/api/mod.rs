use async_graphql::{Context , Object , EmptyMutation, EmptySubscription, Schema};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_warp::{ GraphQLResponse};
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};

#[path="movie.rs"]
mod movie;

use movie::Actor;
use movie::Movie;

struct Query;



#[Object]
impl Query{
    async fn movie<'a>(&self , ctx : &Context<'a> , name : String) -> Movie{
        Movie { 
            Genre : String::from("action"),
            name : name,
            year : 1234,
            description : String::from("i'm description "),
            actors : vec![Actor{name : String::from("actor name") , year : 12534}]
        }
    }
}


pub fn api_filter() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    let schema = Schema::build(Query , EmptyMutation , EmptySubscription).finish();

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );


    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/api")))
    });

    warp::path("api").and(graphql_playground.or(graphql_post))
}
