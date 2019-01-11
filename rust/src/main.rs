#![feature(proc_macro_hygiene, decl_macro)]

use std::error::Error;
use std::io::Read;

use rocket::{Rocket, State, Data, post, routes, http::Status};
use mongodb::{Bson, Client, ThreadedClient, db::ThreadedDatabase};
use serde_json::Value;

fn connect_to_mongo(conn_str: &str) -> Client {
    Client::with_uri(conn_str)
        .expect("Failed to initialize client.")
}

#[post("/", format = "application/json", data = "<data>")]
fn stash(conn: State<Client>, data: Data) -> Status {
    let mut buf = String::new();

    let json_str = match data.open().read_to_string(&mut buf) {
        Ok(_) => buf,
        Err(err) => {
            eprintln!("Stream bufferization error: {}", err);
            return Status::InternalServerError
        }
    };

    let json: Value = match serde_json::from_str(&json_str) {
        Ok(val) => val,
        Err(_) => return Status::BadRequest
    };

    let bson = json.into();

    let doc = match bson {
        Bson::Document(v) => v,
        _ => return Status::BadRequest
    };

    let coll = conn.db("test").collection("test");

    match coll.insert_one(doc, None) {
        Ok(_) => Status::Ok,
        Err(err) => {
            eprintln!("Database error: {}", err.description());
            Status::InternalServerError
        }
    }
}

fn setup_rocket(client: Client) -> Rocket {
    rocket::ignite()
        .mount("/", routes![stash])
        .manage(client)
}

fn main() {
    let db_uri = std::env::var("DB_URI")
        .expect("Failed to get 'DB_URI' env var.");

    let client = connect_to_mongo(&db_uri);
    setup_rocket(client).launch();
}
