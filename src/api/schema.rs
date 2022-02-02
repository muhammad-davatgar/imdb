use async_graphql::{Schema , EmptyMutation, EmptySubscription };
use arangors::connection::{self , GenericConnection};
use uclient::reqwest::ReqwestClient;
use async_trait::async_trait;
use deadpool::managed;


use super::Query;


static DB_NAME :&str = "IMDB";


#[derive(Debug)]
enum Error { Fail }



struct Manager {}

#[async_trait]
impl managed::Manager for Manager {
    type Type = GenericConnection<ReqwestClient>;
    type Error = Error;
    
    async fn create(&self) -> Result<Type, Error> {
        let conn = connection::establish_without_auth("http://localhost:8529")
            .await.expect("couldn't connecto to db");
        let db = conn.db(DB_NAME);
        Ok(db)
    }
    
    async fn recycle(&self, _: &mut Type) -> managed::RecycleResult<Error> {
        Ok(())
    }
}

type Pool = managed::Pool<Manager>;




pub fn schema_builder() -> Schema<Query , EmptyMutation , EmptySubscription> {

    let mgr = Manager {};
    let pool = Pool::builder(mgr).build().expect("failure building connection pool");
    let schema = Schema::build(Query , EmptyMutation , EmptySubscription).data(pool).finish();
    schema
}


