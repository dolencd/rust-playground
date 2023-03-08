pub mod commands;
mod crud;
mod state;
mod types;
use std::collections::HashMap;
use std::sync::Mutex;
use std::vec;

use crud::*;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;
use types::types::DeviceHealthEvent;

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Req<'r> {
    EVENT: &'r str,
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[post("/event", format = "application/json", data = "<req>")]
fn post_user(req: Json<Req>) -> Json<Option<DeviceHealthEvent>> {
    Json(types::types::decode_event(req.EVENT.to_string()))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let a: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    rocket::build()
        .manage(a)
        .mount(
            "/asd",
            routes![
                index,
                post_user,
                get_message,
                post_message,
                delete_message,
                post_command
            ],
        )
        .register("/msg/dogs", catchers![not_found])
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
    nums: Option<Vec<i32>>,
}

fn get_sum() -> Option<i32> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        [
            {
                "name": "John Doe",
                "age": 43,
                "phones": [
                    "+44 1234567",
                    "+44 2345678"
                ],
                "nums": [2,3,4,5,6,7,-84]
            },
            {
                "name": "John Doe",
                "age": 43,
                "phones": [
                    "+44 1234567",
                    "+44 2345678"
                ],
                "nums": [12,23,34,55,26,17, -72]
            },
            {
                "name": "John Doe",
                "age": 43,
                "phones": [
                    "+44 1234567",
                    "+44 2345678"
                ],
                "nums": [12,23,34,55,26,17]
            },
            {
                "name": "John Doe",
                "age": 43,
                "phones": [
                    "+44 1234567",
                    "+44 2345678"
                ]
            }
        ]"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Vec<Person> = serde_json::from_str(data).ok()?;

    let filtered_sum: i32 = p.iter().map(sum_for_person).sum();

    // Do things just like with any other Rust data structure.
    println!(
        "Please call {} at the number {}, total sum: {}",
        p[0].name, p[0].phones[0], filtered_sum
    );

    Some(filtered_sum)
}

fn sum_for_person(p: &Person) -> i32 {
    p.nums
        .as_ref()
        .unwrap_or(&vec![0])
        .iter()
        .filter(|x| *x % 2 == 0)
        .sum()
}

#[test]
fn returns_correct_sum() {
    let p = Person {
        age: 12,
        name: "name".to_string(),
        phones: vec![],
        nums: Some(vec![1, 2, 3, 4, 5, 6]),
    };
    assert_eq!(sum_for_person(&p), 12);
}
