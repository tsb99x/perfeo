#![feature(proc_macro_hygiene, decl_macro)]

use rocket::response::Response;
use std::error::Error;
use std::io::Read;
use std::io::Cursor;

use rocket::{Rocket, State, Data, post, routes, http::Status, http::ContentType};
use mongodb::{Bson, Client, ThreadedClient, db::ThreadedDatabase};
use serde_json::Value;

fn connect_to_mongo(conn_str: &str) -> Client {
    Client::with_uri(conn_str)
        .expect("Failed to initialize client.")
}

#[post("/", format = "application/json", data = "<data>")]
fn stash(conn: State<Client>, data: Data) -> Response {
    let mut buf = String::new();

    let json_str = match data.open().read_to_string(&mut buf) {
        Ok(_) => buf,
        Err(err) => {
            eprintln!("Stream bufferization error: {}", err);
            return Response::build()
                .status(Status::InternalServerError)
                .header(ContentType::Plain)
                .sized_body(Cursor::new("Internal Server Error"))
                .finalize()
        }
    };

    let json: Value = match serde_json::from_str(&json_str) {
        Ok(val) => val,
        Err(_) => return Response::build()
            .status(Status::BadRequest)
            .header(ContentType::Plain)
            .sized_body(Cursor::new("Bad Request"))
            .finalize()
    };

    let bson = json.into();

    let doc = match bson {
        Bson::Document(v) => v,
        _ => return Response::build()
            .status(Status::BadRequest)
            .header(ContentType::Plain)
            .sized_body(Cursor::new("Bad Request"))
            .finalize()
    };

    let coll = conn.db("test").collection("test");

    match coll.insert_one(doc, None) {
        Ok(_) => Response::build()
            .status(Status::Ok)
            .header(ContentType::Plain)
            .sized_body(Cursor::new("OK"))
            .finalize(),
        Err(err) => {
            eprintln!("Database error: {}", err.description());
            Response::build()
                .status(Status::InternalServerError)
                .header(ContentType::Plain)
                .sized_body(Cursor::new("Internal Server Error"))
                .finalize()
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
