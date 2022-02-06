use async_graphql::SimpleObject;
use serde::Deserialize;
#[path="actor.rs"]
mod actor;
pub use actor::Actor;



#[derive(SimpleObject , Deserialize)]
pub struct Movie{
    pub genre : String,
    pub title : String,
    pub released : String,
    pub description : String,
    pub runtime : i32,
}


