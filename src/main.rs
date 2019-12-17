#![deny(unused_imports)]

extern crate actix_web;
extern crate env_logger;
extern crate log;

use actix_web::{web, App, HttpRequest, HttpServer, Responder, middleware::Logger};
use log::info;

const DEFAULT_PORT : u16 = 8080;
const PORT_ENV_VARIABLE: &'static str = "HTTP_PORT";

pub fn main() {
  std::env::set_var("RUST_LOG", "info");
  env_logger::init();

  let port = std::env::var(PORT_ENV_VARIABLE)
    .ok()
    .and_then(|p| p.parse::<u16>().ok())
    .unwrap_or(DEFAULT_PORT);

  let hostname = "localhost";
  let address : String = format!("{}:{}", hostname, port).into();

  info!("Serving on {}", address);

  HttpServer::new(|| {
    App::new()
      .wrap(Logger::default())
      .route("/", web::get().to(hello))
      .route("/{name}", web::get().to(hello))
  })
  .bind(address)
  .expect("Can not bind to address/port")
  .run()
  .unwrap();
}

fn hello(req: HttpRequest) -> impl Responder {
  let name = req.match_info()
    .get("name")
    .unwrap_or("World");
  format!("Hello {}!", &name)
}

