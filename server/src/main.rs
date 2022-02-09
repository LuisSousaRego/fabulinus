#[macro_use]
extern crate nickel;

use nickel::{HttpRouter, MiddlewareResult, Nickel, Request, Response};
use std::collections::HashMap;

mod db;

fn index<'a, D>(_: &mut Request<D>, res: Response<'a, D>) -> MiddlewareResult<'a, D> {
    let mut data = HashMap::new();
    data.insert("name", "joel");
    return res.render("src/templates/index.html", &data);
}

fn main() {
    let mut server = Nickel::new();

    server.get("/", index);

    server.listen("localhost:6767").unwrap();
}
