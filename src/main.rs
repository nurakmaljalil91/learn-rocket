#[macro_use]
extern crate rocket;

#[get("/")] // <- route attribute
fn index() -> &'static str {
    "Hello, world!"
} // <- request handler

#[get("/sakura")]
fn hello_sakura() -> &'static str { "Hello Sakura my love!" }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/hello", routes![world])
// } // <- This will make Clion not responsive

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/api/v1", routes![
            index,
            hello_sakura
        ])
        .launch()
        .await;
}