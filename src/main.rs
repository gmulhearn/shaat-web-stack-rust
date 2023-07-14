pub mod askama_to_actix_responder;
pub use askama_to_actix_responder::*;

use actix_web::{get, web, App, HttpServer, Responder};
use askama::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
struct HelloTemplate<'a> { // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    // format!("Hello {}!", &name)
    HelloTemplate { name: &name }.to_response()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(hello))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
