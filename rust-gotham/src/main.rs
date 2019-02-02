use futures::{future, Future, Stream};

use hyper::{Body, StatusCode};

use gotham::{
    middleware::state::StateMiddleware,
    handler::{HandlerFuture, IntoHandlerError},
    helpers::http::response::create_response,
    pipeline::{single::single_pipeline, single_middleware},
    router::{Router, builder::{build_router, DrawRoutes, DefineSingleRoute}},
    state::{State, FromState}
};

use gotham_derive::StateData;

use serde_json::Value;

use mongodb::{Client, ThreadedClient, db::ThreadedDatabase, Bson, Document};

#[derive(Clone, StateData)]
struct DatabaseClient(Client);

impl DatabaseClient {
    fn conn(&self) -> Client {
        self.0.clone()
    }
}

fn form_handler(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let body_content = valid_body.into_bytes();

                let json: Value = match serde_json::from_slice(&body_content) {
                    Ok(v) => v,
                    Err(_) => {
                        let res = create_response(&state, StatusCode::BAD_REQUEST, mime::TEXT_PLAIN, "BAD_REQUEST\n");
                        println!("JSON deserialize error.\n");
                        return future::ok((state, res))
                    }
                };

                let doc: Document = match json.into() {
                    Bson::Document(v) => v,
                    _ => {
                        let res = create_response(&state, StatusCode::BAD_REQUEST, mime::TEXT_PLAIN, "BAD_REQUEST\n");
                        println!("BSON is not a Document.");
                        return future::ok((state, res))
                    }
                };

                let conn = DatabaseClient::borrow_from(&state).conn();

                let coll = conn.db("test").collection("test");

                match coll.insert_one(doc, None) {
                    Ok(_) => {
                        let res = create_response(&state, StatusCode::OK, mime::TEXT_PLAIN, "OK\n");
                        future::ok((state, res))
                    }
                    Err(_) => {
                        let res = create_response(&state, StatusCode::INTERNAL_SERVER_ERROR, mime::TEXT_PLAIN, "INTERNAL_SERVER_ERROR\n");
                        println!("Failed to put Document in DB.");
                        future::ok((state, res))
                    }
                }
            }
            Err(e) => future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}

fn router() -> Router {
    let db_uri = std::env::var("DB_URI")
        .expect("Failed to get 'DB_URI' env var.");

    let client = Client::with_uri(&db_uri)
        .expect("Failed to initialize client.");

    println!("Connected to MongoDB");

    let middleware = StateMiddleware::new(DatabaseClient(client));
    let pipeline = single_middleware(middleware);
    let (chain, pipelines) = single_pipeline(pipeline);

    build_router(chain, pipelines, |route| {
        route.post("/").to(form_handler);
    })
}

pub fn main() {
    let port = std::env::var("PORT")
        .expect("Failed to get 'PORT' env var.");

    let addr = format!("0.0.0.0:{}", port);
    println!("Listening for requests at {}", addr);
    gotham::start(addr, router())
}
