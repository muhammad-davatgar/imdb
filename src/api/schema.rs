use async_graphql::{Schema , EmptyMutation, EmptySubscription };
use arangors::connection::Connection;
use async_trait::async_trait;
use deadpool::managed;


// use super::Query;
use crate::server::api::Query;


#[derive(Debug)]
pub enum Error { Fail }



pub struct Manager {}

#[async_trait]
impl managed::Manager for Manager {
    type Type = Connection;
    type Error = Error;
    async fn create(&self) -> Result<Connection, Error> {
        let conn = Connection::establish_without_auth("http://localhost:8529")
            .await.expect("couldn't connecto to db");
        // let db = conn.db(DB_NAME);
        Ok(conn)
    }
    
    async fn recycle(&self, _: &mut Connection) -> managed::RecycleResult<Error> {
        Ok(())
    }
}

pub type Pool = managed::Pool<Manager>;




pub fn schema_builder() -> Schema<Query , EmptyMutation , EmptySubscription> {

    let mgr = Manager {};
    let pool = Pool::builder(mgr).build().expect("failure building connection pool");
    let schema = Schema::build(Query::default() , EmptyMutation , EmptySubscription).data(pool).finish();
    schema
}


