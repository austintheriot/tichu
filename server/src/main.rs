#[macro_use]
extern crate rocket;
extern crate common;

use rocket::{tokio::time::{sleep, Duration}};
use rocket::serde::{Serialize, json::Json};



#[get("/")]
fn index() -> String {
    format!("Hello, world!")
}

#[get("/wait/<seconds>")]
async fn wait(seconds: u64) -> &'static str {
    sleep(Duration::from_secs(seconds)).await;
    "Test response"
}

#[get("/segment/<segment..>")]
fn segment(segment: std::path::PathBuf) -> String {
    format!("Segment: {:?}", segment)
}

#[derive(Serialize)]
struct Person {
    name: String,
}

#[get("/json/<name>")]
fn json(name: &str) -> Json<Person> {
    // NOTE: In a real application, we'd use `rocket_contrib::json::Json`.
    let person = Person { name: String::from(name) };
    // Json(serde_json::to_string(&person).unwrap())
    Json(person)
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, wait, segment, json])
}
