use async_graphql::{MergedObject, EmptyMutation, EmptySubscription, Schema};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_warp::{GraphQLResponse};
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter};

#[path="movie.rs"]
mod movie;
use movie::MovieQuery;

#[path="actor.rs"]
mod actor;
use actor::ActorQuery;

#[path="schema.rs"]
mod schema;


#[derive(MergedObject, Default)]
pub struct Query(MovieQuery , ActorQuery);



// TODO : 
//      split the Object implementations into several files 
//      use arangosearch 
//      return graph related data 
//      return stream (cursor or ?)


pub fn api_filter() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    let my_schema = schema::schema_builder();

    let graphql_post = async_graphql_warp::graphql(my_schema).and_then(
        |(my_schema, request): (
            Schema<Query, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(my_schema.execute(request).await))
        },
    );


    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/api")))
    });

    warp::path("api").and(graphql_playground.or(graphql_post))
}
