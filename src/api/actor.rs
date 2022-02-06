use async_graphql::{SimpleObject , Object , Result , Error as Qerr , Context , ErrorExtensions};
use serde::{Deserialize , Serialize};
use std::collections::HashMap;



use crate::DB_NAME;

// #[path = "schema.rs"]
// mod schema;
// pub use schema::Pool;
use crate::server::api::schema::Pool;


#[derive(Serialize, Deserialize, Debug)]
struct Parameter {
    param : String
}



#[derive(SimpleObject, Deserialize, Debug)]
pub struct Actor {
    pub name : String,
    pub birthday : String,
    pub birthplace : String
}


#[derive(Default)]
pub struct ActorQuery;

#[Object]
impl ActorQuery{
    async fn actor<'a>(&self , ctx : &Context<'a> , name : String) -> Result<Actor> {
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
        match result.pop() { 
            Some(v) => Ok(v),
            None => Err(Qerr::new("not found").extend_with(|_err, e| e.set("code", 404)))
        }
    }
}
