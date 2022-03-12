mod db;
mod template;

use crate::db::*;
use crate::template::Context;
use crate::template::index;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use tinytemplate::TinyTemplate;

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let r = get_question_and_answers().await;

    let mut tt = TinyTemplate::new();

    tt.add_template("hello", index()).unwrap();

    let context = Context {
        name: "piroca".to_string(),
    };

    let rendered = tt.render("hello", &context).unwrap();

    Ok(Response::new(Body::from(rendered)))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello)) });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
