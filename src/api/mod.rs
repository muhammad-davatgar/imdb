use async_graphql::{Context , Object};


struct QueryRoot;



#[Object]
impl QueryRoot{
    async fn movie<'a>(&self , ctx : &Context<'a> , name : String) -> Movie{
        Movie { 
            Genre : String::from("action"),
            name : name,
            year : 1234,
            description : String::from("i'm description "),
            actors : vec![Actor{name : String::form("actor name") , year : 12534}]
        }
    }
}




struct Movie{
    Genre : String,
    name : String,
    year : i32,
    description : String,
    actors : Vec<Actor>
}


struct Actor {
    name : String,
    year : i32,
}