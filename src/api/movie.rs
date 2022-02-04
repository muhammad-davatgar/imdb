use async_graphql::SimpleObject;

#[path="actor.rs"]
mod actor;
pub use actor::Actor;



#[derive(SimpleObject)]
pub struct Movie{
    pub genre : String,
    pub name : String,
    pub year : i32,
    pub description : String,
    pub actors : Vec<Actor>
}


