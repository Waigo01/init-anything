use tide::{Request, Response, StatusCode};
use tide::security::{CorsMiddleware, Origin};
use tide::prelude::*;
use std::env;
use tera::{Tera, Context};

#[macro_use]
extern crate lazy_static;

lazy_static!{
    pub static ref TEMPLATES: Tera = {
        let tera = Tera::new("./templates/**/*").unwrap();
        tera
    };
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut app = tide::new();
    let cors = CorsMiddleware::new().allow_origin(Origin::from("*"));
    app.with(cors);
    app.at("/static/").serve_dir("./static/")?;
    app.at("/").serve_file("./pages/index.html")?;
    app.listen(format!("0.0.0.0:{}", args[1])).await?;
    Ok(())
}
