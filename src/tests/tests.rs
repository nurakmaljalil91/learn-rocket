use rocket::local::blocking::Client;
use rocket::http::{RawStr, Status};

#[test]
fn hello_world(){
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/api").dispatch();
    assert_eq!(response.into_string().unwrap(), expectde);
}