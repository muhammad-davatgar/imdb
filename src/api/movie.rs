use async_graphql::{Context , Object};

#[path="actor.rs"]
mod actor;
pub use actor::Actor;

pub struct Movie{
    pub Genre : String,
    pub name : String,
    pub year : i32,
    pub description : String,
    pub actors : Vec<Actor>
}



#[Object]
impl Movie{
    async fn Genre(&self) -> String {
        self.Genre.clone()
    }
}