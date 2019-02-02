use hyper::{
    Body, Request, Response, Server, Method, StatusCode, Chunk,
    service::service_fn_ok,
    rt::{self, Future}
};

use futures::Stream;

fn main() {
    pretty_env_logger::init();
    
    let addr = ([0, 0, 0, 0], 3000).into();

    let service = || {
        service_fn_ok(move |req: Request<Body>| {
            match (req.method(), req.uri().path()) {
                (&Method::POST, "/") => {
                    req.body()
                        .concat2()
                        .and_then(move |chunk: Chunk| {
                            serde_json::from_slice(&chunk).unwrap()
                        });

                    Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::empty())
                        .unwrap()
                },
                _ => {
                    Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::empty())
                        .unwrap()
                }
            }
        })
    };

    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on {}", addr);

    rt::run(server);

    /*pretty_env_logger::init();

    let port = std::env::var("PORT")
        .expect("Failed to get 'PORT' env var.");

    let db_uri = std::env::var("DB_URI")
        .expect("Failed to get 'DB_URI' env var.");

    let client = Client::with_uri(&db_uri)
            .expect("Failed to initialize client.");

    let addr = ([127, 0, 0, 1], 3000).into();

    let new_service = move || {
        let client = client.clone();

        /*service_fn_ok(move |req: Request<Body>| {
            match (req.method(), req.uri().path()) {
                (&Method::POST, "/") => {
                    req.body().concat2().and_then(move |chunk: Chunk| {
                        serde_json::from_slice(&chunk).unwrap()
                    })

                    /*let json = match serde_json::from_slice(&body) {
                        Ok(v) => v,
                        Err(_) => return Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::empty())
                            .unwrap()
                    };*/

                    /*Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::empty())
                        .unwrap()*/
                },
                (..) => {
                    Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::empty())
                        .unwrap()
                }
            }

            /*let prev = cnt.get();
            cnt.set(prev + 1);
            Response::new(Body::from(format!("Request count: {}", prev + 1)))*/
        })*/
    };

    let exec = current_thread::TaskExecutor::current();

    let server = Server::bind(&addr)
        .executor(exec)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);*/
}
