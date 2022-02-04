use async_graphql::SimpleObject;
use serde::Deserialize;


#[derive(SimpleObject, Deserialize, Debug)]
pub struct Actor {
    pub name : String,
    pub year : i32,
}

