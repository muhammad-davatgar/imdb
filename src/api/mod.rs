use async_graphql::{Context , Object , EmptyMutation, EmptySubscription, Schema , Result as Qres , Error as Qerr};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_warp::{ GraphQLResponse};
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};
use arangors::{Connection , Document};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
#[path="movie.rs"]
mod movie;
use movie::Actor;
use movie::Movie;

#[path="schema.rs"]
mod schema;
use schema::Pool;

use crate::DB_NAME;


pub struct Query;

#[derive(Serialize, Deserialize, Debug)]
struct Parameter {
    param : String
}

// TODO : 
//      split the Object implementations into several files 
#[Object]
impl Query{
    async fn movie<'a>(&self , ctx : &Context<'a> , name : String) -> Movie{
        // getting the connection
        let pool = ctx.data::<Pool>().expect("failure using the pool");
        let conn = pool.get().await.expect("failure getting the connection");
        let conn = conn.db(DB_NAME).await.expect("failure getting db");

        // making the query
        let mut vars = HashMap::new();
        let param = Parameter{
            param : name
        };
        vars.insert("name" , serde_json::value::to_value(&param).unwrap());

        let mut result: Vec<Movie> = conn
            .aql_bind_vars(r#"FOR doc in imdb_vertices
             FILTER doc.title ==@name.param 
             return {genre : doc.genre , 
                title : doc.title , 
                released : doc.released , 
                runtime : doc.runtime , 
                description : doc.description}"#, vars)
            .await.expect("here");


        result.pop().unwrap_or_else(||{ 
            Movie { 
                genre : "not found".to_string(),
                title : "dunno how to return 404".to_string(),
                released : "-1".to_string() ,
                runtime : -1 ,
                description : "not found".to_string()
            }
        })

        // Movie { 
        //     genre : String::from("action"),
        //     name : name,
        //     year : 1234,
        //     description : String::from("i'm description "),
        //     actors : vec![Actor{name : String::from("actor name") , birthday : "12534" , birthplace : "khan amir"}]
        // }
    }

    async fn acotr<'a>(&self , ctx : &Context<'a> , name : String) -> Actor {
        let pool = ctx.data::<Pool>().expect("failure using the pool");
        let conn = pool.get().await.expect("failure getting the connection");
        let conn = conn.db(DB_NAME).await.expect("failure getting DB");
        // let collection = conn.collection("imdb_vertices").await.expect("failure getting collection");

        let mut vars = HashMap::new();
        let param = Parameter{
            param : name
        };
        vars.insert("name" , serde_json::value::to_value(&param).unwrap());
        let mut result: Vec<Actor> = conn
            .aql_bind_vars(r#"FOR doc in imdb_vertices
             FILTER doc.name ==@name.param 
             return {name : doc.name , birthday : doc.birthday , birthplace : doc.birthplace}"#, vars)
            // .aql_str(r#"for doc in imdb_vertices filter doc.name == "James Cameron" return doc "#)
            .await
            .expect("here");
        result.pop().unwrap_or_else(|| Actor{ name : "dunno how to return 404".to_string() , birthday : "-1".to_string() , birthplace : "khan amir".to_string()})
    }
}


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
