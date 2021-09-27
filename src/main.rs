#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

use webapi;



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![webapi::routes::hello::hello])
        .mount("/hello", routes![webapi::routes::hello::world, webapi::routes::hello::mir])
        .mount("/wave", routes![webapi::routes::hello::wave])
}

// #[rocket::main]
// async fn main() {
//     rocket::build()
//         .mount("/api/v1", routes![
//             index,
//             hello_sakura
//         ])
//         .launch()
//         .await;
// }