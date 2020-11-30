#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
//#[macro_use] extern crate serde_derive;
use serde::{Serialize, Deserialize};

//use rocket::State;
//use rocket_contrib::json::{Json, JsonValue};

//#[post("/user", format = "application/json", data = "<user>")]
//fn new_user(user: User) { /* ... */ }

#[get("/")]
fn index() -> &'static str {
    ""
}


//#[derive(Deserialize)]
//struct Task {
//    description: String,
//    complete: bool
//}
//
//#[post("/todo", data = "<task>")]
//fn new(task: Json<Task>) { /* .. */ }

//#[get("/<id>", format = "json")]
//fn get(id: ID, map: State<MessageMap>) -> Option<Json<Message>> {
//    let hashmap = map.lock().unwrap();
//    hashmap.get(&id).map(|contents| {
//        Json(Message {
//            id: Some(id),
//            contents: contents.clone()
//        })
//    })
//}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}