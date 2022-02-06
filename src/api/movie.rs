use async_graphql::{SimpleObject , Object , Result , Error as Qerr , Context , ErrorExtensions};
use serde::{Deserialize , Serialize};
use std::collections::HashMap;


use crate::DB_NAME;

#[path="actor.rs"]
mod actor;
pub use actor::{Actor};
use crate::server::api::schema::Pool;


// #[path="schema.rs"]
// mod schema;
// use schema::Pool;


#[derive(Serialize, Deserialize, Debug)]
struct Parameter {
    param : String
}



#[derive(SimpleObject , Deserialize)]
pub struct Movie{
    pub genre : String,
    pub title : String,
    pub released : String,
    pub description : String,
    pub runtime : i32,
}

#[derive(Default)]
pub struct MovieQuery;


#[Object]
impl MovieQuery {
    async fn movie<'a>(&self , ctx : &Context<'a> , name : String) -> Result<Movie>{
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


        match result.pop() { 
            Some(v) => Ok(v),
            None => Err(Qerr::new("not found").extend_with(|_err, e| e.set("code", 404)))
        }

    }
}