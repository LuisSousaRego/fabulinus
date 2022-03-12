use serde::Serialize;

#[derive(Serialize)]
pub struct Context {
    pub name: String,
}

pub fn index() -> &'static str {
    include_str!("index.html")
}
