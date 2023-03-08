use crate::commands::{add_num, add_to_arr, create, delete_state};
use rocket::{http::Status, State as RocketState};
use std::{collections::HashMap, sync::Mutex};

#[post("/command", data = "<data>")]
pub fn post_command(data: String) -> Result<String, String> {
    let req: Vec<&str> = data.split(' ').collect();
    println!("{:?}", req);
    let res = match req.first() {
        None => Err("No command supplied".to_string()),
        Some(&"") => Err("No command supplied".to_string()),
        Some(&"create") => create(req),
        Some(&"addnum") => add_num(req),
        Some(&"addtoarr") => add_to_arr(req),
        Some(&"delete") => delete_state(req),
        Some(_) => Err("Unknown command".to_string()),
    };
    res.map(|s| {
        if let Some(val) = s {
            val.to_string()
        } else {
            "".to_string()
        }
    })
}

#[post("/msg/<id>", data = "<data>")]
pub fn post_message(
    id: String,
    data: String,
    state: &RocketState<Mutex<HashMap<String, String>>>,
) -> Status {
    match state.lock() {
        Ok(mut val) => {
            if val.contains_key(&id) {
                return Status::ImATeapot;
            }
            val.insert(id, data);
            Status::Ok
        }
        Err(_) => Status::InternalServerError,
    }
}

#[get("/msg/<id>")]
pub fn get_message(id: String, state: &RocketState<Mutex<HashMap<String, String>>>) -> String {
    let locked_state = state.lock().unwrap();
    if let Some(data) = locked_state.get(&id) {
        let own_data: String = data.to_owned();
        own_data
    } else {
        "Not found".to_string()
    }
}

#[delete("/msg/<id>")]
pub fn delete_message(id: String, state: &RocketState<Mutex<HashMap<String, String>>>) -> Status {
    let mut locked_state = state.lock().unwrap();
    if locked_state.contains_key(&id) {
        locked_state.remove(&id);
        Status::NoContent
    } else {
        Status::NotFound
    }
}
